use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::{
    FromRow, MySql, Pool,
    mysql::MySqlQueryResult,
    types::{
        Decimal,
        time::{Date, PrimitiveDateTime, Time},
    },
};
use sqlx_binder::MySqlBinder;
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::EndPoint,
    fetch::{ExecuteResponse, execute_fetch_vec, fetch_json_api},
};

/// Doctor's IPD Admission Note from KPHIS
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, MySqlBinder, ToSchema)]
#[schema(example = json!(IpdDrAdmissionNote::demo()))]
pub struct IpdDrAdmissionNote {
    #[Demo(value = r#"String::from("0001234")"#)]
    pub hn: String,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receiver_medication_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub receiver_medication_time: Option<Time>,

    #[Demo(value = r#"Some(String::from("มาเอง"))"#)]
    pub take_medication_by: Option<String>,
    #[Demo(value = r#"Some(String::from("เดินมา"))"#)]
    pub arrive_by: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub taken_by_relative: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub taken_by_nurse: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub taken_by_crib: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub taken_by_etc: Option<String>,
    #[Demo(value = r#"Some(String::from("EMS"))"#)]
    pub taken_by: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub informant_patient: Option<String>,
    #[Demo(value = r#"Some(String::from("ตา"))"#)]
    pub informant_relatives: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub informant_deliverer: Option<String>,
    #[Demo(value = r#"Some(String::from("เพื่อน"))"#)]
    pub informant_etc: Option<String>,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub chief_complaints: Option<String>,
    #[Demo(value = r#"Some(String::from("Present Hx"))"#)]
    pub medical_history: Option<String>,

    #[Demo(value = r#"Some(String::from("120/80"))"#)]
    pub bp: Option<String>,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub t: Option<Decimal>, // decimal(3,1) unsigned
    #[Demo(value = "Some(80)")]
    pub pr: Option<u32>,
    #[Demo(value = "Some(20)")]
    pub rr: Option<i32>,
    #[Demo(value = "Some(15)")]
    pub gcs: Option<i32>,
    #[Demo(value = r#"Some(String::from("4"))"#)]
    pub e: Option<String>,
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub v: Option<String>,
    #[Demo(value = r#"Some(String::from("6"))"#)]
    pub m: Option<String>,
    /// total,item1,..,item6
    #[Demo(value = r#"Some(String::from("23,4,4,4,4,4,3"))"#)]
    pub braden_scale: Option<String>,

    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub disease: Option<String>,
    #[Demo(value = r#"Some(String::from("HT^Best Hospital"))"#)]
    pub disease_detail: Option<String>, // concat_mutable_vec:  disease_name + ' ' + disease_year + ' ' + disease_hospital
    /// NOT USE
    #[Demo(default)]
    pub disease_etc: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub last_dose_taken_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub last_dose_taken_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("Appendectomy"))"#)]
    pub operation_history: Option<String>,
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub allergy_history: Option<String>,
    #[Demo(value = r#"Some(String::from("Penicillin^Rash"))"#)]
    pub allergy_drug_history: Option<String>, // concat_mutable_vec
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub allergy_drug_history_hosxp: Option<String>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub allergy_drug_pharmacy_check_person: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub allergy_drug_pharmacy_check_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Bread^Rash"))"#)]
    pub allergy_food_history: Option<String>, // concat_mutable_vec
    #[Demo(value = r#"Some(String::from("Flower^Rash"))"#)]
    pub allergy_etc_history: Option<String>, // concat_mutable_vec
    /// NOT USE
    #[Demo(default)]
    pub allergy_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub family_medical_history: Option<String>,
    #[Demo(value = r#"Some(String::from("HT^Father"))"#)]
    pub family_medical_history_detail: Option<String>, // concat_mutable_vec

    #[Demo(value = r#"Some(String::from("MMR"))"#)]
    pub receives_immunisation_history_kid: Option<String>,
    #[Demo(value = r#"Some(String::from("Speech"))"#)]
    pub developmentally_kid: Option<String>,
    #[Demo(value = "Some(1)")]
    pub g: Option<i32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub p: Option<String>,
    #[Demo(value = r#"Some(String::from("Best Hospital"))"#)]
    pub anc: Option<String>,
    #[Demo(value = "Some(3)")]
    pub tt: Option<i32>,
    #[Demo(value = r#"Some(String::from("40"))"#)]
    pub gestational_age: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub gestational_day: Option<String>,

    #[Demo(value = "Some(1)")]
    pub last_child: Option<i32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub last_abort: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub curette: Option<String>,
    #[Demo(value = "Some(date!(2022-11-11))")]
    pub lmp: Option<Date>,
    #[Demo(value = "Some(date!(2023-11-11))")]
    pub edc: Option<Date>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub pb_no: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub giant_baby: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub distocia: Option<String>,
    #[Demo(value = r#"Some(String::from("Vacuum"))"#)]
    pub extraction: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pph: Option<String>,
    #[Demo(value = r#"Some(String::from("PIH"))"#)]
    pub pb_etc: Option<String>,
    /// Height of Fundus
    #[Demo(value = "Some(33)")]
    pub hf: Option<i32>,
    #[Demo(value = r#"Some(String::from("LOA"))"#)]
    pub hf_position: Option<String>,

    #[Demo(value = r#"Some(String::from("RLQ"))"#)]
    pub lr_back_fetus: Option<String>,
    #[Demo(value = r#"Some(String::from("Vertex"))"#)]
    pub lr_presentation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub lr_engagement: Option<String>,
    #[Demo(value = r#"Some(String::from("Right"))"#)]
    pub lr_prominence: Option<String>,
    #[Demo(value = r#"Some(String::from("Flexion"))"#)]
    pub lr_attitude: Option<String>,
    #[Demo(value = "Some(140)")]
    pub lr_fhr: Option<u16>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub lr_fhr_irrigular: Option<String>,
    #[Demo(value = "Some(2500)")]
    pub lr_efw: Option<u16>,
    #[Demo(value = r#"Some(String::from("3'45\""))"#)]
    pub lr_interval: Option<String>,
    #[Demo(value = "Some(40)")]
    pub lr_duration: Option<u8>,
    #[Demo(value = r#"Some(String::from("2+"))"#)]
    pub lr_intensity: Option<String>,
    #[Demo(value = "Some(Decimal::new(115,1))")]
    pub lr_pelvic_diagonal: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(100,1))")]
    pub lr_pelvic_interspinous: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("Diverge"))"#)]
    pub lr_pelvic_sidewall: Option<String>,
    #[Demo(value = r#"Some(String::from("Blunt"))"#)]
    pub lr_ischeal_spine: Option<String>,
    #[Demo(value = r#"Some(String::from("Concave"))"#)]
    pub lr_sacral_curve: Option<String>,
    #[Demo(value = "Some(90)")]
    pub lr_pubic_angle: Option<u8>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub lr_pelvic_ok: Option<String>,
    #[Demo(value = "Some(8)")]
    pub lr_cx_dilate: Option<u8>,
    #[Demo(value = "Some(80)")]
    pub lr_cx_efface: Option<u8>,
    #[Demo(value = "Some(-2)")]
    pub lr_cx_station: Option<i8>,
    #[Demo(value = r#"Some(String::from("Anterior"))"#)]
    pub lr_cx_position: Option<String>,
    #[Demo(value = r#"Some(String::from("Soft"))"#)]
    pub lr_cx_consistency: Option<String>,
    #[Demo(value = "Some(11)")]
    pub lr_cx_bishop: Option<u8>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub lr_cx_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Spontaneous ruptured"))"#)]
    pub lr_membrane: Option<String>,
    #[Demo(value = r#"Some(String::from("Clear"))"#)]
    pub lr_amniotic_color: Option<String>,
    #[Demo(value = r#"Some(String::from("Normal"))"#)]
    pub lr_amniotic_smell: Option<String>,

    #[Demo(value = r#"Some(String::from("P"))"#)]
    pub hiv: Option<String>,
    #[Demo(value = r#"Some(String::from("Reactive"))"#)]
    pub vdrl: Option<String>,
    #[Demo(value = r#"Some(String::from("Positive"))"#)]
    pub hbs_ag: Option<String>,
    #[Demo(value = "Some(Decimal::new(33,0))")]
    pub hct: Option<Decimal>, // decimal(3,1)
    #[Demo(value = r#"Some(String::from("P"))"#)]
    pub hiv2: Option<String>,
    #[Demo(value = r#"Some(String::from("Reactive"))"#)]
    pub vdrl2: Option<String>,
    #[Demo(value = r#"Some(String::from("Positive"))"#)]
    pub hbs_ag2: Option<String>,
    #[Demo(value = "Some(Decimal::new(33,0))")]
    pub hct2: Option<Decimal>, // decimal(3,1)
    /// Blood group
    #[Demo(value = r#"Some(String::from("AB"))"#)]
    pub gr: Option<String>,
    #[Demo(value = r#"Some(String::from("A2A"))"#)]
    pub thalassemia: Option<String>,
    /// Husband Thalassemia result
    #[Demo(value = r#"Some(String::from("A2A"))"#)]
    pub husband: Option<String>,
    #[Demo(value = r#"Some(String::from("Edema"))"#)]
    pub condition_pregnant: Option<String>,

    #[Demo(value = r#"Some(String::from("C/S"))"#)]
    pub deliver_anomalies: Option<String>,
    #[Demo(value = r#"Some(String::from("CPD"))"#)]
    pub deliver_anomalies_means: Option<String>,
    #[Demo(value = r#"Some(String::from("Best Hospital"))"#)]
    pub deliver_location: Option<String>,
    #[Demo(value = "Some(Decimal::new(3200,0))")]
    pub deliver_first_weight: Option<Decimal>, // decimal(5,0)
    #[Demo(value = r#"Some(String::from("Good"))"#)]
    pub deliver_first_health: Option<String>,
    #[Demo(value = "Some(6)")]
    pub fant_breast_feeding_end_age_month: Option<i32>,
    #[Demo(value = "Some(9)")]
    pub fant_artificial_feeding_start_age_month: Option<i32>,
    #[Demo(value = r#"Some(String::from("Banana"))"#)]
    pub fant_feeding_etc: Option<String>,
    #[Demo(value = r#"Some(String::from("ได้รับ"))"#)]
    pub supplementary_feeding: Option<String>,
    #[Demo(value = "Some(6)")]
    pub supplementary_feeding_start_age_month: Option<i32>,

    /// NOT USE
    #[Demo(default)]
    pub disease_operation_allergy: Option<String>,
    #[Demo(value = r#"Some(String::from("เคย"))"#)]
    pub inpatient_history: Option<String>,
    #[Demo(value = r#"Some(String::from("2023-11-11 11:11:11"))"#)]
    pub inpatient_last_date: Option<String>,
    #[Demo(value = r#"Some(String::from("Best Hospital"))"#)]
    pub inpatient_location: Option<String>,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub inpatient_because: Option<String>,

    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_general: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_skin: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_heent: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_neck: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_breastthorax: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_heart: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_lungs: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_abdomen: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_rectalgenitalia: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_extremities: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_neurological: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_ob_gynexam: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Look Good"))"#)]
    pub pe_text: Option<String>,

    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_eent: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_neuro: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_lung: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_tb: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_ht: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_heart: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_liver: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_gi: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_endocrine: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_kidney: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_tumour: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_hemato: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_rheumato: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_psychia: Option<String>,
    #[Demo(value = r#"Some(String::from("ไม่มี"))"#)]
    pub ros_other: Option<String>,

    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub addict: Option<String>,
    /// total,item1,..,item6
    #[Demo(value = r#"Some(String::from("Amphetamine^39,6,6,7,8,6,6|กระท่อม^39,6,6,7,8,6,6"))"#)]
    pub addict_assist: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub addict_inj: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub addict_inj_often: Option<String>,
    /// total,h,a,r,item1,..,item10
    #[Demo(value = r#"Some(String::from("40,12,12,12,4,4,4,4,4,4,4,4,4,4"))"#)]
    pub amphetamine_awq: Option<String>,
    /// total,item1,..,item3
    #[Demo(value = r#"Some(String::from("3,2,3,1"))"#)]
    pub aggression_oas: Option<String>,
    #[Demo(value = "Some(10)")]
    pub motivation_scale: Option<u8>,
    #[Demo(value = "Some(10)")]
    pub craving_scale: Option<u8>,
    #[Demo(value = "Some(6)")]
    pub stage_of_change_id: Option<u8>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("40,4,4,4,4,4,4,4,4,4,4"))"#)]
    pub alcohol_audit: Option<String>,
    /// total,item1,..,item7
    #[Demo(value = r#"Some(String::from("27,4,3,4,4,4,4,4"))"#)]
    pub alcohol_aws: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("67,7,7,7,7,7,7,7,7,7,4"))"#)]
    pub alcohol_ciwa: Option<String>,
    /// total,item1,item2
    #[Demo(value = r#"Some(String::from("2,1,1"))"#)]
    pub depress_2q: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("27,3,3,3,3,3,3,3,3,3"))"#)]
    pub depress_9q: Option<String>,
    /// total,item1,..,item27
    #[Demo(value = r#"Some(String::from("54,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2"))"#)]
    pub depress_cdi: Option<String>,
    /// total,item1,..,item20
    #[Demo(value = r#"Some(String::from("60,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3"))"#)]
    pub depress_cesd: Option<String>,
    /// total,is_dx,is_risk,item1,..,item10
    #[Demo(value = r#"Some(String::from("27,1,1,3,3,3,3,3,3,3,3,3,1,1"))"#)]
    pub depress_phqa: Option<String>,
    /// total,item1,..,item6
    #[Demo(value = r#"Some(String::from("10,3,3,1,1,1,1"))"#)]
    pub nicotin_ftnd: Option<String>,
    /// total,item1,..,item8
    #[Demo(value = r#"Some(String::from("8,1,1,1,1,1,1,1,1"))"#)]
    pub ptsd_screen: Option<String>,
    /// total,item1,..,item10
    #[Demo(value = r#"Some(String::from("30,3,3,3,3,3,3,3,3,3,3"))"#)]
    pub ptsd_pisces: Option<String>,
    /// total,item1,..,item13
    #[Demo(value = r#"Some(String::from("65,5,5,5,5,5,5,5,5,5,5,5,5,5"))"#)]
    pub ptsd_cries: Option<String>,
    /// total,item1,..,item9
    #[Demo(value = r#"Some(String::from("52,1,2,6,8,8,9,4,10,4"))"#)]
    pub suicide_8q: Option<String>,
    /// total,item1,..,item5
    #[Demo(value = r#"Some(String::from("15,3,3,3,3,3"))"#)]
    pub stress_st5: Option<String>,

    #[Demo(value = r#"Some(String::from("<?xml version=\"1.0\"?><!DOCTYPE svg><\\svg>"))"#)]
    pub svg_tag: Option<String>,
    #[Demo(value = r#"Some(String::from("HT"))"#)]
    pub impression: Option<String>,
    #[Demo(value = r#"Some(String::from("DLP"))"#)]
    pub diff_dx: Option<String>,
    #[Demo(value = r#"Some(String::from("Control BP"))"#)]
    pub plan_management: Option<String>,

    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub nurse_name: Option<String>,
    #[Demo(value = r#"Some(String::from("พยาบาล"))"#)]
    pub nurse_pos: Option<String>,
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub nurse_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doc_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub doc_pos: Option<String>,

    #[Demo(value = "Some(3)")]
    pub mem_ruptured_hours: Option<u16>,

    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(String::from("009/99/91NWC10PTESHVK95PNAAA.webp,009/99/91NWC10PTESHVK95PNAAB.webp"))"#)]
    pub imgs: Option<String>,

    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(String::from("Relapse"))"#)]
    pub stage_of_change_name: Option<String>,

    #[Demo(value = "1")]
    pub admission_note_id: u32, // AUTO_INCREMENT
}

/// Doctor's IPD Admission Note to be save
#[derive(Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(IpdAdmissionNoteDrSave::demo()))]
pub struct IpdAdmissionNoteDrSave {
    #[Demo(value = "IpdDrAdmissionNote::demo()")]
    pub admission_note: IpdDrAdmissionNote,
    #[Demo(value = r#"vec![String::from("007")]"#)]
    pub admission_note_doctors: Vec<String>,
}

impl IpdAdmissionNoteDrSave {
    /// - POST `EndPoint::IpdAdmissionNoteDr`
    /// - PUT `EndPoint::IpdAdmissionNoteDr`
    pub async fn call_api_save(&self, method: &str, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IpdAdmissionNoteDrSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IpdAdmissionNoteDrSave"))?;

        execute_fetch_vec(&EndPoint::IpdAdmissionNoteDr.base(), method, Some(&body), app).await
    }
}

/// Data required for create Doctor's IPD Admission Note
#[derive(Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(IpdAdmissionNoteDrRaw::demo()))]
pub struct IpdAdmissionNoteDrRaw {
    #[Demo(value = "Some(IpdDrAdmissionNote::demo())")]
    pub admission_note: Option<IpdDrAdmissionNote>,
    #[Demo(value = "vec![AdmissionNoteDoctor::demo()]")]
    pub admission_note_doctors: Vec<AdmissionNoteDoctor>,
    #[Demo(value = "Some(OpdscreenPe::demo())")]
    pub opdscreen_pe: Option<OpdscreenPe>,
    #[Demo(value = "Some(Vs::demo())")]
    pub vs: Option<Vs>,
    #[Demo(value = "Some(Period::demo())")]
    pub period: Option<Period>,
    #[Demo(value = r#"vec![String::from("Operation")]"#)]
    pub operation_list: Vec<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub old_regdatetime: Option<PrimitiveDateTime>,
    #[Demo(value = "vec![OpdErAllergyHistory::demo()]")]
    pub opd_er_allergy_histories: Vec<OpdErAllergyHistory>,
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub opd_er_allergy_history: Option<String>,
    #[Demo(value = r#"vec![String::from("Dr.Doctor")]"#)]
    pub doctor_in_charge: Vec<String>,
}

impl IpdAdmissionNoteDrRaw {
    /// GET `EndPoint::IpdAdmissionNoteDrAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[&EndPoint::IpdAdmissionNoteDrAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdAdmissionNoteDrRaw"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdAdmissionNoteDrRaw"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Doctor who signed Doctor's IPD Admission Note
#[derive(Clone, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(AdmissionNoteDoctor::demo()))]
pub struct AdmissionNoteDoctor {
    #[Demo(value = "1")]
    pub admission_note_item_id: u32, // AUTO_INCREMENT
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub admission_note_doctor: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub admission_note_doctorname: Option<String>,
    #[Demo(value = r#"Some(String::from("ว00000"))"#)]
    pub licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub entryposition: Option<String>,
}

/// CC, HPI, Physical Examination from HIS
#[derive(Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdscreenPe::demo()))]
pub struct OpdscreenPe {
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub vstdatetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(50.0)")]
    pub bw: Option<f64>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = "Some(7)")]
    pub pain_score: Option<i32>,
    #[Demo(value = "Some(80.0)")]
    pub bpd: Option<f64>,
    #[Demo(value = "Some(120.0)")]
    pub bps: Option<f64>,
    #[Demo(value = "Some(80.0)")]
    pub pulse: Option<f64>,
    #[Demo(value = "Some(20.0)")]
    pub rr: Option<f64>,
    #[Demo(value = "Some(37.5)")]
    pub temperature: Option<f64>,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub cc: Option<String>,
    #[Demo(value = r#"Some(String::from("Present Hx"))"#)]
    pub hpi: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_ga_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_heent_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_heart_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_lung_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_ab_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_ext_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_neuro_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_skin_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_chest_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_gy_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_gu_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_head_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_gi_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_pv_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_pr_text: Option<String>,
    // #[Demo(value = r#"Some(String::from("Look well"))"#)]
    // pub pe_gen_text: Option<String>,
}

#[derive(Clone, Default, FromRow)]
pub struct Ipt {
    pub hn: Option<String>,
    pub vn: Option<String>,
    pub regdatetime: Option<PrimitiveDateTime>,
}

/// Vital Sign data from KPHIS
#[derive(Clone, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(Vs::demo()))]
pub struct Vs {
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub vs_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(120)")]
    pub sbp: Option<u32>,
    #[Demo(value = "Some(80)")]
    pub dbp: Option<u32>,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub bt: Option<Decimal>,
    #[Demo(value = "Some(80)")]
    pub pr: Option<u32>,
    #[Demo(value = "Some(20)")]
    pub rr: Option<u32>,
    #[Demo(value = "Some(4)")]
    pub eye: Option<i32>,
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub verbal: Option<String>,
    #[Demo(value = "Some(6)")]
    pub movement: Option<i32>,
    #[Demo(value = r#"Some(String::from("23,4,4,4,4,4,3"))"#)]
    pub braden: Option<String>,
    #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59|36.0|102|20|110|N|N|1|1|1|92|||1|||5.6|4|5|6||"))"#)]
    pub ews_concat: Option<String>,
}

/// Nurse's Admission Note associated data from KPHIS
#[derive(Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(Period::demo()))]
pub struct Period {
    #[Demo(value = r#"Some(String::from("มี"))"#)]
    pub period: Option<String>,
    #[Demo(value = r#"Some(String::from("LMP"))"#)]
    pub period_normal: Option<String>,
    #[Demo(value = r#"Some(String::from("Dysmenorrhea"))"#)]
    pub period_disorders: Option<String>,
    #[Demo(value = r#"Some(String::from("21/12/66"))"#)]
    pub period_lmp: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub period_menopause: Option<String>,
    #[Demo(value = r#"Some(String::from("Farmer"))"#)]
    pub occupation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub no_risk: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub smoking: Option<String>,
    #[Demo(value = r#"Some(String::from("1 ซอง"))"#)]
    pub smoke_year: Option<String>,
    #[Demo(value = r#"Some(String::from("ทุกวัน"))"#)]
    pub smoke_frequency: Option<String>,
    #[Demo(value = r#"Some(String::from("3 ปี"))"#)]
    pub smoke_stopped: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub alcohol: Option<String>,
    #[Demo(value = r#"Some(String::from("1 กลม"))"#)]
    pub alc_year: Option<String>,
    #[Demo(value = r#"Some(String::from("ทุกวัน"))"#)]
    pub alc_frequency: Option<String>,
    #[Demo(value = r#"Some(String::from("3 ปี"))"#)]
    pub alc_stopped: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub medication_used: Option<String>,
    #[Demo(value = r#"Some(String::from("Cannabis"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("10 เม็ด"))"#)]
    pub med_year: Option<String>,
    #[Demo(value = r#"Some(String::from("ทุกวัน"))"#)]
    pub med_frequency: Option<String>,
    #[Demo(value = r#"Some(String::from("3 ปี"))"#)]
    pub med_stopped: Option<String>,
}

/// Allergy History from KPHIS OPD-ER data
#[derive(Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdErAllergyHistory::demo()))]
pub struct OpdErAllergyHistory {
    #[Demo(value = r#"Some(String::from("PENICILLIN"))"#)]
    pub er_allergy_history_agent: Option<String>,
    #[Demo(value = r#"Some(String::from("Rash"))"#)]
    pub er_allergy_history_symptom: Option<String>,
}
