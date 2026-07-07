// Book 2 pdf page 42

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if let Some(result) = process_preluse(grouper, &input.pdx, &input.procs) {
        result
    } else if grouper.has_any_mdc_ppdc(Mdc::M02, &input.procs) {
        process_proc(grouper, &input.procs)
    } else if let Some(uorp_res) = process_uorp(Mdc::M02, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.age_y)
    }
}

fn process_preluse(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> Option<MdcResult> {
    if grouper.has_mdc_ppdc(Mdc::M02, "2PA", procs) {
        Some(MdcResult::Dc(String::from("0201")))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PJ", procs) {
        Some(MdcResult::Dc(String::from("0209")))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PH", procs) {
        if grouper.has_mdc_pax("2PDX", procs) {
            Some(MdcResult::Dc(String::from("0206")))
        } else {
            Some(MdcResult::Dc(String::from("0201")))
        }
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PB", procs) {
        if grouper.is_pdx_pdc(Mdc::M02, "2E", pdx) {
            Some(MdcResult::Dc(String::from("0210")))
        } else {
            Some(MdcResult::Dc(String::from("0202")))
        }
    } else if grouper.has_mdc_ax_pdx("2BX", pdx) && grouper.has_mdc_pax("2PCX", procs) {
        Some(MdcResult::Dc(String::from("0203")))
    } else {
        None
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_ppdc(Mdc::M02, "2PK", procs) {
        MdcResult::Dc(String::from("0211"))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PL", procs) {
        MdcResult::Dc(String::from("0212"))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PE", procs) {
        MdcResult::Dc(String::from("0206"))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PM", procs) {
        MdcResult::Dc(String::from("0213"))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PD", procs) {
        MdcResult::Dc(String::from("0205"))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PF", procs) {
        MdcResult::Dc(String::from("0207"))
    } else if grouper.has_mdc_ppdc(Mdc::M02, "2PG", procs) {
        MdcResult::Dc(String::from("0208"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M02, "2C", pdx) {
        MdcResult::Dc(String::from("0250"))
    } else if grouper.is_pdx_pdc(Mdc::M02, "2A", pdx) {
        if *age_y > 54 { MdcResult::Dc(String::from("0251")) } else { MdcResult::Dc(String::from("0252")) }
    } else if grouper.is_pdx_pdc(Mdc::M02, "2E", pdx) {
        MdcResult::Dc(String::from("0255"))
    } else if grouper.is_pdx_pdc(Mdc::M02, "2B", pdx) {
        MdcResult::Dc(String::from("0253"))
    } else if grouper.is_pdx_pdc(Mdc::M02, "2D", pdx) {
        MdcResult::Dc(String::from("0254"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_prelude() {
        let p2pa = process_preluse(&GROUPER, "", &["1400"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pa, Some(MdcResult::Dc(String::from("0201"))));
        let p2pj = process_preluse(&GROUPER, "", &["1160"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pj, Some(MdcResult::Dc(String::from("0209"))));
        let p2ph_cataract = process_preluse(&GROUPER, "", &["1474", "1342"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2ph_cataract, Some(MdcResult::Dc(String::from("0206"))));
        let p2ph = process_preluse(&GROUPER, "", &["1474"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2ph, Some(MdcResult::Dc(String::from("0201"))));
        let p2pb_ca = process_preluse(&GROUPER, "C441", &["0844"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pb_ca, Some(MdcResult::Dc(String::from("0210"))));
        let p2pb = process_preluse(&GROUPER, "", &["0844"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pb, Some(MdcResult::Dc(String::from("0202"))));
        let p2ax = process_preluse(&GROUPER, "S021", &["0811"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2ax, Some(MdcResult::Dc(String::from("0203"))));
    }

    #[test]
    fn test_process_proc() {
        let p2pk = process_proc(&GROUPER, &["1341>1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pk, MdcResult::Dc(String::from("0211")));
        let p2pl = process_proc(&GROUPER, &["1311>1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pl, MdcResult::Dc(String::from("0212")));
        let p2pe = process_proc(&GROUPER, &["1341"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pe, MdcResult::Dc(String::from("0206")));
        let p2pm = process_proc(&GROUPER, &["090"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pm, MdcResult::Dc(String::from("0213")));
        let p2pd = process_proc(&GROUPER, &["1151"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pd, MdcResult::Dc(String::from("0205")));
        let p2pf = process_proc(&GROUPER, &["1300"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pf, MdcResult::Dc(String::from("0207")));
        let p2pg = process_proc(&GROUPER, &["1471"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p2pg, MdcResult::Dc(String::from("0208")));
    }

    #[test]
    fn test_process_pdx() {
        let p2c = process_pdx(&GROUPER, "H113", &20);
        assert_eq!(p2c, MdcResult::Dc(String::from("0250")));
        let p2a_aged = process_pdx(&GROUPER, "H050", &60);
        assert_eq!(p2a_aged, MdcResult::Dc(String::from("0251")));
        let p2a = process_pdx(&GROUPER, "H050", &20);
        assert_eq!(p2a, MdcResult::Dc(String::from("0252")));
        let p2e = process_pdx(&GROUPER, "C431", &20);
        assert_eq!(p2e, MdcResult::Dc(String::from("0255")));
        let p2b = process_pdx(&GROUPER, "G245", &20);
        assert_eq!(p2b, MdcResult::Dc(String::from("0253")));
        let p2d = process_pdx(&GROUPER, "H000", &20);
        assert_eq!(p2d, MdcResult::Dc(String::from("0254")));
    }
}
