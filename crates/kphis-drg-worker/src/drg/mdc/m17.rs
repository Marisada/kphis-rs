// Book 2 pdf page 296

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M17, &input.procs) {
        process_proc(grouper, &input.pdx, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M17, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M17, "17PA", procs) {
        if grouper.is_pdx_pdc(Mdc::M17, "17A", pdx) || grouper.is_pdx_pdc(Mdc::M17, "17B", pdx) {
            MdcResult::Dc(String::from("1701"))
        } else if grouper.is_pdx_pdc(Mdc::M17, "17C", pdx) {
            MdcResult::Dc(String::from("1702"))
        } else {
            MdcResult::Drg(String::from("26509"))
        }
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M17, "17A", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if (grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs))
            || (grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_pax("17PBX", procs))
            || (grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) && grouper.has_mdc_pax("17PBX", procs))
        {
            MdcResult::Dc(String::from("1756"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1757"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("1758"))
        } else if grouper.has_mdc_pax("17PBX", procs) {
            MdcResult::Dc(String::from("1759"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1760"))
        } else {
            MdcResult::Dc(String::from("1750"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M17, "17B", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        // or ITCCRx: Proc Combination as AX 99PGX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs)
            && grouper.has_mdc_pax("99PEX", procs)
            && ((grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs)) || grouper.has_mdc_pax("99PGX", procs))
        {
            MdcResult::Dc(String::from("1761"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        // or ITCCRx: Proc Combination as AX 99PGX
        } else if (grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs)) || grouper.has_mdc_pax("99PGX", procs) {
            MdcResult::Dc(String::from("1762"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("1763"))
        } else if grouper.has_mdc_pax("17PBX", procs) {
            MdcResult::Dc(String::from("1764"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1765"))
        } else {
            MdcResult::Dc(String::from("1751"))
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
        let p17pa_a = process_proc(&GROUPER, "C910", &["0112"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17pa_a, MdcResult::Dc(String::from("1701")));
        let p17pa_b = process_proc(&GROUPER, "C911", &["0112"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17pa_b, MdcResult::Dc(String::from("1701")));
        let p17pa_c = process_proc(&GROUPER, "C960", &["0112"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17pa_c, MdcResult::Dc(String::from("1702")));
    }

    #[test]
    fn test_process_pdx() {
        let p17a_rt_rx = process_pdx(
            &GROUPER,
            "C910",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17a_rt_rx, MdcResult::Dc(String::from("1756")));
        let p17a_rt_dx = process_pdx(
            &GROUPER,
            "C910",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "0111"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17a_rt_dx, MdcResult::Dc(String::from("1756")));
        let p17a_rx_dx = process_pdx(
            &GROUPER,
            "C910",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925", "0111"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17a_rx_dx, MdcResult::Dc(String::from("1756")));
        let p17a_rx = process_pdx(
            &GROUPER,
            "C910",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17a_rx, MdcResult::Dc(String::from("1757")));
        let p17a_rt = process_pdx(
            &GROUPER,
            "C910",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17a_rt, MdcResult::Dc(String::from("1758")));
        let p17a_dx = process_pdx(&GROUPER, "C910", &HashSet::new(), &["0111"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17a_dx, MdcResult::Dc(String::from("1759")));
        let p17a_blood = process_pdx(&GROUPER, "C910", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17a_blood, MdcResult::Dc(String::from("1760")));
        let p17a = process_pdx(&GROUPER, "C910", &HashSet::new(), &HashSet::new());
        assert_eq!(p17a, MdcResult::Dc(String::from("1750")));

        let p17b_rt_rx = process_pdx(
            &GROUPER,
            "C911",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17b_rt_rx, MdcResult::Dc(String::from("1761")));
        let p17b_rt_crx = process_pdx(
            &GROUPER,
            "C911",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "CE022"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17b_rt_crx, MdcResult::Dc(String::from("1761")));
        let p17b_rx = process_pdx(
            &GROUPER,
            "C911",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17b_rx, MdcResult::Dc(String::from("1762")));
        let p17b_crx = process_pdx(&GROUPER, "C911", &HashSet::new(), &["CE022"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17b_crx, MdcResult::Dc(String::from("1762")));
        let p17b_rt = process_pdx(
            &GROUPER,
            "C911",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p17b_rt, MdcResult::Dc(String::from("1763")));
        let p17b_dx = process_pdx(&GROUPER, "C911", &HashSet::new(), &["0111"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17b_dx, MdcResult::Dc(String::from("1764")));
        let p17b_blood = process_pdx(&GROUPER, "C911", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p17b_blood, MdcResult::Dc(String::from("1765")));
        let p17b = process_pdx(&GROUPER, "C911", &HashSet::new(), &HashSet::new());
        assert_eq!(p17b, MdcResult::Dc(String::from("1751")));
    }
}
