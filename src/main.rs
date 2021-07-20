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

const DEFAULT_DECK_NAME: &str = "Icelandic Vocabulary";
const DEFAULT_DECK_DESCRIPTION: &str = "Deck for studying Icelandic Vocabulary";
const DEFAULT_BIN_CSV: &str = "SHsnid.csv";
const DEFAULT_DECK: &str = "deck.apkg";
const BIN_CSV_URL: &str = "https://bin.arnastofnun.is/django/api/nidurhal/?file=SHsnid.csv.zip";
const NOUN_MODEL_ID: usize = 1625673414000;
const ADJECTIVE_MODEL_ID: usize = 1625673414010;
const VERB_MODEL_ID: usize = 1625673414020;
const ADVERB_MODEL_ID: usize = 1625673414030;
const PHRASE_MODEL_ID: usize = 1625673414040;
const PRONOUN_MODEL_ID: usize = 1625673414050;
const INDEFINITE_PRONOUN_MODEL_ID: usize = 1625673414060;
const DECK_ID: usize = 1625673415000;

const CSS: &str = r#".card {
  font-family: Helvetica, Arial, sans-serif;
  font-size: 20px;
  text-align: center;
  color: black;
  background-color: white;
}
table {
  border-collapse: collapse;
  margin-left: auto;
  margin-right: auto;
  width: 100%;
}
td {
  border: 1px solid #ccc;
  padding: 6px;
}
th {
  font-style: italic;
  padding: 8px;
}
.wclass {
  color: #009900;
  font-weight: bold;
  font-style: italic;
}
.definition {
  border: 1px solid black;
  padding: 12px;
  background: #eee;
  font-size: 120%;
  font-weight: bold;
  color: #000099;
}
.vp {
  color: #999;
  font-style: italic;
  font-weight: normal;
}
.ncl {
  width: 12%;
}
.acl {
  width: 13%;
}
.nfh,.nfm {
  width: 44%;
}
.nfm {
  font-weight: bold;
  font-size: 120%;
}
.pfm {
  width: 88%;
  font-weight: bold;
  font-size: 120%;
}
.afh,.afm {
  width: 29%;
}
.afm {
  font-weight: bold;
  font-size: 120%;
}
.vfm {
  width: 50%;
  font-weight: bold;
  font-size: 120%;
}"#;

const NOUN_TMPL: &str = r#"{{FrontSide}}
<p class="wclass">Noun — {{Gender}}</p>
<p class="definition">
 {{Definition}}
</p>
<h3>Indefinite</h3>
<table>
 <tr>
  <th class="ncl"></th>
  <th class="nfh">Singular</th>
  <th class="nfh">Plural</th>
 </tr>
 <tr>
  <th class="ncl">nom.</th>
  <td class="nfm"><span class="vp">hér er</span> {{Nominative Singular}}</td>
  <td class="nfm"><span class="vp">hér eru</span> {{Nominative Plural}}</td>
 </tr>
 <tr>
  <th class="ncl">acc.</th>
  <td class="nfm"><span class="vp">um</span> {{Accusative Singular}}</td>
  <td class="nfm"><span class="vp">um</span> {{Accusative Plural}}</td>
 </tr>
 <tr>
  <th class="ncl">dat.</th>
  <td class="nfm"><span class="vp">frá</span> {{Dative Singular}}</td>
  <td class="nfm"><span class="vp">frá</span> {{Dative Plural}}</td>
 </tr>
 <tr>
  <th class="ncl">gen.</th>
  <td class="nfm"><span class="vp">til</span> {{Genitive Singular}}</td>
  <td class="nfm"><span class="vp">til</span> {{Genitive Plural}}</td>
 </tr>
</table>
<h3>Definite</h3>
<table>
 <tr>
  <th class="ncl"></th>
  <th class="nfh">Singular</th>
  <th class="nfh">Plural</th>
 </tr>
 <tr>
  <th class="ncl">nom.</th>
  <td class="nfm"><span class="vp">hér er</span> {{Nominative Singular Definite}}</td>
  <td class="nfm"><span class="vp">hér eru</span> {{Nominative Plural Definite}}</td>
 </tr>
 <tr>
  <th class="ncl">acc.</th>
  <td class="nfm"><span class="vp">um</span> {{Accusative Singular Definite}}</td>
  <td class="nfm"><span class="vp">um</span> {{Accusative Plural Definite}}</td>
 </tr>
 <tr>
  <th class="ncl">dat.</th>
  <td class="nfm"><span class="vp">frá</span> {{Dative Singular Definite}}</td>
  <td class="nfm"><span class="vp">frá</span> {{Dative Plural Definite}}</td>
 </tr>
 <tr>
  <th class="ncl">gen.</th>
  <td class="nfm"><span class="vp">til</span> {{Genitive Singular Definite}}</td>
  <td class="nfm"><span class="vp">til</span> {{Genitive Plural Definite}}</td>
 </tr>
</table>"#;

const ADJ_TMPL: &str = r#"{{FrontSide}}
<p class="wclass">{{Word Class}}</p>
<p class="definition">{{Definition}}</p>
<h3>Singular</h3>
<table>
 <tr>
  <th class="acl"></th>
  <th class="afh">masc.</th>
  <th class="afh">fem.</th>
  <th class="afh">neut.</th>
 </tr>
 <tr>
  <th class="acl">nom.</th>
  <td class="afm">{{Masculine Singular Nominative}}</td>
  <td class="afm">{{Feminine Singular Nominative}}</td>
  <td class="afm">{{Neuter Singular Nominative}}</td>
 </tr>
 <tr>
  <th class="acl">acc.</th>
  <td class="afm">{{Masculine Singular Accusative}}</td>
  <td class="afm">{{Feminine Singular Accusative}}</td>
  <td class="afm">{{Neuter Singular Accusative}}</td>
 </tr>
 <tr>
  <th class="acl">dat.</th>
  <td class="afm">{{Masculine Singular Dative}}</td>
  <td class="afm">{{Feminine Singular Dative}}</td>
  <td class="afm">{{Neuter Singular Dative}}</td>
 </tr>
 <tr>
  <th class="acl">gen.</th>
  <td class="afm">{{Masculine Singular Genitive}}</td>
  <td class="afm">{{Feminine Singular Genitive}}</td>
  <td class="afm">{{Neuter Singular Genitive}}</td>
 </tr>
</table>
<h3>Plural</h3>
<table>
 <tr>
  <th class="acl"></th>
  <th class="afh">masc.</th>
  <th class="afh">fem.</th>
  <th class="afh">neut.</th>
 </tr>
 <tr>
  <th class="acl">nom.</th>
  <td class="afm">{{Masculine Plural Nominative}}</td>
  <td class="afm">{{Feminine Plural Nominative}}</td>
  <td class="afm">{{Neuter Plural Nominative}}</td>
 </tr>
 <tr>
  <th class="acl">acc.</th>
  <td class="afm">{{Masculine Plural Accusative}}</td>
  <td class="afm">{{Feminine Plural Accusative}}</td>
  <td class="afm">{{Neuter Plural Accusative}}</td>
 </tr>
 <tr>
  <th class="acl">dat.</th>
  <td class="afm">{{Masculine Plural Dative}}</td>
  <td class="afm">{{Feminine Plural Dative}}</td>
  <td class="afm">{{Neuter Plural Dative}}</td>
 </tr>
 <tr>
  <th class="acl">gen.</th>
  <td class="afm">{{Masculine Plural Genitive}}</td>
  <td class="afm">{{Feminine Plural Genitive}}</td>
  <td class="afm">{{Neuter Plural Genitive}}</td>
 </tr>
</table>"#;

const VERB_TMPL: &str = r#"{{FrontSide}}
<p class="wclass">Verb</p>
<p class="definition">{{Definition}}</p>
<h3>Present Indicative</h3>
<table>
 <tr>
  <td class="vfm">
   <span class="vp">ég</span> {{Present 1st Singular}}
  </td>
  <td class="vfm">
   <span class="vp">við</span> {{Present 1st Plural}}
  </td>
 </tr>
 <tr>
  <td class="vfm">
   <span class="vp">þú</span> {{Present 2nd Singular}}
  </td>
  <td class="vfm">
   <span class="vp">þið</span> {{Present 2nd Plural}}
  </td>
 </tr>
 <tr>
  <td class="vfm">
   <span class="vp">hann/hún/það</span> {{Present 3rd Singular}}
  </td>
  <td class="vfm">
   <span class="vp">þeir/þær/þau</span> {{Present 3rd Plural}}
  </td>
 </tr>
</table>
<h3>Past Indicative</h3>
<table>
 <tr>
  <td class="vfm">
   <span class="vp">ég</span> {{Past 1st Singular}}
  </td>
  <td class="vfm">
   <span class="vp">við</span> {{Past 1st Plural}}
  </td>
 </tr>
 <tr>
  <td class="vfm">
   <span class="vp">þú</span> {{Past 2nd Singular}}
  </td>
  <td class="vfm">
   <span class="vp">þið</span> {{Past 2nd Plural}}
  </td>
 </tr>
 <tr>
  <td class="vfm">
   <span class="vp">hann/hún/það</span> {{Past 3rd Singular}}
  </td>
  <td class="vfm">
   <span class="vp">þeir/þær/þau</span> {{Past 3rd Plural}}
  </td>
 </tr>
</table>"#;

const ADVERB_TMPL: &str = r#"{{FrontSide}}
<p class="wclass">Phrase</p>
<p class="definition">{{Definition}}</p>"#;

const PHRASE_TMPL: &str = r#"{{FrontSide}}
<p class="wclass">Phrase</p>
<p class="definition">{{Definition}}</p>"#;

const PRONOUN_TMPL: &str = r#"{{FrontSide}}
<p class="wclass">Pronoun</p>
<p class="definition">{{Definition}}</p>
<table>
 <tr>
  <th class="acl">nom.</th>
  <td class="pfm">{{Nominative}}</td>
 </tr>
 <tr>
  <th class="acl">acc.</th>
  <td class="pfm">{{Accusative}}</td>
 </tr>
 <tr>
  <th class="acl">dat.</th>
  <td class="pfm">{{Dative}}</td>
 </tr>
 <tr>
  <th class="acl">gen.</th>
  <td class="pfm">{{Genitive}}</td>
 </tr>
</table>"#;

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

fn common_fields() -> Vec<Field> {
    vec![
        Field::new("Root"),
        Field::new("Word Class"),
        Field::new("Definition"),
        Field::new("Masculine Singular Nominative"),
        Field::new("Feminine Singular Nominative"),
        Field::new("Neuter Singular Nominative"),
        Field::new("Masculine Singular Accusative"),
        Field::new("Feminine Singular Accusative"),
        Field::new("Neuter Singular Accusative"),
        Field::new("Masculine Singular Dative"),
        Field::new("Feminine Singular Dative"),
        Field::new("Neuter Singular Dative"),
        Field::new("Masculine Singular Genitive"),
        Field::new("Feminine Singular Genitive"),
        Field::new("Neuter Singular Genitive"),
        Field::new("Masculine Plural Nominative"),
        Field::new("Feminine Plural Nominative"),
        Field::new("Neuter Plural Nominative"),
        Field::new("Masculine Plural Accusative"),
        Field::new("Feminine Plural Accusative"),
        Field::new("Neuter Plural Accusative"),
        Field::new("Masculine Plural Dative"),
        Field::new("Feminine Plural Dative"),
        Field::new("Neuter Plural Dative"),
        Field::new("Masculine Plural Genitive"),
        Field::new("Feminine Plural Genitive"),
        Field::new("Neuter Plural Genitive"),
    ]
}

fn generate_deck(
    dictionary: &Dictionary,
    bin_data: &BinData,
    config: &AppConfig,
) -> Result<Deck, ProgramError> {
    let mut deck = Deck::new(DECK_ID, &config.deck_name, &config.deck_description);

    let adjective_model = Model::new_with_options(
        ADJECTIVE_MODEL_ID,
        "Icelandic Adjective",
        common_fields(),
        vec![Template::new("Icelandic Adjective").qfmt("<h1>{{Root}}</h1>").afmt(ADJ_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let indef_pronoun_model = Model::new_with_options(
        INDEFINITE_PRONOUN_MODEL_ID,
        "Icelandic Indefinite Pronoun",
        common_fields(),
        vec![Template::new("Icelandic Indefinite Pronoun")
            .qfmt("<h1>{{Root}}</h1>")
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
            Field::new("Root"),
            Field::new("Gender"),
            Field::new("Definition"),
            Field::new("Nominative Singular"),
            Field::new("Nominative Singular Definite"),
            Field::new("Accusative Singular"),
            Field::new("Accusative Singular Definite"),
            Field::new("Dative Singular"),
            Field::new("Dative Singular Definite"),
            Field::new("Genitive Singular"),
            Field::new("Genitive Singular Definite"),
            Field::new("Nominative Plural"),
            Field::new("Nominative Plural Definite"),
            Field::new("Accusative Plural"),
            Field::new("Accusative Plural Definite"),
            Field::new("Dative Plural"),
            Field::new("Dative Plural Definite"),
            Field::new("Genitive Plural"),
            Field::new("Genitive Plural Definite"),
        ],
        vec![Template::new("Icelandic Noun").qfmt("<h1>{{Root}}</h1>").afmt(NOUN_TMPL)],
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
            Field::new("Root"),
            Field::new("Definition"),
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
        ],
        vec![Template::new("Icelandic Verb").qfmt("<h1>að {{Root}}</h1>").afmt(VERB_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let adverb_model = Model::new_with_options(
        ADVERB_MODEL_ID,
        "Icelandic Adverb",
        vec![Field::new("Root"), Field::new("Definition")],
        vec![Template::new("Icelandic Adverb").qfmt("<h1>{{Root}}</h1>").afmt(ADVERB_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let phrase_model = Model::new_with_options(
        PHRASE_MODEL_ID,
        "Icelandic Phrase",
        vec![Field::new("Root"), Field::new("Definition")],
        vec![Template::new("Icelandic Phrase").qfmt("<h1>{{Root}}</h1>").afmt(PHRASE_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    let pronoun_model = Model::new_with_options(
        PRONOUN_MODEL_ID,
        "Icelandic Pronoun",
        vec![
            Field::new("Root"),
            Field::new("Definition"),
            Field::new("Nominative"),
            Field::new("Accusative"),
            Field::new("Dative"),
            Field::new("Genitive"),
        ],
        vec![Template::new("Icelandic Pronoun").qfmt("<h1>{{Root}}</h1>").afmt(PRONOUN_TMPL)],
        Some(CSS),
        None,
        None,
        None,
        None,
    );

    for (key, definition) in &dictionary.entries {
        let root = &key.root;

        let note = match key.category {
            Category::Noun => noun(&root, bin_data, definition, &noun_model),
            Category::Adjective => adjective(&root, bin_data, definition, &adjective_model),
            Category::Verb => verb(&root, bin_data, definition, &verb_model),
            Category::Adverb => simple_note(&root, definition, &adverb_model),
            Category::Phrase => simple_note(&root, definition, &phrase_model),
            Category::Pronoun => pronoun(&root, bin_data, definition, &pronoun_model),
            Category::IndefinitePronoun => {
                indefinite_pronoun(&root, bin_data, definition, &indef_pronoun_model)
            }
        };

        match note {
            Some(note) => deck.add_note(note),
            None => println!(
                "WARNING: No entry found for root {}, category {:?}. Skipping.",
                &root, key.category
            ),
        }
    }

    Ok(deck)
}

fn indefinite_pronoun(
    root: &str,
    bin_data: &BinData,
    definition: &str,
    model: &Model,
) -> Option<Note> {
    match bin_data.indefinite_pronoun(root) {
        Some(entry) => Some(
            Note::new(
                model.clone(),
                vec![
                    root,
                    "Indefinite Pronoun",
                    definition,
                    &entry.masc_nom_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.masc_acc_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.masc_dat_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.masc_gen_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_nom_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_acc_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_dat_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_gen_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_nom_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_acc_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_dat_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_gen_sg.unwrap_or_else(|| "—".to_string()),
                    &entry.masc_nom_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.masc_acc_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.masc_dat_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.masc_gen_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_nom_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_acc_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_dat_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.fem_gen_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_nom_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_acc_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_dat_pl.unwrap_or_else(|| "—".to_string()),
                    &entry.neut_gen_pl.unwrap_or_else(|| "—".to_string()),
                ],
            )
            .unwrap(),
        ),
        _ => None,
    }
}

fn adjective(root: &str, bin_data: &BinData, definition: &str, model: &Model) -> Option<Note> {
    match bin_data.adjective(root) {
        Some(adjective_entry) => Some(
            Note::new(
                model.clone(),
                vec![
                    root,
                    "Adjective",
                    definition,
                    &adjective_entry.masc_nom_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_nom_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_nom_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_acc_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_acc_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_acc_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_dat_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_dat_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_dat_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_gen_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_gen_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_gen_sg_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_nom_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_nom_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_nom_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_acc_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_acc_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_acc_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_dat_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_dat_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_dat_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.masc_gen_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.fem_gen_pl_strong.unwrap_or_else(|| "—".to_string()),
                    &adjective_entry.neut_gen_pl_strong.unwrap_or_else(|| "—".to_string()),
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
                    match noun_entry.gender {
                        Gender::Masculine => "Masculine",
                        Gender::Feminine => "Feminine",
                        Gender::Neuter => "Neuter",
                    },
                    definition,
                    &noun_entry.nom_sg.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.nom_sg_def.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.acc_sg.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.acc_sg_def.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.dat_sg.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.dat_sg_def.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.gen_sg.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.gen_sg_def.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.nom_pl.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.nom_pl_def.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.acc_pl.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.acc_pl_def.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.dat_pl.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.dat_pl_def.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.gen_pl.unwrap_or_else(|| "—".to_string()),
                    &noun_entry.gen_pl_def.unwrap_or_else(|| "—".to_string()),
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
                    definition,
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
                ],
            )
            .unwrap(),
        ),
        _ => None,
    }
}

fn pronoun(root: &str, bin_data: &BinData, definition: &str, model: &Model) -> Option<Note> {
    match bin_data.pronoun(root) {
        Some(pronoun_entry) => Some(
            Note::new(
                model.clone(),
                vec![
                    root,
                    definition,
                    &pronoun_entry.nom.unwrap_or_else(|| "—".to_string()),
                    &pronoun_entry.acc.unwrap_or_else(|| "—".to_string()),
                    &pronoun_entry.dat.unwrap_or_else(|| "—".to_string()),
                    &pronoun_entry.gen.unwrap_or_else(|| "—".to_string()),
                ],
            )
            .unwrap(),
        ),
        _ => None,
    }
}

fn simple_note(root: &str, definition: &str, model: &Model) -> Option<Note> {
    Some(Note::new(model.clone(), vec![root, definition]).unwrap())
}

/// Read application config from command line arguments.
fn app_config(project_dirs: &ProjectDirs) -> AppConfig {
    let arg_matches = App::new("Icelandic Anki Flashcard Generator")
        .version("1.0")
        .author("Seth Morabito")
        .arg(
            Arg::with_name("output")
                .help("Anki deck output file")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .default_value("deck.apkg")
                .required(false),
        )
        .arg(
            Arg::with_name("name")
                .help("Anki deck name")
                .long("name")
                .value_name("NAME")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("description")
                .help("Anki deck description")
                .long("description")
                .value_name("DESCRIPTION")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("wordlist")
                .help("List of words, categories, and definitions (tab separated)")
                .required(true),
        )
        .get_matches();

    let bin_data: PathBuf = project_dirs.data_dir().join(DEFAULT_BIN_CSV);

    let output: String = match arg_matches.value_of("output") {
        Some(deck) => deck.to_string(),
        None => DEFAULT_DECK.to_string(),
    };

    let deck_name: String = match arg_matches.value_of("name") {
        Some(name) => name.to_string(),
        None => DEFAULT_DECK_NAME.to_string(),
    };

    let deck_description: String = match arg_matches.value_of("description") {
        Some(description) => description.to_string(),
        None => DEFAULT_DECK_DESCRIPTION.to_string(),
    };

    let wordlist: PathBuf = match arg_matches.value_of("wordlist") {
        Some(wordlist) => Path::new(wordlist).to_path_buf(),
        None => Path::new("wordlist.txt").to_path_buf(),
    };

    AppConfig { bin_data, output, deck_name, deck_description, wordlist }
}

#[derive(Debug)]
struct AppConfig {
    bin_data: PathBuf,
    output: String,
    deck_name: String,
    deck_description: String,
    wordlist: PathBuf,
}

fn setup_project_dirs(project_dirs: &ProjectDirs) -> Result<(), ProgramError> {
    std::fs::create_dir_all(project_dirs.data_dir())?;
    std::fs::create_dir_all(project_dirs.config_dir())?;

    Ok(())
}

async fn get_bin_csv(app_config: &AppConfig) -> Result<(), ProgramError> {
    let mut tmp_file = tempfile()?;

    println!("Downloading BIN data from URL {:?}...", BIN_CSV_URL);

    let response = reqwest::get(BIN_CSV_URL).await?;
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
            deck.write_to_file(&config.output)?;

            println!("Done!");
        }
        None => println!("Cannot access default application storage directory. Giving up."),
    }

    Ok(())
}
