// Book 2 pdf page 262

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum M14R {
    R1,
    R2,
    R3,
    Rx,
}

impl M14R {
    fn process(&self, grouper: &Grouper, input: &GrouperInput) -> MdcResult {
        match self {
            Self::R1 => process_r1(grouper, input),
            Self::R2 => process_r2(grouper, input),
            Self::R3 => process_r3(grouper, input),
            Self::Rx => process_rx(grouper, &input.pdx, &input.procs),
        }
    }
}

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    let route = process_main(grouper, input);
    route.process(grouper, input)
}

fn process_main(grouper: &Grouper, input: &GrouperInput) -> M14R {
    if grouper.is_pdx_pdc(Mdc::M14, "14A", &input.pdx) {
        M14R::R1
    } else if grouper.is_pdx_pdc(Mdc::M14, "14B", &input.pdx) {
        if grouper.has_mdc_ax_sdxs("14BX", &input.sdxs) {
            M14R::R1
        } else if grouper.has_mdc_ax_sdxs("14EX", &input.sdxs) {
            M14R::R2
        } else {
            M14R::R3
        }
    } else if grouper.is_pdx_pdc(Mdc::M14, "14C", &input.pdx) {
        if grouper.has_mdc_ax_sdxs("14BX", &input.sdxs) { M14R::R1 } else { M14R::R2 }
    } else if grouper.is_pdx_pdc(Mdc::M14, "14D", &input.pdx) {
        M14R::R2
    } else if grouper.is_pdx_pdc(Mdc::M14, "14E", &input.pdx) {
        M14R::R3
    } else {
        M14R::Rx
    }
}

fn process_r1(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_mdc_pax("14PBX", &input.procs) {
        MdcResult::Dc(String::from("1401"))
    } else if grouper.has_mdc_pax("14PCX", &input.procs) {
        MdcResult::Dc(String::from("1402"))
    } else if grouper.has_mdc_pax("14PGX", &input.procs) {
        MdcResult::Dc(String::from("1407"))
    } else if let Some(uorp_res) = process_uorp(Mdc::M14, grouper, input) {
        uorp_res.process(grouper, input)
    } else if grouper.has_mdc_pax("14PHX", &input.procs) {
        MdcResult::Dc(String::from("1408"))
    } else {
        MdcResult::Dc(String::from("1450"))
    }
}

fn process_r2(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_mdc_pax("14PDX", &input.procs) {
        MdcResult::Dc(String::from("1404"))
    } else if let Some(uorp_res) = process_uorp(Mdc::M14, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        MdcResult::Dc(String::from("1451"))
    }
}

fn process_r3(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if grouper.has_mdc_pax("14PDX", &input.procs) {
        MdcResult::Dc(String::from("1404"))
    } else if let Some(uorp_res) = process_uorp(Mdc::M14, grouper, input) {
        uorp_res.process(grouper, input)
    } else if grouper.has_mdc_ax_pdx("14FX", &input.pdx) {
        MdcResult::Dc(String::from("1459"))
    } else if grouper.has_mdc_ax_pdx("14LX", &input.pdx) {
        MdcResult::Dc(String::from("1460"))
    } else if grouper.has_mdc_ax_pdx("14MX", &input.pdx) {
        check_reassign(grouper, &input.pdx, &input.sdxs, &input.gender)
    } else {
        MdcResult::Dc(String::from("1452"))
    }
}

fn process_rx(grouper: &Grouper, pdx: &str, procs: &HashSet<String>) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M14, "14F", pdx) {
        if grouper.has_mdc_pax("14PDX", procs) {
            MdcResult::Dc(String::from("1412"))
        } else {
            MdcResult::Dc(String::from("1453"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M14, "14G", pdx) {
        MdcResult::Dc(String::from("1454"))
    } else if grouper.is_pdx_pdc(Mdc::M14, "14H", pdx) || grouper.is_pdx_pdc(Mdc::M14, "14K", pdx) || grouper.is_pdx_pdc(Mdc::M14, "14L", pdx) {
        if grouper.has_mdc_pax("14PDX", procs) {
            MdcResult::Dc(String::from("1404"))
        } else if grouper.is_pdx_pdc(Mdc::M14, "14H", pdx) {
            if grouper.has_mdc_pax("14PEX", procs) {
                MdcResult::Dc(String::from("1405"))
            } else {
                MdcResult::Dc(String::from("1455"))
            }
        } else if grouper.is_pdx_pdc(Mdc::M14, "14K", pdx) {
            if grouper.has_mdc_pax("14PEX", procs) {
                MdcResult::Dc(String::from("1410"))
            } else {
                MdcResult::Dc(String::from("1457"))
            }
        } else if grouper.is_pdx_pdc(Mdc::M14, "14L", pdx) {
            if grouper.has_mdc_pax("14PEX", procs) {
                MdcResult::Dc(String::from("1411"))
            } else {
                MdcResult::Dc(String::from("1458"))
            }
        } else {
            MdcResult::Drg(String::from("26529"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M14, "14J", pdx) {
        MdcResult::Dc(String::from("1456"))
    } else {
        MdcResult::Drg(String::from("26529"))
    }
}

fn check_reassign(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, gender: &Option<String>) -> MdcResult {
    match pdx {
        "O980" => {
            // A150–A199
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| sdx.starts_with("A15") || sdx.starts_with("A16") || sdx.starts_with("A17") || sdx.starts_with("A18") || sdx.starts_with("A19"))
                    .max(),
            )
        }
        "O981" => {
            // A500-A539
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| sdx.starts_with("A50") || sdx.starts_with("A51") || sdx.starts_with("A52") || sdx.starts_with("A53"))
                    .max(),
            )
        }
        "O982" => {
            // A540-A549
            reassign(grouper, gender, sdxs.iter().filter(|sdx| sdx.starts_with("A54")).max())
        }
        "O983" => {
            // A55-A64
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| {
                        sdx.starts_with("A55")
                            || sdx.starts_with("A56")
                            || sdx.starts_with("A57")
                            || sdx.starts_with("A58")
                            || sdx.starts_with("A59")
                            || sdx.starts_with("A60")
                            || sdx.starts_with("A63")
                            || sdx.starts_with("A64")
                    })
                    .max(),
            )
        }
        "O984" => {
            // B150-B199
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| sdx.starts_with("B15") || sdx.starts_with("B16") || sdx.starts_with("B17") || sdx.starts_with("B18") || sdx.starts_with("B19"))
                    .max(),
            )
        }
        "O985" => {
            // A800-B09, B250-B349
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| {
                        sdx.starts_with("A8")
                            || sdx.starts_with("A9")
                            || sdx.starts_with("B0")
                            || sdx.starts_with("B25")
                            || sdx.starts_with("B26")
                            || sdx.starts_with("B27")
                            || sdx.starts_with("B30")
                            || sdx.starts_with("B33")
                            || sdx.starts_with("B34")
                    })
                    .max(),
            )
        }
        "O986" => {
            // B500-B64
            reassign(
                grouper,
                gender,
                sdxs.iter().filter(|sdx| sdx.starts_with("B5") || sdx.starts_with("B60") || sdx.starts_with("B64")).max(),
            )
        }
        "O988" => {
            // A000-A099, A200-A499, A65-A799, B350-B49, B650-B949, B99
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| {
                        sdx.starts_with("A0")
                            || sdx.starts_with("A2")
                            || sdx.starts_with("A3")
                            || sdx.starts_with("A4")
                            || sdx.starts_with("A65")
                            || sdx.starts_with("A66")
                            || sdx.starts_with("A67")
                            || sdx.starts_with("A68")
                            || sdx.starts_with("A69")
                            || sdx.starts_with("A7")
                            || sdx.starts_with("B35")
                            || sdx.starts_with("B36")
                            || sdx.starts_with("B37")
                            || sdx.starts_with("B38")
                            || sdx.starts_with("B39")
                            || sdx.starts_with("B4")
                            || sdx.starts_with("B65")
                            || sdx.starts_with("B66")
                            || sdx.starts_with("B67")
                            || sdx.starts_with("B68")
                            || sdx.starts_with("B69")
                            || sdx.starts_with("B7")
                            || sdx.starts_with("B8")
                            || sdx.starts_with("B90")
                            || sdx.starts_with("B91")
                            || sdx.starts_with("B92")
                            || sdx.starts_with("B94")
                            || sdx.starts_with("B99")
                    })
                    .max(),
            )
        }
        "O990" => {
            // D500-D649
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| sdx.starts_with("D5") || sdx.starts_with("D60") || sdx.starts_with("D61") || sdx.starts_with("D62") || sdx.starts_with("D63") || sdx.starts_with("D64"))
                    .max(),
            )
        }
        "O991" => {
            // D65-D899
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| {
                        sdx.starts_with("D65") || sdx.starts_with("D66") || sdx.starts_with("D67") || sdx.starts_with("D68") || sdx.starts_with("D69") || sdx.starts_with("D7") || sdx.starts_with("D8")
                    })
                    .max(),
            )
        }
        "O992" => {
            // E000-E90
            reassign(grouper, gender, sdxs.iter().filter(|sdx| sdx.starts_with("E")).max())
        }
        "O993" => {
            // F000-F99 (except F530, F531), G000-G998
            reassign(grouper, gender, sdxs.iter().filter(|sdx| sdx.starts_with("F")).max())
        }
        "O994" => {
            // I00-I99
            reassign(grouper, gender, sdxs.iter().filter(|sdx| sdx.starts_with("I")).max())
        }
        "O995" => {
            // J00-J998
            reassign(grouper, gender, sdxs.iter().filter(|sdx| sdx.starts_with("J")).max())
        }
        "O996" => {
            // K000-K938
            reassign(grouper, gender, sdxs.iter().filter(|sdx| sdx.starts_with("K")).max())
        }
        "O997" => {
            // L00-L998
            reassign(grouper, gender, sdxs.iter().filter(|sdx| sdx.starts_with("L")).max())
        }
        "O998" => {
            // D500-D649, D65-D899, E000-E90, F00-F99 (Except F530-F531), G000-G998, I00-I99, J00-J998, K000-K938, L00-L998, C000-D489, H000-H959, M000-M9999, N000-N999, Q000-Q999
            reassign(
                grouper,
                gender,
                sdxs.iter()
                    .filter(|sdx| {
                        sdx.starts_with("D5")
                            || sdx.starts_with("D6")
                            || sdx.starts_with("E")
                            || (sdx.starts_with("F") && *sdx != "F530" && *sdx != "F531")
                            || sdx.starts_with("G")
                            || sdx.starts_with("I")
                            || sdx.starts_with("J")
                            || sdx.starts_with("K")
                            || sdx.starts_with("L")
                            || sdx.starts_with("C")
                            || sdx.starts_with("D0")
                            || sdx.starts_with("D1")
                            || sdx.starts_with("D2")
                            || sdx.starts_with("D3")
                            || sdx.starts_with("D4")
                            || sdx.starts_with("H")
                            || sdx.starts_with("M")
                            || sdx.starts_with("N")
                            || sdx.starts_with("Q")
                    })
                    .max(),
            )
        }
        _ => MdcResult::Dc(String::from("1452")),
    }
}

fn reassign(grouper: &Grouper, gender: &Option<String>, new_pdx_opt: Option<&String>) -> MdcResult {
    if let Some(new_pdx) = new_pdx_opt {
        if let Some(mdc) = grouper.i10(new_pdx, gender).and_then(|i10| Mdc::new(&i10.mdc)) {
            MdcResult::MdcNewPdx(mdc, new_pdx.to_owned())
        } else {
            MdcResult::Dc(String::from("1452"))
        }
    } else {
        MdcResult::Dc(String::from("1452"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_main() {
        let mut input = GrouperInput::default();

        input.set_pdx("O151");
        let p14a = process_main(&GROUPER, &input);
        assert_eq!(p14a, M14R::R1);

        input.set_pdx("O100");
        let p14b = process_main(&GROUPER, &input);
        assert_eq!(p14b, M14R::R3);
        input.set_sdxs(&["O800"]);
        let p14b_dev = process_main(&GROUPER, &input);
        assert_eq!(p14b_dev, M14R::R1);
        input.set_sdxs(&["Z390"]);
        let p14b_pp = process_main(&GROUPER, &input);
        assert_eq!(p14b_pp, M14R::R2);

        input.set_pdx("O152");
        input.set_sdxs(&[]);
        let p14c = process_main(&GROUPER, &input);
        assert_eq!(p14c, M14R::R2);
        input.set_sdxs(&["O800"]);
        let p14c_dev = process_main(&GROUPER, &input);
        assert_eq!(p14c_dev, M14R::R1);

        input.set_pdx("O080");
        let p14d = process_main(&GROUPER, &input);
        assert_eq!(p14d, M14R::R2);

        input.set_pdx("O280");
        let p14e = process_main(&GROUPER, &input);
        assert_eq!(p14e, M14R::R3);

        input.set_pdx("O000");
        let p14f = process_main(&GROUPER, &input);
        assert_eq!(p14f, M14R::Rx);
    }

    #[test]
    fn test_process_r1() {
        let mut input = GrouperInput::default();

        input.set_procs(&["740"]);
        let p14pbx = process_r1(&GROUPER, &input);
        assert_eq!(p14pbx, MdcResult::Dc(String::from("1401")));

        input.set_procs(&["4871"]);
        let p14pcx = process_r1(&GROUPER, &input);
        assert_eq!(p14pcx, MdcResult::Dc(String::from("1402")));

        input.set_procs(&["6621"]);
        let p14pgx = process_r1(&GROUPER, &input);
        assert_eq!(p14pgx, MdcResult::Dc(String::from("1407")));

        input.set_procs(&["7029"]);
        let p14phx = process_r1(&GROUPER, &input);
        assert_eq!(p14phx, MdcResult::Dc(String::from("1408")));

        input.set_procs(&[]);
        let p14 = process_r1(&GROUPER, &input);
        assert_eq!(p14, MdcResult::Dc(String::from("1450")));
    }

    #[test]
    fn test_process_r2() {
        let mut input = GrouperInput::default();

        input.set_procs(&["5411"]);
        let p14pdx = process_r2(&GROUPER, &input);
        assert_eq!(p14pdx, MdcResult::Dc(String::from("1404")));

        input.set_procs(&[]);
        let p14 = process_r2(&GROUPER, &input);
        assert_eq!(p14, MdcResult::Dc(String::from("1451")));
    }

    #[test]
    fn test_process_r3() {
        let mut input = GrouperInput::default();

        input.set_procs(&["5411"]);
        let p14pdx = process_r3(&GROUPER, &input);
        assert_eq!(p14pdx, MdcResult::Dc(String::from("1404")));

        input.set_procs(&[]);
        input.set_pdx("O102");
        let p14fx = process_r3(&GROUPER, &input);
        assert_eq!(p14fx, MdcResult::Dc(String::from("1459")));

        input.set_pdx("O230");
        let p14lx = process_r3(&GROUPER, &input);
        assert_eq!(p14lx, MdcResult::Dc(String::from("1460")));

        input.set_pdx("O980");
        let p14mx = process_r3(&GROUPER, &input);
        assert_eq!(p14mx, MdcResult::Dc(String::from("1452")));
        input.set_sdxs(&["A150", "A199"]);
        let p14mx_re = process_r3(&GROUPER, &input);
        assert_eq!(p14mx_re, MdcResult::MdcNewPdx(Mdc::M18, String::from("A199")));

        input.set_pdx("");
        input.set_sdxs(&[]);
        input.set_procs(&[]);
        let p14 = process_r3(&GROUPER, &input);
        assert_eq!(p14, MdcResult::Dc(String::from("1452")));
    }

    #[test]
    fn test_process_rx() {
        let p14f_ectopic = process_rx(&GROUPER, "O001", &["5411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p14f_ectopic, MdcResult::Dc(String::from("1412")));
        let p14f = process_rx(&GROUPER, "O001", &HashSet::new());
        assert_eq!(p14f, MdcResult::Dc(String::from("1453")));
        let p14g = process_rx(&GROUPER, "O200", &HashSet::new());
        assert_eq!(p14g, MdcResult::Dc(String::from("1454")));

        let p14h_ectopic = process_rx(&GROUPER, "O020", &["5411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p14h_ectopic, MdcResult::Dc(String::from("1404")));
        let p14h_dc = process_rx(&GROUPER, "O020", &["6901"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p14h_dc, MdcResult::Dc(String::from("1405")));
        let p14h = process_rx(&GROUPER, "O020", &HashSet::new());
        assert_eq!(p14h, MdcResult::Dc(String::from("1455")));

        let p14k_ectopic = process_rx(&GROUPER, "O021", &["5411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p14k_ectopic, MdcResult::Dc(String::from("1404")));
        let p14k_dc = process_rx(&GROUPER, "O021", &["6901"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p14k_dc, MdcResult::Dc(String::from("1410")));
        let p14k = process_rx(&GROUPER, "O021", &HashSet::new());
        assert_eq!(p14k, MdcResult::Dc(String::from("1457")));

        let p14l_ectopic = process_rx(&GROUPER, "O010", &["5411"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p14l_ectopic, MdcResult::Dc(String::from("1404")));
        let p14l_dc = process_rx(&GROUPER, "O010", &["6901"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>());
        assert_eq!(p14l_dc, MdcResult::Dc(String::from("1411")));
        let p14l = process_rx(&GROUPER, "O010", &HashSet::new());
        assert_eq!(p14l, MdcResult::Dc(String::from("1458")));

        let p14j = process_rx(&GROUPER, "O470", &HashSet::new());
        assert_eq!(p14j, MdcResult::Dc(String::from("1456")));
    }
}
