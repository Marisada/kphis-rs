// Book 2 pdf page 360

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_any_mdc_ppdc(Mdc::M24, &input.procs) {
        process_proc(grouper, &input.procs)
    } else if grouper.proc_with_max_orp_group(&input.procs).first().map(|p| p.proc_cgr >= 3).unwrap_or_default() {
        MdcResult::Dc(String::from("2414"))
    } else {
        process_rest(grouper, &input.procs)
    }
}

fn process_proc(grouper: &Grouper, procs: &HashSet<String>) -> MdcResult {
    let proc_sites = grouper.proc_sites(procs);
    if proc_sites.contains(&"E".to_owned())
        && ![String::from("A"), String::from("D"), String::from("G"), String::from("H")]
            .iter()
            .collect::<HashSet<&String>>()
            .is_disjoint(&proc_sites)
    {
        MdcResult::Dc(String::from("2405"))
    } else if (proc_sites.contains(&"A".to_owned()) && ![String::from("D"), String::from("G"), String::from("H")].iter().collect::<HashSet<&String>>().is_disjoint(&proc_sites))
        || (proc_sites.contains(&"D".to_owned()) && proc_sites.contains(&"G".to_owned()))
    {
        MdcResult::Dc(String::from("2401"))
    } else if grouper.has_mdc_pax("24PBX", procs) {
        MdcResult::Dc(String::from("2417"))
    } else if proc_sites.contains(&"E".to_owned()) {
        MdcResult::Dc(String::from("2412"))
    } else if proc_sites.contains(&"A".to_owned()) {
        MdcResult::Dc(String::from("2411"))
    } else if proc_sites.contains(&"D".to_owned()) {
        MdcResult::Dc(String::from("2418"))
    } else if proc_sites.contains(&"C".to_owned()) {
        MdcResult::Dc(String::from("2419"))
    } else if proc_sites.contains(&"B".to_owned()) {
        MdcResult::Dc(String::from("2420"))
    } else if proc_sites.contains(&"G".to_owned()) {
        MdcResult::Dc(String::from("2421"))
    } else if proc_sites.contains(&"F".to_owned()) {
        MdcResult::Dc(String::from("2422"))
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

fn process_rest(grouper: &Grouper, procs: &HashSet<String>) -> MdcResult {
    if grouper.has_mdc_pax("0PCX", procs) {
        MdcResult::Dc(String::from("2423"))
    } else {
        MdcResult::Dc(String::from("2450"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    // 0061 proc site A
    // 1762 proc site B
    // 3179 proc site C
    // 3834 proc site D
    // 8050 proc site E
    // 8071 proc site F
    // 8075 proc site G
    // 8582 proc site H
    // 7939 proc site J

    #[test]
    fn test_process_proc() {
        let p24ea = process_proc(&GROUPER, &["8050", "0061"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24ea, MdcResult::Dc(String::from("2405")));
        let p24ed = process_proc(&GROUPER, &["8050", "3834"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24ed, MdcResult::Dc(String::from("2405")));
        let p24eg = process_proc(&GROUPER, &["8050", "8075"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24eg, MdcResult::Dc(String::from("2405")));
        let p24eh = process_proc(&GROUPER, &["8050", "8582"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24eh, MdcResult::Dc(String::from("2405")));

        let p24ad = process_proc(&GROUPER, &["0061", "3834"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24ad, MdcResult::Dc(String::from("2401")));
        let p24ag = process_proc(&GROUPER, &["0061", "8075"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24ag, MdcResult::Dc(String::from("2401")));
        let p24ah = process_proc(&GROUPER, &["0061", "8582"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24ah, MdcResult::Dc(String::from("2401")));
        let p24dg = process_proc(&GROUPER, &["3834", "8075"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24dg, MdcResult::Dc(String::from("2401")));

        let p24bx = process_proc(&GROUPER, &["8622*1"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24bx, MdcResult::Dc(String::from("2417")));

        let p24e = process_proc(&GROUPER, &["8050"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24e, MdcResult::Dc(String::from("2412")));
        let p24a = process_proc(&GROUPER, &["0061"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24a, MdcResult::Dc(String::from("2411")));
        let p24d = process_proc(&GROUPER, &["3834"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24d, MdcResult::Dc(String::from("2418")));
        let p24c = process_proc(&GROUPER, &["3179"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24c, MdcResult::Dc(String::from("2419")));
        let p24b = process_proc(&GROUPER, &["1762"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24b, MdcResult::Dc(String::from("2420")));
        let p24g = process_proc(&GROUPER, &["8075"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24g, MdcResult::Dc(String::from("2421")));
        let p24f = process_proc(&GROUPER, &["8071"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24f, MdcResult::Dc(String::from("2422")));
    }

    #[test]
    fn test_process_rest() {
        let p24_vent = process_rest(&GROUPER, &["9672"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p24_vent, MdcResult::Dc(String::from("2423")));
        let p24 = process_rest(&GROUPER, &HashSet::new());
        assert_eq!(p24, MdcResult::Dc(String::from("2450")));
    }
}
