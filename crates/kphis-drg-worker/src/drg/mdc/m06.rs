// Book 2 pdf page 102

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M06, &input.procs) {
        process_proc(grouper, &input.pdx, &input.procs, &input.age_y)
    } else if let Some(uorp_res) = process_uorp(Mdc::M06, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.age_y, &input.dch_type)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, procs: &HashSet<String>, age_y: &u8) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M06, "6QB", procs) {
        MdcResult::Dc(String::from("0640"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PV", procs) {
        MdcResult::Dc(String::from("0630"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PS", procs) {
        MdcResult::Dc(String::from("0627"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PW", procs) {
        MdcResult::Dc(String::from("0635"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PB", procs) {
        MdcResult::Dc(String::from("0603"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PX", procs) {
        MdcResult::Dc(String::from("0636"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PA", procs) {
        MdcResult::Dc(String::from("0604"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6QC", procs) {
        MdcResult::Dc(String::from("0643"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PT", procs) {
        MdcResult::Dc(String::from("0628"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PE", procs) {
        if grouper.is_pdx_pdc(Mdc::M06, "6A", pdx) {
            MdcResult::Dc(String::from("0601"))
        } else {
            MdcResult::Dc(String::from("0602"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PY", procs) {
        MdcResult::Dc(String::from("0637"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PC", procs) {
        MdcResult::Dc(String::from("0605"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PK", procs) {
        if grouper.is_pdx_pdc(Mdc::M06, "6A", pdx) {
            MdcResult::Dc(String::from("0613"))
        } else {
            MdcResult::Dc(String::from("0614"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PR", procs) {
        MdcResult::Dc(String::from("0626"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PM", procs) {
        MdcResult::Dc(String::from("0616"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PZ", procs) {
        MdcResult::Dc(String::from("0638"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PD", procs) {
        MdcResult::Dc(String::from("0608"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PU", procs) {
        MdcResult::Dc(String::from("0629"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PL", procs) {
        MdcResult::Dc(String::from("0615"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PJ", procs) {
        if grouper.has_mdc_ax_pdx("6CX", pdx) {
            MdcResult::Dc(String::from("0632"))
        } else {
            MdcResult::Dc(String::from("0607"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6QD", procs) {
        MdcResult::Dc(String::from("0644"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PN", procs) {
        if grouper.has_mdc_ax_pdx("6BX", pdx) {
            MdcResult::Dc(String::from("0619"))
        } else {
            MdcResult::Dc(String::from("0621"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PP", procs) {
        MdcResult::Dc(String::from("0623"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PQ", procs) {
        MdcResult::Dc(String::from("0624"))
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PG", procs) || grouper.has_mdc_ppdc(Mdc::M06, "6PH", procs) {
        if *age_y > 14 {
            if grouper.has_mdc_ppdc(Mdc::M06, "6PH", procs) {
                MdcResult::Dc(String::from("0610"))
            } else {
                MdcResult::Dc(String::from("0611"))
            }
        } else {
            MdcResult::Dc(String::from("0612"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M06, "6PF", procs) {
        MdcResult::Dc(String::from("0609"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, age_y: &u8, dch_type: &str) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M06, "6A", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0668"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0669"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("0670"))
        } else if grouper.has_mdc_pax("6PBX", procs) {
            MdcResult::Dc(String::from("0671"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("0672"))
        } else if dch_type == "04" {
            MdcResult::Dc(String::from("0673"))
        } else {
            MdcResult::Dc(String::from("0650"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M06, "6C", pdx) {
        MdcResult::Dc(String::from("0651"))
    } else if grouper.is_pdx_pdc(Mdc::M06, "6D", pdx) {
        MdcResult::Dc(String::from("0654"))
    } else if grouper.is_pdx_pdc(Mdc::M06, "6E", pdx) {
        MdcResult::Dc(String::from("0655"))
    } else if grouper.is_pdx_pdc(Mdc::M06, "6F", pdx) {
        if dch_type == "04" {
            MdcResult::Dc(String::from("0674"))
        } else {
            MdcResult::Dc(String::from("0656"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M06, "6N", pdx) {
        if *age_y > 9 { MdcResult::Dc(String::from("0676")) } else { MdcResult::Dc(String::from("0658")) }
    } else if grouper.is_pdx_pdc(Mdc::M06, "6G", pdx) {
        if *age_y > 9 { MdcResult::Dc(String::from("0657")) } else { MdcResult::Dc(String::from("0658")) }
    } else if grouper.is_pdx_pdc(Mdc::M06, "6H", pdx) {
        if *age_y > 9 { MdcResult::Dc(String::from("0666")) } else { MdcResult::Dc(String::from("0663")) }
    } else if grouper.is_pdx_pdc(Mdc::M06, "6J", pdx) {
        if dch_type == "04" {
            MdcResult::Dc(String::from("0675"))
        } else {
            MdcResult::Dc(String::from("0660"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M06, "6K", pdx) {
        MdcResult::Dc(String::from("0661"))
    } else if grouper.is_pdx_pdc(Mdc::M06, "6L", pdx) {
        if *age_y > 9 { MdcResult::Dc(String::from("0662")) } else { MdcResult::Dc(String::from("0663")) }
    } else if grouper.is_pdx_pdc(Mdc::M06, "6M", pdx) {
        if *age_y > 9 { MdcResult::Dc(String::from("0664")) } else { MdcResult::Dc(String::from("0665")) }
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
        let p6qb = process_proc(&GROUPER, "", &["4240"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6qb, MdcResult::Dc(String::from("0640")));
        let p6pv = process_proc(&GROUPER, "", &["435"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pv, MdcResult::Dc(String::from("0630")));
        let p6ps = process_proc(&GROUPER, "", &["4438"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6ps, MdcResult::Dc(String::from("0627")));
        let p6pw = process_proc(&GROUPER, "", &["4842"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pw, MdcResult::Dc(String::from("0635")));
        let p6pb = process_proc(&GROUPER, "", &["3806"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pb, MdcResult::Dc(String::from("0603")));
        let p6px = process_proc(&GROUPER, "", &["4581"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6px, MdcResult::Dc(String::from("0636")));
        let p6pa = process_proc(&GROUPER, "", &["4840"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pa, MdcResult::Dc(String::from("0604")));
        let p6qc = process_proc(&GROUPER, "", &["2931"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6qc, MdcResult::Dc(String::from("0643")));
        let p6pt = process_proc(&GROUPER, "", &["5451"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pt, MdcResult::Dc(String::from("0628")));
        let p6pe_cancer = process_proc(&GROUPER, "C150", &["4319"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pe_cancer, MdcResult::Dc(String::from("0601")));
        let p6pe = process_proc(&GROUPER, "", &["4319"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pe, MdcResult::Dc(String::from("0602")));
        let p6py = process_proc(&GROUPER, "", &["5342"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6py, MdcResult::Dc(String::from("0637")));
        let p6pc = process_proc(&GROUPER, "", &["5459"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pc, MdcResult::Dc(String::from("0605")));
        let p6pk_cancer = process_proc(&GROUPER, "C150", &["5411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pk_cancer, MdcResult::Dc(String::from("0613")));
        let p6pk = process_proc(&GROUPER, "", &["5411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pk, MdcResult::Dc(String::from("0614")));
        let p6pr = process_proc(&GROUPER, "", &["4281"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pr, MdcResult::Dc(String::from("0626")));
        let p6pm = process_proc(&GROUPER, "", &["4311"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pm, MdcResult::Dc(String::from("0616")));
        let p6pz = process_proc(&GROUPER, "", &["1711"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pz, MdcResult::Dc(String::from("0638")));
        let p6pd = process_proc(&GROUPER, "", &["4500"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pd, MdcResult::Dc(String::from("0608")));
        let p6pu = process_proc(&GROUPER, "", &["4701"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pu, MdcResult::Dc(String::from("0629")));
        let p6pl = process_proc(&GROUPER, "", &["433"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pl, MdcResult::Dc(String::from("0615")));
        let p6pj_comp = process_proc(&GROUPER, "C181", &["4709"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pj_comp, MdcResult::Dc(String::from("0632")));
        let p6pj = process_proc(&GROUPER, "", &["4709"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pj, MdcResult::Dc(String::from("0607")));
        let p6qd = process_proc(&GROUPER, "", &["4223*1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6qd, MdcResult::Dc(String::from("0644")));
        let p6pn_maj = process_proc(&GROUPER, "C150", &["4222"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pn_maj, MdcResult::Dc(String::from("0619")));
        let p6pn = process_proc(&GROUPER, "", &["4222"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pn, MdcResult::Dc(String::from("0621")));
        let p6pp = process_proc(&GROUPER, "", &["4542"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pp, MdcResult::Dc(String::from("0623")));
        let p6pq = process_proc(&GROUPER, "", &["4522"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pq, MdcResult::Dc(String::from("0624")));
        let p6pg_o14_ph = process_proc(&GROUPER, "", &["5341", "5339"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pg_o14_ph, MdcResult::Dc(String::from("0610")));
        let p6ph_o14 = process_proc(&GROUPER, "", &["5339"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6ph_o14, MdcResult::Dc(String::from("0610")));
        let p6pg_o14 = process_proc(&GROUPER, "", &["5341"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pg_o14, MdcResult::Dc(String::from("0611")));
        let p6pg_l14 = process_proc(&GROUPER, "", &["5341"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p6pg_l14, MdcResult::Dc(String::from("0612")));
        let p6ph_l14 = process_proc(&GROUPER, "", &["5339"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p6ph_l14, MdcResult::Dc(String::from("0612")));
        let p6pf = process_proc(&GROUPER, "", &["4533"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p6pf, MdcResult::Dc(String::from("0609")));
    }

    #[test]
    fn test_process_pdx() {
        let rt_rx = process_pdx(
            &GROUPER,
            "C150",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
            "01",
        );
        assert_eq!(rt_rx, MdcResult::Dc(String::from("0668")));
        let rx = process_pdx(
            &GROUPER,
            "C150",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
            "01",
        );
        assert_eq!(rx, MdcResult::Dc(String::from("0669")));
        let rt = process_pdx(
            &GROUPER,
            "C150",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
            "01",
        );
        assert_eq!(rt, MdcResult::Dc(String::from("0670")));
        let dx = process_pdx(&GROUPER, "C150", &HashSet::new(), &["4222"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20, "01");
        assert_eq!(dx, MdcResult::Dc(String::from("0671")));
        let blood = process_pdx(&GROUPER, "C150", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20, "01");
        assert_eq!(blood, MdcResult::Dc(String::from("0672")));

        let p6a_refer = process_pdx(&GROUPER, "C150", &HashSet::new(), &HashSet::new(), &20, "04");
        assert_eq!(p6a_refer, MdcResult::Dc(String::from("0673")));
        let p6a = process_pdx(&GROUPER, "C150", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6a, MdcResult::Dc(String::from("0650")));
        let p6c = process_pdx(&GROUPER, "I850", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6c, MdcResult::Dc(String::from("0651")));
        let p6d = process_pdx(&GROUPER, "K253", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6d, MdcResult::Dc(String::from("0654")));
        let p6e = process_pdx(&GROUPER, "K500", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6e, MdcResult::Dc(String::from("0655")));
        let p6f_refer = process_pdx(&GROUPER, "K561", &HashSet::new(), &HashSet::new(), &20, "04");
        assert_eq!(p6f_refer, MdcResult::Dc(String::from("0674")));
        let p6f = process_pdx(&GROUPER, "K561", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6f, MdcResult::Dc(String::from("0656")));
        let p6n_o9 = process_pdx(&GROUPER, "K522", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6n_o9, MdcResult::Dc(String::from("0676")));
        let p6n = process_pdx(&GROUPER, "K522", &HashSet::new(), &HashSet::new(), &5, "01");
        assert_eq!(p6n, MdcResult::Dc(String::from("0658")));
        let p6g_o9 = process_pdx(&GROUPER, "A000", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6g_o9, MdcResult::Dc(String::from("0657")));
        let p6g = process_pdx(&GROUPER, "A000", &HashSet::new(), &HashSet::new(), &5, "01");
        assert_eq!(p6g, MdcResult::Dc(String::from("0658")));
        let p6h_o9 = process_pdx(&GROUPER, "B378", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6h_o9, MdcResult::Dc(String::from("0666")));
        let p6h = process_pdx(&GROUPER, "B378", &HashSet::new(), &HashSet::new(), &5, "01");
        assert_eq!(p6h, MdcResult::Dc(String::from("0663")));
        let p6j_refer = process_pdx(&GROUPER, "D120", &HashSet::new(), &HashSet::new(), &20, "04");
        assert_eq!(p6j_refer, MdcResult::Dc(String::from("0675")));
        let p6j = process_pdx(&GROUPER, "D120", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6j, MdcResult::Dc(String::from("0660")));
        let p6k = process_pdx(&GROUPER, "I880", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6k, MdcResult::Dc(String::from("0661")));

        let p6l_o9 = process_pdx(&GROUPER, "B462", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6l_o9, MdcResult::Dc(String::from("0662")));
        let p6l = process_pdx(&GROUPER, "B462", &HashSet::new(), &HashSet::new(), &5, "01");
        assert_eq!(p6l, MdcResult::Dc(String::from("0663")));
        let p6m_o9 = process_pdx(&GROUPER, "K210", &HashSet::new(), &HashSet::new(), &20, "01");
        assert_eq!(p6m_o9, MdcResult::Dc(String::from("0664")));
        let p6m = process_pdx(&GROUPER, "K210", &HashSet::new(), &HashSet::new(), &5, "01");
        assert_eq!(p6m, MdcResult::Dc(String::from("0665")));
    }
}
