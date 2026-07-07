// Book 2 pdf page 250

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M13, &input.procs) {
        process_proc(grouper, &input.pdx, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M13, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M13, "13PJ", procs) {
        MdcResult::Dc(String::from("1312"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PL", procs) {
        MdcResult::Dc(String::from("1320"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PA", procs) {
        MdcResult::Dc(String::from("1301"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PK", procs) {
        if grouper.is_pdx_pdc(Mdc::M13, "13A", pdx) || grouper.is_pdx_pdc(Mdc::M13, "13C", pdx) {
            MdcResult::Dc(String::from("1313"))
        } else if grouper.is_pdx_pdc(Mdc::M13, "13B", pdx) {
            MdcResult::Dc(String::from("1314"))
        } else {
            MdcResult::Dc(String::from("1316"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PB", procs) {
        if grouper.is_pdx_pdc(Mdc::M13, "13A", pdx) || grouper.is_pdx_pdc(Mdc::M13, "13C", pdx) {
            MdcResult::Dc(String::from("1302"))
        } else if grouper.is_pdx_pdc(Mdc::M13, "13B", pdx) {
            MdcResult::Dc(String::from("1303"))
        } else {
            MdcResult::Dc(String::from("1305"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PM", procs) {
        MdcResult::Dc(String::from("1321"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PH", procs) {
        MdcResult::Dc(String::from("1311"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PC", procs) {
        MdcResult::Dc(String::from("1308"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PF", procs) {
        MdcResult::Dc(String::from("1310"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PE", procs) {
        MdcResult::Dc(String::from("1306"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PD", procs) {
        MdcResult::Dc(String::from("1307"))
    } else if grouper.has_mdc_ppdc(Mdc::M13, "13PG", procs) {
        MdcResult::Dc(String::from("1309"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M13, "13A", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1356"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1357"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("1358"))
        } else if grouper.has_mdc_pax("13PBX", procs) {
            MdcResult::Dc(String::from("1359"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1360"))
        } else {
            MdcResult::Dc(String::from("1350"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M13, "13B", pdx) {
        MdcResult::Dc(String::from("1351"))
    } else if grouper.is_pdx_pdc(Mdc::M13, "13C", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("1363"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1362"))
        } else if grouper.has_mdc_pax("13PBX", procs) {
            MdcResult::Dc(String::from("1364"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1365"))
        } else {
            MdcResult::Dc(String::from("1352"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M13, "13D", pdx) {
        MdcResult::Dc(String::from("1353"))
    } else if grouper.is_pdx_pdc(Mdc::M13, "13E", pdx) {
        MdcResult::Dc(String::from("1354"))
    } else if grouper.is_pdx_pdc(Mdc::M13, "13F", pdx) {
        MdcResult::Dc(String::from("1355"))
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
        let p13pj = process_proc(&GROUPER, "", &["688"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pj, MdcResult::Dc(String::from("1312")));
        let p13pl = process_proc(&GROUPER, "", &["6861"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pl, MdcResult::Dc(String::from("1320")));
        let p13pa = process_proc(&GROUPER, "", &["6869"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pa, MdcResult::Dc(String::from("1301")));
        let p13pk_a = process_proc(&GROUPER, "D390", &["6501"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pk_a, MdcResult::Dc(String::from("1313")));
        let p13pk_c = process_proc(&GROUPER, "D391", &["6501"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pk_c, MdcResult::Dc(String::from("1313")));
        let p13pk_situ = process_proc(&GROUPER, "D060", &["6501"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pk_situ, MdcResult::Dc(String::from("1314")));
        let p13pk = process_proc(&GROUPER, "", &["6501"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pk, MdcResult::Dc(String::from("1316")));
        let p13pb_a = process_proc(&GROUPER, "D390", &["6509"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pb_a, MdcResult::Dc(String::from("1302")));
        let p13pb_c = process_proc(&GROUPER, "D391", &["6509"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pb_c, MdcResult::Dc(String::from("1302")));
        let p13pb_situ = process_proc(&GROUPER, "D060", &["6509"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pb_situ, MdcResult::Dc(String::from("1303")));
        let p13pb = process_proc(&GROUPER, "", &["6509"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pb, MdcResult::Dc(String::from("1305")));
        let p13pm = process_proc(&GROUPER, "", &["6581"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pm, MdcResult::Dc(String::from("1321")));
        let p13ph = process_proc(&GROUPER, "", &["6695"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13ph, MdcResult::Dc(String::from("1311")));
        let p13pc = process_proc(&GROUPER, "", &["6921"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pc, MdcResult::Dc(String::from("1308")));
        let p13pf = process_proc(&GROUPER, "", &["6621"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pf, MdcResult::Dc(String::from("1310")));
        let p13pe = process_proc(&GROUPER, "", &["6631"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pe, MdcResult::Dc(String::from("1306")));
        let p13pd = process_proc(&GROUPER, "", &["6711"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pd, MdcResult::Dc(String::from("1307")));
        let p13pg = process_proc(&GROUPER, "", &["6812"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13pg, MdcResult::Dc(String::from("1309")));
    }

    #[test]
    fn test_process_pdx() {
        let p13a_rt_rx = process_pdx(
            &GROUPER,
            "C549",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p13a_rt_rx, MdcResult::Dc(String::from("1356")));
        let p13a_rx = process_pdx(
            &GROUPER,
            "C549",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p13a_rx, MdcResult::Dc(String::from("1357")));
        let p13a_rt = process_pdx(
            &GROUPER,
            "C549",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p13a_rt, MdcResult::Dc(String::from("1358")));
        let p13a_dx = process_pdx(&GROUPER, "C549", &HashSet::new(), &["7021"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13a_dx, MdcResult::Dc(String::from("1359")));
        let p13a_blood = process_pdx(&GROUPER, "C549", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13a_blood, MdcResult::Dc(String::from("1360")));
        let p13a = process_pdx(&GROUPER, "C549", &HashSet::new(), &HashSet::new());
        assert_eq!(p13a, MdcResult::Dc(String::from("1350")));

        let p13b = process_pdx(&GROUPER, "D060", &HashSet::new(), &HashSet::new());
        assert_eq!(p13b, MdcResult::Dc(String::from("1351")));

        let p13c_rt = process_pdx(
            &GROUPER,
            "D391",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p13c_rt, MdcResult::Dc(String::from("1363")));
        let p13c_rx = process_pdx(
            &GROUPER,
            "D391",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p13c_rx, MdcResult::Dc(String::from("1362")));
        let p13c_dx = process_pdx(&GROUPER, "D391", &HashSet::new(), &["7021"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13c_dx, MdcResult::Dc(String::from("1364")));
        let p13c_blood = process_pdx(&GROUPER, "D391", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p13c_blood, MdcResult::Dc(String::from("1365")));
        let p13c = process_pdx(&GROUPER, "D391", &HashSet::new(), &HashSet::new());
        assert_eq!(p13c, MdcResult::Dc(String::from("1352")));

        let p13d = process_pdx(&GROUPER, "N72", &HashSet::new(), &HashSet::new());
        assert_eq!(p13d, MdcResult::Dc(String::from("1353")));
        let p13e = process_pdx(&GROUPER, "N700", &HashSet::new(), &HashSet::new());
        assert_eq!(p13e, MdcResult::Dc(String::from("1354")));
        let p13f = process_pdx(&GROUPER, "E280", &HashSet::new(), &HashSet::new());
        assert_eq!(p13f, MdcResult::Dc(String::from("1355")));
    }
}
