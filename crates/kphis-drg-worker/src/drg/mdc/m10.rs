// Book 2 pdf page 216

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M10, &input.procs) {
        process_proc(grouper, &input.procs, &input.age_y)
    } else if let Some(uorp_res) = process_uorp(Mdc::M10, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.age_y)
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>, age_y: &u8) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M10, "10PB", procs) {
        MdcResult::Dc(String::from("1001"))
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PC", procs) {
        if *age_y > 59 { MdcResult::Dc(String::from("1003")) } else { MdcResult::Dc(String::from("1004")) }
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PK", procs) {
        MdcResult::Dc(String::from("1013"))
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PE", procs) {
        MdcResult::Dc(String::from("1005"))
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PA", procs) {
        MdcResult::Dc(String::from("1002"))
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PF", procs) {
        MdcResult::Dc(String::from("1007"))
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PG", procs) {
        MdcResult::Dc(String::from("1008"))
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PJ", procs) {
        MdcResult::Dc(String::from("1010"))
    } else if grouper.has_mdc_ppdc(Mdc::M10, "10PH", procs) {
        MdcResult::Dc(String::from("1009"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M10, "10A", pdx) {
        MdcResult::Dc(String::from("1050"))
    } else if grouper.is_pdx_pdc(Mdc::M10, "10B", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("1051")) } else { MdcResult::Dc(String::from("1052")) }
    } else if grouper.is_pdx_pdc(Mdc::M10, "10C", pdx) {
        MdcResult::Dc(String::from("1053"))
    } else if grouper.is_pdx_pdc(Mdc::M10, "10D", pdx) {
        MdcResult::Dc(String::from("1054"))
    } else if grouper.is_pdx_pdc(Mdc::M10, "10F", pdx) {
        MdcResult::Dc(String::from("1056"))
    } else if grouper.is_pdx_pdc(Mdc::M10, "10G", pdx) {
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1059"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("1060"))
        } else {
            MdcResult::Dc(String::from("1061"))
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
        let p10pb = process_proc(&GROUPER, &["0713"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pb, MdcResult::Dc(String::from("1001")));
        let p10pc_o59 = process_proc(&GROUPER, &["8411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &60);
        assert_eq!(p10pc_o59, MdcResult::Dc(String::from("1003")));
        let p10pc = process_proc(&GROUPER, &["8411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pc, MdcResult::Dc(String::from("1004")));
        let p10pk = process_proc(&GROUPER, &["4382"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pk, MdcResult::Dc(String::from("1013")));
        let p10pe = process_proc(&GROUPER, &["4431"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pe, MdcResult::Dc(String::from("1005")));
        let p10pa = process_proc(&GROUPER, &["0700"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pa, MdcResult::Dc(String::from("1002")));
        let p10pf = process_proc(&GROUPER, &["0613"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pf, MdcResult::Dc(String::from("1007")));
        let p10pg = process_proc(&GROUPER, &["0619"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pg, MdcResult::Dc(String::from("1008")));
        let p10pj = process_proc(&GROUPER, &["3802"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10pj, MdcResult::Dc(String::from("1010")));
        let p10ph = process_proc(&GROUPER, &["067"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p10ph, MdcResult::Dc(String::from("1009")));
    }

    #[test]
    fn test_process_pdx() {
        let p10a = process_pdx(&GROUPER, "A187", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p10a, MdcResult::Dc(String::from("1050")));
        let p10b_o17 = process_pdx(&GROUPER, "E40", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p10b_o17, MdcResult::Dc(String::from("1051")));
        let p10b = process_pdx(&GROUPER, "E40", &HashSet::new(), &HashSet::new(), &10);
        assert_eq!(p10b, MdcResult::Dc(String::from("1052")));
        let p10c = process_pdx(&GROUPER, "E45", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p10c, MdcResult::Dc(String::from("1053")));
        let p10d = process_pdx(&GROUPER, "E700", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p10d, MdcResult::Dc(String::from("1054")));
        let p10f = process_pdx(&GROUPER, "E109", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p10f, MdcResult::Dc(String::from("1056")));

        let rx = process_pdx(
            &GROUPER,
            "C740",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rx, MdcResult::Dc(String::from("1059")));
        let rt = process_pdx(
            &GROUPER,
            "C740",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rt, MdcResult::Dc(String::from("1060")));

        let p10g = process_pdx(&GROUPER, "C740", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p10g, MdcResult::Dc(String::from("1061")));
    }
}
