use crate::ProgramError;
use csv::ReaderBuilder;
use std::collections::BTreeMap;
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
pub struct BinEntry {
    pub id: u64,
    pub word_class: String,
    pub classification: String,
    pub form: String,
    pub tag: String,
}

impl BinEntry {
    fn is_adjective(&self) -> bool {
        self.word_class == "lo"
    }

    fn is_noun(&self) -> bool {
        self.word_class == "kk" || self.word_class == "kvk" || self.word_class == "hk"
    }

    fn is_verb(&self) -> bool {
        self.word_class == "so"
    }

    fn is_indefinite_pronoun(&self) -> bool {
        self.word_class == "fn"
    }

    fn is_number(&self) -> bool {
        self.word_class == "to"
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, Eq, PartialEq)]
pub struct VerbEntry {
    pub pres_ind_first_sg: Option<String>,
    pub pres_ind_second_sg: Option<String>,
    pub pres_ind_third_sg: Option<String>,
    pub pres_ind_first_pl: Option<String>,
    pub pres_ind_second_pl: Option<String>,
    pub pres_ind_third_pl: Option<String>,
    pub past_ind_first_sg: Option<String>,
    pub past_ind_second_sg: Option<String>,
    pub past_ind_third_sg: Option<String>,
    pub past_ind_first_pl: Option<String>,
    pub past_ind_second_pl: Option<String>,
    pub past_ind_third_pl: Option<String>,
    // Many more fields could go here. Icelandic conjugations are huge.
    // TODO: Subjunctive mood, mediopassive voice, Past Participle, Imperative, etc.
}

#[derive(Debug, Eq, PartialEq)]
pub struct NounEntry {
    pub gender: Gender,
    pub nom_sg: Option<String>,
    pub acc_sg: Option<String>,
    pub dat_sg: Option<String>,
    pub gen_sg: Option<String>,
    pub nom_pl: Option<String>,
    pub acc_pl: Option<String>,
    pub dat_pl: Option<String>,
    pub gen_pl: Option<String>,
    pub nom_sg_def: Option<String>,
    pub acc_sg_def: Option<String>,
    pub dat_sg_def: Option<String>,
    pub gen_sg_def: Option<String>,
    pub nom_pl_def: Option<String>,
    pub acc_pl_def: Option<String>,
    pub dat_pl_def: Option<String>,
    pub gen_pl_def: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct AdjectiveEntry {
    pub masc_nom_sg_strong: Option<String>,
    pub masc_acc_sg_strong: Option<String>,
    pub masc_dat_sg_strong: Option<String>,
    pub masc_gen_sg_strong: Option<String>,
    pub fem_nom_sg_strong: Option<String>,
    pub fem_acc_sg_strong: Option<String>,
    pub fem_dat_sg_strong: Option<String>,
    pub fem_gen_sg_strong: Option<String>,
    pub neut_nom_sg_strong: Option<String>,
    pub neut_acc_sg_strong: Option<String>,
    pub neut_dat_sg_strong: Option<String>,
    pub neut_gen_sg_strong: Option<String>,
    pub masc_nom_pl_strong: Option<String>,
    pub masc_acc_pl_strong: Option<String>,
    pub masc_dat_pl_strong: Option<String>,
    pub masc_gen_pl_strong: Option<String>,
    pub fem_nom_pl_strong: Option<String>,
    pub fem_acc_pl_strong: Option<String>,
    pub fem_dat_pl_strong: Option<String>,
    pub fem_gen_pl_strong: Option<String>,
    pub neut_nom_pl_strong: Option<String>,
    pub neut_acc_pl_strong: Option<String>,
    pub neut_dat_pl_strong: Option<String>,
    pub neut_gen_pl_strong: Option<String>,
    pub masc_nom_sg_weak: Option<String>,
    pub masc_acc_sg_weak: Option<String>,
    pub masc_dat_sg_weak: Option<String>,
    pub masc_gen_sg_weak: Option<String>,
    pub fem_nom_sg_weak: Option<String>,
    pub fem_acc_sg_weak: Option<String>,
    pub fem_dat_sg_weak: Option<String>,
    pub fem_gen_sg_weak: Option<String>,
    pub neut_nom_sg_weak: Option<String>,
    pub neut_acc_sg_weak: Option<String>,
    pub neut_dat_sg_weak: Option<String>,
    pub neut_gen_sg_weak: Option<String>,
    pub masc_nom_pl_weak: Option<String>,
    pub masc_acc_pl_weak: Option<String>,
    pub masc_dat_pl_weak: Option<String>,
    pub masc_gen_pl_weak: Option<String>,
    pub fem_nom_pl_weak: Option<String>,
    pub fem_acc_pl_weak: Option<String>,
    pub fem_dat_pl_weak: Option<String>,
    pub fem_gen_pl_weak: Option<String>,
    pub neut_nom_pl_weak: Option<String>,
    pub neut_acc_pl_weak: Option<String>,
    pub neut_dat_pl_weak: Option<String>,
    pub neut_gen_pl_weak: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PronounEntry {
    pub nom: Option<String>,
    pub acc: Option<String>,
    pub dat: Option<String>,
    pub gen: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NumberEntry {
    pub masc_nom: Option<String>,
    pub masc_acc: Option<String>,
    pub masc_dat: Option<String>,
    pub masc_gen: Option<String>,
    pub fem_nom: Option<String>,
    pub fem_acc: Option<String>,
    pub fem_dat: Option<String>,
    pub fem_gen: Option<String>,
    pub neut_nom: Option<String>,
    pub neut_acc: Option<String>,
    pub neut_dat: Option<String>,
    pub neut_gen: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IndefinitePronounEntry {
    pub masc_nom_sg: Option<String>,
    pub masc_acc_sg: Option<String>,
    pub masc_dat_sg: Option<String>,
    pub masc_gen_sg: Option<String>,
    pub fem_nom_sg: Option<String>,
    pub fem_acc_sg: Option<String>,
    pub fem_dat_sg: Option<String>,
    pub fem_gen_sg: Option<String>,
    pub neut_nom_sg: Option<String>,
    pub neut_acc_sg: Option<String>,
    pub neut_dat_sg: Option<String>,
    pub neut_gen_sg: Option<String>,
    pub masc_nom_pl: Option<String>,
    pub masc_acc_pl: Option<String>,
    pub masc_dat_pl: Option<String>,
    pub masc_gen_pl: Option<String>,
    pub fem_nom_pl: Option<String>,
    pub fem_acc_pl: Option<String>,
    pub fem_dat_pl: Option<String>,
    pub fem_gen_pl: Option<String>,
    pub neut_nom_pl: Option<String>,
    pub neut_acc_pl: Option<String>,
    pub neut_dat_pl: Option<String>,
    pub neut_gen_pl: Option<String>,
}

pub struct BinData {
    pub data: BTreeMap<String, Vec<BinEntry>>,
}

impl BinData {
    pub fn load<T>(reader: T) -> Result<Box<Self>, ProgramError>
    where
        T: Read,
    {
        let mut bin_data = Box::new(BinData { data: BTreeMap::new() });

        let mut db_reader =
            ReaderBuilder::new().has_headers(false).delimiter(b';').from_reader(reader);

        for result in db_reader.records() {
            let record = result?;

            let lemma = record.get(0).unwrap().to_string();
            let id = record.get(1).unwrap().parse::<u64>().unwrap();
            let word_class = record.get(2).unwrap().to_string();
            let classification = record.get(3).unwrap().to_string();
            let form = record.get(4).unwrap().to_string();
            let tag = record.get(5).unwrap().to_string();

            let entry = bin_data.data.entry(lemma).or_insert_with(Vec::new);
            entry.push(BinEntry { id, word_class, classification, form, tag });
        }

        Ok(bin_data)
    }

    pub fn pronoun(&self, root: &str) -> Option<PronounEntry> {
        // Personal pronouns require some special handling.
        let (entries, tag) = match root {
            "ég" => (self.data.get("ég"), Some("FET")),
            "við" => (self.data.get("ég"), Some("FFT")),
            "þú" => (self.data.get("þú"), Some("FET")),
            "þið" => (self.data.get("þú"), Some("FFT")),
            "hann" => (self.data.get("hann"), Some("FET")),
            "þeir" => (self.data.get("hann"), Some("FFT")),
            "hún" => (self.data.get("hún"), Some("FET")),
            "þær" => (self.data.get("hún"), Some("FFT")),
            "það" => (self.data.get("það"), Some("FET")),
            "þau" => (self.data.get("það"), Some("FFT")),
            _ => (None, None),
        };

        match (entries, tag) {
            (Some(entries), Some(tag)) => {
                if entries.is_empty() {
                    None
                } else {
                    Some(PronounEntry {
                        nom: entries
                            .iter()
                            .find(|&e| e.tag == format!("N{}", tag))
                            .map(|e| e.form.to_string()),
                        acc: entries
                            .iter()
                            .find(|&e| e.tag == format!("Þ{}", tag))
                            .map(|e| e.form.to_string()),
                        dat: entries
                            .iter()
                            .find(|&e| e.tag == format!("ÞG{}", tag))
                            .map(|e| e.form.to_string()),
                        gen: entries
                            .iter()
                            .find(|&e| e.tag == format!("E{}", tag))
                            .map(|e| e.form.to_string()),
                    })
                }
            }
            (_, _) => None,
        }
    }

    pub fn number(&self, root: &str) -> Option<NumberEntry> {
        let entries = self.data.get(root);

        match entries {
            Some(entries) => {
                let entries = entries.iter().filter(|&e| e.is_number()).collect::<Vec<&BinEntry>>();

                if entries.is_empty() {
                    None
                } else {
                    Some(NumberEntry {
                        masc_nom: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-NFET" || e.tag == "KK-NFFT")
                            .map(|&e| e.form.to_string()),
                        masc_acc: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-ÞFET" || e.tag == "KK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-ÞGFET" || e.tag == "KK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        masc_gen: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-EFET" || e.tag == "KK-EFFT")
                            .map(|&e| e.form.to_string()),
                        fem_nom: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-NFET" || e.tag == "KVK-NFFT")
                            .map(|&e| e.form.to_string()),
                        fem_acc: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-ÞFET" || e.tag == "KVK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-ÞGFET" || e.tag == "KVK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        fem_gen: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-EFET" || e.tag == "KVK-EFFT")
                            .map(|&e| e.form.to_string()),
                        neut_nom: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-NFET" || e.tag == "HK-NFFT")
                            .map(|&e| e.form.to_string()),
                        neut_acc: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-ÞFET" || e.tag == "HK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-ÞGFET" || e.tag == "HK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        neut_gen: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-EFET" || e.tag == "HK-EFFT")
                            .map(|&e| e.form.to_string()),
                    })
                }
            }
            None => None,
        }
    }

    pub fn indefinite_pronoun(&self, root: &str) -> Option<IndefinitePronounEntry> {
        let entries = self.data.get(root);

        match entries {
            Some(entries) => {
                let entries = entries
                    .iter()
                    .filter(|&e| e.is_indefinite_pronoun())
                    .collect::<Vec<&BinEntry>>();

                if entries.is_empty() {
                    None
                } else {
                    Some(IndefinitePronounEntry {
                        masc_nom_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-NFET")
                            .map(|&e| e.form.to_string()),
                        masc_acc_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        masc_dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        masc_gen_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-EFET")
                            .map(|&e| e.form.to_string()),
                        fem_nom_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-NFET")
                            .map(|&e| e.form.to_string()),
                        fem_acc_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        fem_dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        fem_gen_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-EFET")
                            .map(|&e| e.form.to_string()),
                        neut_nom_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-NFET")
                            .map(|&e| e.form.to_string()),
                        neut_acc_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        neut_dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        neut_gen_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-EFET")
                            .map(|&e| e.form.to_string()),
                        masc_nom_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-NFFT")
                            .map(|&e| e.form.to_string()),
                        masc_acc_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        masc_gen_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-EFFT")
                            .map(|&e| e.form.to_string()),
                        fem_nom_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-NFFT")
                            .map(|&e| e.form.to_string()),
                        fem_acc_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        fem_gen_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-EFFT")
                            .map(|&e| e.form.to_string()),
                        neut_nom_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-NFFT")
                            .map(|&e| e.form.to_string()),
                        neut_acc_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        neut_gen_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-EFFT")
                            .map(|&e| e.form.to_string()),
                    })
                }
            }
            None => None,
        }
    }

    pub fn adjective(&self, root: &str) -> Option<AdjectiveEntry> {
        let entries = self.data.get(root);

        match entries {
            Some(entries) => {
                let entries =
                    entries.iter().filter(|&e| e.is_adjective()).collect::<Vec<&BinEntry>>();

                if entries.is_empty() {
                    None
                } else {
                    Some(AdjectiveEntry {
                        masc_nom_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-NFET")
                            .map(|&e| e.form.to_string()),
                        masc_acc_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        masc_dat_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        masc_gen_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-EFET")
                            .map(|&e| e.form.to_string()),
                        fem_nom_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-NFET")
                            .map(|&e| e.form.to_string()),
                        fem_acc_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        fem_dat_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        fem_gen_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-EFET")
                            .map(|&e| e.form.to_string()),
                        neut_nom_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-NFET")
                            .map(|&e| e.form.to_string()),
                        neut_acc_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        neut_dat_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        neut_gen_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-EFET")
                            .map(|&e| e.form.to_string()),
                        masc_nom_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-NFFT")
                            .map(|&e| e.form.to_string()),
                        masc_acc_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        masc_gen_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-EFFT")
                            .map(|&e| e.form.to_string()),
                        fem_nom_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-NFFT")
                            .map(|&e| e.form.to_string()),
                        fem_acc_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        fem_gen_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-EFFT")
                            .map(|&e| e.form.to_string()),
                        neut_nom_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-NFFT")
                            .map(|&e| e.form.to_string()),
                        neut_acc_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        neut_gen_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-EFFT")
                            .map(|&e| e.form.to_string()),
                        masc_nom_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-NFET")
                            .map(|&e| e.form.to_string()),
                        masc_acc_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        masc_dat_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        masc_gen_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-EFET")
                            .map(|&e| e.form.to_string()),
                        fem_nom_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-NFET")
                            .map(|&e| e.form.to_string()),
                        fem_acc_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        fem_dat_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        fem_gen_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-EFET")
                            .map(|&e| e.form.to_string()),
                        neut_nom_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-NFET")
                            .map(|&e| e.form.to_string()),
                        neut_acc_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-ÞFET")
                            .map(|&e| e.form.to_string()),
                        neut_dat_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-ÞGFET")
                            .map(|&e| e.form.to_string()),
                        neut_gen_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-EFET")
                            .map(|&e| e.form.to_string()),
                        masc_nom_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-NFFT")
                            .map(|&e| e.form.to_string()),
                        masc_acc_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        masc_gen_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-EFFT")
                            .map(|&e| e.form.to_string()),
                        fem_nom_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-NFFT")
                            .map(|&e| e.form.to_string()),
                        fem_acc_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        fem_gen_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-EFFT")
                            .map(|&e| e.form.to_string()),
                        neut_nom_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-NFFT")
                            .map(|&e| e.form.to_string()),
                        neut_acc_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-ÞFFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        neut_gen_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-EFFT")
                            .map(|&e| e.form.to_string()),
                    })
                }
            }
            None => None,
        }
    }

    pub fn noun(&self, root: &str) -> Option<NounEntry> {
        let entries = self.data.get(root);

        match entries {
            Some(entries) => {
                let entries = entries.iter().filter(|&e| e.is_noun()).collect::<Vec<&BinEntry>>();

                if entries.is_empty() {
                    None
                } else {
                    let gender = entries.first().unwrap().word_class.as_str();
                    Some(NounEntry {
                        gender: match gender {
                            "kvk" => Gender::Feminine,
                            "hk" => Gender::Neuter,
                            _ => Gender::Masculine,
                        },
                        nom_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "NFET")
                            .map(|&e| e.form.to_string()),
                        acc_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞFET")
                            .map(|&e| e.form.to_string()),
                        dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞGFET")
                            .map(|&e| e.form.to_string()),
                        gen_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "EFET")
                            .map(|&e| e.form.to_string()),
                        nom_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "NFFT")
                            .map(|&e| e.form.to_string()),
                        acc_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞFFT")
                            .map(|&e| e.form.to_string()),
                        dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞGFFT")
                            .map(|&e| e.form.to_string()),
                        gen_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "EFFT")
                            .map(|&e| e.form.to_string()),
                        nom_sg_def: entries
                            .iter()
                            .find(|&&e| e.tag == "NFETgr")
                            .map(|&e| e.form.to_string()),
                        acc_sg_def: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞFETgr")
                            .map(|&e| e.form.to_string()),
                        dat_sg_def: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞGFETgr")
                            .map(|&e| e.form.to_string()),
                        gen_sg_def: entries
                            .iter()
                            .find(|&&e| e.tag == "EFETgr")
                            .map(|&e| e.form.to_string()),
                        nom_pl_def: entries
                            .iter()
                            .find(|&&e| e.tag == "NFFTgr")
                            .map(|&e| e.form.to_string()),
                        acc_pl_def: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞFFTgr")
                            .map(|&e| e.form.to_string()),
                        dat_pl_def: entries
                            .iter()
                            .find(|&&e| e.tag == "ÞGFFTgr")
                            .map(|&e| e.form.to_string()),
                        gen_pl_def: entries
                            .iter()
                            .find(|&&e| e.tag == "EFFTgr")
                            .map(|&e| e.form.to_string()),
                    })
                }
            }
            None => None,
        }
    }

    pub fn verb(&self, root: &str) -> Option<VerbEntry> {
        let entries = self.data.get(root);

        match entries {
            Some(entries) => {
                let entries = entries.iter().filter(|&e| e.is_verb()).collect::<Vec<&BinEntry>>();

                if entries.is_empty() {
                    None
                } else {
                    Some(VerbEntry {
                        pres_ind_first_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-NT-1P-ET")
                            .map(|&e| e.form.to_string()),
                        pres_ind_second_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-NT-2P-ET")
                            .map(|&e| e.form.to_string()),
                        pres_ind_third_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-NT-3P-ET")
                            .map(|&e| e.form.to_string()),
                        pres_ind_first_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-NT-1P-FT")
                            .map(|&e| e.form.to_string()),
                        pres_ind_second_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-NT-2P-FT")
                            .map(|&e| e.form.to_string()),
                        pres_ind_third_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-NT-3P-FT")
                            .map(|&e| e.form.to_string()),
                        // Past Indicative
                        past_ind_first_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-ÞT-1P-ET")
                            .map(|&e| e.form.to_string()),
                        past_ind_second_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-ÞT-2P-ET")
                            .map(|&e| e.form.to_string()),
                        past_ind_third_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-ÞT-3P-ET")
                            .map(|&e| e.form.to_string()),
                        past_ind_first_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-ÞT-1P-FT")
                            .map(|&e| e.form.to_string()),
                        past_ind_second_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-ÞT-2P-FT")
                            .map(|&e| e.form.to_string()),
                        past_ind_third_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-ÞT-3P-FT")
                            .map(|&e| e.form.to_string()),
                    })
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "aðalhellir;74631;kk;alm;aðalhellir;NFET
aðalhellir;74631;kk;alm;aðalhellirinn;NFETgr
aðalhellir;74631;kk;alm;aðalhelli;ÞFET
aðalhellir;74631;kk;alm;aðalhellinn;ÞFETgr
aðalhellir;74631;kk;alm;aðalhelli;ÞGFET
aðalhellir;74631;kk;alm;aðalhellinum;ÞGFETgr
aðalhellir;74631;kk;alm;aðalhellis;EFET
aðalhellir;74631;kk;alm;aðalhellisins;EFETgr
aðalhellir;74631;kk;alm;aðalhellar;NFFT
aðalhellir;74631;kk;alm;aðalhellarnir;NFFTgr
aðalhellir;74631;kk;alm;aðalhella;ÞFFT
aðalhellir;74631;kk;alm;aðalhellana;ÞFFTgr
aðalhellir;74631;kk;alm;aðalhellum;ÞGFFT
aðalhellir;74631;kk;alm;aðalhellunum;ÞGFFTgr
aðalhellir;74631;kk;alm;aðalhella;EFFT
aðalhellir;74631;kk;alm;aðalhellanna;EFFTgr
aðalhenda;153961;kvk;alm;aðalhenda;NFET
aðalhenda;153961;kvk;alm;aðalhendan;NFETgr
aðalhenda;153961;kvk;alm;aðalhendu;ÞFET
aðalhenda;153961;kvk;alm;aðalhenduna;ÞFETgr
aðalhenda;153961;kvk;alm;aðalhendu;ÞGFET
aðalhenda;153961;kvk;alm;aðalhendunni;ÞGFETgr
aðalhenda;153961;kvk;alm;aðalhendu;EFET
aðalhenda;153961;kvk;alm;aðalhendunnar;EFETgr
aðalhenda;153961;kvk;alm;aðalhendur;NFFT
aðalhenda;153961;kvk;alm;aðalhendurnar;NFFTgr
aðalhenda;153961;kvk;alm;aðalhendur;ÞFFT
aðalhenda;153961;kvk;alm;aðalhendurnar;ÞFFTgr
aðalhenda;153961;kvk;alm;aðalhendum;ÞGFFT
aðalhenda;153961;kvk;alm;aðalhendunum;ÞGFFTgr
aðalhenda;153961;kvk;alm;aðalhendna;EFFT
aðalhenda;153961;kvk;alm;aðalhenda;EFFT2
aðalhenda;153961;kvk;alm;aðalhendnanna;EFFTgr
aðalhenda;153961;kvk;alm;aðalhendanna;EFFTgr2
fallegur;168136;lo;alm;fallegur;FSB-KK-NFET
fallegur;168136;lo;alm;fallegan;FSB-KK-ÞFET
fallegur;168136;lo;alm;fallegum;FSB-KK-ÞGFET
fallegur;168136;lo;alm;fallegs;FSB-KK-EFET
fallegur;168136;lo;alm;fallegir;FSB-KK-NFFT
fallegur;168136;lo;alm;fallega;FSB-KK-ÞFFT
fallegur;168136;lo;alm;fallegum;FSB-KK-ÞGFFT
fallegur;168136;lo;alm;fallegra;FSB-KK-EFFT
fallegur;168136;lo;alm;falleg;FSB-KVK-NFET
fallegur;168136;lo;alm;fallega;FSB-KVK-ÞFET
fallegur;168136;lo;alm;fallegri;FSB-KVK-ÞGFET
fallegur;168136;lo;alm;fallegrar;FSB-KVK-EFET
fallegur;168136;lo;alm;fallegar;FSB-KVK-NFFT
fallegur;168136;lo;alm;fallegar;FSB-KVK-ÞFFT
fallegur;168136;lo;alm;fallegum;FSB-KVK-ÞGFFT
fallegur;168136;lo;alm;fallegra;FSB-KVK-EFFT
fallegur;168136;lo;alm;fallegt;FSB-HK-NFET
fallegur;168136;lo;alm;fallegt;FSB-HK-ÞFET
fallegur;168136;lo;alm;fallegu;FSB-HK-ÞGFET
fallegur;168136;lo;alm;fallegs;FSB-HK-EFET
fallegur;168136;lo;alm;falleg;FSB-HK-NFFT
fallegur;168136;lo;alm;falleg;FSB-HK-ÞFFT
fallegur;168136;lo;alm;fallegum;FSB-HK-ÞGFFT
fallegur;168136;lo;alm;fallegra;FSB-HK-EFFT
fallegur;168136;lo;alm;fallegi;FVB-KK-NFET
fallegur;168136;lo;alm;fallega;FVB-KK-ÞFET
fallegur;168136;lo;alm;fallega;FVB-KK-ÞGFET
fallegur;168136;lo;alm;fallega;FVB-KK-EFET
fallegur;168136;lo;alm;fallegu;FVB-KK-NFFT
fallegur;168136;lo;alm;fallegu;FVB-KK-ÞFFT
fallegur;168136;lo;alm;fallegu;FVB-KK-ÞGFFT
fallegur;168136;lo;alm;fallegu;FVB-KK-EFFT
fallegur;168136;lo;alm;fallega;FVB-KVK-NFET
fallegur;168136;lo;alm;fallegu;FVB-KVK-ÞFET
fallegur;168136;lo;alm;fallegu;FVB-KVK-ÞGFET
fallegur;168136;lo;alm;fallegu;FVB-KVK-EFET
fallegur;168136;lo;alm;fallegu;FVB-KVK-NFFT
fallegur;168136;lo;alm;fallegu;FVB-KVK-ÞFFT
fallegur;168136;lo;alm;fallegu;FVB-KVK-ÞGFFT
fallegur;168136;lo;alm;fallegu;FVB-KVK-EFFT
fallegur;168136;lo;alm;fallega;FVB-HK-NFET
fallegur;168136;lo;alm;fallega;FVB-HK-ÞFET
fallegur;168136;lo;alm;fallega;FVB-HK-ÞGFET
fallegur;168136;lo;alm;fallega;FVB-HK-EFET
fallegur;168136;lo;alm;fallegu;FVB-HK-NFFT
fallegur;168136;lo;alm;fallegu;FVB-HK-ÞFFT
fallegur;168136;lo;alm;fallegu;FVB-HK-ÞGFFT
fallegur;168136;lo;alm;fallegu;FVB-HK-EFFT
fallegur;168136;lo;alm;fallegri;MST-KK-NFET
fallegur;168136;lo;alm;fallegri;MST-KK-ÞFET
fallegur;168136;lo;alm;fallegri;MST-KK-ÞGFET
fallegur;168136;lo;alm;fallegri;MST-KK-EFET
fallegur;168136;lo;alm;fallegri;MST-KK-NFFT
fallegur;168136;lo;alm;fallegri;MST-KK-ÞFFT
fallegur;168136;lo;alm;fallegri;MST-KK-ÞGFFT
fallegur;168136;lo;alm;fallegri;MST-KK-EFFT
fallegur;168136;lo;alm;fallegri;MST-KVK-NFET
fallegur;168136;lo;alm;fallegri;MST-KVK-ÞFET
fallegur;168136;lo;alm;fallegri;MST-KVK-ÞGFET
fallegur;168136;lo;alm;fallegri;MST-KVK-EFET
fallegur;168136;lo;alm;fallegri;MST-KVK-NFFT
fallegur;168136;lo;alm;fallegri;MST-KVK-ÞFFT
fallegur;168136;lo;alm;fallegri;MST-KVK-ÞGFFT
fallegur;168136;lo;alm;fallegri;MST-KVK-EFFT
fallegur;168136;lo;alm;fallegra;MST-HK-NFET
fallegur;168136;lo;alm;fallegra;MST-HK-ÞFET
fallegur;168136;lo;alm;fallegra;MST-HK-ÞGFET
fallegur;168136;lo;alm;fallegra;MST-HK-EFET
fallegur;168136;lo;alm;fallegri;MST-HK-NFFT
fallegur;168136;lo;alm;fallegri;MST-HK-ÞFFT
fallegur;168136;lo;alm;fallegri;MST-HK-ÞGFFT
fallegur;168136;lo;alm;fallegri;MST-HK-EFFT
fallegur;168136;lo;alm;fallegastur;ESB-KK-NFET
fallegur;168136;lo;alm;fallegastan;ESB-KK-ÞFET
fallegur;168136;lo;alm;fallegustum;ESB-KK-ÞGFET
fallegur;168136;lo;alm;fallegasts;ESB-KK-EFET
fallegur;168136;lo;alm;fallegastir;ESB-KK-NFFT
fallegur;168136;lo;alm;fallegasta;ESB-KK-ÞFFT
fallegur;168136;lo;alm;fallegustum;ESB-KK-ÞGFFT
fallegur;168136;lo;alm;fallegastra;ESB-KK-EFFT
fallegur;168136;lo;alm;fallegust;ESB-KVK-NFET
fallegur;168136;lo;alm;fallegasta;ESB-KVK-ÞFET
fallegur;168136;lo;alm;fallegastri;ESB-KVK-ÞGFET
fallegur;168136;lo;alm;fallegastrar;ESB-KVK-EFET
fallegur;168136;lo;alm;fallegastar;ESB-KVK-NFFT
fallegur;168136;lo;alm;fallegastar;ESB-KVK-ÞFFT
fallegur;168136;lo;alm;fallegustum;ESB-KVK-ÞGFFT
fallegur;168136;lo;alm;fallegastra;ESB-KVK-EFFT
fallegur;168136;lo;alm;fallegast;ESB-HK-NFET
fallegur;168136;lo;alm;fallegast;ESB-HK-ÞFET
fallegur;168136;lo;alm;fallegustu;ESB-HK-ÞGFET
fallegur;168136;lo;alm;fallegasts;ESB-HK-EFET
fallegur;168136;lo;alm;fallegust;ESB-HK-NFFT
fallegur;168136;lo;alm;fallegust;ESB-HK-ÞFFT
fallegur;168136;lo;alm;fallegustum;ESB-HK-ÞGFFT
fallegur;168136;lo;alm;fallegastra;ESB-HK-EFFT
fallegur;168136;lo;alm;fallegasti;EVB-KK-NFET
fallegur;168136;lo;alm;fallegasta;EVB-KK-ÞFET
fallegur;168136;lo;alm;fallegasta;EVB-KK-ÞGFET
fallegur;168136;lo;alm;fallegasta;EVB-KK-EFET
fallegur;168136;lo;alm;fallegustu;EVB-KK-NFFT
fallegur;168136;lo;alm;fallegustu;EVB-KK-ÞFFT
fallegur;168136;lo;alm;fallegustu;EVB-KK-ÞGFFT
fallegur;168136;lo;alm;fallegustu;EVB-KK-EFFT
fallegur;168136;lo;alm;fallegasta;EVB-KVK-NFET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-ÞFET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-ÞGFET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-EFET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-NFFT
fallegur;168136;lo;alm;fallegustu;EVB-KVK-ÞFFT
fallegur;168136;lo;alm;fallegustu;EVB-KVK-ÞGFFT
fallegur;168136;lo;alm;fallegustu;EVB-KVK-EFFT
fallegur;168136;lo;alm;fallegasta;EVB-HK-NFET
fallegur;168136;lo;alm;fallegasta;EVB-HK-ÞFET
fallegur;168136;lo;alm;fallegasta;EVB-HK-ÞGFET
fallegur;168136;lo;alm;fallegasta;EVB-HK-EFET
fallegur;168136;lo;alm;fallegustu;EVB-HK-NFFT
fallegur;168136;lo;alm;fallegustu;EVB-HK-ÞFFT
fallegur;168136;lo;alm;fallegustu;EVB-HK-ÞGFFT
fallegur;168136;lo;alm;fallegustu;EVB-HK-EFFT
læra;435046;so;alm;læra;GM-NH
læra;435046;so;alm;læri;GM-FH-NT-1P-ET
læra;435046;so;alm;lærir;GM-FH-NT-2P-ET
læra;435046;so;alm;lærir;GM-FH-NT-3P-ET
læra;435046;so;alm;lærum;GM-FH-NT-1P-FT
læra;435046;so;alm;lærið;GM-FH-NT-2P-FT
læra;435046;so;alm;læra;GM-FH-NT-3P-FT
læra;435046;so;alm;lærði;GM-FH-ÞT-1P-ET
læra;435046;so;alm;lærðir;GM-FH-ÞT-2P-ET
læra;435046;so;alm;lærði;GM-FH-ÞT-3P-ET
læra;435046;so;alm;lærðum;GM-FH-ÞT-1P-FT
læra;435046;so;alm;lærðuð;GM-FH-ÞT-2P-FT
læra;435046;so;alm;lærðu;GM-FH-ÞT-3P-FT
læra;435046;so;alm;læri;GM-VH-NT-1P-ET
læra;435046;so;alm;lærir;GM-VH-NT-2P-ET
læra;435046;so;alm;læri;GM-VH-NT-3P-ET
læra;435046;so;alm;lærum;GM-VH-NT-1P-FT
læra;435046;so;alm;lærið;GM-VH-NT-2P-FT
læra;435046;so;alm;læri;GM-VH-NT-3P-FT
læra;435046;so;alm;lærði;GM-VH-ÞT-1P-ET
læra;435046;so;alm;lærðir;GM-VH-ÞT-2P-ET
læra;435046;so;alm;lærði;GM-VH-ÞT-3P-ET
læra;435046;so;alm;lærðum;GM-VH-ÞT-1P-FT
læra;435046;so;alm;lærðuð;GM-VH-ÞT-2P-FT
læra;435046;so;alm;lærðu;GM-VH-ÞT-3P-FT
læra;435046;so;alm;lærast;MM-NH
læra;435046;so;alm;lærist;MM-FH-NT-1P-ET
læra;435046;so;alm;lærist;MM-FH-NT-2P-ET
læra;435046;so;alm;lærist;MM-FH-NT-3P-ET
læra;435046;so;alm;lærumst;MM-FH-NT-1P-FT
læra;435046;so;alm;lærist;MM-FH-NT-2P-FT
læra;435046;so;alm;lærast;MM-FH-NT-3P-FT
læra;435046;so;alm;lærðist;MM-FH-ÞT-1P-ET
læra;435046;so;alm;lærðist;MM-FH-ÞT-2P-ET
læra;435046;so;alm;lærðist;MM-FH-ÞT-3P-ET
læra;435046;so;alm;lærðumst;MM-FH-ÞT-1P-FT
læra;435046;so;alm;lærðust;MM-FH-ÞT-2P-FT
læra;435046;so;alm;lærðust;MM-FH-ÞT-3P-FT
læra;435046;so;alm;lærist;MM-VH-NT-1P-ET
læra;435046;so;alm;lærist;MM-VH-NT-2P-ET
læra;435046;so;alm;lærist;MM-VH-NT-3P-ET
læra;435046;so;alm;lærumst;MM-VH-NT-1P-FT
læra;435046;so;alm;lærist;MM-VH-NT-2P-FT
læra;435046;so;alm;lærist;MM-VH-NT-3P-FT
læra;435046;so;alm;lærðist;MM-VH-ÞT-1P-ET
læra;435046;so;alm;lærðist;MM-VH-ÞT-2P-ET
læra;435046;so;alm;lærðist;MM-VH-ÞT-3P-ET
læra;435046;so;alm;lærðumst;MM-VH-ÞT-1P-FT
læra;435046;so;alm;lærðust;MM-VH-ÞT-2P-FT
læra;435046;so;alm;lærðust;MM-VH-ÞT-3P-FT
læra;435046;so;alm;lær;GM-BH-ST
læra;435046;so;alm;lærðu;GM-BH-ET
læra;435046;so;alm;lærið;GM-BH-FT
læra;435046;so;alm;lærandi;LHNT
læra;435046;so;alm;lært;GM-SAGNB
læra;435046;so;alm;lærst;MM-SAGNB
læra;435046;so;alm;lærður;LHÞT-SB-KK-NFET
læra;435046;so;alm;lærðan;LHÞT-SB-KK-ÞFET
læra;435046;so;alm;lærðum;LHÞT-SB-KK-ÞGFET
læra;435046;so;alm;lærðs;LHÞT-SB-KK-EFET
læra;435046;so;alm;lærðir;LHÞT-SB-KK-NFFT
læra;435046;so;alm;lærða;LHÞT-SB-KK-ÞFFT
læra;435046;so;alm;lærðum;LHÞT-SB-KK-ÞGFFT
læra;435046;so;alm;lærðra;LHÞT-SB-KK-EFFT
læra;435046;so;alm;lærð;LHÞT-SB-KVK-NFET
læra;435046;so;alm;lærða;LHÞT-SB-KVK-ÞFET
læra;435046;so;alm;lærðri;LHÞT-SB-KVK-ÞGFET
læra;435046;so;alm;lærðrar;LHÞT-SB-KVK-EFET
læra;435046;so;alm;lærðar;LHÞT-SB-KVK-NFFT
læra;435046;so;alm;lærðar;LHÞT-SB-KVK-ÞFFT
læra;435046;so;alm;lærðum;LHÞT-SB-KVK-ÞGFFT
læra;435046;so;alm;lærðra;LHÞT-SB-KVK-EFFT
læra;435046;so;alm;lært;LHÞT-SB-HK-NFET
læra;435046;so;alm;lært;LHÞT-SB-HK-ÞFET
læra;435046;so;alm;lærðu;LHÞT-SB-HK-ÞGFET
læra;435046;so;alm;lærðs;LHÞT-SB-HK-EFET
læra;435046;so;alm;lærð;LHÞT-SB-HK-NFFT
læra;435046;so;alm;lærð;LHÞT-SB-HK-ÞFFT
læra;435046;so;alm;lærðum;LHÞT-SB-HK-ÞGFFT
læra;435046;so;alm;lærðra;LHÞT-SB-HK-EFFT
læra;435046;so;alm;lærði;LHÞT-VB-KK-NFET
læra;435046;so;alm;lærða;LHÞT-VB-KK-ÞFET
læra;435046;so;alm;lærða;LHÞT-VB-KK-ÞGFET
læra;435046;so;alm;lærða;LHÞT-VB-KK-EFET
læra;435046;so;alm;lærðu;LHÞT-VB-KK-NFFT
læra;435046;so;alm;lærðu;LHÞT-VB-KK-ÞFFT
læra;435046;so;alm;lærðu;LHÞT-VB-KK-ÞGFFT
læra;435046;so;alm;lærðu;LHÞT-VB-KK-EFFT
læra;435046;so;alm;lærða;LHÞT-VB-KVK-NFET
læra;435046;so;alm;lærðu;LHÞT-VB-KVK-ÞFET
læra;435046;so;alm;lærðu;LHÞT-VB-KVK-ÞGFET
læra;435046;so;alm;lærðu;LHÞT-VB-KVK-EFET
læra;435046;so;alm;lærðu;LHÞT-VB-KVK-NFFT
læra;435046;so;alm;lærðu;LHÞT-VB-KVK-ÞFFT
læra;435046;so;alm;lærðu;LHÞT-VB-KVK-ÞGFFT
læra;435046;so;alm;lærðu;LHÞT-VB-KVK-EFFT
læra;435046;so;alm;lærða;LHÞT-VB-HK-NFET
læra;435046;so;alm;lærða;LHÞT-VB-HK-ÞFET
læra;435046;so;alm;lærða;LHÞT-VB-HK-ÞGFET
læra;435046;so;alm;lærða;LHÞT-VB-HK-EFET
læra;435046;so;alm;lærðu;LHÞT-VB-HK-NFFT
læra;435046;so;alm;lærðu;LHÞT-VB-HK-ÞFFT
læra;435046;so;alm;lærðu;LHÞT-VB-HK-ÞGFFT
læra;435046;so;alm;lærðu;LHÞT-VB-HK-EFFT
læra;435046;so;alm;lærist;OP-ÞGF-MM-FH-NT-1P-ET
læra;435046;so;alm;lærist;OP-ÞGF-MM-FH-NT-1P-FT
læra;435046;so;alm;lærist;OP-ÞGF-MM-FH-NT-2P-ET
læra;435046;so;alm;lærist;OP-ÞGF-MM-FH-NT-2P-FT
læra;435046;so;alm;lærist;OP-ÞGF-MM-FH-NT-3P-ET
læra;435046;so;alm;lærist;OP-ÞGF-MM-FH-NT-3P-FT
læra;435046;so;alm;lærðist;OP-ÞGF-MM-FH-ÞT-1P-ET
læra;435046;so;alm;lærðist;OP-ÞGF-MM-FH-ÞT-1P-FT
læra;435046;so;alm;lærðist;OP-ÞGF-MM-FH-ÞT-2P-ET
læra;435046;so;alm;lærðist;OP-ÞGF-MM-FH-ÞT-2P-FT
læra;435046;so;alm;lærðist;OP-ÞGF-MM-FH-ÞT-3P-ET
læra;435046;so;alm;lærðist;OP-ÞGF-MM-FH-ÞT-3P-FT
læra;435046;so;alm;lærist;OP-ÞGF-MM-VH-NT-1P-ET
læra;435046;so;alm;lærist;OP-ÞGF-MM-VH-NT-1P-FT
læra;435046;so;alm;lærist;OP-ÞGF-MM-VH-NT-2P-ET
læra;435046;so;alm;lærist;OP-ÞGF-MM-VH-NT-2P-FT
læra;435046;so;alm;lærist;OP-ÞGF-MM-VH-NT-3P-ET
læra;435046;so;alm;lærist;OP-ÞGF-MM-VH-NT-3P-FT
læra;435046;so;alm;lærðist;OP-ÞGF-MM-VH-ÞT-1P-ET
læra;435046;so;alm;lærðist;OP-ÞGF-MM-VH-ÞT-1P-FT
læra;435046;so;alm;lærðist;OP-ÞGF-MM-VH-ÞT-2P-ET
læra;435046;so;alm;lærðist;OP-ÞGF-MM-VH-ÞT-2P-FT
læra;435046;so;alm;lærðist;OP-ÞGF-MM-VH-ÞT-3P-ET
læra;435046;so;alm;lærðist;OP-ÞGF-MM-VH-ÞT-3P-FT
ég;403780;pfn;alm;ég;NFET
ég;403780;pfn;alm;mig;ÞFET
ég;403780;pfn;alm;mér;ÞGFET
ég;403780;pfn;alm;mín;EFET
ég;403780;pfn;alm;við;NFFT
ég;403780;pfn;alm;okkur;ÞFFT
ég;403780;pfn;alm;okkur;ÞGFFT
ég;403780;pfn;alm;okkar;EFFT
þú;403782;pfn;alm;þú;NFET
þú;403782;pfn;alm;þig;ÞFET
þú;403782;pfn;alm;þér;ÞGFET
þú;403782;pfn;alm;þín;EFET
þú;403782;pfn;alm;þið;NFFT
þú;403782;pfn;alm;ykkur;ÞFFT
þú;403782;pfn;alm;ykkur;ÞGFFT
þú;403782;pfn;alm;ykkar;EFFT
hann;403784;pfn;alm;hann;NFET
hann;403784;pfn;alm;hann;ÞFET
hann;403784;pfn;alm;honum;ÞGFET
hann;403784;pfn;alm;hans;EFET
hann;403784;pfn;alm;þeir;NFFT
hann;403784;pfn;alm;þá;ÞFFT
hann;403784;pfn;alm;þeim;ÞGFFT
hann;403784;pfn;alm;þeirra;EFFT
hún;403785;pfn;alm;hún;NFET
hún;403785;pfn;alm;hana;ÞFET
hún;403785;pfn;alm;henni;ÞGFET
hún;403785;pfn;alm;hennar;EFET
hún;403785;pfn;alm;þær;NFFT
hún;403785;pfn;alm;þær;ÞFFT
hún;403785;pfn;alm;þeim;ÞGFFT
hún;403785;pfn;alm;þeirra;EFFT
það;403786;pfn;alm;það;NFET
það;403786;pfn;alm;það;ÞFET
það;403786;pfn;alm;því;ÞGFET
það;403786;pfn;alm;þess;EFET
það;403786;pfn;alm;þau;NFFT
það;403786;pfn;alm;þau;ÞFFT
það;403786;pfn;alm;þeim;ÞGFFT
það;403786;pfn;alm;þeirra;EFFT";

    #[test]
    pub fn gets_noun_entry() {
        let bin_data = BinData::load(TEST_DATA.as_bytes()).unwrap();
        let noun_entry = bin_data.noun("aðalhenda").unwrap();

        assert_eq!(Gender::Feminine, noun_entry.gender);
        // Singular
        assert_eq!("aðalhenda", noun_entry.nom_sg.unwrap());
        assert_eq!("aðalhendan", noun_entry.nom_sg_def.unwrap());
        assert_eq!("aðalhendu", noun_entry.acc_sg.unwrap());
        assert_eq!("aðalhenduna", noun_entry.acc_sg_def.unwrap());
        assert_eq!("aðalhendu", noun_entry.dat_sg.unwrap());
        assert_eq!("aðalhendunni", noun_entry.dat_sg_def.unwrap());
        assert_eq!("aðalhendu", noun_entry.gen_sg.unwrap());
        assert_eq!("aðalhendunnar", noun_entry.gen_sg_def.unwrap());
        // Plural
        assert_eq!("aðalhendur", noun_entry.nom_pl.unwrap());
        assert_eq!("aðalhendurnar", noun_entry.nom_pl_def.unwrap());
        assert_eq!("aðalhendur", noun_entry.acc_pl.unwrap());
        assert_eq!("aðalhendurnar", noun_entry.acc_pl_def.unwrap());
        assert_eq!("aðalhendum", noun_entry.dat_pl.unwrap());
        assert_eq!("aðalhendunum", noun_entry.dat_pl_def.unwrap());
        assert_eq!("aðalhendna", noun_entry.gen_pl.unwrap());
        assert_eq!("aðalhendnanna", noun_entry.gen_pl_def.unwrap());
    }

    #[test]
    pub fn gets_adjective_entry() {
        let bin_data = BinData::load(TEST_DATA.as_bytes()).unwrap();
        let adjective_entry = bin_data.adjective("fallegur").unwrap();

        assert_eq!("fallegur", adjective_entry.masc_nom_sg_strong.unwrap());
        assert_eq!("fallegan", adjective_entry.masc_acc_sg_strong.unwrap());
        assert_eq!("fallegum", adjective_entry.masc_dat_sg_strong.unwrap());
        assert_eq!("fallegs", adjective_entry.masc_gen_sg_strong.unwrap());

        assert_eq!("falleg", adjective_entry.fem_nom_sg_strong.unwrap());
        assert_eq!("fallega", adjective_entry.fem_acc_sg_strong.unwrap());
        assert_eq!("fallegri", adjective_entry.fem_dat_sg_strong.unwrap());
        assert_eq!("fallegrar", adjective_entry.fem_gen_sg_strong.unwrap());

        assert_eq!("fallegt", adjective_entry.neut_nom_sg_strong.unwrap());
        assert_eq!("fallegt", adjective_entry.neut_acc_sg_strong.unwrap());
        assert_eq!("fallegu", adjective_entry.neut_dat_sg_strong.unwrap());
        assert_eq!("fallegs", adjective_entry.neut_gen_sg_strong.unwrap());

        assert_eq!("fallegir", adjective_entry.masc_nom_pl_strong.unwrap());
        assert_eq!("fallega", adjective_entry.masc_acc_pl_strong.unwrap());
        assert_eq!("fallegum", adjective_entry.masc_dat_pl_strong.unwrap());
        assert_eq!("fallegra", adjective_entry.masc_gen_pl_strong.unwrap());

        assert_eq!("fallegar", adjective_entry.fem_nom_pl_strong.unwrap());
        assert_eq!("fallegar", adjective_entry.fem_acc_pl_strong.unwrap());
        assert_eq!("fallegum", adjective_entry.fem_dat_pl_strong.unwrap());
        assert_eq!("fallegra", adjective_entry.fem_gen_pl_strong.unwrap());

        assert_eq!("falleg", adjective_entry.neut_nom_pl_strong.unwrap());
        assert_eq!("falleg", adjective_entry.neut_acc_pl_strong.unwrap());
        assert_eq!("fallegum", adjective_entry.neut_dat_pl_strong.unwrap());
        assert_eq!("fallegra", adjective_entry.neut_gen_pl_strong.unwrap());

        assert_eq!("fallegi", adjective_entry.masc_nom_sg_weak.unwrap());
        assert_eq!("fallega", adjective_entry.masc_acc_sg_weak.unwrap());
        assert_eq!("fallega", adjective_entry.masc_dat_sg_weak.unwrap());
        assert_eq!("fallega", adjective_entry.masc_gen_sg_weak.unwrap());

        assert_eq!("fallega", adjective_entry.fem_nom_sg_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.fem_acc_sg_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.fem_dat_sg_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.fem_gen_sg_weak.unwrap());

        assert_eq!("fallega", adjective_entry.neut_nom_sg_weak.unwrap());
        assert_eq!("fallega", adjective_entry.neut_acc_sg_weak.unwrap());
        assert_eq!("fallega", adjective_entry.neut_dat_sg_weak.unwrap());
        assert_eq!("fallega", adjective_entry.neut_gen_sg_weak.unwrap());

        assert_eq!("fallegu", adjective_entry.masc_nom_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.masc_acc_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.masc_dat_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.masc_gen_pl_weak.unwrap());

        assert_eq!("fallegu", adjective_entry.fem_nom_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.fem_acc_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.fem_dat_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.fem_gen_pl_weak.unwrap());

        assert_eq!("fallegu", adjective_entry.neut_nom_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.neut_acc_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.neut_dat_pl_weak.unwrap());
        assert_eq!("fallegu", adjective_entry.neut_gen_pl_weak.unwrap());
    }

    #[test]
    pub fn gets_verb_entry() {
        let bin_data = BinData::load(TEST_DATA.as_bytes()).unwrap();
        let verb_entry = bin_data.verb("læra").unwrap();

        assert_eq!("læri", verb_entry.pres_ind_first_sg.unwrap());
        assert_eq!("lærir", verb_entry.pres_ind_second_sg.unwrap());
        assert_eq!("lærir", verb_entry.pres_ind_third_sg.unwrap());

        assert_eq!("lærum", verb_entry.pres_ind_first_pl.unwrap());
        assert_eq!("lærið", verb_entry.pres_ind_second_pl.unwrap());
        assert_eq!("læra", verb_entry.pres_ind_third_pl.unwrap());

        assert_eq!("lærði", verb_entry.past_ind_first_sg.unwrap());
        assert_eq!("lærðir", verb_entry.past_ind_second_sg.unwrap());
        assert_eq!("lærði", verb_entry.past_ind_third_sg.unwrap());

        assert_eq!("lærðum", verb_entry.past_ind_first_pl.unwrap());
        assert_eq!("lærðuð", verb_entry.past_ind_second_pl.unwrap());
        assert_eq!("lærðu", verb_entry.past_ind_third_pl.unwrap());
    }

    #[test]
    pub fn gets_pronoun_entries() {
        let bin_data = BinData::load(TEST_DATA.as_bytes()).unwrap();

        let e = bin_data.pronoun("ég").unwrap();
        assert_eq!("ég", e.nom.unwrap());
        assert_eq!("mig", e.acc.unwrap());
        assert_eq!("mér", e.dat.unwrap());
        assert_eq!("mín", e.gen.unwrap());

        let e = bin_data.pronoun("þú").unwrap();
        assert_eq!("þú", e.nom.unwrap());
        assert_eq!("þig", e.acc.unwrap());
        assert_eq!("þér", e.dat.unwrap());
        assert_eq!("þín", e.gen.unwrap());

        let e = bin_data.pronoun("hann").unwrap();
        assert_eq!("hann", e.nom.unwrap());
        assert_eq!("hann", e.acc.unwrap());
        assert_eq!("honum", e.dat.unwrap());
        assert_eq!("hans", e.gen.unwrap());

        let e = bin_data.pronoun("hún").unwrap();
        assert_eq!("hún", e.nom.unwrap());
        assert_eq!("hana", e.acc.unwrap());
        assert_eq!("henni", e.dat.unwrap());
        assert_eq!("hennar", e.gen.unwrap());

        let e = bin_data.pronoun("það").unwrap();
        assert_eq!("það", e.nom.unwrap());
        assert_eq!("það", e.acc.unwrap());
        assert_eq!("því", e.dat.unwrap());
        assert_eq!("þess", e.gen.unwrap());

        let e = bin_data.pronoun("við").unwrap();
        assert_eq!("við", e.nom.unwrap());
        assert_eq!("okkur", e.acc.unwrap());
        assert_eq!("okkur", e.dat.unwrap());
        assert_eq!("okkar", e.gen.unwrap());

        let e = bin_data.pronoun("þið").unwrap();
        assert_eq!("þið", e.nom.unwrap());
        assert_eq!("ykkur", e.acc.unwrap());
        assert_eq!("ykkur", e.dat.unwrap());
        assert_eq!("ykkar", e.gen.unwrap());

        let e = bin_data.pronoun("þeir").unwrap();
        assert_eq!("þeir", e.nom.unwrap());
        assert_eq!("þá", e.acc.unwrap());
        assert_eq!("þeim", e.dat.unwrap());
        assert_eq!("þeirra", e.gen.unwrap());

        let e = bin_data.pronoun("þær").unwrap();
        assert_eq!("þær", e.nom.unwrap());
        assert_eq!("þær", e.acc.unwrap());
        assert_eq!("þeim", e.dat.unwrap());
        assert_eq!("þeirra", e.gen.unwrap());

        let e = bin_data.pronoun("þau").unwrap();
        assert_eq!("þau", e.nom.unwrap());
        assert_eq!("þau", e.acc.unwrap());
        assert_eq!("þeim", e.dat.unwrap());
        assert_eq!("þeirra", e.gen.unwrap());
    }
}
