mod btree;
mod contains;
mod trigram;

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    sync::Arc,
    time::Instant,
};

use dump_builder::drg_grouper::new_grouper;
use kphis_drg_worker::{
    drg::{grouper::Grouper, model::I10vx},
    i10::{
        claml::{ClassKind, I10Claml, PROPOSITIONS, Reference, RubricKind, UsageKind},
        index::{I10Index, I10Pointer},
    },
};
use kphis_util::{
    british_american::TRANSLATOR,
    util::{is_icd10_without_dot, sanity_space},
};

pub fn test_search(keyword: &str, n: usize) {
    let i10grouper_path = "crates/benches/dump/grouper.dump";
    if !std::fs::exists(i10grouper_path).unwrap() {
        let grouper = new_grouper();
        let grouper_bytes = bitcode::encode(&grouper);
        println!("i10vx data has {} rows", grouper.i10vx.len());
        write_to(&grouper_bytes, i10grouper_path);
    }

    let i10tm_path = "crates/benches/dump/i10tm-noex.dump";
    if !std::fs::exists(i10tm_path).unwrap() {
        dump_i10tm_not_ex_only();
    }

    let i10who_path = "crates/benches/dump/i10who.dump";
    if !std::fs::exists(i10who_path).unwrap() {
        let i10_asset = dump_builder::i10_claml::new_i10_claml();
        let i10who = extract_keywords(&i10_asset);
        println!("i10 WHO data has {} rows", i10who.len());

        let i10_bytes = bitcode::encode(&i10who);
        write_to(&i10_bytes, i10who_path);
    }

    let i10index_path = "crates/benches/dump/i10-index.dump";
    if !std::fs::exists(i10index_path).unwrap() {
        let i10_index = dump_builder::i10_index::parse_i10_index();
        let i10_index_bytes = bitcode::encode(&i10_index);
        write_to(&i10_index_bytes, i10index_path);
    }

    {
        let mut read_file = File::open(i10grouper_path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let grouper = bitcode::decode::<Grouper>(&read_bytes).unwrap();
        let vx_bt = grouper.i10vx;
        let vx_hm = vx_bt.clone().into_iter().collect::<std::collections::HashMap<String, Arc<I10vx>>>();

        println!("Test I10vx HashMap search '{}' with 'trigram'", keyword);
        // trigram
        let start_trigram = Instant::now();
        let trigram_result = trigram::search_i10vx_fuzzy(keyword, &vx_hm, n);
        let trigram_duration = start_trigram.elapsed();
        display_vx_result(trigram_result);
        println!("I10vx HashMap Trigram execution time: {:.2?}\n", trigram_duration);

        println!("Test I10vx HashMap search '{}' with 'contains'", keyword);
        // contains
        let start_contains = Instant::now();
        let contains_result = contains::search_i10vx_contains(keyword, &vx_hm, n);
        let contains_duration = start_contains.elapsed();
        display_vx_result(contains_result);
        println!("I10vx HashMap Contains execution time: {:.2?}\n", contains_duration);

        if is_icd10_without_dot(keyword) {
            println!("Test I10vx BtreeMap search '{}'", keyword);
            // BTree
            let start_btree = Instant::now();
            let btree_result = btree::search_i10vx_btree(keyword, &vx_bt);
            let btree_duration = start_btree.elapsed();
            display_vx_result(btree_result);
            println!("I10vx BtreeMap execution time: {:.2?}\n", btree_duration);
        }
    }

    {
        let mut read_file = File::open(i10tm_path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let i10tm = bitcode::decode::<HashMap<String, String>>(&read_bytes).unwrap();

        println!("Test I10TM search '{}' with 'trigram'", keyword);
        // trigram
        let start_trigram = Instant::now();
        let trigram_result = trigram::search_i10tm_fuzzy(keyword, &i10tm, n);
        let trigram_duration = start_trigram.elapsed();
        display_result(trigram_result);
        println!("I10TM Trigram execution time: {:.2?}\n", trigram_duration);
        println!("Test I10TM search '{}' with 'contains'", keyword);
        // contains
        let start_contains = Instant::now();
        let contains_result = contains::search_i10tm_contains(keyword, &i10tm, n);
        let contains_duration = start_contains.elapsed();
        display_result(contains_result);
        println!("I10TM Contains execution time: {:.2?}\n", contains_duration);
    }

    {
        let mut read_file = File::open(i10who_path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let i10who = bitcode::decode::<Vec<Arc<I10Keywords>>>(&read_bytes).unwrap();

        let mut a = HashMap::new();
        for i in i10who.iter() {
            // if i.detail.as_str() == "Meningitis due to other and unspecified causes" {
            //     dbg!(i);
            // }
            let key = if matches!(i.rubric_kind, RubricKind::Exclusion) {
                [&i.detail, " : ", &i.references.iter().map(|r| r.label.as_str()).collect::<Vec<&str>>().concat()].concat()
            } else {
                [&i.detail, " : ", &i.class_code].concat()
            };
            if let Some(v) = a.get_mut(&key) {
                *v += 1;
            } else {
                a.insert(key, 1);
            }
        }
        let mut b = a.into_iter().collect::<Vec<(String, i32)>>();
        b.sort_by(|a, b| b.1.cmp(&a.1));
        // dbg!(b.into_iter().take(5).collect::<Vec<(String, i32)>>());

        println!("Test I10WHO search '{}' with 'trigram'", keyword);
        // trigram
        let start_trigram = Instant::now();
        let trigram_result = trigram::search_i10who_fuzzy(keyword, &i10who, n);
        let trigram_duration = start_trigram.elapsed();
        display_i10_result(trigram_result);
        println!("I10WHO Trigram execution time: {:.2?}\n", trigram_duration);
        println!("Test I10WHO search '{}' with 'contains'", keyword);
        // contains
        let start_contains = Instant::now();
        let contains_result = contains::search_i10who_contains(keyword, &i10who, n);
        let contains_duration = start_contains.elapsed();
        display_i10_result(contains_result);
        println!("I10WHO Contains execution time: {:.2?}\n", contains_duration);
    }

    {
        let mut read_file = File::open(i10index_path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let i10index = bitcode::decode::<I10Index>(&read_bytes).unwrap();

        println!("Test I10 Index search '{}' with 'trigram'", keyword);
        // trigram
        let start_trigram = Instant::now();
        let trigram_result = trigram::search_i10_index_fuzzy(keyword, &i10index.diagnosis, n);
        let trigram_duration = start_trigram.elapsed();
        display_index_result(trigram_result);
        println!("I10 Index Trigram execution time: {:.2?}\n", trigram_duration);
        println!("Test I10 Index search '{}' with 'contains'", keyword);
        // contains
        let start_contains = Instant::now();
        let contains_result = contains::search_i10_index_contains(keyword, &i10index.diagnosis, n);
        let contains_duration = start_contains.elapsed();
        display_index_result(contains_result);
        println!("I10 Index Contains execution time: {:.2?}\n", contains_duration);
        // contains V2
        let start_contains_v2 = Instant::now();
        let contains_result_v2 = contains::search_i10_index_contains_v2(keyword, &i10index.diagnosis, n);
        let contains_duration_v2 = start_contains_v2.elapsed();
        display_index_result(contains_result_v2);
        println!("I10 Index Contains V2 execution time: {:.2?}\n", contains_duration_v2);
    }
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct I10Keywords {
    // RubricKind::Exclusion will be ""
    pub class_code: String,
    // RubricKind::Exclusion will be None
    pub class_usage: Option<UsageKind>,
    pub rubric_kind: RubricKind,
    pub rubric_usage: Option<UsageKind>,
    pub references: Vec<Reference>,
    pub detail: String,
}

// duplicate detail prevention
// - preferred: class_code + detail
// - inclusion: class_code + detail
// - exclusion: detail + reference.labels, only has Reference
fn extract_keywords(i10: &I10Claml) -> Vec<Arc<I10Keywords>> {
    let mut result = Vec::new();
    let mut mod_created = Vec::new();
    let mut detail_created = Vec::new();
    for (code, class) in i10.classes.iter() {
        if matches!(class.kind, ClassKind::Category) {
            // modified (has ClassKind, without_modifier_class, NO SubClass, NO Rubric)
            if let Some(parent_code) = &class.without_modifier_class {
                if let Some(parent) = i10.classes.get(parent_code) {
                    // Class has only one Preferred / PreferredLong Rubric
                    if let Some((_, parent_preferred_rubric)) = parent
                        .rubrics
                        .iter()
                        .find(|(_, r)| matches!(r.kind, RubricKind::PreferredLong))
                        .or(parent.rubrics.iter().find(|(_, r)| matches!(r.kind, RubricKind::Preferred)))
                    {
                        // ModifierClass 1 and 2
                        if let (Some(mc_1_code), Some(mc_2_code)) = (&class.with_modifier_class_1, &class.with_modifier_class_2) {
                            if let (Some(mc_1), Some(mc_2)) = (i10.modifier_classes.get(mc_1_code), i10.modifier_classes.get(mc_2_code)) {
                                let mut detail = parent_preferred_rubric.text.to_owned();
                                let mut refs = parent_preferred_rubric.reference.clone();
                                let (mc_1_preferred, mc_1_others): (Vec<_>, Vec<_>) = mc_1.rubrics.iter().partition(|(_, r)| matches!(r.kind, RubricKind::Preferred));
                                if let Some((_, r)) = mc_1_preferred.first() {
                                    detail.push(' ');
                                    detail.push_str(&r.text);
                                    refs.extend(r.reference.clone());
                                }
                                let mod_1_code_concat = [parent_code, mc_1.code.as_str()].concat();
                                for (id, r) in mc_1_others.iter() {
                                    if matches!(r.kind, RubricKind::Inclusion) || (matches!(r.kind, RubricKind::Exclusion) && !r.reference.is_empty()) {
                                        let mod_1_rubric = [&mod_1_code_concat, id.as_str()].concat();
                                        let mod_1_detail = [&parent_preferred_rubric.text, " ", &r.text].concat();
                                        let (class_code, class_usage) = if matches!(r.kind, RubricKind::Exclusion) {
                                            (String::new(), None)
                                        } else {
                                            (mod_1_code_concat.replace('.', ""), parent.usage.to_owned())
                                        };
                                        let detail_key = if matches!(r.kind, RubricKind::Exclusion) {
                                            [&mod_1_detail, " : ", &r.reference.iter().map(|rf| rf.label.as_str()).collect::<Vec<&str>>().concat()].concat()
                                        } else {
                                            [&mod_1_detail, " : ", &class_code].concat()
                                        };
                                        if !mod_created.contains(&mod_1_rubric) && !detail_created.contains(&detail_key) {
                                            result.push(Arc::new(I10Keywords {
                                                class_code,
                                                class_usage,
                                                rubric_kind: r.kind.to_owned(),
                                                rubric_usage: r.usage.to_owned(),
                                                references: r.reference.to_owned(),
                                                detail: mod_1_detail.clone(),
                                            }));
                                            mod_created.push(mod_1_rubric);
                                            detail_created.push(detail_key);
                                        }
                                    }
                                }
                                let (mc_2_preferred, mc_2_others): (Vec<_>, Vec<_>) = mc_2.rubrics.iter().partition(|(_, r)| matches!(r.kind, RubricKind::Preferred));
                                if let Some((_, r)) = mc_2_preferred.first() {
                                    detail.push(' ');
                                    detail.push_str(&r.text);
                                    refs.extend(r.reference.clone());
                                }
                                let mod_2_code_concat = [parent_code, mc_2.code.as_str()].concat();
                                for (id, r) in mc_2_others.iter() {
                                    if matches!(r.kind, RubricKind::Inclusion) || (matches!(r.kind, RubricKind::Exclusion) && !r.reference.is_empty()) {
                                        let mod_2_rubric = [&mod_2_code_concat, id.as_str()].concat();
                                        let mod_2_detail = [&parent_preferred_rubric.text, " ", &r.text].concat();
                                        let (class_code, class_usage) = if matches!(r.kind, RubricKind::Exclusion) {
                                            (String::new(), None)
                                        } else {
                                            (mod_2_code_concat.replace('.', ""), parent.usage.to_owned())
                                        };
                                        let detail_key = if matches!(r.kind, RubricKind::Exclusion) {
                                            [&mod_2_detail, " : ", &r.reference.iter().map(|rf| rf.label.as_str()).collect::<Vec<&str>>().concat()].concat()
                                        } else {
                                            [&mod_2_detail, " : ", &class_code].concat()
                                        };
                                        if !mod_created.contains(&mod_2_rubric) && !detail_created.contains(&detail_key) {
                                            result.push(Arc::new(I10Keywords {
                                                class_code,
                                                class_usage,
                                                rubric_kind: r.kind.to_owned(),
                                                rubric_usage: r.usage.to_owned(),
                                                references: r.reference.to_owned(),
                                                detail: mod_2_detail.clone(),
                                            }));
                                            mod_created.push(mod_2_rubric);
                                            detail_created.push(detail_key);
                                        }
                                    }
                                }
                                let class_code = code.replace('.', "");
                                let detail_key = [&detail, " : ", &class_code].concat();
                                if detail_created.contains(&detail_key) {
                                    result.retain(|rs| !(matches!(rs.rubric_kind, RubricKind::Exclusion) && rs.detail == detail));
                                } else {
                                    detail_created.push(detail_key);
                                }
                                result.push(Arc::new(I10Keywords {
                                    class_code,
                                    class_usage: class.usage.to_owned(),
                                    rubric_kind: parent_preferred_rubric.kind.to_owned(),
                                    rubric_usage: parent_preferred_rubric.usage.to_owned(),
                                    references: refs,
                                    detail,
                                }));
                            }
                        // ModifierClass 1 only
                        } else if let Some(mc_1_code) = &class.with_modifier_class_1 {
                            if let Some(mc_1) = i10.modifier_classes.get(mc_1_code) {
                                let (mc_1_preferred, mc_1_others): (Vec<_>, Vec<_>) = mc_1.rubrics.iter().partition(|(_, r)| matches!(r.kind, RubricKind::Preferred));
                                if let Some((_, r)) = mc_1_preferred.first() {
                                    let class_code = code.replace('.', "");
                                    let mod_1_detail = [&parent_preferred_rubric.text, " ", &r.text].concat();
                                    let detail_key = [&mod_1_detail, " : ", &class_code].concat();
                                    if detail_created.contains(&detail_key) {
                                        result.retain(|rs| !(matches!(rs.rubric_kind, RubricKind::Exclusion) && rs.detail == mod_1_detail));
                                    } else {
                                        detail_created.push(detail_key);
                                    }
                                    result.push(Arc::new(I10Keywords {
                                        class_code,
                                        class_usage: class.usage.to_owned(),
                                        rubric_kind: r.kind.to_owned(),
                                        rubric_usage: r.usage.to_owned(),
                                        references: r.reference.to_owned(),
                                        detail: mod_1_detail,
                                    }));
                                }
                                let mod_1_code_concat = [parent_code, mc_1.code.as_str()].concat();
                                for (id, r) in mc_1_others.iter() {
                                    if matches!(r.kind, RubricKind::Inclusion) || (matches!(r.kind, RubricKind::Exclusion) && !r.reference.is_empty()) {
                                        let mod_1_rubric = [&mod_1_code_concat, id.as_str()].concat();
                                        let mod_1_detail = [&parent_preferred_rubric.text, " ", &r.text].concat();
                                        let (class_code, class_usage) = if matches!(r.kind, RubricKind::Exclusion) {
                                            (String::new(), None)
                                        } else {
                                            (mod_1_code_concat.replace('.', ""), parent.usage.to_owned())
                                        };
                                        let detail_key = if matches!(r.kind, RubricKind::Exclusion) {
                                            [&mod_1_detail, " : ", &r.reference.iter().map(|rf| rf.label.as_str()).collect::<Vec<&str>>().concat()].concat()
                                        } else {
                                            [&mod_1_detail, " : ", &class_code].concat()
                                        };
                                        if !mod_created.contains(&mod_1_rubric) && !detail_created.contains(&detail_key) {
                                            result.push(Arc::new(I10Keywords {
                                                class_code,
                                                class_usage,
                                                rubric_kind: r.kind.to_owned(),
                                                rubric_usage: r.usage.to_owned(),
                                                references: r.reference.to_owned(),
                                                detail: mod_1_detail.clone(),
                                            }));
                                            mod_created.push(mod_1_rubric);
                                            detail_created.push(detail_key);
                                        }
                                    }
                                }
                            }
                        // ModifierClass 2 only
                        } else if let Some(mc_2_code) = &class.with_modifier_class_2 {
                            if let Some(mc_2) = i10.modifier_classes.get(mc_2_code) {
                                let (mc_2_preferred, mc_2_others): (Vec<_>, Vec<_>) = mc_2.rubrics.iter().partition(|(_, r)| matches!(r.kind, RubricKind::Preferred));
                                if let Some((_, r)) = mc_2_preferred.first() {
                                    let class_code = code.replace('.', "");
                                    let mod_2_detail = [&parent_preferred_rubric.text, " ", &r.text].concat();
                                    let detail_key = [&mod_2_detail, " : ", &class_code].concat();
                                    if detail_created.contains(&detail_key) {
                                        result.retain(|rs| !(matches!(rs.rubric_kind, RubricKind::Exclusion) && rs.detail == mod_2_detail));
                                    } else {
                                        detail_created.push(detail_key);
                                    }
                                    result.push(Arc::new(I10Keywords {
                                        class_code,
                                        class_usage: class.usage.to_owned(),
                                        rubric_kind: r.kind.to_owned(),
                                        rubric_usage: r.usage.to_owned(),
                                        references: r.reference.to_owned(),
                                        detail: mod_2_detail,
                                    }));
                                }
                                let mod_2_code_concat = [parent_code, mc_2.code.as_str()].concat();
                                for (id, r) in mc_2_others.iter() {
                                    if matches!(r.kind, RubricKind::Inclusion) || (matches!(r.kind, RubricKind::Exclusion) && !r.reference.is_empty()) {
                                        let mod_2_rubric = [&mod_2_code_concat, id.as_str()].concat();
                                        let mod_2_detail = [&parent_preferred_rubric.text, " ", &r.text].concat();
                                        let (class_code, class_usage) = if matches!(r.kind, RubricKind::Exclusion) {
                                            (String::new(), None)
                                        } else {
                                            (mod_2_code_concat.replace('.', ""), parent.usage.to_owned())
                                        };
                                        let detail_key = if matches!(r.kind, RubricKind::Exclusion) {
                                            [&mod_2_detail, " : ", &r.reference.iter().map(|rf| rf.label.as_str()).collect::<Vec<&str>>().concat()].concat()
                                        } else {
                                            [&mod_2_detail, " : ", &class_code].concat()
                                        };
                                        if !mod_created.contains(&mod_2_rubric) && !detail_created.contains(&detail_key) {
                                            result.push(Arc::new(I10Keywords {
                                                class_code,
                                                class_usage,
                                                rubric_kind: r.kind.to_owned(),
                                                rubric_usage: r.usage.to_owned(),
                                                references: r.reference.to_owned(),
                                                detail: mod_2_detail.clone(),
                                            }));
                                            mod_created.push(mod_2_rubric);
                                            detail_created.push(detail_key);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            // original
            } else {
                // // without prefix
                // for (_, r) in class.rubrics.iter() {
                //     if matches!(r.kind, RubricKind::Preferred | RubricKind::Inclusion | RubricKind::Exclusion) {
                //         result.push(Arc::new(I10Keywords {
                //             class_code: code.replace('.', ""),
                //             class_usage: class.usage.to_owned(),
                //             rubric_kind: r.kind.to_owned(),
                //             rubric_usage: r.usage.to_owned(),
                //             references: r.reference.to_owned(),
                //             detail: r.text.to_owned(),
                //         }));
                //     }
                // }
                // with prefix
                let (preferred, others): (Vec<_>, Vec<_>) = class.rubrics.iter().partition(|(_, r)| matches!(r.kind, RubricKind::PreferredLong | RubricKind::Preferred));
                let mut preferred_prefix = String::new();
                let (long, pref): (Vec<_>, Vec<_>) = preferred.iter().partition(|(_, r)| matches!(r.kind, RubricKind::PreferredLong));
                if let Some((_, r)) = long.first().or(pref.first()) {
                    let class_code = code.replace('.', "");
                    let detail_key = [&r.text, " : ", &class_code].concat();
                    if detail_created.contains(&detail_key) {
                        result.retain(|rs| !(matches!(rs.rubric_kind, RubricKind::Exclusion) && rs.detail == r.text));
                    } else {
                        detail_created.push(detail_key);
                    }
                    result.push(Arc::new(I10Keywords {
                        class_code,
                        class_usage: class.usage.to_owned(),
                        rubric_kind: r.kind.to_owned(),
                        rubric_usage: r.usage.to_owned(),
                        references: r.reference.to_owned(),
                        detail: r.text.to_owned(),
                    }));
                    preferred_prefix = r.text.to_owned();
                }
                for (_, r) in others.iter() {
                    if matches!(r.kind, RubricKind::Inclusion) || (matches!(r.kind, RubricKind::Exclusion) && !r.reference.is_empty()) {
                        let detail = if PROPOSITIONS.iter().any(|p| r.text.starts_with(p)) {
                            [&preferred_prefix, " ", &r.text].concat()
                        } else {
                            r.text.to_owned()
                        };
                        let (class_code, class_usage) = if matches!(r.kind, RubricKind::Exclusion) {
                            (String::new(), None)
                        } else {
                            (code.replace('.', ""), class.usage.to_owned())
                        };
                        let detail_key = if matches!(r.kind, RubricKind::Exclusion) {
                            [&detail, " : ", &r.reference.iter().map(|rf| rf.label.as_str()).collect::<Vec<&str>>().concat()].concat()
                        } else {
                            [&detail, " : ", &class_code].concat()
                        };
                        let new = Arc::new(I10Keywords {
                            class_code: class_code.clone(),
                            class_usage,
                            rubric_kind: r.kind.to_owned(),
                            rubric_usage: r.usage.to_owned(),
                            references: r.reference.to_owned(),
                            detail,
                        });
                        if detail_created.contains(&detail_key) {
                            if let Some(pos) = result.iter().position(|rs| {
                                if matches!(rs.rubric_kind, RubricKind::Exclusion) {
                                    [&rs.detail, " : ", &rs.references.iter().map(|rf| rf.label.as_str()).collect::<Vec<&str>>().concat()].concat() == detail_key
                                } else {
                                    [&rs.detail, " : ", &rs.class_code].concat() == detail_key
                                }
                            }) {
                                let _ = result.swap_remove(pos);
                                result.push(new);
                            }
                        } else {
                            result.push(new);
                            detail_created.push(detail_key);
                        }
                    }
                }
            }
        }
    }

    result
}

fn dump_i10tm_not_ex_only() {
    let mut i10tm_rdr = csv::Reader::from_reader(&include_bytes!("../../dump-builder/raw-icd-tm/icd-10-tm2016-20210805.csv")[..]);
    let mut i10tm = HashMap::new();
    for record in i10tm_rdr.records() {
        let row = record.expect("invalid icd-10-tm2016-20210805.csv");
        if let (Some(code), Some(detail)) = (row.get(0), row.get(1)) {
            if !code.starts_with(&['V', 'W', 'X', 'Y']) {
                i10tm.insert(code.to_string(), TRANSLATOR.translate(&sanity_space(detail)));
            }
        }
    }

    let write_bytes = bitcode::encode(&i10tm);
    let path = "crates/benches/dump/i10tm-noex.dump";
    write_to(&write_bytes, path);
}

fn write_to(bytes: &[u8], path: &str) {
    let mut file = File::create(path).unwrap_or_else(|e| {
        panic!("Error creating '{}': {}", path, e);
    });
    if let Err(e) = file.write_all(&bytes) {
        panic!("Error writing to '{}': {}", path, e);
    }
}

fn display_result(results: Vec<((String, String), f32, u8)>) {
    for ((code, detail), r, col) in results {
        println!("{} {:.3} {} {}", col, r, code, detail);
    }
}

fn display_index_result(results: Vec<((String, Arc<I10Pointer>), f32, u8)>) {
    for ((detail, pointer), r, col) in results {
        println!("{} {:.3} {} {:?}", col, r, detail, pointer);
    }
}

fn display_i10_result(results: Vec<(Arc<I10Keywords>, f32, u8)>) {
    for (i10, r, col) in results {
        let (code, pad) = if matches!(i10.rubric_kind, RubricKind::Exclusion) {
            (i10.references.iter().map(|r| r.label.replace('.', "")).collect::<Vec<String>>().concat(), i10.class_code.to_owned())
        } else {
            (i10.class_code.to_owned(), String::new())
        };
        println!("{} {:.3} {} {} {}", col, r, code, &i10.detail, pad);
    }
}

fn display_vx_result(results: Vec<((String, Arc<I10vx>), f32, u8)>) {
    for ((code, vx), r, col) in results {
        println!("{} {:.3} {} {}", col, r, code, vx.desc);
    }
}
