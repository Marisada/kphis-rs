// Book 3
// Download PDF book from https://www.chi.or.th/Drg/ICD10_2016_WHO.html
// copy text out of PDF file to `/raw-icd-who/icd102016en-book3.txt`
// - Remove complicated data (Notes, table) at page(PDF) 24, 36, 38, 92, 99, 109, 129,
// 163, 170, 217, 304, 354, 364, 374, 375, 423, 443, 478, 479, 484, 504, 529, 611, 677
// - Remove 'Abortion' table at page(PDF) 23
// - Special parse of Section I 'Neoplasm' table (page(PDF) 437) and Section III
//
// Section I: ended with "Section II"
// Section II: ended with "Section III"
// Section III: ended at EOF

// NOTE:
// - remove 'Aerosinusitis W94' because 'Aerosinusitis T70.1' is more accurate

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use kphis_drg_worker::i10::index::{Code, I10Index, I10Pointer, NeoplasmTag, Note};
use kphis_util::util::is_icd10_resemble;

pub fn parse_i10_index() -> I10Index {
    let raw = include_str!("../raw-icd-who/icd102016en-book3-tm.txt");
    let codes = crate::drg_grouper::new_grouper().valid_codes();
    // Split Section I-III
    let (section_1, section_2, section_3) = split_section(raw);
    let mut diagnosis = HashMap::new();
    let mut external = HashMap::new();
    let mut substance = HashMap::new();
    let mut aster_dagger = crate::drg_grouper::get_grouper_aster_dagger();
    let claml_aster_dagger = crate::i10_claml::get_claml_asterisk_dagger();
    for (k, v) in claml_aster_dagger {
        aster_dagger.entry(k).and_modify(|hs| hs.extend(v.clone())).or_insert(v);
    }

    for item in section_1.iter() {
        for pat in parse_alphabet_to_pattern(&item, 1) {
            if let Some(Code::DaggerAster(dag, aster)) = &pat.code {
                // clean wildcard code e.g. `X00.-`
                match (dag.ends_with('-'), aster.ends_with('-')) {
                    (true, true) => {
                        panic!("Not support wildcard of both dagger and asterisk");
                    }
                    (true, false) => {
                        let dag_exact = dag.trim_end_matches(['-', '.']);
                        for d in codes.iter().filter(|s| s.starts_with(dag_exact)) {
                            // println!("{}, {}", d, aster);
                            aster_dagger.entry(aster.replace('.', "")).or_insert(HashSet::new()).insert(d.replace('.', ""));
                        }
                    }
                    (false, true) => {
                        let aster_exact = aster.trim_end_matches(['-', '.']);
                        for a in codes.iter().filter(|s| s.starts_with(aster_exact)) {
                            // println!("{}, {}", dag, a);
                            aster_dagger.entry(a.replace('.', "")).or_insert(HashSet::new()).insert(dag.replace('.', ""));
                        }
                    }
                    (false, false) => {
                        aster_dagger.entry(aster.replace('.', "")).or_insert(HashSet::new()).insert(dag.replace('.', ""));
                    }
                }
            }
            let point = Arc::new(I10Pointer {
                note: pat.note,
                bracket_notes: pat.bracket_notes,
                code: pat.code,
            });
            if let Some(old) = diagnosis.get(&pat.concat) {
                if *old != point {
                    panic!("Duplicate '{}', OLD: {:?} NEW: {:?}", &pat.concat, old, &point);
                }
            } else {
                diagnosis.insert(pat.concat, point);
            }
        }
    }
    for item in section_2.iter() {
        for pat in parse_alphabet_to_pattern(&item, 2) {
            let point = Arc::new(I10Pointer {
                note: pat.note,
                bracket_notes: pat.bracket_notes,
                code: pat.code,
            });
            if let Some(old) = external.get(&pat.concat) {
                if *old != point {
                    panic!("Duplicate '{}', OLD: {:?} NEW: {:?}", &pat.concat, old, &point);
                }
            } else {
                external.insert(pat.concat, point);
            }
        }
    }
    for item in section_3.iter() {
        for pat in parse_alphabet_to_pattern(&item, 3) {
            if let Some(Code::DaggerAster(dagger, aster)) = &pat.code {
                aster_dagger.entry(aster.to_owned()).or_insert(HashSet::new()).insert(dagger.to_owned());
            }
            let point = Arc::new(I10Pointer {
                note: pat.note,
                bracket_notes: pat.bracket_notes,
                code: pat.code,
            });
            if let Some(old) = substance.get(&pat.concat) {
                if *old != point {
                    panic!("Duplicate '{}', OLD: {:?} NEW: {:?}", &pat.concat, old, &point);
                }
            } else {
                substance.insert(pat.concat, point);
            }
        }
    }

    I10Index {
        diagnosis,
        external,
        substance,
        aster_dagger,
    }
}

const NEED_NEXT_WORD_ENDED: [&str; 15] = [
    " against", " also", " and", " as", " by", " for", " in", " of", " or", " see", " to", " type", " under", " with", " without",
];
const NEED_NEXT_WORD_EXACT: [&str; 15] = ["against", "also", "and", "as", "by", "for", "in", "of", "or", "see", "to", "type", "under", "with", "without"];
const DISCARD_PARENTHESES_WORD: [&str; 9] = ["(by)", "(for)", "(from)", "(in)", "(of)", "(to)", "(type)", "(with)", "(without)"];

/// Split raw book by line, discard unused line
/// - return (Section) of (A-Z) of (indent, text)
fn split_section(raw: &str) -> (Vec<Vec<(usize, String)>>, Vec<Vec<(usize, String)>>, Vec<Vec<(usize, String)>>) {
    let mut section_1 = Vec::new();
    let mut section_2 = Vec::new();
    let mut section_3 = Vec::new();

    let mut started = false;

    let mut current_char_vec: Vec<(usize, String)> = Vec::new();

    let mut current_section = Section::Undefined;
    let mut alphabet = '0';
    let mut is_start_new_alphabet = false;
    let mut next_alphabet = '0';
    let mut prev_need_next_word = false;

    let lines = raw.lines().collect::<Vec<&str>>();
    for line in lines.windows(2) {
        match LineType::new(
            line[0],
            line[1],
            &mut current_section,
            &mut alphabet,
            &mut is_start_new_alphabet,
            &mut next_alphabet,
            &prev_need_next_word,
        ) {
            LineType::Empty => {}
            LineType::PageLabel => {}
            LineType::PageNumber => {}
            // each appear only once at the start of section
            LineType::Section(section) => {
                // collect "Z" current_char_vec at the end of section I and II
                match section {
                    Section::II => {
                        section_1.push(current_char_vec.clone());
                        current_char_vec.clear();
                    }
                    Section::III => {
                        section_2.push(current_char_vec.clone());
                        current_char_vec.clear();
                    }
                    _ => {}
                }
                current_section = section;
            }
            LineType::Alphabet(c) => {
                if c != 'A' {
                    match current_section {
                        Section::I => {
                            section_1.push(current_char_vec.clone());
                            current_char_vec.clear();
                        }
                        Section::II => {
                            section_2.push(current_char_vec.clone());
                            current_char_vec.clear();
                        }
                        Section::III => {
                            section_3.push(current_char_vec.clone());
                            current_char_vec.clear();
                        }
                        Section::Undefined => {}
                    }
                } else {
                    started = true;
                }
            }
            LineType::Continued => {}
            LineType::Keyword(s) => {
                if started {
                    if prev_need_next_word {
                        if let Some((_, last)) = current_char_vec.last_mut() {
                            last.push(' ');
                            last.push_str(&s);
                        }
                    } else {
                        current_char_vec.push((0, s.to_owned()));
                    }
                }
            }
            LineType::Sub(i, s) => {
                if started {
                    current_char_vec.push((i, s.to_owned()));
                }
            }
            LineType::Concat(s) => {
                if started {
                    if let Some((_, last)) = current_char_vec.last_mut() {
                        last.push(' ');
                        last.push_str(&s);
                    }
                }
            }
        }
        prev_need_next_word = NEED_NEXT_WORD_ENDED.iter().any(|w| line[0].ends_with(w)) || NEED_NEXT_WORD_EXACT.iter().any(|w| line[0] == *w);
    }

    // End section 3 at EOF
    section_3.push(current_char_vec);

    (section_1, section_2, section_3)
}

// from alphabet's group of (indent, text) to patterned data
fn parse_alphabet_to_pattern(group: &[(usize, String)], section: usize) -> Vec<Patterned> {
    let mut results = Vec::with_capacity(group.len());
    let mut parents = Vec::new();
    for (indent, line) in group.iter() {
        if parents.len() < *indent {
            panic!("Line '{}' has {} indent but buffer has {} items", line, indent, parents.len());
        } else {
            let _ = parents.split_off(*indent);
            let result = Patterned::new(line, &parents, section == 3);
            parents.push(result.main.clone());
            if result.code.is_some() || result.note.is_some() || !result.bracket_notes.is_empty() {
                results.push(result);
            }
        }
    }
    results
}

enum Section {
    Undefined,
    I,
    II,
    III,
}

enum LineType {
    Empty,
    PageLabel,
    PageNumber,
    // each appear only once at the start of section
    Section(Section),
    // always follow page
    Alphabet(char),
    Continued,
    Keyword(String),
    Sub(usize, String),
    Concat(String),
}

impl LineType {
    /// parse line type by
    /// - is_empty() == Empty
    /// - all is number == PageNumber
    /// - one uppercase char == Aplhabet
    /// - match whole word == PageLabel, Section
    /// - end with `––continued` == skip
    /// - count start with `– ` == indent
    /// - is continue keyword (same first char, 2nd char not less than previous char)
    fn new(line: &str, next_line: &str, section: &mut Section, alphabet: &mut char, is_start_new_alphabet: &mut bool, next_alphabet: &mut char, prev_need_next_word: &bool) -> Self {
        if line.is_empty() {
            Self::Empty
        } else if line.parse::<u32>().is_ok() {
            Self::PageNumber
        } else if line.len() == 1 {
            if let Some(c) = line.chars().next()
                && c.is_ascii_uppercase()
            {
                *alphabet = c;
                *is_start_new_alphabet = true;
                Self::Alphabet(c)
            } else {
                panic!("Single charactor, not A-Z: {}", line);
            }
        } else {
            match line {
                "INTERNATIONAL CLASSIFICATION OF DISEASES"
                | "ALPHABETICAL INDEX TO DISEASES AND NATURE OF INJURY"
                | "EXTERNAL CAUSES OF INJURIES"
                | "EXTERNAL CAUSES OF INJURY"
                | "TABLE OF DRUGS AND CHEMICALS" => Self::PageLabel,
                "Section I" => {
                    *section = Section::I;
                    Self::Section(Section::I)
                }
                "Section II" => {
                    *section = Section::II;
                    Self::Section(Section::II)
                }
                "Section III" => {
                    *section = Section::III;
                    Self::Section(Section::III)
                }
                _ => {
                    if matches!(section, Section::III)
                        && [
                            "Poisoning",
                            "Substance Chapter XIX Accidental",
                            "Intentional",
                            "self-harm",
                            "Undetermined",
                            "intent",
                            "Adverse effect",
                            "in therapeutic",
                            "use",
                        ]
                        .contains(&line)
                    {
                        Self::PageLabel
                    } else if line.ends_with("––continued")
                        || line.ends_with("––")
                        || line == "continued"
                        || (next_line.ends_with("––continued") && !next_line.starts_with(*alphabet) && !next_line.starts_with('–'))
                    {
                        Self::Continued
                    } else if line.starts_with("– – – – – – – – – – – ") {
                        Self::Sub(11, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – – – – – – – – ") {
                        Self::Sub(10, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – – – – – – – ") {
                        Self::Sub(9, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – – – – – – ") {
                        Self::Sub(8, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – – – – – ") {
                        Self::Sub(7, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – – – – ") {
                        Self::Sub(6, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – – – ") {
                        Self::Sub(5, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – – ") {
                        Self::Sub(4, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – – ") {
                        Self::Sub(3, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– – ") {
                        Self::Sub(2, line.trim_start_matches("– ").to_owned())
                    } else if line.starts_with("– ") {
                        Self::Sub(1, line.trim_start_matches("– ").to_owned())
                    } else {
                        if *is_start_new_alphabet {
                            *is_start_new_alphabet = false;
                            *next_alphabet = '0';
                        }
                        if is_keyword(line, &alphabet, next_alphabet, &prev_need_next_word) {
                            Self::Keyword(line.to_owned())
                        } else {
                            Self::Concat(line.to_owned())
                        }
                    }
                }
            }
        }
    }
}

// List of text patterns
//
// Altitude, high (effects) – see Effect, adverse, high altitude
// accessory sinus (chronic) (see also Sinusitis) J32.9
// circulatory disease (conditions in category I00-I99, Q20-Q28) P00.3
// right (ventricular) (secondary to left heart failure, conditions in I50.1) (see also Failure, heart, congestive) I50.0
// cervix NEC (acquired) (congenital), in pregnancy or childbirth O34.4
// conditions in
//
//  – see Contraction, joint
//  – see also Malposition, congenital
//  – see categories O03-O06
//  – code to N00-N07 with fourth character .0
//
//  R86.-
//  R74.8
//  A06.6† G07
//  A00-A07, O98.8
struct Patterned {
    /// keywords without brackets/note/code data, a prefix for concat in children
    main: String,
    /// ` – see `, ` – see also `, ` – see categories `, ` – code to `
    note: Option<Note>,
    /// ` (see ..)`, ` (see also ..)`, (conditions in ..)`
    bracket_notes: Vec<Note>,
    /// ICD10 code
    code: Option<Code>,
    /// parents + main + brackets in original order
    concat: String,
}

impl Patterned {
    fn new(text: &str, parents: &[String], is_section_3: bool) -> Self {
        let is_neoplasm = text.starts_with("Neoplasm, neoplastic") || parents.first().map(|s| s.starts_with("Neoplasm, neoplastic")).unwrap_or_default();

        let mut main = String::new();
        let mut code = None;

        let mut concat = String::new();
        let mut last_parent_need_next_word = false;
        for (pi, parent) in parents.iter().enumerate() {
            if pi == 0 {
                concat.push_str(parent);
            } else if last_parent_need_next_word {
                concat.push(' ');
                concat.push_str(parent);
            } else {
                concat.push_str(", ");
                concat.push_str(parent);
            }
            last_parent_need_next_word = NEED_NEXT_WORD_ENDED.iter().any(|w| parent.ends_with(w)) || NEED_NEXT_WORD_EXACT.iter().any(|w| parent == *w);
        }

        let mut note = if text.contains(" – see also ") {
            Some(Note::SeeAlso(String::new()))
        } else if text.contains(" – see categories ") {
            Some(Note::SeeCategory(String::new()))
        } else if text.contains(" – see ") {
            Some(Note::See(String::new()))
        } else if text.contains(" – code") {
            Some(Note::Code(String::new()))
        } else {
            None
        };
        let mut bracket_notes = Vec::new();

        let mut is_between_bracket = false;

        let mut is_in_see_bracket = false;
        let mut is_see_also_bracket = false;
        let mut is_after_bracket = false;

        let mut see_bracket = String::new();
        let mut dagger = None;

        let mut has_neoplasm_tag_hash = false;
        let mut has_neoplasm_tag_star = false;
        let mut neoplasm_codes = Vec::new();

        let mut substance_codes = Vec::new();

        let mut after_hyphen = false;
        let mut last_word_end_with_comma = false;

        for (i, w) in text.split(' ').enumerate() {
            let will_concat_with_space = if i == 0 { last_parent_need_next_word } else { !last_word_end_with_comma && !is_after_bracket };

            let w_trim = w.trim();
            let end_with_comma = w_trim.ends_with(',');

            // remove ..... in Section I Neoplasm
            let word = w_trim.trim_end_matches(['.', ',']);
            if word.is_empty() {
                continue;
            }

            if word == "#" {
                has_neoplasm_tag_hash = true;
            } else if word == "" {
                has_neoplasm_tag_star = true;
            } else if word == "–" {
                after_hyphen = true;
            // ICD10
            } else if is_icd10_resemble(word) {
                if i == 0 {
                    main.push_str(word);
                    if !concat.is_empty() {
                        if last_parent_need_next_word {
                            concat.push(' ');
                        } else {
                            concat.push_str(", ");
                        }
                    }
                    concat.push_str(word);
                } else if is_section_3 {
                    substance_codes.push(word.to_owned());
                } else if is_neoplasm {
                    neoplasm_codes.push(word.to_owned());
                } else if word.ends_with('†') {
                    dagger.replace(word.trim_end_matches('†'));
                } else if word.ends_with('') {
                    if let Some(dagger_inner) = dagger.take() {
                        code.replace(Code::DaggerAster(dagger_inner.to_owned(), word.trim_end_matches('').to_owned()));
                    } else {
                        panic!("Found asterisk without dagger in '{}'", text);
                    }
                } else if dagger.is_some() {
                    panic!("No asterisk after dagger in '{}'", text);
                } else if let Some(Code::Single(old_code)) = code.as_ref() {
                    // > 1 ICD10 codes, move old to bracket or main, use last ICD10 as code
                    if is_in_see_bracket {
                        if !see_bracket.is_empty() {
                            see_bracket.push(' ');
                        }
                        see_bracket.push_str(old_code);
                    } else if is_between_bracket {
                        if !concat.is_empty() {
                            concat.push(' ');
                        }
                        concat.push_str(old_code);
                    } else {
                        if !main.is_empty() {
                            main.push(' ');
                        }
                        main.push_str(old_code);
                        if !concat.is_empty() {
                            concat.push(' ');
                        }
                        concat.push_str(old_code);
                    }
                    code.replace(Code::Single(word.to_owned()));
                } else if code.is_some() {
                    panic!("Double code (non-single) in '{}'", text);
                } else if is_in_see_bracket {
                    if let Some(exact) = word.strip_suffix(')') {
                        let inner = if see_bracket.is_empty() { exact.to_owned() } else { [&see_bracket, " ", exact].concat() };
                        let new_note = if is_see_also_bracket { Note::SeeAlso(inner) } else { Note::See(inner) };
                        bracket_notes.push(new_note);
                        is_in_see_bracket = false;
                        is_see_also_bracket = false;
                        is_between_bracket = false;
                        is_after_bracket = true;
                        see_bracket.clear();
                    }
                    if !see_bracket.is_empty() {
                        see_bracket.push(' ');
                    }
                    see_bracket.push_str(word);
                } else if is_between_bracket {
                    if word.ends_with(')') {
                        is_between_bracket = false;
                        is_after_bracket = true;
                    }
                    if !concat.is_empty() {
                        concat.push(' ');
                    }
                    concat.push_str(word);
                } else {
                    code.replace(Code::Single(word.to_owned()));
                }
            } else if after_hyphen {
                // has note
                if let Some(note_inner) = note.as_mut() {
                    match note_inner {
                        Note::SeeAlso(s) => {
                            if !["see", "also"].contains(&word) {
                                if !s.is_empty() {
                                    s.push(' ');
                                }
                                s.push_str(&word);
                            }
                        }
                        Note::SeeCategory(s) => {
                            if !["see", "categories"].contains(&word) {
                                if !s.is_empty() {
                                    s.push(' ');
                                }
                                s.push_str(&word);
                            }
                        }
                        Note::See(s) => {
                            if word != "see" {
                                if !s.is_empty() {
                                    s.push(' ');
                                }
                                s.push_str(&word);
                            }
                        }
                        Note::Code(s) => {
                            if !s.is_empty() {
                                s.push(' ');
                            }
                            s.push_str(&word);
                        }
                    }
                } else {
                    panic!("No note exists after hyphen in '{}'", text);
                }
            // before hyphen
            } else {
                match (word.starts_with('('), word.ends_with(')')) {
                    // '(xxxx)'
                    (true, true) => {
                        if !DISCARD_PARENTHESES_WORD.contains(&word) {
                            if !concat.is_empty() {
                                concat.push_str(", ");
                                is_after_bracket = true;
                            }
                            concat.push_str(word);
                        }
                    }
                    // '(xxx'
                    (true, false) => {
                        if word == "(see" {
                            is_in_see_bracket = true;
                        } else {
                            if !concat.is_empty() {
                                concat.push_str(", ");
                            }
                            concat.push_str(word);
                        }
                        is_between_bracket = true;
                    }
                    // 'xxx)'
                    (false, true) => {
                        if is_in_see_bracket {
                            if let Some(exact) = word.strip_suffix(')') {
                                let inner = if see_bracket.is_empty() { exact.to_owned() } else { [&see_bracket, " ", exact].concat() };
                                let new_note = if is_see_also_bracket { Note::SeeAlso(inner) } else { Note::See(inner) };
                                bracket_notes.push(new_note);
                                is_in_see_bracket = false;
                                is_see_also_bracket = false;
                                is_between_bracket = false;
                                is_after_bracket = true;
                                see_bracket.clear();
                            }
                        } else if is_between_bracket {
                            if !concat.is_empty() {
                                concat.push(' ');
                            }
                            concat.push_str(word);
                            is_between_bracket = false;
                            is_after_bracket = true;
                        } else {
                            if !main.is_empty() {
                                if last_word_end_with_comma || is_after_bracket {
                                    main.push_str(", ");
                                    is_after_bracket = false;
                                } else {
                                    main.push(' ');
                                }
                            }
                            main.push_str(word);
                            if !concat.is_empty() {
                                if will_concat_with_space {
                                    concat.push(' ');
                                } else {
                                    concat.push_str(", ");
                                    is_after_bracket = false;
                                }
                            }
                            concat.push_str(word);
                            // if !main.is_empty() {
                            //     main.push(' ');
                            // }
                            // main.push_str(word);
                            // if !concat.is_empty() {
                            //     concat.push(' ');
                            // }
                            // concat.push_str(word);
                        }
                    }
                    // any word
                    (false, false) => {
                        if is_in_see_bracket {
                            if word == "also" {
                                is_see_also_bracket = true;
                            } else {
                                if !see_bracket.is_empty() {
                                    see_bracket.push(' ');
                                }
                                see_bracket.push_str(word);
                            }
                        } else if is_between_bracket {
                            if !concat.is_empty() {
                                concat.push(' ');
                            }
                            concat.push_str(word);
                        } else {
                            if !main.is_empty() {
                                if last_word_end_with_comma || is_after_bracket {
                                    main.push_str(", ");
                                    is_after_bracket = false;
                                } else {
                                    main.push(' ');
                                }
                            }
                            main.push_str(word);
                            if !concat.is_empty() {
                                if will_concat_with_space {
                                    concat.push(' ');
                                } else {
                                    concat.push_str(", ");
                                    is_after_bracket = false;
                                }
                            }
                            concat.push_str(word);
                        }
                    }
                }
            }
            last_word_end_with_comma = end_with_comma;
        }

        if code.is_none() {
            if is_section_3 && !substance_codes.is_empty() {
                if substance_codes.len() > 5 {
                    panic!("Substance codes > 5 in '{}'", text);
                }
                code.replace(Code::Substance(substance_codes));
            } else if !neoplasm_codes.is_empty() {
                if neoplasm_codes.len() > 5 {
                    panic!("Neoplasm codes > 5 in '{}'", text);
                }
                let tag = if has_neoplasm_tag_hash {
                    Some(NeoplasmTag::Hash)
                } else if has_neoplasm_tag_star {
                    Some(NeoplasmTag::Star)
                } else {
                    None
                };
                code.replace(Code::Neoplasm(neoplasm_codes, tag));
            }
        }

        Self {
            main,
            note,
            bracket_notes,
            code,
            concat,
        }
    }
}

fn is_keyword(line: &str, alphabet: &char, next_alphabet: &mut char, prev_need_next_word: &bool) -> bool {
    if *prev_need_next_word {
        false
    } else {
        let mut cs = line.chars();
        if cs.next().map_or(false, |c1| c1 == *alphabet) {
            if let Some(c2) = cs.next() {
                if c2.is_ascii_alphabetic() || is_accent_char(&c2) {
                    let c_lo = convert_accent_char(&c2);
                    if c_lo >= *next_alphabet {
                        *next_alphabet = c_lo;
                        true
                    } else {
                        false
                    }
                } else if c2 == ' ' {
                    // for "Q fever A78"
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn is_accent_char(c: &char) -> bool {
    [
        'á', 'à', 'â', 'ã', 'ä', 'å', 'é', 'è', 'ê', 'ë', 'ē', 'í', 'ì', 'î', 'ĩ', 'ï', 'ó', 'ò', 'ô', 'õ', 'ö', 'ú', 'ù', 'û', 'ũ', 'ü', 'ç',
    ]
    .contains(c)
}

fn convert_accent_char(c: &char) -> char {
    if ['á', 'à', 'â', 'ã', 'ä', 'å'].contains(c) {
        'a'
    } else if ['é', 'è', 'ê', 'ë', 'ē'].contains(c) {
        'e'
    } else if ['í', 'ì', 'î', 'ĩ', 'ï'].contains(c) {
        'i'
    } else if ['ó', 'ò', 'ô', 'õ', 'ö'].contains(c) {
        'o'
    } else if ['ú', 'ù', 'û', 'ũ', 'ü'].contains(c) {
        'u'
    } else if *c == 'ç' {
        'c'
    } else {
        c.to_ascii_lowercase()
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use std::{env::current_dir, io::{BufWriter, Read, Write}, fs};

    use kphis_drg_worker::drg::model::DaggerAsterisk;

    use crate::write_to;
    use super::*;

    fn write_section(section: usize, data: &[Vec<(usize, String)>]) {
        let mut current = current_dir().unwrap();
        current.push("debug");
        current.push(["section-", &section.to_string()].concat());
        if !fs::exists(&current).unwrap() {
            fs::create_dir_all(&current).unwrap();
        }
        if let Some(parent) = current.as_os_str().to_str() {
            for s in data.iter() {
                let prefix = s.first().and_then(|(_, f)| f.chars().next()).unwrap_or_default();
                let path = [parent, "/", &prefix.to_string(), ".txt"].concat();
                let file = fs::File::create(path).unwrap();
                let mut writer = BufWriter::new(file);
                for (indent, line) in s {
                    writeln!(writer, "{}{}", vec!["    "; *indent].concat(), line).unwrap();
                }
            }
        }
    }

    fn write_patterns(section: usize, data: &[Vec<(usize, String)>]) {
        let mut current = current_dir().unwrap();
        current.push("debug");
        current.push(["pattern-", &section.to_string()].concat());
        if !fs::exists(&current).unwrap() {
            fs::create_dir_all(&current).unwrap();
        }
        if let Some(parent) = current.as_os_str().to_str() {
            let path_all = [parent, "/ALL.txt"].concat();
            let file_all = fs::File::create(path_all).unwrap();
            let mut writer_all = BufWriter::new(file_all);
            for s in data.iter() {
                let prefix = s.first().and_then(|(_, f)| f.chars().next()).unwrap_or_default();
                let path = [parent, "/", &prefix.to_string(), ".txt"].concat();
                let file = fs::File::create(path).unwrap();
                let mut writer = BufWriter::new(file);

                let result = parse_alphabet_to_pattern(s, section);
                for item in result.iter() {
                    writeln!(writer_all, "{}; Code: {:?}; Note: {:?} {:?};", item.concat, item.code, item.note, item.bracket_notes).unwrap();
                    writeln!(writer, "{}; Code: {:?}; Note: {:?} {:?};", item.concat, item.code, item.note, item.bracket_notes).unwrap();
                }
            }
        }
    }

    #[test]
    fn test_parsing_to_debug_folder() {
        let raw = include_str!("../raw-icd-who/icd102016en-book3-tm.txt");
        let (section_1, section_2, section_3) = split_section(raw);

        write_section(1, &section_1);
        write_section(2, &section_2);
        write_section(3, &section_3);

        write_patterns(1, &section_1);
        write_patterns(2, &section_2);
        write_patterns(3, &section_3);
    }

    #[test]
    fn test_dump_i10_index() {
        let write_index = parse_i10_index();
        let write_bytes = bitcode::encode(&write_index);
        // dbg!(current_dir().unwrap()); // kphis/crates/kphis-dump-builder
        let path = "../kphis-drg-worker/dump/i10-index.dump";
        write_to(&write_bytes, path);

        let mut read_file = fs::File::open(path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let read_index = bitcode::decode::<I10Index>(&read_bytes).unwrap();

        let diagnosis_len = 52631;
        let external_len = 2715;
        let substance_len = 5277;

        // for line in read_index.diagnosis.keys().take(100) {
        //     println!("{line}");
        // }

        assert_eq!(read_index.diagnosis.len(), diagnosis_len);
        assert_eq!(write_index.diagnosis.len() ,diagnosis_len);
        assert_eq!(read_index.external.len(), external_len);
        assert_eq!(write_index.external.len() ,external_len);
        assert_eq!(read_index.substance.len(), substance_len);
        assert_eq!(write_index.substance.len() ,substance_len);
    }

    #[test]
    fn test_compare_dagger_asterisk() {
        let verbose = false;

        let mut dagger_asterisk_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/dagger-asterisk.csv")[..]);
        let mut grouper_pairs = HashMap::new();
        for i in dagger_asterisk_rdr.deserialize::<DaggerAsterisk>() {
            let row = i.expect("invalid /raw-grouper/dagger-asterisk.csv, please run raw_parser and try again");
            grouper_pairs.entry(row.asterisk.to_owned()).or_insert(HashSet::new()).insert(row.dagger.to_owned());
        }
        let index = parse_i10_index();
        let claml_pairs = crate::i10_claml::get_claml_asterisk_dagger();

        // DaggerAsterisk pairs in grouper NEEDED TO BE a subset of index
        let index_asters = index.aster_dagger.keys().cloned().collect::<HashSet<String>>();
        let grouper_asters = grouper_pairs.keys().cloned().collect::<HashSet<String>>();
        let claml_asters = claml_pairs.keys().cloned().collect::<HashSet<String>>();

        assert_eq!(index_asters.len(), 746);
        assert_eq!(grouper_asters.len(), 506);
        assert_eq!(claml_asters.len(), 433);

        let mut index_diff_grouper = index_asters.difference(&grouper_asters).collect::<Vec<&String>>();
        let mut index_diff_claml = index_asters.difference(&claml_asters).collect::<Vec<&String>>();

        let mut grouper_diff_index = grouper_asters.difference(&index_asters).collect::<Vec<&String>>();
        let mut grouper_diff_claml = grouper_asters.difference(&claml_asters).collect::<Vec<&String>>();

        let mut claml_diff_index = claml_asters.difference(&index_asters).collect::<Vec<&String>>();
        let mut claml_diff_grouper = claml_asters.difference(&grouper_asters).collect::<Vec<&String>>();

        assert_eq!(index_diff_grouper.len(), 240);
        assert_eq!(index_diff_claml.len(), 313);
        // index already included all of grouper's dagger-asterisk pairs
        assert!(grouper_diff_index.is_empty());
        assert_eq!(grouper_diff_claml.len(), 309);
        // index already included all of claml's dagger-asterisk pairs
        assert!(claml_diff_index.is_empty());
        assert_eq!(claml_diff_grouper.len(), 236);

        if verbose {
            index_diff_grouper.sort();
            index_diff_claml.sort();
            grouper_diff_index.sort();
            grouper_diff_claml.sort();
            claml_diff_index.sort();
            claml_diff_grouper.sort();

            // dbg!(&index_diff_grouper);
            // dbg!(&index_diff_claml);

            // dbg!(&grouper_diff_index);
            // dbg!(&grouper_diff_claml);

            // dbg!(&claml_diff_index);
            // dbg!(&claml_diff_grouper);
        }


    }

    #[test]
    fn test_index_code_has_subclass() {
        let verbose = false;

        let index = parse_i10_index();
        let claml = crate::i10_claml::new_i10_claml();
        let mut before_last_child = HashSet::new();
        for class in claml.classes.values() {
            // only true last child
            if class.subclasses.is_empty() && class.sub_modifier.is_none()
                // || (class.without_modifier_class.is_some() && (class.with_modifier_class_2.is_some() || (class.with_modifier_class_1.is_some() && class.with_modifier_class_2.is_none())))
            {
                if let Some(parent) = &class.superclass {
                    before_last_child.insert(parent.to_owned());
                }
            }
        }
        let mut code_is_parent_set = HashSet::new();
        for pointer in index.diagnosis.values() {
            if let Some(code) = pointer.code.as_ref() {
                match code {
                    Code::DaggerAster(dagger, aster) => {
                        if before_last_child.contains(dagger) {
                            code_is_parent_set.insert(dagger.to_owned());
                        }
                        if before_last_child.contains(aster) {
                            code_is_parent_set.insert(aster.to_owned());
                        }
                    }
                    Code::Single(c) => {
                        if before_last_child.contains(c) {
                            code_is_parent_set.insert(c.to_owned());
                        }
                    }
                    _ => {}
                }
            }
        }
        if verbose {
            // TODO edit icd102016en-book3-tm.txt to use the last child code
            // assert_eq!(code_is_parent_set.len(), 610);
            let mut code_is_parent = code_is_parent_set.into_iter().collect::<Vec<String>>();
            code_is_parent.sort();
            dbg!(code_is_parent);
        }
    }
}
