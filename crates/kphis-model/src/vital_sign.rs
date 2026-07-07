use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow, MySql, Pool,
    mysql::MySqlQueryResult,
    types::{Decimal, time::PrimitiveDateTime},
};
use sqlx_binder::MySqlBinder;
use std::rc::Rc;
use time::{Date, macros::datetime};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::date_8601,
    error::{AppError, Source},
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

#[derive(Clone, Default, PartialEq)]
pub enum VsMode {
    #[default]
    General,
    Labour,
    Neuro,
    Psychia,
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct VitalSignParams {
    pub vs_id: Option<u32>,
    pub hn: Option<String>,
    pub an: Option<String>,
    pub opd_er_order_master_id: Option<u32>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}

impl QueryString for VitalSignParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            vs_id: find_qs(params, "vs_id").and_then(|s| s.parse::<u32>().ok()),
            hn: find_qs(params, "hn"),
            an: find_qs(params, "an"),
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            start_date: find_qs(params, "start_date").and_then(|s| date_8601(&s)),
            end_date: find_qs(params, "end_date").and_then(|s| date_8601(&s)),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(6);
        if let Some(vs_id) = &self.vs_id {
            queries.push(["vs_id=", &vs_id.to_string()].concat());
        }
        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(start_date) = &self.start_date {
            queries.push(["start_date=", &start_date.to_string()].concat());
        }
        if let Some(end_date) = &self.end_date {
            queries.push(["end_date=", &end_date.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Vital Sign for save
#[derive(Clone, Demo, Deserialize, Serialize, MySqlBinder, ToSchema)]
#[schema(example = json!(VitalSignSave::demo()))]
pub struct VitalSignSave {
    #[Demo(value = "Some(1)")]
    pub action_id: Option<u32>,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub vs_datetime: PrimitiveDateTime,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub bt: Option<Decimal>,
    #[Demo(value = "Some(80)")]
    pub pr: Option<u32>,
    #[Demo(value = "Some(20)")]
    pub rr: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub respirator: Option<String>,
    #[Demo(value = "Some(120)")]
    pub sbp: Option<u32>,
    #[Demo(value = "Some(80)")]
    pub dbp: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inotrope: Option<String>,
    #[Demo(value = "Some(87)")]
    pub map: Option<i32>,
    #[Demo(value = "Some(99)")]
    pub sat: Option<u32>,
    #[Demo(value = "Some(94)")]
    pub sat_room_air: Option<u32>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub cvp: Option<String>,
    #[Demo(value = "Some(40)")]
    pub end_co2: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub conscious_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub bw: Option<Decimal>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub urine: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub catheter: Option<String>,
    #[Demo(value = "Some(1)")]
    pub urine_amount: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_duration: Option<u32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub feces: Option<String>,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub head: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub t_inc: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub line_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(75,1))")]
    pub line_no: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(20,0))")]
    pub line_mark: Option<Decimal>,
    /// total,item1,..,item6
    #[Demo(value = r#"Some(String::from("23,4,4,4,4,4,3"))"#)]
    pub braden: Option<String>,
    #[Demo(value = "Some(1)")]
    pub pain: Option<i32>,
    #[Demo(value = "Some(4)")]
    pub eye: Option<i32>,
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub verbal: Option<String>,
    #[Demo(value = "Some(6)")]
    pub movement: Option<i32>,
    #[Demo(value = "Some(Decimal::new(2,0))")]
    pub right_pupil: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub right_cha_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(2,0))")]
    pub left_pupil: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub left_cha_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub va_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub mass_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lt_arm: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lt_leg: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub rt_arm: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub rt_leg: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub severity: Option<i32>,
    #[Demo(value = r#"Some(String::from("Levophed 4:250"))"#)]
    pub had_name: Option<String>,
    #[Demo(value = r#"Some(String::from("10"))"#)]
    pub had_drop: Option<String>,
    #[Demo(value = "Some(Decimal::new(37,0))")]
    pub hct: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("88"))"#)]
    pub dtx: Option<String>,
    #[Demo(value = "Some(Decimal::new(44,1))")]
    pub bl: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(123,1))")]
    pub mcb: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub suction: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub nb: Option<String>,
    #[Demo(value = "Some(1)")]
    pub o2_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(10,0))")]
    pub o2_flow: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub tube_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(75,1))")]
    pub tube_no: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(22,0))")]
    pub tube_mark: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("BYRD"))"#)]
    pub ventilator_name: Option<String>,
    #[Demo(value = r#"Some(String::from("AUTO"))"#)]
    pub mode: Option<String>,
    #[Demo(value = "Some(1500)")]
    pub tv: Option<i32>,
    #[Demo(value = "Some(5)")]
    pub pip: Option<i32>,
    #[Demo(value = "Some(12)")]
    pub r_rate: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub i_rate: Option<i32>,
    #[Demo(value = "Some(3)")]
    pub e_rate: Option<i32>,
    #[Demo(value = "Some(Decimal::new(15,1))")]
    pub ti: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub ps: Option<i32>,
    #[Demo(value = "Some(Decimal::new(30,2))")]
    pub fio2: Option<Decimal>,
    #[Demo(value = "Some(5)")]
    pub peep: Option<i32>,
    #[Demo(value = "Some(Decimal::new(5,1))")]
    pub ft: Option<Decimal>,
    #[Demo(value = "Some(6)")]
    pub delta_p: Option<i32>,
    #[Demo(value = "Some(4)")]
    pub o2_map: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub intake_id: Option<u32>,
    // NOT USE
    #[Demo(default)]
    pub intake_type: Option<String>,
    // NOT USE
    #[Demo(default)]
    pub intake_amount: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub intake_absorb: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub output_id: Option<u32>,
    // NOT USE
    #[Demo(default)]
    pub output_amount: Option<i32>,
    #[Demo(value = r#"Some(String::from("Other Note"))"#)]
    pub other: Option<String>,
    #[Demo(value = r#"Some(String::from("3'45\""))"#)]
    pub lr_int: Option<String>,
    #[Demo(value = "Some(45)")]
    pub lr_dur: Option<i32>,
    #[Demo(value = "Some(140)")]
    pub lr_fsh: Option<i32>,
    #[Demo(value = r#"Some(String::from("2+"))"#)]
    pub lr_sev: Option<String>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub lr_cer: Option<String>,
    #[Demo(value = "Some(88)")]
    pub lr_eff: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub lr_sta: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_mem: Option<u32>,
    #[Demo(value = r#"Some(String::from("Clear"))"#)]
    pub lr_af: Option<String>,
    #[Demo(value = "Some(1)")]
    pub breathing_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub avpu_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub gut_feeling_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub pops_other_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(8888,0))")]
    pub wbc: Option<Decimal>,
    #[Demo(value = "Some(555)")]
    pub pleak_flow: Option<i32>,
    // different from original KPHIS
    #[Demo(value = "Some(1)")]
    pub crt: Option<i32>,
    #[Demo(value = "Some(11)")]
    pub band: Option<i32>,
    #[Demo(value = r#"Some(String::from("ROA"))"#)]
    pub lr_pos: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lr_moulding: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_oxytocin_unit: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_oxytocin_rate: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_urine_vol: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_protein: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_sugar: Option<u32>,
    #[Demo(value = r#"Some(String::from("NPO"))"#)]
    pub diet: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("100,10,10,10,10,10,10,10,10,10,10"))"#)]
    pub barthel_index: Option<String>,
    /// total,item1,..,item3
    #[Demo(value = r#"Some(String::from("3,2,3,1"))"#)]
    pub aggression_oas: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("67,7,7,7,7,7,7,7,7,7,4"))"#)]
    pub alcohol_ciwa: Option<String>,
    /// total,item1,..,item7
    #[Demo(value = r#"Some(String::from("27,4,3,4,4,4,4,4"))"#)]
    pub alcohol_aws: Option<String>,
    /// total,h,a,r,item1,..,item10
    #[Demo(value = r#"Some(String::from("40,12,12,12,4,4,4,4,4,4,4,4,4,4"))"#)]
    pub amphetamine_awq: Option<String>,
    #[Demo(value = "Some(10)")]
    pub motivation_scale: Option<u8>,
    #[Demo(value = "Some(10)")]
    pub craving_scale: Option<u8>,
    #[Demo(value = "Some(6)")]
    pub stage_of_change_id: Option<u8>,
    /// total,item1,item2
    #[Demo(value = r#"Some(String::from("2,1,1"))"#)]
    pub depress_2q: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("27,3,3,3,3,3,3,3,3,3"))"#)]
    pub depress_9q: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("52,1,2,6,8,8,9,4,10,4"))"#)]
    pub suicide_8q: Option<String>,

    #[Demo(value = "1")]
    pub vs_id: u32, // AUTO_INCREMENT,
}

impl VitalSignSave {
    /// - POST `EndPoint::IpdVitalSign`
    /// - PUT `EndPoint::IpdVitalSign`
    /// - POST `EndPoint::OpdErVitalSign`
    /// - PUT `EndPoint::OpdErVitalSign`
    pub async fn call_api_save(&self, is_ipd: bool, method: &str, params: &VitalSignParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let ep = if is_ipd { EndPoint::IpdVitalSign } else { EndPoint::OpdErVitalSign };
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send VitalSign"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send VitalSign"))?;

        execute_fetch(&[ep.base(), params.query_string()].concat(), method, Some(&body), app).await
    }

    // new with vs_id == 0
    pub fn new(vs_datetime: PrimitiveDateTime) -> Self {
        Self {
            vs_id: 0,
            action_id: None,
            vs_datetime,
            bt: None,
            pr: None,
            rr: None,
            respirator: None,
            sbp: None,
            dbp: None,
            inotrope: None,
            map: None,
            sat: None,
            sat_room_air: None,
            cvp: None,
            end_co2: None,
            conscious_id: None,
            bw: None,
            height: None,
            urine: None,
            catheter: None,
            urine_amount: None,
            urine_duration: None,
            feces: None,
            head: None,
            t_inc: None,
            line_id: None,
            line_no: None,
            line_mark: None,
            braden: None,
            pain: None,
            eye: None,
            verbal: None,
            movement: None,
            right_pupil: None,
            right_cha_id: None,
            left_pupil: None,
            left_cha_id: None,
            va_id: None,
            mass_id: None,
            lt_arm: None,
            lt_leg: None,
            rt_arm: None,
            rt_leg: None,
            severity: None,
            had_name: None,
            had_drop: None,
            hct: None,
            dtx: None,
            bl: None,
            mcb: None,
            suction: None,
            nb: None,
            o2_id: None,
            o2_flow: None,
            tube_id: None,
            tube_no: None,
            tube_mark: None,
            ventilator_name: None,
            mode: None,
            tv: None,
            pip: None,
            r_rate: None,
            i_rate: None,
            e_rate: None,
            ti: None,
            ps: None,
            fio2: None,
            peep: None,
            ft: None,
            delta_p: None,
            o2_map: None,
            intake_id: None,
            intake_type: None,
            intake_amount: None,
            intake_absorb: None,
            output_id: None,
            output_amount: None,
            other: None,
            lr_int: None,
            lr_dur: None,
            lr_fsh: None,
            lr_sev: None,
            lr_cer: None,
            lr_eff: None,
            lr_sta: None,
            lr_mem: None,
            lr_af: None,
            breathing_id: None,
            avpu_id: None,
            gut_feeling_id: None,
            pops_other_id: None,
            wbc: None,
            pleak_flow: None,
            crt: None,
            band: None,
            lr_pos: None,
            lr_moulding: None,
            lr_oxytocin_unit: None,
            lr_oxytocin_rate: None,
            lr_urine_vol: None,
            urine_protein: None,
            urine_sugar: None,
            diet: None,
            barthel_index: None,
            aggression_oas: None,
            alcohol_ciwa: None,
            alcohol_aws: None,
            amphetamine_awq: None,
            motivation_scale: None,
            craving_scale: None,
            stage_of_change_id: None,
            depress_2q: None,
            depress_9q: None,
            suicide_8q: None,
        }
    }
}

/// Vital Sign with associated data
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(VitalSign::demo()))]
pub struct VitalSign {
    #[Demo(value = "1")]
    pub vs_id: u32,

    #[Demo(value = "Some(1)")]
    pub action_id: Option<u32>,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub vs_datetime: PrimitiveDateTime,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub bt: Option<Decimal>,
    #[Demo(value = "Some(80)")]
    pub pr: Option<u32>,
    #[Demo(value = "Some(20)")]
    pub rr: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub respirator: Option<String>,
    #[Demo(value = "Some(120)")]
    pub sbp: Option<u32>,
    #[Demo(value = "Some(80)")]
    pub dbp: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inotrope: Option<String>,
    #[Demo(value = "Some(87)")]
    pub map: Option<i32>,
    #[Demo(value = "Some(99)")]
    pub sat: Option<u32>,
    #[Demo(value = "Some(94)")]
    pub sat_room_air: Option<u32>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub cvp: Option<String>,
    #[Demo(value = "Some(40)")]
    pub end_co2: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub conscious_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub bw: Option<Decimal>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub urine: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub catheter: Option<String>,
    #[Demo(value = "Some(1)")]
    pub urine_amount: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_duration: Option<u32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub feces: Option<String>,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub head: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub t_inc: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub line_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(75,1))")]
    pub line_no: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(20,0))")]
    pub line_mark: Option<Decimal>,
    /// total,item1,..,item6
    #[Demo(value = r#"Some(String::from("23,4,4,4,4,4,3"))"#)]
    pub braden: Option<String>,
    #[Demo(value = "Some(1)")]
    pub pain: Option<i32>,
    #[Demo(value = "Some(4)")]
    pub eye: Option<i32>,
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub verbal: Option<String>,
    #[Demo(value = "Some(6)")]
    pub movement: Option<i32>,
    #[Demo(value = "Some(Decimal::new(2,0))")]
    pub right_pupil: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub right_cha_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(2,0))")]
    pub left_pupil: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub left_cha_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub va_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub mass_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lt_arm: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lt_leg: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub rt_arm: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub rt_leg: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub severity: Option<i32>,
    #[Demo(value = r#"Some(String::from("Levophed 4:250"))"#)]
    pub had_name: Option<String>,
    #[Demo(value = r#"Some(String::from("10"))"#)]
    pub had_drop: Option<String>,
    #[Demo(value = "Some(Decimal::new(37,0))")]
    pub hct: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("88"))"#)]
    pub dtx: Option<String>,
    #[Demo(value = "Some(Decimal::new(44,1))")]
    pub bl: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(123,1))")]
    pub mcb: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub suction: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub nb: Option<String>,
    #[Demo(value = "Some(1)")]
    pub o2_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(10,0))")]
    pub o2_flow: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub tube_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(75,1))")]
    pub tube_no: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(22,0))")]
    pub tube_mark: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("BYRD"))"#)]
    pub ventilator_name: Option<String>,
    #[Demo(value = r#"Some(String::from("AUTO"))"#)]
    pub mode: Option<String>,
    #[Demo(value = "Some(1500)")]
    pub tv: Option<i32>,
    #[Demo(value = "Some(5)")]
    pub pip: Option<i32>,
    #[Demo(value = "Some(12)")]
    pub r_rate: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub i_rate: Option<i32>,
    #[Demo(value = "Some(3)")]
    pub e_rate: Option<i32>,
    #[Demo(value = "Some(Decimal::new(15,1))")]
    pub ti: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub ps: Option<i32>,
    #[Demo(value = "Some(Decimal::new(30,2))")]
    pub fio2: Option<Decimal>,
    #[Demo(value = "Some(5)")]
    pub peep: Option<i32>,
    #[Demo(value = "Some(Decimal::new(5,1))")]
    pub ft: Option<Decimal>,
    #[Demo(value = "Some(6)")]
    pub delta_p: Option<i32>,
    #[Demo(value = "Some(4)")]
    pub o2_map: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub intake_id: Option<u32>,
    // NOT USE
    #[Demo(default)]
    pub intake_type: Option<String>,
    // NOT USE
    #[Demo(default)]
    pub intake_amount: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub intake_absorb: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub output_id: Option<u32>,
    // NOT USE
    #[Demo(default)]
    pub output_amount: Option<i32>,
    #[Demo(value = r#"Some(String::from("other_vs Note"))"#)]
    pub other: Option<String>,
    #[Demo(value = r#"Some(String::from("3"))"#)]
    pub lr_int: Option<String>,
    #[Demo(value = "Some(45)")]
    pub lr_dur: Option<i32>,
    #[Demo(value = "Some(140)")]
    pub lr_fsh: Option<i32>,
    #[Demo(value = r#"Some(String::from("2+"))"#)]
    pub lr_sev: Option<String>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub lr_cer: Option<String>,
    #[Demo(value = "Some(88)")]
    pub lr_eff: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub lr_sta: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_mem: Option<u32>,
    #[Demo(value = r#"Some(String::from("Clear"))"#)]
    pub lr_af: Option<String>,
    #[Demo(value = "Some(1)")]
    pub breathing_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub avpu_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub gut_feeling_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub pops_other_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(8888,0))")]
    pub wbc: Option<Decimal>,
    #[Demo(value = "Some(555)")]
    pub pleak_flow: Option<i32>,
    // different from original KPHIS
    #[Demo(value = "Some(1)")]
    pub crt: Option<i32>,
    #[Demo(value = "Some(11)")]
    pub band: Option<i32>,
    #[Demo(value = r#"Some(String::from("ROA"))"#)]
    pub lr_pos: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lr_moulding: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_oxytocin_unit: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_oxytocin_rate: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_urine_vol: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_protein: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_sugar: Option<u32>,
    #[Demo(value = r#"Some(String::from("NPO"))"#)]
    pub diet: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("100,10,10,10,10,10,10,10,10,10,10"))"#)]
    pub barthel_index: Option<String>,
    /// total,item1,..,item3
    #[Demo(value = r#"Some(String::from("3,2,3,1"))"#)]
    pub aggression_oas: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("67,7,7,7,7,7,7,7,7,7,4"))"#)]
    pub alcohol_ciwa: Option<String>,
    /// total,item1,..,item7
    #[Demo(value = r#"Some(String::from("27,4,3,4,4,4,4,4"))"#)]
    pub alcohol_aws: Option<String>,
    /// total,h,a,r,item1,..,item10
    #[Demo(value = r#"Some(String::from("40,12,12,12,4,4,4,4,4,4,4,4,4,4"))"#)]
    pub amphetamine_awq: Option<String>,
    #[Demo(value = "Some(10)")]
    pub motivation_scale: Option<u8>,
    #[Demo(value = "Some(10)")]
    pub craving_scale: Option<u8>,
    #[Demo(value = "Some(6)")]
    pub stage_of_change_id: Option<u8>,
    /// total,item1,item2
    #[Demo(value = r#"Some(String::from("2,1,1"))"#)]
    pub depress_2q: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("27,3,3,3,3,3,3,3,3,3"))"#)]
    pub depress_9q: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("52,1,2,6,8,8,9,4,10,4"))"#)]
    pub suicide_8q: Option<String>,

    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,

    #[Demo(value = r#"Some(String::from("user"))"#)]
    pub create_opduser_name: Option<String>,
    #[Demo(value = r#"Some(String::from("user"))"#)]
    pub update_opduser_name: Option<String>,
    #[Demo(value = r#"Some(String::from("รู้สึกตัวดี"))"#)]
    pub conscious_name: Option<String>,
    #[Demo(value = r#"Some(String::from("C-Line"))"#)]
    pub line_name: Option<String>,
    #[Demo(value = r#"Some(String::from("React to light"))"#)]
    pub left_cha_name: Option<String>,
    #[Demo(value = r#"Some(String::from("React to light"))"#)]
    pub right_cha_name: Option<String>,
    #[Demo(value = r#"Some(String::from("A"))"#)]
    pub va_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub mass_name: Option<String>,
    #[Demo(value = r#"Some(String::from("-2"))"#)]
    pub lr_sta_name: Option<String>,
    #[Demo(value = r#"Some(String::from("R"))"#)]
    pub lr_mem_name: Option<String>,
    #[Demo(value = r#"Some(String::from("+1"))"#)]
    pub lr_moulding_name: Option<String>,
    #[Demo(value = r#"Some(String::from("-ve"))"#)]
    pub urine_protein_name: Option<String>,
    #[Demo(value = r#"Some(String::from("-ve"))"#)]
    pub urine_sugar_name: Option<String>,
    #[Demo(value = r#"Some(String::from("I"))"#)]
    pub lt_arm_name: Option<String>,
    #[Demo(value = r#"Some(String::from("I"))"#)]
    pub lt_leg_name: Option<String>,
    #[Demo(value = r#"Some(String::from("I"))"#)]
    pub rt_arm_name: Option<String>,
    #[Demo(value = r#"Some(String::from("I"))"#)]
    pub rt_leg_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Canular"))"#)]
    pub o2_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ET-tube"))"#)]
    pub tube_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Relapse"))"#)]
    pub stage_of_change_name: Option<String>,
    // NOT USE
    #[Demo(default)]
    pub intake_name: Option<String>,
    // NOT USE
    #[Demo(default)]
    pub output_name: Option<String>,
}

impl VitalSign {
    /// GET `EndPoint::IpdVitalSign`<br>
    /// GET `EndPoint::OpdErVitalSign`
    pub async fn call_api_get(is_ipd: bool, params: &VitalSignParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        let endpoint = if is_ipd { EndPoint::IpdVitalSign } else { EndPoint::OpdErVitalSign };
        match fetch_json_api(&[endpoint.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch VitalSign"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch VitalSign"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdVitalSignId`<br>
    /// DELETE `EndPoint::OpdErVitalSignId`
    pub async fn call_api_delete(is_ipd: bool, vs_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let endpoint = if is_ipd { EndPoint::IpdVitalSignId } else { EndPoint::OpdErVitalSignId };
        execute_fetch(&[endpoint.base(), vs_id.to_string()].concat(), "DELETE", None, app).await
    }
}

#[derive(Demo, Deserialize, Serialize, FromRow, MySqlBinder)]
pub struct VitalSignOnly {
    #[Demo(value = "Some(1)")]
    pub action_id: Option<u32>,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub vs_datetime: PrimitiveDateTime,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub bt: Option<Decimal>,
    #[Demo(value = "Some(80)")]
    pub pr: Option<u32>,
    #[Demo(value = "Some(20)")]
    pub rr: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub respirator: Option<String>,
    #[Demo(value = "Some(120)")]
    pub sbp: Option<u32>,
    #[Demo(value = "Some(80)")]
    pub dbp: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inotrope: Option<String>,
    #[Demo(value = "Some(87)")]
    pub map: Option<i32>,
    #[Demo(value = "Some(99)")]
    pub sat: Option<u32>,
    #[Demo(value = "Some(94)")]
    pub sat_room_air: Option<u32>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub cvp: Option<String>,
    #[Demo(value = "Some(40)")]
    pub end_co2: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub conscious_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub bw: Option<Decimal>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub urine: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub catheter: Option<String>,
    #[Demo(value = "Some(1)")]
    pub urine_amount: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_duration: Option<u32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub feces: Option<String>,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub head: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub t_inc: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub line_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(75,1))")]
    pub line_no: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(20,0))")]
    pub line_mark: Option<Decimal>,
    /// total,item1,..,item6
    #[Demo(value = r#"Some(String::from("23,4,4,4,4,4,3"))"#)]
    pub braden: Option<String>,
    #[Demo(value = "Some(1)")]
    pub pain: Option<i32>,
    #[Demo(value = "Some(4)")]
    pub eye: Option<i32>,
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub verbal: Option<String>,
    #[Demo(value = "Some(6)")]
    pub movement: Option<i32>,
    #[Demo(value = "Some(Decimal::new(2,0))")]
    pub right_pupil: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub right_cha_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(2,0))")]
    pub left_pupil: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub left_cha_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub va_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub mass_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lt_arm: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lt_leg: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub rt_arm: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub rt_leg: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub severity: Option<i32>,
    #[Demo(value = r#"Some(String::from("Levophed 4:250"))"#)]
    pub had_name: Option<String>,
    #[Demo(value = r#"Some(String::from("10"))"#)]
    pub had_drop: Option<String>,
    #[Demo(value = "Some(Decimal::new(37,0))")]
    pub hct: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("88"))"#)]
    pub dtx: Option<String>,
    #[Demo(value = "Some(Decimal::new(44,1))")]
    pub bl: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(123,1))")]
    pub mcb: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub suction: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub nb: Option<String>,
    #[Demo(value = "Some(1)")]
    pub o2_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(10,0))")]
    pub o2_flow: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub tube_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(75,1))")]
    pub tube_no: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(22,0))")]
    pub tube_mark: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("BYRD"))"#)]
    pub ventilator_name: Option<String>,
    #[Demo(value = r#"Some(String::from("AUTO"))"#)]
    pub mode: Option<String>,
    #[Demo(value = "Some(1500)")]
    pub tv: Option<i32>,
    #[Demo(value = "Some(5)")]
    pub pip: Option<i32>,
    #[Demo(value = "Some(12)")]
    pub r_rate: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub i_rate: Option<i32>,
    #[Demo(value = "Some(3)")]
    pub e_rate: Option<i32>,
    #[Demo(value = "Some(Decimal::new(15,1))")]
    pub ti: Option<Decimal>,
    #[Demo(value = "Some(1)")]
    pub ps: Option<i32>,
    #[Demo(value = "Some(Decimal::new(30,2))")]
    pub fio2: Option<Decimal>,
    #[Demo(value = "Some(5)")]
    pub peep: Option<i32>,
    #[Demo(value = "Some(Decimal::new(5,1))")]
    pub ft: Option<Decimal>,
    #[Demo(value = "Some(6)")]
    pub delta_p: Option<i32>,
    #[Demo(value = "Some(4)")]
    pub o2_map: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub intake_id: Option<u32>,
    // NOT USE
    #[Demo(default)]
    pub intake_type: Option<String>,
    // NOT USE
    #[Demo(default)]
    pub intake_amount: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub intake_absorb: Option<i32>,
    // NOT USE
    #[Demo(default)]
    pub output_id: Option<u32>,
    // NOT USE
    #[Demo(default)]
    pub output_amount: Option<i32>,
    #[Demo(value = r#"Some(String::from("other_vs Note"))"#)]
    pub other: Option<String>,
    #[Demo(value = r#"Some(String::from("3"))"#)]
    pub lr_int: Option<String>,
    #[Demo(value = "Some(45)")]
    pub lr_dur: Option<i32>,
    #[Demo(value = "Some(140)")]
    pub lr_fsh: Option<i32>,
    #[Demo(value = r#"Some(String::from("2+"))"#)]
    pub lr_sev: Option<String>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub lr_cer: Option<String>,
    #[Demo(value = "Some(88)")]
    pub lr_eff: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub lr_sta: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_mem: Option<u32>,
    #[Demo(value = r#"Some(String::from("Clear"))"#)]
    pub lr_af: Option<String>,
    #[Demo(value = "Some(1)")]
    pub breathing_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub avpu_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub gut_feeling_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub pops_other_id: Option<u32>,
    #[Demo(value = "Some(Decimal::new(8888,0))")]
    pub wbc: Option<Decimal>,
    #[Demo(value = "Some(555)")]
    pub pleak_flow: Option<i32>,
    // different from original KPHIS
    #[Demo(value = "Some(1)")]
    pub crt: Option<i32>,
    #[Demo(value = "Some(11)")]
    pub band: Option<i32>,
    #[Demo(value = r#"Some(String::from("ROA"))"#)]
    pub lr_pos: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lr_moulding: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_oxytocin_unit: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_oxytocin_rate: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub lr_urine_vol: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_protein: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub urine_sugar: Option<u32>,
    #[Demo(value = r#"Some(String::from("NPO"))"#)]
    pub diet: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("100,10,10,10,10,10,10,10,10,10,10"))"#)]
    pub barthel_index: Option<String>,
    /// total,item1,..,item3
    #[Demo(value = r#"Some(String::from("3,2,3,1"))"#)]
    pub aggression_oas: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("67,7,7,7,7,7,7,7,7,7,4"))"#)]
    pub alcohol_ciwa: Option<String>,
    /// total,item1,..,item7
    #[Demo(value = r#"Some(String::from("27,4,3,4,4,4,4,4"))"#)]
    pub alcohol_aws: Option<String>,
    /// total,h,a,r,item1,..,item10
    #[Demo(value = r#"Some(String::from("40,12,12,12,4,4,4,4,4,4,4,4,4,4"))"#)]
    pub amphetamine_awq: Option<String>,
    #[Demo(value = "Some(10)")]
    pub motivation_scale: Option<u8>,
    #[Demo(value = "Some(10)")]
    pub craving_scale: Option<u8>,
    #[Demo(value = "Some(6)")]
    pub stage_of_change_id: Option<u8>,
    /// total,item1,item2
    #[Demo(value = r#"Some(String::from("2,1,1"))"#)]
    pub depress_2q: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("27,3,3,3,3,3,3,3,3,3"))"#)]
    pub depress_9q: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("52,1,2,6,8,8,9,4,10,4"))"#)]
    pub suicide_8q: Option<String>,

    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,

    #[Demo(value = "1")]
    pub vs_id: u32,
}

impl PartialEq for VitalSignOnly {
    fn eq(&self, other_vs: &Self) -> bool {
        // self.vs_id == other_vs.vs_id &&
        self.action_id == other_vs.action_id
            && self.vs_datetime == other_vs.vs_datetime
            && self.bt == other_vs.bt
            && self.pr == other_vs.pr
            && self.rr == other_vs.rr
            && self.respirator == other_vs.respirator
            && self.sbp == other_vs.sbp
            && self.dbp == other_vs.dbp
            && self.inotrope == other_vs.inotrope
            && self.map == other_vs.map
            && self.sat == other_vs.sat
            && self.sat_room_air == other_vs.sat_room_air
            && self.cvp == other_vs.cvp
            && self.end_co2 == other_vs.end_co2
            && self.conscious_id == other_vs.conscious_id
            && self.bw == other_vs.bw
            && self.height == other_vs.height
            && self.urine == other_vs.urine
            && self.catheter == other_vs.catheter
            && self.urine_amount == other_vs.urine_amount
            && self.urine_duration == other_vs.urine_duration
            && self.feces == other_vs.feces
            && self.head == other_vs.head
            && self.t_inc == other_vs.t_inc
            && self.line_id == other_vs.line_id
            && self.line_no == other_vs.line_no
            && self.line_mark == other_vs.line_mark
            && self.braden == other_vs.braden
            && self.pain == other_vs.pain
            && self.eye == other_vs.eye
            && self.verbal == other_vs.verbal
            && self.movement == other_vs.movement
            && self.right_pupil == other_vs.right_pupil
            && self.right_cha_id == other_vs.right_cha_id
            && self.left_pupil == other_vs.left_pupil
            && self.left_cha_id == other_vs.left_cha_id
            && self.va_id == other_vs.va_id
            && self.mass_id == other_vs.mass_id
            && self.lt_arm == other_vs.lt_arm
            && self.lt_leg == other_vs.lt_leg
            && self.rt_arm == other_vs.rt_arm
            && self.rt_leg == other_vs.rt_leg
            && self.severity == other_vs.severity
            && self.had_name == other_vs.had_name
            && self.had_drop == other_vs.had_drop
            && self.hct == other_vs.hct
            && self.dtx == other_vs.dtx
            && self.bl == other_vs.bl
            && self.mcb == other_vs.mcb
            && self.suction == other_vs.suction
            && self.nb == other_vs.nb
            && self.o2_id == other_vs.o2_id
            && self.o2_flow == other_vs.o2_flow
            && self.tube_id == other_vs.tube_id
            && self.tube_no == other_vs.tube_no
            && self.tube_mark == other_vs.tube_mark
            && self.ventilator_name == other_vs.ventilator_name
            && self.mode == other_vs.mode
            && self.tv == other_vs.tv
            && self.pip == other_vs.pip
            && self.r_rate == other_vs.r_rate
            && self.i_rate == other_vs.i_rate
            && self.e_rate == other_vs.e_rate
            && self.ti == other_vs.ti
            && self.ps == other_vs.ps
            && self.fio2 == other_vs.fio2
            && self.peep == other_vs.peep
            && self.ft == other_vs.ft
            && self.delta_p == other_vs.delta_p
            && self.o2_map == other_vs.o2_map
            && self.intake_id == other_vs.intake_id
            && self.intake_type == other_vs.intake_type
            && self.intake_amount == other_vs.intake_amount
            && self.intake_absorb == other_vs.intake_absorb
            && self.output_id == other_vs.output_id
            && self.output_amount == other_vs.output_amount
            && self.other == other_vs.other
            && self.lr_int == other_vs.lr_int
            && self.lr_dur == other_vs.lr_dur
            && self.lr_fsh == other_vs.lr_fsh
            && self.lr_sev == other_vs.lr_sev
            && self.lr_cer == other_vs.lr_cer
            && self.lr_eff == other_vs.lr_eff
            && self.lr_sta == other_vs.lr_sta
            && self.lr_mem == other_vs.lr_mem
            && self.lr_af == other_vs.lr_af
            && self.breathing_id == other_vs.breathing_id
            && self.avpu_id == other_vs.avpu_id
            && self.gut_feeling_id == other_vs.gut_feeling_id
            && self.pops_other_id == other_vs.pops_other_id
            && self.wbc == other_vs.wbc
            && self.pleak_flow == other_vs.pleak_flow
            && self.crt == other_vs.crt
            && self.band == other_vs.band
            && self.lr_pos == other_vs.lr_pos
            && self.lr_moulding == other_vs.lr_moulding
            && self.lr_oxytocin_unit == other_vs.lr_oxytocin_unit
            && self.lr_oxytocin_rate == other_vs.lr_oxytocin_rate
            && self.lr_urine_vol == other_vs.lr_urine_vol
            && self.urine_protein == other_vs.urine_protein
            && self.urine_sugar == other_vs.urine_sugar
            && self.diet == other_vs.diet
            && self.barthel_index == other_vs.barthel_index
            && self.aggression_oas == other_vs.aggression_oas
            && self.alcohol_ciwa == other_vs.alcohol_ciwa
            && self.alcohol_aws == other_vs.alcohol_aws
            && self.amphetamine_awq == other_vs.amphetamine_awq
            && self.motivation_scale == other_vs.motivation_scale
            && self.craving_scale == other_vs.craving_scale
            && self.stage_of_change_id == other_vs.stage_of_change_id
            && self.depress_2q == other_vs.depress_2q
            && self.depress_9q == other_vs.depress_2q
            && self.suicide_8q == other_vs.suicide_8q
            && self.create_user == other_vs.create_user
            && self.create_datetime == other_vs.create_datetime
            && self.update_user == other_vs.update_user
            && self.update_datetime == other_vs.update_datetime
            && self.version == other_vs.version
    }
}
