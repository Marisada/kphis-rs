// Book 2 pdf page 124

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M07, &input.procs) {
        process_proc(grouper, &input.pdx, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M07, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.dch_type)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M07, "7PL", procs) {
        MdcResult::Dc(String::from("0716"))
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PA", procs) {
        MdcResult::Dc(String::from("0701"))
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PK", procs) {
        MdcResult::Dc(String::from("0715"))
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PB", procs) {
        if grouper.is_pdx_pdc(Mdc::M07, "7B", pdx) {
            MdcResult::Dc(String::from("0702"))
        } else {
            MdcResult::Dc(String::from("0703"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PG", procs) {
        MdcResult::Dc(String::from("0711"))
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PE", procs) {
        MdcResult::Dc(String::from("0707"))
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PD", procs) {
        MdcResult::Dc(String::from("0706"))
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PJ", procs) {
        MdcResult::Dc(String::from("0714"))
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PC", procs) {
        if grouper.has_mdc_pax("7PBX", procs) {
            MdcResult::Dc(String::from("0704"))
        } else {
            MdcResult::Dc(String::from("0705"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PF", procs) {
        if grouper.has_mdc_pax("7PBX", procs) {
            MdcResult::Dc(String::from("0709"))
        } else {
            MdcResult::Dc(String::from("0710"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M07, "7PH", procs) {
        MdcResult::Dc(String::from("0708"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, dch_type: &str) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M07, "7A", pdx) {
        MdcResult::Dc(String::from("0750"))
    } else if grouper.is_pdx_pdc(Mdc::M07, "7B", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0756"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0757"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("0758"))
        } else if grouper.has_mdc_pax("7PDX", procs) {
            MdcResult::Dc(String::from("0759"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("0760"))
        } else if dch_type == "04" {
            MdcResult::Dc(String::from("0761"))
        } else {
            MdcResult::Dc(String::from("0751"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M07, "7C", pdx) {
        MdcResult::Dc(String::from("0753"))
    } else if grouper.is_pdx_pdc(Mdc::M07, "7D", pdx) {
        MdcResult::Dc(String::from("0754"))
    } else if grouper.is_pdx_pdc(Mdc::M07, "7E", pdx) {
        MdcResult::Dc(String::from("0755"))
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
        let p7ql = process_proc(&GROUPER, "", &["3926"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7ql, MdcResult::Dc(String::from("0716")));
        let p7pa = process_proc(&GROUPER, "", &["5251"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pa, MdcResult::Dc(String::from("0701")));
        let p7pk = process_proc(&GROUPER, "", &["5022"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pk, MdcResult::Dc(String::from("0715")));
        let p7pb_cancer = process_proc(&GROUPER, "C220", &["5102"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pb_cancer, MdcResult::Dc(String::from("0702")));
        let p7pb = process_proc(&GROUPER, "", &["5102"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pb, MdcResult::Dc(String::from("0703")));
        let p7pg = process_proc(&GROUPER, "", &["1763"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pg, MdcResult::Dc(String::from("0711")));
        let p7pe = process_proc(&GROUPER, "", &["4291"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pe, MdcResult::Dc(String::from("0707")));
        let p7pd = process_proc(&GROUPER, "", &["4411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pd, MdcResult::Dc(String::from("0706")));
        let p7pj = process_proc(&GROUPER, "", &["5014"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pj, MdcResult::Dc(String::from("0714")));
        let p7pc_bx = process_proc(&GROUPER, "", &["5121", "5110"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pc_bx, MdcResult::Dc(String::from("0704")));
        let p7pc = process_proc(&GROUPER, "", &["5121"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pc, MdcResult::Dc(String::from("0705")));
        let p7pf_bx = process_proc(&GROUPER, "", &["5123", "5110"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pf_bx, MdcResult::Dc(String::from("0709")));
        let p7pf = process_proc(&GROUPER, "", &["5123"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7pf, MdcResult::Dc(String::from("0710")));
        let p7ph = process_proc(&GROUPER, "", &["5164"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p7ph, MdcResult::Dc(String::from("0708")));
    }

    #[test]
    fn test_process_pdx() {
        let p7a = process_pdx(&GROUPER, "K701", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p7a, MdcResult::Dc(String::from("0750")));

        let rt_rx = process_pdx(
            &GROUPER,
            "C220",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rt_rx, MdcResult::Dc(String::from("0756")));
        let rx = process_pdx(
            &GROUPER,
            "C220",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rx, MdcResult::Dc(String::from("0757")));
        let rt = process_pdx(
            &GROUPER,
            "C220",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rt, MdcResult::Dc(String::from("0758")));
        let dx = process_pdx(&GROUPER, "C220", &HashSet::new(), &["5011"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(dx, MdcResult::Dc(String::from("0759")));
        let blood = process_pdx(&GROUPER, "C220", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(blood, MdcResult::Dc(String::from("0760")));

        let p7b_refer = process_pdx(&GROUPER, "C220", &HashSet::new(), &HashSet::new(), "04");
        assert_eq!(p7b_refer, MdcResult::Dc(String::from("0761")));
        let p7b = process_pdx(&GROUPER, "C220", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p7b, MdcResult::Dc(String::from("0751")));
        let p7c = process_pdx(&GROUPER, "B252", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p7c, MdcResult::Dc(String::from("0753")));
        let p7d = process_pdx(&GROUPER, "A064", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p7d, MdcResult::Dc(String::from("0754")));
        let p7e = process_pdx(&GROUPER, "K800", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p7e, MdcResult::Dc(String::from("0755")));
    }
}
