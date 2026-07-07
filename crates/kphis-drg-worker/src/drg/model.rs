#![allow(dead_code)]

use serde::de;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    sync::Arc,
};
use time::{Date, Month, PrimitiveDateTime};

use kphis_util::{
    british_american::TRANSLATOR,
    fuzzy_search::{fuzzy_compare, trigrams},
    util::{next_key, sanity_space},
};

use super::{dcl::dcl_pcl, grouper::Grouper, mdc::Mdc};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum GrouperError {
    EmptyCode,
    InvalidGender,
    InvalidAgeY,
    InvalidAdmWt,
    InvalidLos,
    InvalidDchType,

    PDxInSDxs,
    SDxOverload,
    ProcOverload,

    I10NotFound(String),
    I10UnAccPDx(String),
    // female == true
    I10WrongGender(String, bool),
    I10NoGender(String),
    I10ConflictAgeY(String),
    I10ConflictAgeD(String, u16),
    I10NoAgeD(String),
    // // with Asterisks hint
    // I10PDxNoAsterisk(HashSet<String>),
    // // with (Asterisk, Daggers hint)
    // I10SDxSuggestSDxAsDagger((String, Vec<String>)),
    // // with (Asterisk, Daggers hint)
    // I10SDxInfoDaggers((String, Vec<String>)),
    ProcNotFound(String),
    // female == true
    ProcWrongGender(String, bool),
    ProcNoGender(String),

    ErrorSerdeJson(String),
}

impl GrouperError {
    pub fn string(&self) -> String {
        match self {
            // input check
            Self::EmptyCode => "พบ PDx, Sdx หรือ Proc เป็นค่าว่าง".into(),
            Self::InvalidGender => "เพศไม่ถูกต้อง ('1':ชาย หรือ '2':หญิง)".into(),
            Self::InvalidAgeY => "อายุไม่ถูกต้อง (ไม่น้อยกว่า 0 และไม่เกิน 124 ปี)".into(),
            Self::InvalidAdmWt => "ทารกอายุน้อยกว่า 28 วัน ต้องมีน้ำหนักมากกว่า 300 กรัม".into(),
            Self::InvalidLos => "วันนอนไม่ถูกต้อง (ไม่น้อยกว่า 0)".into(),
            Self::InvalidDchType => "ประเภทการจำหน่ายไม่ถูกต้อง".into(),
            Self::PDxInSDxs => "พบ PDx ซ้ำในรายการ SDx".into(),
            Self::SDxOverload => "SDx เกิน 12 รายการ".into(),
            Self::ProcOverload => "Proc เกิน 20 รายการ".into(),
            // i10 check
            Self::I10NotFound(code) => [code, " ไม่สามารถใช้ได้"].concat(),
            Self::I10UnAccPDx(code) => [code, " ห้ามเป็น PDx"].concat(),
            Self::I10WrongGender(code, is_female) => ["ผู้ป่วยเพศ", if *is_female { "หญิง" } else { "ชาย" }, " ห้ามใช้รหัส ", code].concat(),
            Self::I10NoGender(code) => [code, " ต้องระบุเพศผู้ป่วย"].concat(),
            Self::I10ConflictAgeY(code) => ["อายุของผู้ป่วย ห้ามใช้รหัส ", code].concat(),
            Self::I10ConflictAgeD(code, min_day) => [code, " ต้องมีอายุอย่างน้อย ", &min_day.to_string(), " วัน"].concat(),
            Self::I10NoAgeD(code) => [code, " ต้องระบุอายุผู้ป่วย(วัน)"].concat(),
            // // dagger-asterisk check
            // Self::I10PDxNoAsterisk(asterisks) => [
            //     "PDx ต้องมีรหัสคู่ (Dagger-Asterisk system) เป็น SDx เช่น ",
            //     &asterisks
            //         .to_owned()
            //         .into_iter()
            //         .collect::<Vec<String>>()
            //         .join(", "),
            // ]
            // .concat(),
            // Self::I10SDxSuggestSDxAsDagger((asterisk, daggers)) => [
            //     "ข้อเสนอแนะ: SDx ",
            //     asterisk,
            //     " มี SDx อื่นในรายการที่สามารถใช้เป็น PDx ได้ (ตามระบบรหัสคู่: Dagger-Asterisk system) ได้แก่ ",
            //     &daggers.join(", "),
            // ]
            // .concat(),
            // Self::I10SDxInfoDaggers((asterisk, daggers)) => [
            //     "ข้อมูลเพิ่มเติม: SDx ",
            //     asterisk,
            //     " มีรหัสคู่ (Dagger-Asterisk system) เพื่อใช้เป็น PDx ได้แก่ ",
            //     &daggers.join(", "),
            // ]
            // .concat(),
            // proc check
            Self::ProcNotFound(code) => [code, " ไม่สามารถใช้ได้"].concat(),
            Self::ProcWrongGender(code, is_female) => ["ผู้ป่วยเพศ", if *is_female { "หญิง" } else { "ชาย" }, " ห้ามใช้รหัส ", code].concat(),
            Self::ProcNoGender(code) => [code, " ต้องระบุเพศผู้ป่วย"].concat(),
            // system error
            Self::ErrorSerdeJson(error) => ["Invalid input: ", error].concat(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrouperInput {
    pub pdx: String,
    pub sdxs: HashSet<String>,
    pub procs: HashSet<String>,
    pub gender: Option<String>,
    pub age_y: u8,
    pub age_d: Option<u16>,
    pub adm_wt: Option<u16>,
    pub los: u32,
    /// 01,02,03,04,05,08,09
    pub dch_type: String,
    pub origin: GrouperOrigin,
}

impl GrouperInput {
    pub fn new(
        pdx: &str,
        sdxs: &[String],
        procs: &[String],
        gender: &Option<String>,
        dob: Option<PrimitiveDateTime>,
        adm_wt: Option<u16>, // grams
        adm_date: Option<PrimitiveDateTime>,
        dch_date: Option<PrimitiveDateTime>,
        dch_type: &str,
        leave_day: u32,
    ) -> Result<Self, Vec<GrouperError>> {
        let mut results = Vec::with_capacity(9);
        // parse data
        let (age_y, age_d) = if let (Some(birth_day), Some(admit_date)) = (dob, adm_date) {
            calculate_date_yd(birth_day, admit_date)
        } else {
            (None, None)
        };
        // check adm_wt
        if age_y == Some(0) && age_d.map(|d| d < 28).unwrap_or_default() {
            if adm_wt.is_none() || adm_wt.map(|wt| wt < 300).unwrap_or_default() {
                results.push(GrouperError::InvalidLos);
            }
        }
        // check los
        let los = if let (Some(admit_date), Some(discharge_date)) = (adm_date, dch_date) {
            let los_minutes = (discharge_date - admit_date).whole_minutes();
            if los_minutes < 0 {
                results.push(GrouperError::InvalidLos);
                0
            } else {
                let act_los_minutes = los_minutes - (leave_day as i64 * 1440);
                if act_los_minutes < 0 {
                    results.push(GrouperError::InvalidLos);
                    0
                } else {
                    let floor = (act_los_minutes / 1440) as u32;
                    if floor == 0 {
                        // < 24 hr is 0 day
                        0
                    } else if act_los_minutes % 1440 > 360 {
                        // ceiling when over 6 hours
                        floor + 1
                    } else {
                        floor
                    }
                }
            }
        } else {
            0
        };
        // remove special proc ex: IMC, HOMEWARD
        let mut procs_set = procs.to_owned().into_iter().collect::<HashSet<String>>();
        procs_set.remove("IMC");
        procs_set.remove("HOMEWARD");
        // check validity
        if pdx.is_empty() || sdxs.iter().any(|s| s.is_empty()) || procs.iter().any(|s| s.is_empty()) {
            results.push(GrouperError::EmptyCode);
        }
        if sdxs.contains(&pdx.to_owned()) {
            results.push(GrouperError::PDxInSDxs);
        }
        if sdxs.len() > 12 {
            results.push(GrouperError::SDxOverload);
        }
        if procs_set.len() > 20 {
            results.push(GrouperError::ProcOverload);
        }
        if let Some(s) = gender.as_ref()
            && !["1", "2"].contains(&s.as_str())
        {
            results.push(GrouperError::InvalidGender);
        }
        if let Some(y) = age_y {
            if y > 124 {
                results.push(GrouperError::InvalidAgeY);
            }
        } else {
            results.push(GrouperError::InvalidAgeY);
        }
        if !["01", "02", "03", "04", "05", "08", "09"].contains(&dch_type) {
            results.push(GrouperError::InvalidDchType);
        }

        if results.is_empty() {
            Ok(Self {
                pdx: pdx.to_owned(),
                sdxs: sdxs.to_owned().into_iter().collect(),
                procs: procs_set,
                gender: gender.to_owned(),
                age_y: age_y.unwrap_or_default(),
                age_d,
                adm_wt,
                los,
                dch_type: dch_type.to_owned(),
                origin: GrouperOrigin::Original,
            })
        } else {
            Err(results)
        }
    }

    pub(crate) fn clone_swap_pdx(&self, sdx: &String) -> Self {
        let mut swap = self.clone();
        swap.sdxs.remove(sdx);
        swap.sdxs.insert(self.pdx.to_owned());
        swap.pdx = sdx.to_owned();
        swap.origin = GrouperOrigin::UorpSwap;
        swap
    }

    pub(crate) fn clone_with(&self, pdx: String, sdx: String) -> Self {
        if self.pdx == pdx && self.sdxs.contains(&sdx) {
            self.clone()
        } else if self.sdxs.contains(&pdx) && self.pdx == sdx {
            let mut swap = self.clone();
            swap.pdx = pdx.to_owned();
            swap.sdxs.remove(&pdx);
            swap.sdxs.insert(sdx);
            swap.origin = GrouperOrigin::GrouperSwap;
            swap
        } else {
            let mut possible = self.clone();
            possible.pdx = pdx.to_owned();
            possible.sdxs.remove(&pdx);
            possible.sdxs.insert(sdx);
            possible.origin = GrouperOrigin::Possible;
            possible
        }
    }

    pub(crate) fn to_drg_output(&self, drg: Drg) -> GrouperOutputDrg {
        GrouperOutputDrg { drg, source: self.clone() }
    }
}

#[cfg(test)]
impl Default for GrouperInput {
    fn default() -> Self {
        Self {
            pdx: String::from("A099"),
            sdxs: HashSet::new(),
            procs: HashSet::new(),
            gender: None,
            age_y: 20,
            age_d: None,
            adm_wt: None,
            los: 3,
            dch_type: String::from("01"),
            origin: GrouperOrigin::Original,
        }
    }
}

#[cfg(test)]
impl GrouperInput {
    pub(crate) fn set_pdx(&mut self, pdx: &str) -> Self {
        self.pdx = pdx.to_owned();
        self.to_owned()
    }

    pub(crate) fn set_sdxs(&mut self, sdxs: &[&str]) -> Self {
        self.sdxs = sdxs.iter().map(|s| s.to_string()).collect::<HashSet<String>>();
        self.to_owned()
    }

    pub(crate) fn set_procs(&mut self, procs: &[&str]) -> Self {
        self.procs = procs.iter().map(|s| s.to_string()).collect::<HashSet<String>>();
        self.to_owned()
    }

    pub(crate) fn set_los(&mut self, los: u32) -> Self {
        self.los = los;
        self.to_owned()
    }

    pub(crate) fn set_age_y(&mut self, age_y: u8) -> Self {
        self.age_y = age_y;
        self.to_owned()
    }

    pub(crate) fn set_age_d(&mut self, age_d: Option<u16>) -> Self {
        self.age_d = age_d;
        self.to_owned()
    }

    pub(crate) fn set_gender(&mut self, gender: Option<String>) -> Self {
        self.gender = gender;
        self.to_owned()
    }

    // pub(crate) fn set_adm_wt(&mut self, adm_wt: Option<u16>) -> Self {
    //     self.adm_wt = adm_wt;
    //     self.to_owned()
    // }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GrouperOrigin {
    Original,
    GrouperSwap,
    UorpSwap,
    Possible,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GrouperOutput {
    pub drg: Vec<GrouperOutputDrg>,
    pub errors: Vec<GrouperError>,
    pub(crate) mdc_log: Vec<MdcResult>,
}

impl GrouperOutput {
    pub(crate) fn new() -> Self {
        Self {
            drg: Vec::new(),
            errors: Vec::new(),
            mdc_log: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GrouperOutputDrg {
    pub drg: Drg,
    pub source: GrouperInput,
}

impl GrouperOutputDrg {
    pub fn adj_rw(&self) -> f32 {
        self.drg.adj_rw(self.source.los)
    }
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub enum I10e {
    All(Arc<I10>),
    MaleFemale(Arc<I10>, Arc<I10>),
}

impl I10e {
    pub fn new(i10: Arc<I10>) -> Self {
        Self::All(i10)
    }
    /// Some code has 2 MDC (MDC 11,12 for M and MDC 13 for F)
    pub fn to_pair(&mut self, i10: Arc<I10>) -> Self {
        match self {
            Self::All(old) => match (old.mdc.as_str(), i10.mdc.as_str()) {
                ("11", "13") | ("12", "13") => Self::MaleFemale(old.clone(), i10.clone()),
                ("13", "11") | ("13", "12") => Self::MaleFemale(i10.clone(), old.clone()),
                _ => panic!("Invalid MDC gender-pair"),
            },
            Self::MaleFemale(_, _) => panic!("Cannot add MDC gender-pair again"),
        }
    }
    /// return `None` when `I10` has gender related MDC but `gender` argument is `None` or invalid(not `1` or `2`)
    pub(crate) fn get_inner_by_gender(&self, gender: &Option<String>) -> Option<&Arc<I10>> {
        match self {
            Self::All(i10) => Some(i10),
            Self::MaleFemale(male, female) => {
                if let Some(s) = gender.as_ref() {
                    match s.as_str() {
                        "1" => Some(male),
                        "2" => Some(female),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, bitcode::Encode, bitcode::Decode)]
pub struct I10 {
    /// [ICD10-TM] Book 1 - Appendix E: Diagnosis Codes - Code
    #[serde(rename = "CODE,C,6")]
    pub code: String,
    /// [01-25,30] Book 1 - Appendix E: Diagnosis Codes - MDC
    #[serde(rename = "MDC,C,2")]
    pub mdc: String,
    // /// [1A-19N,""] Book 2 - MDC xx ASSIGNMENT OF ICD-10 CODES of each MDC
    // #[serde(rename = "PDC,C,5")]
    // pub(crate) pdc: Option<String>,
    // /// [จริง,เท็จ]
    // #[serde(rename = "CC,L", deserialize_with = "deserialize_thai_bool")]
    // cc: bool,
    /// [ICD10-TM] CC_EXclusion Book: ccex 'the same as'
    #[serde(rename = "MAINCC,C,6")]
    pub main_cc: Option<String>,
    // /// [0,99]
    // #[serde(rename = "CCROW,N,3,0")]
    // cc_row: u8,
    // /// [ICD-10TM]
    // #[serde(rename = "DCLMAIN,C,6")]
    // dlc_main: Option<String>,
    // /// [B-E,""] Book 2 - MDC 25 - AX ASSIGNMENT OF ICD-10 CODES
    // #[serde(rename = "HIV_AX,C,1")]
    // pub(crate) hiv_ax: String,
    /// [0-8,N]=>[Some(1,8)] Book 2 - MDC 24 - Definition of Trauma Diagnoses and Significant Body Site Categories
    #[serde(rename = "TRAUMA,C,1", deserialize_with = "deserialize_0n_none")]
    pub trauma: Option<u8>,
    /// [B,M,F] Book 1 - Appendix A4: Gender Conflict - A4.1 Diagnosis Codes
    #[serde(rename = "SEX,C,1", deserialize_with = "deserialize_gender_ok")]
    pub gender: GenderOk,
    /// [Y,N] Book 1 - Appendix A2: Unacceptable Principal Diagnoses
    #[serde(rename = "ACCPDX,C,1", deserialize_with = "deserialize_yn_bool")]
    pub acc_pdx: bool,
    /// [Y,N] Book 1 - Appendix A3: Age Conflict
    #[serde(rename = "AGEDUSE,C,1", deserialize_with = "deserialize_yn_bool")]
    pub aged_use: bool,
    /// [0,1,8,9,16,31] Book 1 - Appendix A3: Age Conflict
    #[serde(rename = "AGEMIN,N,3,0")]
    pub age_min: u8,
    /// [0,2,10,19,60,124] Book 1 - Appendix A3: Age Conflict
    #[serde(rename = "AGEMAX,N,3,0")]
    pub age_max: u8,
    /// [0,28] Book 1 - Appendix A3: Age Conflict
    #[serde(rename = "AGEDMIN,N,3,0")]
    pub aged_min: u16,
    /// [0,1,""] Book 1 - Appendix D1: Multiple Organ System Diagnoses (MosDx)
    #[serde(rename = "MOSDX,N,1,0", deserialize_with = "deserialize_01nbsp_bool")]
    pub mos_dx: bool,
}

#[derive(Clone, Debug, Deserialize, bitcode::Encode, bitcode::Decode)]
pub struct I10vx {
    /// [ICD10-TM]
    #[serde(rename = "CODE,C,6")]
    pub code: String,
    /// [จริง,เท็จ]
    #[serde(rename = "VALIDCODE,L", deserialize_with = "deserialize_thai_bool")]
    pub is_valid: bool,
    #[serde(rename = "DESC,C,204", deserialize_with = "deserialize_to_american_text")]
    pub desc: String,
    /// [จริง,เท็จ]
    #[serde(rename = "TM,L", deserialize_with = "deserialize_thai_bool")]
    pub is_tm: bool,
}

impl I10vx {
    /// call `search_with_prefix` for ICD code
    ///
    /// return (Arc<I10vx>, rate)
    pub(crate) fn fuzzy_search_best_n(text: &str, list: &BTreeMap<String, Arc<Self>>, n: usize) -> Vec<(Arc<Self>, f32, u8)> {
        let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
        let chars_count = text_lo.chars().count() + 1;
        let trigrams_a = trigrams(&text_lo);

        let mut res = list
            .iter()
            .filter_map(|(_, vx)| {
                if vx.desc.is_empty() {
                    None
                } else {
                    let r = fuzzy_compare(&trigrams_a, chars_count, &vx.desc);
                    (r > 0.0).then(|| (vx, r))
                }
            })
            .collect::<Vec<(&Arc<Self>, f32)>>();
        res.sort_by(|(_, d1), (_, d2)| d2.total_cmp(d1));
        res.into_iter().take(n).map(|(b, x)| (b.to_owned(), x, 2)).collect()
    }

    /// call `search_with_prefix` for ICD code
    ///
    /// return (Arc<I10vx>, rate)
    pub(crate) fn contains_search_best_n(text: &str, list: &BTreeMap<String, Arc<Self>>, n: usize) -> Vec<(Arc<Self>, f32, u8)> {
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
            .filter_map(|(_, item)| {
                let detail_lo = item.desc.to_ascii_lowercase();
                let detail_len = detail_lo.len() as f32;
                let mut res = 0.0;
                for keyword in keywords.iter() {
                    let keyword_len = keyword.len() as f32;
                    if keyword_len > 0.0 && detail_len > 0.0 && res < 1.0 && detail_lo.contains(keyword) {
                        res += (1.0 / keywords_len) + (0.2 * keyword_len / detail_len);
                    }
                }
                (res > 0.0).then(|| (item, res))
            })
            .collect::<Vec<(&Arc<Self>, f32)>>();
        res.sort_by(|(_, d1), (_, d2)| d2.total_cmp(d1));
        res.into_iter().take(n).map(|(k, x)| (k.clone(), x, 2)).collect()
    }

    pub(crate) fn search_with_prefix(prefix: &str, list: &BTreeMap<String, Arc<Self>>) -> Vec<(Arc<Self>, f32, u8)> {
        if prefix.len() > 2 {
            list.range(prefix.to_owned()..next_key(&prefix)).map(|(_, vx)| (vx.to_owned(), 1.0, 1)).collect()
        } else {
            Vec::new()
        }
    }

    pub(crate) fn search_exact(code: &str, list: &BTreeMap<String, Arc<Self>>) -> Option<Arc<Self>> {
        list.get(code).cloned()
    }
}

#[derive(Clone, Debug, Deserialize, bitcode::Encode, bitcode::Decode)]
pub struct I9vx {
    /// [ICD9-CM]
    #[serde(rename = "CODE,C,6")]
    pub code: String,
    /// [จริง,เท็จ]
    #[serde(rename = "VALIDCODE,L", deserialize_with = "deserialize_thai_bool")]
    pub is_valid: bool,
    #[serde(rename = "DESC,C,38", deserialize_with = "deserialize_to_american_text")]
    pub desc: String,
}

impl I9vx {
    /// call `search_with_prefix` for ICD code
    ///
    /// return (Arc<I9vx>, rate)
    pub(crate) fn fuzzy_search_best_n(text: &str, list: &BTreeMap<String, Arc<Self>>, n: usize) -> Vec<(Arc<Self>, f32, u8)> {
        let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
        let chars_count = text_lo.chars().count() + 1;
        let trigrams_a = trigrams(&text_lo);
        let mut res = list
            .iter()
            .filter_map(|(_, value)| {
                if value.desc.is_empty() {
                    None
                } else {
                    let r = fuzzy_compare(&trigrams_a, chars_count, &value.desc);
                    (r > 0.0).then(|| (value, r))
                }
            })
            .collect::<Vec<(&Arc<Self>, f32)>>();
        res.sort_by(|(_, d1), (_, d2)| d2.total_cmp(d1));
        res.into_iter().take(n).map(|(b, x)| (b.to_owned(), x, 2)).collect()
    }

    /// call `search_with_prefix` for ICD code
    ///
    /// return (Arc<I9vx>, rate)
    pub(crate) fn contains_search_best_n(text: &str, list: &BTreeMap<String, Arc<Self>>, n: usize) -> Vec<(Arc<Self>, f32, u8)> {
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
            .filter_map(|(_, item)| {
                let detail_lo = item.desc.to_ascii_lowercase();
                let detail_len = detail_lo.len() as f32;
                let mut res = 0.0;
                for keyword in keywords.iter() {
                    let keyword_len = keyword.len() as f32;
                    if keyword_len > 0.0 && detail_len > 0.0 && res < 1.0 && detail_lo.contains(keyword) {
                        res += (1.0 / keywords_len) + (0.2 * keyword_len / detail_len);
                    }
                }
                (res > 0.0).then(|| (item, res))
            })
            .collect::<Vec<(&Arc<Self>, f32)>>();
        res.sort_by(|(_, d1), (_, d2)| d2.total_cmp(d1));
        res.into_iter().take(n).map(|(k, x)| (k.clone(), x, 2)).collect()
    }

    pub(crate) fn search_with_prefix(prefix: &str, list: &BTreeMap<String, Arc<Self>>) -> Vec<(Arc<Self>, f32, u8)> {
        if prefix.len() > 2 {
            list.range(prefix.to_owned()..next_key(&prefix)).map(|(_, proc)| (proc.to_owned(), 1.0, 1)).collect()
        } else {
            Vec::new()
        }
    }

    pub(crate) fn search_exact(proc: &str, list: &BTreeMap<String, Arc<Self>>) -> Option<Arc<Self>> {
        list.get(proc).cloned()
    }
}

#[derive(Debug, Deserialize, bitcode::Encode, bitcode::Decode)]
pub struct Proc {
    /// [ICD9-CM] Book 1 - Appendix B: Procedure Codes - CODE
    #[serde(rename = "CODE,C,6")]
    pub proc: String,
    // /// [Y,N,P,C,""]
    // #[serde(rename = "ORP,C,1")]
    // orp: String,
    // /// [A,C,N,U,""]
    // #[serde(rename = "ORPTYPE,C,1")]
    // orp_type: Option<String>,
    /// [B,M,F] Book 1 - Appendix A4 Gender Conflict - A4.2 Procedure Codes
    #[serde(rename = "SEX,C,1", deserialize_with = "deserialize_gender_ok")]
    pub(crate) gender: GenderOk,
    /// [A,B,C,D,E,F,G,H,J,"-"] Book 1 - Appendix B: Procedure Codes - SITE
    #[serde(rename = "SITE,C,1", deserialize_with = "deserialize_single_dash_opt")]
    pub(crate) site: Option<String>,
    /// [0-6] : Book 1 - Appendix B: Procedure Codes - ORP
    #[serde(rename = "PROCGR,N,1,0")]
    pub(crate) proc_cgr: u8,
    /// [0,16-90] Book 1 - Appendix B: Procedure Codes - LEV
    #[serde(rename = "PROCLEV,N,2,0")]
    pub(crate) proc_lev: u8,
    // /// [0-3]
    // #[serde(rename = "PCPART,N,1,0")]
    // pc_part: u8,
    // /// [0,2,5]
    // #[serde(rename = "EXTLEV,N,1,0")]
    // ext_lev: u8,
    // /// [>,*,""]
    // #[serde(rename = "EXTTYPE,C,1")]
    // ext_type: Option<String>,
    /// ["01".."17","21","--"] Book 1 - Appendix B: Procedure Codes - MDC (Main)
    #[serde(rename = "MMDC,C,2", deserialize_with = "deserialize_double_dash_opt")]
    pub(crate) m_mdc: Option<String>,
    /// ["XX,XX,..",""] Book 1 - Appendix B: Procedure Codes - MDC (Others)
    #[serde(rename = "OMDCS,C,20")]
    pub(crate) o_mdcs: Option<String>,
    // /// [0,1,""]
    // #[serde(rename = "MOSPROC,N,1,0")]
    // mos_proc: Option<u8>,
    /// [2611-2614,""] Book 1 - Appendix D2: Multiple Organ System Procedures (MosProc) - DC
    #[serde(rename = "MOSDC,C,4")]
    pub(crate) mos_dc: Option<String>,
    /// [1-4,""] Book 1 - Appendix D2: Multiple Organ System Procedures (MosProc) - Hierar
    #[serde(rename = "MOSHIERAR,C,2")]
    pub(crate) mos_hierar: Option<u8>,
    // /// no value
    // #[serde(rename = "JSSITE,C,1")]
    // js_site: Option<String>,
    // #[serde(rename = "DESC,C,26", deserialize_with = "deserialize_to_american_text")]
    // pub desc: String,
    // /// [จริง,เท็จ]
    // #[serde(rename = "DRGUSE,L", deserialize_with = "deserialize_thai_bool")]
    // drg_use: bool,
    // /// no value
    // #[serde(rename = "MAYUN,L")]
    // may_un: Option<String>,
}

// This data come from TDRG Groupper's Book 1 Appendix A1
// aim to try to substitute Dagger and Asterisk pair and try Asterisk as PDx (with MDC)
// we already checked that `Asterisk` and `PDx` always the same
// so we remove `PDx` from colection
#[derive(Debug, Deserialize)]
pub struct DaggerAsterisk {
    #[serde(rename = "Dagger")]
    pub dagger: String,
    #[serde(rename = "Asterisk")]
    pub asterisk: String,
    // #[serde(rename = "PDx")]
    // pub(crate) pdx: String,
    // #[serde(rename = "MDC")]
    // mdc: String,
}

#[derive(Debug, Deserialize)]
pub struct MdcPdc {
    pub mdc: u8,
    pub code: String,
    pub pdc: String,
}

#[derive(Debug, Deserialize)]
pub struct MdcPpdc {
    pub mdc: u8,
    pub proc: String,
    pub pdc: String,
}

#[derive(Debug, Deserialize)]
pub struct MdcAx {
    pub code: String,
    pub ax: String,
}

#[derive(Debug, Deserialize)]
pub struct MdcPax {
    pub proc: String,
    pub pax: String,
}

#[derive(Debug, Deserialize, bitcode::Encode, bitcode::Decode)]
pub struct DcPclDrg {
    pub drg: String,
    pub dc: String,
    pub pcl_min: u8,
    pub pcl_max: u8,
}

#[derive(Debug, Deserialize)]
pub struct Dcl {
    pub code: String,
    pub dc: String,
    pub dcl: u8,
}

#[derive(Debug, Deserialize)]
pub struct DclEq {
    pub code: String,
    pub main: String,
}

#[derive(Debug, Deserialize)]
pub struct CcEx {
    #[serde(rename = "CC10,C,6")]
    pub ex: String,
    #[serde(rename = "NOTFOR10,C,6")]
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, bitcode::Encode, bitcode::Decode)]
pub struct Drg {
    #[serde(rename = "MDC,C,2")]
    pub(crate) mdc: String,
    #[serde(rename = "DC,C,4")]
    pub(crate) dc: String,
    #[serde(rename = "DRG,C,5")]
    pub drg: String,
    #[serde(rename = "RW0D,N,7,4")]
    pub(crate) rw0d: f32,
    #[serde(rename = "RW,N,7,4")]
    pub rw: f32,
    #[serde(rename = "WTLOS,N,6,2")]
    pub wtlos: f32,
    /// Outlier Trim Point: OT
    #[serde(rename = "OT,N,4,0")]
    pub ot: f32,
    /// OF value
    #[serde(rename = "MDF,N,4,2")]
    pub(crate) mdf: f32,
    #[serde(rename = "DRGNAME,C,84", deserialize_with = "deserialize_to_american_text")]
    pub detail: String,
}

impl Drg {
    // Book 1 page 198
    pub fn adj_rw(&self, los: u32) -> f32 {
        let los = los as f32;
        // 1.
        if los < 1.0 || los < (self.wtlos / 3.0) {
            if self.rw0d == 0.0 {
                self.rw
            } else if los < 1.0 {
                self.rw0d
            } else if self.wtlos > 3.0 {
                self.rw0d + (los * (self.rw - self.rw0d) / (self.wtlos / 3.0).ceil())
            } else {
                self.rw
            }
        // 2.
        } else if los > self.ot {
            let (b12, b23) = self.b12_b23();
            if los > (self.ot * 3.0) {
                // 2.3
                self.rw + (self.mdf * self.ot * (b12 + b23))
            } else if los > (self.ot * 2.0) {
                // 2.2
                self.rw + (self.mdf * b12 * self.ot) + (self.mdf * b23 * (los - (self.ot * 2.0)))
            } else {
                // 2.1
                self.rw + (self.mdf * b12 * (los - self.ot))
            }
        // 3.
        } else {
            self.rw
        }
    }

    fn b12_b23(&self) -> (f32, f32) {
        if self.drg.chars().nth(2).map(|c| ['0', '1', '2', '3', '4'].contains(&c)).unwrap_or_default() {
            // P
            if self.rw < 2.0 { (0.0904, 0.0584) } else { (0.1580, 0.1268) }
        // M
        } else if self.rw < 0.7 {
            (0.0770, 0.0480)
        } else {
            (0.1212, 0.0743)
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) enum MdcResult {
    Mdc(Mdc),
    /// (MDC, new pdx)
    MdcNewPdx(Mdc, String),
    Dc(String),
    Drg(String),
}

impl MdcResult {
    pub(crate) fn is_drg(&self) -> bool {
        matches!(self, Self::Drg(_))
    }

    pub(crate) fn process(&self, grouper: &Grouper, input: &GrouperInput) -> Self {
        match self {
            Self::Mdc(mdc) => mdc.process(grouper, input),
            Self::MdcNewPdx(mdc, new_pdx) => {
                let new_input = input.clone_swap_pdx(new_pdx);
                mdc.process(grouper, &new_input)
            }
            Self::Dc(dc) => {
                let drg = grouper
                    .dc_pcl_drg(dc)
                    .map(|vs| match vs.len() {
                        0 => [dc, "9"].concat(),
                        1 => vs[0].drg.to_owned(),
                        _ => {
                            let pcl = dcl_pcl(grouper, dc, &input.pdx, &input.sdxs, &input.gender);
                            vs.iter().find(|v| pcl >= v.pcl_min && pcl <= v.pcl_max).map(|v| v.drg.to_owned()).unwrap_or([dc, "9"].concat())
                        }
                    })
                    .unwrap_or([dc, "9"].concat());
                Self::Drg(drg)
            }
            drg => drg.clone(),
        }
    }
}

#[derive(Clone, Debug, bitcode::Encode, bitcode::Decode)]
pub enum GenderOk {
    Male,
    Female,
    Both,
}

fn deserialize_0n_none<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    if let Some(u) = s.parse::<u8>().ok() { Ok((u > 0).then(|| u)) } else { Ok(None) }
}

fn deserialize_to_american_text<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    Ok(TRANSLATOR.translate(&sanity_space(s)))
}

fn deserialize_single_dash_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    if s == "-" { Ok(None) } else { Ok(Some(s.to_owned())) }
}

fn deserialize_double_dash_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    if s == "--" { Ok(None) } else { Ok(Some(s.to_owned())) }
}

fn deserialize_gender_ok<'de, D>(deserializer: D) -> Result<GenderOk, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "M" => Ok(GenderOk::Male),
        "F" => Ok(GenderOk::Female),
        "B" => Ok(GenderOk::Both),
        _ => Err(de::Error::unknown_variant(s, &["M", "F", "B"])),
    }
}

fn deserialize_yn_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "Y" => Ok(true),
        "N" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["Y", "N"])),
    }
}

fn deserialize_thai_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "จริง" => Ok(true),
        "เท็จ" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["จริง", "เท็จ"])),
    }
}

fn deserialize_01nbsp_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "1" => Ok(true),
        "0" | "" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["1", "0", ""])),
    }
}

// if admit before birth then None
fn calculate_date_yd(birth_day: PrimitiveDateTime, admit_date: PrimitiveDateTime) -> (Option<u8>, Option<u16>) {
    if admit_date >= birth_day {
        let alt_bday = if birth_day.month() == Month::February && birth_day.day() == 29 { 28 } else { birth_day.day() };
        // This unwrap never panic because birth_day.day() always valid so alt_bday is valid
        let full_this_year = PrimitiveDateTime::new(Date::from_calendar_date(admit_date.year(), birth_day.month(), alt_bday).unwrap(), birth_day.time());
        if full_this_year > admit_date {
            // This unwrap never panic because birth_day.day() always valid so alt_bday is valid
            let full_prev_year = PrimitiveDateTime::new(Date::from_calendar_date(admit_date.year() - 1, birth_day.month(), alt_bday).unwrap(), birth_day.time());
            (Some((admit_date.year() - birth_day.year() - 1) as u8), Some((admit_date - full_prev_year).whole_days() as u16))
        } else {
            (Some((admit_date.year() - birth_day.year()) as u8), Some((admit_date - full_this_year).whole_days() as u16))
        }
    } else {
        (None, None)
    }
}

#[cfg(test)]
pub mod tests {

    use time::macros::datetime;

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_calculate_date_yd() {
        // birthday before admit
        let (age_y_1, age_d_1) = calculate_date_yd(datetime!(2021-09-01 00:00), datetime!(2022-09-30 00:00));
        assert_eq!(age_y_1, Some(1));
        assert_eq!(age_d_1, Some(29));
        // birthday after admit
        let (age_y_2, age_d_2) = calculate_date_yd(datetime!(2021-10-01 00:00), datetime!(2022-09-30 00:00));
        assert_eq!(age_y_2, Some(0));
        assert_eq!(age_d_2, Some(364));
        // birthday at admit
        let (age_y_3, age_d_3) = calculate_date_yd(datetime!(2021-09-30 00:00), datetime!(2022-09-30 00:00));
        assert_eq!(age_y_3, Some(1));
        assert_eq!(age_d_3, Some(0));
        // admit at new-year
        let (age_y_4, age_d_4) = calculate_date_yd(datetime!(2021-12-31 00:00), datetime!(2023-01-01 00:00));
        assert_eq!(age_y_4, Some(1));
        assert_eq!(age_d_4, Some(1));
        // admit at end-year
        let (age_y_5, age_d_5) = calculate_date_yd(datetime!(2021-01-01 00:00), datetime!(2022-12-31 00:00));
        assert_eq!(age_y_5, Some(1));
        assert_eq!(age_d_5, Some(364));
        // birth at admit
        let (age_y_6, age_d_6) = calculate_date_yd(datetime!(2021-01-01 00:00), datetime!(2021-01-01 00:00));
        assert_eq!(age_y_6, Some(0));
        assert_eq!(age_d_6, Some(0));
        // birth after admit
        let (age_y_7, age_d_7) = calculate_date_yd(datetime!(2021-01-02 00:00), datetime!(2021-01-01 00:00));
        assert!(age_y_7.is_none());
        assert!(age_d_7.is_none());
    }

    #[test]
    fn test_check_input() {
        let valid = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        assert!(valid.is_ok());

        let empty_pdx = GrouperInput::new(
            "",
            &[String::from("G050"), String::from("A321")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = empty_pdx {
            assert_eq!(errors, vec![GrouperError::EmptyCode]);
        }

        let empty_sdx = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = empty_sdx {
            assert_eq!(errors, vec![GrouperError::EmptyCode]);
        }

        let empty_proc = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &[String::from("4100"), String::from("")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = empty_proc {
            assert_eq!(errors, vec![GrouperError::EmptyCode]);
        }

        let pdx_in_sdx = GrouperInput::new(
            "A321",
            &[String::from("G050"), String::from("A321")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = pdx_in_sdx {
            assert_eq!(errors, vec![GrouperError::PDxInSDxs]);
        }

        let sdx_overload = GrouperInput::new(
            "A321",
            &(0..14).map(|s| s.to_string()).collect::<Vec<String>>(),
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = sdx_overload {
            assert_eq!(errors, vec![GrouperError::SDxOverload]);
        }

        let proc_overload = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &(0..22).map(|s| s.to_string()).collect::<Vec<String>>(),
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = proc_overload {
            assert_eq!(errors, vec![GrouperError::ProcOverload]);
        }

        let valid_gender = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("3")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = valid_gender {
            assert_eq!(errors, vec![GrouperError::InvalidGender]);
        }

        let valid_age_y = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(1897-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-03-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = valid_age_y {
            assert_eq!(errors, vec![GrouperError::InvalidAgeY]);
        }

        let no_adm_wt = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-11 00:00)),
            Some(datetime!(2022-03-11 00:00)),
            "01",
            1,
        );
        if let Err(errors) = no_adm_wt {
            assert_eq!(errors, vec![GrouperError::InvalidAdmWt]);
        }

        let invalid_adm_wt = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            Some(250),
            Some(datetime!(2022-02-11 00:00)),
            Some(datetime!(2022-03-11 00:00)),
            "01",
            1,
        );
        if let Err(errors) = invalid_adm_wt {
            assert_eq!(errors, vec![GrouperError::InvalidAdmWt]);
        }

        let valid_los = GrouperInput::new(
            "A321",
            &[String::from("A428"), String::from("G050")],
            &[String::from("4100"), String::from("4101")],
            &Some(String::from("1")),
            Some(datetime!(2022-01-01 00:00)),
            None,
            Some(datetime!(2022-02-01 00:00)),
            Some(datetime!(2022-02-01 00:00)),
            "01",
            1,
        );
        if let Err(errors) = valid_los {
            assert_eq!(errors, vec![GrouperError::InvalidLos]);
        }
    }

    // Book 1 page 198
    #[test]
    fn test_adj_rw() {
        // example 1
        assert_eq!(GROUPER.drg("06571").map(|drg| drg.adj_rw(0)).unwrap_or_default(), 0.6201);
        assert_eq!(GROUPER.drg("06571").map(|drg| drg.adj_rw(2)).unwrap_or_default(), 0.6814);
        assert_eq!(GROUPER.drg("06571").map(|drg| drg.adj_rw(14)).unwrap_or_default(), 0.9124);
        // example 2
        assert_eq!(GROUPER.drg("06072").map(|drg| drg.adj_rw(2)).unwrap_or_default(), 2.3522668);
        // NOTE: example 2 in the book is wrongly use OF = 1.0 instead of 0.67 (dbf is 0.67, book 1 page 181 is 0.67)
        assert_eq!(GROUPER.drg("06072").map(|drg| drg.adj_rw(45)).unwrap_or_default(), 5.0908327); // 6.3744
        assert_eq!(GROUPER.drg("06072").map(|drg| drg.adj_rw(65)).unwrap_or_default(), 6.110304); // 7.8960
    }
}
