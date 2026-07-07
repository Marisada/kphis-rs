// KPHIS's MEWS score

use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub struct Mews {
    bt: Option<Decimal>,
    pr: Option<u32>,
    rr: Option<u32>,
    sbp: Option<u32>,
    inotrope: bool,
    respirator: bool,
    conscious_id: Option<u32>,
    urine_amount: Option<u32>,
    urine_duration: Option<u32>,

    score_bt: Option<u32>,
    score_pr: Option<u32>,
    score_rr: Option<u32>,
    score_sbp: Option<u32>,
    score_conscious: Option<u32>,
    score_urine: Option<u32>,

    score: Option<u32>,
}

impl Mews {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bt: Option<Decimal>,
        pr: Option<u32>,
        rr: Option<u32>,
        sbp: Option<u32>,
        inotrope: bool,
        respirator: bool,
        conscious_id: Option<u32>,
        urine_amount: Option<u32>,
        urine_duration: Option<u32>,
    ) -> Self {
        let score_bt = Self::score_bt(bt);
        let score_pr = Self::score_pr(pr);
        let score_rr = Self::score_rr(rr, respirator);
        let score_sbp = Self::score_sbp(sbp, inotrope);
        let score_conscious = Self::score_conscious(conscious_id);
        let score_urine = Self::score_urine(urine_amount, urine_duration);
        let score = Self::scoring(score_bt, score_pr, score_rr, score_sbp, score_conscious, score_urine);

        Self {
            bt,
            pr,
            rr,
            sbp,
            inotrope,
            respirator,
            conscious_id,
            urine_amount,
            urine_duration,
            score_bt,
            score_pr,
            score_rr,
            score_sbp,
            score_conscious,
            score_urine,
            score,
        }
    }

    fn score_bt(bt_opt: Option<Decimal>) -> Option<u32> {
        bt_opt.map(|bt| {
            if bt <= Decimal::new(35, 0) {
                2
            } else if bt <= Decimal::new(36, 0) {
                1
            } else if bt <= Decimal::new(38, 0) {
                0
            } else if bt < Decimal::new(385, 1) {
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
            51..=100 => 0,
            101..=120 => 1,
            121..=139 => 2,
            140.. => 3,
        })
    }

    fn score_rr(rr_opt: Option<u32>, respirator: bool) -> Option<u32> {
        rr_opt.map(|rr| {
            if respirator && (9..=25).contains(&rr) {
                2
            } else {
                match rr {
                    ..=8 => 3,
                    9..=20 => 0,
                    21..=25 => 1,
                    26..=34 => 2,
                    35.. => 3,
                }
            }
        })
    }

    fn score_sbp(sbp_opt: Option<u32>, inotrope: bool) -> Option<u32> {
        sbp_opt.map(|sbp| {
            if inotrope {
                3
            } else {
                match sbp {
                    ..=80 => 3,
                    81..=90 => 2,
                    91..=100 => 1,
                    101..=180 => 0,
                    181..=199 => 1,
                    200.. => 2,
                }
            }
        })
    }

    fn score_conscious(conscious_opt: Option<u32>) -> Option<u32> {
        conscious_opt.map(|cons| match cons {
            0..=1 => 0,
            2..=4 => 1,
            5 => 2,
            6.. => 3,
        })
    }

    fn score_urine(urine_amount_opt: Option<u32>, urine_duration_opt: Option<u32>) -> Option<u32> {
        match (urine_amount_opt, urine_duration_opt) {
            (Some(ua), Some(ud)) => Some(match (ua, ud) {
                (2, 1) => 1,
                (3, 1) => 2,
                (3, 2) => 1,
                (4, 1..=2) => 2,
                (4, 3) => 1,
                (5..=6, 1..=3) => 2,
                (6, 4) => 1,
                (7, _) => 2,
                (_, _) => 0,
            }),
            _ => None,
        }
    }

    fn scoring(score_bt: Option<u32>, score_pr: Option<u32>, score_rr: Option<u32>, score_sbp: Option<u32>, score_conscious: Option<u32>, score_urine: Option<u32>) -> Option<u32> {
        if let (Some(bt), Some(pr), Some(rr), Some(sbp), Some(conscious), Some(urine)) = (score_bt, score_pr, score_rr, score_sbp, score_conscious, score_urine) {
            Some(bt + pr + rr + sbp + conscious + urine)
        } else {
            None
        }
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(self.score_bt, self.score_pr, self.score_rr, self.score_sbp, self.score_conscious, self.score_urine);
    }
}

impl super::Scorable for Mews {
    fn from_concat(pipes: &[&str], _birthday: Option<Date>) -> Self {
        let bt = Decimal::from_str_exact(pipes[1]).ok();
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let sbp = pipes[4].parse::<u32>().ok();
        let inotrope = pipes[5] == "Y";
        let respirator = pipes[6] == "Y";
        let conscious_id = pipes[7].parse::<u32>().ok();
        let urine_amount = pipes[8].parse::<u32>().ok();
        let urine_duration = pipes[9].parse::<u32>().ok();

        Self::new(bt, pr, rr, sbp, inotrope, respirator, conscious_id, urine_amount, urine_duration)
    }

    fn from_vs(vs: &Rc<VitalSign>, _birthday: Option<time::Date>) -> Self {
        Self::new(
            vs.bt,
            vs.pr,
            vs.rr,
            vs.sbp,
            vs.inotrope.as_ref().map(|s| s == "Y").unwrap_or_default(),
            vs.respirator.as_ref().map(|s| s == "Y").unwrap_or_default(),
            vs.conscious_id,
            vs.urine_amount,
            vs.urine_duration,
        )
    }

    fn contains(item: &str) -> bool {
        ["bt", "pr", "rr", "sbp", "inotrope", "respirator", "conscious_id", "urine_amount", "urine_duration"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "bt" => "MEWS: > 36.0 and ≤ 38.0 is normal, [0-2]",
            "pr" => "MEWS: > 50 and ≤ 100 is normal, [0-3]",
            "rr" => "MEWS: > 8 and ≤ 20 is normal, calculated with 'ใส่เครื่องช่วยหายใจ', [0-3]",
            "sbp" => "MEWS: > 100 and ≤ 180 is normal, calculated with 'ให้ยากระตุ้นความดันโลหิต', [0-3]",
            "inotrope" => "MEWS: Using inotrope will set score to 3",
            "respirator" => "MEWS: Using respitator will set minimum score to 2",
            "conscious_id" => "MEWS: 'รู้สึกตัวดี' is normal, [0-3]",
            "urine_amount" => "MEWS: Calculated with 'Urine (ระยะเวลา)', [0-2]",
            "urine_duration" => "MEWS: Calculated with 'Urine (ปริมาณ)', [0-2]",
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Mews {
    fn color_total(&self) -> &'static str {
        self.score.map(|score| if score < 8 { "black" } else { "white" }).unwrap_or("white")
    }

    fn bg_color_total(&self) -> &'static str {
        self.score
            .map(|score| match score {
                0 => "lime",
                1..=3 => "gold",
                4..=7 => "salmon",
                8.. => "crimson",
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
            "inotrope" => self.score_sbp,
            "respirator" => self.score_rr,
            "conscious_id" => self.score_conscious,
            "urine_amount" => self.score_urine,
            "urine_duration" => self.score_urine,
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
                self.score_rr = Self::score_rr(self.rr, self.respirator);
            }
            "sbp" => {
                self.sbp = value.parse::<u32>().ok();
                self.score_sbp = Self::score_sbp(self.sbp, self.inotrope);
            }
            "inotrope" => {
                self.inotrope = value == "Y";
                self.score_sbp = Self::score_sbp(self.sbp, self.inotrope);
            }
            "respirator" => {
                self.respirator = value == "Y";
                self.score_rr = Self::score_rr(self.rr, self.respirator);
            }
            "conscious_id" => {
                self.conscious_id = value.parse::<u32>().ok();
                self.score_conscious = Self::score_conscious(self.conscious_id);
            }
            "urine_amount" => {
                self.urine_amount = value.parse::<u32>().ok();
                self.score_urine = Self::score_urine(self.urine_amount, self.urine_duration);
            }
            "urine_duration" => {
                self.urine_duration = value.parse::<u32>().ok();
                self.score_urine = Self::score_urine(self.urine_amount, self.urine_duration);
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
            CONS(",
            &self.score_conscious.unwrap_or_default().to_string(),
            "), \
            URINE(",
            &self.score_urine.unwrap_or_default().to_string(),
            ")",
        ]
        .concat()
    }
}
