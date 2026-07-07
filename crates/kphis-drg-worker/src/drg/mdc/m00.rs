// Book 2 pdf page 10

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M00, &input.procs) {
        // Heart-Lung Transplant (PDx as AX 0DX and Proc as PDC 0PB)
        if grouper.has_mdc_ax_pdx("0DX", &input.pdx) && grouper.has_mdc_ppdc(Mdc::M00, "0PB", &input.procs) {
            MdcResult::Drg(String::from("00029"))
        // Liver Transplant (PDx as AX 0CX and Proc as PDC 0PA)
        } else if grouper.has_mdc_ax_pdx("0CX", &input.pdx) && grouper.has_mdc_ppdc(Mdc::M00, "0PA", &input.procs) {
            MdcResult::Drg(String::from("00019"))
        // Bone Marrow Transplant (PDx as AX 0EX and Proc as PDC 0PD)
        } else if grouper.has_mdc_ax_pdx("0EX", &input.pdx) && grouper.has_mdc_ppdc(Mdc::M00, "0PD", &input.procs) {
            MdcResult::Drg(String::from("00049"))
        // Laryngectomy (PDx as AX 0GX and Proc as PDC 0PG)
        } else if grouper.has_mdc_ax_pdx("0GX", &input.pdx) && grouper.has_mdc_ppdc(Mdc::M00, "0PG", &input.procs) {
            MdcResult::Dc(String::from("0009"))
        // Tracheostomy (Proc as PDC 0PF)
        } else if grouper.has_mdc_ppdc(Mdc::M00, "0PF", &input.procs) {
            // LOS >20 days
            if input.los > 20 {
                // Major Procedure (AX 0PBX)
                if grouper.has_mdc_pax("0PBX", &input.procs) {
                    MdcResult::Dc(String::from("0012"))
                // Cont. Mech Vent 96+ hr (AX 0PCX)
                } else if grouper.has_mdc_pax("0PCX", &input.procs) {
                    MdcResult::Dc(String::from("0010"))
                } else {
                    MdcResult::Dc(String::from("0011"))
                }
            // PDx UAC (AX 0FX)
            } else if grouper.has_mdc_ax_pdx("0FX", &input.pdx) {
                // Upper Airways Proc (AX 0PDX)
                if grouper.has_mdc_pax("0PDX", &input.procs) {
                    MdcResult::Dc(String::from("0006"))
                } else {
                    MdcResult::Dc(String::from("0007"))
                }
            } else {
                process_2nd(grouper, input)
            }
        } else {
            process_2nd(grouper, input)
        }
    } else {
        process_2nd(grouper, input)
    }
}

fn process_2nd(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    // Tracheostomy Status/Revision (PDx or SDx as AX 0HX Or Proc as AX 0PEX)
    if grouper.has_mdc_ax_pdx_or_sdxs("0HX", &input.pdx, &input.sdxs) || grouper.has_mdc_pax("0PEX", &input.procs) {
        // LOS >20 days
        if input.los > 20 {
            // Major Procedure (AX 0PBX)
            if grouper.has_mdc_pax("0PBX", &input.procs) {
                MdcResult::Dc(String::from("0012"))
            // Cont. Mech Vent 96+ hr (AX 0PCX)
            } else if grouper.has_mdc_pax("0PCX", &input.procs) {
                MdcResult::Dc(String::from("0013"))
            } else {
                process_3rd(grouper, input)
            }
        } else {
            process_3rd(grouper, input)
        }
    } else {
        process_3rd(grouper, input)
    }
}

fn process_3rd(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    // MDC 24
    if let Some(pdx_trauma) = grouper.i10(&input.pdx, &input.gender).and_then(|i| i.trauma) {
        let mut sdx_site = input.sdxs.iter().fold(HashSet::new(), |mut acc, sdx| {
            if let Some(site) = grouper.i10(sdx, &input.gender).and_then(|i| i.trauma) {
                acc.insert(site);
            }
            acc
        });
        sdx_site.insert(pdx_trauma);
        let proc_site = grouper.proc_sites(&input.procs);
        if sdx_site.len() > 1 || proc_site.len() > 1 {
            MdcResult::Mdc(Mdc::M24)
        } else {
            process_4th(grouper, input)
        }
    } else {
        process_4th(grouper, input)
    }
}

fn process_4th(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    // MDC 25
    if grouper.i10(&input.pdx, &input.gender).map(|i| i.mdc.as_str() == "25").unwrap_or_default() {
        MdcResult::Mdc(Mdc::M25)
    // MDC 15
    } else if input.age_y == 0 && input.age_d.map(|d| d < 28).unwrap_or_default() {
        MdcResult::Mdc(Mdc::M15)
    // MDC 1-23
    } else if let Some(mdc) = grouper.i10(&input.pdx, &input.gender).and_then(|i| Mdc::new(&i.mdc)) {
        MdcResult::Mdc(mdc)
    // we already checked that PDx is valid (fn check_i10_a234), so this will never happen
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process() {
        let heart_lung_transplant = process(&GROUPER, &GrouperInput::default().set_pdx("C966").set_procs(&["3350"]));
        assert_eq!(heart_lung_transplant, MdcResult::Drg(String::from("00029")));
        let liver_transplant = process(&GROUPER, &GrouperInput::default().set_pdx("B150").set_procs(&["5051"]));
        assert_eq!(liver_transplant, MdcResult::Drg(String::from("00019")));
        let bone_marrow_transplant = process(&GROUPER, &GrouperInput::default().set_pdx("C400").set_procs(&["4100"]));
        assert_eq!(bone_marrow_transplant, MdcResult::Drg(String::from("00049")));
        let laryngectomy = process(&GROUPER, &GrouperInput::default().set_pdx("C101").set_procs(&["301"]));
        assert_eq!(laryngectomy, MdcResult::Dc(String::from("0009")));
        let tracheostomy_loso20_major_proc = process(&GROUPER, &GrouperInput::default().set_procs(&["311", "0132"]).set_los(21));
        assert_eq!(tracheostomy_loso20_major_proc, MdcResult::Dc(String::from("0012")));
        let tracheostomy_loso20_vento96 = process(&GROUPER, &GrouperInput::default().set_procs(&["311", "9672"]).set_los(21));
        assert_eq!(tracheostomy_loso20_vento96, MdcResult::Dc(String::from("0010")));
        let tracheostomy_loso20 = process(&GROUPER, &GrouperInput::default().set_procs(&["311"]).set_los(21));
        assert_eq!(tracheostomy_loso20, MdcResult::Dc(String::from("0011")));
        let tracheostomy_uac_upper_airway = process(&GROUPER, &GrouperInput::default().set_pdx("C01").set_procs(&["311", "062"]));
        assert_eq!(tracheostomy_uac_upper_airway, MdcResult::Dc(String::from("0006")));
        let tracheostomy_uac = process(&GROUPER, &GrouperInput::default().set_pdx("C01").set_procs(&["311"]));
        assert_eq!(tracheostomy_uac, MdcResult::Dc(String::from("0007")));
    }

    #[test]
    fn test_process_2nd() {
        let rev_tracheostomy_loso20_major_proc_by_proc = process(&GROUPER, &GrouperInput::default().set_procs(&["3174", "0132"]).set_los(21));
        assert_eq!(rev_tracheostomy_loso20_major_proc_by_proc, MdcResult::Dc(String::from("0012")));
        let rev_tracheostomy_loso20_major_proc = process(&GROUPER, &GrouperInput::default().set_pdx("J950").set_procs(&["0132"]).set_los(21));
        assert_eq!(rev_tracheostomy_loso20_major_proc, MdcResult::Dc(String::from("0012")));
        let rev_tracheostomy_loso20_vento96 = process(&GROUPER, &GrouperInput::default().set_pdx("J950").set_procs(&["9672"]).set_los(21));
        assert_eq!(rev_tracheostomy_loso20_vento96, MdcResult::Dc(String::from("0013")));
        let rev_tracheostomy_loso20 = process(&GROUPER, &GrouperInput::default().set_pdx("J950").set_los(21));
        assert_eq!(rev_tracheostomy_loso20, MdcResult::Mdc(Mdc::M04));
        let rev_tracheostomy = process(&GROUPER, &GrouperInput::default().set_pdx("J950"));
        assert_eq!(rev_tracheostomy, MdcResult::Mdc(Mdc::M04));
    }

    #[test]
    fn test_process_3rd() {
        let trauma_2site_code = process(&GROUPER, &GrouperInput::default().set_pdx("S3681").set_sdxs(&["S370"]));
        assert_eq!(trauma_2site_code, MdcResult::Mdc(Mdc::M24));
        let trauma_2site_proc = process(&GROUPER, &GrouperInput::default().set_pdx("S3681").set_procs(&["0692", "0700"]));
        assert_eq!(trauma_2site_proc, MdcResult::Mdc(Mdc::M24));
        let trauma_1site_code = process(&GROUPER, &GrouperInput::default().set_pdx("S3681"));
        assert_eq!(trauma_1site_code, MdcResult::Mdc(Mdc::M06));
    }

    #[test]
    fn test_process_4th() {
        let hiv = process(&GROUPER, &GrouperInput::default().set_pdx("B200"));
        assert_eq!(hiv, MdcResult::Mdc(Mdc::M25));
        let new_born = process(&GROUPER, &GrouperInput::default().set_age_y(0).set_age_d(Some(10)));
        assert_eq!(new_born, MdcResult::Mdc(Mdc::M15));
    }
}
