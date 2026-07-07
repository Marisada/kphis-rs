use std::{collections::HashMap, sync::Arc};

use super::I10Keywords;
use kphis_drg_worker::{drg::model::I10vx, i10::index::I10Pointer};
use kphis_util::fuzzy_search::{fuzzy_compare, trigrams};

// I10vx::fuzzy_search_best_n()
pub(crate) fn search_i10vx_fuzzy(text: &str, list: &HashMap<String, Arc<I10vx>>, n: usize) -> Vec<((String, Arc<I10vx>), f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let chars_count = text_lo.chars().count() + 1;
    let trigrams_a = trigrams(&text_lo);
    let mut res = list
        .iter()
        .filter_map(|(code, vx)| {
            let mut res = 0.0;
            let mut col = 0;
            if code.chars().count() > 0 {
                let r = fuzzy_compare(&trigrams_a, chars_count, code);
                if r > res {
                    res = r;
                    col = 1;
                }
            }
            if res < 1.0 {
                if vx.desc.chars().count() > 0 {
                    let r = fuzzy_compare(&trigrams_a, chars_count, &vx.desc);
                    if r > res {
                        res = r;
                        col = 2;
                    }
                }
            }
            (res > 0.0).then(|| ((code, vx), res, col))
        })
        .collect::<Vec<((&String, &Arc<I10vx>), f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
}

pub(crate) fn search_i10tm_fuzzy(text: &str, list: &HashMap<String, String>, n: usize) -> Vec<((String, String), f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let chars_count = text_lo.chars().count() + 1;
    let trigrams_a = trigrams(&text_lo);
    let mut res = list
        .iter()
        .filter_map(|(code, detail)| {
            let mut res = 0.0;
            let mut col = 0;
            if code.chars().count() > 0 {
                let r = fuzzy_compare(&trigrams_a, chars_count, code);
                if r > res {
                    res = r;
                    col = 1;
                }
            }
            if res < 1.0 {
                if detail.chars().count() > 0 {
                    let r = fuzzy_compare(&trigrams_a, chars_count, detail);
                    if r > res {
                        res = r;
                        col = 2;
                    }
                }
            }
            (res > 0.0).then(|| ((code, detail), res, col))
        })
        .collect::<Vec<((&String, &String), f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
}

pub(crate) fn search_i10who_fuzzy(text: &str, list: &[Arc<I10Keywords>], n: usize) -> Vec<(Arc<I10Keywords>, f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let chars_count = text_lo.chars().count() + 1;
    let trigrams_a = trigrams(&text_lo);
    let mut res = list
        .iter()
        .filter_map(|i10| {
            let mut res = 0.0;
            let mut col = 0;
            if i10.class_code.chars().count() > 0 {
                let r = fuzzy_compare(&trigrams_a, chars_count, &i10.class_code);
                if r > res {
                    res = r;
                    col = 1;
                }
            }
            if res < 1.0 {
                if i10.detail.chars().count() > 0 {
                    let r = fuzzy_compare(&trigrams_a, chars_count, &i10.detail);
                    if r > res {
                        res = r;
                        col = 2;
                    }
                }
            }
            (res > 0.0).then(|| (i10, res, col))
        })
        .collect::<Vec<(&Arc<I10Keywords>, f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|(k, x, y)| (k.clone(), x, y)).collect()
}

pub(crate) fn search_i10_index_fuzzy(text: &str, list: &HashMap<String, Arc<I10Pointer>>, n: usize) -> Vec<((String, Arc<I10Pointer>), f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let chars_count = text_lo.chars().count() + 1;
    let trigrams_a = trigrams(&text_lo);
    let mut res = list
        .iter()
        .filter_map(|(detail, pointer)| {
            let mut res = 0.0;
            let mut col = 0;
            if detail.chars().count() > 0 {
                let r = fuzzy_compare(&trigrams_a, chars_count, detail);
                if r > res {
                    res = r;
                    col = 1;
                }
            }
            (res > 0.0).then(|| ((detail, pointer), res, col))
        })
        .collect::<Vec<((&String, &Arc<I10Pointer>), f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
}
