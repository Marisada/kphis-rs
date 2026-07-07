#![allow(dead_code)]

use std::collections::HashSet;

use super::grouper::Grouper;

pub(crate) fn dcl_pcl(grouper: &Grouper, dc: &str, pdx: &str, sdxs: &HashSet<String>, gender: &Option<String>) -> u8 {
    let dcls = dcl_recursion(grouper, dc, pdx, sdxs, gender);
    pcl_formula(&dcls)
}

fn dcl_recursion(grouper: &Grouper, dc: &str, pdx: &str, sdxs: &HashSet<String>, gender: &Option<String>) -> Vec<u8> {
    // find dcl of pdx and sdxs
    let pdx_iter = [&pdx.to_owned()];
    let mut code_dcl = sdxs
        .iter()
        .chain(pdx_iter)
        .filter_map(|code| {
            let dcl = grouper.dcl(code, dc);
            (dcl > 0).then(|| (code, dcl))
        })
        .collect::<Vec<(&String, u8)>>();

    // sort (dcl then code) from MAX to MIN
    code_dcl.sort_by(|(code_1, dcl_1), (code_2, dcl_2)| if dcl_1 == dcl_2 { code_2.cmp(code_1) } else { dcl_2.cmp(dcl_1) });

    // recursion exclusion
    let end_pos = code_dcl.len().saturating_sub(1);
    let mut pos = 0;
    while pos < end_pos {
        let (head_code, head_dcl) = code_dcl[pos];
        if head_dcl > 0 {
            let (_, tail_with_zero) = code_dcl.split_at_mut(pos + 1);
            tail_with_zero
                .iter_mut()
                .filter_map(|(cur_code, cur_dcl)| (*cur_dcl > 0).then(|| (cur_code, cur_dcl)))
                .for_each(|(cur_code, cur_dcl)| {
                    if let Some(main_cc) = grouper.i10(cur_code, gender).and_then(|i10| i10.main_cc.as_ref()) {
                        if grouper.has_ccex(main_cc, head_code) {
                            *cur_dcl = 0;
                        }
                    }
                });
        }
        pos = pos + 1;
    }

    // remove dcl = 0
    code_dcl.iter().filter_map(|(_, dcl)| (*dcl > 0).then(|| *dcl)).collect::<Vec<u8>>()
}

fn pcl_formula(dcls: &[u8]) -> u8 {
    let not_round = dcls.iter().enumerate().fold(0.0f32, |acc, (i, dcl)| acc + (*dcl as f32 * (0.82f32.powi(i as i32))));
    let may_over_nine = not_round.round() as u8;
    if may_over_nine > 9 { 9 } else { may_over_nine }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_dcl_recursion() {
        // Book 1 page 170
        assert_eq!(
            dcl_recursion(
                &GROUPER,
                "0553",
                "I213",
                &["E119", "I10", "N182", "I092", "K250", "I209", "A419", "E875", "E876"]
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect::<HashSet<String>>(),
                &None,
            ),
            // as (4.) result in Book 1 page 171
            vec![3, 2, 2, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_dcl_recursion_debug() {
        assert_eq!(
            dcl_recursion(&GROUPER, "0459", "J205", &["J441"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &None,),
            vec![2]
        );
        assert_eq!(
            dcl_recursion(&GROUPER, "0351", "R42", &["E875", "I10", "E119"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), &None,),
            vec![2, 1, 1]
        );
    }

    #[test]
    fn test_pcl_formula() {
        // 2 + 0.82 + 0.6724 + 0.5514 + 0.4521 + 0.3707 + 0.3040 = 5.1703 round to 5
        assert_eq!(pcl_formula(&[2, 1, 1, 1, 1, 1, 1]), 5);
        // 3 + 1.64 + 1.3448 + 0.5513 + 0.4521 + 0.3707 + 0.3040 = 7.6630 round to 8
        assert_eq!(pcl_formula(&[3, 2, 2, 1, 1, 1, 1]), 8);
        // 3 + 2.46 + 2.0172 + 1.1027 + 0.9042 + 0.3707 + 0.3040 = 10.1589 limit to 9
        assert_eq!(pcl_formula(&[3, 3, 3, 2, 2, 1, 1]), 9);
        // 2 + 2.46 = 4.46 round to 4
        assert_eq!(pcl_formula(&[2, 2]), 4);
        // 2 + 0.82 + 0.6724 = 3.4924 round to 3
        assert_eq!(pcl_formula(&[2, 1, 1]), 3);
    }
}
