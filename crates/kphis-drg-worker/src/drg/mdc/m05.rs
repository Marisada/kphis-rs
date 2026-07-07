// Book 2 pdf page 82

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M05, "5A", &input.pdx) {
        process_pdx_5a(grouper, &input.pdx, &input.sdxs, &input.procs, &input.dch_type)
    } else if grouper.has_any_mdc_ppdc(Mdc::M05, &input.procs) {
        process_proc(grouper, &input.pdx, &input.sdxs, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M05, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.dch_type)
    }
}

fn process_pdx_5a(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, dch_type: &str) -> MdcResult {
    if grouper.has_mdc_pax("5PEX", procs) {
        if grouper.has_mdc_pax("5PCX", procs) {
            MdcResult::Dc(String::from("0525"))
        } else {
            MdcResult::Dc(String::from("0526"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PK", procs) {
        MdcResult::Dc(String::from("0510"))
    } else if grouper.has_mdc_pax("5PFX", procs) {
        MdcResult::Dc(String::from("0527"))
    } else if grouper.has_mdc_pax("5PGX", procs) {
        MdcResult::Dc(String::from("0529"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PT", procs) {
        if grouper.has_mdc_ax_pdx_or_sdxs("5DX", pdx, sdxs) {
            MdcResult::Dc(String::from("0521"))
        } else {
            MdcResult::Dc(String::from("0522"))
        }
    } else if grouper.has_mdc_pax("5PHX", procs) {
        if grouper.has_mdc_ax_sdxs("5CX", sdxs) {
            MdcResult::Dc(String::from("0550"))
        } else {
            MdcResult::Dc(String::from("0551"))
        }
    } else if dch_type == "04" {
        MdcResult::Dc(String::from("0569"))
    } else if grouper.has_mdc_ax_sdxs("5CX", sdxs) {
        MdcResult::Dc(String::from("0552"))
    } else {
        MdcResult::Dc(String::from("0553"))
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M05, "5PE", procs) {
        MdcResult::Dc(String::from("0507"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PC", procs) {
        if grouper.has_mdc_pax("5PCX", procs) {
            MdcResult::Dc(String::from("0503"))
        } else {
            if grouper.has_mdc_ppdc(Mdc::M05, "5PT", procs) {
                MdcResult::Dc(String::from("0504"))
            } else {
                MdcResult::Dc(String::from("0505"))
            }
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PX", procs) {
        MdcResult::Dc(String::from("0537"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PV", procs) {
        if grouper.has_mdc_pax("5PBX", procs) {
            MdcResult::Dc(String::from("0532"))
        } else {
            MdcResult::Dc(String::from("0533"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PA", procs) {
        if grouper.has_mdc_pax("5PBX", procs) {
            MdcResult::Dc(String::from("0501"))
        } else {
            MdcResult::Dc(String::from("0502"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PD", procs) {
        MdcResult::Dc(String::from("0506"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PF", procs) {
        MdcResult::Dc(String::from("0508"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PU", procs) {
        MdcResult::Dc(String::from("0513"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PG", procs) {
        if grouper.has_mdc_pax("5PDX", procs) {
            MdcResult::Dc(String::from("0523"))
        } else {
            MdcResult::Dc(String::from("0524"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PW", procs) {
        MdcResult::Dc(String::from("0536"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PH", procs) {
        MdcResult::Dc(String::from("0514"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PM", procs) {
        MdcResult::Dc(String::from("0512"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PJ", procs) {
        MdcResult::Dc(String::from("0509"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PS", procs) {
        if grouper.has_mdc_pax("5PJX", procs) {
            MdcResult::Dc(String::from("0531"))
        } else {
            MdcResult::Dc(String::from("0515"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PK", procs) {
        if grouper.has_mdc_ax_pdx("5BX", pdx) {
            MdcResult::Dc(String::from("0510"))
        } else {
            MdcResult::Dc(String::from("0511"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PN", procs) {
        MdcResult::Dc(String::from("0517"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PR", procs) {
        MdcResult::Dc(String::from("0520"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PT", procs) {
        if grouper.has_mdc_ax_pdx_or_sdxs("5DX", pdx, sdxs) {
            MdcResult::Dc(String::from("0521"))
        } else {
            MdcResult::Dc(String::from("0522"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PL", procs) {
        MdcResult::Dc(String::from("0516"))
    } else if grouper.has_mdc_ppdc(Mdc::M05, "5PQ", procs) {
        MdcResult::Dc(String::from("0519"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, dch_type: &str) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M05, "5B", pdx) {
        MdcResult::Dc(String::from("0554"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5C", pdx) {
        MdcResult::Dc(String::from("0555"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5D", pdx) {
        MdcResult::Dc(String::from("0556"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5E", pdx) {
        MdcResult::Dc(String::from("0557"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5F", pdx) {
        if dch_type == "04" {
            MdcResult::Dc(String::from("0570"))
        } else {
            MdcResult::Dc(String::from("0558"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M05, "5G", pdx) {
        MdcResult::Dc(String::from("0559"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5H", pdx) {
        MdcResult::Dc(String::from("0560"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5J", pdx) {
        MdcResult::Dc(String::from("0561"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5K", pdx) {
        MdcResult::Dc(String::from("0562"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5L", pdx) {
        MdcResult::Dc(String::from("0563"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5M", pdx) {
        MdcResult::Dc(String::from("0564"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5P", pdx) {
        MdcResult::Dc(String::from("0566"))
    } else if grouper.is_pdx_pdc(Mdc::M05, "5R", pdx) {
        MdcResult::Dc(String::from("0568"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_pdx_5a() {
        let p5pex_cx = process_pdx_5a(
            &GROUPER,
            "I210",
            &HashSet::new(),
            &["3553", "0066"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(p5pex_cx, MdcResult::Dc(String::from("0525")));
        let p5pex = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &["3553"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p5pex, MdcResult::Dc(String::from("0526")));
        let p5pk = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &["C5657"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p5pk, MdcResult::Dc(String::from("0510")));
        let p5pfx = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &["0041"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p5pfx, MdcResult::Dc(String::from("0527")));
        let p5pgx = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &["0040"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p5pgx, MdcResult::Dc(String::from("0529")));
        let p5pt_dx = process_pdx_5a(
            &GROUPER,
            "I210",
            &["I110"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["8852"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(p5pt_dx, MdcResult::Dc(String::from("0521")));
        let p5pt = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &["8852"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p5pt, MdcResult::Dc(String::from("0522")));
        let p5phx_cx = process_pdx_5a(
            &GROUPER,
            "I210",
            &["G463"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9910"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(p5phx_cx, MdcResult::Dc(String::from("0550")));
        let p5phx = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &["9910"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p5phx, MdcResult::Dc(String::from("0551")));
        let p5_refer = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &HashSet::new(), "04");
        assert_eq!(p5_refer, MdcResult::Dc(String::from("0569")));
        let p5cx = process_pdx_5a(&GROUPER, "I210", &["G463"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &HashSet::new(), "01");
        assert_eq!(p5cx, MdcResult::Dc(String::from("0552")));
        let p5 = process_pdx_5a(&GROUPER, "I210", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p5, MdcResult::Dc(String::from("0553")));
    }

    #[test]
    fn test_process_proc() {
        let p5pe = process_proc(&GROUPER, "", &HashSet::new(), &["C4445"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pe, MdcResult::Dc(String::from("0507")));
        let p5pc_cx = process_proc(&GROUPER, "", &HashSet::new(), &["3610", "0066"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pc_cx, MdcResult::Dc(String::from("0503")));
        let p5pc_pt = process_proc(&GROUPER, "", &HashSet::new(), &["3610", "3721"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pc_pt, MdcResult::Dc(String::from("0504")));
        let p5pc = process_proc(&GROUPER, "", &HashSet::new(), &["3610"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pc, MdcResult::Dc(String::from("0505")));
        let p5px = process_proc(&GROUPER, "", &HashSet::new(), &["3542"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5px, MdcResult::Dc(String::from("0537")));
        let p5pv_bx = process_proc(&GROUPER, "", &HashSet::new(), &["CK001", "3721"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pv_bx, MdcResult::Dc(String::from("0532")));
        let p5pv = process_proc(&GROUPER, "", &HashSet::new(), &["CK001"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pv, MdcResult::Dc(String::from("0533")));
        let p5pa_bx = process_proc(&GROUPER, "", &HashSet::new(), &["3505", "3721"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pa_bx, MdcResult::Dc(String::from("0501")));
        let p5pa = process_proc(&GROUPER, "", &HashSet::new(), &["3505"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pa, MdcResult::Dc(String::from("0502")));
        let p5pd = process_proc(&GROUPER, "", &HashSet::new(), &["3531"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pd, MdcResult::Dc(String::from("0506")));
        let p5pf = process_proc(&GROUPER, "", &HashSet::new(), &["3712"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pf, MdcResult::Dc(String::from("0508")));
        let p5pu = process_proc(&GROUPER, "", &HashSet::new(), &["3534"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pu, MdcResult::Dc(String::from("0513")));
        let p5pg_dx = process_proc(&GROUPER, "", &HashSet::new(), &["3596", "3606"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pg_dx, MdcResult::Dc(String::from("0523")));
        let p5pg = process_proc(&GROUPER, "", &HashSet::new(), &["3596"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pg, MdcResult::Dc(String::from("0524")));
        let p5pw = process_proc(&GROUPER, "", &HashSet::new(), &["8622*1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pw, MdcResult::Dc(String::from("0536")));
        let p5ph = process_proc(&GROUPER, "", &HashSet::new(), &["3720"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5ph, MdcResult::Dc(String::from("0514")));
        let p5pm = process_proc(&GROUPER, "", &HashSet::new(), &["3794"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pm, MdcResult::Dc(String::from("0512")));
        let p5pj = process_proc(&GROUPER, "", &HashSet::new(), &["8407"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pj, MdcResult::Dc(String::from("0509")));
        let p5ps_jx = process_proc(&GROUPER, "", &HashSet::new(), &["0061", "0055"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5ps_jx, MdcResult::Dc(String::from("0531")));
        let p5ps = process_proc(&GROUPER, "", &HashSet::new(), &["0061"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5ps, MdcResult::Dc(String::from("0515")));
        let p5pk_bx = process_proc(&GROUPER, "I110", &HashSet::new(), &["C5657"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pk_bx, MdcResult::Dc(String::from("0510")));
        let p5pk = process_proc(&GROUPER, "", &HashSet::new(), &["C5657"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pk, MdcResult::Dc(String::from("0511")));
        let p5pn = process_proc(&GROUPER, "", &HashSet::new(), &["0056"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pn, MdcResult::Dc(String::from("0517")));
        let p5pr = process_proc(&GROUPER, "", &HashSet::new(), &["0523"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pr, MdcResult::Dc(String::from("0520")));
        let p5pt_dx = process_proc(
            &GROUPER,
            "",
            &["I130"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["3721"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p5pt_dx, MdcResult::Dc(String::from("0521")));
        let p5pt = process_proc(&GROUPER, "", &HashSet::new(), &["3721"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pt, MdcResult::Dc(String::from("0522")));
        let p5pl = process_proc(&GROUPER, "", &HashSet::new(), &["8400"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pl, MdcResult::Dc(String::from("0516")));
        let p5pq = process_proc(&GROUPER, "", &HashSet::new(), &["3809"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p5pq, MdcResult::Dc(String::from("0519")));
    }

    #[test]
    fn test_process_pdx() {
        let p5b = process_pdx(&GROUPER, "B376", "01");
        assert_eq!(p5b, MdcResult::Dc(String::from("0554")));
        let p5c = process_pdx(&GROUPER, "I110", "01");
        assert_eq!(p5c, MdcResult::Dc(String::from("0555")));
        let p5d = process_pdx(&GROUPER, "I800", "01");
        assert_eq!(p5d, MdcResult::Dc(String::from("0556")));
        let p5e = process_pdx(&GROUPER, "I830", "01");
        assert_eq!(p5e, MdcResult::Dc(String::from("0557")));
        let p5f_refer = process_pdx(&GROUPER, "I821", "04");
        assert_eq!(p5f_refer, MdcResult::Dc(String::from("0570")));
        let p5f = process_pdx(&GROUPER, "I821", "01");
        assert_eq!(p5f, MdcResult::Dc(String::from("0558")));
        let p5g = process_pdx(&GROUPER, "I878", "01");
        assert_eq!(p5g, MdcResult::Dc(String::from("0559")));
        let p5h = process_pdx(&GROUPER, "I10", "01");
        assert_eq!(p5h, MdcResult::Dc(String::from("0560")));
        let p5j = process_pdx(&GROUPER, "Q200", "01");
        assert_eq!(p5j, MdcResult::Dc(String::from("0561")));
        let p5k = process_pdx(&GROUPER, "R010", "01");
        assert_eq!(p5k, MdcResult::Dc(String::from("0562")));
        let p5l = process_pdx(&GROUPER, "R960", "01");
        assert_eq!(p5l, MdcResult::Dc(String::from("0563")));
        let p5m = process_pdx(&GROUPER, "I440", "01");
        assert_eq!(p5m, MdcResult::Dc(String::from("0564")));
        let p5p = process_pdx(&GROUPER, "I951", "01");
        assert_eq!(p5p, MdcResult::Dc(String::from("0566")));
        let p5r = process_pdx(&GROUPER, "I950", "01");
        assert_eq!(p5r, MdcResult::Dc(String::from("0568")));
    }
}
