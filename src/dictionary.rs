use std::borrow::Borrow;
use std::time::Duration;

use genanki_rs::Error;
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use serde::{Deserialize, Serialize};

use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Translation {
    pub meaning: String,
    pub usage: Option<String>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Definition {
    pub translations: Vec<Translation>,
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

impl Display for Definition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.translations.len() {
            1 => {
                write!(f, "{}", &self.translations.get(0).unwrap())
            }
            _ => {
                write!(
                    f,
                    "{}",
                    &self
                        .translations
                        .iter()
                        .enumerate()
                        .map(|(i, t)| format!("{}. {}", i + 1, t))
                        .collect::<Vec<String>>()
                        .join("; ")
                )
            }
        }
    }
}

impl Translation {
    pub fn new(translation: String, usage: Option<String>) -> Self {
        Translation { meaning: translation, usage }
    }
}

/// Look up a root word in the Online Icelandic Dictionary and return
/// a vector of definitions for the root.
pub async fn search(client: &reqwest::Client, word: &str) -> Result<Definition, Error> {
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
            Ok(Definition { translations: get_translations(&document) })
        }

        None => Ok(Definition { translations: get_translations(&document) }),
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
