use kphis_drg_worker::{drg::model::I10vx, i10::index::I10Pointer};
use std::{collections::HashMap, sync::Arc};

use super::I10Keywords;

// I10vx::contains_search_best_n
pub(crate) fn search_i10vx_contains(text: &str, list: &HashMap<String, Arc<I10vx>>, n: usize) -> Vec<((String, Arc<I10vx>), f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let keywords = text_lo
        .split(" ")
        .filter_map(|s| {
            let exact = s.trim();
            (!exact.is_empty()).then(|| exact)
        })
        .collect::<Vec<&str>>();
    let keywords_len = keywords.len() as f32;
    let mut res = list
        .iter()
        .filter_map(|(code, vx)| {
            let code_lo = code.to_ascii_lowercase();
            let detail_lo = vx.desc.to_ascii_lowercase();
            let code_len = code_lo.len() as f32;
            let detail_len = detail_lo.len() as f32;
            let mut res = 0.0;
            let mut col = 0;
            for keyword in keywords.iter() {
                let keyword_len = keyword.len() as f32;
                if keywords_len == 1.0 && code_len > 0.0 && code_lo.contains(keyword) {
                    res = keyword_len / code_len;
                    col = 1;
                }
                if detail_len > 0.0 && res < 1.0 && detail_lo.contains(keyword) {
                    res += (1.0 / keywords_len) + (0.2 * keyword_len / detail_len);
                    col = 2;
                }
            }
            (res > 0.0).then(|| ((code, vx), res, col))
        })
        .collect::<Vec<((&String, &Arc<I10vx>), f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
}

pub(crate) fn search_i10tm_contains(text: &str, list: &HashMap<String, String>, n: usize) -> Vec<((String, String), f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let keywords = text_lo
        .split(" ")
        .filter_map(|s| {
            let exact = s.trim();
            (!exact.is_empty()).then(|| exact)
        })
        .collect::<Vec<&str>>();
    let keywords_len = keywords.len() as f32;
    let mut res = list
        .iter()
        .filter_map(|(code, detail)| {
            let code_lo = code.to_ascii_lowercase();
            let detail_lo = detail.to_ascii_lowercase();
            let code_len = code_lo.len() as f32;
            let detail_len = detail_lo.len() as f32;
            let mut res = 0.0;
            let mut col = 0;
            for keyword in keywords.iter() {
                let keyword_len = keyword.len() as f32;
                if keywords_len == 1.0 && code_len > 0.0 && code_lo.contains(keyword) {
                    res = keyword_len / code_len;
                    col = 1;
                }
                if detail_len > 0.0 && res < 1.0 && detail_lo.contains(keyword) {
                    res += (1.0 / keywords_len) + (0.2 * keyword_len / detail_len);
                    col = 2;
                }
            }
            (res > 0.0).then(|| ((code, detail), res, col))
        })
        .collect::<Vec<((&String, &String), f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
}

pub(crate) fn search_i10who_contains(text: &str, list: &[Arc<I10Keywords>], n: usize) -> Vec<(Arc<I10Keywords>, f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let keywords = text_lo
        .split(" ")
        .filter_map(|s| {
            let exact = s.trim();
            (!exact.is_empty()).then(|| exact)
        })
        .collect::<Vec<&str>>();
    let keywords_len = keywords.len() as f32;
    let mut res = list
        .iter()
        .filter_map(|i10| {
            let code_lo = i10.class_code.to_ascii_lowercase();
            let detail_lo = i10.detail.to_ascii_lowercase();
            let code_len = code_lo.len() as f32;
            let detail_len = detail_lo.len() as f32;
            let mut res = 0.0;
            let mut col = 0;
            for keyword in keywords.iter() {
                let keyword_len = keyword.len() as f32;
                if keywords_len == 1.0 && code_len > 0.0 && code_lo.contains(keyword) {
                    res = keyword_len / code_len;
                    col = 1;
                }
                if detail_len > 0.0 && res < 1.0 && detail_lo.contains(keyword) {
                    res += (1.0 / keywords_len) + (0.2 * keyword_len / detail_len);
                    col = 2;
                }
            }
            (res > 0.0).then(|| (i10, res, col))
        })
        .collect::<Vec<(&Arc<I10Keywords>, f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|(k, x, y)| (k.clone(), x, y)).collect()
}

// I10Pointer::contains_search_best_n()
pub(crate) fn search_i10_index_contains(text: &str, list: &HashMap<String, Arc<I10Pointer>>, n: usize) -> Vec<((String, Arc<I10Pointer>), f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let keywords = text_lo
        .split(" ")
        .filter_map(|s| {
            let exact = s.trim();
            (!exact.is_empty()).then(|| exact)
        })
        .collect::<Vec<&str>>();
    let keywords_len = keywords.len() as f32;
    let mut res = list
        .iter()
        .filter_map(|(detail, pointer)| {
            let detail_lo = detail.to_ascii_lowercase();
            let detail_len = detail_lo.len() as f32;
            let mut res = 0.0;
            let mut col = 0;
            for keyword in keywords.iter() {
                let keyword_len = keyword.len() as f32;
                if detail_len > 0.0 && detail_lo.contains(keyword) {
                    res += (1.0 / keywords_len) + (0.2 * keyword_len / detail_len);
                    col = 1;
                }
            }
            (res > 0.0).then(|| ((detail, pointer), res, col))
        })
        .collect::<Vec<((&String, &Arc<I10Pointer>), f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
}

// I10Pointer::contains_search_best_n()
pub(crate) fn search_i10_index_contains_v2(text: &str, list: &HashMap<String, Arc<I10Pointer>>, n: usize) -> Vec<((String, Arc<I10Pointer>), f32, u8)> {
    let text_lo = text.replace(['.', ':'], "").to_ascii_lowercase();
    let keywords = text_lo
        .split(" ")
        .filter_map(|s| {
            let exact = s.trim();
            (!exact.is_empty()).then(|| exact)
        })
        .collect::<Vec<&str>>();
    let keywords_len = keywords.len() as f32;
    let mut res = list
        .iter()
        .filter_map(|(detail, pointer)| {
            let detail_lo = detail.to_ascii_lowercase();
            let (subs, mains): (Vec<&str>, Vec<&str>) = detail_lo.split(", ").partition(|s| s.starts_with('('));
            let main = mains.concat();
            let sub = subs.concat();
            let main_len = mains.len() as f32;
            let sub_len = subs.len() as f32;
            let mut res = 0.0;
            let mut col = 0;
            for keyword in keywords.iter() {
                if main_len > 0.0 && main.contains(keyword) {
                    res += (1.0 / keywords_len) + (0.2 / main_len);
                    col = 1;
                }
                if sub_len > 0.0 && sub.contains(keyword) {
                    res += (0.5 / keywords_len) + (0.1 / sub_len);
                    col = 1;
                }
            }
            (res > 0.0).then(|| ((detail, pointer), res, col))
        })
        .collect::<Vec<((&String, &Arc<I10Pointer>), f32, u8)>>();
    res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    res.into_iter().take(n).map(|((a, b), x, y)| ((a.to_owned(), b.to_owned()), x, y)).collect()
}
