use std::borrow::Borrow;
use std::time::Duration;

use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde::{Deserialize, Serialize};

use crate::ProgramError;
use csv::ReaderBuilder;
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Category {
    Noun,
    Adjective,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noun" | "nouns" => Ok(Category::Noun),
            "adjective" | "adjectives" => Ok(Category::Adjective),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Translation {
    pub meaning: String,
    pub usage: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct DictionaryEntry {
    pub category: Category,
    pub translations: Vec<Translation>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Dictionary {
    pub entries: BTreeMap<String, DictionaryEntry>,
}

impl Display for Translation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.usage {
            Some(usage) => {
                write!(f, "({}) {}", usage, self.meaning)
            }
            None => {
                write!(f, "{}", self.meaning)
            }
        }
    }
}

impl Translation {
    pub fn new(translation: String, usage: Option<String>) -> Self {
        Translation { meaning: translation, usage }
    }
}

impl DictionaryEntry {
    pub fn definition(&self) -> String {
        if self.translations.len() == 1 {
            format!("{}", self.translations.get(0).unwrap())
        } else {
            self.translations
                .iter()
                .enumerate()
                .map(|(i, t)| format!("{}. {}", i + 1, t))
                .collect::<Vec<String>>()
                .join("; ")
        }
    }
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary { entries: BTreeMap::new() }
    }

    /// Retrieve a dictionary from the specified reader.
    pub fn load<T>(input: T) -> Result<Self, ProgramError>
    where
        T: std::io::Read,
    {
        Ok(serde_json::from_reader(input)?)
    }

    /// Store a dictionary to the specified writer.
    pub fn store<T>(&self, output: &mut T) -> Result<(), ProgramError>
    where
        T: std::io::Write,
    {
        Ok(serde_json::to_writer_pretty(output, self)?)
    }

    /// Import a set of words into a dictionary, returning the number of entries added.
    pub fn import_wordlist<T>(&mut self, wordlist: T) -> Result<usize, ProgramError>
    where
        T: std::io::Read,
    {
        let mut synced: usize = 0;

        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(true)
            .from_reader(wordlist);

        for record in reader.records().flatten() {
            if let (Some(root), Some(category)) = (record.get(0), record.get(1)) {
                if !self.entries.contains_key(root) {
                    self.entries.insert(
                        root.to_string(),
                        DictionaryEntry {
                            // TODO: Convert to ProgramError
                            category: Category::from_str(category).unwrap(),
                            translations: vec![],
                        },
                    );
                    synced += 1;
                }
            }
        }

        Ok(synced)
    }

    /// TODO: Testing! How do we test this? It'll be nasty, but we have to.
    pub async fn update_definitions(&mut self) -> Result<usize, ProgramError> {
        let client = reqwest::Client::new();
        let mut updated: usize = 0;

        for (root, entry) in &mut self.entries {
            if entry.translations.is_empty() {
                entry.translations.append(&mut search(&client, &root).await?);
                updated += 1;
            }
        }

        Ok(updated)
    }
}

/// Look up a root word in the Online Icelandic Dictionary and return
/// a vector of definitions for the root.
pub async fn search(
    client: &reqwest::Client,
    word: &str,
) -> Result<Vec<Translation>, ProgramError> {
    println!("Searching Icelandic online dictionary for definitions: {}", word);

    let url = format!("https://digicoll.library.wisc.edu/cgi-bin/IcelOnline/IcelOnline.TEId-idx?type=simple&size=First+100&rgn=lemma&q1={}&submit=Search", word);

    // Dictionary entries may contain a slash or numbers for disambiguation.
    // These must be removed before comparison.
    let lemma_re = Regex::new(r"[\d/]+").unwrap();
    let res = client.get(url).send().await?.text().await?;
    let document = Document::from(res.borrow());

    match document.select(Class("results")).next() {
        Some(result_list) => {
            let lemma = result_list
                .select(Class("lemma"))
                .find(|n| lemma_re.replace_all(&n.text(), "") == word)
                .unwrap();
            // TODO: Error handling!
            let link = lemma.select(Name("a")).next().unwrap().attr("href").unwrap();
            let res = client
                .get(format!("https://digicoll.library.wisc.edu{}", link))
                .send()
                .await?
                .text()
                .await?;
            let document = Document::from(res.borrow());
            Ok(get_translations(&document))
        }

        None => Ok(get_translations(&document)),
    }
}

/// Return definitions contained in a dictionary entry page.
fn get_translations(document: &Document) -> Vec<Translation> {
    std::thread::sleep(Duration::from_secs(1));

    document
        .select(Class("entry").child(Class("sense")))
        .map(|n| {
            let usg = n.select(Class("usg")).next();
            let trans = n.select(Class("trans")).next();
            match (trans, usg) {
                (Some(trans), Some(usg)) => Some(Translation::new(trans.text(), Some(usg.text()))),
                (Some(trans), None) => Some(Translation::new(trans.text(), None)),
                (_, _) => None,
            }
        })
        .flatten()
        .collect::<Vec<Translation>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn stores_and_loads_dictionary() {
        let mut serialized = Vec::new();

        let mut stored_dictionary = Dictionary::new();

        stored_dictionary.entries.insert(
            "foo".to_owned(),
            DictionaryEntry {
                category: Category::Adjective,
                translations: vec![Translation {
                    meaning: "A Word".to_owned(),
                    usage: Some("A Usage".to_owned()),
                }],
            },
        );
        stored_dictionary.entries.insert(
            "bar".to_string(),
            DictionaryEntry {
                category: Category::Adjective,
                translations: vec![Translation {
                    meaning: "Another Word".to_string(),
                    usage: None,
                }],
            },
        );

        let _ = stored_dictionary.store(&mut serialized).unwrap();

        let json = std::str::from_utf8(serialized.as_slice()).unwrap().to_string();

        let loaded_dictionary = Dictionary::load(json.as_bytes()).unwrap();

        assert_eq!(loaded_dictionary, stored_dictionary);
    }

    #[test]
    pub fn loads_new_words_into_dictionary() {
        let wordlist = "foo\tnouns\nbar\tadjectives\nbaz\tnouns\nquux\tadjectives".as_bytes();

        let mut dictionary = Dictionary::new();

        assert!(dictionary.entries.is_empty());

        let synced = dictionary.import_wordlist(wordlist).unwrap();
        assert_eq!(4, synced);
        assert!(dictionary.entries.contains_key("foo"));
        assert!(dictionary.entries.contains_key("bar"));
        assert!(dictionary.entries.contains_key("baz"));
        assert!(dictionary.entries.contains_key("quux"));
    }

    #[test]
    pub fn does_not_overwrite_words_in_dictionary() {
        let wordlist = "foo\tnouns\nbar\tadjectives\nbaz\tnouns\nquux\tadjectives".as_bytes();

        let mut dictionary = Dictionary::new();

        dictionary.entries.insert(
            "baz".to_owned(),
            DictionaryEntry {
                category: Category::Noun,
                translations: vec![Translation {
                    meaning: "The definition of baz".to_string(),
                    usage: Some("Some Usage".to_string()),
                }],
            },
        );

        assert_eq!(1, dictionary.entries.len());

        let synced = dictionary.import_wordlist(wordlist).unwrap();
        // Only three synced out of four
        assert_eq!(3, synced);
        assert!(dictionary.entries.contains_key("foo"));
        assert!(dictionary.entries.contains_key("bar"));
        assert!(dictionary.entries.contains_key("baz"));
        assert!(dictionary.entries.contains_key("quux"));

        assert_eq!(
            "The definition of baz",
            dictionary.entries.get("baz").unwrap().translations.get(0).unwrap().meaning
        );
    }
}
