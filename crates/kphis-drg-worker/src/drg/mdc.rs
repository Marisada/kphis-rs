mod m00;
mod m01;
mod m02;
mod m03;
mod m04;
mod m05;
mod m06;
mod m07;
mod m08;
mod m09;
mod m10;
mod m11;
mod m12;
mod m13;
mod m14;
mod m15;
mod m16;
mod m17;
mod m18;
mod m19;
mod m20;
mod m21;
mod m22;
mod m23;
mod m24;
mod m25;

use serde_derive::{Deserialize, Serialize};

use crate::drg::{
    grouper::Grouper,
    model::{GrouperInput, MdcResult},
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub(crate) enum Mdc {
    M00,
    M01,
    M02,
    M03,
    M04,
    M05,
    M06,
    M07,
    M08,
    M09,
    M10,
    M11,
    M12,
    M13,
    M14,
    M15,
    M16,
    M17,
    M18,
    M19,
    M20,
    M21,
    M22,
    M23,
    M24,
    M25,
    M26,
    M30,
}

impl Mdc {
    pub(crate) fn new(s: &str) -> Option<Self> {
        match s {
            "00" => Some(Self::M00),
            "01" => Some(Self::M01),
            "02" => Some(Self::M02),
            "03" => Some(Self::M03),
            "04" => Some(Self::M04),
            "05" => Some(Self::M05),
            "06" => Some(Self::M06),
            "07" => Some(Self::M07),
            "08" => Some(Self::M08),
            "09" => Some(Self::M09),
            "10" => Some(Self::M10),
            "11" => Some(Self::M11),
            "12" => Some(Self::M12),
            "13" => Some(Self::M13),
            "14" => Some(Self::M14),
            "15" => Some(Self::M15),
            "16" => Some(Self::M16),
            "17" => Some(Self::M17),
            "18" => Some(Self::M18),
            "19" => Some(Self::M19),
            "20" => Some(Self::M20),
            "21" => Some(Self::M21),
            "22" => Some(Self::M22),
            "23" => Some(Self::M23),
            "24" => Some(Self::M24),
            "25" => Some(Self::M25),
            "26" => Some(Self::M26),
            "30" => Some(Self::M30),
            _ => None,
        }
    }

    pub(crate) fn to_digit(&self) -> u8 {
        match self {
            Self::M00 => 0,
            Self::M01 => 1,
            Self::M02 => 2,
            Self::M03 => 3,
            Self::M04 => 4,
            Self::M05 => 5,
            Self::M06 => 6,
            Self::M07 => 7,
            Self::M08 => 8,
            Self::M09 => 9,
            Self::M10 => 10,
            Self::M11 => 11,
            Self::M12 => 12,
            Self::M13 => 13,
            Self::M14 => 14,
            Self::M15 => 15,
            Self::M16 => 16,
            Self::M17 => 17,
            Self::M18 => 18,
            Self::M19 => 19,
            Self::M20 => 20,
            Self::M21 => 21,
            Self::M22 => 22,
            Self::M23 => 23,
            Self::M24 => 24,
            Self::M25 => 25,
            Self::M26 => 26,
            Self::M30 => 30,
        }
    }

    pub(crate) fn process(&self, grouper: &Grouper, input: &GrouperInput) -> MdcResult {
        match self {
            Self::M00 => m00::process(grouper, input),
            Self::M01 => m01::process(grouper, input),
            Self::M02 => m02::process(grouper, input),
            Self::M03 => m03::process(grouper, input),
            Self::M04 => m04::process(grouper, input),
            Self::M05 => m05::process(grouper, input),
            Self::M06 => m06::process(grouper, input),
            Self::M07 => m07::process(grouper, input),
            Self::M08 => m08::process(grouper, input),
            Self::M09 => m09::process(grouper, input),
            Self::M10 => m10::process(grouper, input),
            Self::M11 => m11::process(grouper, input),
            Self::M12 => m12::process(grouper, input),
            Self::M13 => m13::process(grouper, input),
            Self::M14 => m14::process(grouper, input),
            Self::M15 => m15::process(grouper, input),
            Self::M16 => m16::process(grouper, input),
            Self::M17 => m17::process(grouper, input),
            Self::M18 => m18::process(grouper, input),
            Self::M19 => m19::process(grouper, input),
            Self::M20 => m20::process(grouper, input),
            Self::M21 => m21::process(grouper, input),
            Self::M22 => m22::process(grouper, input),
            Self::M23 => m23::process(grouper, input),
            Self::M24 => m24::process(grouper, input),
            Self::M25 => m25::process(grouper, input),
            Self::M26 | Self::M30 => MdcResult::Drg(String::from("26509")),
        }
    }
}
