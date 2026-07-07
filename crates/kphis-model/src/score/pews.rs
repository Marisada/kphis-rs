// Phayakkhaphum Phisai Hospital PEWS
// last modified 21/8/67

// BT, PR, RR, SBP, SAT, AVPU, OTHER
// age <= 15

use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;

use kphis_util::datetime::{date_8601, js_now};

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub enum AgeGroup {
    Month1,  // 0 - <1 month
    Month12, // 1 - <12 month
    Year5,   // 1 - <5 year
    Year10,  // 5 - <10 year
    Year16,  // 12 - <16 year
    Adult,   // >= 16 year
}

impl AgeGroup {
    pub fn from_birthday(birthday: Date, regday: Date) -> Self {
        let days = (regday - birthday).whole_days();
        match days / 365 {
            ..=0 => match days {
                ..=30 => Self::Month1,
                31.. => Self::Month12,
            },
            1..=4 => Self::Year5,
            5..=10 => Self::Year10,
            11..=15 => Self::Year16,
            16.. => Self::Adult,
        }
    }
}

#[derive(Clone)]
pub struct Pews {
    age_group: Option<AgeGroup>,

    bt: Option<Decimal>,
    pr: Option<u32>,
    rr: Option<u32>,
    sbp: Option<u32>,
    sat: Option<u32>,
    o2_id: Option<u32>,
    avpu_id: Option<u32>,
    pops_other_id: Option<u32>,

    score_bt: Option<u32>,
    score_pr: Option<u32>,
    score_rr: Option<u32>,
    score_sbp: Option<u32>,
    score_sat: Option<u32>,
    score_o2: Option<u32>,
    score_avpu: Option<u32>,
    score_pops_other: Option<u32>,

    score: Option<u32>,
}

impl Pews {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        birthday: Option<Date>,
        regday: Date,
        bt: Option<Decimal>,
        pr: Option<u32>,
        rr: Option<u32>,
        sbp: Option<u32>,
        sat: Option<u32>,
        o2_id: Option<u32>,
        avpu_id: Option<u32>,
        pops_other_id: Option<u32>,
    ) -> Self {
        let age_group = birthday.map(|day| AgeGroup::from_birthday(day, regday));
        let score_bt = Self::score_bt(&age_group, bt);
        let score_pr = Self::score_pr(&age_group, pr);
        let score_rr = Self::score_rr(&age_group, rr);
        let score_sbp = Self::score_sbp(&age_group, sbp);
        let score_sat = Self::score_sat(&age_group, sat);
        let score_o2 = Self::score_o2(o2_id);
        let score_avpu = Self::score_avpu(avpu_id);
        let score_pops_other = Self::score_pops_other(pops_other_id);
        let score = Self::scoring(score_bt, score_pr, score_rr, score_sbp, score_sat, score_o2, score_avpu, score_pops_other);

        Self {
            age_group,
            bt,
            pr,
            rr,
            sbp,
            sat,
            o2_id,
            avpu_id,
            pops_other_id,
            score_bt,
            score_pr,
            score_rr,
            score_sbp,
            score_sat,
            score_o2,
            score_avpu,
            score_pops_other,
            score,
        }
    }

    fn score_bt(age_group: &Option<AgeGroup>, bt_opt: Option<Decimal>) -> Option<u32> {
        age_group.as_ref().and_then(|age_gr| match age_gr {
            AgeGroup::Month1 => None, // bt_opt.map(|bt| {
            //     if bt < Decimal::new(36,0) {
            //         2
            //     } else if bt < Decimal::new(365,1) {
            //         1
            //     } else if bt <= Decimal::new(375,1) {
            //         0
            //     } else if bt <= Decimal::new(38,0) {
            //         1
            //     } else {
            //         2
            //     }
            // }),
            AgeGroup::Month12 => bt_opt.map(|bt| {
                if bt < Decimal::new(36, 0) {
                    2
                } else if bt < Decimal::new(365, 1) {
                    1
                } else if bt <= Decimal::new(38, 0) {
                    0
                } else {
                    1
                }
            }),
            AgeGroup::Year5 | AgeGroup::Year10 | AgeGroup::Year16 => bt_opt.map(|bt| {
                if bt < Decimal::new(36, 0) {
                    1
                } else if bt <= Decimal::new(385, 1) {
                    0
                } else {
                    1
                }
            }),
            AgeGroup::Adult => None,
        })
    }

    fn score_pr(age_group: &Option<AgeGroup>, pr_opt: Option<u32>) -> Option<u32> {
        age_group.as_ref().and_then(|age_gr| match age_gr {
            AgeGroup::Month1 => None, // pr_opt.map(|pr| {
            //     match pr {
            //         ..=79 => 3,
            //         80..=100 => 2,
            //         101..=160 => 0,
            //         161..=180 => 1,
            //         181..=220 => 2,
            //         221.. => 3,
            //     }
            // }),
            AgeGroup::Month12 => pr_opt.map(|pr| match pr {
                ..=59 => 3,
                60..=80 => 2,
                81..=89 => 1,
                90..=130 => 0,
                131..=140 => 1,
                141..=180 => 2,
                181.. => 3,
            }),
            AgeGroup::Year5 => pr_opt.map(|pr| match pr {
                ..=59 => 3,
                60..=69 => 1,
                70..=120 => 0,
                121..=130 => 1,
                131..=150 => 2,
                151.. => 3,
            }),
            AgeGroup::Year10 => pr_opt.map(|pr| match pr {
                ..=40 => 3,
                41..=60 => 1,
                61..=110 => 0,
                111..=120 => 1,
                121..=140 => 2,
                141.. => 3,
            }),
            AgeGroup::Year16 => pr_opt.map(|pr| match pr {
                ..=40 => 3,
                41..=60 => 1,
                61..=100 => 0,
                101..=120 => 1,
                121..=140 => 2,
                141.. => 3,
            }),
            AgeGroup::Adult => None,
        })
    }

    fn score_rr(age_group: &Option<AgeGroup>, rr_opt: Option<u32>) -> Option<u32> {
        age_group.as_ref().and_then(|age_gr| match age_gr {
            AgeGroup::Month1 => None, // rr_opt.map(|rr| {
            //     match rr {
            //         ..=19 => 2,
            //         20..=39 => 1,
            //         40..=60 => 0,
            //         61..=80 => 2,
            //         81.. => 3,
            //     }
            // }),
            AgeGroup::Month12 => rr_opt.map(|rr| match rr {
                ..=9 => 3,
                10..=20 => 2,
                21..=30 => 1,
                31..=50 => 0,
                51..=60 => 1,
                61..=70 => 2,
                71.. => 3,
            }),
            AgeGroup::Year5 => rr_opt.map(|rr| match rr {
                ..=14 => 3,
                15..=19 => 1,
                20..=30 => 0,
                31..=40 => 1,
                41..=45 => 2,
                46.. => 3,
            }),
            AgeGroup::Year10 => rr_opt.map(|rr| match rr {
                ..=11 => 3,
                12..=14 => 1,
                15..=24 => 0,
                25..=30 => 1,
                31.. => 3,
            }),
            AgeGroup::Year16 => rr_opt.map(|rr| match rr {
                ..=9 => 3,
                10..=14 => 1,
                15..=22 => 0,
                23..=25 => 1,
                26..=30 => 2,
                31.. => 3,
            }),
            AgeGroup::Adult => None,
        })
    }

    fn score_sbp(age_group: &Option<AgeGroup>, sbp_opt: Option<u32>) -> Option<u32> {
        age_group.as_ref().and_then(|age_gr| match age_gr {
            AgeGroup::Month1 => None,
            AgeGroup::Month12 => sbp_opt.map(|sbp| match sbp {
                ..=69 => 3,
                70..=105 => 0,
                106..=120 => 1,
                121.. => 3,
            }),
            AgeGroup::Year5 => sbp_opt.map(|sbp| match sbp {
                ..=74 => 3,
                75..=84 => 1,
                85..=110 => 0,
                111..=130 => 1,
                131.. => 3,
            }),
            AgeGroup::Year10 => sbp_opt.map(|sbp| match sbp {
                ..=79 => 3,
                80..=89 => 1,
                90..=120 => 0,
                121..=130 => 1,
                131..=150 => 2,
                151.. => 3,
            }),
            AgeGroup::Year16 => sbp_opt.map(|sbp| match sbp {
                ..=89 => 3,
                90..=125 => 0,
                126..=130 => 1,
                131..=150 => 2,
                151.. => 3,
            }),
            AgeGroup::Adult => None,
        })
    }

    fn score_sat(age_group: &Option<AgeGroup>, sat_opt: Option<u32>) -> Option<u32> {
        age_group.as_ref().and_then(|age_gr| match age_gr {
            AgeGroup::Month1 => None, // sat_opt.map(|sat| {
            //     match sat {
            //         ..=79 => 3,
            //         80..=90 => 2,
            //         91..=94 => 1,
            //         95.. => 0,
            //     }
            // }),
            AgeGroup::Month12 | AgeGroup::Year5 | AgeGroup::Year10 | AgeGroup::Year16 => sat_opt.map(|sat| match sat {
                ..=84 => 3,
                85..=89 => 2,
                90..=93 => 1,
                94.. => 0,
            }),
            AgeGroup::Adult => None,
        })
    }

    fn score_o2(o2_opt: Option<u32>) -> Option<u32> {
        o2_opt
            .map(|o2| {
                match o2 {
                    4 => 2, // high flow
                    _ => 1,
                }
            })
            .or(Some(0))
    }

    fn score_avpu(avpu_opt: Option<u32>) -> Option<u32> {
        avpu_opt.map(|avpu| match avpu {
            ..=1 => 0,
            2 => 1,
            3 => 2,
            4.. => 3,
        })
    }

    fn score_pops_other(pops_other_opt: Option<u32>) -> Option<u32> {
        pops_other_opt.map(|pops_other| match pops_other {
            ..=1 => 0,
            2.. => 2,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn scoring(
        score_bt: Option<u32>,
        score_pr: Option<u32>,
        score_rr: Option<u32>,
        score_sbp: Option<u32>,
        score_sat: Option<u32>,
        score_o2: Option<u32>,
        score_avpu: Option<u32>,
        score_pops_other: Option<u32>,
    ) -> Option<u32> {
        if let (Some(bt), Some(pr), Some(rr), Some(sbp), Some(sat), Some(avpu), Some(o2), Some(pops_other)) =
            (score_bt, score_pr, score_rr, score_sbp, score_sat, score_o2, score_avpu, score_pops_other)
        {
            Some(bt + pr + rr + sbp + sat + avpu + o2 + pops_other)
        } else {
            None
        }
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(
            self.score_bt,
            self.score_pr,
            self.score_rr,
            self.score_sbp,
            self.score_sat,
            self.score_o2,
            self.score_avpu,
            self.score_pops_other,
        );
    }
}

impl super::Scorable for Pews {
    fn from_concat(pipes: &[&str], birthday: Option<Date>) -> Self {
        let regday = date_8601(pipes[0]).unwrap_or(js_now().date());
        let bt = Decimal::from_str_exact(pipes[1]).ok();
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let sbp = pipes[4].parse::<u32>().ok();
        let sat = pipes[10].parse::<u32>().ok();
        let o2_id = pipes[11].parse::<u32>().ok();
        let avpu_id = pipes[13].parse::<u32>().ok();
        let pops_other_id = pipes[15].parse::<u32>().ok();

        Self::new(birthday, regday, bt, pr, rr, sbp, sat, o2_id, avpu_id, pops_other_id)
    }

    fn from_vs(vs: &Rc<VitalSign>, birthday: Option<time::Date>) -> Self {
        Self::new(birthday, vs.vs_datetime.date(), vs.bt, vs.pr, vs.rr, vs.sbp, vs.sat, vs.o2_id, vs.avpu_id, vs.pops_other_id)
    }

    fn contains(item: &str) -> bool {
        ["bt", "pr", "rr", "sbp", "sat", "o2_id", "avpu_id", "pops_other_id"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "bt" => "PEWS: Age-group scoring, [0-2]",
            "pr" => "PEWS: Age-group scoring, [0-3]",
            "rr" => "PEWS: Age-group scoring, [0-3]\nscore 3 if has gasping or severe retraction",
            "sbp" => "PEWS: Age-group scoring, [0-3]",
            "sat" => "PEWS: Age-group scoring, [0-3]",
            "o2_id" => "PEWS: Using O\u{2082} is 1, using high flow is 2, [0-2]",
            "avpu_id" => "PEWS: Alert is normal, [0-3]",
            "pops_other_id" => {
                "PEWS: NA is normal, [0,2]\n\
                PMH includes\n  \
                - ชักนาน ≥ 5 นาที หรือชักบ่อย ≥ 3 ครั้ง\n  \
                - พ่นยาทุก 15 นาที หรือ พ่นยาต่อเนื่อง\n  \
                - โรคประจำตัว เช่น CLD, BPD, Cerebral Palsy, Immunodeficiency\n\
                Oncology Patient with\n  \
                - BT ≥ 38.3 x 1 ครั้ง หรือ BT > 38 มากกว่า 1 hr ร่วมกับ ANC < 500 หรือ ANC < 1,000 ที่มีแนวโน้มลดลง\n  \
                - ถ่ายเหลว ≥ 3 ครั้ง หรือ ถ่ายเป็นมูกเลือด ≥ 1 ครั้ง"
            }
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Pews {
    fn color_total(&self) -> &'static str {
        self.score.map(|score| if score < 7 { "black" } else { "white" }).unwrap_or("white")
    }

    fn bg_color_total(&self) -> &'static str {
        self.score
            .map(|score| match score {
                0..=3 => "lime",
                4..=6 => "gold",
                7.. => "crimson",
            })
            .unwrap_or("grey")
    }

    fn color_item(&self, item: &str) -> &'static str {
        self.score_item(item).map(|score| if score < 3 { "black" } else { "white" }).unwrap_or("white")
    }

    fn bg_color_item(&self, item: &str) -> &'static str {
        self.score_item(item)
            .map(|score| match score {
                0 => "lime",
                1 => "gold",
                2 => "salmon",
                3.. => "crimson",
            })
            .unwrap_or("grey")
    }

    fn custom_select_option(&self, item: &str) -> Option<Vec<SelectOption>> {
        match item {
            "avpu_id" => Some(vec![
                SelectOption {
                    key: String::from("1"),
                    value: String::from("ตื่นดีหรือหลับแต่ปลุกตื่นง่าย"),
                },
                SelectOption {
                    key: String::from("2"),
                    value: String::from("กระสับกระส่าย"),
                },
                SelectOption {
                    key: String::from("3"),
                    value: String::from("ดูซึม ไม่มีแรง"),
                },
                SelectOption {
                    key: String::from("4"),
                    value: String::from("ชัก, ไม่ตอบสนอง"),
                },
            ]),
            _ => None,
        }
    }

    fn score(&self) -> Option<u32> {
        self.score
    }

    fn score_item(&self, item: &str) -> Option<u32> {
        match item {
            "bt" => self.score_bt,
            "pr" => self.score_pr,
            "rr" => self.score_rr,
            "sbp" => self.score_sbp,
            "sat" => self.score_sat,
            "o2_id" => self.score_o2,
            "avpu_id" => self.score_avpu,
            "pops_other_id" => self.score_pops_other,
            _ => None,
        }
    }

    fn set_item(&mut self, item: &str, value: &str) {
        match item {
            "bt" => {
                self.bt = Decimal::from_str_exact(value).ok();
                self.score_bt = Self::score_bt(&self.age_group, self.bt);
            }
            "pr" => {
                self.pr = value.parse::<u32>().ok();
                self.score_pr = Self::score_pr(&self.age_group, self.pr);
            }
            "rr" => {
                self.rr = value.parse::<u32>().ok();
                self.score_rr = Self::score_rr(&self.age_group, self.rr);
            }
            "sbp" => {
                self.sbp = value.parse::<u32>().ok();
                self.score_sbp = Self::score_sbp(&self.age_group, self.sbp);
            }
            "sat" => {
                self.sat = value.parse::<u32>().ok();
                self.score_sat = Self::score_sat(&self.age_group, self.sat);
            }
            "o2_id" => {
                self.o2_id = value.parse::<u32>().ok();
                self.score_o2 = Self::score_o2(self.o2_id);
            }
            "avpu_id" => {
                self.avpu_id = value.parse::<u32>().ok();
                self.score_avpu = Self::score_avpu(self.avpu_id);
            }
            "pops_other_id" => {
                self.pops_other_id = value.parse::<u32>().ok();
                self.score_pops_other = Self::score_pops_other(self.pops_other_id);
            }
            _ => {}
        }
        self.rescore();
    }

    fn title(&self) -> String {
        [
            "BT(",
            &self.score_bt.unwrap_or_default().to_string(),
            "), \
            PR(",
            &self.score_pr.unwrap_or_default().to_string(),
            "), \
            RR(",
            &self.score_rr.unwrap_or_default().to_string(),
            "), \
            SBP(",
            &self.score_sbp.unwrap_or_default().to_string(),
            "), \
            SAT(",
            &self.score_sat.unwrap_or_default().to_string(),
            "), \
            O\u{2082}(",
            &self.score_o2.unwrap_or_default().to_string(),
            "), \
            AVPU(",
            &self.score_avpu.unwrap_or_default().to_string(),
            "), \
            OTHER(",
            &self.score_pops_other.unwrap_or_default().to_string(),
            ")",
        ]
        .concat()
    }
}
