pub mod dx;
pub mod hosp;
pub mod ivfluid;
pub mod lab;
pub mod med;
pub mod opd_visit;
pub mod patient;
pub mod proc;
pub mod xray;

use dominator::{Dom, html, text};
use kphis_util::util::{icd9_dot, icd10_dot};

const BOX_HEIGHT: f64 = 300.0;

pub fn dec_to_color(dec: i32) -> String {
    if dec < 256 {
        format!("#{:0>2X}0000", dec)
    } else if dec < 256 * 256 {
        let g = dec / 256;
        let r = dec % 256;
        format!("#{:0>2X}{:0>2X}00", r, g)
    } else if dec < 256 * 256 * 256 {
        let b = dec / (256 * 256);
        let gr = dec % (256 * 256);
        let g = gr / 256;
        let r = gr % 256;
        format!("#{:0>2X}{:0>2X}{:0>2X}", r, g, b)
    } else {
        String::from("#000000")
    }
}

// fn red_chars_in_words(chars: &str, words: &str) -> impl Iterator<Item = Dom> {
//     let mut cs = chars.chars().map(|c| c.to_ascii_lowercase()).collect::<Vec<char>>();
//     cs.dedup();
//     words.chars().map(move |c| {
//         let mut buf = [0u8; 4];
//         let s = c.encode_utf8(&mut buf);
//         if cs.contains(&c.to_ascii_lowercase()) {
//             html!("span", {.style("color","red").text(s)})
//         } else {
//             text(s)
//         }
//     })
// }

pub fn red_keywords_in_sentense(keywords: &str, sentense: &str) -> Vec<Dom> {
    if keywords.is_empty() {
        return vec![text(sentense)];
    }
    let keywords_lo = keywords.to_ascii_lowercase();
    let k_words_lo = keywords_lo.split(' ').collect::<Vec<&str>>();
    let s_words = sentense.split(' ').collect::<Vec<&str>>();
    let sentense_lo = sentense.to_ascii_lowercase();
    if !k_words_lo.iter().any(|k| sentense_lo.contains(k)) {
        return vec![text(sentense)];
    }
    let mut results = Vec::new();
    let s_len = s_words.len();
    for (n, s_word) in s_words.iter().enumerate() {
        let s_word_lo = s_word.to_ascii_lowercase();
        let mut founds = Vec::new();
        for k_word_lo in k_words_lo.iter() {
            if !k_word_lo.is_empty() {
                if let Some(pos) = s_word_lo.find(k_word_lo) {
                    founds.push((pos, pos + k_word_lo.len() - 1));
                }
            }
        }
        let mut is_red = false;
        let mut red = String::new();
        let mut normal = String::new();
        let mut pos = 0;
        for c in s_word.chars() {
            let is_match = founds.iter().any(|(s, e)| pos >= *s && pos <= *e);
            if is_match {
                red.push(c);
            } else {
                normal.push(c);
            }
            if is_red && !is_match && !red.is_empty() {
                results.push(html!("span", {.style("color","red").text(&red)}));
                red.clear();
            } else if !is_red && is_match && !normal.is_empty() {
                results.push(text(&normal));
                normal.clear();
            }
            is_red = is_match;
            pos += c.len_utf8();
        }
        if is_red && !red.is_empty() {
            results.push(html!("span", {.style("color","red").text(&red)}));
        } else if !is_red && !normal.is_empty() {
            results.push(text(&normal));
        }
        if n < (s_len - 1) {
            results.push(text(" "));
        }
    }

    results
}

pub fn red_keywords_in_icd_dot(is_icd9: bool, keyword: &str, icd10_with_dot: &str) -> Vec<Dom> {
    if keyword.is_empty() {
        return vec![text(icd10_with_dot)];
    }
    let keyword_lo = if is_icd9 {
        icd9_dot(keyword).to_ascii_lowercase()
    } else {
        icd10_dot(keyword).to_ascii_lowercase()
    };
    let s_word_lo = icd10_with_dot.to_ascii_lowercase();
    let mut results = Vec::new();
    if let Some(s) = s_word_lo.find(&keyword_lo) {
        let e = s + keyword_lo.len() - 1;
        let mut is_red = false;
        let mut red = String::new();
        let mut normal = String::new();
        for (i, c) in icd10_with_dot.chars().enumerate() {
            let is_match = i >= s && i <= e;
            if is_match {
                red.push(c);
            } else {
                normal.push(c);
            }
            if is_red && !is_match && !red.is_empty() {
                results.push(html!("span", {.style("color","red").text(&red)}));
                red.clear();
            } else if !is_red && is_match && !normal.is_empty() {
                results.push(text(&normal));
                normal.clear();
            }
            is_red = is_match;
        }
        if is_red && !red.is_empty() {
            results.push(html!("span", {.style("color","red").text(&red)}));
        } else if !is_red && !normal.is_empty() {
            results.push(text(&normal));
        }
    } else {
        results.push(text(icd10_with_dot));
    }

    results
}
