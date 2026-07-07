// Book 2 pdf page 70

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M04, &input.procs) {
        process_proc(grouper, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M04, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.dch_type)
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M04, "4PA", procs) {
        MdcResult::Dc(String::from("0401"))
    } else if grouper.has_mdc_ppdc(Mdc::M04, "4PF", procs) {
        MdcResult::Dc(String::from("0409"))
    } else if grouper.has_mdc_ppdc(Mdc::M04, "4PG", procs) {
        MdcResult::Dc(String::from("0410"))
    } else if grouper.has_mdc_ppdc(Mdc::M04, "4PB", procs) {
        MdcResult::Dc(String::from("0402"))
    } else if grouper.has_mdc_ppdc(Mdc::M04, "4PD", procs) {
        MdcResult::Dc(String::from("0403"))
    } else if grouper.has_mdc_ppdc(Mdc::M04, "4PE", procs) {
        MdcResult::Dc(String::from("0407"))
    } else if grouper.has_mdc_ppdc(Mdc::M04, "4PC", procs) {
        MdcResult::Dc(String::from("0408"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, dch_type: &str) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M04, "4B", pdx) {
        MdcResult::Dc(String::from("0451"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4C", pdx) {
        MdcResult::Dc(String::from("0452"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4D", pdx) {
        MdcResult::Dc(String::from("0453"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4E", pdx) {
        if dch_type == "04" {
            MdcResult::Dc(String::from("0471"))
        } else {
            MdcResult::Dc(String::from("0454"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M04, "4F", pdx) {
        MdcResult::Dc(String::from("0455"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4G", pdx) {
        MdcResult::Dc(String::from("0456"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4H", pdx) {
        MdcResult::Dc(String::from("0457"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4J", pdx) {
        MdcResult::Dc(String::from("0458"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4K", pdx) {
        MdcResult::Dc(String::from("0459"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4M", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0465"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0466"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("0467"))
        } else if grouper.has_mdc_pax("4PCX", procs) {
            MdcResult::Dc(String::from("0468"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("0469"))
        } else {
            MdcResult::Dc(String::from("0461"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M04, "4M", pdx) {
        MdcResult::Dc(String::from("0461"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4R", pdx) {
        MdcResult::Dc(String::from("0470"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4N", pdx) {
        if dch_type == "04" {
            MdcResult::Dc(String::from("0472"))
        } else {
            MdcResult::Dc(String::from("0462"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M04, "4P", pdx) {
        MdcResult::Dc(String::from("0463"))
    } else if grouper.is_pdx_pdc(Mdc::M04, "4Q", pdx) {
        MdcResult::Dc(String::from("0464"))
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
        let p4pa = process_proc(&GROUPER, &["321"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p4pa, MdcResult::Dc(String::from("0401")));
        let p4pf = process_proc(&GROUPER, &["3220"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p4pf, MdcResult::Dc(String::from("0409")));
        let p4pg = process_proc(&GROUPER, &["3320"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p4pg, MdcResult::Dc(String::from("0410")));
        let p4pb = process_proc(&GROUPER, &["0716"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p4pb, MdcResult::Dc(String::from("0402")));
        let p4pd = process_proc(&GROUPER, &["9670"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p4pd, MdcResult::Dc(String::from("0403")));
        let p4pe = process_proc(&GROUPER, &["9390"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p4pe, MdcResult::Dc(String::from("0407")));
        let p4pc = process_proc(&GROUPER, &["2161"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p4pc, MdcResult::Dc(String::from("0408")));
    }

    #[test]
    fn test_process_pdx() {
        let p4b = process_pdx(&GROUPER, "I260", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4b, MdcResult::Dc(String::from("0451")));
        let p4c = process_pdx(&GROUPER, "E321", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4c, MdcResult::Dc(String::from("0452")));
        let p4d = process_pdx(&GROUPER, "G473", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4d, MdcResult::Dc(String::from("0453")));
        let p4e_refer = process_pdx(&GROUPER, "J681", &HashSet::new(), &HashSet::new(), "04");
        assert_eq!(p4e_refer, MdcResult::Dc(String::from("0471")));
        let p4e = process_pdx(&GROUPER, "J681", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4e, MdcResult::Dc(String::from("0454")));
        let p4f = process_pdx(&GROUPER, "J684", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4f, MdcResult::Dc(String::from("0455")));
        let p4g = process_pdx(&GROUPER, "S222", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4g, MdcResult::Dc(String::from("0456")));
        let p4h = process_pdx(&GROUPER, "R042", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4h, MdcResult::Dc(String::from("0457")));
        let p4j = process_pdx(&GROUPER, "S270", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4j, MdcResult::Dc(String::from("0458")));
        let p4k = process_pdx(&GROUPER, "A370", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4k, MdcResult::Dc(String::from("0459")));

        let rt_rx = process_pdx(
            &GROUPER,
            "C33",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rt_rx, MdcResult::Dc(String::from("0465")));
        let rx = process_pdx(
            &GROUPER,
            "C33",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rx, MdcResult::Dc(String::from("0466")));
        let rt = process_pdx(
            &GROUPER,
            "C33",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            "01",
        );
        assert_eq!(rt, MdcResult::Dc(String::from("0467")));
        let dx = process_pdx(&GROUPER, "C33", &HashSet::new(), &["3322"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(dx, MdcResult::Dc(String::from("0468")));
        let blood = process_pdx(&GROUPER, "C33", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(blood, MdcResult::Dc(String::from("0469")));

        let p4m = process_pdx(&GROUPER, "C33", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4m, MdcResult::Dc(String::from("0461")));
        let p4r = process_pdx(&GROUPER, "J860", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4r, MdcResult::Dc(String::from("0470")));
        let p4n_refer = process_pdx(&GROUPER, "J90", &HashSet::new(), &HashSet::new(), "04");
        assert_eq!(p4n_refer, MdcResult::Dc(String::from("0472")));
        let p4n = process_pdx(&GROUPER, "J90", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4n, MdcResult::Dc(String::from("0462")));
        let p4p = process_pdx(&GROUPER, "J990", &HashSet::new(), &["2301"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01");
        assert_eq!(p4p, MdcResult::Dc(String::from("0463")));
        let p4q = process_pdx(&GROUPER, "J981", &HashSet::new(), &HashSet::new(), "01");
        assert_eq!(p4q, MdcResult::Dc(String::from("0464")));
    }
}
