// MKH's S-NEWS

// BT, PR, RR, SBP, SAT, O2, AVPU
// age_y > 15

use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub struct Snews {
    bt: Option<Decimal>,
    pr: Option<u32>,
    rr: Option<u32>,
    sbp: Option<u32>,
    sat: Option<u32>,
    o2_id: Option<u32>,
    avpu_id: Option<u32>,

    score_bt: Option<u32>,
    score_pr: Option<u32>,
    score_rr: Option<u32>,
    score_sbp: Option<u32>,
    score_sat: Option<u32>,
    score_o2: Option<u32>,
    score_avpu: Option<u32>,

    score: Option<u32>,
}

impl Snews {
    #[allow(clippy::too_many_arguments)]
    pub fn new(bt: Option<Decimal>, pr: Option<u32>, rr: Option<u32>, sbp: Option<u32>, sat: Option<u32>, o2_id: Option<u32>, avpu_id: Option<u32>) -> Self {
        let score_bt = Self::score_bt(bt);
        let score_pr = Self::score_pr(pr);
        let score_rr = Self::score_rr(rr);
        let score_sbp = Self::score_sbp(sbp);
        let score_sat = Self::score_sat(sat);
        let score_o2 = Self::score_o2(o2_id);
        let score_avpu = Self::score_avpu(avpu_id);
        let score = Self::scoring(score_bt, score_pr, score_rr, score_sbp, score_sat, score_o2, score_avpu);

        Self {
            bt,
            pr,
            rr,
            sbp,
            sat,
            o2_id,
            avpu_id,
            score_bt,
            score_pr,
            score_rr,
            score_sbp,
            score_sat,
            score_o2,
            score_avpu,
            score,
        }
    }

    fn score_bt(bt_opt: Option<Decimal>) -> Option<u32> {
        bt_opt.map(|bt| {
            if bt < Decimal::new(36, 0) {
                1
            } else if bt <= Decimal::new(38, 0) {
                0
            } else if bt <= Decimal::new(39, 0) {
                1
            } else {
                2
            }
        })
    }

    fn score_pr(pr_opt: Option<u32>) -> Option<u32> {
        pr_opt.map(|pr| match pr {
            ..=40 => 3,
            41..=50 => 1,
            51..=90 => 0,
            91..=110 => 1,
            111..=139 => 2,
            140.. => 3,
        })
    }

    fn score_rr(rr_opt: Option<u32>) -> Option<u32> {
        rr_opt.map(|rr| match rr {
            ..=11 => 1,
            12..=20 => 0,
            21..=24 => 2,
            25.. => 3,
        })
    }

    pub fn score_sbp(sbp_opt: Option<u32>) -> Option<u32> {
        sbp_opt.map(|sbp| match sbp {
            ..=90 => 3,
            91..=100 => 2,
            101..=110 => 1,
            111..=219 => 0,
            220.. => 3,
        })
    }

    fn score_sat(sat_opt: Option<u32>) -> Option<u32> {
        sat_opt.map(|sat| match sat {
            0..=91 => 3,
            92..=93 => 2,
            94..=95 => 1,
            96.. => 0,
        })
    }

    fn score_o2(o2_opt: Option<u32>) -> Option<u32> {
        o2_opt.map(|_| 2).or(Some(0))
    }

    fn score_avpu(avpu_opt: Option<u32>) -> Option<u32> {
        avpu_opt.map(|avpu| match avpu {
            ..=1 => 0,
            2.. => 3,
        })
    }

    pub fn scoring(score_bt: Option<u32>, score_pr: Option<u32>, score_rr: Option<u32>, score_sbp: Option<u32>, score_sat: Option<u32>, score_o2: Option<u32>, score_avpu: Option<u32>) -> Option<u32> {
        if let (Some(bt), Some(pr), Some(rr), Some(sat), Some(avpu), Some(sbp), Some(o2)) = (score_bt, score_pr, score_rr, score_sbp, score_sat, score_o2, score_avpu) {
            Some(bt + pr + rr + sbp + sat + o2 + avpu)
        } else {
            None
        }
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(self.score_bt, self.score_pr, self.score_rr, self.score_sbp, self.score_sat, self.score_o2, self.score_avpu);
    }
}

impl super::Scorable for Snews {
    fn from_concat(pipes: &[&str], _birthday: Option<Date>) -> Self {
        let bt = Decimal::from_str_exact(pipes[1]).ok();
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let sbp = pipes[4].parse::<u32>().ok();
        let sat = pipes[10].parse::<u32>().ok();
        let o2_id = pipes[11].parse::<u32>().ok();
        let avpu_id = pipes[13].parse::<u32>().ok();

        Self::new(bt, pr, rr, sbp, sat, o2_id, avpu_id)
    }

    fn from_vs(vs: &Rc<VitalSign>, _birthday: Option<time::Date>) -> Self {
        Self::new(vs.bt, vs.pr, vs.rr, vs.sbp, vs.sat, vs.o2_id, vs.avpu_id)
    }

    fn contains(item: &str) -> bool {
        ["bt", "pr", "rr", "sbp", "sat", "o2_id", "avpu_id"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "bt" => "S-NEWS: 36.0 - 38.0 is normal, [0-2]",
            "pr" => "S-NEWS: > 50 and ≤ 90 is normal, [0-3]",
            "rr" => "S-NEWS: 12 - 20 is normal, [0-3]",
            "sbp" => "S-NEWS: > 110 and < 220 is normal, [0-3]",
            "sat" => "S-NEWS: > 95 is normal, [0-3]",
            "o2_id" => "S-NEWS: Not using O\u{2082} is normal, [0,2]",
            "avpu_id" => "S-NEWS: Alert is normal, [0,3]",
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Snews {
    fn color_total(&self) -> &'static str {
        self.score.map(|score| if score < 8 { "black" } else { "white" }).unwrap_or("white")
    }

    fn bg_color_total(&self) -> &'static str {
        self.score
            .map(|score| match score {
                0..=2 => "lime",
                3..=4 => "gold",
                5..=7 => "salmon",
                8..=10 => "crimson",
                11.. => "indigo",
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
            "sbp" => self.score_sbp,
            "sat" => self.score_sat,
            "o2_id" => self.score_o2,
            "avpu_id" => self.score_avpu,
            _ => None,
        }
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
            "sbp" => {
                self.sbp = value.parse::<u32>().ok();
                self.score_sbp = Self::score_sbp(self.sbp);
            }
            "sat" => {
                self.sat = value.parse::<u32>().ok();
                self.score_sat = Self::score_sat(self.sat);
            }
            "o2_id" => {
                self.o2_id = value.parse::<u32>().ok();
                self.score_o2 = Self::score_o2(self.o2_id);
            }
            "avpu_id" => {
                self.avpu_id = value.parse::<u32>().ok();
                self.score_avpu = Self::score_avpu(self.avpu_id);
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
            ")",
        ]
        .concat()
    }
}
