// Book 2 pdf page 278

use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    process_proc(grouper, &input.procs, &input.dch_type, input.los, input.age_y, input.age_d, input.adm_wt)
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>, dch_type: &str, los: u32, age_y: u8, age_d: Option<u16>, adm_wt: Option<u16>) -> MdcResult {
    if los < 5 && ["04", "08", "09"].contains(&dch_type) {
        if grouper.has_mdc_pax("15PBX", procs) || grouper.has_mdc_pax("15PDX", procs) || grouper.has_mdc_pax("15PEX", procs) {
            MdcResult::Dc(String::from("1501"))
        } else if grouper.has_mdc_pax("15PCX", procs) {
            MdcResult::Dc(String::from("1502"))
        } else if dch_type == "04" {
            MdcResult::Dc(String::from("1550"))
        } else {
            MdcResult::Dc(String::from("1555"))
        }
    } else if grouper.has_mdc_pax("15PEX", procs) {
        MdcResult::Dc(String::from("1512"))
    } else if grouper.has_mdc_pax("15PDX", procs) {
        MdcResult::Dc(String::from("1511"))
    } else if (adm_wt.unwrap_or_default() > 2499) || age_y > 0 || (age_y == 0 && age_d.unwrap_or_default() > 27) {
        if grouper.has_mdc_pax("15PBX", procs) {
            MdcResult::Dc(String::from("1509"))
        } else if grouper.has_mdc_pax("15PCX", procs) {
            MdcResult::Dc(String::from("1510"))
        } else {
            MdcResult::Dc(String::from("1554"))
        }
    } else if adm_wt.unwrap_or_default() > 1499 {
        if grouper.has_mdc_pax("15PBX", procs) {
            MdcResult::Dc(String::from("1507"))
        } else if grouper.has_mdc_pax("15PCX", procs) {
            MdcResult::Dc(String::from("1508"))
        } else {
            MdcResult::Dc(String::from("1553"))
        }
    } else if adm_wt.unwrap_or_default() > 999 {
        if grouper.has_mdc_pax("15PBX", procs) {
            MdcResult::Dc(String::from("1505"))
        } else if grouper.has_mdc_pax("15PCX", procs) {
            MdcResult::Dc(String::from("1506"))
        } else {
            MdcResult::Dc(String::from("1552"))
        }
    } else if grouper.has_mdc_pax("15PBX", procs) {
        MdcResult::Dc(String::from("1503"))
    } else if grouper.has_mdc_pax("15PCX", procs) {
        MdcResult::Dc(String::from("1504"))
    } else {
        MdcResult::Dc(String::from("1551"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_proc() {
        let p15_severe_bx = process_proc(&GROUPER, &["0114"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "04", 3, 0, Some(15), Some(3000));
        assert_eq!(p15_severe_bx, MdcResult::Dc(String::from("1501")));
        let p15_severe_dx = process_proc(&GROUPER, &["0123"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "08", 3, 0, Some(15), Some(3000));
        assert_eq!(p15_severe_dx, MdcResult::Dc(String::from("1501")));
        let p15_severe_ex = process_proc(&GROUPER, &["3500"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "09", 3, 0, Some(15), Some(3000));
        assert_eq!(p15_severe_ex, MdcResult::Dc(String::from("1501")));
        let p15_severe_cx = process_proc(&GROUPER, &["0096"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "09", 3, 0, Some(15), Some(3000));
        assert_eq!(p15_severe_cx, MdcResult::Dc(String::from("1502")));
        let p15_severe_died = process_proc(&GROUPER, &HashSet::new(), "09", 3, 0, Some(15), Some(3000));
        assert_eq!(p15_severe_died, MdcResult::Dc(String::from("1555")));
        let p15_severe_refer = process_proc(&GROUPER, &HashSet::new(), "04", 3, 0, Some(15), Some(3000));
        assert_eq!(p15_severe_refer, MdcResult::Dc(String::from("1550")));
        let p15_ex = process_proc(&GROUPER, &["3500"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "04", 7, 0, Some(15), Some(3000));
        assert_eq!(p15_ex, MdcResult::Dc(String::from("1512")));
        let p15_dx = process_proc(&GROUPER, &["0123"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "04", 7, 0, Some(15), Some(3000));
        assert_eq!(p15_dx, MdcResult::Dc(String::from("1511")));

        let p15_aged_bx = process_proc(&GROUPER, &["0114"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 1, Some(15), None);
        assert_eq!(p15_aged_bx, MdcResult::Dc(String::from("1509")));
        let p15_aged_cx = process_proc(&GROUPER, &["0096"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 1, Some(15), None);
        assert_eq!(p15_aged_cx, MdcResult::Dc(String::from("1510")));
        let p15_aged = process_proc(&GROUPER, &HashSet::new(), "01", 7, 1, Some(15), None);
        assert_eq!(p15_aged, MdcResult::Dc(String::from("1554")));

        let p15_2499_bx = process_proc(&GROUPER, &["0114"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(3000));
        assert_eq!(p15_2499_bx, MdcResult::Dc(String::from("1509")));
        let p15_2499_cx = process_proc(&GROUPER, &["0096"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(3000));
        assert_eq!(p15_2499_cx, MdcResult::Dc(String::from("1510")));
        let p15_2499 = process_proc(&GROUPER, &HashSet::new(), "01", 7, 0, Some(15), Some(3000));
        assert_eq!(p15_2499, MdcResult::Dc(String::from("1554")));

        let p15_1499_bx = process_proc(&GROUPER, &["0114"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(2000));
        assert_eq!(p15_1499_bx, MdcResult::Dc(String::from("1507")));
        let p15_1499_cx = process_proc(&GROUPER, &["0096"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(2000));
        assert_eq!(p15_1499_cx, MdcResult::Dc(String::from("1508")));
        let p15_1499 = process_proc(&GROUPER, &HashSet::new(), "01", 7, 0, Some(15), Some(2000));
        assert_eq!(p15_1499, MdcResult::Dc(String::from("1553")));

        let p15_999_bx = process_proc(&GROUPER, &["0114"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(1300));
        assert_eq!(p15_999_bx, MdcResult::Dc(String::from("1505")));
        let p15_999_cx = process_proc(&GROUPER, &["0096"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(1300));
        assert_eq!(p15_999_cx, MdcResult::Dc(String::from("1506")));
        let p15_999 = process_proc(&GROUPER, &HashSet::new(), "01", 7, 0, Some(15), Some(1300));
        assert_eq!(p15_999, MdcResult::Dc(String::from("1552")));

        let p15_bx = process_proc(&GROUPER, &["0114"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(800));
        assert_eq!(p15_bx, MdcResult::Dc(String::from("1503")));
        let p15_cx = process_proc(&GROUPER, &["0096"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "01", 7, 0, Some(15), Some(800));
        assert_eq!(p15_cx, MdcResult::Dc(String::from("1504")));
        let p15 = process_proc(&GROUPER, &HashSet::new(), "01", 7, 0, Some(15), Some(800));
        assert_eq!(p15, MdcResult::Dc(String::from("1551")));
    }
}
