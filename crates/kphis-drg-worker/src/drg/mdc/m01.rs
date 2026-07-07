// Book 2 pdf page 22

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M01, &input.procs) {
        process_proc(grouper, &input.pdx, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M01, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.dch_type)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M01, "1PK", procs) {
        if grouper.has_mdc_ax_pdx("1BX", pdx) {
            MdcResult::Dc(String::from("0112"))
        } else {
            MdcResult::Dc(String::from("0113"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PL", procs) {
        MdcResult::Dc(String::from("0117"))
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PJ", procs) {
        MdcResult::Dc(String::from("0114"))
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PH", procs) {
        if grouper.has_mdc_ax_pdx("1CX", pdx) {
            MdcResult::Dc(String::from("0108"))
        } else {
            MdcResult::Dc(String::from("0109"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PB", procs) {
        if grouper.has_mdc_ax_pdx("1BX", pdx) {
            MdcResult::Dc(String::from("0101"))
        } else {
            MdcResult::Dc(String::from("0102"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PC", procs) {
        MdcResult::Dc(String::from("0103"))
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PD", procs) {
        MdcResult::Dc(String::from("0105"))
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PA", procs) {
        MdcResult::Dc(String::from("0104"))
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PF", procs) || grouper.has_mdc_ppdc(Mdc::M01, "1PG", procs) {
        MdcResult::Dc(String::from("0106"))
    } else if grouper.has_mdc_ppdc(Mdc::M01, "1PE", procs) {
        MdcResult::Dc(String::from("0107"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, dch_type: &str) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M01, "1A", pdx) {
        MdcResult::Dc(String::from("0150"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1B", pdx) {
        MdcResult::Dc(String::from("0151"))
    // [CaRT: SDx as AX 99BX and Proc as AX 99PEX] + [CaCRx: SDx as AX 99CX and Proc as AX 99PFX]
    } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
        MdcResult::Dc(String::from("0170"))
    // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
    } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
        MdcResult::Dc(String::from("0171"))
    // CaRT: SDx as AX 99BX and Proc as AX 99PEX
    } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
        MdcResult::Dc(String::from("0172"))
    } else if grouper.has_mdc_pax("1PBX", procs) {
        MdcResult::Dc(String::from("0173"))
    } else if grouper.has_mdc_pax("99PBX", procs) {
        MdcResult::Dc(String::from("0174"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1C", pdx) {
        MdcResult::Dc(String::from("0152"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1D", pdx) {
        MdcResult::Dc(String::from("0153"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1E", pdx) {
        MdcResult::Dc(String::from("0154"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1F", pdx) {
        MdcResult::Dc(String::from("0155"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1G", pdx) {
        MdcResult::Dc(String::from("0156"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1H", pdx) {
        MdcResult::Dc(String::from("0157"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1J", pdx) {
        MdcResult::Dc(String::from("0158"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1K", pdx) {
        if dch_type == "04" {
            MdcResult::Dc(String::from("0176"))
        } else {
            MdcResult::Dc(String::from("0159"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M01, "1L", pdx) {
        MdcResult::Dc(String::from("0160"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1M", pdx) {
        MdcResult::Dc(String::from("0161"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1N", pdx) {
        MdcResult::Dc(String::from("0162"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1P", pdx) {
        MdcResult::Dc(String::from("0163"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1Q", pdx) {
        MdcResult::Dc(String::from("0164"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1R", pdx) {
        MdcResult::Dc(String::from("0165"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1S", pdx) {
        MdcResult::Dc(String::from("0166"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1T", pdx) {
        MdcResult::Dc(String::from("0167"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1U", pdx) {
        MdcResult::Dc(String::from("0168"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1V", pdx) {
        MdcResult::Dc(String::from("0169"))
    } else if grouper.is_pdx_pdc(Mdc::M01, "1W", pdx) {
        MdcResult::Dc(String::from("0177"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

// CaRT: SDx as AX 99BX and Proc as AX 99PEX
// CaCRx: SDx as AX 99CX and Proc as AX 99PFX
// ITCCRx: Proc Combination as AX 99PGX

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_proc() {
        let p1pk_trauma = process_proc(&GROUPER, "F072", &["0121>1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pk_trauma, MdcResult::Dc(String::from("0112")));
        let p1pk = process_proc(&GROUPER, "", &["0121>1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pk, MdcResult::Dc(String::from("0113")));
        let p1pl = process_proc(&GROUPER, "", &["9971"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pl, MdcResult::Dc(String::from("0117")));
        let p1pj = process_proc(&GROUPER, "", &["0062"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pj, MdcResult::Dc(String::from("0114")));
        let p1ph_hemorrhage = process_proc(&GROUPER, "I600", &["0213"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1ph_hemorrhage, MdcResult::Dc(String::from("0108")));
        let p1ph = process_proc(&GROUPER, "", &["0213"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1ph, MdcResult::Dc(String::from("0109")));
        let p1pb_trauma = process_proc(&GROUPER, "F072", &["0110"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pb_trauma, MdcResult::Dc(String::from("0101")));
        let p1pb = process_proc(&GROUPER, "", &["0110"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pb, MdcResult::Dc(String::from("0102")));
        let p1pc = process_proc(&GROUPER, "", &["0301"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pc, MdcResult::Dc(String::from("0103")));
        let p1pd = process_proc(&GROUPER, "", &["0061"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pd, MdcResult::Dc(String::from("0105")));
        let p1pa = process_proc(&GROUPER, "", &["0242"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pa, MdcResult::Dc(String::from("0104")));
        let p1pf = process_proc(&GROUPER, "", &["0402"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pf, MdcResult::Dc(String::from("0106")));
        let p1pg = process_proc(&GROUPER, "", &["C7080"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pg, MdcResult::Dc(String::from("0106")));
        let p1pe = process_proc(&GROUPER, "", &["0443"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p1pe, MdcResult::Dc(String::from("0107")));
    }

    #[test]
    fn test_process_pdx() {
        let p1a = process_pdx(&GROUPER, "G041", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1a, MdcResult::Dc(String::from("0150")));
        let p1b = process_pdx(&GROUPER, "G800", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1b, MdcResult::Dc(String::from("0151")));

        let rt_rx = process_pdx(
            &GROUPER,
            "",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rt_rx, MdcResult::Dc(String::from("0170")));
        let rx = process_pdx(
            &GROUPER,
            "",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rx, MdcResult::Dc(String::from("0171")));
        let rt = process_pdx(
            &GROUPER,
            "",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rt, MdcResult::Dc(String::from("0172")));
        let dx = process_pdx(&GROUPER, "", &HashSet::new(), &["0111"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(dx, MdcResult::Dc(String::from("0173")));
        let blood = process_pdx(&GROUPER, "", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(blood, MdcResult::Dc(String::from("0174")));

        let p1c = process_pdx(&GROUPER, "C700", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1c, MdcResult::Dc(String::from("0152")));
        let p1d = process_pdx(&GROUPER, "E512", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1d, MdcResult::Dc(String::from("0153")));
        let p1e = process_pdx(&GROUPER, "G110", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1e, MdcResult::Dc(String::from("0154")));
        let p1f = process_pdx(&GROUPER, "G463", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1f, MdcResult::Dc(String::from("0155")));
        let p1g = process_pdx(&GROUPER, "I650", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1g, MdcResult::Dc(String::from("0156")));
        let p1h = process_pdx(&GROUPER, "I64", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1h, MdcResult::Dc(String::from("0157")));
        let p1j = process_pdx(&GROUPER, "M213", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1j, MdcResult::Dc(String::from("0158")));
        let p1k_refer = process_pdx(&GROUPER, "A066", &HashSet::new(), &HashSet::new(), "04");
        assert_eq!(p1k_refer, MdcResult::Dc(String::from("0176")));
        let p1k = process_pdx(&GROUPER, "A066", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1k, MdcResult::Dc(String::from("0159")));
        let p1l = process_pdx(&GROUPER, "A870", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1l, MdcResult::Dc(String::from("0160")));
        let p1m = process_pdx(&GROUPER, "G935", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1m, MdcResult::Dc(String::from("0161")));
        let p1n = process_pdx(&GROUPER, "R560", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1n, MdcResult::Dc(String::from("0162")));
        let p1p = process_pdx(&GROUPER, "R568", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1p, MdcResult::Dc(String::from("0163")));
        let p1q = process_pdx(&GROUPER, "F072", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1q, MdcResult::Dc(String::from("0164")));
        let p1r = process_pdx(&GROUPER, "S061", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1r, MdcResult::Dc(String::from("0165")));
        let p1s = process_pdx(&GROUPER, "T020", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1s, MdcResult::Dc(String::from("0166")));
        let p1t = process_pdx(&GROUPER, "S060", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1t, MdcResult::Dc(String::from("0167")));
        let p1u = process_pdx(&GROUPER, "T850", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1u, MdcResult::Dc(String::from("0168")));
        let p1v = process_pdx(&GROUPER, "G610", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1v, MdcResult::Dc(String::from("0169")));
        let p1w = process_pdx(&GROUPER, "S140", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p1w, MdcResult::Dc(String::from("0177")));
    }
}
