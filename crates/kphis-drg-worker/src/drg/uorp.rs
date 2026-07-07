use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use super::{
    grouper::Grouper,
    mdc::Mdc,
    model::{GrouperInput, MdcResult, Proc},
};

pub(crate) fn process_uorp(mdc: Mdc, grouper: &Grouper, input: &GrouperInput) -> Option<MdcResult> {
    process_uorp_1st(mdc, grouper, &input.pdx, &input.sdxs, &input.procs, &input.gender, input.los)
}

// Book 1 page 122
fn process_uorp_1st(mdc: Mdc, grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, procs: &HashSet<String>, gender: &Option<String>, los: u32) -> Option<MdcResult> {
    let max_procs = grouper.proc_with_max_orp_group(procs);
    if !max_procs.is_empty() {
        // #1 MDC reassignment by Proc and PDx
        if grouper.is_pdx_mosdx(pdx, gender) {
            if let Some(new_mdc) = max_procs[0].m_mdc.as_ref().and_then(|s| Mdc::new(s)) {
                if new_mdc == mdc { None } else { Some(MdcResult::Mdc(new_mdc)) }
            } else {
                process_uorp_2nd(mdc, grouper, sdxs, &max_procs, gender, los)
            }
        } else {
            process_uorp_2nd(mdc, grouper, sdxs, &max_procs, gender, los)
        }
    } else {
        None
    }
}

// #2 MDC reassignment by Proc and SDx
fn process_uorp_2nd(mdc: Mdc, grouper: &Grouper, sdxs: &HashSet<String>, max_procs: &[&Arc<Proc>], gender: &Option<String>, los: u32) -> Option<MdcResult> {
    let mut m_mdc_procs = HashMap::new();
    for (m_mdc, proc) in max_procs.iter().filter_map(|p| p.m_mdc.as_ref().map(|mdc| (mdc, &p.proc))) {
        m_mdc_procs.entry(m_mdc.to_owned()).or_insert(HashSet::new()).insert(proc);
    }

    let mut o_mdc_procs = HashMap::new();
    for (o_mdcs, proc) in max_procs
        .iter()
        .map(|p| (p.o_mdcs.as_ref().map(|os| os.split(',').collect::<Vec<&str>>()).unwrap_or_default(), &p.proc))
    {
        for o_mdc in o_mdcs {
            o_mdc_procs.entry(o_mdc.to_owned()).or_insert(HashSet::new()).insert(proc);
        }
    }

    let mut mdc_sdxs = HashMap::new();
    for (mdc, sdx) in sdxs.iter().filter_map(|sdx| grouper.i10(sdx, gender).map(|i| (&i.mdc, sdx))) {
        mdc_sdxs.entry(mdc).or_insert(HashSet::new()).insert(sdx);
    }

    let m_mdc_procs_keys = m_mdc_procs.keys().collect::<HashSet<&String>>();
    let o_mdc_procs_keys = o_mdc_procs.keys().collect::<HashSet<&String>>();
    let mdc_procs_keys = m_mdc_procs_keys.union(&o_mdc_procs_keys).collect::<HashSet<&&String>>();
    let mdc_sdxs_keys = mdc_sdxs.keys().collect::<HashSet<&&String>>();
    let mdc_in_both = mdc_procs_keys.intersection(&mdc_sdxs_keys).collect::<Vec<&&&String>>();
    match mdc_in_both.len() {
        0 => process_uorp_3rd(mdc, max_procs, los),
        1 => {
            let new_mdc = mdc_in_both[0];
            let mut sdxs = mdc_sdxs.get(*new_mdc).map(|ss| ss.into_iter().collect::<Vec<&&String>>()).unwrap_or_default();
            sdxs.sort();
            sdxs.last().and_then(|new_pdx| Mdc::new(new_mdc.to_owned()).map(|mdc| MdcResult::MdcNewPdx(mdc, new_pdx.to_string())))
        }
        _ => {
            let mdc_in_both_vec = mdc_in_both.iter().map(|s| ***s).collect::<HashSet<&String>>();
            let match_m_mdcs = m_mdc_procs_keys.intersection(&mdc_in_both_vec).collect::<Vec<&&String>>();
            match match_m_mdcs.len() {
                0 => {
                    let match_o_mdcs = o_mdc_procs_keys.intersection(&mdc_in_both_vec).collect::<Vec<&&String>>();
                    match match_o_mdcs.len() {
                        0 => process_uorp_3rd(mdc, max_procs, los),
                        1 => {
                            let new_mdc = match_o_mdcs[0];
                            let mut sdxs = mdc_sdxs.get(*new_mdc).map(|ss| ss.into_iter().collect::<Vec<&&String>>()).unwrap_or_default();
                            sdxs.sort();
                            sdxs.last().and_then(|new_pdx| Mdc::new(new_mdc.to_owned()).map(|mdc| MdcResult::MdcNewPdx(mdc, new_pdx.to_string())))
                        }
                        _ => {
                            let mut ps = Vec::new();
                            for mdc in match_o_mdcs {
                                if let Some(mps) = o_mdc_procs.get(*mdc) {
                                    for p in mps.iter() {
                                        ps.push((p, mdc));
                                    }
                                }
                            }
                            ps.sort_by(|(a1, _), (a2, _)| a2.cmp(a1));
                            let (_, new_mdc) = ps[0];
                            let mut sdxs = mdc_sdxs.get(new_mdc).map(|ss| ss.into_iter().collect::<Vec<&&String>>()).unwrap_or_default();
                            sdxs.sort();
                            sdxs.last().and_then(|new_pdx| Mdc::new(new_mdc).map(|mdc| MdcResult::MdcNewPdx(mdc, new_pdx.to_string())))
                        }
                    }
                }
                1 => {
                    let new_mdc = match_m_mdcs[0];
                    let mut sdxs = mdc_sdxs.get(*new_mdc).map(|ss| ss.into_iter().collect::<Vec<&&String>>()).unwrap_or_default();
                    sdxs.sort();
                    sdxs.last().and_then(|new_pdx| Mdc::new(new_mdc.to_owned()).map(|mdc| MdcResult::MdcNewPdx(mdc, new_pdx.to_string())))
                }
                _ => {
                    let mut ps = Vec::new();
                    for mdc in match_m_mdcs {
                        if let Some(mps) = m_mdc_procs.get(*mdc) {
                            for p in mps.iter() {
                                ps.push((p, mdc));
                            }
                        }
                    }
                    ps.sort_by(|(a1, _), (a2, _)| a2.cmp(a1));
                    let (_, new_mdc) = ps[0];
                    let mut sdxs = mdc_sdxs.get(new_mdc).map(|ss| ss.into_iter().collect::<Vec<&&String>>()).unwrap_or_default();
                    sdxs.sort();
                    sdxs.last().and_then(|new_pdx| Mdc::new(new_mdc).map(|mdc| MdcResult::MdcNewPdx(mdc, new_pdx.to_string())))
                }
            }
        }
    }
}

// #3 DC from MosProc
fn process_uorp_3rd(mdc: Mdc, max_procs: &[&Arc<Proc>], los: u32) -> Option<MdcResult> {
    let mut tuples = max_procs
        .iter()
        .filter_map(|proc| proc.mos_dc.as_ref().map(|dc| (dc, proc.mos_hierar.unwrap_or_default())))
        .collect::<Vec<(&String, u8)>>();
    tuples.sort_by(|(_, a), (_, b)| a.cmp(&b));
    if let Some((dc, _)) = tuples.first() {
        Some(MdcResult::Dc(dc.to_string()))
    } else {
        match mdc {
            Mdc::M18 => process_uorp_4th_m18(max_procs, los),
            Mdc::M23 => process_uorp_4th_m23(max_procs, los),
            Mdc::M25 => process_uorp_4th_m25(max_procs, los),
            _ => process_uorp_4th(max_procs, los),
        }
    }
}

// #4 DC from ORP Group
fn process_uorp_4th(max_procs: &[&Arc<Proc>], los: u32) -> Option<MdcResult> {
    match max_procs.first().map(|p| p.proc_cgr).unwrap_or_default() {
        0 => None,
        1 => {
            if los < 4 {
                Some(MdcResult::Dc(String::from("2601")))
            } else {
                None
            }
        }
        2 => {
            if los < 5 {
                Some(MdcResult::Dc(String::from("2602")))
            } else {
                None
            }
        }
        3 => Some(MdcResult::Dc(String::from("2603"))),
        4 => Some(MdcResult::Dc(String::from("2604"))),
        _ => Some(MdcResult::Dc(String::from("2605"))),
    }
}

fn process_uorp_4th_m18(max_procs: &[&Arc<Proc>], los: u32) -> Option<MdcResult> {
    match max_procs.first().map(|p| p.proc_cgr).unwrap_or_default() {
        0 => None,
        1 => {
            if los < 4 {
                Some(MdcResult::Dc(String::from("1801")))
            } else {
                None
            }
        }
        2 => {
            if los < 5 {
                Some(MdcResult::Dc(String::from("1802")))
            } else {
                None
            }
        }
        3 => Some(MdcResult::Dc(String::from("1803"))),
        4 => Some(MdcResult::Dc(String::from("1804"))),
        _ => Some(MdcResult::Dc(String::from("1805"))),
    }
}

fn process_uorp_4th_m23(max_procs: &[&Arc<Proc>], los: u32) -> Option<MdcResult> {
    match max_procs.first().map(|p| p.proc_cgr).unwrap_or_default() {
        0 => None,
        1 => {
            if los < 4 {
                Some(MdcResult::Dc(String::from("2304")))
            } else {
                None
            }
        }
        2 => {
            if los < 5 {
                Some(MdcResult::Dc(String::from("2305")))
            } else {
                None
            }
        }
        3 => Some(MdcResult::Dc(String::from("2306"))),
        _ => Some(MdcResult::Dc(String::from("2307"))),
    }
}

fn process_uorp_4th_m25(max_procs: &[&Arc<Proc>], los: u32) -> Option<MdcResult> {
    match max_procs.first().map(|p| p.proc_cgr).unwrap_or_default() {
        0 => None,
        1 => {
            if los < 4 {
                Some(MdcResult::Dc(String::from("2501")))
            } else {
                None
            }
        }
        2 => {
            if los < 5 {
                Some(MdcResult::Dc(String::from("2502")))
            } else {
                None
            }
        }
        3 => Some(MdcResult::Dc(String::from("2503"))),
        _ => Some(MdcResult::Dc(String::from("2504"))),
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    // proc gr lev ..
    // 0052 3 35 C 05
    // 0053 3 39 C 05
    // 0054 3 38 C 05
    // 0055 0 0 - /05
    // 0056 0 0 - 05#PCom
    #[test]
    fn test_process_uorp_1st() {
        // C498 is MosDx
        let procs = &["0056", "0052", "0053", "0054", "0055"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
        let res = process_uorp_1st(Mdc::M01, &GROUPER, "C498", &HashSet::new(), &procs, &None, 5);
        assert_eq!(res, Some(MdcResult::Mdc(Mdc::M05)));
    }

    // proc gr lev ..
    // 0052 3 35 C 05
    // 3802 5 54 - 01:05,10,21/15
    // 3806 5 57 - 05:06,11
    // 3812 5 58 - 01:05
    // code mdc dc
    // D151,05,5R,เท็จ,,0,,,N,B,Y,N,0,124,0,0
    // E105,05,5F,จริง,E105,99,E105,,N,B,Y,N,0,124,0,0
    // I010,05,5R,จริง,I010,99,I010,,N,B,Y,N,0,124,0,0
    // I120,11,11A,จริง,I120,99,I120,,N,B,Y,N,0,124,0,0
    #[test]
    fn test_process_uorp_2nd() {
        let sdxs = ["D151", "E105", "I010", "I120"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
        let procs = ["0052", "3802", "3806", "3812"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>();
        let res = process_uorp_2nd(Mdc::M01, &GROUPER, &sdxs, &procs, &None, 5);
        assert_eq!(res, Some(MdcResult::MdcNewPdx(Mdc::M05, String::from("I010"))));

        // Book 1 page 128 example 1
        let ex1_sdxs = ["I10", "H259", "S7200", "E876", "K250"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
        let ex1_procs = ["7935", "4441"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>();
        let ex1_res = process_uorp_2nd(Mdc::M05, &GROUPER, &ex1_sdxs, &ex1_procs, &None, 5);
        assert_eq!(ex1_res, Some(MdcResult::MdcNewPdx(Mdc::M08, String::from("S7200"))));
    }

    // proc gr lev ..
    // 3800 3 32 - 05:21
    // 4319 3 39 D 06:03/15
    // 8682 3 39 - 09
    // MosProc
    // code dc hierar desc
    // 3800 2614 4 Incision of vessel NOS
    // 4319 2612 2 Other gastrostomy
    #[test]
    fn test_process_uorp_3rd() {
        let procs = ["4319", "8682", "3800"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>();
        let res = process_uorp_3rd(Mdc::M01, &procs, 5);
        assert_eq!(res, Some(MdcResult::Dc(String::from("2612"))));
    }

    // proc gr lev ..
    // 0055 0 0 - /05
    // 0443 1 16 - 01:08
    // 0444 2 30 - 01:08
    // 046 3 34 J 01/15
    // 0471 4 47 - 03:01
    // 0712 5 50 - 10
    // 0152 6 69 A 01/00
    #[test]
    fn test_process_uorp_4th() {
        let gr0_res = process_uorp_4th(&["0055"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr0_res, None);
        let gr1_res_los_below4 = process_uorp_4th(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr1_res_los_below4, Some(MdcResult::Dc(String::from("2601"))));
        let gr1_res_los_not_below4 = process_uorp_4th(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 4);
        assert_eq!(gr1_res_los_not_below4, None);
        let gr2_res_los_below5 = process_uorp_4th(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr2_res_los_below5, Some(MdcResult::Dc(String::from("2602"))));
        let gr2_res_los_not_below5 = process_uorp_4th(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 5);
        assert_eq!(gr2_res_los_not_below5, None);
        let gr3_res = process_uorp_4th(&["046"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr3_res, Some(MdcResult::Dc(String::from("2603"))));
        let gr4_res = process_uorp_4th(&["0471"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr4_res, Some(MdcResult::Dc(String::from("2604"))));
        let gr5_res = process_uorp_4th(&["0712"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr5_res, Some(MdcResult::Dc(String::from("2605"))));
        let gr6_res = process_uorp_4th(&["0152"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr6_res, Some(MdcResult::Dc(String::from("2605"))));
    }

    #[test]
    fn test_process_uorp_4th_m18() {
        let gr0_res = process_uorp_4th_m18(&["0055"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr0_res, None);
        let gr1_res_los_below4 = process_uorp_4th_m18(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr1_res_los_below4, Some(MdcResult::Dc(String::from("1801"))));
        let gr1_res_los_not_below4 = process_uorp_4th_m18(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 4);
        assert_eq!(gr1_res_los_not_below4, None);
        let gr2_res_los_below5 = process_uorp_4th_m18(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr2_res_los_below5, Some(MdcResult::Dc(String::from("1802"))));
        let gr2_res_los_not_below5 = process_uorp_4th_m18(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 5);
        assert_eq!(gr2_res_los_not_below5, None);
        let gr3_res = process_uorp_4th_m18(&["046"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr3_res, Some(MdcResult::Dc(String::from("1803"))));
        let gr4_res = process_uorp_4th_m18(&["0471"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr4_res, Some(MdcResult::Dc(String::from("1804"))));
        let gr5_res = process_uorp_4th_m18(&["0712"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr5_res, Some(MdcResult::Dc(String::from("1805"))));
        let gr6_res = process_uorp_4th_m18(&["0152"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr6_res, Some(MdcResult::Dc(String::from("1805"))));
    }

    #[test]
    fn test_process_uorp_4th_m23() {
        let gr0_res = process_uorp_4th_m23(&["0055"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr0_res, None);
        let gr1_res_los_below4 = process_uorp_4th_m23(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr1_res_los_below4, Some(MdcResult::Dc(String::from("2304"))));
        let gr1_res_los_not_below4 = process_uorp_4th_m23(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 4);
        assert_eq!(gr1_res_los_not_below4, None);
        let gr2_res_los_below5 = process_uorp_4th_m23(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr2_res_los_below5, Some(MdcResult::Dc(String::from("2305"))));
        let gr2_res_los_not_below5 = process_uorp_4th_m23(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 5);
        assert_eq!(gr2_res_los_not_below5, None);
        let gr3_res = process_uorp_4th_m23(&["046"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr3_res, Some(MdcResult::Dc(String::from("2306"))));
        let gr4_res = process_uorp_4th_m23(&["0471"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr4_res, Some(MdcResult::Dc(String::from("2307"))));
        let gr5_res = process_uorp_4th_m23(&["0712"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr5_res, Some(MdcResult::Dc(String::from("2307"))));
        let gr6_res = process_uorp_4th_m23(&["0152"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr6_res, Some(MdcResult::Dc(String::from("2307"))));
    }

    #[test]
    fn test_process_uorp_4th_m25() {
        let gr0_res = process_uorp_4th_m25(&["0055"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr0_res, None);
        let gr1_res_los_below4 = process_uorp_4th_m25(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr1_res_los_below4, Some(MdcResult::Dc(String::from("2501"))));
        let gr1_res_los_not_below4 = process_uorp_4th_m25(&["0443"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 4);
        assert_eq!(gr1_res_los_not_below4, None);
        let gr2_res_los_below5 = process_uorp_4th_m25(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr2_res_los_below5, Some(MdcResult::Dc(String::from("2502"))));
        let gr2_res_los_not_below5 = process_uorp_4th_m25(&["0444"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 5);
        assert_eq!(gr2_res_los_not_below5, None);
        let gr3_res = process_uorp_4th_m25(&["046"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr3_res, Some(MdcResult::Dc(String::from("2503"))));
        let gr4_res = process_uorp_4th_m25(&["0471"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr4_res, Some(MdcResult::Dc(String::from("2504"))));
        let gr5_res = process_uorp_4th_m25(&["0712"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr5_res, Some(MdcResult::Dc(String::from("2504"))));
        let gr6_res = process_uorp_4th_m25(&["0152"].iter().filter_map(|proc| GROUPER.proc(proc)).collect::<Vec<&Arc<Proc>>>(), 3);
        assert_eq!(gr6_res, Some(MdcResult::Dc(String::from("2504"))));
    }

    // Book 1 page 128 example 1
    #[test]
    fn test_uorp_example1() {
        let mut input = GrouperInput::default();
        input.set_age_y(75);
        input.set_los(35);
        input.set_pdx("I500");
        input.set_sdxs(&["I10", "H259", "S7200", "E876", "K250"]);
        input.set_procs(&["7935", "9346", "9904", "4513", "1341", "1371", "4441"]);
        let result = process_uorp(Mdc::M05, &GROUPER, &input);
        assert_eq!(result, Some(MdcResult::MdcNewPdx(Mdc::M08, String::from("S7200"))));
    }

    // Book 1 page 129 example 2
    #[test]
    fn test_uorp_example2() {
        // Book 1 page 128 example 1
        let mut input = GrouperInput::default();
        input.set_gender(Some(String::from("2")));
        input.set_age_y(19);
        input.set_los(3);
        input.set_pdx("O996");
        input.set_sdxs(&["K358"]);
        input.set_procs(&["4709"]);
        let result = process_uorp(Mdc::M14, &GROUPER, &input);
        assert_eq!(result, Some(MdcResult::MdcNewPdx(Mdc::M06, String::from("K358"))));
    }
}
