#![allow(dead_code)]

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::{Arc, LazyLock},
};

use super::{
    mdc::Mdc,
    model::{DcPclDrg, Drg, GenderOk, GrouperError, GrouperInput, GrouperOrigin, GrouperOutput, I9vx, I10, I10e, I10vx, MdcResult, Proc},
};

pub static GROUPER: LazyLock<Grouper> = LazyLock::new(|| Grouper::new());

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Grouper {
    /// (code, I10), without external causes
    pub i10: HashMap<String, I10e>,
    /// (code, I10vx)
    pub i10vx: BTreeMap<String, Arc<I10vx>>,
    /// (code, I10vx)
    pub i10vx_ex: BTreeMap<String, Arc<I10vx>>,
    /// (code, Proc)
    pub i9: BTreeMap<String, Arc<Proc>>,
    /// (code, I9vx)
    pub i9vx: BTreeMap<String, Arc<I9vx>>,
    /// (drg, Drg)
    pub drg: HashMap<String, Drg>,
    /// (dagger, asterisks)
    pub dagger_asterisks: HashMap<String, HashSet<String>>,
    /// (mdc, (code, pdc))
    pub mdc_pdc: HashMap<u8, HashMap<String, String>>,
    /// (mdc, (proc, pdc))
    pub mdc_ppdc: HashMap<u8, HashMap<String, String>>,
    /// (ax, codes)
    pub mdc_ax: HashMap<String, HashSet<String>>,
    /// (pax, procs)
    pub mdc_pax: HashMap<String, HashSet<String>>,
    /// (dc, (dc, drg, pcl_min, pcl_max))
    pub dc_pcl_drg: HashMap<String, Vec<DcPclDrg>>,
    /// (code, (dc, dcl))
    pub dcl: HashMap<String, Arc<HashMap<String, u8>>>,
    /// (ex, codes)
    pub ccex: HashMap<String, HashSet<String>>,
}

impl Grouper {
    fn new() -> Self {
        let bytes = include_bytes!("../../dump/grouper.dump");
        bitcode::decode(bytes).expect("Cannot decode GROUPER binary")
    }

    pub(crate) fn run(&self, input_json: &str) -> String {
        let mut result = GrouperOutput::new();
        match serde_json::from_str::<GrouperInput>(input_json) {
            Ok(input) => {
                // #1 check input
                let (check_errors, pdx_sdx_pairs) = self.check(&input);
                if !check_errors.is_empty() {
                    result.errors.extend(check_errors);
                }
                if result.errors.is_empty() {
                    self.process_mdc(&input, &mut result);
                    if !pdx_sdx_pairs.is_empty() {
                        for (pdx, sdx) in pdx_sdx_pairs {
                            let alt_input = input.clone_with(pdx, sdx);
                            if !matches!(alt_input.origin, GrouperOrigin::Original) {
                                self.process_mdc(&alt_input, &mut result);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                result.errors.push(GrouperError::ErrorSerdeJson(e.to_string()));
            }
        }

        serde_json::to_string(&result).unwrap_or_default()
    }

    pub fn valid_codes(&self) -> Vec<String> {
        self.i10vx.iter().filter_map(|(c, vx)| vx.is_valid.then(|| c.clone())).collect()
    }

    /// Check Appendex A1-4, find all swapable Dagger-Asterisk pairs
    fn check(&self, input: &GrouperInput) -> (Vec<GrouperError>, Vec<(String, String)>) {
        let mut errors = Vec::new();
        let mut dagger_asterisk_pairs = Vec::new();

        // #1 check PDx (A1-4.1)
        errors.extend(self.check_i10_a234(&input.pdx, true, &input.gender, input.age_y, input.age_d));
        let dagger_asterisks_opt = self.check_a1_dagger(&input.pdx, &input.sdxs);
        if let Some((dagger, asterisks)) = dagger_asterisks_opt {
            dagger_asterisk_pairs.extend(asterisks.into_iter().map(|sdx| (dagger.clone(), sdx)));
        }
        // #2 check SDx (A1-4.1), find Dagger-Asterisk swap suggestion (SDx, possible PDx in SDx)
        if !input.sdxs.is_empty() {
            for sdx in &input.sdxs {
                errors.extend(self.check_i10_a234(&sdx, false, &input.gender, input.age_y, input.age_d));
            }
        }
        // #3 check Proc (A4.2)
        if !input.procs.is_empty() {
            errors.extend(self.check_proc_a4(&input.procs, &input.gender));
        }

        (errors, dagger_asterisk_pairs)
    }

    fn process_mdc(&self, input: &GrouperInput, result: &mut GrouperOutput) {
        let mut mdc_result = MdcResult::Mdc(Mdc::M00);
        let mut iter_check = 0;
        while !mdc_result.is_drg() && iter_check < 20 {
            iter_check = iter_check + 1;
            // process
            let stage = mdc_result.process(&self, input);
            // log
            result.mdc_log.push(stage.clone());
            mdc_result = stage;
        }
        match mdc_result {
            MdcResult::Drg(drg_code) => {
                if let Some(drg) = self.drg.get(&drg_code).cloned() {
                    result.drg.push(input.to_drg_output(drg));
                }
            }
            MdcResult::Mdc(_) | MdcResult::MdcNewPdx(_, _) | MdcResult::Dc(_) => {}
        }
    }

    /// Check SDx has Asterisk of Dagger(PDx)<br>
    /// Return (pdx, matched SDxs)
    fn check_a1_dagger(&self, pdx: &str, sdxs: &HashSet<String>) -> Option<(String, Vec<String>)> {
        self.dagger_asterisks.get(pdx).and_then(|asterisks| {
            let asterisks_in_sdx = asterisks.intersection(&sdxs).cloned().collect::<Vec<String>>();
            if asterisks_in_sdx.is_empty() { None } else { Some((pdx.to_owned(), asterisks_in_sdx)) }
        })
    }

    // Appendix A2: Unacceptable principal diagnoses
    // Appendix A3: Age conflict
    // Appendix A4.1: Gender conflict
    fn check_i10_a234(&self, icd10: &str, is_pdx: bool, gender: &Option<String>, age_y: u8, age_d: Option<u16>) -> Vec<GrouperError> {
        let mut results = Vec::with_capacity(4);
        if let Some(i10v) = self.i10.get(icd10) {
            if let Some(i10) = i10v.get_inner_by_gender(gender) {
                if is_pdx && !i10.acc_pdx {
                    results.push(GrouperError::I10UnAccPDx(icd10.to_owned()));
                }
                match i10.gender {
                    GenderOk::Both => {}
                    GenderOk::Male => {
                        if let Some(s) = gender.as_ref() {
                            if s != "1" {
                                results.push(GrouperError::I10WrongGender(icd10.to_owned(), true));
                            }
                        } else {
                            results.push(GrouperError::I10NoGender(icd10.to_owned()));
                        }
                    }
                    GenderOk::Female => {
                        if let Some(s) = gender.as_ref() {
                            if s != "2" {
                                results.push(GrouperError::I10WrongGender(icd10.to_owned(), false));
                            }
                        } else {
                            results.push(GrouperError::I10NoGender(icd10.to_owned()));
                        }
                    }
                }
                if age_y < i10.age_min || age_y > i10.age_max {
                    results.push(GrouperError::I10ConflictAgeY(icd10.to_owned()));
                }
                if i10.aged_use {
                    if age_y == 0 {
                        if let Some(d) = age_d
                            && d < i10.aged_min
                        {
                            results.push(GrouperError::I10ConflictAgeD(icd10.to_owned(), i10.aged_min));
                        } else {
                            results.push(GrouperError::I10NoAgeD(icd10.to_owned()));
                        }
                    }
                }
            } else {
                results.push(GrouperError::I10NoGender(icd10.to_owned()));
            }
        } else {
            results.push(GrouperError::I10NotFound(icd10.to_owned()));
        }

        results
    }

    // Appendix A4.2: Gender conflict - Procedure Codes
    fn check_proc_a4(&self, procs: &HashSet<String>, gender: &Option<String>) -> Vec<GrouperError> {
        let mut results = Vec::with_capacity(procs.len());
        for proc in procs {
            if let Some(p) = self.i9.get(proc) {
                match p.gender {
                    GenderOk::Both => {}
                    GenderOk::Male => {
                        if let Some(s) = gender.as_ref() {
                            if s != "1" {
                                results.push(GrouperError::ProcWrongGender(proc.to_owned(), true));
                            }
                        } else {
                            results.push(GrouperError::ProcNoGender(proc.to_owned()));
                        }
                    }
                    GenderOk::Female => {
                        if let Some(s) = gender.as_ref() {
                            if s != "2" {
                                results.push(GrouperError::ProcWrongGender(proc.to_owned(), false));
                            }
                        } else {
                            results.push(GrouperError::ProcNoGender(proc.to_owned()));
                        }
                    }
                }
            }
        }

        results
    }

    pub(crate) fn i10(&self, dx: &str, gender: &Option<String>) -> Option<&Arc<I10>> {
        self.i10.get(dx).and_then(|i| i.get_inner_by_gender(gender))
    }

    pub(crate) fn is_pdx_pdc(&self, mdc: Mdc, pdc: &str, pdx: &str) -> bool {
        self.mdc_pdc.get(&mdc.to_digit()).and_then(|pdc_map| pdc_map.get(pdx).map(|rpdc| rpdc == pdc)).unwrap_or_default()
    }

    pub(crate) fn is_pdx_or_sdxs_pdc(&self, mdc: Mdc, pdc: &str, pdx: &str, sdxs: &HashSet<String>) -> bool {
        self.mdc_pdc
            .get(&mdc.to_digit())
            .map(|pdc_map| pdc_map.get(pdx).map(|rpdc| rpdc == pdc).unwrap_or_default() || sdxs.iter().any(|sdx| pdc_map.get(sdx).map(|rpdc| rpdc == pdc).unwrap_or_default()))
            .unwrap_or_default()
    }

    /// pdx is found + mos_dx is true
    pub(crate) fn is_pdx_mosdx(&self, pdx: &str, gender: &Option<String>) -> bool {
        self.i10(pdx, gender).map(|i10| i10.mos_dx).unwrap_or_default()
    }

    pub(crate) fn search_i10vx_desc(&self, is_ex: bool, text: &str) -> Vec<(Arc<I10vx>, f32, u8)> {
        let source = if is_ex { &self.i10vx_ex } else { &self.i10vx };
        // I10vx::fuzzy_search_best_n(&text, &source, 20)
        I10vx::contains_search_best_n(&text, &source, 20)
    }

    pub(crate) fn search_i10vx_code_prefix(&self, is_ex: bool, prefix: &str) -> Vec<(Arc<I10vx>, f32, u8)> {
        let source = if is_ex { &self.i10vx_ex } else { &self.i10vx };
        I10vx::search_with_prefix(prefix, &source)
    }

    pub(crate) fn search_proc_desc(&self, text: &str) -> Vec<(Arc<I9vx>, f32, u8)> {
        // I9vx::fuzzy_search_best_n(&text, &self.procs, 20)
        I9vx::contains_search_best_n(&text, &self.i9vx, 20)
    }

    pub(crate) fn search_proc_code_prefix(&self, prefix: &str) -> Vec<(Arc<I9vx>, f32, u8)> {
        I9vx::search_with_prefix(&prefix, &self.i9vx)
    }

    pub(crate) fn search_proc_code_exact(&self, proc: &str) -> Option<Arc<I9vx>> {
        I9vx::search_exact(&proc, &self.i9vx)
    }

    pub(crate) fn proc(&self, proc: &str) -> Option<&Arc<Proc>> {
        self.i9.get(proc)
    }

    pub(crate) fn proc_sites(&self, procs: &HashSet<String>) -> HashSet<&String> {
        procs.iter().fold(HashSet::new(), |mut acc, proc| {
            if let Some(site) = self.proc(proc).and_then(|i| i.site.as_ref()) {
                acc.insert(site);
            }
            acc
        })
    }

    pub(crate) fn has_any_mdc_ppdc(&self, mdc: Mdc, input_procs: &HashSet<String>) -> bool {
        self.mdc_ppdc(&mdc).map(|pdc_map| input_procs.iter().any(|proc| pdc_map.contains_key(proc))).unwrap_or_default()
    }

    pub(crate) fn has_mdc_ppdc(&self, mdc: Mdc, pdc: &str, input_procs: &HashSet<String>) -> bool {
        self.mdc_ppdc(&mdc)
            .map(|pdc_map| input_procs.iter().any(|proc| pdc_map.get(proc).map(|rpdc| rpdc == pdc).unwrap_or_default()))
            .unwrap_or_default()
    }

    pub(crate) fn has_mdc_ax_pdx(&self, ax: &str, pdx: &str) -> bool {
        self.mdc_ax.get(ax).map(|codes| codes.contains(pdx)).unwrap_or_default()
    }

    pub(crate) fn has_mdc_ax_sdxs(&self, ax: &str, sdxs: &HashSet<String>) -> bool {
        self.mdc_ax.get(ax).map(|codes| codes.intersection(sdxs).count() > 0).unwrap_or_default()
    }

    pub(crate) fn has_mdc_ax_pdx_or_sdxs(&self, ax: &str, pdx: &str, sdxs: &HashSet<String>) -> bool {
        self.mdc_ax.get(ax).map(|codes| codes.contains(pdx) || codes.intersection(sdxs).count() > 0).unwrap_or_default()
    }

    pub(crate) fn has_mdc_pax(&self, pax: &str, input_procs: &HashSet<String>) -> bool {
        self.mdc_pax.get(pax).map(|procs| procs.intersection(input_procs).count() > 0).unwrap_or_default()
    }

    fn mdc_ppdc(&self, mdc: &Mdc) -> Option<&HashMap<String, String>> {
        self.mdc_ppdc.get(&mdc.to_digit())
    }

    /// find procs at has MAX ORP_group
    pub(crate) fn proc_with_max_orp_group(&self, procs: &HashSet<String>) -> Vec<&Arc<Proc>> {
        let mut ps = procs.iter().filter_map(|proc| self.i9.get(proc).and_then(|p| (p.proc_cgr > 0).then(|| p))).collect::<Vec<&Arc<Proc>>>();
        ps.sort_by(|p1, p2| {
            if p2.proc_cgr == p1.proc_cgr {
                if p2.proc_lev == p1.proc_lev { p2.proc.cmp(&p1.proc) } else { p2.proc_lev.cmp(&p1.proc_lev) }
            } else {
                p2.proc_cgr.cmp(&p1.proc_cgr)
            }
        });
        if ps.len() > 1 {
            let max = ps[0].proc_cgr;
            ps.into_iter().filter(|p| p.proc_cgr == max).collect()
        } else {
            ps
        }
    }

    /// code + dc => dcl (None will be 0)
    pub(crate) fn dcl(&self, code: &str, dc: &str) -> u8 {
        self.dcl.get(code).and_then(|v| v.get(dc).cloned()).unwrap_or_default()
    }

    pub(crate) fn dc_pcl_drg(&self, dc: &str) -> Option<&Vec<DcPclDrg>> {
        self.dc_pcl_drg.get(dc)
    }

    /// Find any exists of ref_code(right side code in ccex table) paired with ex_code(left side in ccex table)<br>
    /// DCL of ex_code will be set to 0
    pub(crate) fn has_ccex(&self, ex_code: &str, ref_code: &str) -> bool {
        self.ccex.get(ex_code).map(|v| v.contains(ref_code)).unwrap_or_default()
    }

    #[cfg(test)]
    pub(crate) fn drg(&self, drg: &str) -> Option<&Drg> {
        self.drg.get(drg)
    }
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use super::*;

    #[test]
    fn test_grouper_check_i10_a234() {
        // O904,14,14C,จริง,O904,99,O903,,N,F,Y,N,9,60,0,0
        let correct = GROUPER.check_i10_a234("O94", false, &Some(String::from("2")), 30, None);
        assert_eq!(correct.len(), 0);
        let all_wrong = GROUPER.check_i10_a234("O94", true, &Some(String::from("1")), 66, None);
        assert_eq!(all_wrong, vec![
            GrouperError::I10UnAccPDx(String::from("O94")),
            GrouperError::I10WrongGender(String::from("O94"), false),
            GrouperError::I10ConflictAgeY(String::from("O94")),
        ]);
        let pdx_2mdc_need_gender = GROUPER.check_i10_a234("A510", true, &None, 30, None);
        assert_eq!(pdx_2mdc_need_gender, vec![
            GrouperError::I10NoGender(String::from("A510")),
        ]);
        let invalid_age_max = GROUPER.check_i10_a234("O94", false, &Some(String::from("2")), 125, None);
        assert_eq!(invalid_age_max, vec![
            GrouperError::I10ConflictAgeY(String::from("O94")),
        ]);
        let need_aged = GROUPER.check_i10_a234("F88", false, &Some(String::from("2")), 0, None);
        assert_eq!(need_aged, vec![
            GrouperError::I10NoAgeD(String::from("F88")),
        ]);
        let invalid_aged_min = GROUPER.check_i10_a234("F88", false, &Some(String::from("2")), 0, Some(15));
        assert_eq!(invalid_aged_min, vec![
            GrouperError::I10ConflictAgeD(String::from("F88"), 28),
        ]);
    }

    #[test]
    fn test_grouper_check_prc_a4() {
        // 740 F Classical c-section
        let correct = GROUPER.check_proc_a4(&[String::from("740")].into_iter().collect(), &Some(String::from("2")));
        assert_eq!(correct.len(), 0);
        let need_gender = GROUPER.check_proc_a4(&[String::from("740")].into_iter().collect(), &None);
        assert_eq!(need_gender, vec![
            GrouperError::ProcNoGender(String::from("740")),
        ]);
        let wrong_gender = GROUPER.check_proc_a4(&[String::from("740")].into_iter().collect(), &Some(String::from("1")));
        assert_eq!(wrong_gender, vec![
            GrouperError::ProcWrongGender(String::from("740"), false),
        ]);
    }

    #[test]
    fn test_check_a1_dagger() {
        // A321,G01,G01,01
        // A321,G050,G050,01
        let has_pair = GROUPER.check_a1_dagger("A321", &HashSet::from_iter([String::from("G050")]));
        assert_eq!(has_pair, Some((String::from("A321"), vec![String::from("G050")])));
        let no_pair = GROUPER.check_a1_dagger("A321", &HashSet::from_iter([String::from("G099")]));
        assert!(no_pair.is_none());
    }

    // #[test]
    // fn test_check_a1_asterisk() {
    //     // A321,G01,G01,01
    //     // A321,G050,G050,01
    //     let (one_sdx_has_pdx_as_dagger, no_suggest) = GROUPER.check_a1_asterisk("G050", "A321", &HashSet::from_iter([String::from("G050")]));
    //     assert!(one_sdx_has_pdx_as_dagger.is_none());
    //     assert!(no_suggest.is_none());
    //     let (many_sdx_has_sdx_as_dagger, has_suggest) = GROUPER.check_a1_asterisk("G050", "A099", &HashSet::from_iter([String::from("A321"), String::from("G050")]));
    //     assert!(many_sdx_has_sdx_as_dagger.is_none());
    //     assert_eq!(has_suggest, Some((String::from("G050"), vec![String::from("A321")])));
    //     let (no_sdx_as_dagger, no_suggest_2) = GROUPER.check_a1_asterisk("G050", "A099", &HashSet::from_iter([String::from("G050")]));
    //     assert!(no_sdx_as_dagger.is_some());
    //     if let Some(GrouperError::I10SDxInfoDaggers((sdx, daggers))) = no_sdx_as_dagger {
    //         assert_eq!(sdx, String::from("G050"));
    //         assert_eq!(daggers.into_iter().collect::<HashSet<String>>(), HashSet::from_iter([String::from("A321"), String::from("A368"), String::from("A398"), String::from("A428")]));
    //     }
    //     assert!(no_suggest_2.is_none());
    // }

    // 0052 3 35 C 05
    // 0053 3 39 C 05
    // 0054 3 38 C 05
    // 0055 0 0 - /05
    // 0056 0 0 - 05#PCom
    #[test]
    fn test_proc_with_max_orp_group() {
        let result = GROUPER.proc_with_max_orp_group(&["0056","0052","0053","0054","0055"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(result.into_iter().map(|r| r.proc.to_owned()).collect::<Vec<String>>(), vec![String::from("0053"),String::from("0054"),String::from("0052")]);
        // Book 1 page 128 example 1
        let book1_ex1 = GROUPER.proc_with_max_orp_group(&["7935","9346","9904","4513","1341","1371","4441"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(book1_ex1.into_iter().map(|r| r.proc.to_owned()).collect::<Vec<String>>(), vec![String::from("7935"),String::from("4441")]);
    }

    #[test]
    fn test_has_any_mdc_ppdc() {
        // Book 1 page 128 example 1
        assert!(!GROUPER.has_any_mdc_ppdc(Mdc::M05, &["7935","9346","9904","4513","1341","1371","4441"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>()));
    }

    // Book 1 page 128 example 1
    #[test]
    fn test_run_ex1() {
        let mut input = GrouperInput::default();
        input.set_age_y(75);
        input.set_los(35);
        input.set_pdx("I500");
        input.set_sdxs(&["I10","H259","S7200","E876","K250"]);
        input.set_procs(&["7935","9346","9904","4513","1341","1371","4441"]);
        let output: GrouperOutput = serde_json::from_str(&GROUPER.run(&serde_json::to_string(&input).unwrap())).unwrap();
        assert_eq!(output.drg.first().map(|drg| drg.drg.drg.clone()), Some(String::from("08103")));
        assert_eq!(output.mdc_log, vec![MdcResult::Mdc(Mdc::M05), MdcResult::Dc(String::from("0810")), MdcResult::Drg(String::from("08103"))]);
    }

    // Book 1 page 129 example 2
    #[test]
    fn test_run_ex2() {
        let mut input = GrouperInput::default();
        input.set_gender(Some(String::from("2")));
        input.set_age_y(19);
        input.set_los(3);
        input.set_pdx("O996");
        input.set_sdxs(&["K358"]);
        input.set_procs(&["4709"]);
        let output: GrouperOutput = serde_json::from_str(&GROUPER.run(&serde_json::to_string(&input).unwrap())).unwrap();
        assert_eq!(output.drg.first().map(|drg| drg.drg.drg.clone()), Some(String::from("06070")));
        assert_eq!(output.mdc_log, vec![MdcResult::Mdc(Mdc::M14), MdcResult::Dc(String::from("0607")), MdcResult::Drg(String::from("06070"))]);
    }

    // for finding bug
    #[test]
    fn test_run_bug_dcl_overflow() {
        let mut input = GrouperInput::default();
        input.set_gender(Some(String::from("2")));
        input.set_age_y(19);
        input.set_los(3);
        input.set_pdx("T634");
        // input.set_sdxs(&["K358"]);
        // input.set_procs(&["4709"]);
        let output: GrouperOutput = serde_json::from_str(&GROUPER.run(&serde_json::to_string(&input).unwrap())).unwrap();
        assert_eq!(output.drg.first().map(|drg| drg.drg.drg.clone()), Some(String::from("21540")));
        assert_eq!(output.mdc_log, vec![MdcResult::Mdc(Mdc::M21), MdcResult::Dc(String::from("2154")), MdcResult::Drg(String::from("21540"))]);
    }
}
