// Book 2 pdf page 200

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M09, &input.procs) {
        process_proc(grouper, &input.pdx, &input.sdxs, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M09, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.sdxs, &input.procs, &input.age_y)
    }
}

fn process_proc(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M09, "9PJ", procs) {
        MdcResult::Dc(String::from("0911"))
    } else if grouper.has_mdc_ppdc(Mdc::M09, "9PD", procs) {
        if grouper.has_mdc_ax_pdx("9BX", pdx) {
            if grouper.has_mdc_pax("9PBX", procs) {
                MdcResult::Dc(String::from("0910"))
            } else {
                MdcResult::Dc(String::from("0905"))
            }
        } else {
            if grouper.has_mdc_pax("9PBX", procs) {
                MdcResult::Dc(String::from("0915"))
            } else {
                MdcResult::Dc(String::from("0906"))
            }
        }
    } else if grouper.has_mdc_ppdc(Mdc::M09, "9PA", procs) {
        if grouper.has_mdc_pax("9PDX", procs) {
            MdcResult::Dc(String::from("0914"))
        } else {
            if grouper.is_pdx_or_sdxs_pdc(Mdc::M09, "9E", pdx, sdxs) {
                MdcResult::Dc(String::from("0901"))
            } else {
                MdcResult::Dc(String::from("0903"))
            }
        }
    } else if grouper.has_mdc_ppdc(Mdc::M09, "9PF", procs) {
        MdcResult::Dc(String::from("0908"))
    } else if grouper.has_mdc_ppdc(Mdc::M09, "9PB", procs) || grouper.has_mdc_ppdc(Mdc::M09, "9PC", procs) {
        if grouper.is_pdx_or_sdxs_pdc(Mdc::M09, "9E", pdx, sdxs) {
            MdcResult::Dc(String::from("0902"))
        } else if grouper.has_mdc_ppdc(Mdc::M09, "9PB", procs) {
            MdcResult::Dc(String::from("0903"))
        } else {
            // grouper.has_mdc_ppdc(Mdc::M09, "9PC", procs)
            MdcResult::Dc(String::from("0904"))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M09, "9PH", procs) {
        MdcResult::Dc(String::from("0909"))
    } else if grouper.has_mdc_ppdc(Mdc::M09, "9PE", procs) {
        MdcResult::Dc(String::from("0907"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M09, "9A", pdx) {
        MdcResult::Dc(String::from("0950"))
    } else if grouper.is_pdx_pdc(Mdc::M09, "9B", pdx) {
        MdcResult::Dc(String::from("0951"))
    } else if grouper.is_pdx_pdc(Mdc::M09, "9C", pdx) {
        MdcResult::Dc(String::from("0952"))
    } else if grouper.is_pdx_pdc(Mdc::M09, "9D", pdx) {
        MdcResult::Dc(String::from("0953"))
    } else if grouper.is_pdx_pdc(Mdc::M09, "9E", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX + CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) && grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0960"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0961"))
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        } else if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("0962"))
        } else if grouper.has_mdc_pax("9PCX", procs) {
            MdcResult::Dc(String::from("0963"))
        } else if grouper.has_mdc_pax("99PBX", procs) {
            MdcResult::Dc(String::from("0964"))
        } else {
            MdcResult::Dc(String::from("0954"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M09, "9F", pdx) {
        MdcResult::Dc(String::from("0955"))
    } else if grouper.is_pdx_pdc(Mdc::M09, "9G", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("0956")) } else { MdcResult::Dc(String::from("0957")) }
    } else if grouper.is_pdx_pdc(Mdc::M09, "9H", pdx) {
        if *age_y > 17 { MdcResult::Dc(String::from("0958")) } else { MdcResult::Dc(String::from("0959")) }
    } else if grouper.is_pdx_pdc(Mdc::M09, "9J", pdx) {
        // CaRT: SDx as AX 99BX and Proc as AX 99PEX
        if grouper.has_mdc_ax_sdxs("99BX", sdxs) && grouper.has_mdc_pax("99PEX", procs) {
            MdcResult::Dc(String::from("0966"))
        // CaCRx: SDx as AX 99CX and Proc as AX 99PFX
        } else if grouper.has_mdc_ax_sdxs("99CX", sdxs) && grouper.has_mdc_pax("99PFX", procs) {
            MdcResult::Dc(String::from("0967"))
        } else {
            MdcResult::Dc(String::from("0965"))
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
        let p9pj = process_proc(&GROUPER, "", &HashSet::new(), &["8670"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pj, MdcResult::Dc(String::from("0911")));
        let p9pd_ulcer_debride = process_proc(&GROUPER, "L030", &HashSet::new(), &["8585", "8622*1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pd_ulcer_debride, MdcResult::Dc(String::from("0910")));
        let p9pd_ulcer = process_proc(&GROUPER, "L030", &HashSet::new(), &["8585"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pd_ulcer, MdcResult::Dc(String::from("0905")));
        let p9pd_debride = process_proc(&GROUPER, "", &HashSet::new(), &["8585", "8622*1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pd_debride, MdcResult::Dc(String::from("0915")));
        let p9pd = process_proc(&GROUPER, "", &HashSet::new(), &["8585"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pd, MdcResult::Dc(String::from("0906")));
        let p9pa_recon = process_proc(&GROUPER, "", &HashSet::new(), &["8531", "8570"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pa_recon, MdcResult::Dc(String::from("0914")));
        let p9pa_cancer_pdx = process_proc(&GROUPER, "C500", &HashSet::new(), &["8531"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pa_cancer_pdx, MdcResult::Dc(String::from("0901")));
        let p9pa_cancer_sdx = process_proc(
            &GROUPER,
            "",
            &["C500"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["8531"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p9pa_cancer_sdx, MdcResult::Dc(String::from("0901")));
        let p9pa = process_proc(&GROUPER, "", &HashSet::new(), &["8531"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pa, MdcResult::Dc(String::from("0903")));
        let p9pf = process_proc(&GROUPER, "", &HashSet::new(), &["8625"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pf, MdcResult::Dc(String::from("0908")));
        let p9pb = process_proc(&GROUPER, "", &HashSet::new(), &["8522"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pb, MdcResult::Dc(String::from("0903")));
        let p9pb_cancer_pdx = process_proc(&GROUPER, "C500", &HashSet::new(), &["8522"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pb_cancer_pdx, MdcResult::Dc(String::from("0902")));
        let p9pb_cancer_sdx = process_proc(
            &GROUPER,
            "",
            &["C500"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["8522"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p9pb_cancer_sdx, MdcResult::Dc(String::from("0902")));
        let p9pc = process_proc(&GROUPER, "", &HashSet::new(), &["8512"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pc, MdcResult::Dc(String::from("0904")));
        let p9pc_cancer_pdx = process_proc(&GROUPER, "C500", &HashSet::new(), &["8512"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pc_cancer_pdx, MdcResult::Dc(String::from("0902")));
        let p9pc_cancer_sdx = process_proc(
            &GROUPER,
            "",
            &["C500"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["8512"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
        );
        assert_eq!(p9pc_cancer_sdx, MdcResult::Dc(String::from("0902")));

        let p9ph = process_proc(&GROUPER, "", &HashSet::new(), &["8609"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9ph, MdcResult::Dc(String::from("0909")));
        let p9pe = process_proc(&GROUPER, "", &HashSet::new(), &["8621"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p9pe, MdcResult::Dc(String::from("0907")));
    }

    #[test]
    fn test_process_pdx() {
        let p9a = process_pdx(&GROUPER, "L890", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9a, MdcResult::Dc(String::from("0950")));
        let p9b = process_pdx(&GROUPER, "L931", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9b, MdcResult::Dc(String::from("0951")));
        let p9c = process_pdx(&GROUPER, "L920", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9c, MdcResult::Dc(String::from("0952")));
        let p9d = process_pdx(&GROUPER, "A660", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9d, MdcResult::Dc(String::from("0953")));

        let rt_rx = process_pdx(
            &GROUPER,
            "C500",
            &["Z510", "Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223", "9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rt_rx, MdcResult::Dc(String::from("0960")));
        let rx = process_pdx(
            &GROUPER,
            "C500",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rx, MdcResult::Dc(String::from("0961")));
        let rt = process_pdx(
            &GROUPER,
            "C500",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(rt, MdcResult::Dc(String::from("0962")));
        let dx = process_pdx(&GROUPER, "C500", &HashSet::new(), &["8511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(dx, MdcResult::Dc(String::from("0963")));
        let blood = process_pdx(&GROUPER, "C500", &HashSet::new(), &["9903"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &20);
        assert_eq!(blood, MdcResult::Dc(String::from("0964")));

        let p9e = process_pdx(&GROUPER, "C500", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9e, MdcResult::Dc(String::from("0954")));
        let p9f = process_pdx(&GROUPER, "I972", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9f, MdcResult::Dc(String::from("0955")));
        let p9g_o17 = process_pdx(&GROUPER, "L010", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9g_o17, MdcResult::Dc(String::from("0956")));
        let p9g = process_pdx(&GROUPER, "L010", &HashSet::new(), &HashSet::new(), &10);
        assert_eq!(p9g, MdcResult::Dc(String::from("0957")));
        let p9h_o17 = process_pdx(&GROUPER, "S000", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9h_o17, MdcResult::Dc(String::from("0958")));
        let p9h = process_pdx(&GROUPER, "S000", &HashSet::new(), &HashSet::new(), &10);
        assert_eq!(p9h, MdcResult::Dc(String::from("0959")));

        let p9j_rt = process_pdx(
            &GROUPER,
            "A513",
            &["Z510"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9223"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(p9j_rt, MdcResult::Dc(String::from("0966")));
        let p9j_rx = process_pdx(
            &GROUPER,
            "A513",
            &["Z511"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &["9925"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(),
            &20,
        );
        assert_eq!(p9j_rx, MdcResult::Dc(String::from("0967")));
        let p9j = process_pdx(&GROUPER, "A513", &HashSet::new(), &HashSet::new(), &20);
        assert_eq!(p9j, MdcResult::Dc(String::from("0965")));
    }
}
