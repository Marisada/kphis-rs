// Download PDF file from https://www.tcmc.or.th/_content_images/download/fileupload/S0034.pdf
// copy text out of PDF file to `/raw-icd9cm/book.txt`
// This book consists of 2 parts
// - Tabular list
// - Index (start after `Index to Procedures` line)

use std::{
    collections::{BTreeMap, HashSet},
    sync::Arc,
};

use kphis_drg_worker::drg::model::I9vx;

const PAT_38: [(u8, &str); 10] = [
    (0, "unspecified site"),
    (1, "intracranial vessels"),
    (2, "other vessels of head and neck"),
    (3, "upper limb vessels"),
    (4, "aorta"),
    (5, "other thoracic vessels"),
    (6, "abdominal arteries"),
    (7, "abdominal veins"),
    (8, "lower limb arteries"),
    (9, "lower limb veins"),
];

const PAT_77: [(u8, &str); 10] = [
    (0, "unspecified site"),
    (1, "scapula, clavicle and thorax [ribs and sternum]"),
    (2, "humerus"),
    (3, "radius and ulna"),
    (4, "carpals and metacarpals"),
    (5, "femur"),
    (6, "patella"),
    (7, "tibia and fibula"),
    (8, "tarsals and metatarsals"),
    (9, "other"),
];

const PAT_79: [(u8, &str); 10] = [
    (0, "unspecified site"),
    (1, "humerus"),
    (2, "radius and ulna"),
    (3, "carpals and metacarpals"),
    (4, "phalanges of hand"),
    (5, "femur"),
    (6, "tibia and fibula"),
    (7, "tarsals and metatarsals"),
    (8, "phalanges of foot"),
    (9, "other specified bone"),
];

const PAT_80: [(u8, &str); 10] = [
    (0, "unspecified site"),
    (1, "shoulder"),
    (2, "elbow"),
    (3, "wrist"),
    (4, "hand and finger"),
    (5, "hip"),
    (6, "knee"),
    (7, "ankle"),
    (8, "foot and toe"),
    (9, "other specified sites"),
];

const PAT_90: [(u8, &str); 7] = [
    (1, "bacterial smear"),
    (2, "culture"),
    (3, "culture and sensitivity"),
    (4, "parasitology"),
    (5, "toxicology"),
    (6, "cell block and Papanicolaou smear"),
    (9, "other microscopic examination"),
];

/// new from book, add is_valid from DRG grouper's i9vx
pub fn fixed_i9vx_desc_with_book(i9vx: &mut BTreeMap<String, Arc<I9vx>>) {
    let book = parse_i9cm_basic();
    let i9vx_keys = i9vx.keys().cloned().collect::<HashSet<String>>();
    let book_keys = book.keys().cloned().collect::<HashSet<String>>();

    // fixed desc with book data
    for (key, value) in i9vx.iter_mut() {
        if let Some(desc) = book.get(key) {
            *value = Arc::new(I9vx {
                code: value.code.clone(),
                is_valid: value.is_valid,
                desc: desc.to_owned(),
            });
        } else {
            panic!("{} not found in book", key);
        }
    }

    // Add missed item to i9vx
    let not_in_i9vx = book_keys.difference(&i9vx_keys).collect::<Vec<&String>>();
    for key in not_in_i9vx {
        if let Some(desc) = book.get(key) {
            i9vx.insert(
                key.to_owned(),
                Arc::new(I9vx {
                    code: key.to_owned(),
                    is_valid: false,
                    desc: desc.to_owned(),
                }),
            );
        } else {
            panic!("{} not found in book", key);
        }
    }
}

// parse only `code` and `desc` for compared with `i9vx` of DRG grouper
fn parse_i9cm_basic() -> BTreeMap<String, String> {
    // PARSE FROM BOOK
    let raw = include_str!("../raw-icd9cm/book.txt");
    let mut previous_is_code = false;
    let mut start_index_section = false;
    let mut init_results: Vec<(String, String)> = Vec::new();
    for line in raw.lines() {
        if line == "Index to Procedures" {
            start_index_section = true;
        }
        // Tabular section
        if !start_index_section {
            if is_start_with_lowercase(line) {
                if previous_is_code {
                    if let Some((_, last_desc)) = init_results.last_mut() {
                        last_desc.push(' ');
                        last_desc.push_str(&remove_tag(line));
                    }
                    // previous_is_code = false;
                }
            } else {
                let (maybe_code, rest) = split_start_with_icd9cm_dot(line);
                previous_is_code = maybe_code.is_some();
                if let Some(code) = maybe_code {
                    init_results.push((code.replace('.', ""), remove_tag(&rest)));
                }
            }
        }
        // Index section
    }

    // ADD 4th in some code
    let mut results = init_results.into_iter().collect::<BTreeMap<String, String>>();

    add_4th_items(&mut results, "380", AddPattern::All, &PAT_38);
    add_4th_items(&mut results, "381", AddPattern::Zero68, &PAT_38);
    add_4th_items(&mut results, "383", AddPattern::All, &PAT_38);
    add_4th_items(&mut results, "384", AddPattern::All, &PAT_38);
    add_4th_items(&mut results, "385", AddPattern::Zero3579, &PAT_38);
    add_4th_items(&mut results, "386", AddPattern::All, &PAT_38);
    add_4th_items(&mut results, "388", AddPattern::All, &PAT_38);

    add_4th_items(&mut results, "770", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "771", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "772", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "773", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "774", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "776", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "777", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "778", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "779", AddPattern::All, &PAT_77);

    add_4th_items(&mut results, "780", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "781", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "782", AddPattern::Zero2579, &PAT_77);
    add_4th_items(&mut results, "783", AddPattern::Zero2579, &PAT_77);
    add_4th_items(&mut results, "784", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "785", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "786", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "787", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "788", AddPattern::All, &PAT_77);
    add_4th_items(&mut results, "789", AddPattern::All, &PAT_77);

    add_4th_items(&mut results, "790", AddPattern::All, &PAT_79);
    add_4th_items(&mut results, "791", AddPattern::All, &PAT_79);
    add_4th_items(&mut results, "792", AddPattern::All, &PAT_79);
    add_4th_items(&mut results, "793", AddPattern::All, &PAT_79);
    add_4th_items(&mut results, "794", AddPattern::Zero2569, &PAT_79);
    add_4th_items(&mut results, "795", AddPattern::Zero2569, &PAT_79);
    add_4th_items(&mut results, "796", AddPattern::All, &PAT_79);
    add_4th_items(&mut results, "799", AddPattern::All, &PAT_79);

    add_4th_items(&mut results, "800", AddPattern::All, &PAT_80);
    add_4th_items(&mut results, "801", AddPattern::All, &PAT_80);
    add_4th_items(&mut results, "802", AddPattern::All, &PAT_80);
    add_4th_items(&mut results, "803", AddPattern::All, &PAT_80);
    add_4th_items(&mut results, "804", AddPattern::All, &PAT_80);
    add_4th_items(&mut results, "807", AddPattern::All, &PAT_80);
    add_4th_items(&mut results, "808", AddPattern::All, &PAT_80);
    add_4th_items(&mut results, "809", AddPattern::All, &PAT_80);

    add_4th_items(&mut results, "900", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "901", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "902", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "903", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "904", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "905", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "906", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "907", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "908", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "909", AddPattern::All, &PAT_90);

    add_4th_items(&mut results, "910", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "911", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "912", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "913", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "914", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "915", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "916", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "917", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "918", AddPattern::All, &PAT_90);
    add_4th_items(&mut results, "919", AddPattern::All, &PAT_90);

    results
}

fn add_4th_items(data: &mut BTreeMap<String, String>, code: &str, pat: AddPattern, items: &[(u8, &str)]) {
    let clone = data.get(code).cloned();
    if let Some(left) = clone {
        match pat {
            AddPattern::All => {
                for (tail, item) in items {
                    data.insert([code, &tail.to_string()].concat(), [&left, ", ", item].concat());
                }
            }
            AddPattern::Zero2569 => {
                for (tail, item) in items {
                    if [0, 1, 2, 5, 6, 9].contains(tail) {
                        data.insert([code, &tail.to_string()].concat(), [&left, ", ", item].concat());
                    }
                }
            }
            AddPattern::Zero2579 => {
                for (tail, item) in items {
                    if [0, 2, 3, 4, 5, 7, 8, 9].contains(tail) {
                        data.insert([code, &tail.to_string()].concat(), [&left, ", ", item].concat());
                    }
                }
            }
            AddPattern::Zero3579 => {
                for (tail, item) in items {
                    if [0, 1, 2, 3, 5, 7, 9].contains(tail) {
                        data.insert([code, &tail.to_string()].concat(), [&left, ", ", item].concat());
                    }
                }
            }
            AddPattern::Zero68 => {
                for (tail, item) in items {
                    if [0, 1, 2, 3, 4, 5, 6, 8].contains(tail) {
                        data.insert([code, &tail.to_string()].concat(), [&left, ", ", item].concat());
                    }
                }
            }
        }
    }
}

enum AddPattern {
    All,
    /// [0-2,5,6,9]
    Zero2569,
    /// [0,2-5,7-9]
    Zero2579,
    /// [0-3,5,7,9]
    Zero3579,
    /// [0-6,8]
    Zero68,
}

// `` = new
// `` = 4th digit required
// `®` = revised
fn remove_tag(text: &str) -> String {
    // '', '', '®'
    text.replace(['', '', '®'], "").trim().to_owned()
}

fn split_start_with_icd9cm_dot(line: &str) -> (Option<String>, String) {
    let split = line.split(' ').collect::<Vec<&str>>();
    if split.len() > 1 {
        if is_icd9_with_dot(split[0]) {
            (Some(split[0].to_owned()), split[1..].join(" "))
        } else {
            (None, line.to_owned())
        }
    } else {
        (None, line.to_owned())
    }
}

fn is_start_with_lowercase(line: &str) -> bool {
    line.chars().next().map(|c| c.is_ascii_lowercase()).unwrap_or_default()
}

fn is_icd9_with_dot(text: &str) -> bool {
    let chars = text.chars().collect::<Vec<char>>();
    match chars.len() {
        4 => {
            let first_is_digit = chars.get(0).map(|c| c.is_ascii_digit()).unwrap_or_default();
            let second_is_digit = chars.get(1).map(|c| c.is_ascii_digit()).unwrap_or_default();
            let third_is_dot = chars.get(2).map(|c| *c == '.').unwrap_or_default();
            let fourth_is_digit = chars.get(3).map(|c| c.is_ascii_digit()).unwrap_or_default();
            first_is_digit && second_is_digit && third_is_dot && fourth_is_digit
        }
        5 => {
            let first_is_digit = chars.get(0).map(|c| c.is_ascii_digit()).unwrap_or_default();
            let second_is_digit = chars.get(1).map(|c| c.is_ascii_digit()).unwrap_or_default();
            let third_is_dot = chars.get(2).map(|c| *c == '.').unwrap_or_default();
            let fourth_is_digit = chars.get(3).map(|c| c.is_ascii_digit()).unwrap_or_default();
            let fifth_is_digit = chars.get(4).map(|c| c.is_ascii_digit()).unwrap_or_default();
            first_is_digit && second_is_digit && third_is_dot && fourth_is_digit && fifth_is_digit
        }
        _ => false,
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use std::{collections::HashSet, env::current_dir, fs, io::{BufWriter, Write}};
    use super::*;
    
    fn write_icd9cm(data: &BTreeMap<String, String>) {
        let current = current_dir().unwrap().join("/debug/icd9cm");
        if !fs::exists(&current).unwrap() {
            fs::create_dir_all(&current).unwrap();
        }
        let file = fs::File::create(current.join("i9vx.txt")).unwrap();
        let mut writer = BufWriter::new(file);
        for (code, desc) in data {
            writeln!(writer, "{},\"{}\"", code, desc).unwrap();
        }
    }

    fn get_i9vx_from_raw() -> BTreeMap<String, Arc<I9vx>> {
        csv::Reader::from_reader(&include_bytes!("../raw-grouper/i9vx.csv")[..])
            .deserialize::<I9vx>()
            .map(|r| {
                let vx = r.expect("invalid /raw-grouper/proc.csv, please run raw_parser and try again");
                (vx.code.to_owned(), Arc::new(vx))
            })
            .collect::<BTreeMap<String, Arc<I9vx>>>()
    }

    fn compare_i9vx_with_book(i9vx: &BTreeMap<String, Arc<I9vx>>, book: &BTreeMap<String, String>, expect: (usize, usize), verbose: bool) {
        
        let i9vx_keys = i9vx.keys().cloned().collect::<HashSet<String>>();
        let book_keys = book.keys().cloned().collect::<HashSet<String>>();

        let mut not_in_i9vx = book_keys.difference(&i9vx_keys).collect::<Vec<&String>>();
        let mut not_in_book = i9vx_keys.difference(&book_keys).collect::<Vec<&String>>();
        assert_eq!(not_in_i9vx.len(), expect.0);
        assert_eq!(not_in_book.len(), expect.1);

        if verbose {
            not_in_i9vx.sort();
            not_in_book.sort();
            for item in not_in_i9vx.iter() {
                println!("{} not in i9vx", item);
            }
            println!("");
            for item in not_in_book.iter() {
                println!("{} not in book", item);
            }
        }
    }

    #[test]
    fn test_parse_i9vx_from_book() {
        let verbose = false;

        let book = parse_i9cm_basic();
        write_icd9cm(&book);

        let i9vx = get_i9vx_from_raw();
        compare_i9vx_with_book(&i9vx, &book, (10, 0), verbose);
    }

    #[test]
    fn test_fixed_i9vx_desc_with_book() {
        let verbose = false;

        let book = parse_i9cm_basic();
        let mut i9vx = get_i9vx_from_raw();
        fixed_i9vx_desc_with_book(&mut i9vx);
        compare_i9vx_with_book(&i9vx, &book, (0, 0), verbose);
    }
}
