use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    sync::Arc,
};

use kphis_drg_worker::drg::{
    grouper::Grouper,
    model::{CcEx, DaggerAsterisk, DcPclDrg, Dcl, DclEq, Drg, I9vx, I10, I10e, I10vx, MdcAx, MdcPax, MdcPdc, MdcPpdc, Proc},
};

pub fn new_grouper() -> Grouper {
    let mut i10_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/i10.csv")[..]);
    let mut i10 = HashMap::new();
    for i in i10_rdr.deserialize::<I10>() {
        let row = Arc::new(i.expect("invalid /raw-grouper/i10.csv, please run raw_parser and try again"));
        let row_clone = row.clone();
        i10.entry(row.code.to_owned()).and_modify(|v: &mut I10e| *v = v.to_pair(row)).or_insert(I10e::new(row_clone));
    }

    // let mut i10tm_rdr = csv::Reader::from_reader(&include_bytes!("../raw-icd-tm/icd-10-tm2016-20210805.csv")[..]);
    // let mut i10tm = HashMap::new();
    // let mut i10tm_ex = HashMap::new();
    // for record in i10tm_rdr.records() {
    //     let row = record.expect("invalid icd-10-tm2016-20210805.csv");
    //     // // check row start with A-Z
    //     // let row_str = row.as_slice();
    //     // if !row_str.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or_default() {
    //     //     panic!("{} not start with A-Z", row_str);
    //     // }
    //     if let (Some(code), Some(detail)) = (row.get(0), row.get(1)) {
    //         if code.starts_with(&['V','W','X','Y']) {
    //             i10tm_ex.insert(code.to_string(), detail.to_string());
    //         } else {
    //             i10tm.insert(code.to_string(), TRANSLATOR.translate(&sanity_space(detail)));
    //         }
    //     }
    // }

    let mut i10vx_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/i10vx.csv")[..]);
    let mut i10vx = BTreeMap::new();
    let mut i10vx_ex = BTreeMap::new();
    for i in i10vx_rdr.deserialize::<I10vx>() {
        let row = Arc::new(i.expect("invalid /raw-grouper/i10vx.csv, please run raw_parser and try again"));
        if row.code.starts_with(&['V', 'W', 'X', 'Y']) {
            i10vx_ex.insert(row.code.to_owned(), row.clone());
        } else {
            i10vx.insert(row.code.to_owned(), row.clone());
        }
    }

    let i9 = csv::Reader::from_reader(&include_bytes!("../raw-grouper/proc.csv")[..])
        .deserialize::<Proc>()
        .map(|r| {
            let proc = r.expect("invalid /raw-grouper/proc.csv, please run raw_parser and try again");
            (proc.proc.to_owned(), Arc::new(proc))
        })
        .collect::<BTreeMap<String, Arc<Proc>>>();

    let mut i9vx = csv::Reader::from_reader(&include_bytes!("../raw-grouper/i9vx.csv")[..])
        .deserialize::<I9vx>()
        .map(|r| {
            let vx = r.expect("invalid /raw-grouper/proc.csv, please run raw_parser and try again");
            (vx.code.to_owned(), Arc::new(vx))
        })
        .collect::<BTreeMap<String, Arc<I9vx>>>();

    // use desc from book, add missed items from book
    crate::i9_cm::fixed_i9vx_desc_with_book(&mut i9vx);

    let mut dagger_asterisk_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/dagger-asterisk.csv")[..]);
    let mut dagger_asterisks = HashMap::new();
    // let mut asterisk_daggers = HashMap::new();
    for i in dagger_asterisk_rdr.deserialize::<DaggerAsterisk>() {
        let row = i.expect("invalid /raw-grouper/dagger-asterisk.csv, please run raw_parser and try again");
        dagger_asterisks.entry(row.dagger.to_owned()).or_insert(HashSet::new()).insert(row.asterisk.to_owned());
        // asterisk_daggers
        //     .entry(row.asterisk.to_owned())
        //     .or_insert(HashSet::new())
        //     .insert(row.dagger.to_owned());
    }

    let mut mdc_pdc = HashMap::new();
    let mut mdc_pdc_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/mdc-pdc.csv")[..]);
    for i in mdc_pdc_rdr.deserialize::<MdcPdc>() {
        let row = i.expect("invalid /raw-grouper/mdc-pdc.csv, please run raw_parser and try again");
        mdc_pdc.entry(row.mdc).or_insert(HashMap::new()).insert(row.code, row.pdc);
    }

    let mut mdc_ppdc = HashMap::new();
    let mut mdc_ppdc_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/mdc-ppdc.csv")[..]);
    for i in mdc_ppdc_rdr.deserialize::<MdcPpdc>() {
        let row = i.expect("invalid /raw-grouper/mdc-ppdc.csv, please run raw_parser and try again");
        mdc_ppdc.entry(row.mdc).or_insert(HashMap::new()).insert(row.proc, row.pdc);
    }

    let mut mdc_ax = HashMap::new();
    let mut mdc_ax_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/mdc-ax.csv")[..]);
    for i in mdc_ax_rdr.deserialize::<MdcAx>() {
        let row = i.expect("invalid /raw-grouper/mdc-ax.csv, please run raw_parser and try again");
        mdc_ax.entry(row.ax.to_owned()).or_insert(HashSet::new()).insert(row.code.to_owned());
    }

    let mut mdc_pax = HashMap::new();
    let mut mdc_pax_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/mdc-pax.csv")[..]);
    for i in mdc_pax_rdr.deserialize::<MdcPax>() {
        let row = i.expect("invalid /raw-grouper/mdc-pax.csv, please run raw_parser and try again");
        mdc_pax.entry(row.pax.to_owned()).or_insert(HashSet::new()).insert(row.proc.to_owned());
    }

    // generate dcls by mutate inner HashMap
    let mut dcl_mut = HashMap::new();
    let mut dcl_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/dcls.csv")[..]);
    for i in dcl_rdr.deserialize::<Dcl>() {
        let row = i.expect("invalid /raw-grouper/dcls.csv, please run raw_parser and try again");
        dcl_mut.entry(row.code).or_insert(HashMap::new()).insert(row.dc, row.dcl);
    }
    // change to Arc for cheap clone
    let mut dcl = dcl_mut.into_iter().map(|(k, v)| (k, Arc::new(v))).collect::<HashMap<String, Arc<HashMap<String, u8>>>>();
    let mut dcl_eq_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/dcl-eq.csv")[..]);
    for i in dcl_eq_rdr.deserialize::<DclEq>() {
        let row = i.expect("invalid /raw-grouper/dcl-eq.csv, please run raw_parser and try again");
        if let Some(ptr) = dcl.get(&row.main) {
            dcl.insert(row.code, ptr.clone());
        }
    }

    let mut ccex = HashMap::new();
    let mut ccex_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/ccex.csv")[..]);
    for i in ccex_rdr.deserialize::<CcEx>() {
        let row = i.expect("invalid /raw-grouper/ccex.csv, please run raw_parser and try again");
        ccex.entry(row.ex.to_owned()).or_insert(HashSet::new()).insert(row.code.to_owned());
    }

    let mut dc_pcl_drg = HashMap::new();
    let mut dc_pcl_drg_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/dc-pcl-drg.csv")[..]);
    for i in dc_pcl_drg_rdr.deserialize::<DcPclDrg>() {
        let row = i.expect("invalid /raw-grouper/dc-pcl-drg.csv, please run raw_parser and try again");
        dc_pcl_drg.entry(row.dc.to_owned()).or_insert(Vec::new()).push(row);
    }

    let drg = csv::Reader::from_reader(&include_bytes!("../raw-grouper/drg.csv")[..])
        .deserialize::<Drg>()
        .map(|r| {
            let row = r.expect("invalid /raw-grouper/drg.csv, please run raw_parser and try again");
            (row.drg.to_owned(), row)
        })
        .collect::<HashMap<String, Drg>>();

    Grouper {
        i10,
        i10vx,
        i10vx_ex,
        i9,
        i9vx,
        dagger_asterisks,
        mdc_pdc,
        mdc_ppdc,
        mdc_ax,
        mdc_pax,
        dcl,
        ccex,
        dc_pcl_drg,
        drg,
    }
}

pub(crate) fn get_grouper_aster_dagger() -> HashMap<String, HashSet<String>> {
    let mut dagger_asterisk_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/dagger-asterisk.csv")[..]);
    let mut grouper_pairs = HashMap::new();
    for i in dagger_asterisk_rdr.deserialize::<DaggerAsterisk>() {
        let row = i.expect("invalid /raw-grouper/dagger-asterisk.csv, please run raw_parser and try again");
        grouper_pairs.entry(row.asterisk.to_owned()).or_insert(HashSet::new()).insert(row.dagger.to_owned());
    }
    grouper_pairs
}

pub(crate) fn book2_parser() {
    let current_dir = std::env::current_dir().unwrap(); // kphis/crates/kphis-dump-builder
    let raw_file = File::open(current_dir.join("raw-grouper/book2.txt")).expect("Failed to open the book2.txt file");
    let reader = BufReader::new(raw_file);

    let pdc_target = File::create(current_dir.join("raw-grouper/mdc-pdc.csv")).expect("Failed to create mdc-pdc.csv file");
    let ppdc_target = File::create(current_dir.join("raw-grouper/mdc-ppdc.csv")).expect("Failed to create mdc-ppdc.csv file");
    let ax_target = File::create(current_dir.join("raw-grouper/mdc-ax.csv")).expect("Failed to create mdc-ax.csv file");
    let pax_target = File::create(current_dir.join("raw-grouper/mdc-pax.csv")).expect("Failed to create mdc-pax.csv file");
    let pcl_target = File::create(current_dir.join("raw-grouper/dc-pcl-drg.csv")).expect("Failed to create dc-pcl-drg.csv file");

    let mut pdc_writer = BufWriter::new(pdc_target);
    let mut ppdc_writer = BufWriter::new(ppdc_target);
    let mut ax_writer = BufWriter::new(ax_target);
    let mut pax_writer = BufWriter::new(pax_target);
    let mut pcl_writer = BufWriter::new(pcl_target);

    writeln!(pdc_writer, r#""mdc","code","pdc""#).expect("cannot write mdc-pdc.csv");
    writeln!(ppdc_writer, r#""mdc","proc","pdc""#).expect("cannot write mdc-ppdc.csv");
    writeln!(ax_writer, r#""ax","code""#).expect("cannot write mdc-ax.csv");
    writeln!(pax_writer, r#""pax","proc""#).expect("cannot write mdc-pax.csv");
    writeln!(pcl_writer, r#""drg","dc","pcl_min","pcl_max""#).expect("cannot write dc-pcl-drg.csv");

    let mut grp = Grp::Unknown;
    let mut prefix = String::new();
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let line = line.as_str();
        // some use `..CODES`, sone use `..CODE`
        if line.contains(" ASSIGNMENT OF ICD-9-CM CODE") {
            // AX ASSIGNMENT OF ICD-9-CM CODES"
            if line.starts_with("AX ") {
                grp = Grp::Pax;
            // MDC XX ASSIGNMENT OF ICD-9-CM CODES
            } else {
                grp = Grp::Ppdc;
            }
        // some use `..CODES`, sone use `..CODE`
        } else if line.contains(" ASSIGNMENT OF ICD-10 CODE") {
            if line.starts_with("AX ") {
                grp = Grp::Ax;
            } else {
                // MDC XX ASSIGNMENT OF ICD-10 CODES
                grp = Grp::Pdc;
            }
        } else if line == "DC and DRG Definition" {
            grp = Grp::Pcl;
        } else {
            match grp {
                Grp::Pdc => {
                    // #1 Pdc is as
                    // Code PDC Desc Code PDC Desc
                    // A395 5R Meningococcal heart disease
                    // B376 5B Candidal endocarditis (I39.8*)
                    // B570 5R Acute Chagas' disease with heart involvement
                    // (I41.2*,I98.1*)
                    // Z450 5R Adjustment and management of cardiac
                    // devices
                    // ______________________________
                    let split = line.split(" ").collect::<Vec<&str>>();
                    if split.len() > 2 {
                        let mut cs = split[0].chars();
                        let is_1st_uppercase = cs.nth(0).map(|c| c.is_ascii_uppercase()).unwrap_or_default();
                        let is_2nd_digit = cs.nth(0).map(|c| c.is_ascii_digit()).unwrap_or_default();
                        if is_1st_uppercase && is_2nd_digit && split[1].starts_with(|c: char| c.is_ascii_digit()) {
                            let mdc = split[1].replace(|c: char| c.is_ascii_uppercase(), "");
                            writeln!(pdc_writer, "{},{},{}", mdc, split[0], split[1]).expect("cannot write mdc-pdc.csv");
                        }
                    }
                }
                Grp::Ppdc => {
                    // #2 Ppdc is as
                    // Code PDC Desc Code PDC Desc
                    // 0050 5PM Impl CRT pacemaker sys
                    // CK027 5PV 3524 & 3528
                    // ______________________________
                    let split = line.split(" ").collect::<Vec<&str>>();
                    if split.len() > 2 {
                        let mut cs1 = split[0].chars();
                        let first1 = cs1.nth(0);
                        let third1 = cs1.nth(1);
                        let is_1_start_digit_or_c = first1.map(|c| c.is_ascii_digit() || c == 'C').unwrap_or_default();
                        let is_1_3td_digit = third1.map(|c| c.is_ascii_digit()).unwrap_or_default();
                        let mut cs2 = split[1].chars();
                        let first2 = cs2.nth(0);
                        let third2 = cs2.nth(1);
                        let is_2_start_digit = first2.map(|c| c.is_ascii_digit()).unwrap_or_default();
                        let is_2_3td_uppercase = third2.map(|c| c.is_ascii_uppercase()).unwrap_or_default();
                        if is_1_start_digit_or_c && is_1_3td_digit && is_2_start_digit && is_2_3td_uppercase {
                            let mdc = split[1].replace(|c: char| c.is_ascii_uppercase(), "");
                            writeln!(ppdc_writer, "{},{},{}", mdc, split[0], split[1]).expect("cannot write mdc-ppdc.csv");
                        }
                    }
                }
                Grp::Ax => {
                    // #3 Ax is as
                    // AX 5BX Heart failure and shock
                    // G463 Brain stem stroke syndrome (I60-I67+) G464 Cerebellar stroke syndrome (I60-I67+)
                    // I110 Hypertensive heart disease with (congestive) heart
                    // failure
                    // AX 5CX AMI major complication
                    // I253 Aneurysm of heart
                    // ______________________________
                    let split = line.split(" ").collect::<Vec<&str>>();
                    if split.len() > 2 && split[0] == "AX" {
                        prefix = split[1].to_string();
                    } else if split.len() > 1 && !prefix.is_empty() {
                        let codes = split
                            .iter()
                            .filter_map(|s| {
                                let mut cs = s.chars();
                                // this get 1st char
                                let is_start_uppercase = cs.nth(0).map(|c| c.is_ascii_uppercase()).unwrap_or_default();
                                // this get 2nd char
                                let is_follow_by_number = cs.nth(0).map(|c| c.is_ascii_digit()).unwrap_or_default();
                                // this find lowercase after 3rd char
                                let has_lower_case = cs.any(|c: char| c.is_ascii_lowercase());
                                (!has_lower_case && is_start_uppercase && is_follow_by_number).then(|| s)
                            })
                            .collect::<Vec<&&str>>();
                        match codes.len() {
                            2 => {
                                writeln!(ax_writer, "{},{}", prefix, codes[0]).expect("cannot write mdc-ax.csv");
                                writeln!(ax_writer, "{},{}", prefix, codes[1]).expect("cannot write mdc-ax.csv");
                            }
                            1 => {
                                writeln!(ax_writer, "{},{}", prefix, codes[0]).expect("cannot write mdc-ax.csv");
                            }
                            0 => {}
                            _ => panic!("AX has code > 2 in one line\n{}", line),
                        }
                    }
                }
                Grp::Pax => {
                    // #4 Pax is as
                    // AX 5PCX PTCA
                    // 0066 PTCA 1755 Translumi coro atherect
                    // AX 5PGX single vessel PTCA
                    // 0040 Procedure-one vessel CK028 0066 & 0040
                    // AX 5PJX Peripheral stent insertion
                    // 0055 Ins d-e stent oth periph
                    // CK028 0066 & 0040
                    // ______________________________
                    let split = line.split(" ").collect::<Vec<&str>>();
                    if split.len() > 2 && split[0] == "AX" {
                        prefix = split[1].to_string();
                    } else if split.len() > 1 && !prefix.is_empty() {
                        let codes = split
                            .iter()
                            .filter_map(|s| {
                                let mut cs = s.chars();
                                // this get 1st char
                                let is_start_number_or_c = cs.nth(0).map(|c| c.is_ascii_digit() || c == 'C').unwrap_or_default();
                                // this get 3rd char
                                let is_follow_by_number = cs.nth(1).map(|c| c.is_ascii_digit()).unwrap_or_default();
                                // this find lowercase after 3rd char
                                let has_lower_case_or_dot = cs.any(|c: char| c.is_ascii_lowercase() || c == '.');
                                (!has_lower_case_or_dot && is_start_number_or_c && is_follow_by_number).then(|| s)
                            })
                            .collect::<Vec<&&str>>();
                        let ccodes = codes.iter().filter(|c| c.starts_with('C')).collect::<Vec<&&&str>>();
                        match ccodes.len() {
                            2 => {
                                writeln!(pax_writer, "{},{}", prefix, ccodes[0]).expect("cannot write mdc-pax.csv");
                                writeln!(pax_writer, "{},{}", prefix, ccodes[0]).expect("cannot write mdc-pax.csv");
                            }
                            1 => {
                                // xxxx desc Cxxxx desc
                                if codes[0] != *ccodes[0] {
                                    writeln!(pax_writer, "{},{}", prefix, codes[0]).expect("cannot write mdc-pax.csv");
                                }
                                // Cxxx desc xxxx desc never exists
                                writeln!(pax_writer, "{},{}", prefix, ccodes[0]).expect("cannot write mdc-pax.csv");
                            }
                            0 => match codes.len() {
                                2 => {
                                    writeln!(pax_writer, "{},{}", prefix, codes[0]).expect("cannot write mdc-pax.csv");
                                    writeln!(pax_writer, "{},{}", prefix, codes[1]).expect("cannot write mdc-pax.csv");
                                }
                                1 => {
                                    writeln!(pax_writer, "{},{}", prefix, codes[0]).expect("cannot write mdc-pax.csv");
                                }
                                0 => {}
                                _ => panic!("AX has proc > 2 in one line\n{}", line),
                            },
                            _ => panic!("AX has combined proc > 2 in one line\n{}", line),
                        }
                    }
                }
                Grp::Pcl => {
                    // #5 Pcl is as
                    // DC 0501 Valve replacement and open valvuloplasty w cath
                    // Proc as PDC 5PA and Proc as AX 5PBX
                    // DRG 05019: DC 0501
                    // DC 0502 Valve replacement and open valvuloplasty
                    // Proc as PDC 5PA
                    // DRG 05020: DC 0502 w PCL 0 - 4
                    // DRG 05021: DC 0502 w PCL 5
                    // DRG 05022: DC 0502 w PCL 6 - 9
                    if line.starts_with("DRG ") {
                        let r = line.replace("DRG ", "").replace(" DC ", "").replace(" w PCL ", ":").replace(" - ", ":");
                        let s = r.split(':').collect::<Vec<&str>>();
                        match s.len() {
                            2 => {
                                writeln!(pcl_writer, "{},{},0,9", s[0], s[1]).expect("cannot write dc-pcl-drg.csv");
                            }
                            3 => {
                                writeln!(pcl_writer, "{},{},{},{}", s[0], s[1], s[2], s[2]).expect("cannot write dc-pcl-drg.csv");
                            }
                            4 => {
                                writeln!(pcl_writer, "{},{},{},{}", s[0], s[1], s[2], s[3]).expect("cannot write dc-pcl-drg.csv");
                            }
                            _ => {
                                panic!("pcl len =1 or >4\n{}", line);
                            }
                        }
                    }
                }
                Grp::Unknown => {}
            }
        }
    }
}

pub(crate) fn dcl_from_raw() {
    // Thai DRG Version 6.3.3
    // 2
    // Appendix F1
    // DCL Table
    // Code Code Code Code Code Code Code Code Code Code
    // DC:DCL DC:DCL DC:DCL DC:DCL DC:DCL DC:DCL DC:DCL DC:DCL DC:DCL DC:DCL
    // A000
    // 0010:1
    // 0011:2
    // A001
    // = A000
    // A009
    // = A000
    // A010
    // 0011:1
    // 0012:1
    // Appendix F1 (Dx Codes A-E)
    // 3
    // 1105:2
    // 1106:1
    // ---------------
    // Needed before run this script
    // 1. Change all `\n=` to `=` ex: `A001\n= A000` to `A001=A000`
    let current_dir = std::env::current_dir().unwrap(); // kphis/crates/kphis-dump-builder
    let raw_file = File::open(current_dir.join("raw-grouper/dcl-14.txt")).expect("Failed to open the dcl-14.txt file");
    let target_dcls = File::create(current_dir.join("raw-grouper/dcls.csv")).expect("Failed to create dcls.csv file");
    let target_dcl_eq = File::create(current_dir.join("raw-grouper/dcl-eq.csv")).expect("Failed to open dcl-eq.csv file");
    let reader = BufReader::new(raw_file);
    let mut dcls_writer = BufWriter::new(target_dcls);
    let mut dcl_eq_writer = BufWriter::new(target_dcl_eq);
    writeln!(dcls_writer, r#""code","dc","dcl""#).expect("cannot write dcls.csv");
    writeln!(dcl_eq_writer, r#""code","main""#).expect("cannot write dcl-eq.csv");
    let mut icd = String::new();
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let split = line.split(' ').collect::<Vec<&str>>();
        if split.len() == 1 {
            let mut cs = split[0].chars();
            let is_1st_uppercase = cs.nth(0).map(|c| c.is_ascii_uppercase()).unwrap_or_default();
            let is_2nd_digit = cs.nth(0).map(|c| c.is_ascii_digit()).unwrap_or_default();
            if line.contains('=') {
                let r = line.replace('=', ",");
                writeln!(dcl_eq_writer, "{}", r).expect("cannot write dcl-eq.csv");
            } else if is_1st_uppercase && is_2nd_digit {
                icd = line.clone();
            } else if line.contains(":") {
                if !icd.is_empty() {
                    let r = line.replace(':', ",");
                    writeln!(dcls_writer, "{},{}", icd, r).expect("cannot write dcls.csv");
                }
            }
        }
    }
}

enum Grp {
    Pdc,
    Ppdc,
    Ax,
    Pax,
    Pcl,
    Unknown,
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use std::io::Read;
    use super::*;
    use crate::write_to;

    #[test]
    fn test_book2_parser() {
        book2_parser()
    }

    #[test]
    fn test_dcl_from_raw() {
        dcl_from_raw()
    }

    #[test]
    fn test_dump_grouper() {
        let write_grouper = new_grouper();
        let write_bytes = bitcode::encode(&write_grouper);
        // dbg!(std::env::current_dir().unwrap()); // kphis/crates/kphis-dump-builder
        let path = "../kphis-drg-worker/dump/grouper.dump";
        write_to(&write_bytes, path);

        let mut read_file = File::open(path).unwrap();
        let mut read_bytes = Vec::new();
        read_file.read_to_end(&mut read_bytes).unwrap();
        let read_grouper = bitcode::decode::<Grouper>(&read_bytes).unwrap();

        let i10_len = 14925;
        assert_eq!(read_grouper.i10.len(), i10_len);
        assert_eq!(write_grouper.i10.len() ,i10_len);

        // codes in i10 is a subset of i10vx (i10vx has more invalid code)
        let i10_keys = read_grouper.i10.keys().collect::<HashSet<&String>>();
        let i10vx_keys = read_grouper.i10vx.keys().collect::<HashSet<&String>>();
        let diffs = i10_keys.difference(&i10vx_keys).collect::<Vec<&&String>>();
        assert!(diffs.is_empty());

        let i10vx_len = read_grouper.i10vx.len();
        // without external causes
        assert_eq!(i10vx_len, 16376);

        // i9vx code max length is 4
        assert!(read_grouper.i9vx.keys().all(|code| code.len() <= 4));
    }

    #[test]
    fn test_diff_i10vx_tm() {
        let verbose = false;

        // https://icd.who.int/browse10/2016/en
        // http://www.thcc.or.th/ebook1/2016/mobile/index.html#p=1

        use kphis_util::{british_american::TRANSLATOR, util::sanity_space};

        let mut i10tm_rdr = csv::Reader::from_reader(&include_bytes!("../raw-icd-tm/icd-10-tm2016-20210805.csv")[..]);
        let mut i10tm = HashMap::new();
        for record in i10tm_rdr.records() {
            let row = record.expect("invalid icd-10-tm2016-20210805.csv");
            if let (Some(code), Some(detail)) = (row.get(0), row.get(1)) {
                i10tm.insert(code.to_string(), TRANSLATOR.translate(&sanity_space(detail)));
            }
        }

        let mut i10vx_rdr = csv::Reader::from_reader(&include_bytes!("../raw-grouper/i10vx.csv")[..]);
        let mut i10vx = HashMap::new();
        for i in i10vx_rdr.deserialize::<I10vx>() {
            let row = Arc::new(i.expect("invalid /raw-grouper/i10vx.csv, please run raw_parser and try again"));
            i10vx.insert(row.code.to_owned(), row.clone());
        }

        
        let mut vx_codes = i10vx.keys().collect::<Vec<&String>>();
        let mut tm_codes = i10tm.keys().collect::<Vec<&String>>();
        vx_codes.sort();
        tm_codes.sort();

        // All codes in vx checking
        if verbose {
            // print different
            for code in vx_codes.iter() { // .take(40000) {
                if let (Some(tm_desc), Some(vx_desc)) = (i10tm.get(*code), i10vx.get(*code)) {
                    if vx_desc.desc != *tm_desc {
                        
                        let mut vx_padded = vx_desc.desc.clone();
                        let mut tm_padded = tm_desc.clone();
                        let vx_len = vx_desc.desc.len();
                        let tm_len = tm_desc.len();

                        if tm_len > vx_len {
                            vx_padded.push_str(&vec![" "; tm_len - vx_len].concat());
                        }
                        if vx_len > tm_len {
                            tm_padded.push_str(&vec![" "; vx_len - tm_len].concat());
                        }

                        let mut found = false;
                        let mut stop = false;
                        let mut a_word = Vec::new();
                        let mut b_word = Vec::new();
                        let mut i = None;
                        for (j, (x, y)) in vx_padded.chars().zip(tm_padded.chars()).enumerate() {
                            if !stop {
                                a_word.push(x);
                                b_word.push(y);
                                if x != y {
                                    found = true;
                                    if i.is_none() {
                                        i = Some(j);
                                    }
                                }
                                if x == ' ' || y == ' ' {
                                    if found {
                                        stop = true;
                                    } else {
                                        a_word.clear();
                                        b_word.clear();
                                    }
                                }
                            }
                        }
                        let a: String = a_word.iter().collect();
                        let b: String = b_word.iter().collect();
                        println!("{} :{} '{}' != '{}'", code, i.unwrap_or_default(), a, b);
                    }
                } // else {
                //    println!("{} not exists in i10tm", code);
                // }
            }
        } else {
            assert!(!vx_codes.iter().any(|code| {
                if let (Some(tm_desc), Some(vx_desc)) = (i10tm.get(*code), i10vx.get(*code)) {
                    vx_desc.desc != *tm_desc
                } else {
                    false
                }
            }));
        }

        if verbose {
            // As contents in TDS6306_EXCLUDES.md
            for code in tm_codes {
                if !vx_codes.contains(&code) {
                    if let Some(tm) = i10tm.get(code) {
                        println!("{},\"{}\"", code, tm);
                    }
                }
            }
        }

    }
}
