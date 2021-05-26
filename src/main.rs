use clap::{arg_enum, value_t, App, Arg};
use genanki_rs::{Deck, Error, Field, Model, Note, Template};
use rand::prelude::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::SystemTime;

const CSS: &'static str = "\
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

const ADJ_TMPL: &'static str = r#"{{FrontSide}}
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
<p>{{Definition}}</p>
"#;

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum Category {
        Nouns,
        Adjectives,
    }
}

struct Adjective {
    definition: String,
    masc_singular: Option<String>,
    fem_singular: Option<String>,
    neut_singular: Option<String>,
    masc_plural: Option<String>,
    fem_plural: Option<String>,
    neut_plural: Option<String>,
}

impl Adjective {
    fn new(definition: &str) -> Self {
        Adjective {
            definition: String::from(definition),
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
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis() as usize
        + delta)
}

/// Consume a tab-separated file, each line of which must be in the
/// format "<word>\t<definition>", and return a map between the two.
fn build_map(file: &str) -> HashMap<String, String> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(file)
        .unwrap();

    reader
        .records()
        .map(|r| {
            let record = r.unwrap();
            let root = record.get(0).unwrap();
            let definition = record.get(1).unwrap();
            (root.to_owned(), definition.to_owned())
        })
        .collect()
}

/// Build an Anki deck based on adjectives
fn adjectives(word_list: &str, csv_file: &str) -> Result<Deck, Error> {
    let adj_map = build_map(word_list);

    let mut deck = Deck::new(
        random_id()?,
        "Icelandic Adjectives",
        "Deck for studying Icelandic adjectives",
    );

    let model = Model::new_with_options(
        random_id()?,
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
        vec![Template::new("Card 1")
            .qfmt("<h1>{{MascSg}}</h1>")
            .afmt(ADJ_TMPL)],
        Some(CSS.clone()),
        None,
        None,
        None,
        None,
    );

    let mut db_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_file)?;

    let mut adjective_cards: HashMap<String, Adjective> = HashMap::new();

    for result in db_reader.records() {
        let record = result?;
        let root = record.get(0).unwrap();
        let key = record.get(2).unwrap();

        if adj_map.contains_key(root) && key == "lo" {
            if !adjective_cards.contains_key(root) {
                let definition = adj_map.get(root).unwrap();
                adjective_cards.insert(String::from(root), Adjective::new(definition));
            }

            let card = adjective_cards.get_mut(root).unwrap();

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

    // Now map over the adjectives and build cards.
    for (key, val) in adjective_cards {
        println!("adding {}...", key);

        let note = Note::new(
            model.clone(),
            vec![
                val.masc_singular.unwrap_or(String::from("")).borrow(),
                val.fem_singular.unwrap_or(String::from("")).borrow(),
                val.neut_singular.unwrap_or(String::from("")).borrow(),
                val.masc_plural.unwrap_or(String::from("")).borrow(),
                val.fem_plural.unwrap_or(String::from("")).borrow(),
                val.neut_plural.unwrap_or(String::from("")).borrow(),
                val.definition.borrow(),
            ],
        );

        deck.add_note(note.unwrap());
    }

    Ok(deck)
}

/// Build an Anki deck based on nouns
fn nouns(word_list: &str, csv_file: &str) -> Result<Deck, Error> {
    let noun_map = build_map(word_list);

    let mut deck = Deck::new(
        random_id()?,
        "Icelandic Noun Plurals",
        "Deck for studying Icelandic noun plurals",
    );

    let model = Model::new_with_options(
        random_id()?,
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
        Some(CSS.clone()),
        None,
        None,
        None,
        None,
    );

    let mut db_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path(csv_file)?;

    for result in db_reader.records() {
        let record = result?;
        let form = record.get(5).unwrap();
        let class = record.get(3).unwrap();

        if "alm" == class && "NFFT" == form {
            if let Some(root) = record.get(0) {
                if noun_map.contains_key(root) {
                    println!("adding {}...", root);
                    let gender = match record.get(2).unwrap() {
                        "kk" => "masc.",
                        "hk" => "neut.",
                        "kvk" => "fem.",
                        _ => "...",
                    };
                    let plural = record.get(4).unwrap();
                    let definition = noun_map.get(root).unwrap();
                    let note = Note::new(model.clone(), vec![root, plural, gender, definition]);
                    deck.add_note(note.unwrap());
                }
            }
        }
    }

    Ok(deck)
}

fn main() -> Result<(), Error> {
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
            Arg::with_name("wordlist")
                .help("List of words and definitions, tab separated, one per line")
                .required(true),
        )
        .get_matches();

    let csv_file = matches.value_of("bindata").unwrap();
    let deck_file = matches.value_of("deck").unwrap();
    let cat: Category = value_t!(matches, "category", Category).unwrap();
    let input_file = matches.value_of("wordlist").unwrap();

    let deck = match cat {
        Category::Nouns => nouns(input_file, csv_file)?,
        Category::Adjectives => adjectives(input_file, csv_file)?,
    };

    deck.write_to_file(deck_file)?;
    Ok(())
}
