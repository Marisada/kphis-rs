use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode, PartialEq)]
pub struct I10Index {
    /// (keywords, pointer)
    pub diagnosis: HashMap<String, Arc<I10Pointer>>,
    /// (keywords, pointer)
    pub external: HashMap<String, Arc<I10Pointer>>,
    /// (keywords, pointer)
    pub substance: HashMap<String, Arc<I10Pointer>>,
    /// (asterisk, daggers)
    pub aster_dagger: HashMap<String, HashSet<String>>,
}

impl I10Index {
    pub(crate) fn new() -> Self {
        let bytes = include_bytes!("../../dump/i10-index.dump");
        bitcode::decode(bytes).expect("Cannot decode I10_INDEX binary")
    }

    pub fn search_diagnosis(&self, text: &str) -> Vec<((String, Arc<I10Pointer>), f32, u8)> {
        // I10vx::fuzzy_search_best_n(&text, &source, 20)
        I10Pointer::contains_search_best_n(&text, &self.diagnosis, 20)
    }

    pub fn search_external(&self, text: &str) -> Vec<((String, Arc<I10Pointer>), f32, u8)> {
        // I10vx::fuzzy_search_best_n(&text, &source, 20)
        I10Pointer::contains_search_best_n(&text, &self.external, 20)
    }

    pub fn search_substance(&self, text: &str) -> Vec<((String, Arc<I10Pointer>), f32, u8)> {
        // I10vx::fuzzy_search_best_n(&text, &source, 20)
        I10Pointer::contains_search_best_n(&text, &self.substance, 20)
    }

    /// use ICD10 without dot
    /// find asterisk in codes -> find dagger in codes
    pub fn find_dagger_aster_pairs(&self, codes: &HashSet<String>) -> Vec<(Option<String>, String)> {
        codes
            .iter()
            .filter_map(|aster| {
                let mut rest = codes.clone();
                rest.remove(aster);
                self.aster_dagger.get(aster).map(|daggers| {
                    let results = daggers.intersection(&rest).collect::<Vec<&String>>();
                    if results.is_empty() {
                        // Asterisk without Dagger matched
                        vec![(None, aster.to_owned())]
                    } else {
                        results.into_iter().map(|dagger| (Some(dagger.to_owned()), aster.to_owned())).collect()
                    }
                })
            })
            .flatten()
            .collect()
    }
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode, PartialEq)]
pub struct I10Pointer {
    /// ` – see `, ` – see also `, ` – see categories `, ` – code to `
    pub note: Option<Note>,
    /// ` (see ..)`, ` (see also ..)`
    pub bracket_notes: Vec<Note>,
    /// ICD10 code
    pub code: Option<Code>,
}

impl I10Pointer {
    /// return ((String, Arc<I10Pointer>), rate, 1)
    pub fn contains_search_best_n(text: &str, list: &HashMap<String, Arc<Self>>, n: usize) -> Vec<((String, Arc<Self>), f32, u8)> {
        let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
        let keywords = text_lo
            .split(" ")
            .filter_map(|s| {
                let exact = s.trim();
                (!exact.is_empty()).then(|| exact)
            })
            .collect::<Vec<&str>>();
        let keywords_len = keywords.len() as f32;
        let mut res = list
            .iter()
            .filter_map(|(detail, pointer)| {
                let detail_lo = detail.to_ascii_lowercase();
                let (subs, mains): (Vec<&str>, Vec<&str>) = detail_lo.split(", ").partition(|s| s.starts_with('('));
                let main = mains.concat();
                let sub = subs.concat();
                let main_len = mains.len() as f32;
                let sub_len = subs.len() as f32;
                let mut res = 0.0;
                let mut col = 0;
                for keyword in keywords.iter() {
                    if main_len > 0.0 && main.contains(keyword) {
                        res += (1.0 / keywords_len) + (0.2 / main_len);
                        col = 1;
                    }
                    if sub_len > 0.0 && sub.contains(keyword) {
                        res += (0.5 / keywords_len) + (0.1 / sub_len);
                        col = 1;
                    }
                }
                (res > 0.0).then(|| ((detail, pointer), res, col))
            })
            .collect::<Vec<((&String, &Arc<I10Pointer>), f32, u8)>>();
        res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
        res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
    }
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode, PartialEq)]
pub enum Note {
    /// keywords (with commas/or)
    See(String),
    /// keywords (with commas/or)
    SeeAlso(String),
    /// code range
    SeeCategory(String),
    /// code text (include 'code' word)
    Code(String),
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode, PartialEq)]
pub enum Code {
    /// ex. R74.8 or R86.-
    Single(String),
    /// (Dagger, Asterisk)
    DaggerAster(String, String),
    /// 5 codes (for Neoplasm in Section I only)
    /// - Primary: C00-C75, C81-C97
    /// - Secondary: C76-C80
    /// - In Situ: D00-D09
    /// - Benign: D10-D36
    /// - Uncertain: D37-D48
    Neoplasm(Vec<String>, Option<NeoplasmTag>),
    /// 5 codes (for Section III only)
    /// - Chapter XIX: T36-T65 (Poison and toxic effect)
    /// - Accidental: X40-X49
    /// - Intention self-harm: X60-X69
    /// - Undetermine intent: Y10-Y19
    /// - Adverse effect in Rx: Y40-Y59
    Substance(Vec<String>),
}

#[derive(Debug, Clone, bitcode::Encode, bitcode::Decode, PartialEq)]
pub enum NeoplasmTag {
    /// as `#` means `Skin of this site`
    Hash,
    /// as /u{2727} means `Preferred C79.5`
    Star,
}
