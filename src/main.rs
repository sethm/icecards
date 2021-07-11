use crate::bindata::{BinData, Gender};
use crate::dictionary::{Category, Dictionary};
use clap::{App, Arg};
use directories_next::ProjectDirs;
use genanki_rs::{Deck, Field, Model, Note, Template};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::tempfile;
use thiserror::Error;
use zip::result::ZipError;

mod bindata;
mod dictionary;

const DEFAULT_BIN_CSV: &str = "SHsnid.csv";
const DEFAULT_DECK: &str = "deck.apkg";
const BIN_CSV_URL: &str = "https://bin.arnastofnun.is/django/api/nidurhal/?file=SHsnid.csv.zip";
const NOUN_MODEL_ID: usize = 1625673414000;
const ADJECTIVE_MODEL_ID: usize = 1625673414010;
const VERB_MODEL_ID: usize = 1625673414020;
const DECK_ID: usize = 1625673414030;

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

const NOUN_TMPL: &str = r#"{{FrontSide}}
<hr id="gender"/>
<h2>{{Gender}}</h2>
<hr id="forms"/>
<h2><em>g. sg.</em> {{Genitive Singular}}, <em>n. pl.</em> {{Nominative Plural}}</h2>
<p>{{Definition}}</p>"#;

const ADJ_TMPL: &str = r#"{{FrontSide}}
<hr id="forms">
<table>
 <tr>
  <th></th><th>m.</th><th>f.</th><th>n.</th>
 </tr>
 <tr>
  <th>Sg.</th><td>{{Masculine Singular}}</td><td>{{Feminine Singular}}</td><td>{{Neuter Singular}}</td>
 </tr>
 <tr>
  <th>Pl.</th><td>{{Masculine Plural}}</td><td>{{Feminine Plural}}</td><td>{{Neuter Plural}}</td>
 </tr>
</table>
<hr id="definition">
<p>{{Definition}}</p>"#;

const VERB_TMPL: &str = r#"{{FrontSide}}
<hr id="pres">
<h3>Present Indicative</h3>
<table>
 <tr>
  <td>ég {{Present 1st Singular}}</td><td>við {{Present 1st Plural}}</td>
 </tr>
 <tr>
  <td>þú {{Present 2nd Singular}}</td><td>þið {{Present 2nd Plural}}</td>
 </tr>
 <tr>
  <td>hann/hún/það {{Present 3rd Singular}}</td><td>þeir/þær/þau {{Present 3rd Plural}}</td>
 </tr>
</table>
<h3>Past Indicative</h3>
<table>
 <tr>
  <td>ég {{Past 1st Singular}}</td><td>við {{Past 1st Plural}}</td>
 </tr>
 <tr>
  <td>þú {{Past 2nd Singular}}</td><td>þið {{Past 2nd Plural}}</td>
 </tr>
 <tr>
  <td>hann/hún/það {{Past 3rd Singular}}</td><td>þeir/þær/þau {{Past 3rd Plural}}</td>
 </tr>
</table>
<hr id="definition">
<p>{{Definition}}</p>"#;

#[derive(Error, Debug)]
pub enum ProgramError {
    #[error("cannot access configuration")]
    Configuration,
    #[error("invalid dictionary file")]
    Dictionary,
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("zip error")]
    Zip(#[from] ZipError),
    #[error("bin data file does not exist")]
    BinData,
    #[error("CSV parse failed")]
    Csv(#[from] csv::Error),
    #[error("Anki Generation")]
    Anki(#[from] genanki_rs::Error),
}

fn generate_deck(
    dictionary: &Dictionary,
    bin_data: &BinData,
    config: &AppConfig,
) -> Result<Deck, ProgramError> {
    let mut deck =
        Deck::new(config.deck_id, "Icelandic Vocabulary", "Deck for studying Icelandic Vocabulary");

    let adjective_model = Model::new_with_options(
        ADJECTIVE_MODEL_ID,
        "Icelandic Adjective",
        vec![
            Field::new("Masculine Singular"),
            Field::new("Feminine Singular"),
            Field::new("Neuter Singular"),
            Field::new("Masculine Plural"),
            Field::new("Feminine Plural"),
            Field::new("Neuter Plural"),
            Field::new("Definition"),
        ],
        vec![Template::new("Icelandic Adjective")
            .qfmt("<h1>{{Masculine Singular}}</h1>")
            .afmt(ADJ_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let noun_model = Model::new_with_options(
        NOUN_MODEL_ID,
        "Icelandic Noun",
        vec![
            Field::new("Nominative Singular"),
            Field::new("Genitive Singular"),
            Field::new("Nominative Plural"),
            Field::new("Gender"),
            Field::new("Definition"),
        ],
        vec![Template::new("Icelandic Noun")
            .qfmt("<h1>{{Nominative Singular}}</h1>")
            .afmt(NOUN_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let verb_model = Model::new_with_options(
        VERB_MODEL_ID,
        "Icelandic Verb",
        vec![
            Field::new("Infinitive"),
            Field::new("Present 1st Singular"),
            Field::new("Present 2nd Singular"),
            Field::new("Present 3rd Singular"),
            Field::new("Present 1st Plural"),
            Field::new("Present 2nd Plural"),
            Field::new("Present 3rd Plural"),
            Field::new("Past 1st Singular"),
            Field::new("Past 2nd Singular"),
            Field::new("Past 3rd Singular"),
            Field::new("Past 1st Plural"),
            Field::new("Past 2nd Plural"),
            Field::new("Past 3rd Plural"),
            Field::new("Definition"),
        ],
        vec![Template::new("Icelandic Verb").qfmt("<h1>að {{Infinitive}}</h1>").afmt(VERB_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    for (key, definition) in &dictionary.entries {
        let root = &key.root;
        match key.category {
            Category::Noun => {
                if let Some(note) = noun(&root, bin_data, definition, &noun_model) {
                    deck.add_note(note)
                }
            }
            Category::Adjective => {
                if let Some(note) = adjective(&root, bin_data, definition, &adjective_model) {
                    deck.add_note(note)
                }
            }
            Category::Verb => {
                if let Some(note) = verb(&root, bin_data, definition, &verb_model) {
                    deck.add_note(note)
                }
            }
        }
    }

    Ok(deck)
}

fn adjective(root: &str, bin_data: &BinData, definition: &str, model: &Model) -> Option<Note> {
    match bin_data.adjective(root) {
        Some(adjective_entry) => Some(
            Note::new(
                model.clone(),
                vec![
                    &adjective_entry.masc_nom_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_nom_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_nom_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_nom_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_nom_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_nom_pl_strong.unwrap_or_else(|| "—".to_string()),
                    definition,
                ],
            )
            .unwrap(),
        ),
        _ => None,
    }
}

fn noun(root: &str, bin_data: &BinData, definition: &str, model: &Model) -> Option<Note> {
    match bin_data.noun(root) {
        Some(noun_entry) => Some(
            Note::new(
                model.clone(),
                vec![
                    root,
                    &noun_entry.gen_sg.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.nom_pl.unwrap_or_else(|| "—".to_string()),
                    match noun_entry.gender {
                        Gender::Masculine => "Masculine",
                        Gender::Feminine => "Feminine",
                        Gender::Neuter => "Neuter",
                    },
                    definition,
                ],
            )
            .unwrap(),
        ),
        _ => None,
    }
}

fn verb(root: &str, bin_data: &BinData, definition: &str, model: &Model) -> Option<Note> {
    match bin_data.verb(root) {
        Some(verb_entry) => Some(
            Note::new(
                model.clone(),
                vec![
                    root,
                    &verb_entry.pres_ind_first_sg.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.pres_ind_second_sg.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.pres_ind_third_sg.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.pres_ind_first_pl.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.pres_ind_second_pl.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.pres_ind_third_pl.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.past_ind_first_sg.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.past_ind_second_sg.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.past_ind_third_sg.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.past_ind_first_pl.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.past_ind_second_pl.unwrap_or_else(|| "—".to_string()),
                    &verb_entry.past_ind_third_pl.unwrap_or_else(|| "—".to_string()),
                    definition,
                ],
            )
            .unwrap(),
        ),
        _ => None,
    }
}

/// Read application config from command line arguments.
fn app_config(project_dirs: &ProjectDirs) -> AppConfig {
    let arg_matches = App::new("Icelandic Anki Flashcard Generator")
        .version("1.0")
        .author("Seth Morabito")
        .arg(
            Arg::with_name("binurl")
                .help("URL to fetch BÍN CSV")
                .long("binurl")
                .value_name("URL")
                .takes_value(true)
                .default_value(BIN_CSV_URL)
                .required(false),
        )
        .arg(
            Arg::with_name("deck")
                .help("Anki Deck output file")
                .short("d")
                .long("deck")
                .value_name("FILE")
                .default_value("deck.apkg")
                .takes_value(true),
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
            Arg::with_name("wordlist")
                .help("List of words and categories, tab separated, one per line")
                .required(true),
        )
        .get_matches();

    let dictionary = project_dirs.data_dir().join("dictionary.json");
    let bin_data: PathBuf = project_dirs.data_dir().join(DEFAULT_BIN_CSV);

    let bin_csv_url = match arg_matches.value_of("binurl") {
        Some(binurl) => binurl.to_string(),
        None => BIN_CSV_URL.to_string(),
    };

    let deck: String = match arg_matches.value_of("deck") {
        Some(deck) => deck.to_string(),
        None => DEFAULT_DECK.to_string(),
    };

    let wordlist: PathBuf = match arg_matches.value_of("wordlist") {
        Some(wordlist) => Path::new(wordlist).to_path_buf(),
        None => Path::new("wordlist.txt").to_path_buf(),
    };

    let deck_id = match arg_matches.value_of("deck-id") {
        Some(id) => id.parse::<usize>().unwrap(),
        None => DECK_ID,
    };

    AppConfig { bin_csv_url, bin_data, dictionary, deck, wordlist, deck_id }
}

#[derive(Debug)]
struct AppConfig {
    bin_csv_url: String,
    bin_data: PathBuf,
    dictionary: PathBuf,
    deck: String,
    wordlist: PathBuf,
    deck_id: usize,
}

fn setup_project_dirs(project_dirs: &ProjectDirs) -> Result<(), ProgramError> {
    let data_dir = project_dirs.data_dir();
    let config_dir = project_dirs.config_dir();
    std::fs::create_dir_all(data_dir)?;
    std::fs::create_dir_all(config_dir)?;

    Ok(())
}

async fn get_bin_csv(app_config: &AppConfig) -> Result<(), ProgramError> {
    let mut tmp_file = tempfile()?;

    println!("Downloading BIN data from URL {:?}...", &app_config.bin_csv_url);

    let response = reqwest::get(&app_config.bin_csv_url).await?;
    let content = response.bytes().await?;

    tmp_file.write_all(content.as_ref())?;

    println!("Extracting ZIP file to {:?}...", &app_config.bin_data);

    let mut archive = zip::ZipArchive::new(tmp_file)?;
    let mut file = archive.by_name(DEFAULT_BIN_CSV)?;
    let mut outfile = File::create(&app_config.bin_data)?;
    io::copy(&mut file, &mut outfile)?;

    Ok(())
}

/// Ensure that the BIN CSV data file exists locally. If it does not exist,
/// it will be downloaded and unzipped automatically.
///
/// # Arguments
///
/// * `config` - The application config.
///
async fn ensure_bin_data_exists(config: &AppConfig) -> Result<(), ProgramError> {
    if config.bin_data.exists() {
        return Ok(());
    }

    println!("===============================================================================");
    println!("The required BIN data file {} does not exist. It can be downloaded", DEFAULT_BIN_CSV);
    println!("automatically for you, or you may download it and unzip it yourself.");
    println!();
    println!("The compressed download is about 35 MB, and the uncompressed file uses about");
    println!("325 MB of disk space.");
    println!();
    println!("This download only needs to occur once. The file will be saved as:");
    println!("  {:?}", config.bin_data);
    println!("===============================================================================");
    println!();
    print!("Continue with download? [y/N]: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input)?;

    if input.trim().to_ascii_lowercase().starts_with('y') {
        get_bin_csv(&config).await?;
        Ok(())
    } else {
        Err(ProgramError::BinData)
    }
}

#[tokio::main]
async fn main() -> Result<(), ProgramError> {
    // Establish directories for holding state
    match ProjectDirs::from("com", "loomcom", "is-anki-gen") {
        Some(project_dirs) => {
            let config = app_config(&project_dirs);

            // If the word list doesn't exist, bail immediately.
            if !config.wordlist.exists() {
                println!("Word list file {:?} does not exist.", config.wordlist);
                return Err(ProgramError::Configuration);
            }

            setup_project_dirs(&project_dirs)?;

            if let Err(e) = ensure_bin_data_exists(&config).await {
                match e {
                    ProgramError::BinData => {
                        println!("BIN file not downloaded or found locally.");
                    }
                    _ => {
                        println!("Couldn't download BIN file: {:?}", e);
                    }
                }
                println!("Good bye!");
                return Err(e);
            }

            let dictionary = Dictionary::load(File::open(&config.wordlist)?)?;

            println!("Loading BIN Data...");
            let bin_data_file = File::open(&config.bin_data)?;
            let bin_data = BinData::load(bin_data_file)?;

            println!("Starting Anki deck generation...");
            let deck = generate_deck(&dictionary, &bin_data, &config)?;

            println!("Saving Anki deck...");
            deck.write_to_file(&config.deck)?;

            println!("Done!");
        }
        None => println!("Cannot access default application storage directory. Giving up."),
    }

    Ok(())
}
