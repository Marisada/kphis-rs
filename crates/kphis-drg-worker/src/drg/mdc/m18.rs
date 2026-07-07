// Book 2 pdf page 304

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
    uorp::process_uorp,
};
use std::collections::HashSet;

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    if let Some(uorp_res) = process_uorp(Mdc::M18, grouper, input) {
        uorp_res.process(grouper, input)
    } else {
        process_pdx(grouper, &input.pdx, &input.procs, &input.dch_type, &input.age_y)
    }
}

fn process_pdx(grouper: &Grouper, pdx: &str, sdxs: &HashSet<String>, dch_type: &str, age_y: &u8) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M18, "18A", pdx) {
        if *age_y > 14 {
            if dch_type == "04" {
                MdcResult::Dc(String::from("1872"))
            } else {
                MdcResult::Dc(String::from("1850"))
            }
        } else {
            MdcResult::Dc(String::from("1851"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18B", pdx) {
        if *age_y > 54 { MdcResult::Dc(String::from("1852")) } else { MdcResult::Dc(String::from("1853")) }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18C", pdx) {
        if *age_y > 14 { MdcResult::Dc(String::from("1854")) } else { MdcResult::Dc(String::from("1855")) }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18D", pdx) {
        if grouper.has_mdc_ax_pdx_or_sdxs("18BX", pdx, sdxs) {
            MdcResult::Dc(String::from("1870"))
        } else if *age_y > 14 {
            MdcResult::Dc(String::from("1856"))
        } else {
            MdcResult::Dc(String::from("1857"))
        }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18E", pdx) {
        if *age_y > 14 { MdcResult::Dc(String::from("1858")) } else { MdcResult::Dc(String::from("1859")) }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18F", pdx) {
        if *age_y > 14 { MdcResult::Dc(String::from("1860")) } else { MdcResult::Dc(String::from("1861")) }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18G", pdx) {
        if *age_y > 14 { MdcResult::Dc(String::from("1862")) } else { MdcResult::Dc(String::from("1863")) }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18H", pdx) {
        if *age_y > 14 { MdcResult::Dc(String::from("1864")) } else { MdcResult::Dc(String::from("1865")) }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18J", pdx) {
        if *age_y > 14 { MdcResult::Dc(String::from("1866")) } else { MdcResult::Dc(String::from("1867")) }
    } else if grouper.is_pdx_pdc(Mdc::M18, "18K", pdx) {
        if *age_y > 14 { MdcResult::Dc(String::from("1868")) } else { MdcResult::Dc(String::from("1869")) }
    } else {
        MdcResult::Drg(String::from("26509"))
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::drg::grouper::GROUPER;

    #[test]
    fn test_process_pdx() {
        let p18a_o14_refer = process_pdx(&GROUPER, "A021", &HashSet::new(), "04", &20);
        assert_eq!(p18a_o14_refer, MdcResult::Dc(String::from("1872")));
        let p18a_o14 = process_pdx(&GROUPER, "A021", &HashSet::new(), "01", &20);
        assert_eq!(p18a_o14, MdcResult::Dc(String::from("1850")));
        let p18a = process_pdx(&GROUPER, "A021", &HashSet::new(), "01", &10);
        assert_eq!(p18a, MdcResult::Dc(String::from("1851")));

        let p18b_o54 = process_pdx(&GROUPER, "T793", &HashSet::new(), "01", &60);
        assert_eq!(p18b_o54, MdcResult::Dc(String::from("1852")));
        let p18b = process_pdx(&GROUPER, "T793", &HashSet::new(), "01", &20);
        assert_eq!(p18b, MdcResult::Dc(String::from("1853")));

        let p18c_o14 = process_pdx(&GROUPER, "B500", &HashSet::new(), "01", &20);
        assert_eq!(p18c_o14, MdcResult::Dc(String::from("1854")));
        let p18c = process_pdx(&GROUPER, "B500", &HashSet::new(), "01", &10);
        assert_eq!(p18c, MdcResult::Dc(String::from("1855")));

        let p18d_bx = process_pdx(&GROUPER, "A90", &["A910"].into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>(), "04", &20);
        assert_eq!(p18d_bx, MdcResult::Dc(String::from("1870")));
        let p18d_o14 = process_pdx(&GROUPER, "A90", &HashSet::new(), "01", &20);
        assert_eq!(p18d_o14, MdcResult::Dc(String::from("1856")));
        let p18d = process_pdx(&GROUPER, "A90", &HashSet::new(), "01", &10);
        assert_eq!(p18d, MdcResult::Dc(String::from("1857")));

        let p18e_o14 = process_pdx(&GROUPER, "R508", &HashSet::new(), "01", &20);
        assert_eq!(p18e_o14, MdcResult::Dc(String::from("1858")));
        let p18e = process_pdx(&GROUPER, "R508", &HashSet::new(), "01", &10);
        assert_eq!(p18e, MdcResult::Dc(String::from("1859")));

        let p18f_o14 = process_pdx(&GROUPER, "Z21", &HashSet::new(), "01", &20);
        assert_eq!(p18f_o14, MdcResult::Dc(String::from("1860")));
        let p18f = process_pdx(&GROUPER, "Z21", &HashSet::new(), "01", &10);
        assert_eq!(p18f, MdcResult::Dc(String::from("1861")));

        let p18g_o14 = process_pdx(&GROUPER, "B377", &HashSet::new(), "01", &20);
        assert_eq!(p18g_o14, MdcResult::Dc(String::from("1862")));
        let p18g = process_pdx(&GROUPER, "B377", &HashSet::new(), "01", &10);
        assert_eq!(p18g, MdcResult::Dc(String::from("1863")));

        let p18h_o14 = process_pdx(&GROUPER, "B550", &HashSet::new(), "01", &20);
        assert_eq!(p18h_o14, MdcResult::Dc(String::from("1864")));
        let p18h = process_pdx(&GROUPER, "B550", &HashSet::new(), "01", &10);
        assert_eq!(p18h, MdcResult::Dc(String::from("1865")));

        let p18j_o14 = process_pdx(&GROUPER, "A241", &HashSet::new(), "01", &20);
        assert_eq!(p18j_o14, MdcResult::Dc(String::from("1866")));
        let p18j = process_pdx(&GROUPER, "A241", &HashSet::new(), "01", &10);
        assert_eq!(p18j, MdcResult::Dc(String::from("1867")));

        let p18k_o14 = process_pdx(&GROUPER, "A270", &HashSet::new(), "01", &20);
        assert_eq!(p18k_o14, MdcResult::Dc(String::from("1868")));
        let p18k = process_pdx(&GROUPER, "A270", &HashSet::new(), "01", &10);
        assert_eq!(p18k, MdcResult::Dc(String::from("1869")));
    }
}
