// Adult qSOFA
// SBP < 100
// RR > 20
// GCS < 15

use std::rc::Rc;

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub struct Qsofa {
    rr: Option<u32>,
    sbp: Option<u32>,
    eye: Option<i32>,
    verbal: Option<i32>, // T = 1
    movement: Option<i32>,

    score_rr: Option<bool>,
    score_sbp: Option<bool>,
    score_gcs: Option<bool>,

    score: Option<u32>,
}

impl Qsofa {
    pub fn new(rr: Option<u32>, sbp: Option<u32>, eye: Option<i32>, verbal: &Option<String>, movement: Option<i32>) -> Self {
        let verbal = verbal.as_ref().and_then(|v| v.parse::<i32>().ok().or(Some(1)));
        let score_rr = Self::score_rr(rr);
        let score_sbp = Self::score_sbp(sbp);
        let score_gcs = Self::score_gcs(eye, verbal, movement);
        let score = Self::scoring(score_rr, score_sbp, score_gcs);
        Self {
            rr,
            sbp,
            eye,
            verbal,
            movement,
            score_rr,
            score_sbp,
            score_gcs,
            score,
        }
    }

    fn score_rr(rr_opt: Option<u32>) -> Option<bool> {
        rr_opt.map(|rr| rr > 20)
    }

    fn score_sbp(sbp_opt: Option<u32>) -> Option<bool> {
        sbp_opt.map(|sbp| sbp < 100)
    }

    fn score_gcs(eye_opt: Option<i32>, verbal_opt: Option<i32>, movement_opt: Option<i32>) -> Option<bool> {
        let eye = eye_opt.map(|e| e == 4);
        let verbal = verbal_opt.map(|v| v == 5);
        let movement = movement_opt.map(|m| m == 6);
        if let (Some(e), Some(v), Some(m)) = (eye, verbal, movement) { Some(!(e && v && m)) } else { None }
    }

    fn scoring(score_rr: Option<bool>, score_sbp: Option<bool>, score_gcs: Option<bool>) -> Option<u32> {
        if let (Some(rr), Some(sbp), Some(gcs)) = (score_rr, score_sbp, score_gcs) {
            Some(rr as u32 + sbp as u32 + gcs as u32)
        } else {
            None
        }
    }

    fn rescore(&mut self) {
        self.score = Self::scoring(self.score_rr, self.score_sbp, self.score_gcs);
    }
}

impl super::Scorable for Qsofa {
    fn from_concat(pipes: &[&str], _birthday: Option<time::Date>) -> Self {
        let rr = pipes[3].parse::<u32>().ok();
        let sbp = pipes[4].parse::<u32>().ok();
        let eye = pipes[17].parse::<i32>().ok();
        let verbal = pipes[18].parse::<String>().ok();
        let movement = pipes[19].parse::<i32>().ok();

        Self::new(rr, sbp, eye, &verbal, movement)
    }

    fn from_vs(vs: &Rc<VitalSign>, _birthday: Option<time::Date>) -> Self {
        Self::new(vs.rr, vs.sbp, vs.eye, &vs.verbal, vs.movement)
    }

    fn contains(item: &str) -> bool {
        ["rr", "sbp", "eye", "verbal", "movement"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "rr" => "qSOFA: ≤ 20 is normal, [0-1]",
            "sbp" => "qSOFA: ≥ 100 is normal, [0-1]",
            "eye" => "qSOFA: E4V5M6 is normal, [0-1]",
            "verbal" => "qSOFA: E4V5M6 is normal, [0-1]",
            "movement" => "qSOFA: E4V5M6 is normal, [0-1]",
            _ => "",
        }
    }
}

impl super::ScoreDispatch for Qsofa {
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
            "rr" => self.score_rr,
            "sbp" => self.score_sbp,
            "eye" => self.score_gcs,
            "verbal" => self.score_gcs,
            "movement" => self.score_gcs,
            _ => None,
        }
        .map(|b| b as u32)
    }

    fn set_item(&mut self, item: &str, value: &str) {
        match item {
            "rr" => {
                self.rr = value.parse::<u32>().ok();
                self.score_rr = Self::score_rr(self.rr);
            }
            "sbp" => {
                self.sbp = value.parse::<u32>().ok();
                self.score_sbp = Self::score_sbp(self.sbp);
            }
            "eye" => {
                self.eye = value.parse::<i32>().ok();
                self.score_gcs = Self::score_gcs(self.eye, self.verbal, self.movement);
            }
            "verbal" => {
                self.verbal = value.parse::<i32>().ok().or(Some(1));
                self.score_gcs = Self::score_gcs(self.eye, self.verbal, self.movement);
            }
            "movement" => {
                self.movement = value.parse::<i32>().ok();
                self.score_gcs = Self::score_gcs(self.eye, self.verbal, self.movement);
            }
            _ => {}
        }
        self.rescore();
    }

    fn title(&self) -> String {
        [
            "RR(",
            &(self.score_rr.unwrap_or_default() as u32).to_string(),
            "), \
            SBP(",
            &(self.score_sbp.unwrap_or_default() as u32).to_string(),
            "), \
            GCS(",
            &(self.score_gcs.unwrap_or_default() as u32).to_string(),
            ")",
        ]
        .concat()
    }
}
