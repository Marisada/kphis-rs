use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool, mysql::MySqlQueryResult};
use sqlx_binder::MySqlBinder;
use std::rc::Rc;
use time::{
    Date, Time,
    macros::{date, time},
};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::EndPoint,
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

/// Nurse's IPD Admission Note from KPHIS
#[derive(Clone, Demo, Default, Deserialize, Serialize, FromRow, MySqlBinder, ToSchema)]
#[schema(example = json!(IpdNurseAdmissionNote::demo()))]
pub struct IpdNurseAdmissionNote {
    #[Demo(value = r#"String::from("0001234")"#)]
    pub hn: String,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    /// informant's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_patient: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_parent: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_spouse: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_child: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_relatives: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_sender: Option<String>,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub chief_complaints: Option<String>,
    #[Demo(value = r#"Some(String::from("Present Hx"))"#)]
    pub medical_history: Option<String>,
    #[Demo(value = r#"Some(String::from("T: 38.8 °C, P: 88 /min, R: 20 /min, BP: 120/88 mmHg"))"#)]
    pub vs_admit: Option<String>,
    #[Demo(value = r#"Some(String::from("รู้สึกตัวดี"))"#)]
    pub concious: Option<String>,

    /// breathing's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_breath: Option<String>,
    /// breathing's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub kussmaul: Option<String>,
    /// breathing's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tachypnea: Option<String>,
    /// breathing's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dyspnea: Option<String>,
    /// breathing's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub apnea: Option<String>,
    /// breathing's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tube: Option<String>,

    /// cardio's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_hr: Option<String>,
    /// cardio's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub arregular: Option<String>,
    /// cardio's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub weakness: Option<String>,
    /// cardio's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub arrhythmia: Option<String>,
    /// cardio's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub chestpain: Option<String>,
    /// cardio's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pacemaker: Option<String>,
    /// cardio's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub cardio_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Heart problem"))"#)]
    pub cardio_other_text: Option<String>,

    /// circulation's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_cir: Option<String>,
    /// circulation's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pale: Option<String>,
    /// circulation's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub cyanosis: Option<String>,
    /// circulation's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub generalized_edema: Option<String>,
    /// circulation's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub localized_edema: Option<String>,
    #[Demo(value = r#"Some(String::from("Area"))"#)]
    pub localized_edema_text: Option<String>,
    /// circulation's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pitting_edema: Option<String>,
    #[Demo(value = r#"Some(String::from("Grade"))"#)]
    pub pitting_edema_text: Option<String>,
    /// circulation's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub circulation_orther: Option<String>,
    #[Demo(value = r#"Some(String::from("Circulation problem"))"#)]
    pub circulation_orther_text: Option<String>,

    /// skin's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_skin: Option<String>,
    /// skin's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dry: Option<String>,
    /// skin's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub bruise: Option<String>,
    /// skin's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub erythema: Option<String>,
    /// skin's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub abscess: Option<String>,
    /// skin's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub joudice: Option<String>,
    /// skin's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub skin_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Skin problem"))"#)]
    pub skin_other_text: Option<String>,

    /// pain's option
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub pain: Option<String>,
    #[Demo(value = r#"Some(String::from("Location"))"#)]
    pub location: Option<String>,
    /// pain's option
    #[Demo(value = r#"Some(String::from("อื่นๆ"))"#)]
    pub pain_charac: Option<String>,
    #[Demo(value = r#"Some(String::from("Sharp"))"#)]
    pub pain_charac_text: Option<String>,
    #[Demo(value = r#"Some(String::from("7/10"))"#)]
    pub pain_score: Option<String>,

    /// behavior's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_behav: Option<String>,
    /// behavior's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub agitate: Option<String>,
    /// behavior's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub aggressive: Option<String>,
    /// behavior's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub depression: Option<String>,
    /// behavior's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub madness: Option<String>,
    /// behavior's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub behaviour_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Behavior problem"))"#)]
    pub behaviour_other_text: Option<String>,

    /// emotion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_emotional: Option<String>,
    /// emotion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub angry: Option<String>,
    /// emotion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub moody: Option<String>,
    /// emotion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub anxiety: Option<String>,
    /// emotion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub fear: Option<String>,
    /// emotion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub emotional_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Emotion problem"))"#)]
    pub emotional_other_text: Option<String>,

    /// anxiety's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub no_anxiety: Option<String>,
    /// anxiety's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub study: Option<String>,
    /// anxiety's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub family: Option<String>,
    /// anxiety's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub economy: Option<String>,
    /// anxiety's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub habitation: Option<String>,
    /// anxiety's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub illness: Option<String>,

    /// spiritual's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub spiritual_no: Option<String>,
    /// spiritual's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub spiritual_back_home: Option<String>,
    /// spiritual's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub spiritual_need_family: Option<String>,
    /// spiritual's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub spiritual_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Spirit problem"))"#)]
    pub spiritual_other_text: Option<String>,
    /// spiritual's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub spiritual_cant_rated: Option<String>,
    #[Demo(value = r#"Some(String::from("Cannot rate"))"#)]
    pub spiritual_cant_rated_text: Option<String>,

    /// mental main's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub no_mental_state: Option<String>,
    #[Demo(value = r#"Some(String::from("Cannot evaluate"))"#)]
    pub no_mental_state_text: Option<String>,

    /// social's option
    #[Demo(value = r#"Some(String::from("ได้รับ"))"#)]
    pub education: Option<String>,
    #[Demo(value = r#"Some(String::from("ประถมศึกษา"))"#)]
    pub education_result: Option<String>,
    /// social's option
    #[Demo(value = r#"Some(String::from("เกษตรกร"))"#)]
    pub occupation: Option<String>,
    /// social's option
    #[Demo(value = r#"Some(String::from("เพียงพอ"))"#)]
    pub income: Option<String>,
    #[sqlx(rename = "self")]
    #[sqlx_binder(rename = "self")]
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub self_value: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub person_family: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub neighbor: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub assistant_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Assistant"))"#)]
    pub assistant_other_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Assistant job"))"#)]
    pub assistant_occupation: Option<String>,

    /// self care's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub clinic: Option<String>,
    /// self care's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub buy_medicine: Option<String>,

    /// risk's main option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub no_risk: Option<String>,
    /// risk's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub smoking: Option<String>,
    #[Demo(value = r#"Some(String::from("10"))"#)]
    pub smoke_year: Option<String>,
    #[Demo(value = r#"Some(String::from("10 มวน"))"#)]
    pub smoke_frequency: Option<String>,
    #[Demo(value = r#"Some(String::from("5 ปีก่อน"))"#)]
    pub smoke_stopped: Option<String>,
    /// risk's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub alcohol: Option<String>,
    #[Demo(value = r#"Some(String::from("5 ปี"))"#)]
    pub alc_year: Option<String>,
    #[Demo(value = r#"Some(String::from("1 กลม"))"#)]
    pub alc_frequency: Option<String>,
    #[Demo(value = r#"Some(String::from("5 ปีก่อน"))"#)]
    pub alc_stopped: Option<String>,
    /// risk's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub medication_used: Option<String>,
    #[Demo(value = r#"Some(String::from("Drug"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("5 ปี"))"#)]
    pub med_year: Option<String>,
    #[Demo(value = r#"Some(String::from("5 เม็ด"))"#)]
    pub med_frequency: Option<String>,
    #[Demo(value = r#"Some(String::from("5 ปีก่อน"))"#)]
    pub med_stopped: Option<String>,

    /// diet's option
    #[Demo(value = r#"Some(String::from("อาหารทั่วไป"))"#)]
    pub diet_regular: Option<String>,
    /// diet's option
    #[Demo(value = r#"Some(String::from("Special food"))"#)]
    pub diet_spec: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub nutrition_risk: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub loss_appetite: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dysphagia: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub loss_gustation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub denture: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub nutrition_risk_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Nutrition risk"))"#)]
    pub nutrition_risk_other_text: Option<String>,

    /// excretion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_urine: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dysuria: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub incontinence: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub staining: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub hematuria: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub catheter: Option<String>,
    /// excretion's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub normal_feces: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub constipation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub diarrhea: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub bowel_incontinence: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub hemorrhoid: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub colostomy: Option<String>,

    /// activity's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub activity1: Option<String>,
    /// activity's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub activity2: Option<String>,
    /// activity's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub activity3: Option<String>,
    /// activity's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub activity4: Option<String>,
    #[Demo(value = r#"Some(String::from("Cain"))"#)]
    pub o_p_use: Option<String>,

    /// sleep's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub sleep_per_day: Option<String>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub sleep_hour: Option<String>,
    /// sleep's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub sleep_problems: Option<String>,
    #[Demo(value = r#"Some(String::from("Sleep problem"))"#)]
    pub sleep_problems_detail: Option<String>,
    /// sleep's option
    #[Demo(value = r#"Some(String::from("เป็นประจำ"))"#)]
    pub sleep_med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Drug"))"#)]
    pub sleep_med_name_detail: Option<String>,

    /// brain's option
    #[Demo(value = r#"Some(String::from("ไม่ตรง"))"#)]
    pub cognitive: Option<String>,
    /// brain's option
    #[Demo(value = r#"Some(String::from("ผิดปกติ"))"#)]
    pub memory: Option<String>,
    #[Demo(value = r#"Some(String::from("Memory problem"))"#)]
    pub memory_detail: Option<String>,
    /// brain's option
    #[Demo(value = r#"Some(String::from("ผิดปกติ"))"#)]
    pub hearing: Option<String>,
    #[Demo(value = r#"Some(String::from("Hearing problem"))"#)]
    pub hearing_detail: Option<String>,
    /// brain's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub eartone: Option<String>,
    /// brain's option
    #[Demo(value = r#"Some(String::from("ผิดปกติ"))"#)]
    pub vision: Option<String>,
    #[Demo(value = r#"Some(String::from("Vision problem"))"#)]
    pub vision_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub vision_eyeglasses: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub vision_contactlens: Option<String>,
    /// brain's option
    #[Demo(value = r#"Some(String::from("ผิดปกติ"))"#)]
    pub speech: Option<String>,
    #[Demo(value = r#"Some(String::from("Speech problem"))"#)]
    pub speech_detail: Option<String>,

    /// self's option
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub self_image: Option<String>,
    #[Demo(value = r#"Some(String::from("Self image"))"#)]
    pub self_image_detail: Option<String>,
    /// self's option
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub self_activity: Option<String>,
    #[Demo(value = r#"Some(String::from("Activity"))"#)]
    pub self_activity_detail: Option<String>,

    /// sick burden's main option
    #[Demo(value = r#"Some(String::from("มีผลกระทบต่อ"))"#)]
    pub sickness_effect: Option<String>,
    /// sick burden's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub sickness_family: Option<String>,
    /// sick burden's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub sickness_occupation: Option<String>,
    /// sick burden's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub sickness_education: Option<String>,
    /// sick burden's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub sickness_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Other"))"#)]
    pub sickness_other_text: Option<String>,

    /// period's main option
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub period: Option<String>,
    /// period's option
    #[Demo(value = r#"Some(String::from("ผิดปกติ"))"#)]
    pub period_normal: Option<String>,
    /// period's option
    #[Demo(value = r#"Some(String::from("Period problem"))"#)]
    pub period_disorders: Option<String>,
    #[Demo(value = r#"Some(String::from("last month, 3 days"))"#)]
    pub period_lmp: Option<String>,
    /// period's option
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub period_menopause: Option<String>,

    /// breast's option
    #[Demo(value = r#"Some(String::from("ผิดปกติ"))"#)]
    pub breast: Option<String>,
    #[Demo(value = r#"Some(String::from("Breast problem"))"#)]
    pub breast_disorders: Option<String>,

    /// stress's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub consult: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub seclude: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub medication: Option<String>,
    #[Demo(value = r#"Some(String::from("Medication"))"#)]
    pub medication_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub religion: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub coping_stress_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Stress management"))"#)]
    pub coping_stress_other_detail: Option<String>,

    /// belief's option
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub belief_sickness_behave: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub belief_sickness_age: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub belief_sickness_destiny: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub belief_sickness_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Belief"))"#)]
    pub belief_sickness_other_text: Option<String>,
    /// belief's option
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub belief_believe: Option<String>,
    #[Demo(value = r#"Some(String::from("God"))"#)]
    pub belief_believe_text: Option<String>,
    /// belief's option
    #[Demo(value = r#"Some(String::from("ต้องการ"))"#)]
    pub religious_activity: Option<String>,
    #[Demo(value = r#"Some(String::from("Religion"))"#)]
    pub religious_activity_text: Option<String>,

    #[Demo(value = "1")]
    pub nurse_admission_note_id: u32, //  AUTO_INCREMENT,

    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub nurse_name: Option<String>,
    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(String::from("พยาบาล"))"#)]
    pub nurse_pos: Option<String>,
    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub nurse_licenseno: Option<String>,

    #[sqlx_binder(skip)]
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receiver_medication_date: Option<Date>,
    #[sqlx_binder(skip)]
    #[Demo(value = "Some(time!(23:59:59))")]
    pub receiver_medication_time: Option<Time>,
}

impl IpdNurseAdmissionNote {
    /// GET `EndPoint::IpdAdmissionNoteNurseAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdAdmissionNoteNurseAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdNurseAdmissionNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdNurseAdmissionNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// - POST `EndPoint::IpdAdmissionNoteNurse`
    /// - PUT `EndPoint::IpdAdmissionNoteNurse`
    pub async fn call_api_save(&self, method: &str, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IpdNurseAdmissionNote"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IpdNurseAdmissionNote"))?;

        execute_fetch(&EndPoint::IpdAdmissionNoteNurse.base(), method, Some(&body), app).await
    }
}
