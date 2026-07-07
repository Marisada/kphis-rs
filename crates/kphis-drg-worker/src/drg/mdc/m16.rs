// Book 2 pdf page 290

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M16, &input.procs) {
        process_proc(grouper, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M16, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.procs)
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M16, "16PA", procs) {
        MdcResult::Dc(String::from("1601"))
    } else if grouper.has_mdc_ppdc(Mdc::M16, "16PB", procs) {
        MdcResult::Dc(String::from("1602"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M16, "16A", pdx) {
        if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1653"))
        } else {
            MdcResult::Dc(String::from("1650"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M16, "16B", pdx) {
        if grouper.has_mdc_pax("16PBX", procs) {
            MdcResult::Dc(String::from("1654"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1655"))
        } else {
            MdcResult::Dc(String::from("1651"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M16, "16C", pdx) {
        if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1656"))
        } else {
            MdcResult::Dc(String::from("1652"))
        }
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
        let p16pa = process_proc(&GROUPER, &["4141"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p16pa, MdcResult::Dc(String::from("1601")));
        let p16pb = process_proc(&GROUPER, &["0799"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p16pb, MdcResult::Dc(String::from("1602")));
    }

    #[test]
    fn test_process_pdx() {
        let p16a_bx = process_pdx(&GROUPER, "D460", &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p16a_bx, MdcResult::Dc(String::from("1653")));
        let p16a = process_pdx(&GROUPER, "D460", &HashSet::new());
        assert_eq!(p16a, MdcResult::Dc(String::from("1650")));

        let p16b_bx = process_pdx(&GROUPER, "D65", &["0096"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p16b_bx, MdcResult::Dc(String::from("1654")));
        let p16b_9bx = process_pdx(&GROUPER, "D65", &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p16b_9bx, MdcResult::Dc(String::from("1655")));
        let p16b = process_pdx(&GROUPER, "D65", &HashSet::new());
        assert_eq!(p16b, MdcResult::Dc(String::from("1651")));

        let p16c_bx = process_pdx(&GROUPER, "D472", &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p16c_bx, MdcResult::Dc(String::from("1656")));
        let p16c = process_pdx(&GROUPER, "D472", &HashSet::new());
        assert_eq!(p16c, MdcResult::Dc(String::from("1652")));
    }
}
