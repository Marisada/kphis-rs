// Book 2 pdf page 324

use super::Mdc;
use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
};

pub(crate) fn process(grouper: &Grouper, input: &GrouperInput) -> MdcResult {
    process_pdx(grouper, &input.pdx)
}

fn process_pdx(grouper: &Grouper, pdx: &str) -> MdcResult {
    if grouper.is_pdx_pdc(Mdc::M20, "20A", pdx) {
        MdcResult::Dc(String::from("2050"))
    } else if grouper.is_pdx_pdc(Mdc::M20, "20B", pdx) {
        MdcResult::Dc(String::from("2051"))
    } else if grouper.is_pdx_pdc(Mdc::M20, "20C", pdx) {
        MdcResult::Dc(String::from("2052"))
    } else if grouper.is_pdx_pdc(Mdc::M20, "20D", pdx) {
        MdcResult::Dc(String::from("2053"))
    } else if grouper.is_pdx_pdc(Mdc::M20, "20E", pdx) {
        MdcResult::Dc(String::from("2054"))
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
        let p20a = process_pdx(&GROUPER, "F100");
        assert_eq!(p20a, MdcResult::Dc(String::from("2050")));
        let p20b = process_pdx(&GROUPER, "F113");
        assert_eq!(p20b, MdcResult::Dc(String::from("2051")));
        let p20c = process_pdx(&GROUPER, "F101");
        assert_eq!(p20c, MdcResult::Dc(String::from("2052")));
        let p20d = process_pdx(&GROUPER, "F110");
        assert_eq!(p20d, MdcResult::Dc(String::from("2053")));
        let p20e = process_pdx(&GROUPER, "F120");
        assert_eq!(p20e, MdcResult::Dc(String::from("2054")));
    }
}
