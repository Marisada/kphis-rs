// Mahasarakham Newborn Early Warning Sign (NEWS)
// BT, PR, RR, SAT, BREATHING, AVPU, GUT
// age <= 1 month

use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;

use crate::{select_utils::SelectOption, vital_sign::VitalSign};

#[derive(Clone)]
pub struct News {
    bt: Option<Decimal>,
    pr: Option<u32>,
    rr: Option<u32>,
    sat: Option<u32>,
    breathing_id: Option<u32>,
    avpu_id: Option<u32>,
    gut_feeling_id: Option<u32>,

    score_bt: Option<u32>,
    score_pr: Option<u32>,
    score_rr: Option<u32>,
    score_sat: Option<u32>,
    score_breathing: Option<u32>,
    score_avpu: Option<u32>,
    score_gut_feeling: Option<u32>,

    score: Option<u32>,
}

impl News {
    #[allow(clippy::too_many_arguments)]
    pub fn new(bt: Option<Decimal>, pr: Option<u32>, rr: Option<u32>, sat: Option<u32>, breathing_id: Option<u32>, avpu_id: Option<u32>, gut_feeling_id: Option<u32>) -> Self {
        let score_bt = Self::score_bt(bt);
        let score_pr = Self::score_pr(pr);
        let score_rr = Self::score_rr(rr);
        let score_sat = Self::score_sat(sat);
        let score_breathing = Self::score_breathing(breathing_id);
        let score_avpu = Self::score_avpu(avpu_id);
        let score_gut_feeling = Self::score_gut_feeling(gut_feeling_id);
        let score = Self::scoring(score_bt, score_pr, score_rr, score_sat, score_breathing, score_avpu, score_gut_feeling);

        Self {
            bt,
            pr,
            rr,
            sat,
            breathing_id,
            avpu_id,
            gut_feeling_id,
            score_bt,
            score_pr,
            score_rr,
            score_sat,
            score_breathing,
            score_avpu,
            score_gut_feeling,
            score,
        }
    }

    fn score_bt(bt_opt: Option<Decimal>) -> Option<u32> {
        bt_opt.map(|bt| {
            if bt < Decimal::new(35, 0) {
                2
            } else if bt < Decimal::new(365, 1) {
                1
            } else if bt < Decimal::new(375, 1) {
                0
            } else if bt < Decimal::new(38, 0) {
                1
            } else {
                2
            }
        })
    }

    fn score_pr(pr_opt: Option<u32>) -> Option<u32> {
        pr_opt.map(|pr| match pr {
            ..=59 => 2,
            60..=100 => 1,
            101..=159 => 0,
            160..=180 => 1,
            181.. => 2,
        })
    }

    fn score_rr(rr_opt: Option<u32>) -> Option<u32> {
        rr_opt.map(|rr| match rr {
            ..=29 => 2,
            30..=39 => 1,
            40..=60 => 0,
            61..=80 => 1,
            81.. => 2,
        })
    }

    fn score_sat(sat_opt: Option<u32>) -> Option<u32> {
        sat_opt.map(|sat| match sat {
            0..=89 => 2,
            90..=94 => 1,
            95.. => 0,
        })
    }

    // Default value                    NEWS value
    // 1 No distress                    : No Grunting
    // 2 Audible grunt or wheeze        : ได้ยิน Grunting โดยใช้ Stethoscope
    // 3 Mild or Moderate Recession     : ได้ยิน Grunting โดยใช้ Stethoscope
    // 4 Stridor                        : ได้ยิน Grunting โดยไม่ได้ใช้ Stethoscope
    // 5 Severe Recession               : ได้ยิน Grunting โดยไม่ได้ใช้ Stethoscope
    fn score_breathing(breathing_opt: Option<u32>) -> Option<u32> {
        breathing_opt.map(|breathing| match breathing {
            ..=1 => 0,
            2..=3 => 1,
            4.. => 2,
        })
    }

    // Default value    NEWS value
    // 1 Alert          : ตื่นดี รับนมได้ปกติ
    // 2 Verbal         : ซึมลง
    // 3 Pain           : ไม่ตื่นดี
    // 4 Unresponse     : กล้ามเนื้ออ่อนแรง หยุดหายใจ หรือชัก
    fn score_avpu(avpu_opt: Option<u32>) -> Option<u32> {
        avpu_opt.map(|avpu| match avpu {
            ..=1 => 0,
            2..=3 => 1,
            4.. => 2,
        })
    }

    // Default value            NEWS value
    // 1 Well                   : รับนมได้ปกติ
    // 2 Low level concern      : ท้องอืด
    // 3 High level concern     : สำรอกนม รับไม่ได้
    // 4 Child looks unwell     : ถ่ายอุจจาระเป็นเลือด
    fn score_gut_feeling(gut_feeling_opt: Option<u32>) -> Option<u32> {
        gut_feeling_opt.map(|gut_feeling| match gut_feeling {
            ..=1 => 0,
            2..=3 => 1,
            4.. => 2,
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
    ) -> Option<u32> {
        if let (Some(bt), Some(pr), Some(rr), Some(sat), Some(avpu), Some(breathing), Some(gut_feeling)) = (score_bt, score_pr, score_rr, score_sat, score_breathing, score_avpu, score_gut_feeling) {
            Some(bt + pr + rr + sat + avpu + breathing + gut_feeling)
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
        );
    }
}

impl super::Scorable for News {
    fn from_concat(pipes: &[&str], _birthday: Option<Date>) -> Self {
        let bt = Decimal::from_str_exact(pipes[1]).ok();
        let pr = pipes[2].parse::<u32>().ok();
        let rr = pipes[3].parse::<u32>().ok();
        let sat = pipes[10].parse::<u32>().ok();
        let breathing_id = pipes[12].parse::<u32>().ok();
        let avpu_id = pipes[13].parse::<u32>().ok();
        let gut_feeling_id = pipes[14].parse::<u32>().ok();

        Self::new(bt, pr, rr, sat, breathing_id, avpu_id, gut_feeling_id)
    }

    fn from_vs(vs: &Rc<VitalSign>, _birthday: Option<time::Date>) -> Self {
        Self::new(vs.bt, vs.pr, vs.rr, vs.sat, vs.breathing_id, vs.avpu_id, vs.gut_feeling_id)
    }

    fn contains(item: &str) -> bool {
        ["bt", "pr", "rr", "sat", "breathing_id", "avpu_id", "gut_feeling_id"].contains(&item)
    }

    fn title_item(item: &str) -> &'static str {
        match item {
            "bt" => "NEWS: 36.5 - 37.4 is normal, [0-2]",
            "pr" => "NEWS: 100 - 160 is normal, [0-2]",
            "rr" => "NEWS: 40 - 60 is normal, [0-2]",
            "sat" => {
                "NEWS: แดง หรือ > 94 is normal, [0-2]\n  \
                1. แดง หรือ SpO\u{2082} > 94%\n  \
                2. เขียวปลายมือปลายเท้า หรือ SpO\u{2082} 90-94%\n  \
                3. เขียวรอบปาก หรือ SpO\u{2082} < 90%"
            }
            "breathing_id" => "NEWS: No distress is normal, [0-2]",
            "avpu_id" => "NEWS: No Grunting is normal, [0-2]",
            "gut_feeling_id" => "NEWS: 'รับนมได้ปกติ' is normal, [0-2]",
            _ => "",
        }
    }
}

impl super::ScoreDispatch for News {
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

    fn custom_select_option(&self, item: &str) -> Option<Vec<SelectOption>> {
        match item {
            "breathing_id" => Some(vec![
                SelectOption {
                    key: String::from("1"),
                    value: String::from("No Grunting"),
                },
                SelectOption {
                    key: String::from("2"),
                    value: String::from("ได้ยิน Grunting โดยใช้ Stethoscope"),
                },
                SelectOption {
                    key: String::from("4"),
                    value: String::from("ได้ยิน Grunting โดยไม่ได้ใช้ Stethoscope"),
                },
            ]),
            "avpu_id" => Some(vec![
                SelectOption {
                    key: String::from("1"),
                    value: String::from("ตื่นดี รับนมได้ปกติ"),
                },
                SelectOption {
                    key: String::from("2"),
                    value: String::from("ซึมลง ไม่ตื่นดี"),
                },
                SelectOption {
                    key: String::from("4"),
                    value: String::from("กล้ามเนื้ออ่อนแรง หยุดหายใจ หรือชัก"),
                },
            ]),
            "gut_feeling_id" => Some(vec![
                SelectOption {
                    key: String::from("1"),
                    value: String::from("รับนมได้ปกติ"),
                },
                SelectOption {
                    key: String::from("2"),
                    value: String::from("ท้องอืด"),
                },
                SelectOption {
                    key: String::from("3"),
                    value: String::from("สำรอกนม รับไม่ได้"),
                },
                SelectOption {
                    key: String::from("4"),
                    value: String::from("ถ่ายอุจจาระเป็นเลือด"),
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
            "sat" => self.score_sat,
            "breathing_id" => self.score_breathing,
            "avpu_id" => self.score_avpu,
            "gut_feeling_id" => self.score_gut_feeling,
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
            ")",
        ]
        .concat()
    }
}
