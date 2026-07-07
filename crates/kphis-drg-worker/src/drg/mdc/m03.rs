// Book 2 pdf page 54

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M03, &input.procs) {
        process_proc(grouper, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M03, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs)
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M03, "3PA", procs) {
        if grouper.has_mdc_pax("3PEX", procs) {
            MdcResult::Dc(String::from("0321"))
        } else {
            MdcResult::Dc(String::from("0301"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M03, "3PP", procs) {
        MdcResult::Dc(String::from("0314"))
    } else if grouper.has_mdc_ppdc(Mdc::M03, "3PL", procs) {
        MdcResult::Dc(String::from("0310"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M03, "3A", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("0360"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0359"))
        } else if grouper.has_mdc_pax("3PCX", procs) {
            MdcResult::Dc(String::from("0361"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("0362"))
        } else {
            MdcResult::Dc(String::from("0350"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M03, "3B", pdx) {
        MdcResult::Dc(String::from("0351"))
    } else if grouper.is_pdx_pdc(Mdc::M03, "3C", pdx) {
        MdcResult::Dc(String::from("0352"))
    } else if grouper.is_pdx_pdc(Mdc::M03, "3H", pdx) {
        if grouper.has_mdc_pax("3PBX", procs) {
            MdcResult::Dc(String::from("0312"))
        } else {
            MdcResult::Dc(String::from("0357"))
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
        let p3pa_recon = process_proc(&GROUPER, &["2183", "2755"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p3pa_recon, MdcResult::Dc(String::from("0321")));
        let p3pa = process_proc(&GROUPER, &["2183"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p3pa, MdcResult::Dc(String::from("0301")));
        let p3pp = process_proc(&GROUPER, &["2104"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p3pp, MdcResult::Dc(String::from("0314")));
        let p3pl = process_proc(&GROUPER, &["0609"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p3pl, MdcResult::Dc(String::from("0310")));
    }

    #[test]
    fn test_process_pdx() {
        let rt = process_pdx(
            &GROUPER,
            "C000",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(rt, MdcResult::Dc(String::from("0360")));
        let rx = process_pdx(
            &GROUPER,
            "C000",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(rx, MdcResult::Dc(String::from("0359")));
        let dx = process_pdx(&GROUPER, "C000", &HashSet::new(), &["2122"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(dx, MdcResult::Dc(String::from("0361")));
        let blood = process_pdx(&GROUPER, "C000", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(blood, MdcResult::Dc(String::from("0362")));

        let p3a = process_pdx(&GROUPER, "C000", &HashSet::new(), &HashSet::new());
        assert_eq!(p3a, MdcResult::Dc(String::from("0350")));
        let p3b = process_pdx(&GROUPER, "H600", &HashSet::new(), &HashSet::new());
        assert_eq!(p3b, MdcResult::Dc(String::from("0351")));
        let p3c = process_pdx(&GROUPER, "J00", &HashSet::new(), &HashSet::new());
        assert_eq!(p3c, MdcResult::Dc(String::from("0352")));
        let p3h_ex = process_pdx(&GROUPER, "K000", &HashSet::new(), &["2301"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p3h_ex, MdcResult::Dc(String::from("0312")));
        let p3h = process_pdx(&GROUPER, "K000", &HashSet::new(), &HashSet::new());
        assert_eq!(p3h, MdcResult::Dc(String::from("0357")));
    }
}
