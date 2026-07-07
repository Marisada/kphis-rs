// Book 2 pdf page 346

use std::collections::HashSet;

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
};

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs)
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M22, "22A", pdx) {
        if grouper.has_mdc_pax("22PEX", procs) {
            if grouper.has_mdc_pax("22PBX", procs) || grouper.has_mdc_pax("22PCX", procs) || grouper.has_mdc_pax("22PDX", procs) {
                MdcResult::Dc(String::from("2201"))
            } else {
                MdcResult::Dc(String::from("2205"))
            }
        } else {
            MdcResult::Dc(String::from("2250"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M22, "22B", pdx) {
        if grouper.has_mdc_pax("22PEX", procs) {
            if grouper.has_mdc_pax("22PBX", procs) {
                MdcResult::Dc(String::from("2206"))
            } else if grouper.has_mdc_pax("22PCX", procs) || grouper.has_mdc_ax_sdxs("22BX", sdxs) {
                MdcResult::Dc(String::from("2202"))
            } else {
                MdcResult::Dc(String::from("2207"))
            }
        } else {
            MdcResult::Dc(String::from("2251"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M22, "22C", pdx) {
        MdcResult::Dc(String::from("2252"))
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
        let p22a_ex_bx = process_pdx(&GROUPER, "T312", &HashSet::new(), &["8622", "8670"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p22a_ex_bx, MdcResult::Dc(String::from("2201")));
        let p22a_ex_cx = process_pdx(&GROUPER, "T312", &HashSet::new(), &["8622", "8660"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p22a_ex_cx, MdcResult::Dc(String::from("2201")));
        let p22a_ex_dx = process_pdx(&GROUPER, "T312", &HashSet::new(), &["8622", "9915"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p22a_ex_dx, MdcResult::Dc(String::from("2201")));
        let p22a_ex = process_pdx(&GROUPER, "T312", &HashSet::new(), &["8622"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p22a_ex, MdcResult::Dc(String::from("2205")));
        let p22a = process_pdx(&GROUPER, "T312", &HashSet::new(), &HashSet::new());
        assert_eq!(p22a, MdcResult::Dc(String::from("2250")));

        let p22b_ex_bx = process_pdx(&GROUPER, "T203", &HashSet::new(), &["8622", "8670"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p22b_ex_bx, MdcResult::Dc(String::from("2206")));
        let p22b_ex_cx = process_pdx(&GROUPER, "T203", &HashSet::new(), &["8622", "8660"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p22b_ex_cx, MdcResult::Dc(String::from("2202")));
        let p22b_ex_x = process_pdx(
            &GROUPER,
            "T203",
            &["J960"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["8622"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p22b_ex_x, MdcResult::Dc(String::from("2202")));
        let p22b_ex = process_pdx(&GROUPER, "T203", &HashSet::new(), &["8622"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p22b_ex, MdcResult::Dc(String::from("2207")));
        let p22b = process_pdx(&GROUPER, "T203", &HashSet::new(), &HashSet::new());
        assert_eq!(p22b, MdcResult::Dc(String::from("2251")));

        let p22c = process_pdx(&GROUPER, "T200", &HashSet::new(), &HashSet::new());
        assert_eq!(p22c, MdcResult::Dc(String::from("2252")));
    }
}
