// Book 2 pdf page 314

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    process_pdx(grouper, &input.pdx, &input.procs)
}

fn process_pdx(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M19, "19A", pdx) {
        if grouper.has_mdc_pax("19PBX", procs) {
            MdcResult::Dc(String::from("1901"))
        } else {
            MdcResult::Dc(String::from("1950"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M19, "19B", pdx) {
        if grouper.has_mdc_pax("19PBX", procs) {
            MdcResult::Dc(String::from("1902"))
        } else {
            MdcResult::Dc(String::from("1951"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M19, "19C", pdx) {
        if grouper.has_mdc_pax("19PBX", procs) {
            MdcResult::Dc(String::from("1903"))
        } else {
            MdcResult::Dc(String::from("1952"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M19, "19D", pdx) {
        MdcResult::Dc(String::from("1953"))
    } else if grouper.is_pdx_pdc(Mdc::M19, "19E", pdx) {
        MdcResult::Dc(String::from("1954"))
    } else if grouper.is_pdx_pdc(Mdc::M19, "19F", pdx) {
        MdcResult::Dc(String::from("1955"))
    } else if grouper.is_pdx_pdc(Mdc::M19, "19G", pdx) {
        MdcResult::Dc(String::from("1956"))
    } else if grouper.is_pdx_pdc(Mdc::M19, "19H", pdx) {
        MdcResult::Dc(String::from("1957"))
    } else if grouper.is_pdx_pdc(Mdc::M19, "19J", pdx) {
        MdcResult::Dc(String::from("1958"))
    } else if grouper.is_pdx_pdc(Mdc::M19, "19K", pdx) {
        if grouper.has_mdc_pax("19PBX", procs) {
            MdcResult::Dc(String::from("1904"))
        } else {
            MdcResult::Dc(String::from("1959"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M19, "19L", pdx) {
        MdcResult::Dc(String::from("1960"))
    } else if grouper.is_pdx_pdc(Mdc::M19, "19N", pdx) {
        MdcResult::Dc(String::from("1962"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_pdx() {
        let p19a_ect = process_pdx(&GROUPER, "F230", &["9426"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p19a_ect, MdcResult::Dc(String::from("1901")));
        let p19a = process_pdx(&GROUPER, "F230", &HashSet::new());
        assert_eq!(p19a, MdcResult::Dc(String::from("1950")));
        let p19b_ect = process_pdx(&GROUPER, "F200", &["9426"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p19b_ect, MdcResult::Dc(String::from("1902")));
        let p19b = process_pdx(&GROUPER, "F200", &HashSet::new());
        assert_eq!(p19b, MdcResult::Dc(String::from("1951")));
        let p19c_ect = process_pdx(&GROUPER, "F300", &["9426"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p19c_ect, MdcResult::Dc(String::from("1903")));
        let p19c = process_pdx(&GROUPER, "F300", &HashSet::new());
        assert_eq!(p19c, MdcResult::Dc(String::from("1952")));
        let p19d = process_pdx(&GROUPER, "F340", &HashSet::new());
        assert_eq!(p19d, MdcResult::Dc(String::from("1953")));
        let p19e = process_pdx(&GROUPER, "F430", &HashSet::new());
        assert_eq!(p19e, MdcResult::Dc(String::from("1954")));
        let p19f = process_pdx(&GROUPER, "F400", &HashSet::new());
        assert_eq!(p19f, MdcResult::Dc(String::from("1955")));
        let p19g = process_pdx(&GROUPER, "F420", &HashSet::new());
        assert_eq!(p19g, MdcResult::Dc(String::from("1956")));
        let p19h = process_pdx(&GROUPER, "F520", &HashSet::new());
        assert_eq!(p19h, MdcResult::Dc(String::from("1957")));
        let p19j = process_pdx(&GROUPER, "F800", &HashSet::new());
        assert_eq!(p19j, MdcResult::Dc(String::from("1958")));
        let p19k_ect = process_pdx(&GROUPER, "R410", &["9426"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p19k_ect, MdcResult::Dc(String::from("1904")));
        let p19k = process_pdx(&GROUPER, "R410", &HashSet::new());
        assert_eq!(p19k, MdcResult::Dc(String::from("1959")));
        let p19l = process_pdx(&GROUPER, "R440", &HashSet::new());
        assert_eq!(p19l, MdcResult::Dc(String::from("1960")));
        let p19n = process_pdx(&GROUPER, "F700", &HashSet::new());
        assert_eq!(p19n, MdcResult::Dc(String::from("1962")));
    }
}
