use clap::{App, Arg};
use genanki_rs::{Deck, Error, Field, Model, Note, Template};
use rand::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;

const CSS: &'static str = ".card { font-family: arial; font-size: 20px; text-align: center; color: black; background-color: white; }";

fn random_id() -> Result<usize, Error> {
    let mut rng = thread_rng();
    let delta: usize = rng.gen_range(0..100);
    Ok(SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis() as usize
        + delta)
}

fn plural_nouns(word_list: &str, csv_file: &str) -> Result<Deck, Error> {
    let mut noun_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(word_list)?;

    let mut noun_map: HashMap<String, String> = HashMap::new();

    for result in noun_reader.records() {
        let record = result?;
        let root = record.get(0).unwrap();
        let definition = record.get(1).unwrap_or("");
        noun_map.insert(String::from(root), String::from(definition));
    }

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
            Arg::with_name("wordlist")
                .help("List of words and definitions, tab separated, one per line")
                .short("w")
                .long("wordlist")
                .value_name("FILE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("csv")
                .help("BÍN CSV File")
                .short("c")
                .long("csv")
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
        .get_matches();

    let input_file = matches.value_of("wordlist").unwrap();
    let csv_file = matches.value_of("csv").unwrap();
    let deck_file = matches.value_of("deck").unwrap();

    let deck = plural_nouns(input_file, csv_file)?;
    deck.write_to_file(deck_file)?;
    Ok(())
}
