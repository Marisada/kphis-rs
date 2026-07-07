// Book 2 pdf page 132

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M08, &input.procs) {
        process_proc(grouper, &input.procs, &input.age_y)
    } else if let Some(uorp_res) = process_uorp(Mdc::M08, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.age_y)
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>, age_y: &u8) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M08, "8QE", procs) {
        MdcResult::Dc(String::from("0835"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8QF", procs) {
        MdcResult::Dc(String::from("0836"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8QD", procs)
        || (grouper.has_mdc_pax("8PCX", procs) && grouper.has_mdc_pax("8PDX", procs))
        || (grouper.has_mdc_pax("8PCX", procs) && grouper.has_mdc_pax("8PEX", procs))
        || (grouper.has_mdc_pax("8PDX", procs) && grouper.has_mdc_pax("8PEX", procs))
    {
        MdcResult::Dc(String::from("0801"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PW", procs) {
        MdcResult::Dc(String::from("0824"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8QB", procs) {
        if grouper.has_mdc_ppdc(Mdc::M08, "8PH", procs) {
            MdcResult::Dc(String::from("0828"))
        } else {
            MdcResult::Dc(String::from("0830"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PX", procs) {
        MdcResult::Dc(String::from("0825"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PD", procs) {
        MdcResult::Dc(String::from("0805"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PA", procs) {
        MdcResult::Dc(String::from("0802"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PF", procs) {
        MdcResult::Dc(String::from("0807"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PE", procs) {
        MdcResult::Dc(String::from("0806"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PV", procs) {
        MdcResult::Dc(String::from("0823"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PC", procs) {
        MdcResult::Dc(String::from("0804"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PB", procs) {
        MdcResult::Dc(String::from("0803"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PK", procs) {
        MdcResult::Dc(String::from("0812"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PJ", procs) {
        if *age_y > 17 { MdcResult::Dc(String::from("0810")) } else { MdcResult::Dc(String::from("0811")) }
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PH", procs) {
        if grouper.has_mdc_ppdc(Mdc::M08, "8QA", procs) {
            MdcResult::Dc(String::from("0829"))
        } else {
            MdcResult::Dc(String::from("0809"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PG", procs) {
        MdcResult::Dc(String::from("0808"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PT", procs) {
        MdcResult::Dc(String::from("0821"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PU", procs) {
        MdcResult::Dc(String::from("0822"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PM", procs) {
        if *age_y > 17 { MdcResult::Dc(String::from("0814")) } else { MdcResult::Dc(String::from("0815")) }
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8QA", procs) {
        MdcResult::Dc(String::from("0831"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PL", procs) {
        MdcResult::Dc(String::from("0813"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PS", procs) {
        MdcResult::Dc(String::from("0820"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PQ", procs) {
        MdcResult::Dc(String::from("0818"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PP", procs) {
        MdcResult::Dc(String::from("0817"))
    } else if grouper.has_mdc_ppdc(Mdc::M08, "8PN", procs) {
        MdcResult::Dc(String::from("0816"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M08, "8A", pdx) {
        MdcResult::Dc(String::from("0850"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8B", pdx) {
        MdcResult::Dc(String::from("0851"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8C", pdx) {
        MdcResult::Dc(String::from("0852"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8D", pdx) {
        MdcResult::Dc(String::from("0853"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8E", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0868"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0869"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("0870"))
        } else if grouper.has_mdc_pax("8PFX", procs) {
            MdcResult::Dc(String::from("0871"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("0872"))
        } else {
            MdcResult::Dc(String::from("0854"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M08, "8F", pdx) {
        MdcResult::Dc(String::from("0855"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8G", pdx) {
        MdcResult::Dc(String::from("0856"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8H", pdx) {
        MdcResult::Dc(String::from("0857"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8J", pdx) {
        MdcResult::Dc(String::from("0858"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8L", pdx) {
        MdcResult::Dc(String::from("0860"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8M", pdx) {
        MdcResult::Dc(String::from("0861"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8N", pdx) {
        MdcResult::Dc(String::from("0862"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8P", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("0863")) } else { MdcResult::Dc(String::from("0864")) }
    } else if grouper.is_pdx_pdc(Mdc::M08, "8Q", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("0865")) } else { MdcResult::Dc(String::from("0866")) }
    } else if grouper.is_pdx_pdc(Mdc::M08, "8R", pdx) {
        MdcResult::Dc(String::from("0867"))
    } else if grouper.is_pdx_pdc(Mdc::M08, "8S", pdx) {
        MdcResult::Dc(String::from("0873"))
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
        let p8qe = process_proc(&GROUPER, &["9971"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8qe, MdcResult::Dc(String::from("0835")));
        let p8qf = process_proc(&GROUPER, &["8622*4"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8qf, MdcResult::Dc(String::from("0836")));
        let p8qd = process_proc(&GROUPER, &["0070>1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8qd, MdcResult::Dc(String::from("0801")));
        let p8cx_dx = process_proc(&GROUPER, &["0070", "0080"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8cx_dx, MdcResult::Dc(String::from("0801")));
        let p8cx_ex = process_proc(&GROUPER, &["0070", "8156"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8cx_ex, MdcResult::Dc(String::from("0801")));
        let p8dx_ex = process_proc(&GROUPER, &["0080", "8156"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8dx_ex, MdcResult::Dc(String::from("0801")));
        let p8pw = process_proc(&GROUPER, &["0070"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pw, MdcResult::Dc(String::from("0824")));
        let p8qb_graft = process_proc(&GROUPER, &["8622*1", "8660"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8qb_graft, MdcResult::Dc(String::from("0828")));
        let p8qb = process_proc(&GROUPER, &["8622*1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8qb, MdcResult::Dc(String::from("0830")));
        let p8px = process_proc(&GROUPER, &["0080"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8px, MdcResult::Dc(String::from("0825")));
        let p8pd = process_proc(&GROUPER, &["8100"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pd, MdcResult::Dc(String::from("0805")));
        let p8pa = process_proc(&GROUPER, &["8151"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pa, MdcResult::Dc(String::from("0802")));
        let p8pf = process_proc(&GROUPER, &["8403"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pf, MdcResult::Dc(String::from("0807")));
        let p8pe = process_proc(&GROUPER, &["8459"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pe, MdcResult::Dc(String::from("0806")));
        let p8pv = process_proc(&GROUPER, &["0086"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pv, MdcResult::Dc(String::from("0823")));
        let p8pc = process_proc(&GROUPER, &["8156"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pc, MdcResult::Dc(String::from("0804")));
        let p8pb = process_proc(&GROUPER, &["8154"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pb, MdcResult::Dc(String::from("0803")));
        let p8pk = process_proc(&GROUPER, &["7706"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pk, MdcResult::Dc(String::from("0812")));
        let p8pj_o17 = process_proc(&GROUPER, &["7705"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pj_o17, MdcResult::Dc(String::from("0810")));
        let p8pj = process_proc(&GROUPER, &["7705"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pj, MdcResult::Dc(String::from("0811")));
        let p8ph_qa = process_proc(&GROUPER, &["8660", "8622"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8ph_qa, MdcResult::Dc(String::from("0829")));
        let p8ph = process_proc(&GROUPER, &["8660"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8ph, MdcResult::Dc(String::from("0809")));
        let p8pg = process_proc(&GROUPER, &["0115"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pg, MdcResult::Dc(String::from("0808")));
        let p8pt = process_proc(&GROUPER, &["8020"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pt, MdcResult::Dc(String::from("0821")));
        let p8pu = process_proc(&GROUPER, &["8010"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pu, MdcResult::Dc(String::from("0822")));
        let p8pm_o17 = process_proc(&GROUPER, &["8017"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(p8pm_o17, MdcResult::Dc(String::from("0814")));
        let p8pm = process_proc(&GROUPER, &["8017"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pm, MdcResult::Dc(String::from("0815")));
        let p8qa = process_proc(&GROUPER, &["8622"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8qa, MdcResult::Dc(String::from("0831")));
        let p8pl = process_proc(&GROUPER, &["7703"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pl, MdcResult::Dc(String::from("0813")));
        let p8ps = process_proc(&GROUPER, &["8070"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8ps, MdcResult::Dc(String::from("0820")));
        let p8pq = process_proc(&GROUPER, &["8085"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pq, MdcResult::Dc(String::from("0818")));
        let p8pp = process_proc(&GROUPER, &["8048"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pp, MdcResult::Dc(String::from("0817")));
        let p8pn = process_proc(&GROUPER, &["8043"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &10);
        assert_eq!(p8pn, MdcResult::Dc(String::from("0816")));
    }

    #[test]
    fn test_process_pdx() {
        let p8a = process_pdx(&GROUPER, "S722", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8a, MdcResult::Dc(String::from("0850")));
        let p8b = process_pdx(&GROUPER, "S323", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8b, MdcResult::Dc(String::from("0851")));
        let p8c = process_pdx(&GROUPER, "S333", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8c, MdcResult::Dc(String::from("0852")));
        let p8d = process_pdx(&GROUPER, "B453", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8d, MdcResult::Dc(String::from("0853")));

        let rt_rx = process_pdx(
            &GROUPER,
            "C400",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rt_rx, MdcResult::Dc(String::from("0868")));
        let rx = process_pdx(
            &GROUPER,
            "C400",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rx, MdcResult::Dc(String::from("0869")));
        let rt = process_pdx(
            &GROUPER,
            "C400",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rt, MdcResult::Dc(String::from("0870")));
        let dx = process_pdx(&GROUPER, "C400", &HashSet::new(), &["8721"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(dx, MdcResult::Dc(String::from("0871")));
        let blood = process_pdx(&GROUPER, "C400", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(blood, MdcResult::Dc(String::from("0872")));

        let p8e = process_pdx(&GROUPER, "C400", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8e, MdcResult::Dc(String::from("0854")));
        let p8f = process_pdx(&GROUPER, "M45", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8f, MdcResult::Dc(String::from("0855")));
        let p8g = process_pdx(&GROUPER, "M730", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8g, MdcResult::Dc(String::from("0856")));
        let p8h = process_pdx(&GROUPER, "M961", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8h, MdcResult::Dc(String::from("0857")));
        let p8j = process_pdx(&GROUPER, "E550", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8j, MdcResult::Dc(String::from("0858")));
        let p8l = process_pdx(&GROUPER, "I730", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8l, MdcResult::Dc(String::from("0860")));
        let p8m = process_pdx(&GROUPER, "M2420", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8m, MdcResult::Dc(String::from("0861")));
        let p8n = process_pdx(&GROUPER, "M966", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8n, MdcResult::Dc(String::from("0862")));
        let p8p_o17 = process_pdx(&GROUPER, "S520", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8p_o17, MdcResult::Dc(String::from("0863")));
        let p8p = process_pdx(&GROUPER, "S520", &HashSet::new(), &HashSet::new(), &10);
        assert_eq!(p8p, MdcResult::Dc(String::from("0864")));
        let p8q_o17 = process_pdx(&GROUPER, "S527", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8q_o17, MdcResult::Dc(String::from("0865")));
        let p8q = process_pdx(&GROUPER, "S527", &HashSet::new(), &HashSet::new(), &10);
        assert_eq!(p8q, MdcResult::Dc(String::from("0866")));
        let p8r = process_pdx(&GROUPER, "T021", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8r, MdcResult::Dc(String::from("0867")));
        let p8s = process_pdx(&GROUPER, "E850", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p8s, MdcResult::Dc(String::from("0873")));
    }
}
