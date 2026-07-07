// Pedriatric SIRS https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4913352/
// with adult SIRS fallback
// Adult SIRS (2 or more)
// - Temp > 38 or < 36
// - PR > 90
// - RR > 20 or pCO2 < 32
// - WBC > 12000 or < 4000 or band > 10

use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;

use kphis_util::datetime::{date_8601, js_now};

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
enum AgeGroup {
    Week,      // 0-7 day
    Month,     // 8 day - 1 month
    Year,      // 1 month - 2 year
    Kinder,    // 2-5 year
    Primary,   // 6-12 year
    Secondary, // 13-18 year
    Adult,     // > 18 year
}

impl AgeGroup {
    fn from_birthday(birthday: Date, regday: Date) -> Self {
        let days = (regday - birthday).whole_days();
        match days / 365 {
            ..=0 => match days {
                ..=6 => Self::Week,
                7..=30 => Self::Month,
                31.. => Self::Year,
            },
            1 => Self::Year,
            2..=5 => Self::Kinder,
            6..=12 => Self::Primary,
            13..=18 => Self::Secondary,
            19.. => Self::Adult,
        }
    }
}

#[derive(Clone)]
pub struct Psirs {
    age_group: Option<AgeGroup>,

    bt: Option<Decimal>,
    pr: Option<u32>,
    rr: Option<u32>,
    wbc: Option<Decimal>,
    band: Option<i32>,

    score_bt: Option<bool>,
    score_pr: Option<bool>,
    score_rr: Option<bool>,
    score_wbc_band: Option<bool>,

    score: Option<u32>,
}

impl Psirs {
    pub fn new(birthday: Option<Date>, regday: Date, bt: Option<Decimal>, pr: Option<u32>, rr: Option<u32>, wbc: Option<Decimal>, band: Option<i32>) -> Self {
        let age_group = birthday.map(|day| AgeGroup::from_birthday(day, regday));
        let score_bt = Self::score_bt(&age_group, bt);
        let score_pr = Self::score_pr(&age_group, pr);
        let score_rr = Self::score_rr(&age_group, rr);
        let score_wbc_band = Self::score_wbc(&age_group, wbc, band);
        let score = Self::scoring(&age_group, score_bt, score_pr, score_rr, score_wbc_band);

        Self {
            age_group,
            bt,
            pr,
            rr,
            wbc,
            band,
            score_bt,
            score_pr,
            score_rr,
            score_wbc_band,
            score,
        }
    }

    fn score_bt(age_group: &Option<AgeGroup>, bt_opt: Option<Decimal>) -> Option<bool> {
        age_group.as_ref().and_then(|age_gr| {
            bt_opt.map(|bt| {
                if matches!(age_gr, AgeGroup::Adult) {
                    bt < Decimal::new(36, 0) || bt > Decimal::new(38, 0)
                } else {
                    bt < Decimal::new(36, 0) || bt > Decimal::new(385, 1)
                }
            })
        })
    }

    fn score_pr(age_group: &Option<AgeGroup>, pr_opt: Option<u32>) -> Option<bool> {
        age_group.as_ref().and_then(|age_gr| {
            pr_opt.map(|pr| match age_gr {
                AgeGroup::Week | AgeGroup::Month => !(100..=180).contains(&pr),
                AgeGroup::Year => !(90..=180).contains(&pr),
                AgeGroup::Kinder => pr > 140,
                AgeGroup::Primary => pr > 130,
                AgeGroup::Secondary => pr > 110,
                AgeGroup::Adult => pr > 90,
            })
        })
    }

    fn score_rr(age_group: &Option<AgeGroup>, rr_opt: Option<u32>) -> Option<bool> {
        age_group.as_ref().and_then(|age_gr| {
            rr_opt.map(|rr| match age_gr {
                AgeGroup::Week => rr > 50,
                AgeGroup::Month => rr > 40,
                AgeGroup::Year => rr > 34,
                AgeGroup::Kinder => rr > 22,
                AgeGroup::Primary => rr > 18,
                AgeGroup::Secondary => rr > 14,
                AgeGroup::Adult => rr > 20,
            })
        })
    }

    fn score_wbc(age_group: &Option<AgeGroup>, wbc_opt: Option<Decimal>, band_opt: Option<i32>) -> Option<bool> {
        if band_opt.map(|band| band > 10).unwrap_or_default() {
            Some(true)
        } else {
            age_group.as_ref().and_then(|age_gr| {
                wbc_opt.map(|wbc| match age_gr {
                    AgeGroup::Week => wbc > Decimal::new(34, 0),
                    AgeGroup::Month => wbc < Decimal::new(6, 0) || wbc > Decimal::new(195, 1),
                    AgeGroup::Year => wbc < Decimal::new(6, 0) || wbc > Decimal::new(175, 1),
                    AgeGroup::Kinder => wbc < Decimal::new(6, 0) || wbc > Decimal::new(155, 1),
                    AgeGroup::Primary => wbc < Decimal::new(45, 1) || wbc > Decimal::new(135, 1),
                    AgeGroup::Secondary => wbc < Decimal::new(45, 1) || wbc > Decimal::new(11, 0),
                    AgeGroup::Adult => wbc < Decimal::new(4, 0) || wbc > Decimal::new(12, 0),
                })
            })
        }
    }

    fn scoring(age_group: &Option<AgeGroup>, score_bt: Option<bool>, score_pr: Option<bool>, score_rr: Option<bool>, score_wbc_band: Option<bool>) -> Option<u32> {
        age_group.as_ref().and_then(|age_gr| {
            if let (Some(bt), Some(pr), Some(rr), Some(wbc_band)) = (score_bt, score_pr, score_rr, score_wbc_band) {
                Some(if matches!(age_gr, AgeGroup::Adult) {
                    bt as u32 + pr as u32 + rr as u32 + wbc_band as u32
                } else {
                    bt as u32 + (pr || rr) as u32 + wbc_band as u32
                })
            } else {
                None
            }
        })
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(&self.age_group, self.score_bt, self.score_pr, self.score_rr, self.score_wbc_band);
    }
}

impl super::Scorable for Psirs {
    fn from_concat(pipes: &[&str], birthday: Option<Date>) -> Self {
        let regday = date_8601(pipes[0]).unwrap_or(js_now().date());
        let bt = Decimal::from_str_exact(pipes[1]).ok();
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let wbc = Decimal::from_str_exact(pipes[16]).ok();
        let band = pipes[21].parse::<i32>().ok();

        Self::new(birthday, regday, bt, pr, rr, wbc, band)
    }

    fn from_vs(vs: &Rc<VitalSign>, birthday: Option<time::Date>) -> Self {
        Self::new(birthday, vs.vs_datetime.date(), vs.bt, vs.pr, vs.rr, vs.wbc, vs.band)
    }

    fn contains(item: &str) -> bool {
        ["bt", "pr", "rr", "wbc", "band"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "bt" => "SIRS: Age-group scoring, [0-1]",
            "pr" => "SIRS: Age-group scoring, [0-1]",
            "rr" => "SIRS: Age-group scoring, [0-1]",
            "wbc" => "SIRS: Age-group scoring, [0-1], using with band",
            "band" => "SIRS: If band > 10, WBC score is 1",
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Psirs {
    fn color_total(&self) -> &'static str {
        self.score.map(|score| if score < 2 { "black" } else { "white" }).unwrap_or("white")
    }

    fn bg_color_total(&self) -> &'static str {
        self.score
            .map(|score| match score {
                0..=1 => "lime",
                2.. => "crimson",
            })
            .unwrap_or("grey")
    }

    fn color_item(&self, item: &str) -> &'static str {
        self.score_item(item).map(|score| if score > 0 { "white" } else { "black" }).unwrap_or("white")
    }

    fn bg_color_item(&self, item: &str) -> &'static str {
        self.score_item(item).map(|score| if score > 0 { "crimson" } else { "lime" }).unwrap_or("grey")
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
            "wbc" => self.score_wbc_band,
            "band" => self.score_wbc_band,
            _ => None,
        }
        .map(|b| b as u32)
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
            "wbc" => {
                self.wbc = Decimal::from_str_exact(value).ok();
                self.score_wbc_band = Self::score_wbc(&self.age_group, self.wbc, self.band);
            }
            "band" => {
                self.band = value.parse::<i32>().ok();
                self.score_wbc_band = Self::score_wbc(&self.age_group, self.wbc, self.band);
            }
            _ => {}
        }
        self.rescore();
    }

    fn title(&self) -> String {
        [
            "BT(",
            &(self.score_bt.unwrap_or_default() as u32).to_string(),
            "), \
            PR(",
            &(self.score_pr.unwrap_or_default() as u32).to_string(),
            "), \
            RR(",
            &(self.score_rr.unwrap_or_default() as u32).to_string(),
            "), \
            WBC/BAND(",
            &(self.score_wbc_band.unwrap_or_default() as u32).to_string(),
            ")",
        ]
        .concat()
    }
}
