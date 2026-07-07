// Adult SIRS (2 or more)
// - Temp > 38 or < 36
// - PR > 90
// - RR > 20 or pCO2 < 32
// - WBC > 12000 or < 4000 or band > 10

use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub struct Sirs {
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

impl Sirs {
    pub fn new(bt: Option<Decimal>, pr: Option<u32>, rr: Option<u32>, wbc: Option<Decimal>, band: Option<i32>) -> Self {
        let score_bt = Self::score_bt(bt);
        let score_pr = Self::score_pr(pr);
        let score_rr = Self::score_rr(rr);
        let score_wbc_band = Self::score_wbc(wbc, band);
        let score = Self::scoring(score_bt, score_pr, score_rr, score_wbc_band);

        Self {
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

    fn score_bt(bt_opt: Option<Decimal>) -> Option<bool> {
        bt_opt.map(|bt| bt < Decimal::new(36, 0) || bt > Decimal::new(38, 0))
    }

    fn score_pr(pr_opt: Option<u32>) -> Option<bool> {
        pr_opt.map(|pr| pr > 90)
    }

    fn score_rr(rr_opt: Option<u32>) -> Option<bool> {
        rr_opt.map(|rr| rr > 20)
    }

    fn score_wbc(wbc_opt: Option<Decimal>, band_opt: Option<i32>) -> Option<bool> {
        if band_opt.map(|band| band > 10).unwrap_or_default() {
            Some(true)
        } else {
            wbc_opt.map(|wbc| wbc < Decimal::new(4, 0) || wbc > Decimal::new(12, 0))
        }
    }

    fn scoring(score_bt: Option<bool>, score_pr: Option<bool>, score_rr: Option<bool>, score_wbc_band: Option<bool>) -> Option<u32> {
        if let (Some(bt), Some(pr), Some(rr), Some(wbc_band)) = (score_bt, score_pr, score_rr, score_wbc_band) {
            Some(bt as u32 + pr as u32 + rr as u32 + wbc_band as u32)
        } else {
            None
        }
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(self.score_bt, self.score_pr, self.score_rr, self.score_wbc_band);
    }
}

impl super::Scorable for Sirs {
    fn from_concat(pipes: &[&str], _birthday: Option<Date>) -> Self {
        let bt = Decimal::from_str_exact(pipes[1]).ok();
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let wbc = Decimal::from_str_exact(pipes[16]).ok();
        let band = pipes[21].parse::<i32>().ok();

        Self::new(bt, pr, rr, wbc, band)
    }

    fn from_vs(vs: &Rc<VitalSign>, _birthday: Option<time::Date>) -> Self {
        Self::new(vs.bt, vs.pr, vs.rr, vs.wbc, vs.band)
    }

    fn contains(item: &str) -> bool {
        ["bt", "pr", "rr", "wbc", "band"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "bt" => "SIRS: 36 - 38 is normal, [0-1]",
            "pr" => "SIRS: ≤ 90 is normal, [0-1]",
            "rr" => "SIRS: ≤ 20 is normal, [0-1]",
            "wbc" => "SIRS: 4k - 12k is normal, [0-1], using with band",
            "band" => "SIRS: If band > 10, WBC score is 1",
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Sirs {
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
                self.score_bt = Self::score_bt(self.bt);
            }
            "pr" => {
                self.pr = value.parse::<u32>().ok();
                self.score_pr = Self::score_pr(self.pr);
            }
            "rr" => {
                self.rr = value.parse::<u32>().ok();
                self.score_rr = Self::score_rr(self.rr);
            }
            "wbc" => {
                self.wbc = Decimal::from_str_exact(value).ok();
                self.score_wbc_band = Self::score_wbc(self.wbc, self.band);
            }
            "band" => {
                self.band = value.parse::<i32>().ok();
                self.score_wbc_band = Self::score_wbc(self.wbc, self.band);
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
