// Book 2 pdf page 350

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if let Some(uorp_res) = process_uorp(Mdc::M23, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.age_y)
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M23, "23A", pdx) {
        if grouper.has_mdc_ax_sdxs("23BX", sdxs) {
            MdcResult::Dc(String::from("2355"))
        } else {
            MdcResult::Dc(String::from("2350"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M23, "23B", pdx) {
        MdcResult::Dc(String::from("2351"))
    } else if grouper.is_pdx_pdc(Mdc::M23, "23C", pdx) {
        if grouper.has_mdc_pax("23PBX", procs) {
            MdcResult::Dc(String::from("2303"))
        } else {
            MdcResult::Dc(String::from("2352"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M23, "23D", pdx) {
        if *age_y > 54 { MdcResult::Dc(String::from("2353")) } else { MdcResult::Dc(String::from("2354")) }
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
        let p23a_neuro = process_pdx(&GROUPER, "Z440", &["G800"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &HashSet::new(), &20);
        assert_eq!(p23a_neuro, MdcResult::Dc(String::from("2355")));
        let p23a = process_pdx(&GROUPER, "Z440", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p23a, MdcResult::Dc(String::from("2350")));
        let p23b = process_pdx(&GROUPER, "G933", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p23b, MdcResult::Dc(String::from("2351")));
        let p23c_endo = process_pdx(&GROUPER, "Z080", &HashSet::new(), &["2121"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p23c_endo, MdcResult::Dc(String::from("2303")));
        let p23c = process_pdx(&GROUPER, "Z080", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p23c, MdcResult::Dc(String::from("2352")));
        let p23d_o54 = process_pdx(&GROUPER, "R99", &HashSet::new(), &HashSet::new(), &60);
        assert_eq!(p23d_o54, MdcResult::Dc(String::from("2353")));
        let p23d = process_pdx(&GROUPER, "R99", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p23d, MdcResult::Dc(String::from("2354")));
    }
}
