// Book 2 pdf page 380

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if let Some(uorp_res) = process_uorp(Mdc::M25, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_dx(grouper, &input.pdx, &input.sdxs, &input.dch_type)
    }
}

fn process_dx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, dch_type: &str) -> MdcResult {
    if grouper.has_mdc_ax_pdx_or_sdxs("25BX", pdx, sdxs) {
        MdcResult::Dc(String::from("2550"))
    } else if grouper.has_mdc_ax_pdx_or_sdxs("25CX", pdx, sdxs) {
        MdcResult::Dc(String::from("2551"))
    } else if grouper.has_mdc_ax_pdx_or_sdxs("25DX", pdx, sdxs) {
        if dch_type == "04" {
            MdcResult::Dc(String::from("2554"))
        } else {
            MdcResult::Dc(String::from("2552"))
        }
    } else {
        MdcResult::Dc(String::from("2553"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_dx() {
        let p25bx_p = process_dx(&GROUPER, "A812", &HashSet::new(), "01");
        assert_eq!(p25bx_p, MdcResult::Dc(String::from("2550")));
        let p25bx_s = process_dx(&GROUPER, "", &["A812"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p25bx_s, MdcResult::Dc(String::from("2550")));

        let p25cx_p = process_dx(&GROUPER, "B210", &HashSet::new(), "01");
        assert_eq!(p25cx_p, MdcResult::Dc(String::from("2551")));
        let p25cx_s = process_dx(&GROUPER, "", &["B210"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p25cx_s, MdcResult::Dc(String::from("2551")));

        let p25dx_p_refer = process_dx(&GROUPER, "A020", &HashSet::new(), "04");
        assert_eq!(p25dx_p_refer, MdcResult::Dc(String::from("2554")));
        let p25dx_s_refer = process_dx(&GROUPER, "", &["A020"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "04");
        assert_eq!(p25dx_s_refer, MdcResult::Dc(String::from("2554")));

        let p25dx_p = process_dx(&GROUPER, "A020", &HashSet::new(), "01");
        assert_eq!(p25dx_p, MdcResult::Dc(String::from("2552")));
        let p25dx_s = process_dx(&GROUPER, "", &["A020"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p25dx_s, MdcResult::Dc(String::from("2552")));

        let p25 = process_dx(&GROUPER, "O987", &HashSet::new(), "01");
        assert_eq!(p25, MdcResult::Dc(String::from("2553")));
    }
}
