// Book 2 pdf page 334

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M21, &input.procs) {
        process_proc(grouper, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M21, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.age_y)
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M21, "21PF", procs) {
        MdcResult::Dc(String::from("2107"))
    } else if grouper.has_mdc_ppdc(Mdc::M21, "21PB", procs) {
        MdcResult::Dc(String::from("2102"))
    } else if grouper.has_mdc_ppdc(Mdc::M21, "21PD", procs) || grouper.has_mdc_ppdc(Mdc::M21, "21PE", procs) {
        MdcResult::Dc(String::from("2104"))
    } else if grouper.has_mdc_ppdc(Mdc::M21, "21PC", procs) {
        MdcResult::Dc(String::from("2103"))
    } else if grouper.has_mdc_ppdc(Mdc::M21, "21PA", procs) {
        MdcResult::Dc(String::from("2101"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M21, "21A", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("2150")) } else { MdcResult::Dc(String::from("2151")) }
    } else if grouper.is_pdx_pdc(Mdc::M21, "21B", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("2152")) } else { MdcResult::Dc(String::from("2153")) }
    } else if grouper.is_pdx_pdc(Mdc::M21, "21C", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("2154")) } else { MdcResult::Dc(String::from("2155")) }
    } else if grouper.is_pdx_pdc(Mdc::M21, "21D", pdx) {
        MdcResult::Dc(String::from("2156"))
    } else if grouper.is_pdx_pdc(Mdc::M21, "21E", pdx) {
        MdcResult::Dc(String::from("2157"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_proc() {
        let p21pf = process_proc(&GROUPER, &["8622*1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p21pf, MdcResult::Dc(String::from("2107")));
        let p21pb = process_proc(&GROUPER, &["8584"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p21pb, MdcResult::Dc(String::from("2102")));
        let p21pd = process_proc(&GROUPER, &["0131"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p21pd, MdcResult::Dc(String::from("2104")));
        let p21pe = process_proc(&GROUPER, &["C7080"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p21pe, MdcResult::Dc(String::from("2104")));
        let p21pc = process_proc(&GROUPER, &["7764"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p21pc, MdcResult::Dc(String::from("2103")));
        let p21pa = process_proc(&GROUPER, &["8622"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p21pa, MdcResult::Dc(String::from("2101")));
    }

    #[test]
    fn test_process_pdx() {
        let p21a_o17 = process_pdx(&GROUPER, "S010", &20);
        assert_eq!(p21a_o17, MdcResult::Dc(String::from("2150")));
        let p21a = process_pdx(&GROUPER, "S010", &10);
        assert_eq!(p21a, MdcResult::Dc(String::from("2151")));
        let p21b_o17 = process_pdx(&GROUPER, "T780", &20);
        assert_eq!(p21b_o17, MdcResult::Dc(String::from("2152")));
        let p21b = process_pdx(&GROUPER, "T780", &10);
        assert_eq!(p21b, MdcResult::Dc(String::from("2153")));
        let p21c_o17 = process_pdx(&GROUPER, "T789", &20);
        assert_eq!(p21c_o17, MdcResult::Dc(String::from("2154")));
        let p21c = process_pdx(&GROUPER, "T789", &10);
        assert_eq!(p21c, MdcResult::Dc(String::from("2155")));
        let p21d = process_pdx(&GROUPER, "T740", &20);
        assert_eq!(p21d, MdcResult::Dc(String::from("2156")));
        let p21e = process_pdx(&GROUPER, "T330", &20);
        assert_eq!(p21e, MdcResult::Dc(String::from("2157")));
    }
}
