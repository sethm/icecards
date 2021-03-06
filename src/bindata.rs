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
            "??g" => (self.data.get("??g"), Some("FET")),
            "vi??" => (self.data.get("??g"), Some("FFT")),
            "????" => (self.data.get("????"), Some("FET")),
            "??i??" => (self.data.get("????"), Some("FFT")),
            "hann" => (self.data.get("hann"), Some("FET")),
            "??eir" => (self.data.get("hann"), Some("FFT")),
            "h??n" => (self.data.get("h??n"), Some("FET")),
            "????r" => (self.data.get("h??n"), Some("FFT")),
            "??a??" => (self.data.get("??a??"), Some("FET")),
            "??au" => (self.data.get("??a??"), Some("FFT")),
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
                            .find(|&e| e.tag == format!("??{}", tag))
                            .map(|e| e.form.to_string()),
                        dat: entries
                            .iter()
                            .find(|&e| e.tag == format!("??G{}", tag))
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
                            .find(|&&e| e.tag == "KK-??FET" || e.tag == "KK-??FFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-??GFET" || e.tag == "KK-??GFFT")
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
                            .find(|&&e| e.tag == "KVK-??FET" || e.tag == "KVK-??FFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-??GFET" || e.tag == "KVK-??GFFT")
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
                            .find(|&&e| e.tag == "HK-??FET" || e.tag == "HK-??FFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-??GFET" || e.tag == "HK-??GFFT")
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
                            .find(|&&e| e.tag == "KK-??FET")
                            .map(|&e| e.form.to_string()),
                        masc_dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-??GFET")
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
                            .find(|&&e| e.tag == "KVK-??FET")
                            .map(|&e| e.form.to_string()),
                        fem_dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-??GFET")
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
                            .find(|&&e| e.tag == "HK-??FET")
                            .map(|&e| e.form.to_string()),
                        neut_dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-??GFET")
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
                            .find(|&&e| e.tag == "KK-??FFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KK-??GFFT")
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
                            .find(|&&e| e.tag == "KVK-??FFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "KVK-??GFFT")
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
                            .find(|&&e| e.tag == "HK-??FFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "HK-??GFFT")
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
                            .find(|&&e| e.tag == "FSB-KK-??FET")
                            .map(|&e| e.form.to_string()),
                        masc_dat_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-??GFET")
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
                            .find(|&&e| e.tag == "FSB-KVK-??FET")
                            .map(|&e| e.form.to_string()),
                        fem_dat_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-??GFET")
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
                            .find(|&&e| e.tag == "FSB-HK-??FET")
                            .map(|&e| e.form.to_string()),
                        neut_dat_sg_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-??GFET")
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
                            .find(|&&e| e.tag == "FSB-KK-??FFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KK-??GFFT")
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
                            .find(|&&e| e.tag == "FSB-KVK-??FFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-KVK-??GFFT")
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
                            .find(|&&e| e.tag == "FSB-HK-??FFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat_pl_strong: entries
                            .iter()
                            .find(|&&e| e.tag == "FSB-HK-??GFFT")
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
                            .find(|&&e| e.tag == "FVB-KK-??FET")
                            .map(|&e| e.form.to_string()),
                        masc_dat_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-??GFET")
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
                            .find(|&&e| e.tag == "FVB-KVK-??FET")
                            .map(|&e| e.form.to_string()),
                        fem_dat_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-??GFET")
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
                            .find(|&&e| e.tag == "FVB-HK-??FET")
                            .map(|&e| e.form.to_string()),
                        neut_dat_sg_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-??GFET")
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
                            .find(|&&e| e.tag == "FVB-KK-??FFT")
                            .map(|&e| e.form.to_string()),
                        masc_dat_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KK-??GFFT")
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
                            .find(|&&e| e.tag == "FVB-KVK-??FFT")
                            .map(|&e| e.form.to_string()),
                        fem_dat_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-KVK-??GFFT")
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
                            .find(|&&e| e.tag == "FVB-HK-??FFT")
                            .map(|&e| e.form.to_string()),
                        neut_dat_pl_weak: entries
                            .iter()
                            .find(|&&e| e.tag == "FVB-HK-??GFFT")
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
                            .find(|&&e| e.tag == "??FET")
                            .map(|&e| e.form.to_string()),
                        dat_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "??GFET")
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
                            .find(|&&e| e.tag == "??FFT")
                            .map(|&e| e.form.to_string()),
                        dat_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "??GFFT")
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
                            .find(|&&e| e.tag == "??FETgr")
                            .map(|&e| e.form.to_string()),
                        dat_sg_def: entries
                            .iter()
                            .find(|&&e| e.tag == "??GFETgr")
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
                            .find(|&&e| e.tag == "??FFTgr")
                            .map(|&e| e.form.to_string()),
                        dat_pl_def: entries
                            .iter()
                            .find(|&&e| e.tag == "??GFFTgr")
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
                            .find(|&&e| e.tag == "GM-FH-??T-1P-ET")
                            .map(|&e| e.form.to_string()),
                        past_ind_second_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-??T-2P-ET")
                            .map(|&e| e.form.to_string()),
                        past_ind_third_sg: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-??T-3P-ET")
                            .map(|&e| e.form.to_string()),
                        past_ind_first_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-??T-1P-FT")
                            .map(|&e| e.form.to_string()),
                        past_ind_second_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-??T-2P-FT")
                            .map(|&e| e.form.to_string()),
                        past_ind_third_pl: entries
                            .iter()
                            .find(|&&e| e.tag == "GM-FH-??T-3P-FT")
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

    const TEST_DATA: &str = "a??alhellir;74631;kk;alm;a??alhellir;NFET
a??alhellir;74631;kk;alm;a??alhellirinn;NFETgr
a??alhellir;74631;kk;alm;a??alhelli;??FET
a??alhellir;74631;kk;alm;a??alhellinn;??FETgr
a??alhellir;74631;kk;alm;a??alhelli;??GFET
a??alhellir;74631;kk;alm;a??alhellinum;??GFETgr
a??alhellir;74631;kk;alm;a??alhellis;EFET
a??alhellir;74631;kk;alm;a??alhellisins;EFETgr
a??alhellir;74631;kk;alm;a??alhellar;NFFT
a??alhellir;74631;kk;alm;a??alhellarnir;NFFTgr
a??alhellir;74631;kk;alm;a??alhella;??FFT
a??alhellir;74631;kk;alm;a??alhellana;??FFTgr
a??alhellir;74631;kk;alm;a??alhellum;??GFFT
a??alhellir;74631;kk;alm;a??alhellunum;??GFFTgr
a??alhellir;74631;kk;alm;a??alhella;EFFT
a??alhellir;74631;kk;alm;a??alhellanna;EFFTgr
a??alhenda;153961;kvk;alm;a??alhenda;NFET
a??alhenda;153961;kvk;alm;a??alhendan;NFETgr
a??alhenda;153961;kvk;alm;a??alhendu;??FET
a??alhenda;153961;kvk;alm;a??alhenduna;??FETgr
a??alhenda;153961;kvk;alm;a??alhendu;??GFET
a??alhenda;153961;kvk;alm;a??alhendunni;??GFETgr
a??alhenda;153961;kvk;alm;a??alhendu;EFET
a??alhenda;153961;kvk;alm;a??alhendunnar;EFETgr
a??alhenda;153961;kvk;alm;a??alhendur;NFFT
a??alhenda;153961;kvk;alm;a??alhendurnar;NFFTgr
a??alhenda;153961;kvk;alm;a??alhendur;??FFT
a??alhenda;153961;kvk;alm;a??alhendurnar;??FFTgr
a??alhenda;153961;kvk;alm;a??alhendum;??GFFT
a??alhenda;153961;kvk;alm;a??alhendunum;??GFFTgr
a??alhenda;153961;kvk;alm;a??alhendna;EFFT
a??alhenda;153961;kvk;alm;a??alhenda;EFFT2
a??alhenda;153961;kvk;alm;a??alhendnanna;EFFTgr
a??alhenda;153961;kvk;alm;a??alhendanna;EFFTgr2
fallegur;168136;lo;alm;fallegur;FSB-KK-NFET
fallegur;168136;lo;alm;fallegan;FSB-KK-??FET
fallegur;168136;lo;alm;fallegum;FSB-KK-??GFET
fallegur;168136;lo;alm;fallegs;FSB-KK-EFET
fallegur;168136;lo;alm;fallegir;FSB-KK-NFFT
fallegur;168136;lo;alm;fallega;FSB-KK-??FFT
fallegur;168136;lo;alm;fallegum;FSB-KK-??GFFT
fallegur;168136;lo;alm;fallegra;FSB-KK-EFFT
fallegur;168136;lo;alm;falleg;FSB-KVK-NFET
fallegur;168136;lo;alm;fallega;FSB-KVK-??FET
fallegur;168136;lo;alm;fallegri;FSB-KVK-??GFET
fallegur;168136;lo;alm;fallegrar;FSB-KVK-EFET
fallegur;168136;lo;alm;fallegar;FSB-KVK-NFFT
fallegur;168136;lo;alm;fallegar;FSB-KVK-??FFT
fallegur;168136;lo;alm;fallegum;FSB-KVK-??GFFT
fallegur;168136;lo;alm;fallegra;FSB-KVK-EFFT
fallegur;168136;lo;alm;fallegt;FSB-HK-NFET
fallegur;168136;lo;alm;fallegt;FSB-HK-??FET
fallegur;168136;lo;alm;fallegu;FSB-HK-??GFET
fallegur;168136;lo;alm;fallegs;FSB-HK-EFET
fallegur;168136;lo;alm;falleg;FSB-HK-NFFT
fallegur;168136;lo;alm;falleg;FSB-HK-??FFT
fallegur;168136;lo;alm;fallegum;FSB-HK-??GFFT
fallegur;168136;lo;alm;fallegra;FSB-HK-EFFT
fallegur;168136;lo;alm;fallegi;FVB-KK-NFET
fallegur;168136;lo;alm;fallega;FVB-KK-??FET
fallegur;168136;lo;alm;fallega;FVB-KK-??GFET
fallegur;168136;lo;alm;fallega;FVB-KK-EFET
fallegur;168136;lo;alm;fallegu;FVB-KK-NFFT
fallegur;168136;lo;alm;fallegu;FVB-KK-??FFT
fallegur;168136;lo;alm;fallegu;FVB-KK-??GFFT
fallegur;168136;lo;alm;fallegu;FVB-KK-EFFT
fallegur;168136;lo;alm;fallega;FVB-KVK-NFET
fallegur;168136;lo;alm;fallegu;FVB-KVK-??FET
fallegur;168136;lo;alm;fallegu;FVB-KVK-??GFET
fallegur;168136;lo;alm;fallegu;FVB-KVK-EFET
fallegur;168136;lo;alm;fallegu;FVB-KVK-NFFT
fallegur;168136;lo;alm;fallegu;FVB-KVK-??FFT
fallegur;168136;lo;alm;fallegu;FVB-KVK-??GFFT
fallegur;168136;lo;alm;fallegu;FVB-KVK-EFFT
fallegur;168136;lo;alm;fallega;FVB-HK-NFET
fallegur;168136;lo;alm;fallega;FVB-HK-??FET
fallegur;168136;lo;alm;fallega;FVB-HK-??GFET
fallegur;168136;lo;alm;fallega;FVB-HK-EFET
fallegur;168136;lo;alm;fallegu;FVB-HK-NFFT
fallegur;168136;lo;alm;fallegu;FVB-HK-??FFT
fallegur;168136;lo;alm;fallegu;FVB-HK-??GFFT
fallegur;168136;lo;alm;fallegu;FVB-HK-EFFT
fallegur;168136;lo;alm;fallegri;MST-KK-NFET
fallegur;168136;lo;alm;fallegri;MST-KK-??FET
fallegur;168136;lo;alm;fallegri;MST-KK-??GFET
fallegur;168136;lo;alm;fallegri;MST-KK-EFET
fallegur;168136;lo;alm;fallegri;MST-KK-NFFT
fallegur;168136;lo;alm;fallegri;MST-KK-??FFT
fallegur;168136;lo;alm;fallegri;MST-KK-??GFFT
fallegur;168136;lo;alm;fallegri;MST-KK-EFFT
fallegur;168136;lo;alm;fallegri;MST-KVK-NFET
fallegur;168136;lo;alm;fallegri;MST-KVK-??FET
fallegur;168136;lo;alm;fallegri;MST-KVK-??GFET
fallegur;168136;lo;alm;fallegri;MST-KVK-EFET
fallegur;168136;lo;alm;fallegri;MST-KVK-NFFT
fallegur;168136;lo;alm;fallegri;MST-KVK-??FFT
fallegur;168136;lo;alm;fallegri;MST-KVK-??GFFT
fallegur;168136;lo;alm;fallegri;MST-KVK-EFFT
fallegur;168136;lo;alm;fallegra;MST-HK-NFET
fallegur;168136;lo;alm;fallegra;MST-HK-??FET
fallegur;168136;lo;alm;fallegra;MST-HK-??GFET
fallegur;168136;lo;alm;fallegra;MST-HK-EFET
fallegur;168136;lo;alm;fallegri;MST-HK-NFFT
fallegur;168136;lo;alm;fallegri;MST-HK-??FFT
fallegur;168136;lo;alm;fallegri;MST-HK-??GFFT
fallegur;168136;lo;alm;fallegri;MST-HK-EFFT
fallegur;168136;lo;alm;fallegastur;ESB-KK-NFET
fallegur;168136;lo;alm;fallegastan;ESB-KK-??FET
fallegur;168136;lo;alm;fallegustum;ESB-KK-??GFET
fallegur;168136;lo;alm;fallegasts;ESB-KK-EFET
fallegur;168136;lo;alm;fallegastir;ESB-KK-NFFT
fallegur;168136;lo;alm;fallegasta;ESB-KK-??FFT
fallegur;168136;lo;alm;fallegustum;ESB-KK-??GFFT
fallegur;168136;lo;alm;fallegastra;ESB-KK-EFFT
fallegur;168136;lo;alm;fallegust;ESB-KVK-NFET
fallegur;168136;lo;alm;fallegasta;ESB-KVK-??FET
fallegur;168136;lo;alm;fallegastri;ESB-KVK-??GFET
fallegur;168136;lo;alm;fallegastrar;ESB-KVK-EFET
fallegur;168136;lo;alm;fallegastar;ESB-KVK-NFFT
fallegur;168136;lo;alm;fallegastar;ESB-KVK-??FFT
fallegur;168136;lo;alm;fallegustum;ESB-KVK-??GFFT
fallegur;168136;lo;alm;fallegastra;ESB-KVK-EFFT
fallegur;168136;lo;alm;fallegast;ESB-HK-NFET
fallegur;168136;lo;alm;fallegast;ESB-HK-??FET
fallegur;168136;lo;alm;fallegustu;ESB-HK-??GFET
fallegur;168136;lo;alm;fallegasts;ESB-HK-EFET
fallegur;168136;lo;alm;fallegust;ESB-HK-NFFT
fallegur;168136;lo;alm;fallegust;ESB-HK-??FFT
fallegur;168136;lo;alm;fallegustum;ESB-HK-??GFFT
fallegur;168136;lo;alm;fallegastra;ESB-HK-EFFT
fallegur;168136;lo;alm;fallegasti;EVB-KK-NFET
fallegur;168136;lo;alm;fallegasta;EVB-KK-??FET
fallegur;168136;lo;alm;fallegasta;EVB-KK-??GFET
fallegur;168136;lo;alm;fallegasta;EVB-KK-EFET
fallegur;168136;lo;alm;fallegustu;EVB-KK-NFFT
fallegur;168136;lo;alm;fallegustu;EVB-KK-??FFT
fallegur;168136;lo;alm;fallegustu;EVB-KK-??GFFT
fallegur;168136;lo;alm;fallegustu;EVB-KK-EFFT
fallegur;168136;lo;alm;fallegasta;EVB-KVK-NFET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-??FET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-??GFET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-EFET
fallegur;168136;lo;alm;fallegustu;EVB-KVK-NFFT
fallegur;168136;lo;alm;fallegustu;EVB-KVK-??FFT
fallegur;168136;lo;alm;fallegustu;EVB-KVK-??GFFT
fallegur;168136;lo;alm;fallegustu;EVB-KVK-EFFT
fallegur;168136;lo;alm;fallegasta;EVB-HK-NFET
fallegur;168136;lo;alm;fallegasta;EVB-HK-??FET
fallegur;168136;lo;alm;fallegasta;EVB-HK-??GFET
fallegur;168136;lo;alm;fallegasta;EVB-HK-EFET
fallegur;168136;lo;alm;fallegustu;EVB-HK-NFFT
fallegur;168136;lo;alm;fallegustu;EVB-HK-??FFT
fallegur;168136;lo;alm;fallegustu;EVB-HK-??GFFT
fallegur;168136;lo;alm;fallegustu;EVB-HK-EFFT
l??ra;435046;so;alm;l??ra;GM-NH
l??ra;435046;so;alm;l??ri;GM-FH-NT-1P-ET
l??ra;435046;so;alm;l??rir;GM-FH-NT-2P-ET
l??ra;435046;so;alm;l??rir;GM-FH-NT-3P-ET
l??ra;435046;so;alm;l??rum;GM-FH-NT-1P-FT
l??ra;435046;so;alm;l??ri??;GM-FH-NT-2P-FT
l??ra;435046;so;alm;l??ra;GM-FH-NT-3P-FT
l??ra;435046;so;alm;l??r??i;GM-FH-??T-1P-ET
l??ra;435046;so;alm;l??r??ir;GM-FH-??T-2P-ET
l??ra;435046;so;alm;l??r??i;GM-FH-??T-3P-ET
l??ra;435046;so;alm;l??r??um;GM-FH-??T-1P-FT
l??ra;435046;so;alm;l??r??u??;GM-FH-??T-2P-FT
l??ra;435046;so;alm;l??r??u;GM-FH-??T-3P-FT
l??ra;435046;so;alm;l??ri;GM-VH-NT-1P-ET
l??ra;435046;so;alm;l??rir;GM-VH-NT-2P-ET
l??ra;435046;so;alm;l??ri;GM-VH-NT-3P-ET
l??ra;435046;so;alm;l??rum;GM-VH-NT-1P-FT
l??ra;435046;so;alm;l??ri??;GM-VH-NT-2P-FT
l??ra;435046;so;alm;l??ri;GM-VH-NT-3P-FT
l??ra;435046;so;alm;l??r??i;GM-VH-??T-1P-ET
l??ra;435046;so;alm;l??r??ir;GM-VH-??T-2P-ET
l??ra;435046;so;alm;l??r??i;GM-VH-??T-3P-ET
l??ra;435046;so;alm;l??r??um;GM-VH-??T-1P-FT
l??ra;435046;so;alm;l??r??u??;GM-VH-??T-2P-FT
l??ra;435046;so;alm;l??r??u;GM-VH-??T-3P-FT
l??ra;435046;so;alm;l??rast;MM-NH
l??ra;435046;so;alm;l??rist;MM-FH-NT-1P-ET
l??ra;435046;so;alm;l??rist;MM-FH-NT-2P-ET
l??ra;435046;so;alm;l??rist;MM-FH-NT-3P-ET
l??ra;435046;so;alm;l??rumst;MM-FH-NT-1P-FT
l??ra;435046;so;alm;l??rist;MM-FH-NT-2P-FT
l??ra;435046;so;alm;l??rast;MM-FH-NT-3P-FT
l??ra;435046;so;alm;l??r??ist;MM-FH-??T-1P-ET
l??ra;435046;so;alm;l??r??ist;MM-FH-??T-2P-ET
l??ra;435046;so;alm;l??r??ist;MM-FH-??T-3P-ET
l??ra;435046;so;alm;l??r??umst;MM-FH-??T-1P-FT
l??ra;435046;so;alm;l??r??ust;MM-FH-??T-2P-FT
l??ra;435046;so;alm;l??r??ust;MM-FH-??T-3P-FT
l??ra;435046;so;alm;l??rist;MM-VH-NT-1P-ET
l??ra;435046;so;alm;l??rist;MM-VH-NT-2P-ET
l??ra;435046;so;alm;l??rist;MM-VH-NT-3P-ET
l??ra;435046;so;alm;l??rumst;MM-VH-NT-1P-FT
l??ra;435046;so;alm;l??rist;MM-VH-NT-2P-FT
l??ra;435046;so;alm;l??rist;MM-VH-NT-3P-FT
l??ra;435046;so;alm;l??r??ist;MM-VH-??T-1P-ET
l??ra;435046;so;alm;l??r??ist;MM-VH-??T-2P-ET
l??ra;435046;so;alm;l??r??ist;MM-VH-??T-3P-ET
l??ra;435046;so;alm;l??r??umst;MM-VH-??T-1P-FT
l??ra;435046;so;alm;l??r??ust;MM-VH-??T-2P-FT
l??ra;435046;so;alm;l??r??ust;MM-VH-??T-3P-FT
l??ra;435046;so;alm;l??r;GM-BH-ST
l??ra;435046;so;alm;l??r??u;GM-BH-ET
l??ra;435046;so;alm;l??ri??;GM-BH-FT
l??ra;435046;so;alm;l??randi;LHNT
l??ra;435046;so;alm;l??rt;GM-SAGNB
l??ra;435046;so;alm;l??rst;MM-SAGNB
l??ra;435046;so;alm;l??r??ur;LH??T-SB-KK-NFET
l??ra;435046;so;alm;l??r??an;LH??T-SB-KK-??FET
l??ra;435046;so;alm;l??r??um;LH??T-SB-KK-??GFET
l??ra;435046;so;alm;l??r??s;LH??T-SB-KK-EFET
l??ra;435046;so;alm;l??r??ir;LH??T-SB-KK-NFFT
l??ra;435046;so;alm;l??r??a;LH??T-SB-KK-??FFT
l??ra;435046;so;alm;l??r??um;LH??T-SB-KK-??GFFT
l??ra;435046;so;alm;l??r??ra;LH??T-SB-KK-EFFT
l??ra;435046;so;alm;l??r??;LH??T-SB-KVK-NFET
l??ra;435046;so;alm;l??r??a;LH??T-SB-KVK-??FET
l??ra;435046;so;alm;l??r??ri;LH??T-SB-KVK-??GFET
l??ra;435046;so;alm;l??r??rar;LH??T-SB-KVK-EFET
l??ra;435046;so;alm;l??r??ar;LH??T-SB-KVK-NFFT
l??ra;435046;so;alm;l??r??ar;LH??T-SB-KVK-??FFT
l??ra;435046;so;alm;l??r??um;LH??T-SB-KVK-??GFFT
l??ra;435046;so;alm;l??r??ra;LH??T-SB-KVK-EFFT
l??ra;435046;so;alm;l??rt;LH??T-SB-HK-NFET
l??ra;435046;so;alm;l??rt;LH??T-SB-HK-??FET
l??ra;435046;so;alm;l??r??u;LH??T-SB-HK-??GFET
l??ra;435046;so;alm;l??r??s;LH??T-SB-HK-EFET
l??ra;435046;so;alm;l??r??;LH??T-SB-HK-NFFT
l??ra;435046;so;alm;l??r??;LH??T-SB-HK-??FFT
l??ra;435046;so;alm;l??r??um;LH??T-SB-HK-??GFFT
l??ra;435046;so;alm;l??r??ra;LH??T-SB-HK-EFFT
l??ra;435046;so;alm;l??r??i;LH??T-VB-KK-NFET
l??ra;435046;so;alm;l??r??a;LH??T-VB-KK-??FET
l??ra;435046;so;alm;l??r??a;LH??T-VB-KK-??GFET
l??ra;435046;so;alm;l??r??a;LH??T-VB-KK-EFET
l??ra;435046;so;alm;l??r??u;LH??T-VB-KK-NFFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-KK-??FFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-KK-??GFFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-KK-EFFT
l??ra;435046;so;alm;l??r??a;LH??T-VB-KVK-NFET
l??ra;435046;so;alm;l??r??u;LH??T-VB-KVK-??FET
l??ra;435046;so;alm;l??r??u;LH??T-VB-KVK-??GFET
l??ra;435046;so;alm;l??r??u;LH??T-VB-KVK-EFET
l??ra;435046;so;alm;l??r??u;LH??T-VB-KVK-NFFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-KVK-??FFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-KVK-??GFFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-KVK-EFFT
l??ra;435046;so;alm;l??r??a;LH??T-VB-HK-NFET
l??ra;435046;so;alm;l??r??a;LH??T-VB-HK-??FET
l??ra;435046;so;alm;l??r??a;LH??T-VB-HK-??GFET
l??ra;435046;so;alm;l??r??a;LH??T-VB-HK-EFET
l??ra;435046;so;alm;l??r??u;LH??T-VB-HK-NFFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-HK-??FFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-HK-??GFFT
l??ra;435046;so;alm;l??r??u;LH??T-VB-HK-EFFT
l??ra;435046;so;alm;l??rist;OP-??GF-MM-FH-NT-1P-ET
l??ra;435046;so;alm;l??rist;OP-??GF-MM-FH-NT-1P-FT
l??ra;435046;so;alm;l??rist;OP-??GF-MM-FH-NT-2P-ET
l??ra;435046;so;alm;l??rist;OP-??GF-MM-FH-NT-2P-FT
l??ra;435046;so;alm;l??rist;OP-??GF-MM-FH-NT-3P-ET
l??ra;435046;so;alm;l??rist;OP-??GF-MM-FH-NT-3P-FT
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-FH-??T-1P-ET
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-FH-??T-1P-FT
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-FH-??T-2P-ET
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-FH-??T-2P-FT
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-FH-??T-3P-ET
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-FH-??T-3P-FT
l??ra;435046;so;alm;l??rist;OP-??GF-MM-VH-NT-1P-ET
l??ra;435046;so;alm;l??rist;OP-??GF-MM-VH-NT-1P-FT
l??ra;435046;so;alm;l??rist;OP-??GF-MM-VH-NT-2P-ET
l??ra;435046;so;alm;l??rist;OP-??GF-MM-VH-NT-2P-FT
l??ra;435046;so;alm;l??rist;OP-??GF-MM-VH-NT-3P-ET
l??ra;435046;so;alm;l??rist;OP-??GF-MM-VH-NT-3P-FT
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-VH-??T-1P-ET
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-VH-??T-1P-FT
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-VH-??T-2P-ET
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-VH-??T-2P-FT
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-VH-??T-3P-ET
l??ra;435046;so;alm;l??r??ist;OP-??GF-MM-VH-??T-3P-FT
??g;403780;pfn;alm;??g;NFET
??g;403780;pfn;alm;mig;??FET
??g;403780;pfn;alm;m??r;??GFET
??g;403780;pfn;alm;m??n;EFET
??g;403780;pfn;alm;vi??;NFFT
??g;403780;pfn;alm;okkur;??FFT
??g;403780;pfn;alm;okkur;??GFFT
??g;403780;pfn;alm;okkar;EFFT
????;403782;pfn;alm;????;NFET
????;403782;pfn;alm;??ig;??FET
????;403782;pfn;alm;????r;??GFET
????;403782;pfn;alm;????n;EFET
????;403782;pfn;alm;??i??;NFFT
????;403782;pfn;alm;ykkur;??FFT
????;403782;pfn;alm;ykkur;??GFFT
????;403782;pfn;alm;ykkar;EFFT
hann;403784;pfn;alm;hann;NFET
hann;403784;pfn;alm;hann;??FET
hann;403784;pfn;alm;honum;??GFET
hann;403784;pfn;alm;hans;EFET
hann;403784;pfn;alm;??eir;NFFT
hann;403784;pfn;alm;????;??FFT
hann;403784;pfn;alm;??eim;??GFFT
hann;403784;pfn;alm;??eirra;EFFT
h??n;403785;pfn;alm;h??n;NFET
h??n;403785;pfn;alm;hana;??FET
h??n;403785;pfn;alm;henni;??GFET
h??n;403785;pfn;alm;hennar;EFET
h??n;403785;pfn;alm;????r;NFFT
h??n;403785;pfn;alm;????r;??FFT
h??n;403785;pfn;alm;??eim;??GFFT
h??n;403785;pfn;alm;??eirra;EFFT
??a??;403786;pfn;alm;??a??;NFET
??a??;403786;pfn;alm;??a??;??FET
??a??;403786;pfn;alm;??v??;??GFET
??a??;403786;pfn;alm;??ess;EFET
??a??;403786;pfn;alm;??au;NFFT
??a??;403786;pfn;alm;??au;??FFT
??a??;403786;pfn;alm;??eim;??GFFT
??a??;403786;pfn;alm;??eirra;EFFT";

    #[test]
    pub fn gets_noun_entry() {
        let bin_data = BinData::load(TEST_DATA.as_bytes()).unwrap();
        let noun_entry = bin_data.noun("a??alhenda").unwrap();

        assert_eq!(Gender::Feminine, noun_entry.gender);
        // Singular
        assert_eq!("a??alhenda", noun_entry.nom_sg.unwrap());
        assert_eq!("a??alhendan", noun_entry.nom_sg_def.unwrap());
        assert_eq!("a??alhendu", noun_entry.acc_sg.unwrap());
        assert_eq!("a??alhenduna", noun_entry.acc_sg_def.unwrap());
        assert_eq!("a??alhendu", noun_entry.dat_sg.unwrap());
        assert_eq!("a??alhendunni", noun_entry.dat_sg_def.unwrap());
        assert_eq!("a??alhendu", noun_entry.gen_sg.unwrap());
        assert_eq!("a??alhendunnar", noun_entry.gen_sg_def.unwrap());
        // Plural
        assert_eq!("a??alhendur", noun_entry.nom_pl.unwrap());
        assert_eq!("a??alhendurnar", noun_entry.nom_pl_def.unwrap());
        assert_eq!("a??alhendur", noun_entry.acc_pl.unwrap());
        assert_eq!("a??alhendurnar", noun_entry.acc_pl_def.unwrap());
        assert_eq!("a??alhendum", noun_entry.dat_pl.unwrap());
        assert_eq!("a??alhendunum", noun_entry.dat_pl_def.unwrap());
        assert_eq!("a??alhendna", noun_entry.gen_pl.unwrap());
        assert_eq!("a??alhendnanna", noun_entry.gen_pl_def.unwrap());
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
        let verb_entry = bin_data.verb("l??ra").unwrap();

        assert_eq!("l??ri", verb_entry.pres_ind_first_sg.unwrap());
        assert_eq!("l??rir", verb_entry.pres_ind_second_sg.unwrap());
        assert_eq!("l??rir", verb_entry.pres_ind_third_sg.unwrap());

        assert_eq!("l??rum", verb_entry.pres_ind_first_pl.unwrap());
        assert_eq!("l??ri??", verb_entry.pres_ind_second_pl.unwrap());
        assert_eq!("l??ra", verb_entry.pres_ind_third_pl.unwrap());

        assert_eq!("l??r??i", verb_entry.past_ind_first_sg.unwrap());
        assert_eq!("l??r??ir", verb_entry.past_ind_second_sg.unwrap());
        assert_eq!("l??r??i", verb_entry.past_ind_third_sg.unwrap());

        assert_eq!("l??r??um", verb_entry.past_ind_first_pl.unwrap());
        assert_eq!("l??r??u??", verb_entry.past_ind_second_pl.unwrap());
        assert_eq!("l??r??u", verb_entry.past_ind_third_pl.unwrap());
    }

    #[test]
    pub fn gets_pronoun_entries() {
        let bin_data = BinData::load(TEST_DATA.as_bytes()).unwrap();

        let e = bin_data.pronoun("??g").unwrap();
        assert_eq!("??g", e.nom.unwrap());
        assert_eq!("mig", e.acc.unwrap());
        assert_eq!("m??r", e.dat.unwrap());
        assert_eq!("m??n", e.gen.unwrap());

        let e = bin_data.pronoun("????").unwrap();
        assert_eq!("????", e.nom.unwrap());
        assert_eq!("??ig", e.acc.unwrap());
        assert_eq!("????r", e.dat.unwrap());
        assert_eq!("????n", e.gen.unwrap());

        let e = bin_data.pronoun("hann").unwrap();
        assert_eq!("hann", e.nom.unwrap());
        assert_eq!("hann", e.acc.unwrap());
        assert_eq!("honum", e.dat.unwrap());
        assert_eq!("hans", e.gen.unwrap());

        let e = bin_data.pronoun("h??n").unwrap();
        assert_eq!("h??n", e.nom.unwrap());
        assert_eq!("hana", e.acc.unwrap());
        assert_eq!("henni", e.dat.unwrap());
        assert_eq!("hennar", e.gen.unwrap());

        let e = bin_data.pronoun("??a??").unwrap();
        assert_eq!("??a??", e.nom.unwrap());
        assert_eq!("??a??", e.acc.unwrap());
        assert_eq!("??v??", e.dat.unwrap());
        assert_eq!("??ess", e.gen.unwrap());

        let e = bin_data.pronoun("vi??").unwrap();
        assert_eq!("vi??", e.nom.unwrap());
        assert_eq!("okkur", e.acc.unwrap());
        assert_eq!("okkur", e.dat.unwrap());
        assert_eq!("okkar", e.gen.unwrap());

        let e = bin_data.pronoun("??i??").unwrap();
        assert_eq!("??i??", e.nom.unwrap());
        assert_eq!("ykkur", e.acc.unwrap());
        assert_eq!("ykkur", e.dat.unwrap());
        assert_eq!("ykkar", e.gen.unwrap());

        let e = bin_data.pronoun("??eir").unwrap();
        assert_eq!("??eir", e.nom.unwrap());
        assert_eq!("????", e.acc.unwrap());
        assert_eq!("??eim", e.dat.unwrap());
        assert_eq!("??eirra", e.gen.unwrap());

        let e = bin_data.pronoun("????r").unwrap();
        assert_eq!("????r", e.nom.unwrap());
        assert_eq!("????r", e.acc.unwrap());
        assert_eq!("??eim", e.dat.unwrap());
        assert_eq!("??eirra", e.gen.unwrap());

        let e = bin_data.pronoun("??au").unwrap();
        assert_eq!("??au", e.nom.unwrap());
        assert_eq!("??au", e.acc.unwrap());
        assert_eq!("??eim", e.dat.unwrap());
        assert_eq!("??eirra", e.gen.unwrap());
    }
}
