// adult score
pub mod mews; // kphis MEWS
pub mod qsofa; // standard qSOFA
pub mod sirs;
pub mod snews; // Mahasarakham S-NEWS // standard SIRS
// child score
pub mod lqsofa; // LqSOFA : https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7786830/
pub mod pews; // Phayakhaphum Phisai PEWS
pub mod pops; // kphis POPS
pub mod psirs; // pediatric SIRS : https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4913352/
// neonate score
pub mod news; // Mahasarakham NEWS

// calculate order by
// - neonate, child then adult
use derive_demo::Demo;
use enum_dispatch::enum_dispatch;
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;
use time::{Date, PrimitiveDateTime};
use utoipa::ToSchema;

use lqsofa::Lqsofa;
use mews::Mews;
use news::News;
use pews::Pews;
use pops::Pops;
use psirs::Psirs;
use qsofa::Qsofa;
use sirs::Sirs;
use snews::Snews;

use kphis_util::datetime::{date_8601, datetime_8601, js_now};

use crate::{app::AppState, select_utils::SelectOption, vital_sign::VitalSign};

pub trait Scorable {
    fn from_concat(pipes: &[&str], birthday: Option<time::Date>) -> Self
    where
        Self: std::marker::Sized;
    fn from_vs(vs: &Rc<VitalSign>, birthday: Option<time::Date>) -> Self
    where
        Self: std::marker::Sized;
    fn contains(item: &str) -> bool;
    fn title_item(item: &str) -> &'static str;
}

#[enum_dispatch(Score)]
pub trait ScoreDispatch {
    fn color_total(&self) -> &'static str;
    fn bg_color_total(&self) -> &'static str;
    fn color_item(&self, item: &str) -> &'static str;
    fn bg_color_item(&self, item: &str) -> &'static str;
    fn custom_select_option(&self, item: &str) -> Option<Vec<SelectOption>>;
    fn score(&self) -> Option<u32>;
    fn score_item(&self, item: &str) -> Option<u32>;
    fn set_item(&mut self, item: &str, value: &str);
    fn title(&self) -> String;
}

pub struct Unsupported;

impl ScoreDispatch for Unsupported {
    fn color_total(&self) -> &'static str {
        "white"
    }
    fn bg_color_total(&self) -> &'static str {
        "red"
    }
    fn color_item(&self, _item: &str) -> &'static str {
        "white"
    }
    fn bg_color_item(&self, _item: &str) -> &'static str {
        "red"
    }
    fn custom_select_option(&self, _item: &str) -> Option<Vec<SelectOption>> {
        None
    }
    fn score(&self) -> Option<u32> {
        None
    }
    fn score_item(&self, _item: &str) -> Option<u32> {
        None
    }
    fn set_item(&mut self, _item: &str, _value: &str) {}
    fn title(&self) -> String {
        String::new()
    }
}

/// Early Warning Score supported
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(SupportedScore::demo_sirs()))]
pub enum SupportedScore {
    Mews,
    Snews,
    Pops,
    Pews,
    News,
    Qsofa,
    LqSofa,
    Sirs,
    Psirs,
    Unsupported,
}

impl SupportedScore {
    pub fn new(text: &str) -> Self {
        match text {
            "MEWS" => Self::Mews,
            "S-NEWS" => Self::Snews,
            "POPS" => Self::Pops,
            "PEWS" => Self::Pews,
            "NEWS" => Self::News,
            "qSOFA" => Self::Qsofa,
            "LqSOFA" => Self::LqSofa,
            "SIRS" => Self::Sirs,
            "pSIRS" => Self::Psirs,
            _ => Self::Unsupported,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Mews => "MEWS",
            Self::Snews => "S-NEWS",
            Self::Pops => "POPS",
            Self::Pews => "PEWS",
            Self::News => "NEWS",
            Self::Qsofa => "qSOFA",
            Self::LqSofa => "LqSOFA",
            Self::Sirs => "SIRS",
            Self::Psirs => "pSIRS",
            Self::Unsupported => "",
        }
    }

    fn can_score(&self, birthday: Option<Date>, regday: Date) -> bool {
        birthday
            .map(|day| {
                let age_d = (regday - day).whole_days();
                let age_y = age_d / 365;
                match self {
                    Self::News => age_d < 31,
                    Self::Pops | Self::Pews => age_y <= 15,
                    Self::LqSofa => age_y <= 17,
                    Self::Psirs => age_y <= 18,
                    Self::Mews | Self::Snews | Self::Qsofa | Self::Sirs => true,
                    Self::Unsupported => false,
                }
            })
            .unwrap_or_default()
    }

    fn score_from_concat(&self, pipes: &[&str], birthday: Option<time::Date>) -> Score {
        match self {
            Self::Mews => Score::Mews(Mews::from_concat(pipes, birthday)),
            Self::Snews => Score::Snews(Snews::from_concat(pipes, birthday)),
            Self::Pops => Score::Pops(Pops::from_concat(pipes, birthday)),
            Self::Pews => Score::Pews(Pews::from_concat(pipes, birthday)),
            Self::News => Score::News(News::from_concat(pipes, birthday)),
            Self::Qsofa => Score::Qsofa(Qsofa::from_concat(pipes, birthday)),
            Self::LqSofa => Score::LqSofa(Lqsofa::from_concat(pipes, birthday)),
            Self::Sirs => Score::Sirs(Sirs::from_concat(pipes, birthday)),
            Self::Psirs => Score::Psirs(Psirs::from_concat(pipes, birthday)),
            Self::Unsupported => Score::Unsupported(Unsupported),
        }
    }

    fn score_from_vs(&self, vs: &Rc<VitalSign>, birthday: Option<time::Date>) -> Score {
        match self {
            Self::Mews => Score::Mews(Mews::from_vs(vs, birthday)),
            Self::Snews => Score::Snews(Snews::from_vs(vs, birthday)),
            Self::Pops => Score::Pops(Pops::from_vs(vs, birthday)),
            Self::Pews => Score::Pews(Pews::from_vs(vs, birthday)),
            Self::News => Score::News(News::from_vs(vs, birthday)),
            Self::Qsofa => Score::Qsofa(Qsofa::from_vs(vs, birthday)),
            Self::LqSofa => Score::LqSofa(Lqsofa::from_vs(vs, birthday)),
            Self::Sirs => Score::Sirs(Sirs::from_vs(vs, birthday)),
            Self::Psirs => Score::Psirs(Psirs::from_vs(vs, birthday)),
            Self::Unsupported => Score::Unsupported(Unsupported),
        }
    }
}

#[enum_dispatch]
pub enum Score {
    Mews(Mews),
    Snews(Snews),
    Pops(Pops),
    Pews(Pews),
    News(News),
    Qsofa(Qsofa),
    LqSofa(Lqsofa),
    Sirs(Sirs),
    Psirs(Psirs),
    Unsupported(Unsupported),
}

impl Score {
    pub fn contains(&self, item: &str) -> bool {
        match self {
            Self::Mews(_) => Mews::contains(item),
            Self::Snews(_) => Snews::contains(item),
            Self::Pops(_) => Pops::contains(item),
            Self::Pews(_) => Pews::contains(item),
            Self::News(_) => News::contains(item),
            Self::Qsofa(_) => Qsofa::contains(item),
            Self::LqSofa(_) => Lqsofa::contains(item),
            Self::Sirs(_) => Sirs::contains(item),
            Self::Psirs(_) => Psirs::contains(item),
            Self::Unsupported(_) => false,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Mews(_) => "MEWS",
            Self::Snews(_) => "S-NEWS",
            Self::Pops(_) => "POPS",
            Self::Pews(_) => "PEWS",
            Self::News(_) => "NEWS",
            Self::Qsofa(_) => "qSOFA",
            Self::LqSofa(_) => "LqSOFA",
            Self::Sirs(_) => "SIRS",
            Self::Psirs(_) => "pSIRS",
            Self::Unsupported(_) => "",
        }
    }

    pub fn title_item(&self, item: &str) -> &'static str {
        match self {
            Self::Mews(_) => Mews::title_item(item),
            Self::Snews(_) => Snews::title_item(item),
            Self::Pops(_) => Pops::title_item(item),
            Self::Pews(_) => Pews::title_item(item),
            Self::News(_) => Pews::title_item(item),
            Self::Qsofa(_) => Qsofa::title_item(item),
            Self::LqSofa(_) => Lqsofa::title_item(item),
            Self::Sirs(_) => Sirs::title_item(item),
            Self::Psirs(_) => Psirs::title_item(item),
            Self::Unsupported(_) => "",
        }
    }
}

pub struct Scores {
    pub vs_datetime: Option<PrimitiveDateTime>,
    pub ews: Score,
    pub qsofa: Score,
    pub sirs: Score,
}

impl Scores {
    pub fn from_concat(pipe_opt: &Option<String>, birthday: Option<Date>, app: Rc<AppState>) -> Option<Self> {
        app.scores_tuple().and_then(|(ewss, qsofas, sirss)| {
            pipe_opt.as_ref().map(|pipe| pipe.split('|').collect::<Vec<&str>>()).and_then(|pipes| {
                (pipes.len() == CONCAT_LEN).then(|| {
                    let regday = date_8601(pipes[0]).unwrap_or(js_now().date());
                    // let age_y = birthday.map(|day| ((regday - day).whole_days() / 365) as i8);
                    let ews = ewss.iter().find(|s| s.can_score(birthday, regday)).unwrap_or(&SupportedScore::Mews);
                    let qsofa = qsofas.iter().find(|s| s.can_score(birthday, regday)).unwrap_or(&SupportedScore::Qsofa);
                    let sirs = sirss.iter().find(|s| s.can_score(birthday, regday)).unwrap_or(&SupportedScore::Sirs);

                    Self {
                        vs_datetime: datetime_8601(pipes[0]),
                        ews: ews.score_from_concat(&pipes, birthday),
                        qsofa: qsofa.score_from_concat(&pipes, birthday),
                        sirs: sirs.score_from_concat(&pipes, birthday),
                    }
                })
            })
        })
    }

    pub fn from_vs(vs: &Rc<VitalSign>, birthday: Option<Date>, app: Rc<AppState>) -> Option<Self> {
        app.scores_tuple().map(|(ewss, qsofas, sirss)| {
            let vs_date = vs.vs_datetime.date();
            // let age_y = birthday.map(|day| ((vs_date - day).whole_days() / 365) as i8);
            let ews = ewss.iter().find(|s| s.can_score(birthday, vs_date)).unwrap_or(&SupportedScore::Mews);
            let qsofa = qsofas.iter().find(|s| s.can_score(birthday, vs_date)).unwrap_or(&SupportedScore::Qsofa);
            let sirs = sirss.iter().find(|s| s.can_score(birthday, vs_date)).unwrap_or(&SupportedScore::Sirs);

            Self {
                vs_datetime: Some(vs.vs_datetime),
                ews: ews.score_from_vs(vs, birthday),
                qsofa: qsofa.score_from_vs(vs, birthday),
                sirs: sirs.score_from_vs(vs, birthday),
            }
        })
    }

    pub fn contains(&self, item: &str) -> bool {
        self.ews.contains(item) || self.qsofa.contains(item) || self.sirs.contains(item)
    }

    pub fn table_header(&self) -> String {
        [self.ews.label(), self.qsofa.label(), self.sirs.label()].join(", ")
    }

    pub fn set_item(&mut self, item: &str, value: &str) {
        self.ews.set_item(item, value);
        self.qsofa.set_item(item, value);
        self.sirs.set_item(item, value);
    }
}

// This *PART* of sub-query will collect all arguments needed to create all score supported by this crate
// ** NEED TO CHECK FILEDS NEEDED IN 'kphis.ipd_vs_vital_sign' AND 'kphis.opd_er_vs_vital_sign' TABLE **
// SELECT CONCAT(
//     IFNULL(vs_datetime,''),'|',
//     IFNULL(bt,''),'|',
//     IFNULL(pr,''),'|',
//     IFNULL(rr,''),'|',
//     IFNULL(sbp,''),'|',
//     IFNULL(inotrope,''),'|',
//     IFNULL(respirator,''),'|',
//     IFNULL(conscious_id,''),'|',
//     IFNULL(urine_amount,''),'|',
//     IFNULL(urine_duration,''),'|',
//     IFNULL(sat,''),'|',
//     IFNULL(o2_id,''),'|',
//     IFNULL(breathing_id,''),'|',
//     IFNULL(avpu_id,''),'|',
//     IFNULL(gut_feeling_id,''),'|',
//     IFNULL(pops_other_id,''),'|',
//     IFNULL(wbc,''),'|',
//     IFNULL(eye,''),'|',
//     IFNULL(verbal,''),'|',
//     IFNULL(movement,''),'|',
//     IFNULL(crt,''),'|',
//     IFNULL(band,'')
// ) FROM kphis.ipd_vs_vital_sign ORDER BY vs_datetime DESC LIMIT 1
pub const CONCAT_SQL: &str = "\
    CONCAT(\
        IFNULL(vs_datetime,''),'|',\
        IFNULL(bt,''),'|',\
        IFNULL(pr,''),'|',\
        IFNULL(rr,''),'|',\
        IFNULL(sbp,''),'|',\
        IFNULL(inotrope,''),'|',\
        IFNULL(respirator,''),'|',\
        IFNULL(conscious_id,''),'|',\
        IFNULL(urine_amount,''),'|',\
        IFNULL(urine_duration,''),'|',\
        IFNULL(sat,''),'|',\
        IFNULL(o2_id,''),'|',\
        IFNULL(breathing_id,''),'|',\
        IFNULL(avpu_id,''),'|',\
        IFNULL(gut_feeling_id,''),'|',\
        IFNULL(pops_other_id,''),'|',\
        IFNULL(wbc,''),'|',\
        IFNULL(eye,''),'|',\
        IFNULL(verbal,''),'|',\
        IFNULL(movement,''),'|',\
        IFNULL(crt,''),'|',\
        IFNULL(band,'')\
    )";
pub const CONCAT_LEN: usize = 22;
pub const CONCAT_EMPTY: &str = "|||||||||||||||||||||";
