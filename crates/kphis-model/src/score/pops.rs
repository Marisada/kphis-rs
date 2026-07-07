// [PAT-POPS and ManChEWS](https://www.researchgate.net/publication/256470148_The_paediatric_observation_priority_score_pops_a_useful_tool_to_predict_likelihood_of_admission_from_the_emergency_department)

// BT, PR, RR, SAT, BREATHING, AVPU, GUT, OTHER
// age <= 15

use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;

use kphis_util::datetime::{date_8601, js_now};

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub struct Pops {
    age_y: Option<i8>,

    bt: Option<Decimal>,
    pr: Option<u32>,
    rr: Option<u32>,
    sat: Option<u32>,
    breathing_id: Option<u32>,
    avpu_id: Option<u32>,
    gut_feeling_id: Option<u32>,
    pops_other_id: Option<u32>,

    score_bt: Option<u32>,
    score_pr: Option<u32>,
    score_rr: Option<u32>,
    score_sat: Option<u32>,
    score_breathing: Option<u32>,
    score_avpu: Option<u32>,
    score_gut_feeling: Option<u32>,
    score_pops_other: Option<u32>,

    score: Option<u32>,
}

impl Pops {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        age_y: Option<i8>,
        bt: Option<Decimal>,
        pr: Option<u32>,
        rr: Option<u32>,
        sat: Option<u32>,
        breathing_id: Option<u32>,
        avpu_id: Option<u32>,
        gut_feeling_id: Option<u32>,
        pops_other_id: Option<u32>,
    ) -> Self {
        let score_bt = Self::score_bt(age_y, bt);
        let score_pr = Self::score_pr(age_y, pr);
        let score_rr = Self::score_rr(age_y, rr);
        let score_sat = Self::score_sat(sat);
        let score_breathing = Self::score_breathing(breathing_id);
        let score_avpu = Self::score_avpu(avpu_id);
        let score_gut_feeling = Self::score_gut_feeling(gut_feeling_id);
        let score_pops_other = Self::score_pops_other(pops_other_id);
        let score = Self::scoring(score_bt, score_pr, score_rr, score_sat, score_breathing, score_avpu, score_gut_feeling, score_pops_other);

        Self {
            age_y,
            bt,
            pr,
            rr,
            sat,
            breathing_id,
            avpu_id,
            gut_feeling_id,
            pops_other_id,
            score_bt,
            score_pr,
            score_rr,
            score_sat,
            score_breathing,
            score_avpu,
            score_gut_feeling,
            score_pops_other,
            score,
        }
    }

    fn score_bt(age_y: Option<i8>, bt_opt: Option<Decimal>) -> Option<u32> {
        if let Some(age_y) = age_y {
            match age_y {
                ..=0 => bt_opt.map(|bt| {
                    if bt < Decimal::new(35, 0) {
                        2
                    } else if bt < Decimal::new(36, 0) {
                        1
                    } else if bt <= Decimal::new(375, 1) {
                        0
                    } else if bt <= Decimal::new(39, 0) {
                        1
                    } else {
                        2
                    }
                }),
                1..=15 => bt_opt.map(|bt| {
                    if bt < Decimal::new(35, 0) {
                        2
                    } else if bt < Decimal::new(36, 0) {
                        1
                    } else if bt < Decimal::new(385, 1) {
                        0
                    } else if bt <= Decimal::new(40, 0) {
                        1
                    } else {
                        2
                    }
                }),
                16.. => None,
            }
        } else {
            None
        }
    }

    fn score_pr(age_y: Option<i8>, pr_opt: Option<u32>) -> Option<u32> {
        if let Some(age_y) = age_y {
            match age_y {
                ..=0 => pr_opt.map(|pr| match pr {
                    ..=89 => 2,
                    90..=109 => 1,
                    110..=160 => 0,
                    161..=180 => 1,
                    181.. => 2,
                }),
                1 => pr_opt.map(|pr| match pr {
                    ..=89 => 2,
                    90..=99 => 1,
                    100..=150 => 0,
                    151..=170 => 1,
                    171.. => 2,
                }),
                2..=4 => pr_opt.map(|pr| match pr {
                    ..=79 => 2,
                    80..=94 => 1,
                    95..=140 => 0,
                    141..=160 => 1,
                    161.. => 2,
                }),
                5..=12 => pr_opt.map(|pr| match pr {
                    ..=69 => 2,
                    70..=79 => 1,
                    80..=120 => 0,
                    121..=150 => 1,
                    151.. => 2,
                }),
                13..=15 => pr_opt.map(|pr| match pr {
                    ..=49 => 2,
                    50..=59 => 1,
                    60..=100 => 0,
                    101..=110 => 1,
                    111.. => 2,
                }),
                16.. => None,
            }
        } else {
            None
        }
    }

    fn score_rr(age_y: Option<i8>, rr_opt: Option<u32>) -> Option<u32> {
        if let Some(age_y) = age_y {
            match age_y {
                ..=0 => rr_opt.map(|rr| match rr {
                    ..=24 => 2,
                    25..=29 => 1,
                    30..=40 => 0,
                    41..=50 => 1,
                    51.. => 2,
                }),
                1 => rr_opt.map(|rr| match rr {
                    ..=19 => 2,
                    20..=24 => 1,
                    25..=35 => 0,
                    36..=50 => 1,
                    51.. => 2,
                }),
                2..=4 => rr_opt.map(|rr| match rr {
                    ..=19 => 2,
                    20..=24 => 1,
                    25..=30 => 0,
                    31..=40 => 1,
                    41.. => 2,
                }),
                5..=12 => rr_opt.map(|rr| match rr {
                    ..=14 => 2,
                    15..=19 => 1,
                    20..=25 => 0,
                    26..=40 => 1,
                    41.. => 2,
                }),
                13..=15 => rr_opt.map(|rr| match rr {
                    ..=11 => 2,
                    12..=14 => 1,
                    15..=20 => 0,
                    21..=25 => 1,
                    26.. => 2,
                }),
                16.. => None,
            }
        } else {
            None
        }
    }

    fn score_sat(sat_opt: Option<u32>) -> Option<u32> {
        sat_opt.map(|sat| match sat {
            0..=89 => 2,
            90..=94 => 1,
            95.. => 0,
        })
    }

    fn score_breathing(breathing_opt: Option<u32>) -> Option<u32> {
        breathing_opt.map(|breathing| match breathing {
            ..=1 => 0,
            2..=3 => 1,
            4.. => 2,
        })
    }

    fn score_avpu(avpu_opt: Option<u32>) -> Option<u32> {
        avpu_opt.map(|avpu| match avpu {
            ..=1 => 0,
            2 => 1,
            3.. => 2,
        })
    }

    fn score_gut_feeling(gut_feeling_opt: Option<u32>) -> Option<u32> {
        gut_feeling_opt.map(|gut_feeling| match gut_feeling {
            ..=1 => 0,
            2 => 1,
            3.. => 2,
        })
    }

    fn score_pops_other(pops_other_opt: Option<u32>) -> Option<u32> {
        pops_other_opt.map(|pops_other| match pops_other {
            ..=1 => 0,
            2 => 1,
            3.. => 2,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn scoring(
        score_bt: Option<u32>,
        score_pr: Option<u32>,
        score_rr: Option<u32>,
        score_sat: Option<u32>,
        score_breathing: Option<u32>,
        score_avpu: Option<u32>,
        score_gut_feeling: Option<u32>,
        score_pops_other: Option<u32>,
    ) -> Option<u32> {
        if let (Some(bt), Some(pr), Some(rr), Some(sat), Some(avpu), Some(breathing), Some(gut_feeling), Some(pops_other)) =
            (score_bt, score_pr, score_rr, score_sat, score_breathing, score_avpu, score_gut_feeling, score_pops_other)
        {
            Some(bt + pr + rr + sat + avpu + breathing + gut_feeling + pops_other)
        } else {
            None
        }
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(
            self.score_bt,
            self.score_pr,
            self.score_rr,
            self.score_sat,
            self.score_breathing,
            self.score_avpu,
            self.score_gut_feeling,
            self.score_pops_other,
        );
    }
}

impl super::Scorable for Pops {
    fn from_concat(pipes: &[&str], birthday: Option<Date>) -> Self {
        let regday = date_8601(pipes[0]).unwrap_or(js_now().date());
        let age_y = birthday.map(|day| ((regday - day).whole_days() / 365) as i8);
        let bt = Decimal::from_str_exact(pipes[1]).ok();
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let sat = pipes[10].parse::<u32>().ok();
        let breathing_id = pipes[12].parse::<u32>().ok();
        let avpu_id = pipes[13].parse::<u32>().ok();
        let gut_feeling_id = pipes[14].parse::<u32>().ok();
        let pops_other_id = pipes[15].parse::<u32>().ok();

        Self::new(age_y, bt, pr, rr, sat, breathing_id, avpu_id, gut_feeling_id, pops_other_id)
    }

    fn from_vs(vs: &Rc<VitalSign>, birthday: Option<time::Date>) -> Self {
        let age_y = birthday.map(|day| ((vs.vs_datetime.date() - day).whole_days() / 365) as i8);
        Self::new(age_y, vs.bt, vs.pr, vs.rr, vs.sat, vs.breathing_id, vs.avpu_id, vs.gut_feeling_id, vs.pops_other_id)
    }

    fn contains(item: &str) -> bool {
        ["bt", "pr", "rr", "sat", "breathing_id", "avpu_id", "gut_feeling_id", "pops_other_id"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "bt" => "POPS: Age-group scoring, [0-2]",
            "pr" => "POPS: Age-group scoring, [0-2]",
            "rr" => "POPS: Age-group scoring, [0-2]",
            "sat" => "POPS: ≥ 95 is normal, [0-2]",
            "breathing_id" => "POPS: No distress is normal, [0-2]",
            "avpu_id" => "POPS: Alert is normal, [0-2]",
            "gut_feeling_id" => "POPS: Well is normal, [0-2]",
            "pops_other_id" => {
                "POPS: NA is normal, [0-2]\n\
                PMH includes\n  \
                - Ex-premature\n  \
                - Syndromic conditions\n  \
                - Cardiac problems\n  \
                - Asthma\n  \
                - Diabetes\n  \
                - Long term steroids\n  \
                - All other chronic conditions"
            }
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Pops {
    fn color_total(&self) -> &'static str {
        self.score.map(|score| if score < 8 { "black" } else { "white" }).unwrap_or("white")
    }

    fn bg_color_total(&self) -> &'static str {
        self.score
            .map(|score| match score {
                0..=1 => "lime",
                2..=3 => "gold",
                4..=7 => "salmon",
                8.. => "crimson",
            })
            .unwrap_or("grey")
    }

    fn color_item(&self, item: &str) -> &'static str {
        self.score_item(item).map(|score| if score < 2 { "black" } else { "white" }).unwrap_or("white")
    }

    fn bg_color_item(&self, item: &str) -> &'static str {
        self.score_item(item)
            .map(|score| match score {
                0 => "lime",
                1 => "gold",
                2.. => "crimson",
            })
            .unwrap_or("grey")
    }

    fn custom_select_option(&self, _item: &str) -> Option<Vec<SelectOption>> {
        None
    }

    fn score(&self) -> Option<u32> {
        self.score
    }

    fn score_item(&self, item: &str) -> Option<u32> {
        match item {
            "bt" => self.score_bt,
            "pr" => self.score_pr,
            "rr" => self.score_rr,
            "sat" => self.score_sat,
            "breathing_id" => self.score_breathing,
            "avpu_id" => self.score_avpu,
            "gut_feeling_id" => self.score_gut_feeling,
            "pops_other_id" => self.score_pops_other,
            _ => None,
        }
    }

    fn set_item(&mut self, item: &str, value: &str) {
        match item {
            "bt" => {
                self.bt = Decimal::from_str_exact(value).ok();
                self.score_bt = Self::score_bt(self.age_y, self.bt);
            }
            "pr" => {
                self.pr = value.parse::<u32>().ok();
                self.score_pr = Self::score_pr(self.age_y, self.pr);
            }
            "rr" => {
                self.rr = value.parse::<u32>().ok();
                self.score_rr = Self::score_rr(self.age_y, self.rr);
            }
            "sat" => {
                self.sat = value.parse::<u32>().ok();
                self.score_sat = Self::score_sat(self.sat);
            }
            "breathing_id" => {
                self.breathing_id = value.parse::<u32>().ok();
                self.score_breathing = Self::score_breathing(self.breathing_id);
            }
            "avpu_id" => {
                self.avpu_id = value.parse::<u32>().ok();
                self.score_avpu = Self::score_avpu(self.avpu_id);
            }
            "gut_feeling_id" => {
                self.gut_feeling_id = value.parse::<u32>().ok();
                self.score_gut_feeling = Self::score_gut_feeling(self.gut_feeling_id);
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
            SAT(",
            &self.score_sat.unwrap_or_default().to_string(),
            "), \
            AVPU(",
            &self.score_avpu.unwrap_or_default().to_string(),
            "), \
            BREATH(",
            &self.score_breathing.unwrap_or_default().to_string(),
            "), \
            GUT(",
            &self.score_gut_feeling.unwrap_or_default().to_string(),
            "), \
            OTHER(",
            &self.score_pops_other.unwrap_or_default().to_string(),
            ")",
        ]
        .concat()
    }
}
