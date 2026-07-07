use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use time::{
    Date, PrimitiveDateTime, Time,
    macros::{date, datetime, time},
};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::EndPoint,
    fetch::{ExecuteResponse, execute_fetch_vec, fetch_json_api},
};

/// HIS refer-out data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HisReferOutData::demo()))]
pub struct HisReferOutData {
    #[Demo(value = "HisReferOut::demo()")]
    pub referout: HisReferOut,
    /// always has one item, but DB structure allow many items
    #[Demo(value = r#"vec![HisReferVitalSign::demo()]"#)]
    pub vital_signs: Vec<HisReferVitalSign>,
}

impl HisReferOutData {
    /// GET `EndPoint::HisReferOutVnan`
    pub async fn call_api_get(vnan: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::HisReferOutVnan.base(), vnan].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisReferOut"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisReferOut"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// HIS referout table
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HisReferOut::demo()))]
pub struct HisReferOut {
    // PK, not auto number, use serial's `referout_id`
    #[Demo(value = "1")]
    pub referout_id: i32,
    // Index, VN or AN
    #[Demo(value = r#"String::from("660001234")"#)]
    pub vn: String,
    // Index, VARCHAR(9)
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    // VARCHAR(10)
    #[Demo(value = r#"Some(String::from("63/0001"))"#)]
    pub refer_number: Option<String>,
    // Index, VARCHAR(5)
    #[Demo(value = r#"Some(String::from("11707"))"#)]
    pub refer_hospcode: Option<String>,
    // Index, วันที่ส่ง
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub refer_date: Option<Date>,
    // เวลาส่งตัว
    #[Demo(value = "Some(time!(23:59:59))")]
    pub refer_time: Option<Time>,
    // วันสิ้นสุด
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub due_date: Option<Date>,
    // วันหมดอายุ
    #[Demo(value = "Some(date!(2024-03-31))")]
    pub expire_date: Option<Date>,
    // VARCHAR(200), การวินิจฉัยขั้นต้น => moph-refer's initial_diagnosis_free_text
    #[Demo(value = r#"Some(String::from("Pneumonia"))"#)]
    pub pre_diagnosis: Option<String>,
    // TEXT, ประวัติการเจ็บป่วยในอดีต => moph-refer's record_illness_past
    #[Demo(value = r#"Some(String::from("OLD DM"))"#)]
    pub pmh: Option<String>,
    // TEXT, ประวัติการเจ็บป่วยในปัจจุบัน => moph-refer's record_illness_present
    #[Demo(value = r#"Some(String::from("1D PTA BLAH BLAH"))"#)]
    pub hpi: Option<String>,
    // TEXT, ผลการตรวจทางห้องปฏิบัติการ => moph-refer's test_results
    #[Demo(value = r#"Some(String::from("HCT 33%"))"#)]
    pub lab_text: Option<String>,
    // TEXT, การรักษาที่ได้ให้ไว้ ยา/หัตถการ => moph-refer's treatment_provided
    #[Demo(value = r#"Some(String::from("IV CEFTAZIDIME"))"#)]
    pub treatment_text: Option<String>,
    // TEXT => moph-refer's more_detail
    #[Demo(value = r#"Some(String::from("Something bad"))"#)]
    pub other_text: Option<String>,
    // // VARCHAR(7)
    // #[Demo(value = r#"Some(String::from("R100"))"#)]
    // pub pdx: Option<String>,
    // TEXT, การวินิจฉัย, NOT USE IN MOPH-REFER
    #[Demo(value = r#"Some(String::from("Acute Abdomen"))"#)]
    pub diagnosis_text: Option<String>,
    // TEXT, ส่งตัวเพื่อ, NOT USE IN MOPH-REFER
    #[Demo(value = r#"Some(String::from("For proper management"))"#)]
    pub request_text: Option<String>,

    // CHAR(3), OPD/IPD
    #[Demo(value = r#"Some(String::from("IPD"))"#)]
    pub department: Option<String>,
    // TINYINT(4), referout_type.referout_type_id
    #[Demo(value = "Some(1)")]
    pub refer_type: Option<i8>,
    // CHAR(2)
    #[Demo(value = r#"Some(String::from("89"))"#)]
    pub pttype: Option<String>,
    // CHAR(2)
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub spclty: Option<String>,
    // TINYINT(4), refer_cause.id
    #[Demo(value = "Some(1)")]
    pub refer_cause: Option<i8>,
    // VARCHAR(10), source point (department name)
    #[Demo(value = r#"Some(String::from("LR"))"#)]
    pub refer_point: Option<String>,
    // INT(11), การหมดอายุ, moph_refer_expire_type.moph_refer_expire_type_id
    #[Demo(value = "Some(1)")]
    pub moph_refer_expire_type_id: Option<i32>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub doctor: Option<String>,
    // CHAR(1)
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub issued_moph_refer: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub update_datetime: Option<PrimitiveDateTime>,

    // JOINED
    #[Demo(value = r#"Some(String::from("รพ.มหาสารคาม"))"#)]
    pub refer_hospcode_name: Option<String>,
    // JOINED
    #[Demo(value = r#"Some(String::from("รับไว้รักษาต่อ"))"#)]
    pub refer_cause_name: Option<String>,
    // JOINED
    #[Demo(value = r#"Some(String::from("ระบุวันหมดอายุของใบส่งตัว"))"#)]
    pub moph_refer_expire_type_name: Option<String>,
    // JOINED
    #[Demo(value = r#"Some(String::from("Dr Doctor"))"#)]
    pub doctor_name: Option<String>,
    // // #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    // // pub refer_begin_time: Option<PrimitiveDateTime>,
    // // #[Demo(value = "Some(datetime!(2024-01-01 01:11:59))")]
    // // pub refer_end_time: Option<PrimitiveDateTime>,
    // // #[Demo(value = "Some(1)")]
    // // pub refer_process_time_hour: Option<i32>,
    // // #[Demo(value = "Some(10)")]
    // // pub refer_process_time_minute: Option<i32>,

    // // CHAR(1), สาเหตุ, kp_rfrcs.rfrcs
    // #[Demo(value = r#"Some(String::from("4"))"#)]
    // pub rfrcs: Option<String>,
    // // INT(11), ประเภทการส่งตัว, referout_type.referout_type_id
    // #[Demo(value = "Some(1)")]
    // pub referout_type_id: Option<i32>,
    // // INT(11), ความเร่งด่วน, referout_emergency_type.referout_emergency_type_id
    // #[Demo(value = "Some(3)")]
    // pub referout_emergency_type_id: Option<i32>,
    // // INT(11), กลุ่มโรคเฉพาะ, referout_sp_type.referout_sp_type_id
    // #[Demo(value = "Some(3)")]
    // pub referout_sp_type_id: Option<i32>,
    // // INT(11), refer_acuity_type.refer_acuity_type_id
    // #[Demo(value = "Some(3)")]
    // pub refer_acuity_type_id: Option<i32>,
    // // // INT(11), refer_response_type.refer_response_type_id
    // // #[Demo(value = "Some(1)")]
    // // pub refer_responst_type_id: Option<i32>,

    // // CHAR(3), source department code
    // #[Demo(value = r#"Some(String::from("011"))"#)]
    // pub depcode: Option<String>,
    // // CHAR(1)
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // pub with_doctor: Option<String>,
    // // CHAR(1)
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // pub with_nurse: Option<String>,
    // // CHAR(1)
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // pub with_ambulance: Option<String>,
    // // CHAR(1)
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // pub refer_in_province: Option<String>,
    // // CHAR(1)
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // pub refer_in_region: Option<String>,
    // // VARCHAR(15)
    // #[Demo(value = r#"Some(String::from("MK 0001"))"#)]
    // pub car_registration_no : Option<String>,
    // // TEXT
    // #[Demo(value = r#"Some(String::from("BP STABLE"))"#)]
    // pub ptstatus_text: Option<String>,

    // // Index, VARCHAR(5)
    // #[Demo(value = r#"Some(String::from("11059"))"#)]
    // pub hospcode: Option<String>,
    // // CHAR(3)
    // #[Demo(value = r#"Some(String::from(""))"#)]
    // pub clinic: Option<String>,
    // #[Demo(value = "Some(date!(2024-01-01))")]
    // pub vst_date: Option<Date>,

    // // VARCHAR(10)
    // #[Demo(value = r#"Some(String::from("ER"))"#)]
    // pub accept_point : Option<String>,
    // // VARCHAR(25)
    // #[Demo(value = r#"Some(String::from("Mrs Nurse"))"#)]
    // pub refer_write_staff: Option<String>,
    // // Index, CHAR(1)
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // pub update_moph_phr: Option<String>,
    // // CHAR(1)
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // pub issue_digital_certificate: Option<String>,
    // #[Demo(value = "Some(1)")]
    // pub doctor_cert_id: Option<i32>,

    // refercheck1 - refercheck4: CHAR(2)
    // refercheck2_data: DATE
    // refercheck3_text: VARCHAR(150)
    // refercheck4_text: VARCHAR(150)
    // docno: VARCHAR(25)
    // confirm_date: DATE
    // confirm_text: TEXT
    // confirm_diagnosis: VARCHAR(7)
    // dest_hospital_department: VARCHAR(250)
    // referout_status_id: INT(11) -> referout_status.referout_status_id
    // referout_transfer_hospcode: CHAR(5)
    // hos_guid: Index, VARCHAR(38)
    // senttonet: VARCHAR(5)
    // rfo_sent: VARCHAR(30)
    // dest_hospcode_instruction_text:
    // i_refer_number: VARCHAR(25)
    // cloud_refer_number: Index, VARCHAR(25)
    // service_request_sct_code: VARCHAR(15)
    // service_request_sct_name: VARCHAR(100)
}

/// HIS refer_vital_sign table
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HisReferVitalSign::demo()))]
pub struct HisReferVitalSign {
    // PK, not auto number, use serial's `refer_vital_sign_id`
    #[Demo(value = "1")]
    pub refer_vital_sign_id: i32,
    // Index, INT(11)
    #[Demo(value = "Some(123)")]
    pub referout_id: Option<i32>,
    // TEXT, chief complain, NOT USE IN MOPH-REFER
    #[Demo(value = r#"Some(String::from("PAIN"))"#)]
    pub cc: Option<String>,
    // TEXT, PE => moph-refer's pe
    #[Demo(value = r#"Some(String::from("MASS AT HEAD"))"#)]
    pub pe: Option<String>,
    // INT(11)
    #[Demo(value = "Some(120)")]
    pub bps: Option<i32>,
    // INT(11)
    #[Demo(value = "Some(80)")]
    pub bpd: Option<i32>,
    // DOUBLE(15,3)
    #[Demo(value = "Some(50.0)")]
    pub body_weight_kg: Option<f64>,
    // DOUBLE(15,3)
    #[Demo(value = "Some(155.0)")]
    pub height_cm: Option<f64>,
    // DOUBLE(15,3)
    #[Demo(value = "Some(37.5)")]
    pub temperature: Option<f64>,
    // INT(11)
    #[Demo(value = "Some(20)")]
    pub rr: Option<i32>,
    // INT(11)
    #[Demo(value = "Some(80)")]
    pub pulse: Option<i32>,
    // DOUBLE(15,3)
    #[Demo(value = "Some(110.0)")]
    pub fbs: Option<f64>,
    // VARCHAR(100), NOT USE IN MOPH-REFER
    #[Demo(value = r#"Some(String::from("PAIN"))"#)]
    pub pre_diagnosis: Option<String>,
    // // Index, INT(11)
    // #[Demo(value = "Some(1)")]
    // pub refer_queue_id: Option<i32>,

    // hospcode: VARCHAR(5)
    // death_by_transfer: CHAR(1)
    // hr: INT(11)
    // consciousness: CHAR(1)
    // consciousness_text: VARCHAR(200)
    // coma_score_e: INT(11)
    // coma_score_v: INT(11)
    // coma_score_m: INT(11)
    // coma_score_e_text: VARCHAR(50)
    // coma_score_v_text: VARCHAR(50)
    // coma_score_m_text: VARCHAR(50)
    // pupil_size: DOUBLE(15,3)
    // pupil_size_r: DOUBLE(15,3)
    // pupil_size_text: VARCHAR(10)
    // pupil_size_r_text: VARCHAR(10)
    // pe_ga: CHAR(1)
    // pe_heent: CHAR(1)
    // pe_heart: CHAR(1)
    // pe_lung: CHAR(1)
    // pe_neuro: CHAR(1)
    // pe_ga_text: VARCHAR(150)
    // pe_heent_text: VARCHAR(150)
    // pe_heart_text: VARCHAR(150)
    // pe_lung_text: VARCHAR(150)
    // pe_neuro_text: VARCHAR(150)
}

/// HIS referout + refer_vital_sign to be save
#[derive(Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(HisReferOutSave::demo()))]
pub struct HisReferOutSave {
    #[Demo(value = "Some(1)")]
    pub referout_id: Option<i32>,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub vn: String,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("11707"))"#)]
    pub refer_hospcode: Option<String>,
    #[Demo(value = "Some(date!(2024-03-31))")]
    pub due_date: Option<Date>,
    #[Demo(value = "Some(date!(2024-03-31))")]
    pub refer_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub refer_time: Option<Time>,
    #[Demo(value = "Some(date!(2024-03-31))")]
    pub expire_date: Option<Date>,
    #[Demo(value = r#"Some(String::from("Pneumonia"))"#)]
    pub pre_diagnosis: Option<String>,
    #[Demo(value = r#"Some(String::from("OLD DM"))"#)]
    pub pmh: Option<String>,
    #[Demo(value = r#"Some(String::from("1D PTA BLAH BLAH"))"#)]
    pub hpi: Option<String>,
    #[Demo(value = r#"Some(String::from("HCT 33%"))"#)]
    pub lab_text: Option<String>,
    #[Demo(value = r#"Some(String::from("IV CEFTAZIDIME"))"#)]
    pub treatment_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Something bad"))"#)]
    pub other_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Acute Abdomen"))"#)]
    pub diagnosis_text: Option<String>,
    #[Demo(value = r#"Some(String::from("For proper management"))"#)]
    pub request_text: Option<String>,

    #[Demo(value = r#"Some(String::from("IPD"))"#)]
    pub department: Option<String>,
    #[Demo(value = "Some(1)")]
    pub refer_type: Option<i8>,
    #[Demo(value = r#"Some(String::from("89"))"#)]
    pub pttype: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub spclty: Option<String>,
    #[Demo(value = "Some(1)")]
    pub refer_cause: Option<i8>,
    #[Demo(value = r#"Some(String::from("LR"))"#)]
    pub refer_point: Option<String>,
    #[Demo(value = "Some(1)")]
    pub moph_refer_expire_type_id: Option<i32>,

    #[Demo(value = "Some(1)")]
    pub refer_vital_sign_id: Option<i32>,
    #[Demo(value = r#"Some(String::from("PAIN"))"#)]
    pub cc: Option<String>,
    #[Demo(value = r#"Some(String::from("MASS AT HEAD"))"#)]
    pub pe: Option<String>,
}

impl HisReferOutSave {
    /// - POST `EndPoint::HisReferOutVnan`
    pub async fn call_api_post(&self, vnan: &str, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send HisReferOutSave"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send HisReferOutSave"))?;

        execute_fetch_vec(&[&EndPoint::HisReferOutVnan.base(), vnan].concat(), "POST", Some(&body), app).await
    }
}
