// Book 2 pdf page 224

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M11, &input.procs) {
        process_proc(grouper, &input.pdx, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M11, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.dch_type, &input.age_y)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M11, "11PA", procs) {
        MdcResult::Dc(String::from("1101"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PL", procs) {
        MdcResult::Dc(String::from("1115"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PC", procs) {
        if grouper.is_pdx_pdc(Mdc::M11, "11C", pdx) {
            MdcResult::Dc(String::from("1103"))
        } else {
            MdcResult::Dc(String::from("1104"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PN", procs) {
        MdcResult::Dc(String::from("1118"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PM", procs) {
        MdcResult::Dc(String::from("1117"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PB", procs) {
        MdcResult::Dc(String::from("1102"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PD", procs) {
        MdcResult::Dc(String::from("1105"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PH", procs) {
        MdcResult::Dc(String::from("1109"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PF", procs) {
        MdcResult::Dc(String::from("1107"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PJ", procs) {
        if grouper.has_mdc_pax("11PBX", procs) {
            MdcResult::Dc(String::from("1116"))
        } else {
            MdcResult::Dc(String::from("1110"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PK", procs) {
        if grouper.has_mdc_pax("11PBX", procs) {
            MdcResult::Dc(String::from("1116"))
        } else {
            MdcResult::Dc(String::from("1111"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PG", procs) {
        MdcResult::Dc(String::from("1108"))
    } else if grouper.has_mdc_ppdc(Mdc::M11, "11PE", procs) {
        MdcResult::Dc(String::from("1106"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, dch_type: &str, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M11, "11A", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("1150")) } else { MdcResult::Dc(String::from("1151")) }
    } else if grouper.is_pdx_pdc(Mdc::M11, "11J", pdx) {
        if *age_y > 17 {
            if dch_type == "04" {
                MdcResult::Dc(String::from("1167"))
            } else {
                MdcResult::Dc(String::from("1159"))
            }
        } else {
            MdcResult::Dc(String::from("1160"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M11, "11B", pdx) {
        MdcResult::Dc(String::from("1152"))
    } else if grouper.is_pdx_pdc(Mdc::M11, "11C", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("1163"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("1162"))
        } else if grouper.has_mdc_pax("11PCX", procs) {
            MdcResult::Dc(String::from("1164"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("1165"))
        } else {
            MdcResult::Dc(String::from("1153"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M11, "11D", pdx) {
        MdcResult::Dc(String::from("1154"))
    } else if grouper.is_pdx_pdc(Mdc::M11, "11E", pdx) {
        if grouper.has_mdc_pax("11PBX", procs) {
            MdcResult::Dc(String::from("1112"))
        } else {
            MdcResult::Dc(String::from("1155"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M11, "11F", pdx) {
        MdcResult::Dc(String::from("1156"))
    } else if grouper.is_pdx_pdc(Mdc::M11, "11G", pdx) {
        MdcResult::Dc(String::from("1157"))
    } else if grouper.is_pdx_pdc(Mdc::M11, "11H", pdx) {
        MdcResult::Dc(String::from("1158"))
    } else if grouper.is_pdx_pdc(Mdc::M11, "11K", pdx) {
        MdcResult::Dc(String::from("1166"))
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
        let p11pa = process_proc(&GROUPER, "", &["5569"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pa, MdcResult::Dc(String::from("1101")));
        let p11pl = process_proc(&GROUPER, "", &["9971"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pl, MdcResult::Dc(String::from("1115")));
        let p11pc_cancer = process_proc(&GROUPER, "C64", &["3924"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pc_cancer, MdcResult::Dc(String::from("1103")));
        let p11pc = process_proc(&GROUPER, "", &["3924"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pc, MdcResult::Dc(String::from("1104")));
        let p11pn = process_proc(&GROUPER, "", &["0681"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pn, MdcResult::Dc(String::from("1118")));
        let p11pm = process_proc(&GROUPER, "", &["5534"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pm, MdcResult::Dc(String::from("1117")));
        let p11pb = process_proc(&GROUPER, "", &["5493"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pb, MdcResult::Dc(String::from("1102")));
        let p11pd = process_proc(&GROUPER, "", &["6021"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pd, MdcResult::Dc(String::from("1105")));
        let p11ph = process_proc(&GROUPER, "", &["6496"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11ph, MdcResult::Dc(String::from("1109")));
        let p11pf = process_proc(&GROUPER, "", &["560"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pf, MdcResult::Dc(String::from("1107")));
        let p11pj_eswl = process_proc(&GROUPER, "", &["5631", "9851"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pj_eswl, MdcResult::Dc(String::from("1116")));
        let p11pj = process_proc(&GROUPER, "", &["5631"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pj, MdcResult::Dc(String::from("1110")));
        let p11pk_eswl = process_proc(&GROUPER, "", &["5691", "9851"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pk_eswl, MdcResult::Dc(String::from("1116")));
        let p11pk = process_proc(&GROUPER, "", &["5691"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pk, MdcResult::Dc(String::from("1111")));
        let p11pg = process_proc(&GROUPER, "", &["5841"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pg, MdcResult::Dc(String::from("1108")));
        let p11pe = process_proc(&GROUPER, "", &["5781"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p11pe, MdcResult::Dc(String::from("1106")));
    }

    #[test]
    fn test_process_pdx() {
        let p11a_o17 = process_pdx(&GROUPER, "I120", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11a_o17, MdcResult::Dc(String::from("1150")));
        let p11a = process_pdx(&GROUPER, "I120", &HashSet::new(), &HashSet::new(), "01", &10);
        assert_eq!(p11a, MdcResult::Dc(String::from("1151")));
        let p11j_o17_refer = process_pdx(&GROUPER, "N170", &HashSet::new(), &HashSet::new(), "04", &20);
        assert_eq!(p11j_o17_refer, MdcResult::Dc(String::from("1167")));
        let p11j_o17 = process_pdx(&GROUPER, "N170", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11j_o17, MdcResult::Dc(String::from("1159")));
        let p11j = process_pdx(&GROUPER, "N170", &HashSet::new(), &HashSet::new(), "01", &10);
        assert_eq!(p11j, MdcResult::Dc(String::from("1160")));
        let p11b = process_pdx(&GROUPER, "Z490", &HashSet::new(), &HashSet::new(), "01", &10);
        assert_eq!(p11b, MdcResult::Dc(String::from("1152")));

        let rt = process_pdx(
            &GROUPER,
            "C64",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
            &20,
        );
        assert_eq!(rt, MdcResult::Dc(String::from("1163")));
        let rx = process_pdx(
            &GROUPER,
            "C64",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
            &20,
        );
        assert_eq!(rx, MdcResult::Dc(String::from("1162")));

        let dx = process_pdx(&GROUPER, "C64", &HashSet::new(), &["5521"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", &20);
        assert_eq!(dx, MdcResult::Dc(String::from("1164")));
        let blood = process_pdx(&GROUPER, "C64", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", &20);
        assert_eq!(blood, MdcResult::Dc(String::from("1165")));

        let p11c = process_pdx(&GROUPER, "C64", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11c, MdcResult::Dc(String::from("1153")));
        let p11d = process_pdx(&GROUPER, "N110", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11d, MdcResult::Dc(String::from("1154")));
        let p11e_eswl = process_pdx(&GROUPER, "N130", &HashSet::new(), &["9851"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", &20);
        assert_eq!(p11e_eswl, MdcResult::Dc(String::from("1112")));
        let p11e = process_pdx(&GROUPER, "N130", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11e, MdcResult::Dc(String::from("1155")));
        let p11f = process_pdx(&GROUPER, "N391", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11f, MdcResult::Dc(String::from("1156")));
        let p11g = process_pdx(&GROUPER, "N350", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11g, MdcResult::Dc(String::from("1157")));
        let p11h = process_pdx(&GROUPER, "E102", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11h, MdcResult::Dc(String::from("1158")));
        let p11k = process_pdx(&GROUPER, "I701", &HashSet::new(), &HashSet::new(), "01", &20);
        assert_eq!(p11k, MdcResult::Dc(String::from("1166")));
    }
}
