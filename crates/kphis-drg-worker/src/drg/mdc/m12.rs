// Book 2 pdf page 242

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M12, &input.procs) {
        process_proc(grouper, &input.pdx, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M12, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M12, "12PA", procs) {
        MdcResult::Dc(String::from("1201"))
    } else if grouper.has_mdc_ppdc(Mdc::M12, "12PB", procs) {
        MdcResult::Dc(String::from("1202"))
    } else if grouper.has_mdc_ppdc(Mdc::M12, "12PF", procs) {
        if grouper.is_pdx_pdc(Mdc::M12, "12A", pdx) {
            MdcResult::Dc(String::from("1206"))
        } else {
            MdcResult::Dc(String::from("1207"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M12, "12PD", procs) {
        MdcResult::Dc(String::from("1204"))
    } else if grouper.has_mdc_ppdc(Mdc::M12, "12PC", procs) {
        MdcResult::Dc(String::from("1203"))
    } else if grouper.has_mdc_ppdc(Mdc::M12, "12PG", procs) {
        MdcResult::Dc(String::from("1208"))
    } else if grouper.has_mdc_ppdc(Mdc::M12, "12PE", procs) {
        MdcResult::Dc(String::from("1205"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M12, "12A", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1255"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1256"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("1257"))
        } else if grouper.has_mdc_pax("12PBX", procs) {
            MdcResult::Dc(String::from("1258"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1259"))
        } else {
            MdcResult::Dc(String::from("1250"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M12, "12B", pdx) {
        MdcResult::Dc(String::from("1251"))
    } else if grouper.is_pdx_pdc(Mdc::M12, "12C", pdx) {
        MdcResult::Dc(String::from("1252"))
    } else if grouper.is_pdx_pdc(Mdc::M12, "12E", pdx) {
        MdcResult::Dc(String::from("1254"))
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
        let p12pa = process_proc(&GROUPER, "", &["4053"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pa, MdcResult::Dc(String::from("1201")));
        let p12pb = process_proc(&GROUPER, "", &["6021"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pb, MdcResult::Dc(String::from("1202")));
        let p12pf_cancer = process_proc(&GROUPER, "C600", &["5733"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pf_cancer, MdcResult::Dc(String::from("1206")));
        let p12pf = process_proc(&GROUPER, "", &["5733"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pf, MdcResult::Dc(String::from("1207")));
        let p12pd = process_proc(&GROUPER, "", &["5843"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pd, MdcResult::Dc(String::from("1204")));
        let p12pc = process_proc(&GROUPER, "", &["612"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pc, MdcResult::Dc(String::from("1203")));
        let p12pg = process_proc(&GROUPER, "", &["8774"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pg, MdcResult::Dc(String::from("1208")));
        let p12pe = process_proc(&GROUPER, "", &["640"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p12pe, MdcResult::Dc(String::from("1205")));
    }

    #[test]
    fn test_process_pdx() {
        let rt_rx = process_pdx(
            &GROUPER,
            "C600",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(rt_rx, MdcResult::Dc(String::from("1255")));
        let rx = process_pdx(
            &GROUPER,
            "C600",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(rx, MdcResult::Dc(String::from("1256")));
        let rt = process_pdx(
            &GROUPER,
            "C600",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(rt, MdcResult::Dc(String::from("1257")));
        let dx = process_pdx(&GROUPER, "C600", &HashSet::new(), &["5731"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(dx, MdcResult::Dc(String::from("1258")));
        let blood = process_pdx(&GROUPER, "C600", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(blood, MdcResult::Dc(String::from("1259")));

        let p12a = process_pdx(&GROUPER, "C600", &HashSet::new(), &HashSet::new());
        assert_eq!(p12a, MdcResult::Dc(String::from("1250")));
        let p12b = process_pdx(&GROUPER, "N40", &HashSet::new(), &HashSet::new());
        assert_eq!(p12b, MdcResult::Dc(String::from("1251")));
        let p12c = process_pdx(&GROUPER, "N341", &HashSet::new(), &HashSet::new());
        assert_eq!(p12c, MdcResult::Dc(String::from("1252")));
        let p12e = process_pdx(&GROUPER, "D176", &HashSet::new(), &HashSet::new());
        assert_eq!(p12e, MdcResult::Dc(String::from("1254")));
    }
}
