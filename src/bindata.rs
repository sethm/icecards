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

#[derive(Debug, Eq, PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
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

    pub fn adjective(&self, root: &str) -> Option<AdjectiveEntry> {
        let entries = self.data.get(root);

        match entries {
            Some(entries) => {
                let entries =
                    entries.iter().filter(|&e| is_adjective(e)).collect::<Vec<&BinEntry>>();

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
                let entries = entries.iter().filter(|&f| is_noun(f)).collect::<Vec<&BinEntry>>();

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
}

fn is_adjective(entry: &BinEntry) -> bool {
    entry.word_class == "lo"
}

fn is_noun(entry: &BinEntry) -> bool {
    entry.word_class == "kk" || entry.word_class == "kvk" || entry.word_class == "hk"
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
fallegur;168136;lo;alm;fallegustu;EVB-HK-EFFT";

    #[test]
    pub fn loads_bin_data() {
        let bin_data = BinData::load(TEST_DATA.as_bytes()).unwrap();

        assert_eq!(3, bin_data.data.len());

        assert!(bin_data.data.contains_key("aðalhellir"));
        assert!(bin_data.data.contains_key("aðalhenda"));

        assert_eq!(16, bin_data.data.get("aðalhellir").unwrap().len());
        assert_eq!(18, bin_data.data.get("aðalhenda").unwrap().len());
        assert_eq!(120, bin_data.data.get("fallegur").unwrap().len());
    }

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
}