// Pediatric LqSOFA https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7786830/
// CRT >= 3s
// AVPU = VPU
// PR > 99th age adjusted
// RR > 99th age adjusted
// age adjusted https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4074640/

use std::rc::Rc;
use time::Date;

use kphis_util::datetime::{date_8601, js_now};

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub enum AgeGroup {
    Month3,  // 0 - <3 month
    Month6,  // 3 - <6 month
    Month9,  // 6 - <9 month
    Month12, // 9 - <12 month
    Month18, // 12 - <18 month
    Month24, // 18 - <24 month
    Year3,   // 2 - <3 year
    Year4,   // 3 - <4 year
    Year6,   // 4 - <6 year
    Year8,   // 6 - <8 year
    Year12,  // 8 - <12 year
    Year15,  // 12 - <15 year
    Year18,  // 15 - <18 year
    Adult,   // >= 18 year
}

impl AgeGroup {
    pub fn from_birthday(birthday: Date, regday: Date) -> Self {
        let days = (regday - birthday).whole_days();
        match days / 365 {
            ..=0 => match days {
                ..=90 => Self::Month3,
                91..=180 => Self::Month6,
                181..=270 => Self::Month9,
                271.. => Self::Month12,
            },
            1 => match days - 365 {
                ..=183 => Self::Month18,
                184.. => Self::Month24,
            },
            2 => Self::Year3,
            3 => Self::Year4,
            4..=5 => Self::Year6,
            6..=7 => Self::Year8,
            8..=11 => Self::Year12,
            12..=14 => Self::Year15,
            15..=17 => Self::Year18,
            18.. => Self::Adult,
        }
    }
}

#[derive(Clone)]
pub struct Lqsofa {
    age_group: Option<AgeGroup>,

    pr: Option<u32>,
    rr: Option<u32>,
    avpu_id: Option<u32>,
    crt: Option<i32>,

    score_pr: Option<bool>,
    score_rr: Option<bool>,
    score_avpu: Option<bool>,
    score_crt: Option<bool>,

    score: Option<u32>,
}

impl Lqsofa {
    pub fn new(birthday: Option<Date>, regday: Date, pr: Option<u32>, rr: Option<u32>, avpu_id: Option<u32>, crt: Option<i32>) -> Self {
        let age_group = birthday.map(|day| AgeGroup::from_birthday(day, regday));
        let score_pr = Self::score_pr(&age_group, pr);
        let score_rr = Self::score_rr(&age_group, rr);
        let score_avpu = Self::score_avpu(avpu_id);
        let score_crt = Self::score_crt(crt);
        let score = Self::scoring(score_pr, score_rr, score_avpu, score_crt);

        Self {
            age_group,
            pr,
            rr,
            avpu_id,
            crt,
            score_pr,
            score_rr,
            score_avpu,
            score_crt,
            score,
        }
    }

    fn score_pr(age_group: &Option<AgeGroup>, pr_opt: Option<u32>) -> Option<bool> {
        age_group.as_ref().and_then(|age_gr| {
            pr_opt.map(|pr| match age_gr {
                AgeGroup::Month3 => pr > 186,
                AgeGroup::Month6 => pr > 182,
                AgeGroup::Month9 => pr > 178,
                AgeGroup::Month12 => pr > 176,
                AgeGroup::Month18 => pr > 173,
                AgeGroup::Month24 => pr > 170,
                AgeGroup::Year3 => pr > 167,
                AgeGroup::Year4 => pr > 164,
                AgeGroup::Year6 => pr > 161,
                AgeGroup::Year8 => pr > 155,
                AgeGroup::Year12 => pr > 147,
                AgeGroup::Year15 => pr > 138,
                AgeGroup::Year18 => pr > 132,
                AgeGroup::Adult => false,
            })
        })
    }

    fn score_rr(age_group: &Option<AgeGroup>, rr_opt: Option<u32>) -> Option<bool> {
        age_group.as_ref().and_then(|age_gr| {
            rr_opt.map(|rr| match age_gr {
                AgeGroup::Month3 => rr > 76,
                AgeGroup::Month6 => rr > 71,
                AgeGroup::Month9 => rr > 67,
                AgeGroup::Month12 => rr > 63,
                AgeGroup::Month18 => rr > 60,
                AgeGroup::Month24 => rr > 57,
                AgeGroup::Year3 => rr > 54,
                AgeGroup::Year4 => rr > 52,
                AgeGroup::Year6 => rr > 50,
                AgeGroup::Year8 => rr > 46,
                AgeGroup::Year12 => rr > 41,
                AgeGroup::Year15 => rr > 35,
                AgeGroup::Year18 => rr > 32,
                AgeGroup::Adult => rr > 20,
            })
        })
    }

    fn score_avpu(avpu_opt: Option<u32>) -> Option<bool> {
        avpu_opt.map(|avpu| avpu > 1)
    }

    fn score_crt(crt_opt: Option<i32>) -> Option<bool> {
        crt_opt.map(|crt| crt >= 3)
    }

    fn scoring(score_pr: Option<bool>, score_rr: Option<bool>, score_avpu: Option<bool>, score_crt: Option<bool>) -> Option<u32> {
        if let (Some(pr), Some(rr), Some(avpu), Some(crt)) = (score_pr, score_rr, score_avpu, score_crt) {
            Some(pr as u32 + rr as u32 + avpu as u32 + crt as u32)
        } else {
            None
        }
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(self.score_pr, self.score_rr, self.score_avpu, self.score_crt);
    }
}

impl super::Scorable for Lqsofa {
    fn from_concat(pipes: &[&str], birthday: Option<Date>) -> Self {
        let regday = date_8601(pipes[0]).unwrap_or(js_now().date());
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let avpu_id = pipes[13].parse::<u32>().ok();
        let crt = pipes[20].parse::<i32>().ok();

        Self::new(birthday, regday, pr, rr, avpu_id, crt)
    }

    fn from_vs(vs: &Rc<VitalSign>, birthday: Option<time::Date>) -> Self {
        Self::new(birthday, vs.vs_datetime.date(), vs.pr, vs.rr, vs.avpu_id, vs.crt)
    }

    fn contains(item: &str) -> bool {
        ["pr", "rr", "avpu_id", "crt"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "pr" => "LqSOFA: Age-group scoring, [0-1]",
            "rr" => "LqSOFA: Age-group scoring, [0-1]",
            "avpu_id" => "LqSOFA: Alert is normal, [0-1]",
            "crt" => "LqSOFA: < 3 seconds is normal, [0-1]",
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Lqsofa {
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
            "pr" => self.score_pr,
            "rr" => self.score_rr,
            "avpu_id" => self.score_avpu,
            "crt" => self.score_crt,
            _ => None,
        }
        .map(|b| b as u32)
    }

    fn set_item(&mut self, item: &str, value: &str) {
        match item {
            "pr" => {
                self.pr = value.parse::<u32>().ok();
                self.score_pr = Self::score_pr(&self.age_group, self.pr);
            }
            "rr" => {
                self.rr = value.parse::<u32>().ok();
                self.score_rr = Self::score_rr(&self.age_group, self.rr);
            }
            "avpu_id" => {
                self.avpu_id = value.parse::<u32>().ok();
                self.score_avpu = Self::score_avpu(self.avpu_id);
            }
            "crt" => {
                self.crt = value.parse::<i32>().ok();
                self.score_crt = Self::score_crt(self.crt);
            }
            _ => {}
        }
        self.rescore();
    }

    fn title(&self) -> String {
        [
            "CRT(",
            &(self.score_crt.unwrap_or_default() as u32).to_string(),
            "), \
            AVPU(",
            &(self.score_avpu.unwrap_or_default() as u32).to_string(),
            "), \
            PR(",
            &(self.score_pr.unwrap_or_default() as u32).to_string(),
            "), \
            RR(",
            &(self.score_rr.unwrap_or_default() as u32).to_string(),
            ")",
        ]
        .concat()
    }
}
