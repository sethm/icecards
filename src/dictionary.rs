use crate::ProgramError;
use csv::ReaderBuilder;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Category {
    Noun,
    Adjective,
    Verb,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noun" | "nouns" => Ok(Category::Noun),
            "adjective" | "adjectives" => Ok(Category::Adjective),
            "verb" | "verbs" => Ok(Category::Verb),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DictionaryKey {
    pub root: String,
    pub category: Category,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dictionary {
    pub entries: BTreeMap<DictionaryKey, String>,
}

impl Dictionary {
    /// Import a set of words into a dictionary, returning the number of entries added.
    pub fn load<T>(wordlist: T) -> Result<Self, ProgramError>
    where
        T: std::io::Read,
    {
        let mut dictionary = Dictionary { entries: BTreeMap::new() };

        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .flexible(true)
            .from_reader(wordlist);

        for record in reader.records().flatten() {
            if let (Some(root), Some(category)) = (record.get(0), record.get(1)) {
                let key = DictionaryKey {
                    root: root.to_string(),
                    category: Category::from_str(category).unwrap(),
                };

                let definition = record.get(2).unwrap_or("—");

                dictionary.entries.insert(key, definition.to_string());
            }
        }

        Ok(dictionary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn loads_dictionary() {
        let wordlist = r#"foo	noun	definition of foo
bar	verb	definition of bar
baz	adjective	definition of baz"#
            .as_bytes();

        let dictionary = Dictionary::load(wordlist).unwrap();

        assert_eq!(
            "definition of foo",
            dictionary
                .entries
                .get(&DictionaryKey { root: "foo".to_string(), category: Category::Noun })
                .unwrap()
        );

        assert_eq!(
            "definition of bar",
            dictionary
                .entries
                .get(&DictionaryKey { root: "bar".to_string(), category: Category::Verb })
                .unwrap()
        );

        assert_eq!(
            "definition of baz",
            dictionary
                .entries
                .get(&DictionaryKey { root: "baz".to_string(), category: Category::Adjective })
                .unwrap()
        );

        // Shouldn't find a non-existent entry
        assert!(dictionary
            .entries
            .get(&DictionaryKey { root: "baz".to_string(), category: Category::Noun })
            .is_none());
    }
}
