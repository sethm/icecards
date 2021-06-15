use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::time::SystemTime;

use crate::dictionary::Definition;
use clap::{arg_enum, value_t, App, Arg};
use csv::{ReaderBuilder, WriterBuilder};
use genanki_rs::{Deck, Error, Field, Model, Note, Template};
use rand::prelude::*;

mod dictionary;

const CSS: &str = "\
.card {\
  font-family: arial;\
  font-size: 20px; \
  text-align: center;\
  color: black;\
  background-color: white;\
}\
table {\
  border-collapse: collapse;\
  margin-left: auto;\
  margin-right: auto;\
  width: 100%;\
}\
td {\
  border: 1px solid #ccc;\
  padding: 6px;\
}";

const ADJ_TMPL: &str = r#"{{FrontSide}}
<hr id="forms">
<table>
 <tr>
  <th></th><th>m.</th><th>f.</th><th>n.</th>
 </tr>
 <tr>
  <th>Sg.</th><td>{{MascSg}}</td><td>{{FemSg}}</td><td>{{NeutSg}}</td>
 </tr>
 <tr>
  <th>Pl.</th><td>{{MascPl}}</td><td>{{FemPl}}</td><td>{{NeutPl}}</td>
 </tr>
</table>
<hr id="definition">
<p>{{Definition}}</p>"#;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Category {
        Nouns,
        Adjectives,
    }
}

struct Adjective {
    definition: Definition,
    masc_singular: Option<String>,
    fem_singular: Option<String>,
    neut_singular: Option<String>,
    masc_plural: Option<String>,
    fem_plural: Option<String>,
    neut_plural: Option<String>,
}

impl Adjective {
    fn new(definition: Definition) -> Self {
        Adjective {
            definition,
            masc_singular: None,
            fem_singular: None,
            neut_singular: None,
            masc_plural: None,
            fem_plural: None,
            neut_plural: None,
        }
    }
}

/// Return a somewhat, kind-of, more-or-less random ID for an Anki record.
fn random_id() -> Result<usize, Error> {
    let mut rng = thread_rng();
    let delta: usize = rng.gen_range(0..100);
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as usize + delta)
}

async fn read_word_map(file_name: &str) -> BTreeMap<String, Definition> {
    let client = reqwest::Client::new();
    let mut result: BTreeMap<String, Definition> = BTreeMap::new();

    if let Ok(mut reader) =
        ReaderBuilder::new().has_headers(false).delimiter(b'\t').flexible(true).from_path(file_name)
    {
        for record in reader.records().flatten() {
            if let Some(root) = record.get(0) {
                // TODO: Error Handling
                let definition = match record.get(1) {
                    Some(json) => serde_json::from_str::<Definition>(json).unwrap(),
                    None => dictionary::search(&client, &root).await.unwrap(),
                };

                result.insert(root.to_owned(), definition);
            }
        }
    }

    result
}

fn update_word_map(file_name: &str, word_map: &BTreeMap<String, Definition>) {
    if let Ok(mut writer) =
        WriterBuilder::new().has_headers(false).delimiter(b'\t').from_path(file_name)
    {
        word_map.iter().for_each(|(k, v)| {
            let definition = serde_json::to_string(v).unwrap();
            let _ = writer.write_record(&[k, &definition]);
        });
    }
}

/// Build an Anki deck based on adjectives
fn adjectives(
    word_map: &BTreeMap<String, Definition>,
    csv_file: &str,
    deck_id: usize,
    model_id: usize,
) -> Result<Deck, Error> {
    let mut deck =
        Deck::new(deck_id, "Icelandic Adjectives", "Deck for studying Icelandic adjectives");

    let model = Model::new_with_options(
        model_id,
        "Icelandic Adjectives",
        vec![
            Field::new("MascSg"),
            Field::new("FemSg"),
            Field::new("NeutSg"),
            Field::new("MascPl"),
            Field::new("FemPl"),
            Field::new("NeutPl"),
            Field::new("Definition"),
        ],
        vec![Template::new("Card 1").qfmt("<h1>{{MascSg}}</h1>").afmt(ADJ_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let mut db_reader =
        ReaderBuilder::new().has_headers(false).delimiter(b';').from_path(csv_file)?;

    let mut adjective_cards: BTreeMap<String, Adjective> = BTreeMap::new();

    for result in db_reader.records() {
        let record = result?;
        let root = record.get(0).unwrap();
        let key = record.get(2).unwrap();

        if word_map.contains_key(root) && key == "lo" {
            if !adjective_cards.contains_key(root) {
                if let Some(definition) = word_map.get(root) {
                    adjective_cards.insert(String::from(root), Adjective::new(definition.clone()));
                }
            }

            if let Some(card) = adjective_cards.get_mut(root) {
                let form = record.get(5).unwrap();
                let decl = record.get(4).unwrap().to_owned();

                match form {
                    "FSB-KK-NFET" => card.masc_singular = Some(decl),
                    "FSB-KVK-NFET" => card.fem_singular = Some(decl),
                    "FSB-HK-NFET" => card.neut_singular = Some(decl),
                    "FSB-KK-NFFT" => card.masc_plural = Some(decl),
                    "FSB-KVK-NFFT" => card.fem_plural = Some(decl),
                    "FSB-HK-NFFT" => card.neut_plural = Some(decl),
                    _ => {}
                }
            }
        }
    }

    // Now map over the adjectives and build cards.
    for (_, val) in adjective_cards {
        let note = Note::new(
            model.clone(),
            vec![
                val.masc_singular.unwrap_or_else(String::new).borrow(),
                val.fem_singular.unwrap_or_else(String::new).borrow(),
                val.neut_singular.unwrap_or_else(String::new).borrow(),
                val.masc_plural.unwrap_or_else(String::new).borrow(),
                val.fem_plural.unwrap_or_else(String::new).borrow(),
                val.neut_plural.unwrap_or_else(String::new).borrow(),
                &val.definition.to_string(),
            ],
        );

        deck.add_note(note.unwrap());
    }

    Ok(deck)
}

/// Build an Anki deck based on nouns
fn nouns(
    word_map: &BTreeMap<String, Definition>,
    csv_file: &str,
    deck_id: usize,
    model_id: usize,
) -> Result<Deck, Error> {
    let mut deck =
        Deck::new(deck_id, "Icelandic Noun Plurals", "Deck for studying Icelandic noun plurals");

    let model = Model::new_with_options(
        model_id,
        "Noun Plurals",
        vec![
            Field::new("Singular"),
            Field::new("Plural"),
            Field::new("Gender"),
            Field::new("Definition"),
        ],
        vec![Template::new("Card 1")
            .qfmt("<h1>{{Singular}}</h1>")
            .afmt(r#"{{FrontSide}}<hr id="plural"><h2>{{Plural}} ({{Gender}})</h2> <p>{{Definition}}</p>"#)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let mut db_reader =
        ReaderBuilder::new().has_headers(false).delimiter(b';').from_path(csv_file)?;

    for result in db_reader.records() {
        let record = result?;
        let form = record.get(5).unwrap();
        let class = record.get(3).unwrap();

        if "alm" == class && "NFFT" == form {
            if let Some(root) = record.get(0) {
                if word_map.contains_key(root) {
                    let gender = match record.get(2).unwrap() {
                        "kk" => "masc.",
                        "hk" => "neut.",
                        "kvk" => "fem.",
                        _ => "...",
                    };
                    let plural = record.get(4).unwrap();
                    if let Some(definition) = word_map.get(root) {
                        let note = Note::new(
                            model.clone(),
                            vec![root, plural, gender, &definition.to_string()],
                        );
                        deck.add_note(note.unwrap());
                    }
                }
            }
        }
    }

    Ok(deck)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("BÍN Scraper")
        .version("1.0")
        .author("Seth Morabito")
        .arg(
            Arg::with_name("bindata")
                .help("BÍN CSV File")
                .short("b")
                .long("bindata")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("deck")
                .help("Anki Deck output file")
                .short("d")
                .long("deck")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("category")
                .help("Wordlist file category ('nouns' or 'adjectives')")
                .long("category")
                .value_name("CATEGORY")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("deck-id")
                .help("Optional numeric ID for the generated Anki deck")
                .long("deck-id")
                .value_name("ID")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("model-id")
                .help("Optional numeric ID for the generated Anki model")
                .long("model-id")
                .value_name("ID")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("wordlist")
                .help("List of words and definitions, tab separated, one per line")
                .required(true),
        )
        .get_matches();

    let csv_file = matches.value_of("bindata").unwrap();
    let deck_file = matches.value_of("deck").unwrap();
    let category: Category = value_t!(matches, "category", Category).unwrap();
    let input_file = matches.value_of("wordlist").unwrap();
    let deck_id = match matches.value_of("deck-id") {
        Some(id) => id.parse::<usize>().unwrap(),
        None => random_id().unwrap(),
    };
    let model_id = match matches.value_of("model-id") {
        Some(id) => id.parse::<usize>().unwrap(),
        None => random_id().unwrap(),
    };

    println!("Generating Anki deck with id `{}`, model id `{}`.", deck_id, model_id);

    println!("Loading {}...", input_file);
    let word_map = read_word_map(input_file).await;

    // TODO: A builder pattern would probably be nice here.
    println!("Starting Anki deck generation...");
    let deck = match category {
        Category::Nouns => nouns(&word_map, csv_file, deck_id, model_id)?,
        Category::Adjectives => adjectives(&word_map, csv_file, deck_id, model_id)?,
    };

    println!("Saving Anki deck...");
    deck.write_to_file(deck_file)?;

    // TODO: Back up original file
    println!("Updating {}...", input_file);
    update_word_map(input_file, &word_map);

    println!("Done!");
    Ok(())
}
