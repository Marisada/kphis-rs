// ipd-dr-admission-note-form.php

use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use rust_decimal::Decimal;
use std::{
    rc::Rc,
    sync::{
        Mutex,
        atomic::{AtomicU32, Ordering},
    },
};
use time::PrimitiveDateTime;
use wasm_bindgen::{JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    SCREEN_WIDTH_EXTRA,
    endpoint::EndPoint,
    fetch::{Method, get_text_from_url},
    image::file_path::ImageUsage,
    ipd::admission_note_dr::{AdmissionNoteDoctor, IpdAdmissionNoteDrRaw, IpdAdmissionNoteDrSave, IpdDrAdmissionNote, OpdErAllergyHistory},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    route::Route,
    tab::Tab,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_component::{
    gadget::{
        aside_resizer::AsideResizerCpn,
        image::{ImageCpn, ImagePaths},
        pdf_button::PdfButtons,
    },
    modal::{
        blank_modal,
        scoring::{
            addict_assist::AddictAssistV2, aggression_oas::AggressionOAS, alcohol_audit::AlcoholAudit, alcohol_aws::AlcoholAws, alcohol_ciwa_ar::AlcoholCiwaAr, amphetamine_awq::AmphetamineAwqV2,
            braden::Braden, depress_2q::Depress2Q, depress_9q::Depress9Q, depress_cdi::DepressCdi, depress_cesd::DepressCesD, depress_phqa::DepressPhqA, nicotin_ftnd::NicotinFtnd,
            ptsd_cries13::PtsdCries13, ptsd_pisces10::PtsdPisces10, ptsd_screen::PtsdScreen, stress_st5::StressST5, suicide_8q::Suicide8Q,
        },
    },
    show_patient_main::ShowPatientMainCpn,
};
use kphis_ui_core::{
    binding::{Canvas, FabricOption, PencilBrush, group_svg_elements, load_svg_from_string},
    class, doms, mixins,
};
use kphis_util::{
    datetime::{JsTime, date_8601, datetime_8601, js_now, time_8601},
    util::{Concat, concat_mutable_vec, decimal_rescale, explode, lr_int_from_quote, lr_int_to_quote, opt_empty_none, opt_zero_none, str_some, svg_to_data_url, zero_none, zero_str_none},
};

const ALL_BODY_SVG_URL: &str = "/statics/picture/allbody.svg";
const PE_ADULT_GENERAL: &str = "Good consciousness, not pale, no jaundice";
const PE_ADULT_SKIN: &str = "No rash";
const PE_ADULT_HEENT: &str = "Not pale conjunctiva, no icteric sclera";
const PE_ADULT_NECK: &str = "No mass, no LN enlargement";
const PE_ADULT_THORAX: &str = "No mass";
const PE_ADULT_HEART: &str = "Normal S1 S2, no murmur";
const PE_ADULT_LUNG: &str = "Equal breath sounds, no adventitious sound";
const PE_ADULT_ABDOMEN: &str = "Soft, not tender, no mass";
const PE_ADULT_GENITALIA: &str = "Normal genital appearance";
const PE_ADULT_EXT: &str = "No limit ROM";
const PE_ADULT_NEURO: &str = "E4V5M6, motor power grade V all extremities";
const PE_ADULT_OBGYN: &str = "No mass, no discharge";
const PE_PED_GENERAL: &str = "Active crying";
const PE_PED_SKIN: &str = "Pink, no rash";
const PE_PED_HEENT: &str = "AF 2x2 cm, no cephalhematoma";
const PE_PED_NECK: &str = "No webbed neck";
const PE_PED_THORAX: &str = "Normal chest contour";
const PE_PED_HEART: &str = "No murmur";
const PE_PED_LUNG: &str = "No adventitious sound, no subcostal retraction";
const PE_PED_ABDOMEN: &str = "No hepatosplenomegaly";
const PE_PED_GENITALIA: &str = "Patent anus, no ambiguous genitalia";
const PE_PED_EXT: &str = "No deformity";
const PE_PED_NEURO: &str = "Moro reflex positive";
const SOMETHING: &str = "มี";
const NOTHING: &str = "ไม่มี";

static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// - GET `EndPoint::IpdAdmissionNoteDrAn`
/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - POST/PUT `EndPoint::IpdAdmissionNoteDr` (guarded, remove 'บันทึก' btn)
/// - POST `EndPoint::ImageUsage` (guarded, no image gadget)
#[derive(Clone, Default)]
pub struct IpdAdmissionNoteDrPage {
    // page mechanic
    loaded: Mutable<bool>,
    loaded_all_body_image: Mutable<bool>,
    changed: Mutable<bool>,
    display_puberty: Mutable<bool>,
    display_psychiatry: Mutable<bool>,
    // we mutate inner of IpdAdmissionNoteDrRaw (behind Rc) after update
    raw: Mutable<Rc<Mutex<IpdAdmissionNoteDrRaw>>>,
    is_set_this_hospital: Mutable<bool>,

    // image component callback state
    image_callback: Mutable<ImagePaths>,

    // temporary canvas state
    fabric_loaded: Mutable<bool>,
    canvas: Mutable<Option<Rc<Canvas>>>,
    redoing: Mutable<bool>,
    h: MutableVec<Rc<JsValue>>,
    pen_color: Mutable<String>,
    pen_width: Mutable<u32>,

    // page assets
    all_body_image: Mutable<Rc<String>>,
    old_regdatetime: Mutable<Option<String>>,

    // patient main
    patient: Mutable<Rc<ShowPatientMainCpn>>,

    // database data
    admission_note_id: Mutable<Option<u32>>, // row

    hn: Mutable<Option<String>>,
    an: Mutable<String>,
    receiver_medication_date: Mutable<String>, // Date?
    receiver_medication_time: Mutable<String>, // Tate?

    take_medication_by: Mutable<TakeMedicationBy>,
    arrive_by: Mutable<ArriveBy>,
    taken_by_relative: Mutable<String>, // "Y"
    taken_by_nurse: Mutable<String>,    // "Y"
    taken_by_crib: Mutable<String>,     // "Y"
    taken_by_etc: Mutable<String>,      // "Y"
    taken_by: Mutable<String>,
    informant_patient: Mutable<Option<String>>, // ไม่รู้สึกตัว, ซักประวัติไม่ได้
    informant_relatives: Mutable<Option<String>>,
    informant_deliverer: Mutable<Option<String>>,
    informant_etc: Mutable<Option<String>>,
    chief_complaints: Mutable<String>,
    medical_history: Mutable<String>,

    chief_complaints_opdscreen: Mutable<String>,
    medical_history_opdscreen: Mutable<String>,
    t_opdscreen: Mutable<String>,
    pr_opdscreen: Mutable<String>,
    rr_opdscreen: Mutable<String>,
    bp_opdscreen: Mutable<String>,

    t: Mutable<String>,
    pr: Mutable<String>,
    rr: Mutable<String>,
    bp: Mutable<String>,
    gcs: Mutable<String>,
    e: Mutable<String>,
    v: Mutable<String>,
    m: Mutable<String>,
    braden_scale: Mutable<String>,

    disease: Mutable<String>,                       // "มี","ไม่มี"
    disease_details: MutableVec<Rc<DiseaseDetail>>, // disease_detail = "disease_name disease_year disease_hospital ..."
    operation_history: Mutable<Option<String>>,
    disease_etc: Mutable<Option<String>>, // unused
    last_dose_taken_time: Mutable<String>,
    last_dose_taken_remark: Mutable<Option<String>>,
    allergy_history: Mutable<String>,
    allergy_drugs: MutableVec<Rc<DrugAllergy>>, // => allergy_drug_history = "agent symptom agent symptom ..."
    allergy_drug_history_hosxp: Mutable<String>,
    allergy_drug_pharmacy_check_person: Mutable<Option<String>>,              // unused
    allergy_drug_pharmacy_check_datetime: Mutable<Option<PrimitiveDateTime>>, // unused
    allergy_foods: MutableVec<Rc<FoodAllergy>>,                               // => allergy_food_history = "agent symptom agent symptom ..."
    allergy_etcs: MutableVec<Rc<EtcAllergy>>,                                 // => allergy_etc_history = "agent symptom agent symptom ..."
    allergy_detail: Mutable<Option<String>>,                                  // unused
    family_medical_history: Mutable<String>,
    family_medicals: MutableVec<Rc<FamilyMedical>>, // => family_medical_history_detail

    receives_immunisation_history_kid: Mutable<Option<String>>,
    developmentally_kid: Mutable<Option<String>>,
    g: Mutable<String>, // i32
    p: Mutable<String>,
    anc: Mutable<String>,
    tt: Mutable<String>, // i32
    gestational_age: Mutable<String>,
    gestational_day: Mutable<String>,
    last_child: Mutable<String>, // i32
    last_abort: Mutable<String>,
    curette: Mutable<String>,
    lmp: Mutable<String>,
    edc: Mutable<String>,
    pb_no: Mutable<String>,
    giant_baby: Mutable<String>,
    distocia: Mutable<String>,
    extraction: Mutable<Option<String>>,
    pph: Mutable<String>,
    pb_etc: Mutable<Option<String>>,
    hf: Mutable<String>, // i32
    hf_position: Mutable<String>,
    mem_ruptured_hours: Mutable<Option<String>>,

    lr_back_fetus: Mutable<String>,
    lr_presentation: Mutable<String>,
    lr_engagement: Mutable<String>,
    lr_prominence: Mutable<String>,
    lr_attitude: Mutable<String>,
    lr_fhr: Mutable<String>, // u16
    lr_fhr_irrigular: Mutable<String>,
    lr_efw: Mutable<String>, // u16
    lr_interval_m: Mutable<String>,
    lr_interval_s: Mutable<String>,
    lr_duration: Mutable<String>, // u8
    lr_intensity: Mutable<String>,
    lr_pelvic_diagonal: Mutable<String>,     // Decimal
    lr_pelvic_interspinous: Mutable<String>, // Decimal
    lr_pelvic_sidewall: Mutable<String>,
    lr_ischeal_spine: Mutable<String>,
    lr_sacral_curve: Mutable<String>,
    lr_pubic_angle: Mutable<String>, // u8
    lr_pelvic_ok: Mutable<String>,
    lr_cx_dilate: Mutable<String>, // u8
    lr_cx_efface: Mutable<String>, // u8
    lr_cx_station: Mutable<String>,
    lr_cx_position: Mutable<String>,
    lr_cx_consistency: Mutable<String>,
    lr_cx_bishop: Mutable<String>, // u8
    lr_cx_ok: Mutable<String>,
    lr_membrane: Mutable<String>,
    lr_amniotic_color: Mutable<String>,
    lr_amniotic_smell: Mutable<String>,

    hiv: Mutable<String>,
    vdrl: Mutable<String>,
    hbs_ag: Mutable<String>,
    hct: Mutable<String>,
    hiv2: Mutable<String>,
    vdrl2: Mutable<String>,
    hbs_ag2: Mutable<String>,
    hct2: Mutable<String>,
    gr: Mutable<String>,
    thalassemia: Mutable<String>,
    husband: Mutable<String>,
    condition_pregnant: Mutable<Option<String>>,
    deliver_anomalies: Mutable<Option<String>>,
    deliver_anomalies_means: Mutable<String>,
    deliver_location: Mutable<String>,
    deliver_first_weight: Mutable<String>,
    deliver_first_health: Mutable<String>,
    fant_breast_feeding_end_age_month: Mutable<Option<String>>,       // i32
    fant_artificial_feeding_start_age_month: Mutable<Option<String>>, // i32
    fant_feeding_etc: Mutable<Option<String>>,
    supplementary_feeding: Mutable<String>,
    supplementary_feeding_start_age_month: Mutable<String>, // i32
    disease_operation_allergy: Mutable<Option<String>>,
    inpatient_history: Mutable<String>,
    inpatient_last_date: Mutable<String>,
    inpatient_location: Mutable<String>,
    inpatient_because: Mutable<String>,

    pe_bw: Mutable<String>,
    pe_height: Mutable<String>,

    pe_general: Mutable<String>,
    pe_skin: Mutable<String>,
    pe_heent: Mutable<String>,
    pe_neck: Mutable<String>,
    pe_breastthorax: Mutable<String>,
    pe_heart: Mutable<String>,
    pe_lungs: Mutable<String>,
    pe_abdomen: Mutable<String>,
    pe_rectalgenitalia: Mutable<String>,
    pe_extremities: Mutable<String>,
    pe_neurological: Mutable<String>,
    pe_ob_gynexam: Mutable<String>,
    pe_other: Mutable<String>,
    pe_text: Mutable<String>,

    ros_eent: Mutable<String>,
    ros_neuro: Mutable<String>,
    ros_lung: Mutable<String>,
    ros_tb: Mutable<String>,
    ros_ht: Mutable<String>,
    ros_heart: Mutable<String>,
    ros_liver: Mutable<String>,
    ros_gi: Mutable<String>,
    ros_endocrine: Mutable<String>,
    ros_kidney: Mutable<String>,
    ros_tumour: Mutable<String>,
    ros_hemato: Mutable<String>,
    ros_rheumato: Mutable<String>,
    ros_psychia: Mutable<String>,
    ros_other: Mutable<String>,

    addict: Mutable<String>,
    addict_assists: MutableVec<Rc<AddictAssist>>, // => addict_assist = "agent assist_score agent assist_score ..."
    addict_inj: Mutable<String>,                  // Y, N
    addict_inj_often: Mutable<String>,            // Y, N
    amphetamine_awq: Mutable<String>,
    aggression_oas: Mutable<String>,
    motivation_scale: Mutable<String>,
    craving_scale: Mutable<String>,
    stage_of_change_id: Mutable<String>,
    alcohol_audit: Mutable<String>,
    alcohol_aws: Mutable<String>,
    alcohol_ciwa: Mutable<String>,
    depress_2q: Mutable<String>,
    depress_9q: Mutable<String>,
    depress_cdi: Mutable<String>,
    depress_cesd: Mutable<String>,
    depress_phqa: Mutable<String>,
    nicotin_ftnd: Mutable<String>,
    ptsd_screen: Mutable<String>,
    ptsd_pisces: Mutable<String>,
    ptsd_cries: Mutable<String>,
    suicide_8q: Mutable<String>,
    stress_st5: Mutable<String>,

    svg_tag: Mutable<String>,
    impression: Mutable<String>,
    diff_dx: Mutable<String>,
    plan_management: Mutable<String>,

    nurse_name: Mutable<String>,
    nurse_pos: Mutable<String>,
    nurse_licenseno: Mutable<String>,
    doc_name: Mutable<Option<String>>, // unused
    doc_pos: Mutable<Option<String>>,  // unused

    admission_note_doctors: MutableVec<Rc<AdmissionNoteDoctor>>, // => doc_name, doc_pos

    // NOT in ipd_doctor_admission_note, just for show
    period_period: Mutable<String>,
    period_period_normal: Mutable<String>,
    period_period_disorders: Mutable<String>,
    period_period_lmp: Mutable<String>,
    period_period_menopause: Mutable<String>,
    period_occupation: Mutable<String>,
    period_no_risk: Mutable<String>,
    period_smoking: Mutable<String>,
    period_smoke_year: Mutable<String>,
    period_smoke_frequency: Mutable<String>,
    period_smoke_stopped: Mutable<String>,
    period_alcohol: Mutable<String>,
    period_alc_year: Mutable<String>,
    period_alc_frequency: Mutable<String>,
    period_alc_stopped: Mutable<String>,
    period_medication_used: Mutable<String>,
    period_med_name: Mutable<String>,
    period_med_year: Mutable<String>,
    period_med_frequency: Mutable<String>,
    period_med_stopped: Mutable<String>,

    braden_modal: Mutable<Option<Rc<Braden>>>,
    addict_assist_modal: Mutable<Option<Rc<AddictAssistV2>>>,
    amphetamine_awq_modal: Mutable<Option<Rc<AmphetamineAwqV2>>>,
    aggression_oas_modal: Mutable<Option<Rc<AggressionOAS>>>,
    alcohol_audit_modal: Mutable<Option<Rc<AlcoholAudit>>>,
    alcohol_ciwa_modal: Mutable<Option<Rc<AlcoholCiwaAr>>>,
    alcohol_aws_modal: Mutable<Option<Rc<AlcoholAws>>>,
    nicotin_ftnd_modal: Mutable<Option<Rc<NicotinFtnd>>>,
    depress_2q_modal: Mutable<Option<Rc<Depress2Q>>>,
    depress_9q_modal: Mutable<Option<Rc<Depress9Q>>>,
    depress_cdi_modal: Mutable<Option<Rc<DepressCdi>>>,
    depress_cesd_modal: Mutable<Option<Rc<DepressCesD>>>,
    depress_phqa_modal: Mutable<Option<Rc<DepressPhqA>>>,
    suicide_8q_modal: Mutable<Option<Rc<Suicide8Q>>>,
    stress_st5_modal: Mutable<Option<Rc<StressST5>>>,
    ptsd_screen_modal: Mutable<Option<Rc<PtsdScreen>>>,
    ptsd_pisces_modal: Mutable<Option<Rc<PtsdPisces10>>>,
    ptsd_cries_modal: Mutable<Option<Rc<PtsdCries13>>>,
}

impl IpdAdmissionNoteDrPage {
    pub fn new(an: String) -> Rc<Self> {
        Rc::new(Self {
            an: Mutable::new(an),
            pen_color: Mutable::new(String::from("#0000ff")),
            pen_width: Mutable::new(2),
            all_body_image: Mutable::new(Rc::new(String::from(
                r#"<svg id="svg" width="800" height="600" version="1.1" viewBox="0 0 800 600" xmlns="http://www.w3.org/2000/svg"></svg>"#,
            ))),
            ..Default::default()
        })
    }

    fn is_female(&self) -> bool {
        self.patient
            .lock_ref()
            .patient
            .lock_ref()
            .as_ref()
            .and_then(|pt| pt.sex.as_ref())
            .map(|sex| sex == "2")
            .unwrap_or_default()
    }

    fn patient_signal(&self) -> impl Signal<Item = Option<Rc<PatientInfo>>> + use<> {
        self.patient.signal_cloned().map(|pt_cpn| pt_cpn.patient.signal_cloned()).flatten()
    }

    fn is_female_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.patient_signal().map(|pt_opt| pt_opt.map(|pt| pt.sex == Some(String::from("2"))).unwrap_or_default())
    }

    /// return true if None
    fn is_puberty_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.patient_signal().map(|pt_opt| {
            pt_opt
                .map(|pt| if let (Some(sex), Some(year)) = (&pt.sex, &pt.age_y) { sex == "2" && *year > 8 } else { true })
                .unwrap_or_default()
        })
    }

    /// return true if None
    fn is_child_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.patient_signal().map(|pt_opt| pt_opt.and_then(|pt| pt.age_y.map(|age_y| age_y < 9)).unwrap_or(true))
    }
    /// return true if None
    fn is_not_neonate_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.patient_signal().map(|pt_opt| pt_opt.and_then(|pt| pt.age_y.map(|age_y| age_y > 0)).unwrap_or(true))
    }

    fn load_all_body_image(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {

                match get_text_from_url(ALL_BODY_SVG_URL, app.state()).await {
                    Ok(response) => {
                        if let Some(image) = response {
                            page.all_body_image.set(Rc::new(image));
                            page.loaded_all_body_image.set(true);
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        app.async_load(true, clone!(app, page => async move {
            // GET `EndPoint::IpdAdmissionNoteDrAn`
            match IpdAdmissionNoteDrRaw::call_api_get(&page.an.lock_ref(), app.state()).await {
                Ok(response) => {
                    let old_regdatetime = response.old_regdatetime.to_owned();
                    if let Some(period) = response.period.as_ref() {
                        page.period_period.set_neq(period.period.to_owned().unwrap_or_default());
                        page.period_period_normal.set_neq(period.period_normal.to_owned().unwrap_or_default());
                        page.period_period_disorders.set_neq(period.period_disorders.to_owned().unwrap_or_default());
                        page.period_period_lmp.set_neq(period.period_lmp.to_owned().unwrap_or_default());
                        page.period_period_menopause.set_neq(period.period_menopause.to_owned().unwrap_or_default());
                        page.period_occupation.set_neq(period.occupation.to_owned().unwrap_or_default());
                        page.period_no_risk.set_neq(period.no_risk.to_owned().unwrap_or_default());
                        page.period_smoking.set_neq(period.smoking.to_owned().unwrap_or_default());
                        page.period_smoke_year.set_neq(period.smoke_year.to_owned().unwrap_or_default());
                        page.period_smoke_frequency.set_neq(period.smoke_frequency.to_owned().unwrap_or_default());
                        page.period_smoke_stopped.set_neq(period.smoke_stopped.to_owned().unwrap_or_default());
                        page.period_alcohol.set_neq(period.alcohol.to_owned().unwrap_or_default());
                        page.period_alc_year.set_neq(period.alc_year.to_owned().unwrap_or_default());
                        page.period_alc_frequency.set_neq(period.alc_frequency.to_owned().unwrap_or_default());
                        page.period_alc_stopped.set_neq(period.alc_stopped.to_owned().unwrap_or_default());
                        page.period_medication_used.set_neq(period.medication_used.to_owned().unwrap_or_default());
                        page.period_med_name.set_neq(period.med_name.to_owned().unwrap_or_default());
                        page.period_med_year.set_neq(period.med_year.to_owned().unwrap_or_default());
                        page.period_med_frequency.set_neq(period.med_frequency.to_owned().unwrap_or_default());
                        page.period_med_stopped.set_neq(period.med_stopped.to_owned().unwrap_or_default());
                    }
                    if let Some(pe) = response.opdscreen_pe.as_ref() {
                        page.chief_complaints_opdscreen.set_neq(pe.cc.to_owned().unwrap_or_default());
                        page.medical_history_opdscreen.set_neq(pe.hpi.to_owned().unwrap_or_default());
                        page.t_opdscreen.set_neq(pe.temperature.map(|f| f.to_string()).unwrap_or_default());
                        page.pr_opdscreen.set_neq(pe.pulse.map(|f| f.to_string()).unwrap_or_default());
                        page.rr_opdscreen.set_neq(pe.rr.map(|f| f.to_string()).unwrap_or_default());
                        page.bp_opdscreen.set_neq([&pe.bps.map(|f| f.to_string()).unwrap_or(String::from("-")), "/", &pe.bpd.map(|f| f.to_string()).unwrap_or(String::from("-"))].concat());
                        page.pe_bw.set_neq(pe.bw.map(|f| f.to_string()).unwrap_or_default());
                        page.pe_height.set_neq(pe.height.map(|i| i.to_string()).unwrap_or_default());
                    } else {
                        page.pe_bw.set_neq(String::new());
                        page.pe_height.set_neq(String::new());
                    }
                    // has note
                    if let Some(note) = response.admission_note.as_ref() {
                        page.hn.set_neq(str_some(note.hn.to_owned()));
                        page.chief_complaints.set_neq(note.chief_complaints.to_owned().unwrap_or_default());
                        page.medical_history.set_neq(note.medical_history.to_owned().unwrap_or_default());
                        page.bp.set_neq(note.bp.to_owned().unwrap_or_default());
                        page.t.set_neq(note.t.as_ref().map(|d| d.to_string()).unwrap_or_default());
                        page.pr.set_neq(note.pr.map(|u| u.to_string()).unwrap_or_default());
                        page.rr.set_neq(note.rr.map(|i| i.to_string()).unwrap_or_default());
                        page.gcs.set_neq(note.gcs.map(|i| i.to_string()).unwrap_or_default());
                        page.e.set_neq(note.e.to_owned().unwrap_or_default());
                        page.v.set_neq(note.v.to_owned().unwrap_or_default());
                        page.m.set_neq(note.m.to_owned().unwrap_or_default());
                        page.braden_scale.set_neq(note.braden_scale.to_owned().unwrap_or_default());

                        page.admission_note_id.set_neq(zero_none(note.admission_note_id));
                        page.receiver_medication_date.set_neq(note.receiver_medication_date.map(|d| d.to_string()).unwrap_or_default());
                        page.receiver_medication_time.set_neq(note.receiver_medication_time.map(|t| t.js_string()).unwrap_or_default());
                        page.arrive_by.set_neq(note.arrive_by.as_ref().map(|by| ArriveBy::from_str(by)).unwrap_or_default());
                        page.informant_patient.set_neq(note.informant_patient.to_owned());
                        page.informant_relatives.set_neq(note.informant_relatives.to_owned());
                        page.informant_deliverer.set_neq(note.informant_deliverer.to_owned());
                        page.informant_etc.set_neq(note.informant_etc.to_owned());
                        page.take_medication_by.set_neq(note.take_medication_by.as_ref().map(|by| TakeMedicationBy::from_str(by)).unwrap_or_default());
                        page.taken_by_relative.set_neq(note.taken_by_relative.to_owned().unwrap_or_default());
                        page.taken_by_nurse.set_neq(note.taken_by_nurse.to_owned().unwrap_or_default());
                        page.taken_by_crib.set_neq(note.taken_by_crib.to_owned().unwrap_or_default());
                        page.taken_by_etc.set_neq(note.taken_by_etc.to_owned().unwrap_or_default());
                        page.taken_by.set_neq(note.taken_by.to_owned().unwrap_or_default());
                        page.disease.set_neq(note.disease.to_owned().unwrap_or_default());

                        let disease_details = note.disease_detail.as_ref().map(|dd| { // "name year hospital name year hospital ..." -> Vec<Rc<DiseaseDetail>>
                            explode(dd, 3).iter().map(|chunk| {
                                Rc::new(DiseaseDetail {
                                    id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                                    name: Mutable::new(chunk[0].to_owned()),
                                    year: Mutable::new(chunk[1].to_owned()),
                                    hospital: Mutable::new(chunk[2].to_owned()),
                                })
                            }).collect::<Vec<Rc<DiseaseDetail>>>()
                        }).unwrap_or(Vec::new());

                        page.disease_details.lock_mut().replace_cloned(disease_details);
                        page.disease_etc.set_neq(note.disease_etc.to_owned());
                        page.last_dose_taken_time.set_neq(note.last_dose_taken_time.map(|dt| dt.js_string()).unwrap_or_default());
                        page.last_dose_taken_remark.set_neq(note.last_dose_taken_remark.to_owned());
                        page.operation_history.set_neq(note.operation_history.to_owned());
                        page.allergy_history.set_neq(note.allergy_history.to_owned().unwrap_or_default());
                        page.allergy_drug_history_hosxp.set_neq(note.allergy_drug_history_hosxp.to_owned().unwrap_or_default());
                        page.allergy_drug_pharmacy_check_person.set_neq(note.allergy_drug_pharmacy_check_person.to_owned());
                        page.allergy_drug_pharmacy_check_datetime.set_neq(note.allergy_drug_pharmacy_check_datetime.to_owned());

                        let allergy_drugs = note.allergy_drug_history.as_ref().map(|dh| {
                            explode(dh, 2).iter().map(|chunk| {
                                Rc::new(DrugAllergy {
                                    id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                                    agent: Mutable::new(chunk[0].to_owned()),
                                    symptom: Mutable::new(chunk[1].to_owned()),
                                })
                            }).collect::<Vec<Rc<DrugAllergy>>>()
                        }).unwrap_or(Vec::new());
                        page.allergy_drugs.lock_mut().replace_cloned(allergy_drugs);
                        let allergy_foods = note.allergy_food_history.as_ref().map(|fh| {
                            explode(fh, 2).iter().map(|chunk| {
                                Rc::new(FoodAllergy {
                                    id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                                    agent: Mutable::new(chunk[0].to_owned()),
                                    symptom: Mutable::new(chunk[1].to_owned()),
                                })
                            }).collect::<Vec<Rc<FoodAllergy>>>()
                        }).unwrap_or(Vec::new());
                        page.allergy_foods.lock_mut().replace_cloned(allergy_foods);
                        let allergy_etcs = note.allergy_etc_history.as_ref().map(|eh| {
                            explode(eh, 2).iter().map(|chunk| {
                                Rc::new(EtcAllergy {
                                    id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                                    agent: Mutable::new(chunk[0].to_owned()),
                                    symptom: Mutable::new(chunk[1].to_owned()),
                                })
                            }).collect::<Vec<Rc<EtcAllergy>>>()
                        }).unwrap_or(Vec::new());
                        page.allergy_etcs.lock_mut().replace_cloned(allergy_etcs);
                        page.allergy_detail.set_neq(note.allergy_detail.to_owned());
                        page.family_medical_history.set_neq(note.family_medical_history.to_owned().unwrap_or_default());
                        let family_medicals = note.family_medical_history_detail.as_ref().map(|fmh| {
                            explode(fmh, 2).iter().map(|chunk| {
                                Rc::new(FamilyMedical {
                                    id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                                    disease: Mutable::new(chunk[0].to_owned()),
                                    relation: Mutable::new(chunk[1].to_owned()),
                                })
                            }).collect::<Vec<Rc<FamilyMedical>>>()
                        }).unwrap_or(Vec::new());
                        page.family_medicals.lock_mut().replace_cloned(family_medicals);

                        page.receives_immunisation_history_kid.set_neq(note.receives_immunisation_history_kid.to_owned());
                        page.developmentally_kid.set_neq(note.developmentally_kid.to_owned());
                        page.g.set_neq(note.g.map(|i| i.to_string()).unwrap_or_default());
                        page.p.set_neq(note.p.to_owned().unwrap_or_default());
                        page.anc.set_neq(note.anc.to_owned().unwrap_or_default());
                        page.tt.set_neq(note.tt.map(|i| i.to_string()).unwrap_or_default());
                        page.gestational_age.set_neq(note.gestational_age.to_owned().unwrap_or_default());
                        page.gestational_day.set_neq(note.gestational_day.to_owned().unwrap_or_default());
                        page.last_child.set_neq(note.last_child.map(|i| i.to_string()).unwrap_or_default());
                        page.last_abort.set_neq(note.last_abort.to_owned().unwrap_or_default());
                        page.curette.set_neq(note.curette.to_owned().unwrap_or_default());
                        page.lmp.set_neq(note.lmp.to_owned().map(|d| d.to_string()).unwrap_or_default());
                        page.edc.set_neq(note.edc.to_owned().map(|d| d.to_string()).unwrap_or_default());
                        page.pb_no.set_neq(note.pb_no.to_owned().unwrap_or_default());
                        page.giant_baby.set_neq(note.giant_baby.to_owned().unwrap_or_default());
                        page.distocia.set_neq(note.distocia.to_owned().unwrap_or_default());
                        page.extraction.set_neq(note.extraction.to_owned());
                        page.pph.set_neq(note.pph.to_owned().unwrap_or_default());
                        page.pb_etc.set_neq(note.pb_etc.to_owned());
                        page.hf.set_neq(note.hf.map(|i| i.to_string()).unwrap_or_default());
                        page.hf_position.set_neq(note.hf_position.to_owned().unwrap_or_default());
                        page.mem_ruptured_hours.set_neq(note.mem_ruptured_hours.map(|u| u.to_string()));

                        page.lr_back_fetus.set_neq(note.lr_back_fetus.to_owned().unwrap_or_default());
                        page.lr_presentation.set_neq(note.lr_presentation.to_owned().unwrap_or_default());
                        page.lr_engagement.set_neq(note.lr_engagement.to_owned().unwrap_or_default());
                        page.lr_prominence.set_neq(note.lr_prominence.to_owned().unwrap_or_default());
                        page.lr_attitude.set_neq(note.lr_attitude.to_owned().unwrap_or_default());
                        page.lr_fhr.set_neq(note.lr_fhr.map(|u| u.to_string()).unwrap_or_default());
                        page.lr_fhr_irrigular.set_neq(note.lr_fhr_irrigular.to_owned().unwrap_or_default());
                        page.lr_efw.set_neq(note.lr_efw.map(|u| u.to_string()).unwrap_or_default());
                        let (lr_int_m, lr_int_s) = if let Some(lr_int) = &note.lr_interval { lr_int_from_quote(lr_int) } else { (0, 0) };
                        page.lr_interval_m.set_neq(zero_none(lr_int_m).map(|u| u.to_string()).unwrap_or_default());
                        page.lr_interval_s.set_neq(zero_none(lr_int_s).map(|u| u.to_string()).unwrap_or_default());
                        page.lr_duration.set_neq(note.lr_duration.map(|u| u.to_string()).unwrap_or_default());
                        page.lr_intensity.set_neq(note.lr_intensity.to_owned().unwrap_or_default());
                        page.lr_pelvic_diagonal.set_neq(note.lr_pelvic_diagonal.map(|d| d.to_string()).unwrap_or_default());
                        page.lr_pelvic_interspinous.set_neq(note.lr_pelvic_interspinous.map(|d| d.to_string()).unwrap_or_default());
                        page.lr_pelvic_sidewall.set_neq(note.lr_pelvic_sidewall.to_owned().unwrap_or_default());
                        page.lr_ischeal_spine.set_neq(note.lr_ischeal_spine.to_owned().unwrap_or_default());
                        page.lr_sacral_curve.set_neq(note.lr_sacral_curve.to_owned().unwrap_or_default());
                        page.lr_pubic_angle.set_neq(note.lr_pubic_angle.map(|u| u.to_string()).unwrap_or_default());
                        page.lr_pelvic_ok.set_neq(note.lr_pelvic_ok.to_owned().unwrap_or_default());
                        page.lr_cx_dilate.set_neq(note.lr_cx_dilate.map(|u| u.to_string()).unwrap_or_default());
                        page.lr_cx_efface.set_neq(note.lr_cx_efface.map(|u| u.to_string()).unwrap_or_default());
                        page.lr_cx_station.set_neq(note.lr_cx_station.map(|i| i.to_string()).unwrap_or_default());
                        page.lr_cx_position.set_neq(note.lr_cx_position.to_owned().unwrap_or_default());
                        page.lr_cx_consistency.set_neq(note.lr_cx_consistency.to_owned().unwrap_or_default());
                        page.lr_cx_bishop.set_neq(note.lr_cx_bishop.map(|u| u.to_string()).unwrap_or_default());
                        page.lr_cx_ok.set_neq(note.lr_cx_ok.to_owned().unwrap_or_default());
                        page.lr_membrane.set_neq(note.lr_membrane.to_owned().unwrap_or_default());
                        page.lr_amniotic_color.set_neq(note.lr_amniotic_color.to_owned().unwrap_or_default());
                        page.lr_amniotic_smell.set_neq(note.lr_amniotic_smell.to_owned().unwrap_or_default());

                        page.hiv.set_neq(note.hiv.to_owned().unwrap_or_default());
                        page.vdrl.set_neq(note.vdrl.to_owned().unwrap_or_default());
                        page.hbs_ag.set_neq(note.hbs_ag.to_owned().unwrap_or_default());
                        page.hct.set_neq(note.hct.to_owned().map(|dc| dc.to_string()).unwrap_or_default());
                        page.hiv2.set_neq(note.hiv2.to_owned().unwrap_or_default());
                        page.vdrl2.set_neq(note.vdrl2.to_owned().unwrap_or_default());
                        page.hbs_ag2.set_neq(note.hbs_ag2.to_owned().unwrap_or_default());
                        page.hct2.set_neq(note.hct2.to_owned().map(|dc| dc.to_string()).unwrap_or_default());
                        page.gr.set_neq(note.gr.to_owned().unwrap_or_default());
                        page.thalassemia.set_neq(note.thalassemia.to_owned().unwrap_or_default());
                        page.husband.set_neq(note.husband.to_owned().unwrap_or_default());
                        page.condition_pregnant.set_neq(note.condition_pregnant.to_owned());
                        page.deliver_anomalies.set_neq(note.deliver_anomalies.to_owned());
                        page.deliver_anomalies_means.set_neq(note.deliver_anomalies_means.to_owned().unwrap_or_default());
                        page.deliver_location.set_neq(note.deliver_location.to_owned().unwrap_or_default());
                        page.deliver_first_weight.set_neq(note.deliver_first_weight.to_owned().map(|dc| dc.to_string()).unwrap_or_default());
                        page.deliver_first_health.set_neq(note.deliver_first_health.to_owned().unwrap_or_default());
                        page.fant_breast_feeding_end_age_month.set_neq(note.fant_breast_feeding_end_age_month.map(|i| i.to_string()));
                        page.fant_artificial_feeding_start_age_month.set_neq(note.fant_artificial_feeding_start_age_month.map(|i| i.to_string()));
                        page.fant_feeding_etc.set_neq(note.fant_feeding_etc.to_owned());
                        page.supplementary_feeding.set_neq(note.supplementary_feeding.to_owned().unwrap_or_default());
                        page.supplementary_feeding_start_age_month.set_neq(note.supplementary_feeding_start_age_month.map(|i| i.to_string()).unwrap_or_default());
                        page.disease_operation_allergy.set_neq(note.disease_operation_allergy.to_owned());
                        page.inpatient_history.set_neq(note.inpatient_history.to_owned().unwrap_or_default());
                        page.inpatient_last_date.set_neq(note.inpatient_last_date.to_owned().unwrap_or_default());
                        page.inpatient_location.set_neq(note.inpatient_location.to_owned().unwrap_or_default());
                        page.inpatient_because.set_neq(note.inpatient_because.to_owned().unwrap_or_default());

                        page.pe_general.set_neq(note.pe_general.to_owned().unwrap_or_default());
                        page.pe_skin.set_neq(note.pe_skin.to_owned().unwrap_or_default());
                        page.pe_heent.set_neq(note.pe_heent.to_owned().unwrap_or_default());
                        page.pe_neck.set_neq(note.pe_neck.to_owned().unwrap_or_default());
                        page.pe_breastthorax.set_neq(note.pe_breastthorax.to_owned().unwrap_or_default());
                        page.pe_heart.set_neq(note.pe_heart.to_owned().unwrap_or_default());
                        page.pe_lungs.set_neq(note.pe_lungs.to_owned().unwrap_or_default());
                        page.pe_abdomen.set_neq(note.pe_abdomen.to_owned().unwrap_or_default());
                        page.pe_rectalgenitalia.set_neq(note.pe_rectalgenitalia.to_owned().unwrap_or_default());
                        page.pe_extremities.set_neq(note.pe_extremities.to_owned().unwrap_or_default());
                        page.pe_neurological.set_neq(note.pe_neurological.to_owned().unwrap_or_default());
                        page.pe_ob_gynexam.set_neq(note.pe_ob_gynexam.to_owned().unwrap_or_default());
                        page.pe_other.set_neq(note.pe_other.to_owned().unwrap_or_default());
                        page.pe_text.set_neq(note.pe_text.to_owned().unwrap_or_default());

                        page.ros_eent.set_neq(note.ros_eent.to_owned().unwrap_or_default());
                        page.ros_neuro.set_neq(note.ros_neuro.to_owned().unwrap_or_default());
                        page.ros_lung.set_neq(note.ros_lung.to_owned().unwrap_or_default());
                        page.ros_tb.set_neq(note.ros_tb.to_owned().unwrap_or_default());
                        page.ros_ht.set_neq(note.ros_ht.to_owned().unwrap_or_default());
                        page.ros_heart.set_neq(note.ros_heart.to_owned().unwrap_or_default());
                        page.ros_liver.set_neq(note.ros_liver.to_owned().unwrap_or_default());
                        page.ros_gi.set_neq(note.ros_gi.to_owned().unwrap_or_default());
                        page.ros_endocrine.set_neq(note.ros_endocrine.to_owned().unwrap_or_default());
                        page.ros_kidney.set_neq(note.ros_kidney.to_owned().unwrap_or_default());
                        page.ros_tumour.set_neq(note.ros_tumour.to_owned().unwrap_or_default());
                        page.ros_hemato.set_neq(note.ros_hemato.to_owned().unwrap_or_default());
                        page.ros_rheumato.set_neq(note.ros_rheumato.to_owned().unwrap_or_default());
                        page.ros_psychia.set_neq(note.ros_psychia.to_owned().unwrap_or_default());
                        page.ros_other.set_neq(note.ros_other.to_owned().unwrap_or_default());

                        page.addict.set_neq(note.addict.to_owned().unwrap_or_default());
                        let addict_assist = note.addict_assist.as_ref().map(|assist| {
                            explode(assist, 2).iter().map(|chunk| {
                                Rc::new(AddictAssist {
                                    id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
                                    agent: Mutable::new(chunk[0].to_owned()),
                                    score: Mutable::new(chunk[1].to_owned()),
                                })
                            }).collect::<Vec<Rc<AddictAssist>>>()
                        }).unwrap_or(Vec::new());

                        page.addict_assists.lock_mut().replace_cloned(addict_assist);
                        page.addict_inj.set_neq(note.addict_inj.to_owned().unwrap_or_default());
                        page.addict_inj_often.set_neq(note.addict_inj_often.to_owned().unwrap_or_default());
                        page.amphetamine_awq.set_neq(note.amphetamine_awq.clone().unwrap_or_default());
                        page.aggression_oas.set_neq(note.aggression_oas.clone().unwrap_or_default());
                        page.motivation_scale.set_neq(note.motivation_scale.map(|u| u.to_string()).unwrap_or_default());
                        page.craving_scale.set_neq(note.craving_scale.map(|u| u.to_string()).unwrap_or_default());
                        page.stage_of_change_id.set_neq(note.stage_of_change_id.map(|u| u.to_string()).unwrap_or_default());
                        page.alcohol_audit.set_neq(note.alcohol_audit.clone().unwrap_or_default());
                        page.alcohol_aws.set_neq(note.alcohol_aws.clone().unwrap_or_default());
                        page.alcohol_ciwa.set_neq(note.alcohol_ciwa.clone().unwrap_or_default());
                        page.depress_2q.set_neq(note.depress_2q.clone().unwrap_or_default());
                        page.depress_9q.set_neq(note.depress_9q.clone().unwrap_or_default());
                        page.depress_cdi.set_neq(note.depress_cdi.clone().unwrap_or_default());
                        page.depress_cesd.set_neq(note.depress_cesd.clone().unwrap_or_default());
                        page.depress_phqa.set_neq(note.depress_phqa.clone().unwrap_or_default());
                        page.nicotin_ftnd.set_neq(note.nicotin_ftnd.clone().unwrap_or_default());
                        page.ptsd_screen.set_neq(note.ptsd_screen.clone().unwrap_or_default());
                        page.ptsd_pisces.set_neq(note.ptsd_pisces.clone().unwrap_or_default());
                        page.ptsd_cries.set_neq(note.ptsd_cries.clone().unwrap_or_default());
                        page.suicide_8q.set_neq(note.suicide_8q.clone().unwrap_or_default());
                        page.stress_st5.set_neq(note.stress_st5.clone().unwrap_or_default());

                        page.svg_tag.set_neq(note.svg_tag.to_owned().unwrap_or_default());

                        page.impression.set_neq(note.impression.to_owned().unwrap_or_default());
                        page.diff_dx.set_neq(note.diff_dx.to_owned().unwrap_or_default());
                        page.plan_management.set_neq(note.plan_management.to_owned().unwrap_or_default());

                        page.nurse_name.set_neq(note.nurse_name.to_owned().unwrap_or_default());
                        page.nurse_pos.set_neq(note.nurse_pos.to_owned().unwrap_or_default());
                        page.nurse_licenseno.set_neq(note.nurse_licenseno.to_owned().unwrap_or_default());
                        page.doc_name.set_neq(note.doc_name.to_owned());
                        page.doc_pos.set_neq(note.doc_pos.to_owned());
                    // not has note
                    } else {
                        let patient = page.patient.lock_ref();
                        if let Some(regdate) = patient.patient.lock_ref().as_ref().and_then(|pt| pt.regdate()) {
                            page.receiver_medication_date.set_neq(regdate.to_string());
                        }
                        if let Some(regtime) = patient.patient.lock_ref().as_ref().and_then(|pt| pt.regtime()) {
                            page.receiver_medication_time.set_neq(regtime.js_string());
                        }
                        if let Some(pe) = response.opdscreen_pe.as_ref() {
                            page.hn.set_neq(pe.hn.to_owned());
                            page.chief_complaints.set_neq(pe.cc.to_owned().unwrap_or_default());
                            page.medical_history.set_neq(pe.hpi.to_owned().unwrap_or_default());
                            page.pe_general.set_neq(pe.pe_ga_text.to_owned().unwrap_or_default());
                            page.pe_heent.set_neq(pe.pe_heent_text.to_owned().unwrap_or_default());
                            page.pe_heart.set_neq(pe.pe_heart_text.to_owned().unwrap_or_default());
                            page.pe_lungs.set_neq(pe.pe_lung_text.to_owned().unwrap_or_default());
                            page.pe_abdomen.set_neq(pe.pe_ab_text.to_owned().unwrap_or_default());
                            page.pe_neurological.set_neq(pe.pe_neuro_text.to_owned().unwrap_or_default());
                            page.pe_extremities.set_neq(pe.pe_ext_text.to_owned().unwrap_or_default());
                            page.pe_text.set_neq(pe.pe.to_owned().unwrap_or_default());

                            page.bp.set_neq([pe.bps.map(|f| f.to_string()).unwrap_or(String::from("--")),String::from("/"),pe.bpd.map(|f| f.to_string()).unwrap_or(String::from("--"))].concat());
                            page.t.set_neq(pe.temperature.map(|f| f.to_string()).unwrap_or_default());
                            page.pr.set_neq(pe.pulse.map(|f| f.to_string()).unwrap_or_default());
                            page.rr.set_neq(pe.rr.map(|f| f.to_string()).unwrap_or_default());
                        } else {
                            page.hn.set_neq(str_some(patient.hn.get_cloned()));
                        }
                        // first kphis.ipd_vs_vital_sign of this AN
                        if let Some(vs) = response.vs.as_ref() {
                            let e = vs.eye.to_owned().unwrap_or_default();
                            let v = vs.verbal.to_owned().unwrap_or_default().parse::<i32>().unwrap_or(0);
                            let m = vs.movement.to_owned().unwrap_or_default();

                            page.bp.set_neq([vs.sbp.map(|u| u.to_string()).unwrap_or(String::from("--")),String::from("/"),vs.dbp.to_owned().map(|u| u.to_string()).unwrap_or(String::from("--"))].concat());
                            page.t.set_neq(vs.bt.to_owned().map(|d| d.to_string()).unwrap_or_default());
                            page.pr.set_neq(vs.pr.map(|u| u.to_string()).unwrap_or_default());
                            page.rr.set_neq(vs.rr.map(|u| u.to_string()).unwrap_or_default());
                            page.e.set_neq(e.to_string());
                            page.v.set_neq(v.to_string());
                            page.m.set_neq(m.to_string());
                            page.gcs.set_neq((e + v + m).to_string());
                            page.braden_scale.set_neq(vs.braden.to_owned().unwrap_or_default());
                        }
                        //let allergy_drugs: Vec<Rc<DrugAllergy>> = .collect();
                        page.allergy_drugs.lock_mut().extend(response.opd_er_allergy_histories.iter().map(|ah| Rc::new(ah.into())));
                        page.inpatient_last_date.set_neq(response.old_regdatetime.map(|dt| dt.js_string()).unwrap_or_default());
                        if old_regdatetime.is_some() {
                            page.inpatient_history.set_neq(String::from("เคย"));
                            page.inpatient_location.set_neq(app.app_status.lock_ref().as_ref().map(|status| status.hospital_name.clone()).unwrap_or_default());
                        }
                    }

                    page.old_regdatetime.set_neq(old_regdatetime.map(|dt| dt.js_string()));
                    page.admission_note_doctors.lock_mut().extend(response.admission_note_doctors.iter().map(|d| Rc::new(d.clone())));

                    page.raw.set(Rc::new(Mutex::new(response)));
                    page.loaded.set(true);
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }))
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - IPD Doctor Admission Note");

        let show_patient_main = ShowPatientMainCpn::new_with_an(page.an.get_cloned());
        let hn = show_patient_main.hn.clone();
        let patient_main = ShowPatientMainCpn::render(false, show_patient_main.clone(), app.clone());
        page.patient.set(show_patient_main);

        let reports = if page.an.lock_ref().len() > app.hosxp_an_len() {
            // Pre-Admit, an is VN
            vec![
                SystemReport::OpdErMedicalHistory,
                SystemReport::Lab,
                SystemReport::LabSummary,
                SystemReport::OpdErOrder,
                SystemReport::OpdErVitalSignGeneral,
                SystemReport::OpdErVitalSignNeuro,
                SystemReport::OpdErVitalSignLabour,
                SystemReport::OpdErVitalSignPsychia,
                SystemReport::OpdErIo,
                SystemReport::OpdErFocusList,
                SystemReport::OpdErFocusNote,
                SystemReport::OpdErIndexPlan,
                // SystemReport::OpdErDischargePlan,
                // SystemReport::OpdErDocument,
                // SystemReport::OpdErEventLog,
                SystemReport::IpdOrder,             // Pre-Admit
                SystemReport::IpdMedReconciliation, // Pre-Admit
            ]
        } else {
            // Admitted
            vec![
                SystemReport::IpdAdmissionNoteDr,
                SystemReport::IpdAdmissionNoteNurse,
                SystemReport::IpdMedReconciliation,
                SystemReport::Lab,
                SystemReport::LabSummary,
                SystemReport::IpdOrder,
                // SystemReport::IpdConsult,
                // SystemReport::IpdTPR,
                SystemReport::IpdVitalSignGeneral,
                SystemReport::IpdVitalSignNeuro,
                SystemReport::IpdVitalSignLabour,
                SystemReport::IpdVitalSignPsychia,
                // SystemReport::IpdIo,
                // SystemReport::IpdFocusList,
                // SystemReport::IpdFocusNote,
                // SystemReport::IpdIndexPlan,
                // SystemReport::IpdDischargePlan,
                // SystemReport::IpdMAR,
                // SystemReport::IpdSummary,
                // SystemReport::IpdEventLog,
                // SystemReport::IpdDocument,
                // SystemReport::IpdMedReconciliationHosXp,
            ]
        };

        html!("div", {
            .child(patient_main)
            .child_signal(window_size().map(|ws| ws.width < SCREEN_WIDTH_EXTRA).dedupe().map(move |is_not_wide| {
                Some(if is_not_wide {
                    Self::render_form(page.clone(), app.clone())
                } else {
                    // aside_resizer
                    let report_selected = SystemReport::new(&app.report_select.lock_ref());
                    AsideResizerCpn::render(
                        Self::render_form(page.clone(), app.clone()),
                        Some((true, page.patient.lock_ref().patient.clone())),
                        AsideResizerCpn::new(
                            Mutable::new(report_selected), Mutable::new(false),
                            Mutable::new(None), Mutable::new(false),
                            page.an.clone(), hn.clone(), reports.clone(),
                            "ipd-admission-note-dr-main", None, None, app.clone()),
                        app.clone(),
                    )
                })
            }))
        })
    }

    fn render_form(page: Rc<Self>, app: Rc<App>) -> Dom {
        let fabric_option = FabricOption { is_drawing_mode: true }.to_value();

        html!("div", {
            // load data form backend
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    // load patient data
                    Self::load(page.clone(), app.clone());
                }
                async {}
            })))
            // load allbody image form backend
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_all_body_image.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    // load all body image
                    Self::load_all_body_image(page.clone(), app.clone());
                }
                async {}
            })))
            // apply fabric.js to canvas element
            .future(map_ref!{
                let loaded = page.fabric_loaded.signal(),
                let ready = is_window_loaded() =>
                !*loaded && *ready
            }.for_each(clone!(app, page, fabric_option => move |value| {
                if value {
                    let canvas = Canvas::new("body_full", &fabric_option);
                    let closure = clone!(page => Closure::new(move || {
                        if !page.redoing.get() {
                            page.h.lock_mut().clear();
                        }
                        page.redoing.set_neq(false);
                        page.changed.set_neq(true);
                    }));
                    canvas.on("object:added", &closure);
                    closure.forget();

                    let brush = PencilBrush::new(&canvas);
                    brush.set_width(2);
                    brush.set_color("#0000ff");
                    canvas.set_free_drawing_brush(&brush);

                    page.canvas.set(Some(Rc::new(canvas)));
                    page.fabric_loaded.set_neq(true);
                }
                async {}
            })))
            // parse svg_tag
            .future(map_ref!{
                let f_loaded = page.fabric_loaded.signal(),
                let svg = page.svg_tag.signal_cloned() =>
                *f_loaded && !svg.is_empty()
            }.for_each(clone!(page => move |value| {
                clone!(page => async move {
                    if value {
                        if let Some(canvas) = page.canvas.lock_ref().as_ref() {
                            let promise = load_svg_from_string(&page.svg_tag.lock_ref());
                            let result = JsFuture::from(promise).await.unwrap();
                            let load_objects = group_svg_elements(&result.get_objects());
                            load_objects.set("selectable", false);
                            canvas.add(&load_objects);
                            canvas.render_all();
                            page.changed.set_neq(false);
                        }
                    }
                })
            })))
            // prevent `allbody.svg` overflow
            .attr("id", "ipd-admission-note-dr-main")
            .style("min-width","955px")
            // .style("min-width","SCREEN_WIDTH_EXTRA")
            .child(html!("div", {
                .class(class::CONF_B)
                .children([
                    html!("div", {
                        .class("row")
                        .children([
                            html!("div", {
                                .class("col-auto")
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUE)
                                    .child(html!("i", {.class(class::FA_L_ARROW)}))
                                    .text(" กลับ")
                                    .event(clone!(app => move |_: events::Click| {
                                        if app.go_back_else() {
                                            for route in [Route::IpdSearchPatientDr, Route::IpdSearchPatientNurse, Route::IpdSearchPatientPharmacist, Route::IpdSearchPatientOther, Route::Info] {
                                                if route.has_permission(app.state()) {
                                                    route.hard_redirect();
                                                    break;
                                                }
                                            }
                                        }
                                    }))
                                }))
                            }),
                            html!("div", {
                                .class("col-auto")
                                .child(html!("h4", {
                                    .text("แบบบันทึกการรับใหม่ผู้ป่วยใน ")
                                    .text_signal(app.hospital_name_signal())
                                }))
                            }),
                        ])
                    }),
                    html!("p"),
                    html!("div", {
                        .class(class::ROW_T)
                        .child(html!("div", {
                            .style("column-width","720px")
                            .style("column-gap","8px")
                            .children([
                                // left column
                                html!("div", {
                                    .class(class::COL_T)
                                    .style("break-inside","avoid")
                                    .children([
                                        html!("div", {
                                            .class(class::ALERT_GREEN)
                                            .class("text-center")
                                            .attr("role","alert")
                                            .text("บันทึกโดยแพทย์")
                                        }),
                                        Self::render_pi_doctor(page.clone()),
                                    ])
                                }),
                                // right column
                                html!("div", {
                                    .class(class::COL_T)
                                    .style("break-inside","avoid")
                                    .children([
                                        html!("div", {
                                            .class(class::ALERT_CYAN)
                                            .class("text-center")
                                            .attr("role","alert")
                                            .text("บันทึกโดยเจ้าหน้าที่")
                                        }),
                                        Self::render_pi_nurse(page.clone()),
                                    ])
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class(class::ALERT_CYAN)
                        .class(class::COL_SM12_C)
                        .attr("role","alert")
                        .text("ประวัติการเจ็บป่วยในอดีต")
                    }),
                    html!("div", {
                        .class(class::ROW)
                        .child(html!("div", {
                            .class("col-sm-12")
                            .child(html!("div", {
                                .class(class::CARD_BCYAN_T)
                                .child(Self::render_ph(page.clone(), app.clone()))
                            }))
                        }))
                    }),
                    Self::render_ros(page.clone()),
                    Self::render_pe(page.clone(), app.clone()),
                ])
            }))
        })
    }

    fn render_pi_doctor(page: Rc<Self>) -> Dom {
        html!("div", {
            .class("card")
            .child(html!("div", {
                .class("card-body")
                .style("overflow-y","auto")
                .child(html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class("col-md-12")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_FR_BLUE)
                                    .text("คัดลอกจาก HOSxP")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.chief_complaints.set(page.chief_complaints_opdscreen.get_cloned());
                                        page.changed.set_neq(true);
                                    }))
                                }),
                                doms::label_row_12("อาการสำคัญ"),
                                html!("div", {
                                    .class(class::ROW)
                                    .child(html!("div", {
                                        .class("col-sm-12")
                                        .child(html!("textarea" => HtmlTextAreaElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("rows","4")
                                            .apply(mixins::textarea_value_auto_expand(page.chief_complaints.clone(), page.changed.clone()))
                                        }))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("col-md-12")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_FR_BLUE)
                                    .text("คัดลอกจาก HOSxP")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.medical_history.set(page.medical_history_opdscreen.get_cloned());
                                        page.changed.set_neq(true);
                                    }))
                                }),
                                doms::label_row_12("ประวัติการเจ็บป่วยปัจจุบัน"),
                                html!("div", {.class(class::ROW)
                                    .child(html!("div", {
                                        .class("col-sm-12")
                                        .child(html!("textarea" => HtmlTextAreaElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("rows","4")
                                            .apply(mixins::textarea_value_auto_expand(page.medical_history.clone(), page.changed.clone()))
                                        }))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("col-md-12")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_FR_BLUE)
                                    .text("คัดลอกจาก HOSxP")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.t.set(page.t_opdscreen.get_cloned());
                                        page.pr.set(page.pr_opdscreen.get_cloned());
                                        page.rr.set(page.rr_opdscreen.get_cloned());
                                        page.bp.set(page.bp_opdscreen.get_cloned());
                                        page.e.set_neq(String::new());
                                        page.v.set_neq(String::new());
                                        page.m.set_neq(String::new());
                                        page.gcs.set_neq(String::new());
                                        page.braden_scale.set_neq(String::new());
                                        page.changed.set_neq(true);
                                    }))
                                }),
                                doms::label_row_12("สัญญาณชีพแรกรับ"),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .text("BP")
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "text")
                                                .attr("maxlength", "10")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("aria-describedby","bp-span")
                                                .apply(mixins::string_value(page.bp.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("bp-span","mmHg"),
                                        ])
                                    }))
                                }),
                                html!("label", {
                                    .class(class::COL_SM3_R)
                                    .text("E")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .attr("maxlength", "1")
                                            .class(class::FORM_CTRL_SM)
                                            .apply(mixins::string_value(page.e.clone(), page.changed.clone()))
                                        }))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .text("T")
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("step", "0.1")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("aria-describedby","t-span")
                                                .apply(mixins::string_value(page.t.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("t-span","° C"),
                                        ])
                                    }))
                                }),
                                html!("label", {
                                    .class(class::COL_SM3_R)
                                    .text("V")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .attr("maxlength", "1")
                                            .class(class::FORM_CTRL_SM)
                                            .apply(mixins::string_value(page.v.clone(), page.changed.clone()))
                                        }))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .text("PR")
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("aria-describedby","pr-span")
                                                .apply(mixins::string_value(page.pr.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("pr-span","/min"),
                                        ])
                                    }))
                                }),
                                html!("label", {
                                    .class(class::COL_SM3_R)
                                    .text("M")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .attr("maxlength", "1")
                                            .class(class::FORM_CTRL_SM)
                                            .apply(mixins::string_value(page.m.clone(), page.changed.clone()))
                                        }))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .text("RR")
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("aria-describedby","rr-span")
                                                .apply(mixins::string_value(page.rr.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("rr-span","/min"),
                                        ])
                                    }))
                                }),
                                html!("label", {
                                    .class(class::COL_SM3_R)
                                    .text("Neuro sign GCS")
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("aria-describedby","gcs-span")
                                                .apply(mixins::string_value(page.gcs.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("gcs-span","คะแนน"),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .text("Braden Scale")
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#bradenModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.braden_modal.set(Some(Braden::new(
                                                        page.braden_scale.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.braden_scale.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..10 => "crimson",
                                                        10..13 => "salmon",
                                                        13..15 => "pink",
                                                        15..19 => "gold",
                                                        19.. => "inherit"
                                                    }
                                                })))
                                                .text_signal(page.braden_scale.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..10 => "Very high risk",
                                                        10..13 => "High risk",
                                                        13..15 => "Moderate risk",
                                                        15..19 => "At risk",
                                                        19.. => "No risk"
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("hr"),
                        doms::label_row_12("น้ำหนักและส่วนสูง"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .text("น้ำหนัก")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input", {
                                                .attr("type", "text")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("disabled","")
                                                .prop_signal("value", map_ref!{
                                                    let pt_bw = page.patient_signal().map(|pt_opt| pt_opt.and_then(|pt| pt.latest_bw).map(|bw| {
                                                        decimal_rescale(bw, 3).to_string()
                                                    }).unwrap_or_default()),
                                                    let pe_bw = page.pe_bw.signal_cloned() =>
                                                    if pe_bw.is_empty() {pt_bw.to_owned()} else {pe_bw.to_owned()}
                                                })
                                            }),
                                            html!("span", {
                                                .class("input-group-text")
                                                .text("Kg.")
                                            }),
                                        ])
                                    }))
                                }),
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .text("ส่วนสูง")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input", {
                                                .attr("type", "text")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("disabled","")
                                                .prop_signal("value", map_ref!{
                                                    let pt_ht = page.patient_signal().map(|pt_opt| pt_opt.and_then(|pt| pt.latest_height).map(|ht| ht.to_string()).unwrap_or_default()),
                                                    let pe_ht = page.pe_height.signal_cloned() =>
                                                    if pe_ht.is_empty() {pt_ht.to_owned()} else {pe_ht.to_owned()}
                                                })
                                            }),
                                            html!("span", {
                                                .class("input-group-text")
                                                .text("cm.")
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "bradenModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.braden_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| Braden::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                    ])
                }))
            }))
        })
    }

    fn render_pi_nurse(page: Rc<Self>) -> Dom {
        html!("div", {
            .class("card")
            .child(html!("div", {
                .class("card-body")
                .style("overflow-y","auto")
                .child(html!("div",{
                    .class("row")
                    .child(html!("div", {
                        .class("col-md-12")
                        .children([
                            doms::label_row_12("เข้ารับการรักษาโดย"),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "entered_by1")
                                                .attr("value","มาเอง")
                                                .with_node!(element => {
                                                    .future(page.take_medication_by.signal_cloned().for_each(clone!(element => move |by| {
                                                        if matches!(by, TakeMedicationBy::YourSelf) {
                                                            element.set_checked(true);
                                                        } else {
                                                            element.set_checked(false);
                                                        }
                                                        async {}
                                                    })))
                                                    .event(clone!(page => move |_: events::Input| {
                                                        let value = TakeMedicationBy::from_str(&element.value());
                                                        if page.take_medication_by.get_cloned() != value {
                                                            page.take_medication_by.set_neq(value);
                                                            page.changed.set_neq(true);
                                                        }
                                                    }))
                                                })
                                            }),
                                            doms::label_check_for("entered_by1","มาเอง"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "entered_by2")
                                                .attr("value","แพทย์นัด")
                                                .with_node!(element => {
                                                    .future(page.take_medication_by.signal_cloned().for_each(clone!(element => move |by| {
                                                        if matches!(by, TakeMedicationBy::Appointment) {
                                                            element.set_checked(true);
                                                        } else {
                                                            element.set_checked(false);
                                                        }
                                                        async {}
                                                    })))
                                                    .event(clone!(page => move |_: events::Input| {
                                                        let value = TakeMedicationBy::from_str(&element.value());
                                                        if page.take_medication_by.get_cloned() != value {
                                                            page.take_medication_by.set_neq(value);
                                                            page.changed.set_neq(true);
                                                        }
                                                    }))
                                                })
                                            }),
                                            doms::label_check_for("entered_by2","แพทย์นัด"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "entered_by3")
                                                .with_node!(element => {
                                                    .future(page.take_medication_by.signal_cloned().for_each(clone!(element => move |by| {
                                                        if matches!(by, TakeMedicationBy::ReferFrom(_)) {
                                                            element.set_checked(true);
                                                        } else {
                                                            element.set_checked(false);
                                                        }
                                                        async {}
                                                    })))
                                                    .event(clone!(page => move |_: events::Input| {
                                                        let value = TakeMedicationBy::ReferFrom(String::new());
                                                        if page.take_medication_by.get_cloned() != value {
                                                            page.take_medication_by.set_neq(value);
                                                            page.changed.set_neq(true);
                                                        }
                                                    }))
                                                })
                                            }),
                                            doms::label_check_for("entered_by3","ส่งตัวจาก"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class("col-sm-5")
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .prop_signal("value", page.take_medication_by.signal_cloned().map(|by| {
                                                if let TakeMedicationBy::ReferFrom(mean) = by.clone() {
                                                    mean
                                                } else {
                                                    String::new()
                                                }
                                            }))
                                            .with_node!(element => {
                                                .future(page.take_medication_by.signal_cloned().for_each(clone!(element => move |by| {
                                                    if matches!(by, TakeMedicationBy::ReferFrom(_)) {
                                                        element.set_disabled(false);
                                                    } else {
                                                        element.set_disabled(true);
                                                    }
                                                    async {}
                                                })))
                                                .event(clone!(page => move |_: events::Change| {
                                                    let value = TakeMedicationBy::from_str(&element.value());
                                                    if page.take_medication_by.get_cloned() != value {
                                                        page.take_medication_by.set_neq(value);
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                            })
                                        }))
                                    }),
                                ])
                            }),
                            html!("hr"),
                            doms::label_row_12("นำส่งผู้ป่วยโดย"),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", "r1")
                                                .apply(mixins::checkbox_toggle(page.taken_by_relative.clone(), page.changed.clone(), "Y", "N"))
                                            }),
                                            doms::label_check_for("r1","ญาติ"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", "r2")
                                                .apply(mixins::checkbox_toggle(page.taken_by_nurse.clone(), page.changed.clone(), "Y", "N"))
                                            }),
                                            doms::label_check_for("r2","พยาบาล"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM3_R)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", "r3")
                                                .apply(mixins::checkbox_toggle(page.taken_by_crib.clone(), page.changed.clone(), "Y", "N"))
                                            }),
                                            doms::label_check_for("r3","พนักงานเปล"),
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", "r4")
                                                .apply(mixins::checkbox_toggle(page.taken_by_etc.clone(), page.changed.clone(), "Y", "N"))
                                                .event(clone!(page => move |_:events::Click| {
                                                    mixins::with_string("", page.taken_by.clone(), page.changed.clone());
                                                }))
                                            }),
                                            doms::label_check_for("r4","อื่น ๆ"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class("col-sm-5")
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .apply(mixins::string_value(page.taken_by.clone(), page.changed.clone()))
                                            .apply(mixins::other_not_match_disable(page.taken_by_etc.clone(), "Y"))
                                        }))
                                    }),
                                ])
                            }),
                            html!("hr"),
                            doms::label_row_12("ผู้ให้ข้อมูล"),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "e1self")
                                                .apply(mixins::radio_opt_match_or_none(page.informant_patient.clone(), page.changed.clone(), "ผู้ป่วย"))
                                            }),
                                            doms::label_check_for("e1self","ผู้ป่วย"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM3)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "e1unconscious")
                                                .apply(mixins::radio_opt_match(page.informant_patient.clone(), page.changed.clone(), "ไม่รู้สึกตัว"))
                                            }),
                                            doms::label_check_for("e1unconscious","ไม่รู้สึกตัว"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM4)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "e1cannot")
                                                .apply(mixins::radio_opt_match(page.informant_patient.clone(), page.changed.clone(), "ซักประวัติไม่ได้"))
                                            }),
                                            doms::label_check_for("e1cannot","ซักประวัติไม่ได้"),
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", "e2")
                                                .apply(mixins::checkbox_some(page.informant_relatives.clone(), page.changed.clone()))
                                            }),
                                            doms::label_check_for("e2","ญาติ"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class("col-sm-5")
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .attr("maxlength", "40")
                                            .class(class::FORM_CTRL_SM)
                                            .apply(mixins::opt_string_value(page.informant_relatives.clone(), page.changed.clone()))
                                            .apply(mixins::other_none_disable(page.informant_relatives.clone()))
                                        }))
                                    })
                                ])
                            }),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", "e3")
                                                .apply(mixins::checkbox_some(page.informant_deliverer.clone(), page.changed.clone()))
                                            }),
                                            doms::label_check_for("e3","ผู้นำส่ง"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class("col-sm-5")
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .apply(mixins::opt_string_value(page.informant_deliverer.clone(), page.changed.clone()))
                                            .apply(mixins::other_none_disable(page.informant_deliverer.clone()))
                                        }))
                                    })
                                ])
                            }),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", "e4")
                                                .apply(mixins::checkbox_some(page.informant_etc.clone(), page.changed.clone()))
                                            }),
                                            doms::label_check_for("e4","อื่นๆ"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class("col-sm-5")
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .apply(mixins::opt_string_value(page.informant_etc.clone(), page.changed.clone()))
                                            .apply(mixins::other_none_disable(page.informant_etc.clone()))
                                        }))
                                    })
                                ])
                            }),
                            html!("hr"),
                            doms::label_row_12("มาถึงหอผู้ปวยโดย"),
                            html!("div", {
                                .class(class::FLEX_WRAP_ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "w1")
                                                .attr("value","เดินมา")
                                                .prop_signal("checked", page.arrive_by.signal_cloned().map(|by| {
                                                    matches!(by, ArriveBy::Walk)
                                                }))
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Input| {
                                                        let value = ArriveBy::from_str(&element.value());
                                                        if page.arrive_by.get_cloned() != value {
                                                            page.arrive_by.set_neq(value);
                                                            page.changed.set_neq(true);
                                                        }
                                                    }))
                                                })
                                            }),
                                            doms::label_check_for("w1","เดินมา"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "w2")
                                                .attr("value","รถนั่ง")
                                                .prop_signal("checked", page.arrive_by.signal_cloned().map(|by| {
                                                    matches!(by, ArriveBy::WheelChair)
                                                }))
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Input| {
                                                        let value = ArriveBy::from_str(&element.value());
                                                        if page.arrive_by.get_cloned() != value {
                                                            page.arrive_by.set_neq(value);
                                                            page.changed.set_neq(true);
                                                        }
                                                    }))
                                                })
                                            }),
                                            doms::label_check_for("w2","รถนั่ง"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "w3")
                                                .attr("value","รถนอน")
                                                .prop_signal("checked", page.arrive_by.signal_cloned().map(|by| {
                                                    matches!(by, ArriveBy::Stretcher)
                                                }))
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Input| {
                                                        let value = ArriveBy::from_str(&element.value());
                                                        if page.arrive_by.get_cloned() != value {
                                                            page.arrive_by.set_neq(value);
                                                            page.changed.set_neq(true);
                                                        }
                                                    }))
                                                })
                                            }),
                                            doms::label_check_for("w3","รถนอน"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM3_R)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "w4")
                                                .attr("value","รถ Transfer")
                                                .prop_signal("checked", page.arrive_by.signal_cloned().map(|by| {
                                                    matches!(by, ArriveBy::Refer)
                                                }))
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Input| {
                                                        let value = ArriveBy::from_str(&element.value());
                                                        if page.arrive_by.get_cloned() != value {
                                                            page.arrive_by.set_neq(value);
                                                            page.changed.set_neq(true);
                                                        }
                                                    }))
                                                })
                                            }),
                                            doms::label_check_for("w4","รถ Transfer"),
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {.class("col-sm-1")}),
                                    html!("div", {
                                        .class(class::FORM_CHK_COL_SM2)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "radio")
                                                .class("form-check-input")
                                                .attr("id", "w5")
                                                .prop_signal("checked", page.arrive_by.signal_cloned().map(|by| {
                                                    matches!(by, ArriveBy::Others(_))
                                                }))
                                                .event(clone!(page => move |_: events::Input| {
                                                    let value = ArriveBy::Others(String::new());
                                                    if page.arrive_by.get_cloned() != value {
                                                        page.arrive_by.set_neq(value);
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                            }),
                                            doms::label_check_for("w5","อื่น ๆ"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class("col-sm-5")
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .prop_signal("value", page.arrive_by.signal_cloned().map(|by| {
                                                if let ArriveBy::Others(mean) = by.clone() {
                                                    mean
                                                } else {
                                                    String::new()
                                                }
                                            }))
                                            .with_node!(element => {
                                                .future(page.arrive_by.signal_cloned().for_each(clone!(element => move |by| {
                                                    if matches!(by, ArriveBy::Others(_)) {
                                                        if element.has_attribute("disabled") {
                                                            element.set_disabled(false);
                                                        }
                                                    } else {
                                                        element.set_disabled(true);
                                                    }
                                                    async {}
                                                })))
                                                .event(clone!(page => move |_: events::Change| {
                                                    let value = ArriveBy::from_str(&element.value());
                                                    if page.arrive_by.get_cloned() != value {
                                                        page.arrive_by.set_neq(value);
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                            })
                                        }))
                                    })
                                ])
                            }),
                            html!("hr"),
                            doms::label_row_12("วันที่รับไว้รักษา"),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    doms::span_group_text("วันที่"),
                                    doms::date_picker(
                                        page.receiver_medication_date.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "receiver_medication_date"),
                                        |s| s, always(None),
                                    ),
                                    doms::span_group_text("เวลา"),
                                    doms::time_picker(
                                        page.receiver_medication_time.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "receiver_medication_time"),
                                        |s| s, always(None),
                                    ),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .text("ปัจจุบัน")
                                        .event(clone!(page => move |_:events::Click| {
                                            let now = js_now();
                                            page.receiver_medication_date.set(now.date().to_string());
                                            page.receiver_medication_time.set(now.time().js_string());
                                            page.changed.set_neq(true);
                                        }))
                                    }),
                                ])
                            }),
                        ])
                    }))
                }))
            }))
        })
    }

    fn render_ph(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("card-body")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .child(html!("div", {
                        .class("col-sm-12")
                        .children([
                            html!("h4", {
                                .child(html!("span", {
                                    .class(class::BADGE_CYAN)
                                    .style("cursor","default")
                                    .text_signal(page.patient_signal().map(|pt_opt| {
                                        pt_opt.and_then(|pt| pt.age_y).map(|age_y| {
                                            if age_y < 1 {
                                                String::from("ผู้ป่วยเด็กอายุ < 1 ปี")
                                            } else {
                                                String::from("ผู้ป่วยทั่วไป")
                                            }
                                        }).unwrap_or(String::from("ไม่ระบุอายุ"))
                                    }))
                                }))
                            }),
                            html!("label", {
                                .text_signal(page.patient_signal().map(|pt_opt| {
                                    pt_opt.map(|pt| {
                                        [
                                            "( อายุ : ", &pt.age_y.unwrap_or_default().to_string(), " ปี ",
                                            &pt.age_m.unwrap_or_default().to_string(), " เดือน ",
                                            &pt.age_d.unwrap_or_default().to_string(), " วัน )"
                                        ].concat()
                                    }).unwrap_or_default()
                                }))
                            }),
                        ])
                    }))
                }),
                doms::label_row_12("โรคประจำตัว"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "t1")
                                    .apply(mixins::radio_match_empty(page.disease.clone(), page.changed.clone(), NOTHING))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.disease_details.lock_mut().clear();
                                    }))
                                }),
                                doms::label_check_for("t1", NOTHING),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "t2")
                                    .apply(mixins::radio_match(page.disease.clone(), page.changed.clone(), SOMETHING))
                                }),
                                doms::label_check_for("t2", "มี (ระบุ)"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-2")}),
                        html!("div", {
                            .class("col-sm-3")
                            .child(html!("label", {
                                .class("text-end")
                                .child(html!("a", {
                                    .attr("href","#")
                                    .child(html!("i", {.class(class::FA_PLUS_SQ)}))
                                    .text(" โรค")
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                        event.prevent_default();
                                        page.disease_details.lock_mut().push_cloned(DiseaseDetail::new());
                                        mixins::with_string(SOMETHING, page.disease.clone(), page.changed.clone());
                                    }))
                                }))
                            }))
                        }),
                        html!("div", {
                            .class("col-sm-3")
                            .child(html!("label", {
                                .class("text-end")
                                .text("จำนวนปี")
                            }))
                        }),
                        html!("div", {
                            .class("col-sm-3")
                            .child(html!("label", {
                                .class("text-end")
                                .text("สถานพยาบาลที่รักษา")
                            }))
                        }),
                    ])
                }),
            ])
            .children_signal_vec(page.disease_details.signal_vec_cloned().map(clone!(page, app => move |dd| {
                DiseaseDetail::render(dd, page.clone(), app.clone())
            })))
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-2")}),
                        html!("div", {
                            .class("col-auto")
                            .child(html!("div", {
                                .class("card")
                                .children([
                                    html!("div", {
                                        .class("card-header")
                                        .text("การกินยามื้อสุดท้าย (Last Dose)")
                                    }),
                                    html!("div", {
                                        .class("card-body")
                                        .child(html!("div", {
                                            .class("row")
                                            .children([
                                                html!("div", {
                                                    .class(class::COLA_T)
                                                    .children([
                                                        html!("label", {.text("เวลาที่กินยามื้อสุดท้าย")}),
                                                        doms::datetime_picker(
                                                            page.last_dose_taken_time.clone(),
                                                            page.changed.clone(), always(false),
                                                            |d| d.style("min-width","175px"),
                                                            |d| d.class("form-control-sm"),
                                                            |d| d.class("form-control-sm"),
                                                            |s| s, always(None),
                                                        ),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(class::COLA_T)
                                                    .children([
                                                        html!("label", {.text("จำนวนยาเหลือ/หมายเหตุ")}),
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("cols", "50")
                                                            .attr("rows", "3")
                                                            .apply(mixins::opt_string_value(page.last_dose_taken_remark.clone(), page.changed.clone()))
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
                doms::label_row_12("การผ่าตัด"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "y1")
                                    // checked when Some("ไม่มี") or None
                                    .apply(mixins::radio_opt_match_or_none(page.operation_history.clone(), page.changed.clone(), NOTHING))
                                }),
                                doms::label_check_for("y1", NOTHING),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "y2")
                                    // checked when Some(not "ไม่มี")
                                    .apply(mixins::radio_opt_match_some_neq(page.operation_history.clone(), page.changed.clone(), NOTHING))
                                }),
                                doms::label_check_for("y2", "มี (ระบุ)"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::COL_SM7_OFSM2)
                    .children(page.raw.lock_ref().lock().map(|guard| {
                        guard.operation_list.iter().flat_map(|op| {[
                            html!("label", {.text(op)}),
                            html!("br"),
                        ]}).collect::<Vec<Dom>>()
                    }).unwrap_or_default())
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {
                            .class(class::COL_SM2_OFS2)
                            .child(html!("label", {.text("รายละเอียด")}))
                        }),
                        html!("div", {
                            .class("col-sm-8")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("rows", "3")
                                .apply(mixins::opt_string_match_show_empty(page.operation_history.clone(), page.changed.clone(), NOTHING))
                                .apply(mixins::other_match_none_disable(page.operation_history.clone(), NOTHING))
                            }))
                        }),
                    ])
                }),
                doms::label_row_12("ประวัติแพ้"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-2")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "u1")
                                    .attr("value", NOTHING)
                                    .with_node!(element => {
                                        .future(page.patient_signal().for_each(clone!(element => move |pt_opt| {
                                            if let Some(pt) = &pt_opt {
                                                element.set_disabled(pt.drugallergy.is_some());
                                            }
                                            async {}
                                        })))
                                        .future(page.allergy_history.signal_cloned().for_each(clone!(element => move |ah| {
                                            if ah == NOTHING || ah.is_empty() {
                                                element.set_checked(true);
                                            } else {
                                                element.set_checked(false);
                                            }
                                            async {}
                                        })))
                                        .event(clone!(page => move |_: events::Click| {
                                            page.allergy_history.set_neq(element.value());
                                            page.allergy_drugs.lock_mut().clear();
                                            page.allergy_foods.lock_mut().clear();
                                            page.allergy_etcs.lock_mut().clear();
                                            page.changed.set_neq(true);
                                        }))
                                    })
                                }),
                                doms::label_check_for("u1", NOTHING),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-2")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "u2")
                                    .attr("value", SOMETHING)
                                    .with_node!(element => {
                                        .future(map_ref! {
                                            let ah = page.allergy_history.signal_cloned(),
                                            let no_da = page.allergy_drugs.signal_vec_cloned().is_empty() =>
                                            ah == SOMETHING || !no_da
                                        }.for_each(clone!(element => move |has| {
                                            if has {
                                                element.set_checked(true);
                                            } else {
                                                element.set_checked(false);
                                            }
                                            async {}
                                        })))
                                        .event(clone!(page => move |_: events::Click| {
                                            mixins::with_string(&element.value(), page.allergy_history.clone(), page.changed.clone());
                                        }))
                                    })
                                }),
                                doms::label_check_for("u2", "มี (ระบุ)"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .text("ประวัติการแพ้ยาใน HOSxP")
                            .children([
                                html!("br"),
                                html!("small", {
                                    .class("text-info")
                                    .text("(ณ เวลาที่บันทึกแรกรับครั้งแรก)")
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class(class::FORM_CTRL_RED)
                                .attr("rows", "3")
                                .attr("readonly", "readonly")
                                .prop_signal("value", page.allergy_drug_history_hosxp.signal_cloned())
                                .with_node!(element => {
                                    .future(map_ref! {
                                        let ah = page.allergy_history.signal_cloned(),
                                        let no_da = page.allergy_drugs.signal_vec_cloned().is_empty() =>
                                        ah == SOMETHING || !no_da
                                    }.for_each(clone!(element => move |has| {
                                        if has {
                                            element.set_disabled(false);
                                        } else {
                                            element.set_disabled(true);
                                        }
                                        async {}
                                    })))
                                    .event(clone!(page => move |_: events::Change| {
                                        mixins::with_string(&element.value(), page.allergy_drug_history_hosxp.clone(), page.changed.clone());
                                    }))
                                })
                            }))
                        }),
                    ])
                }),
            ])
            .child_signal(page.raw.signal_cloned().map(|raw| {
                if let Ok(guard) = raw.lock() {
                    guard.opd_er_allergy_history.as_ref().map(|ah| {
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-1")}),
                                html!("div", {.class(class::FORM_CTRL_COL_SM2)}),
                                html!("div", {
                                    .class("col-sm-2")
                                    .text("แจ้งแพ้ยา (ER)")
                                    .children([
                                        html!("br"),
                                        html!("small", {.class("text-info")}),
                                    ])
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("textarea", {
                                        .class(class::FORM_CTRL_RED)
                                        .attr("rows", "3")
                                        .attr("readonly", "readonly")
                                        .text(ah)
                                    }))
                                }),
                            ])
                        })
                    })
                } else {
                    None
                }
            }))
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {
                            .class("col-sm-5")
                            .child(html!("div", {.class("col-sm-2")}))
                        }),
                        html!("div", {
                            .class("col-sm-3")
                            .child(html!("label", {
                                .class("text-end")
                                .text("ชื่อ")
                            }))
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("label", {
                                .class("text-end")
                                .text("อาการที่แพ้")
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-3")}),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("a", {
                                .attr("href","#")
                                .child(html!("i", {.class(class::FA_PLUS_SQ)}))
                                .text(" ยา")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.allergy_drugs.lock_mut().push_cloned(DrugAllergy::new());
                                    page.allergy_history.set_neq(String::from(SOMETHING));
                                    page.changed.set_neq(true);
                                }))
                            }))
                        }),
                    ])
                }),
            ])
            .children_signal_vec(page.allergy_drugs.signal_vec_cloned().map(clone!(page => move |ad| {
                DrugAllergy::render(ad, page.clone())
            })))
            .child(html!("div", {
                .class(class::ROW)
                .children([
                    html!("div", {.class("col-sm-3")}),
                    html!("div", {
                        .class("col-sm-2")
                        .child(html!("a", {
                            .attr("href","#")
                            .child(html!("i", {.class(class::FA_PLUS_SQ)}))
                            .text(" อาหาร")
                            .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                event.prevent_default();
                                page.allergy_foods.lock_mut().push_cloned(FoodAllergy::new());
                                page.allergy_history.set_neq(String::from(SOMETHING));
                                page.changed.set_neq(true);
                            }))
                        }))
                    }),
                ])
            }))
            .children_signal_vec(page.allergy_foods.signal_vec_cloned().map(clone!(page => move |af| {
                clone!(page => FoodAllergy::render(af, page))
            })))
            .child(html!("div", {
                .class(class::ROW)
                .children([
                    html!("div", {.class("col-sm-3")}),
                    html!("div", {
                        .class("col-sm-2")
                        .child(html!("a", {
                            .attr("href","#")
                            .child(html!("i", {.class(class::FA_PLUS_SQ)}))
                            .text(" อื่นๆ")
                            .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                event.prevent_default();
                                page.allergy_etcs.lock_mut().push_cloned(EtcAllergy::new());
                                page.allergy_history.set_neq(String::from(SOMETHING));
                                page.changed.set_neq(true);
                            }))
                        }))
                    }),
                ])
            }))
            .children_signal_vec(page.allergy_etcs.signal_vec_cloned().map(clone!(page => move |ae| {
                clone!(page => EtcAllergy::render(ae, page))
            })))
            .children([
                doms::label_row_12("ประวัติการเจ็บป่วยในครอบครัว"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "i1")
                                    .apply(mixins::radio_match_empty(page.family_medical_history.clone(), page.changed.clone(), NOTHING))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.family_medicals.lock_mut().clear();
                                        page.changed.set_neq(true);
                                    }))
                                }),
                                doms::label_check_for("i1", NOTHING),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "i2")
                                    .apply(mixins::radio_match(page.family_medical_history.clone(), page.changed.clone(), SOMETHING))
                                }),
                                doms::label_check_for("i2", "มี (ระบุ)"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-3")}),
                        html!("div", {
                            .class("col-sm-5")
                            .child(html!("a", {
                                .attr("href","#")
                                .child(html!("i", {.class(class::FA_PLUS_SQ)}))
                                .text(" โรค")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.family_medicals.lock_mut().push_cloned(FamilyMedical::new());
                                    page.family_medical_history.set_neq(String::from(SOMETHING));
                                    page.changed.set_neq(true);
                                }))
                            }))
                        }),
                        html!("div", {
                            .class("col-sm-3")
                            .child(html!("label", {
                                .class("text-end")
                                .text("เกี่ยวข้องเป็น")
                            }))
                        }),
                    ])
                }),
            ])
            .children_signal_vec(page.family_medicals.signal_vec_cloned().map(clone!(page => move |fm| {
                FamilyMedical::render(fm, page.clone())
            })))
            .children([
                Self::render_ph_occupation(page.clone()),
                Self::render_ph_period(page.clone()),
                Self::render_ph_immunization(page.clone()),
            ])
            .child_signal(page.is_child_signal().map(clone!(page => move |is_child| {
                is_child.then(|| {
                    html!("div", {
                        .child(doms::label_row_12("มารดา"))
                        .children(Self::render_ph_mother(page.clone()))
                    })
                })
            })))
            .children([
                Self::render_ph_obgyn(page.clone()),
                Self::render_ph_mch(page.clone()),
                Self::render_psychiatry(page.clone(), app.clone()),
                html!("hr"),
                Self::render_admit_hx(page.clone()),
            ])
        })
    }

    fn render_ph_occupation(page: Rc<Self>) -> Dom {
        html!("div", {
            .visible_signal(page.is_not_neonate_signal())
            .children([
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class("col-sm-5")
                            .child(html!("B", {.text("อาชีพ (จากแบบประเมินแบบแผนสุขภาพ)")}))
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("div", {
                                .class("row")
                                .child(html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("disabled", "")
                                    .apply(mixins::string_value(page.period_occupation.clone(), page.changed.clone()))
                                }))
                            }))
                        }),
                    ])
                }),
                doms::label_row_12("พฤติกรรมเสี่ยง (จากแบบประเมินแบบแผนสุขภาพ)"),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::FORM_CHK_COL_SM1_OFS1)
                        .children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "checkbox")
                                .class("form-check-input")
                                .attr("id", "no_risk")
                                .attr("disabled", "")
                                .apply(mixins::checkbox_toggle(page.period_no_risk.clone(), page.changed.clone(), "Y", "N"))
                                .event(clone!(page => move |_: events::Click| {
                                    page.period_smoking.set_neq(String::from("N"));
                                    page.period_smoke_year.set_neq(String::new());
                                    page.period_smoke_frequency.set_neq(String::new());
                                    page.period_smoke_stopped.set_neq(String::new());
                                    page.period_alcohol.set_neq(String::from("N"));
                                    page.period_alc_year.set_neq(String::new());
                                    page.period_alc_frequency.set_neq(String::new());
                                    page.period_alc_stopped.set_neq(String::new());
                                    page.period_medication_used.set_neq(String::from("N"));
                                    page.period_med_name.set_neq(String::new());
                                    page.period_med_year.set_neq(String::new());
                                    page.period_med_frequency.set_neq(String::new());
                                    page.period_med_stopped.set_neq(String::new());
                                    page.changed.set_neq(true);
                                }))
                            }),
                            doms::label_check_for("no_risk", "ปฏิเสธ"),
                        ])
                    }))
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM1_OFS1)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "smoking")
                                    .attr("disabled", "")
                                    .apply(mixins::checkbox_toggle(page.period_smoking.clone(), page.changed.clone(), "Y", "N"))
                                    .apply(mixins::other_match_empty_disable(page.period_no_risk.clone(), "Y"))
                                }),
                                doms::label_check_for("smoking", "สูบบุหรี่"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-1")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_smoke_year.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_smoking.clone(), "N"))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-1")
                            .text("ปี ปริมาณ")
                        }),
                        html!("div", {
                            .class("col-sm-1")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_smoke_frequency.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_smoking.clone(), "N"))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-1")
                            .text("/วัน เลิกเมื่อ")
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_smoke_stopped.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_smoking.clone(), "N"))
                            }))
                        }),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM1_OFS1)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "alcohol")
                                    .attr("disabled", "")
                                    .apply(mixins::checkbox_toggle(page.period_alcohol.clone(), page.changed.clone(), "Y", "N"))
                                    .apply(mixins::other_match_empty_disable(page.period_no_risk.clone(), "Y"))
                                }),
                                doms::label_check_for("alcohol", "ดื่มสุรา"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-1")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_alc_year.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_alcohol.clone(), "N"))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-1")
                            .text("ปี ปริมาณ")
                        }),
                        html!("div", {
                            .class("col-sm-1")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_alc_frequency.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_alcohol.clone(), "N"))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-1")
                            .text("/วัน เลิกเมื่อ")
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_alc_stopped.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_alcohol.clone(), "N"))
                            }))
                        }),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM1_OFS1)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "medication_used")
                                    .attr("disabled", "")
                                    .apply(mixins::checkbox_toggle(page.period_medication_used.clone(), page.changed.clone(), "Y", "N"))
                                    .apply(mixins::other_match_empty_disable(page.period_no_risk.clone(), "Y"))
                                }),
                                doms::label_check_for("medication_used", "ยา (ระบุ)"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("rows", "3")
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_med_name.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_medication_used.clone(), "N"))
                            }))
                        }),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM1_OFS2_R)
                            .text("ระยะเวลาที่ใช้")
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_med_year.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_medication_used.clone(), "N"))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-1")
                            .text("ปริมาณ")
                        }),
                        html!("div", {
                            .class("col-sm-1")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_med_frequency.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_medication_used.clone(), "N"))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-1")
                            .text("/วัน เลิกเมื่อ")
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("disabled", "")
                                .apply(mixins::string_value(page.period_med_stopped.clone(), page.changed.clone()))
                                .apply(mixins::other_match_empty_disable(page.period_medication_used.clone(), "N"))
                            }))
                        }),
                    ])
                }),
                html!("p"),
            ])
        })
    }

    fn render_ph_period(page: Rc<Self>) -> Dom {
        html!("div", {
            .visible_signal(page.is_puberty_signal())
            .children([
                doms::label_row_12("ประจำเดือน (จากแบบประเมินแบบแผนสุขภาพ)"),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::FORM_CHK_COL_SM1_OFS1)
                        .children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "radio")
                                .attr("id", "p01")
                                .class("form-check-input")
                                .attr("disabled", "")
                                .apply(mixins::radio_match_empty(page.period_period.clone(), page.changed.clone(), "ยังไม่มี"))
                                .event(clone!(page => move |_: events::Click| {
                                    page.period_period_normal.set_neq(String::new());
                                    page.period_period_disorders.set_neq(String::new());
                                    page.period_period_lmp.set_neq(String::new());
                                    page.period_period_menopause.set_neq(String::new());
                                    page.changed.set_neq(true);
                                }))
                            }),
                            doms::label_check_for("p01", "ยังไม่มี"),
                        ])
                    }))
                }),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::FORM_CHK_COL_SM1_OFS1)
                        .children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "radio")
                                .attr("id", "p02")
                                .class("form-check-input")
                                .attr("disabled", "")
                                .apply(mixins::radio_match(page.period_period.clone(), page.changed.clone(), SOMETHING))
                                .event(clone!(page => move |_: events::Click| {
                                    page.period_period_normal.set_neq(String::from("ปกติ"));
                                    page.period_period_disorders.set_neq(String::new());
                                    page.period_period_lmp.set_neq(String::new());
                                    page.period_period_menopause.set_neq(String::new());
                                    page.changed.set_neq(true);
                                }))
                            }),
                            doms::label_check_for("p02", SOMETHING),
                        ])
                    }))
                }),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::FORM_CHK_COL_SM1_OFS2)
                        .children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "radio")
                                .attr("id", "p03")
                                .class("form-check-input")
                                .attr("disabled", "")
                                .apply(mixins::radio_match(page.period_period_normal.clone(), page.changed.clone(), "ปกติ"))
                                .apply(mixins::other_not_match_disable(page.period_period.clone(), SOMETHING))
                                .event(clone!(page => move |_: events::Click| {
                                    mixins::with_string("ปกติ", page.period_period_disorders.clone(), page.changed.clone());
                                }))
                            }),
                            doms::label_check_for("p03", "ปกติ"),
                        ])
                    }))
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM1_OFS2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .attr("id", "p04")
                                    .class("form-check-input")
                                    .attr("disabled", "")
                                    .apply(mixins::radio_match(page.period_period_normal.clone(), page.changed.clone(), "ผิดปกติ"))
                                    .apply(mixins::other_not_match_disable(page.period_period.clone(), SOMETHING))
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string("ผิดปกติ", page.period_period_disorders.clone(), page.changed.clone());
                                    }))
                                }),
                                doms::label_check_for("p04", "ผิดปกติ"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("div", {
                                .class("row")
                                .child(html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("disabled", "")
                                    .apply(mixins::string_value(page.period_period_disorders.clone(), page.changed.clone()))
                                    .apply(mixins::other_match_empty_disable(page.period_period_normal.clone(), "ปกติ"))
                                }))
                            }))
                        }),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM1_OFS2_R)
                            .text("LMP")
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("div", {
                                .class("row")
                                .child(html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("disabled", "")
                                    .apply(mixins::string_value(page.period_period_lmp.clone(), page.changed.clone()))
                                    .apply(mixins::other_not_match_disable(page.period_period.clone(), SOMETHING))
                                }))
                            }))
                        }),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2_OFS1)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "p06")
                                    .attr("disabled", "")
                                    .apply(mixins::radio_match(page.period_period.clone(), page.changed.clone(), "หมดประจำเดือน"))
                                    .event(clone!(page => move |_: events::Click| {
                                        page.period_period_normal.set_neq(String::new());
                                        page.period_period_disorders.set_neq(String::new());
                                        page.period_period_lmp.set_neq(String::new());
                                        page.period_period_menopause.set_neq(String::new());
                                        page.changed.set_neq(true);
                                    }))
                                }),
                                doms::label_check_for("p06", "หมดประจำเดือน เมื่ออายุ"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-1")
                            .child(html!("div", {
                                .class("row")
                                .child(html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("disabled", "")
                                    .apply(mixins::string_value(page.period_period_menopause.clone(), page.changed.clone()))
                                    .apply(mixins::other_not_match_disable(page.period_period.clone(), "หมดประจำเดือน"))
                                }))
                            }))
                        }),
                        html!("div", {
                            .class("col-sm-1")
                            .child(html!("label", {.text("ปี")}))
                        }),
                    ])
                }),
                html!("p"),
            ])
        })
    }

    fn render_ph_immunization(page: Rc<Self>) -> Dom {
        html!("div", {
            .visible_signal(page.is_child_signal())
            .children([
                doms::label_row_12("ประวัติการได้รับภูมิคุ้มกัน (เฉพาะเด็ก)"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "o1")
                                    .apply(mixins::radio_opt_match_or_none(page.receives_immunisation_history_kid.clone(), page.changed.clone(), "ครบตามวัย"))
                                }),
                                doms::label_check_for("o1", "ครบตามวัย"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "o2")
                                    .apply(mixins::radio_opt_match_some_neq(page.receives_immunisation_history_kid.clone(), page.changed.clone(), "ครบตามวัย"))
                                }),
                                doms::label_check_for("o2", "ไม่ครบ (ระบุ)"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "40")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::opt_string_match_show_empty(page.receives_immunisation_history_kid.clone(), page.changed.clone(), "ครบตามวัย"))
                                .apply(mixins::other_match_none_disable(page.receives_immunisation_history_kid.clone(), "ครบตามวัย"))
                            }))
                        }),
                    ])
                }),
                doms::label_row_12("การพัฒนาการ (เฉพาะเด็ก)"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "p1")
                                    .apply(mixins::radio_opt_match_or_none(page.developmentally_kid.clone(), page.changed.clone(), "ปกติ"))
                                }),
                                doms::label_check_for("p1", "ปกติ"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "p2")
                                    .apply(mixins::radio_opt_match_some_neq(page.developmentally_kid.clone(), page.changed.clone(), "ปกติ"))
                                }),
                                doms::label_check_for("p2", "ผิดปกติ (ระบุ)"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "40")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::opt_string_match_show_empty(page.developmentally_kid.clone(), page.changed.clone(), "ปกติ"))
                                .apply(mixins::other_match_none_disable(page.developmentally_kid.clone(), "ปกติ"))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_ph_mother(page: Rc<Self>) -> Vec<Dom> {
        vec![
            html!("div", {
                .class(class::ROW)
                .children([
                    html!("label", {
                        .class(class::COL_SM3_R)
                        .text("G")
                    }),
                    html!("div", {
                        .class("col-sm-1")
                        .child(html!("input" => HtmlInputElement, {
                            .attr("type", "number")
                            .attr("min", "0")
                            .class(class::FORM_CTRL_SM)
                            .apply(mixins::string_value(page.g.clone(), page.changed.clone()))
                        }))
                    }),
                    html!("label", {
                        .class(class::COL_SM2_R)
                        .text("P/PAL/TPAL")
                    }),
                    html!("div", {
                        .class("col-sm-4")
                        .child(html!("input" => HtmlInputElement, {
                            .attr("type", "text")
                            .attr("maxlength", "50")
                            .class(class::FORM_CTRL_SM)
                            .attr("placeholder","เช่น 1-0-1 หรือ 1A0L1 หรือ 1P0A0L1")
                            .apply(mixins::string_value(page.p.clone(), page.changed.clone()))
                        }))
                    }),
                ])
            }),
            html!("div", {
                .class(class::ROW)
                .children([
                    html!("label", {
                        .class(class::COL_SM3_R)
                        .text("ANC ที่")
                    }),
                    html!("div", {
                        .class("col-sm-7")
                        .child(html!("input" => HtmlInputElement, {
                            .attr("type", "text")
                            .attr("maxlength", "20")
                            .class(class::FORM_CTRL_SM)
                            .apply(mixins::string_value(page.anc.clone(), page.changed.clone()))
                        }))
                    }),
                ])
            }),
            html!("div", {
                .class(class::ROW)
                .children([
                    html!("label", {
                        .class(class::COL_SM3_R)
                        .text("ได้ TT")
                    }),
                    html!("div", {
                        .class("col-sm-2")
                        .child(html!("div", {
                            .class(class::INPUT_GROUP_SM)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "number")
                                    .attr("min", "0")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("aria-describedby", "tt-span")
                                    .apply(mixins::string_value(page.tt.clone(), page.changed.clone()))
                                }),
                                doms::span_group_id("tt-span","   เข็ม  "),
                            ])
                        }))
                    }),
                ])
            }),
            html!("div", {
                .class(class::ROW)
                .children([
                    html!("label", {
                        .class(class::COL_SM3_R)
                        .text("อายุครรภ์")
                    }),
                    html!("div", {
                        .class("col-sm-2")
                        .child(html!("div", {
                            .class(class::INPUT_GROUP_SM)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .attr("maxlength", "10")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("aria-describedby", "ga-span")
                                    .apply(mixins::string_value(page.gestational_age.clone(), page.changed.clone()))
                                }),
                                doms::span_group_id("ga-span"," สัปดาห์  "),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("col-sm-2")
                        .child(html!("div", {
                            .class(class::INPUT_GROUP_SM)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .attr("maxlength", "10")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("aria-describedby", "g-day-span")
                                    .apply(mixins::string_value(page.gestational_day.clone(), page.changed.clone()))
                                }),
                                doms::span_group_id("g-day-span"," วัน  "),
                            ])
                        }))
                    }),
                ])
            }),
        ]
    }

    fn render_ph_obgyn(page: Rc<Self>) -> Dom {
        html!("div", {
            .visible_signal(page.is_puberty_signal())
            .children([
                html!("div", {
                    .class(class::ROW)
                    .child(html!("label", {
                        .class("col-sm-12")
                        .children([
                            html!("B", {.text("ประวัติด้านสูตินรีเวชกรรม ")}),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_R_BLUE)
                                .child(html!("i", {.class(class::FA_EYE)}))
                                .text(" View")
                                .event(clone!(page => move |_: events::Click| {
                                    page.display_puberty.set(!page.display_puberty.get());
                                }))
                            })
                        ])
                    }))
                }),
                html!("div", {
                    .visible_signal(page.display_puberty.signal_cloned())
                    .class("mt-3")
                    .children(Self::render_ph_mother(page.clone()))
                    .children([
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Last child")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("maxlength", "2")
                                                .attr("aria-describedby", "last-child-span")
                                                .apply(mixins::string_value(page.last_child.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("last-child-span"," ปี "),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Last abort")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .attr("maxlength", "50")
                                            .class(class::FORM_CTRL_SM)
                                            //.attr("aria-describedby", "inputGroup-sizing-sm")
                                            .apply(mixins::string_value(page.last_abort.clone(), page.changed.clone()))
                                        }))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("ประวัติการขูดมดลูก")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {
                                                .attr("value", "")
                                                .text("Choose...")
                                            }),
                                            html!("option", {
                                                .attr("value", "Y")
                                                .text("เคย")
                                            }),
                                            html!("option", {
                                                .attr("value", "N")
                                                .text("ไม่เคย")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.curette.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("ประจําเดือนครั้งสุดท้าย")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(doms::date_picker(
                                        page.lmp.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.style("min-width","120px"),
                                        |d| d.class("form-control-sm"),
                                        |d| d.class("form-control-sm"),
                                        |s| s, always(None),
                                    ))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("กำหนดการคลอด")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(doms::date_picker(
                                        page.edc.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.style("min-width","120px"),
                                        |d| d.class("form-control-sm"),
                                        |d| d.class("form-control-sm"),
                                        |s| s, always(None),
                                    ))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .child(html!("div", {
                                .class(class::COL_SM3_R)
                                .child(html!("label", {.text("มีน้ำเดินก่อนมา รพ.")}))
                            }))
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-3")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM1)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("form-check-input")
                                            .attr("id", "mem1")
                                            .apply(mixins::radio_opt_match_or_none(page.mem_ruptured_hours.clone(), page.changed.clone(), "0"))
                                        }),
                                        doms::label_check_for("mem1", NOTHING),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-3")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM1)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("form-check-input")
                                            .attr("id", "mem2")
                                            .apply(mixins::radio_opt_match_some_neq(page.mem_ruptured_hours.clone(), page.changed.clone(), "0"))
                                        }),
                                        doms::label_check_for("mem2", SOMETHING),
                                    ])
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("maxlength", "4")
                                                .attr("aria-describedby", "mem-ruptured-hours-span")
                                                .apply(mixins::opt_string_match_show_empty(page.mem_ruptured_hours.clone(), page.changed.clone(), "0"))
                                                .apply(mixins::other_match_none_disable(page.mem_ruptured_hours.clone(), "0"))
                                            }),
                                            doms::span_group_id("mem-ruptured-hours-span","ชั่วโมง"),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        doms::label_row_12("ประวัติการคลอด"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM2)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("id", "pb_no")
                                            .apply(mixins::checkbox_toggle(page.pb_no.clone(), page.changed.clone(), "Y", "N"))
                                            .future(page.pb_no.signal_cloned().for_each(clone!(page => move |no| {
                                                if no == "Y" {
                                                    page.giant_baby.set_neq(String::new());
                                                    page.distocia.set_neq(String::new());
                                                    page.extraction.set_neq(None);
                                                    page.pph.set_neq(String::new());
                                                    page.pb_etc.set_neq(None);
                                                }
                                                async {}
                                            })))
                                        }),
                                        doms::label_check_for("pb_no", "ปฏิเสธ"),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM5)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("id", "giant_baby")
                                            .apply(mixins::checkbox_toggle(page.giant_baby.clone(), page.changed.clone(), "Y", "N"))
                                            .future(page.giant_baby.signal_cloned().for_each(clone!(page => move |yes| {
                                                if yes == "Y" {
                                                    page.pb_no.set_neq(String::new());
                                                }
                                                async {}
                                            })))
                                        }),
                                        doms::label_check_for("giant_baby", "เคยคลอดบุตร นน. > 4000 กรัม"),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM2)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("id", "distocia")
                                            .apply(mixins::checkbox_toggle(page.distocia.clone(), page.changed.clone(), "Y", "N"))
                                            .future(page.distocia.signal_cloned().for_each(clone!(page => move |yes| {
                                                if yes == "Y" {
                                                    page.pb_no.set_neq(String::new());
                                                }
                                                async {}
                                            })))
                                        }),
                                        doms::label_check_for("distocia", "มีประวัติคลอดยาก"),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM3_R)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("id", "extraction")
                                            .apply(mixins::checkbox_some(page.extraction.clone(), page.changed.clone().clone()))
                                            .future(page.extraction.signal_cloned().for_each(clone!(page => move |yes| {
                                                if yes.is_some() {
                                                    page.pb_no.set_neq(String::new());
                                                }
                                                async {}
                                            })))
                                        }),
                                        doms::label_check_for("extraction", "มีประวัติคลอดหัตถการ (ระบุ)"),
                                    ])
                                }),
                                html!("div", {
                                    .class("col-sm-5")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "200")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::opt_string_value(page.extraction.clone(), page.changed.clone().clone()))
                                        .apply(mixins::other_none_disable(page.extraction.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM3_R)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("id", "pph")
                                            .apply(mixins::checkbox_toggle(page.pph.clone(), page.changed.clone(), "Y", "N"))
                                            .future(page.pph.signal_cloned().for_each(clone!(page => move |yes| {
                                                if yes == "Y" {
                                                    page.pb_no.set_neq(String::new());
                                                }
                                                async {}
                                            })))
                                        }),
                                        doms::label_check_for("pph", "มีประวัติตกเลือดหลังคลอด"),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM3_R)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("id", "pb_etc")
                                            .apply(mixins::checkbox_some(page.pb_etc.clone(), page.changed.clone()))
                                            .future(page.pb_etc.signal_cloned().for_each(clone!(page => move |yes| {
                                                if yes.is_some() {
                                                    page.pb_no.set_neq(String::new());
                                                }
                                                async {}
                                            })))
                                        }),
                                        doms::label_check_for("pb_etc", "อื่นๆ"),
                                    ])
                                }),
                                html!("div", {
                                    .class("col-sm-5")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "200")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::opt_string_value(page.pb_etc.clone(), page.changed.clone()))
                                        .apply(mixins::other_none_disable(page.pb_etc.clone()))
                                    }))
                                }),
                            ])
                        }),
                        doms::label_row_12("อาการระหว่างตั้งครรภ์"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM2)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("form-check-input")
                                            .attr("id", "a1")
                                            .apply(mixins::radio_opt_match_or_none(page.condition_pregnant.clone(), page.changed.clone(), "ปกติ"))
                                        }),
                                        doms::label_check_for("a1", "ปกติ"),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM2)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("form-check-input")
                                            .attr("id", "a2")
                                            .apply(mixins::radio_opt_match_some_neq(page.condition_pregnant.clone(), page.changed.clone(), "ปกติ"))
                                        }),
                                        doms::label_check_for("a2", "ผิดปกติ (ระบุ)"),
                                    ])
                                }),
                                html!("div", {
                                    .class("col-sm-7")
                                    .child(html!("textarea" => HtmlTextAreaElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("rows", "3")
                                        .apply(mixins::opt_string_match_show_empty(page.condition_pregnant.clone(), page.changed.clone(), "ปกติ"))
                                        .apply(mixins::other_match_none_disable(page.condition_pregnant.clone(), "ปกติ"))
                                    }))
                                }),
                            ])
                        }),
                        doms::label_row_12("ผลตรวจทางห้องปฏิบัติการ"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text(" HIV ")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {
                                                .attr("selected", "")
                                                .attr("value", "")
                                                .text("Choose...")
                                            }),
                                            html!("option", {
                                                .attr("value", "Negative")
                                                .text("Negative")
                                            }),
                                            html!("option", {
                                                .attr("value", "P")
                                                .text("P")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.hiv.clone(), page.changed.clone()))
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {
                                                .attr("selected", "")
                                                .attr("value", "")
                                                .text("Choose...")
                                            }),
                                            html!("option", {
                                                .attr("value", "Negative")
                                                .text("Negative")
                                            }),
                                            html!("option", {
                                                .attr("value", "P")
                                                .text("P")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.hiv2.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text(" VDRL ")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {
                                                .attr("selected", "")
                                                .attr("value", "")
                                                .text("Choose...")
                                            }),
                                            html!("option", {
                                                .attr("value", "Reactive")
                                                .text("Reactive")
                                            }),
                                            html!("option", {
                                                .attr("value", "Non reactive")
                                                .text("Non reactive")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.vdrl.clone(), page.changed.clone()))
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {
                                                .attr("selected", "")
                                                .attr("value", "")
                                                .text("Choose...")
                                            }),
                                            html!("option", {
                                                .attr("value", "Reactive")
                                                .text("Reactive")
                                            }),
                                            html!("option", {
                                                .attr("value", "Non reactive")
                                                .text("Non reactive")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.vdrl2.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text(" HBsAg ")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {
                                                .attr("selected", "")
                                                .attr("value", "")
                                                .text("Choose...")
                                            }),
                                            html!("option", {
                                                .attr("value", "Negative")
                                                .text("Negative")
                                            }),
                                            html!("option", {
                                                .attr("value", "Positive")
                                                .text("Positive")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.hbs_ag.clone(), page.changed.clone()))
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {
                                                .attr("selected", "")
                                                .attr("value", "")
                                                .text("Choose...")
                                            }),
                                            html!("option", {
                                                .attr("value", "Negative")
                                                .text("Negative")
                                            }),
                                            html!("option", {
                                                .attr("value", "Positive")
                                                .text("Positive")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.hbs_ag2.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text(" HCT")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("step", "0.1")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("maxlength", "4")
                                                .attr("aria-describedby", "hct-span")
                                                .apply(mixins::string_value(page.hct.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("hct-span","%"),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("step", "0.1")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("maxlength", "4")
                                                .attr("aria-describedby", "hct2-span")
                                                .apply(mixins::string_value(page.hct2.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("hct2-span","%"),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text(" Blood group ")}))
                                }),
                                html!("div", {
                                    .class("col-sm-4")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "200")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.gr.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text(" ผล thalassemia ตัวเอง ")}))
                                }),
                                html!("div", {
                                    .class("col-sm-4")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "200")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.thalassemia.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text(" ผล thalassemia สามี ")}))
                                }),
                                html!("div", {
                                    .class("col-sm-4")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "200")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.husband.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        doms::label_row_12("ตรวจหน้าท้อง"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Height of fundus")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("maxlength", "2")
                                                .attr("aria-describedby", "hf-span")
                                                .apply(mixins::string_value(page.hf.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_id("hf-span","cm."),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Fetal position")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "200")
                                        .attr("placeholder","เช่น ROA")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.hf_position.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Back of fetus")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_back_fetus.clone(), page.changed.clone(), "lr_back_fetus1", "RUQ"),
                                    doms::label_check_for("lr_back_fetus1","Rt Upper"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_back_fetus.clone(), page.changed.clone(), "lr_back_fetus2", "LUQ"),
                                    doms::label_check_for("lr_back_fetus2","Lt Upper"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_back_fetus.clone(), page.changed.clone(), "lr_back_fetus3", "RLQ"),
                                    doms::label_check_for("lr_back_fetus3","Rt Lower"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_back_fetus.clone(), page.changed.clone(), "lr_back_fetus4", "LLQ"),
                                    doms::label_check_for("lr_back_fetus4","Lt Lower"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Presentation")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_presentation.clone(), page.changed.clone(), "lr_presentation1", "Vertex"),
                                    doms::label_check_for("lr_presentation1","Vertex"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_presentation.clone(), page.changed.clone(), "lr_presentation2", "Breech"),
                                    doms::label_check_for("lr_presentation2","Breech"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_presentation.clone(), page.changed.clone(), "lr_presentation3", "Transverse"),
                                    doms::label_check_for("lr_presentation3","Transverse"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Engagement")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_engagement.clone(), page.changed.clone(), "lr_engagement1", "N"),
                                    doms::label_check_for("lr_engagement1","No"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_engagement.clone(), page.changed.clone(), "lr_engagement2", "Y"),
                                    doms::label_check_for("lr_engagement2","Yes"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Cephalic prominence")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "20")
                                        .attr("placeholder", "เช่น Right, Left")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.lr_prominence.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Fetal attitude")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_attitude.clone(), page.changed.clone(), "lr_attitude1", "Flexion"),
                                    doms::label_check_for("lr_attitude1","Flexion"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_attitude.clone(), page.changed.clone(), "lr_attitude2", "Neutral"),
                                    doms::label_check_for("lr_attitude2","Neutral"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_attitude.clone(), page.changed.clone(), "lr_attitude3", "Deflextion"),
                                    doms::label_check_for("lr_attitude3","Deflextion"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_attitude.clone(), page.changed.clone(), "lr_attitude4", "Extension"),
                                    doms::label_check_for("lr_attitude4","Extension"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Fetal heart rate")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "999")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_fhr.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("/min"),
                                        ])
                                    }))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_fhr_irrigular.clone(), page.changed.clone(), "lr_fhr_irrigular1", "N"),
                                    doms::label_check_for("lr_fhr_irrigular1","Regular"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_fhr_irrigular.clone(), page.changed.clone(), "lr_fhr_irrigular2", "Y"),
                                    doms::label_check_for("lr_fhr_irrigular2","Irregular"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Effective fetal weight")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "99999")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_efw.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("gm."),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Interval")}))
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "lr_int_m")
                                                .apply(mixins::string_value(page.lr_interval_m.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("min"),
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "lr_int_s")
                                                .apply(mixins::string_value(page.lr_interval_s.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("sec"),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Duration")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "99")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_duration.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("sec"),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Intensity")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .attr("maxlength", "20")
                                        .attr("placeholder","เช่น 2+")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.lr_intensity.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        doms::label_row_12("ตรวจเชิงกราน"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Diagonal conjugate")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "99.9")
                                                .attr("step","0.1")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_pelvic_diagonal.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("cm."),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Interspinous diameter")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "99.9")
                                                .attr("step","0.1")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_pelvic_interspinous.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("cm."),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Pelvic Sidewall")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_pelvic_sidewall.clone(), page.changed.clone(), "lr_pelvic_sidewall1", "Diverge"),
                                    doms::label_check_for("lr_pelvic_sidewall1","Diverge"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_pelvic_sidewall.clone(), page.changed.clone(), "lr_pelvic_sidewall2", "Straight"),
                                    doms::label_check_for("lr_pelvic_sidewall2","Straight"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_pelvic_sidewall.clone(), page.changed.clone(), "lr_pelvic_sidewall3", "Converge"),
                                    doms::label_check_for("lr_pelvic_sidewall3","Converge"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Ischeal spine")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_ischeal_spine.clone(), page.changed.clone(), "lr_ischeal_spine1", "Blunt"),
                                    doms::label_check_for("lr_ischeal_spine1","Blunt"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_ischeal_spine.clone(), page.changed.clone(), "lr_ischeal_spine2", "Average"),
                                    doms::label_check_for("lr_ischeal_spine2","Average"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_ischeal_spine.clone(), page.changed.clone(), "lr_ischeal_spine3", "Prominent"),
                                    doms::label_check_for("lr_ischeal_spine3","Prominent"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Sacral curve")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_sacral_curve.clone(), page.changed.clone(), "lr_sacral_curve1", "Concave"),
                                    doms::label_check_for("lr_sacral_curve1","Concave"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_sacral_curve.clone(), page.changed.clone(), "lr_sacral_curve2", "Straight"),
                                    doms::label_check_for("lr_sacral_curve2","Straight"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_sacral_curve.clone(), page.changed.clone(), "lr_sacral_curve3", "Anterior"),
                                    doms::label_check_for("lr_sacral_curve3","Anterior"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Sub pubic angle")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "180")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_pubic_angle.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("degree"),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Pelvic assessment")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_pelvic_ok.clone(), page.changed.clone(), "lr_pelvic_ok1", "Y"),
                                    doms::label_check_for("lr_pelvic_ok1","Adequate"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_pelvic_ok.clone(), page.changed.clone(), "lr_pelvic_ok2", "N"),
                                    doms::label_check_for("lr_pelvic_ok2","Contract"),
                                ])}),
                            ])
                        }),
                        doms::label_row_12("ตรวจภายใน"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Dilatation")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "10")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_cx_dilate.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("cm."),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Effacement")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "100")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_cx_efface.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("%"),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Station")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .attr("min", "-4")
                                        .attr("max", "4")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.lr_cx_station.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Cervix position")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_cx_position.clone(), page.changed.clone(), "lr_cx_position1", "Anterior"),
                                    doms::label_check_for("lr_cx_position1","Anterior"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_cx_position.clone(), page.changed.clone(), "lr_cx_position2", "Middle"),
                                    doms::label_check_for("lr_cx_position2","Middle"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_cx_position.clone(), page.changed.clone(), "lr_cx_position3", "Posterior"),
                                    doms::label_check_for("lr_cx_position3","Posterior"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Cervix consistency")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_cx_consistency.clone(), page.changed.clone(), "lr_cx_consistency1", "Soft"),
                                    doms::label_check_for("lr_cx_consistency1","Soft"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_cx_consistency.clone(), page.changed.clone(), "lr_cx_consistency2", "Medium"),
                                    doms::label_check_for("lr_cx_consistency2","Medium"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_cx_consistency.clone(), page.changed.clone(), "lr_cx_consistency3", "Firm"),
                                    doms::label_check_for("lr_cx_consistency3","Firm"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Bishop score")}))
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "number")
                                                .attr("min", "0")
                                                .attr("max", "13")
                                                .class(class::FORM_CTRL_SM)
                                                .apply(mixins::string_value(page.lr_cx_bishop.clone(), page.changed.clone()))
                                            }),
                                            doms::span_group_text("คะแนน"),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_GRAY)
                                                .child(html!("i", {.class(class::FA_MAGIC)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    let pos = match page.lr_cx_position.lock_ref().as_str() {
                                                        "Anterior" => 2,
                                                        "Middle" => 1,
                                                        _ => 0,
                                                    };
                                                    let cons = match page.lr_cx_consistency.lock_ref().as_str() {
                                                        "Soft" => 2,
                                                        "Medium" => 1,
                                                        _ => 0,
                                                    };
                                                    let eff = match page.lr_cx_efface.lock_ref().parse::<u8>().unwrap_or_default() {
                                                        80.. => 3,
                                                        60..80 => 2,
                                                        40..60 => 1,
                                                        ..40 => 0,
                                                    };
                                                    let dil = match page.lr_cx_dilate.lock_ref().parse::<u8>().unwrap_or_default() {
                                                        5.. => 3,
                                                        3..5 => 2,
                                                        1..3 => 1,
                                                        ..1 => 0,
                                                    };
                                                    let sta = match page.lr_cx_station.lock_ref().parse::<i8>().unwrap_or_default() {
                                                        1.. => 3,
                                                        -1..1 => 2,
                                                        -2 => 1,
                                                        ..-2 => 0,
                                                    };
                                                    page.lr_cx_bishop.set_neq((pos + cons + eff + dil + sta).to_string());
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Cervix assessment")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_cx_ok.clone(), page.changed.clone(), "lr_cx_ok1", "Y"),
                                    doms::label_check_for("lr_cx_ok1","Favorable"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_cx_ok.clone(), page.changed.clone(), "lr_cx_ok2", "N"),
                                    doms::label_check_for("lr_cx_ok2","Unfavorable"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Membrane")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_membrane.clone(), page.changed.clone(), "lr_membrane1", "Intact"),
                                    doms::label_check_for("lr_membrane1","Intact"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM3).children([
                                    doms::radio_container(page.lr_membrane.clone(), page.changed.clone(), "lr_membrane2", "Spontaneous ruptured"),
                                    doms::label_check_for("lr_membrane2","Spontaneous ruptured"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Amniotic fluid color")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_amniotic_color.clone(), page.changed.clone(), "lr_amniotic_color1", "Clear"),
                                    doms::label_check_for("lr_amniotic_color1","Clear"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_amniotic_color.clone(), page.changed.clone(), "lr_amniotic_color2", "Blood"),
                                    doms::label_check_for("lr_amniotic_color2","Blood"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_container(page.lr_amniotic_color.clone(), page.changed.clone(), "lr_amniotic_color3", "Meconium"),
                                    doms::label_check_for("lr_amniotic_color3","Meconium"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("label", {.text("Amniotic fluid odor")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                                    doms::radio_container(page.lr_amniotic_smell.clone(), page.changed.clone(), "lr_amniotic_smell1", "Normal"),
                                    doms::label_check_for("lr_amniotic_smell1","Normal"),
                                ])}),
                                html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                                    doms::radio_container(page.lr_amniotic_smell.clone(), page.changed.clone(), "lr_amniotic_smell2", "Foul"),
                                    doms::label_check_for("lr_amniotic_smell2","Foul smell"),
                                ])}),
                            ])
                        }),
                    ])
                })
            ])
        })
    }

    fn render_ph_mch(page: Rc<Self>) -> Dom {
        html!("div", {
            .visible_signal(page.is_child_signal())
            .children([
                doms::label_row_12("วิธีคลอด"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "s1")
                                    .apply(mixins::radio_opt_match_or_none(page.deliver_anomalies.clone(), page.changed.clone(), "ปกติ"))
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string("", page.deliver_anomalies_means.clone(), page.changed.clone());
                                    }))
                                }),
                                doms::label_check_for("s1", "ปกติ"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "s2")
                                    .apply(mixins::radio_opt_match_some_neq(page.deliver_anomalies.clone(), page.changed.clone(), "ปกติ"))
                                }),
                                doms::label_check_for("s2", " ผิดปกติ (ระบุ)"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-3")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "40")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::opt_string_match_show_empty(page.deliver_anomalies.clone(), page.changed.clone(), "ปกติ"))
                                .apply(mixins::other_match_none_disable(page.deliver_anomalies.clone(), "ปกติ"))
                            }))
                        }),
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .text("เนื่องจาก")
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "50")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.deliver_anomalies_means.clone(), page.changed.clone()))
                                .apply(mixins::other_match_none_disable(page.deliver_anomalies.clone(), "ปกติ"))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .text("คลอดที่")
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "50")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.deliver_location.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .text("น้ำหนักแรกคลอด")
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "number")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.deliver_first_weight.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-2")
                            .text("กรัม")
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .text("สุขภาพแรกเกิด")
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "40")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.deliver_first_health.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                doms::label_row_12("การเลี้ยงทารก"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "d1")
                                    .apply(mixins::checkbox_some(page.fant_breast_feeding_end_age_month.clone(), page.changed.clone()))
                                }),
                                doms::label_check_for("d1", "นมมารดา ถึงอายุ"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "number")
                                .attr("min", "0")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::opt_string_value(page.fant_breast_feeding_end_age_month.clone(), page.changed.clone()))
                                .apply(mixins::other_none_disable(page.fant_breast_feeding_end_age_month.clone()))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-2")
                            .text("เดือน")
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "d2")
                                    .apply(mixins::checkbox_some(page.fant_artificial_feeding_start_age_month.clone(), page.changed.clone()))
                                }),
                                doms::label_check_for("d2", "นมผสม เริ่มอายุ"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "number")
                                .attr("min", "0")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::opt_string_value(page.fant_artificial_feeding_start_age_month.clone(), page.changed.clone()))
                                .apply(mixins::other_none_disable(page.fant_artificial_feeding_start_age_month.clone()))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-2")
                            .text("เดือน")
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "d3")
                                    .apply(mixins::checkbox_some(page.fant_feeding_etc.clone(), page.changed.clone()))
                                }),
                                doms::label_check_for("d3", "อื่นๆ"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "40")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::opt_string_value(page.fant_feeding_etc.clone(), page.changed.clone()))
                                .apply(mixins::other_none_disable(page.fant_feeding_etc.clone()))
                            }))
                        }),
                    ])
                }),
                doms::label_row_12("การให้อาหารเสริม"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "f1")
                                    .apply(mixins::radio_match_empty(page.supplementary_feeding.clone(), page.changed.clone(), "ยังไม่ได้รับ"))
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string("ยังไม่ได้รับ", page.supplementary_feeding_start_age_month.clone(), page.changed.clone());
                                    }))
                                }),
                                doms::label_check_for("f1", "ยังไม่ได้รับ"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-1")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "f2")
                                    .apply(mixins::radio_match(page.supplementary_feeding.clone(), page.changed.clone(), "ได้รับ"))
                                }),
                                doms::label_check_for("f2", "ได้รับ เริ่มอายุ"),
                            ])
                        }),
                        html!("div", {
                            .class("col-sm-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "number")
                                .attr("min", "0")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.supplementary_feeding_start_age_month.clone(), page.changed.clone()))
                                .apply(mixins::other_not_match_disable(page.supplementary_feeding.clone(), "ได้รับ"))
                            }))
                        }),
                        html!("label", {
                            .class("col-sm-2")
                            .text("เดือน")
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_psychiatry(page: Rc<Self>, app: Rc<App>) -> Dom {
        let stage_of_change_select_option = app.app_asset.lock_ref().as_ref().map(|asset| asset.stage_of_change_select_option.clone()).unwrap_or_default();

        html!("div", {
            // .visible_signal(not(page.is_child_signal()))
            .children([
                html!("div", {
                    .class(class::ROW)
                    .child(html!("label", {
                        .class("col-sm-12")
                        .children([
                            html!("B", {.text("ประวัติด้านสุขภาพจิตและสารเสพติด ")}),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_R_BLUE)
                                .child(html!("i", {.class(class::FA_EYE)}))
                                .text(" View")
                                .event(clone!(page => move |_: events::Click| {
                                    page.display_psychiatry.set(!page.display_psychiatry.get());
                                }))
                            })
                        ])
                    }))
                }),
                html!("div", {
                    .visible_signal(page.display_psychiatry.signal_cloned())
                    .class("mt-3")
                    .children([
                        doms::label_row_12("ยาและสารเสพติดที่ใช้ใน 3 เดือนที่ผ่านมา"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-1")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM2)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("form-check-input")
                                            .attr("id", "addict1")
                                            .apply(mixins::radio_match_empty(page.addict.clone(), page.changed.clone(), NOTHING))
                                            .event(clone!(page => move |_:events::Click| {
                                                page.addict_assists.lock_mut().clear();
                                            }))
                                        }),
                                        doms::label_check_for("addict1", NOTHING),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-1")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM2)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("form-check-input")
                                            .attr("id", "addict2")
                                            .apply(mixins::radio_match(page.addict.clone(), page.changed.clone(), SOMETHING))
                                        }),
                                        doms::label_check_for("addict2", "มี (ระบุ)"),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("label", {
                                        .class("text-end")
                                        .child(html!("a", {
                                            .attr("href","#")
                                            .child(html!("i", {.class(class::FA_PLUS_SQ)}))
                                            .text(" ยาและสารเสพติด")
                                            .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                event.prevent_default();
                                                page.addict_assists.lock_mut().push_cloned(AddictAssist::new());
                                                mixins::with_string(SOMETHING, page.addict.clone(), page.changed.clone());
                                            }))
                                        }))
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-7")
                                    .child(html!("label", {
                                        .class("text-end")
                                        .text("ประเมินผลกระทบจากการใช้สารเสพติด V2")
                                    }))
                                }),
                            ])
                        }),
                    ])
                    .children_signal_vec(page.addict_assists.signal_vec_cloned().map(clone!(page => move |assist| {
                        AddictAssist::render(assist, page.clone())
                    })))
                    .children([
                        html!("p"),
                        doms::label_row_12("การใช้สารเสพติดชนิดฉีด"),
                        html!("div", {
                            .class("row")
                            .child(html!("div", {
                                .class(class::FORM_CHK_COL_SM2_OFS1)
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "radio")
                                        .class("form-check-input")
                                        .attr("id", "assist_inj1")
                                        .apply(mixins::radio_match(page.addict_inj.clone(), page.changed.clone(), "N"))
                                        .future(page.addict_inj.signal_cloned().for_each(clone!(page => move |yes| {
                                            if yes == "N" {
                                                page.addict_inj_often.set_neq(String::new());
                                            }
                                            async {}
                                        })))
                                    }),
                                    doms::label_check_for("assist_inj1","ไม่เคย"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("row")
                            .child(html!("div", {
                                .class(class::FORM_CHK_COL_SM2_OFS1)
                                .children([
                                    doms::radio_container(page.addict_inj.clone(), page.changed.clone(), "assist_inj2", "Y"),
                                    doms::label_check_for("assist_inj2","เคย"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {.class("col-sm-2")}),
                                html!("div", {
                                    .class("col-sm-8")
                                    .child(html!("div", {.text("ถ้าเคย, ภายใน 3 เดือนที่ผ่านมา ใช้บ่อยเพียงใด")}))
                                })
                            ])
                        }),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {.class("col-sm-3")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM6)
                                    .children([
                                        doms::radio_disable_by_not_container(
                                            page.addict_inj_often.clone(),
                                            page.addict_inj.clone(), "Y",
                                            page.changed.clone(), "assist_inj_not_often", "N",
                                        ),
                                        doms::label_check_for("assist_inj_not_often","1 ครั้งต่อสัปดาห์ หรือน้อยกว่า 3 วันติดต่อกัน"),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {.class("col-sm-3")}),
                                html!("div", {
                                    .class(class::FORM_CHK_COL_SM6)
                                    .children([
                                        doms::radio_disable_by_not_container(
                                            page.addict_inj_often.clone(),
                                            page.addict_inj.clone(), "Y",
                                            page.changed.clone(), "assist_inj_often", "Y",
                                        ),
                                        doms::label_check_for("assist_inj_often","มากกว่า 1 ครั้งต่อสัปดาห์ หรือมากกว่า 3 วันติดต่อกัน"),
                                    ])
                                }),
                            ])
                        }),
                        html!("p"),
                        doms::label_row_12("การประเมินทางสุขภาพจิต"),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินอาการถอนพิษแอมเฟตามีน (AWQv2)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#amphetamineAwqModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.amphetamine_awq_modal.set(Some(AmphetamineAwqV2::new(
                                                        page.amphetamine_awq.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("span", {
                                                .class("input-group-text")
                                                .text_signal(page.amphetamine_awq.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| str_some(s.to_owned())).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                        .child_signal(page.amphetamine_awq.signal_ref(|concat| concat.split(',').nth(1).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                            html!("div", {
                                                .class("input-group-text")
                                                .style("background-color", match score {
                                                    0 => "inherit",
                                                    1..4 => "gold",
                                                    4..7 => "pink",
                                                    7..10 => "salmon",
                                                    10.. => "crimson",
                                                })
                                                .text("H ")
                                                .text(&score.to_string())
                                                .text(" : ")
                                                .text(match score {
                                                    0 => "ไม่มี Hyperarousal",
                                                    1..4 => "Hyperarousal น้อยมาก",
                                                    4..7 => "Hyperarousal พอควร",
                                                    7..10 => "Hyperarousal มาก",
                                                    10.. => "Hyperarousal มากอย่างยิ่ง",
                                                })
                                            })
                                        })))
                                        .child_signal(page.amphetamine_awq.signal_ref(|concat| concat.split(',').nth(2).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                            html!("div", {
                                                .class("input-group-text")
                                                .style("background-color", match score {
                                                    0 => "inherit",
                                                    1..4 => "gold",
                                                    4..7 => "pink",
                                                    7..10 => "salmon",
                                                    10.. => "crimson",
                                                })
                                                .text("A ")
                                                .text(&score.to_string())
                                                .text(" : ")
                                                .text(match score {
                                                    0 => "ไม่มี Anxiety",
                                                    1..4 => "Anxiety น้อยมาก",
                                                    4..7 => "Anxiety พอควร",
                                                    7..10 => "Anxiety มาก",
                                                    10.. => "Anxiety มากอย่างยิ่ง",
                                                })
                                            })
                                        })))
                                        .child_signal(page.amphetamine_awq.signal_ref(|concat| concat.split(',').nth(3).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                            html!("div", {
                                                .class("input-group-text")
                                                .style("background-color", match score {
                                                    0 => "inherit",
                                                    1..4 => "gold",
                                                    4..7 => "pink",
                                                    7..10 => "salmon",
                                                    10.. => "crimson",
                                                })
                                                .text("R ")
                                                .text(&score.to_string())
                                                .text(" : ")
                                                .text(match score {
                                                    0 => "ไม่มี Reversed Vegetative",
                                                    1..4 => "Reversed Vegetative น้อยมาก",
                                                    4..7 => "Reversed Vegetative พอควร",
                                                    7..10 => "Reversed Vegetative มาก",
                                                    10.. => "Reversed Vegetative มากอย่างยิ่ง",
                                                })
                                            })
                                        })))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินพฤติกรรมก้าวร้าวรุนแรง (OAS)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#aggressionOasModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.aggression_oas_modal.set(Some(AggressionOAS::new(
                                                        page.aggression_oas.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.aggression_oas.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        0 => "inherit",
                                                        1 => "gold",
                                                        2 => "pink",
                                                        3.. => "salmon",
                                                    }
                                                })))
                                                .text_signal(page.aggression_oas.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        0 => "ปกติ",
                                                        1 => "กึ่งเร่งด่วน",
                                                        2 => "เร่งด่วน",
                                                        3.. => "ฉุกเฉิน",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("Motivation scale")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .style("max-width","200px")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(["0","1","2","3","4","5","6","7","8","9","10"].into_iter().map(|s| {
                                            html!("option", {.attr("value", s).text(s)})
                                        }))
                                        .apply(mixins::string_value_select(page.motivation_scale.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("Craving scale")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .style("max-width","200px")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(["0","1","2","3","4","5","6","7","8","9","10"].into_iter().map(|s| {
                                            html!("option", {.attr("value", s).text(s)})
                                        }))
                                        .apply(mixins::string_value_select(page.craving_scale.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("Stage of change")
                                }),
                                html!("div", {
                                    .class("col-sm-2")
                                    .style("max-width","200px")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(stage_of_change_select_option.iter().map(|option| {
                                            html!("option", {.attr("value", &option.key).text(&option.value)})
                                        }))
                                        .apply(mixins::string_value_select(page.stage_of_change_id.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินปัญหาการดื่มสุรา (AUDIT)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#alcoholAuditModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.alcohol_audit_modal.set(Some(AlcoholAudit::new(
                                                        page.alcohol_audit.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.alcohol_audit.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..8 => "inherit",
                                                        8..16 => "gold",
                                                        16..20 => "pink",
                                                        20.. => "salmon",
                                                    }
                                                })))
                                                .text_signal(page.alcohol_audit.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..8 => "Low risk drinker",
                                                        8..16 => "Hazardous drinker",
                                                        16..20 => "Harmful use",
                                                        20.. => "Alcohol dependence",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินอาการถอนพิษสุรา (CIWA-Ar)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#alcoholCiwaModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.alcohol_ciwa_modal.set(Some(AlcoholCiwaAr::new(
                                                        page.alcohol_ciwa.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.alcohol_ciwa.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..8 => "inherit",
                                                        8..15 => "gold",
                                                        15..20 => "pink",
                                                        20.. => "salmon",
                                                    }
                                                })))
                                                .text_signal(page.alcohol_ciwa.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..8 => "Mild withdrawal",
                                                        8..15 => "Moderate withdrawal",
                                                        15..20 => "Severe withdrawal",
                                                        20.. => "Very severe withdrawal",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินอาการขาดสุรา (AWS)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#alcoholAwsModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.alcohol_aws_modal.set(Some(AlcoholAws::new(
                                                        page.alcohol_aws.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.alcohol_aws.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..5 => "inherit",
                                                        5..10 => "gold",
                                                        10..15 => "pink",
                                                        15.. => "salmon",
                                                    }
                                                })))
                                                .text_signal(page.alcohol_aws.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..5 => "Mild withdrawal",
                                                        5..10 => "Moderate withdrawal",
                                                        10..15 => "Severe withdrawal",
                                                        15.. => "Very severe withdrawal",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินระดับการติดนิโคติน (FTND)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#nicotinFtndModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.nicotin_ftnd_modal.set(Some(NicotinFtnd::new(
                                                        page.nicotin_ftnd.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.nicotin_ftnd.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..3 => "inherit",
                                                        3..5 => "gold",
                                                        5.. => "pink",
                                                    }
                                                })))
                                                .text_signal(page.nicotin_ftnd.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..3 => "ระดับต่ำ",
                                                        3..5 => "ระดับปานกลาง",
                                                        5.. => "ระดับสูง",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินภาวะซึมเศร้า 2 คำถาม (2Q)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#depress2QModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.depress_2q_modal.set(Some(Depress2Q::new(
                                                        page.depress_2q.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.depress_2q.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    if score > 0 {
                                                        "gold"
                                                    } else {
                                                        "inherit"
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.depress_2q.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = if score > 0 {
                                                        "ให้ประเมิน 9Q ต่อ"
                                                    } else {
                                                        "ไม่มีภาวะซึมเศร้า"
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินภาวะซึมเศร้า 9 คำถาม (9Q)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#depress9QModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.depress_9q_modal.set(Some(Depress9Q::new(
                                                        page.depress_9q.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.depress_9q.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..7 => "inherit",
                                                        7..13 => "gold",
                                                        13..19 => "pink",
                                                        19.. => "salmon",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.depress_9q.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..7 => "ไม่มีภาวะซึมเศร้า",
                                                        7..13 => "มีภาวะซึมเศร้าระดับน้อย ให้ประเมิน 8Q ต่อ",
                                                        13..19 => "ซึมเศร้าระดับปานกลาง ให้ประเมิน 8Q ต่อ",
                                                        19.. => "ซึมเศร้าระดับรุนแรง ให้ประเมิน 8Q ต่อ",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("คัดกรองภาวะซึมเศร้าในเด็กอายุ 7-17 ปี (CDI)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#depressCdiModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.depress_cdi_modal.set(Some(DepressCdi::new(
                                                        page.depress_cdi.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.depress_cdi.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..16 => "inherit",
                                                        16.. => "pink",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.depress_cdi.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        .. 16 => "ไม่มีภาวะซึมเศร้า",
                                                        16.. => "มีภาวะซึมเศร้า",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินภาวะซึมเศร้าในวัยรุ่น อายุ 15-18 ปี (CES-D)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#depressCesdModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.depress_cesd_modal.set(Some(DepressCesD::new(
                                                        page.depress_cesd.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.depress_cesd.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..23 => "inherit",
                                                        23.. => "pink",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.depress_cesd.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        .. 23 => "ไม่มีภาวะซึมเศร้า",
                                                        23.. => "มีภาวะซึมเศร้า",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินภาวะซึมเศร้าในวัยรุ่น อายุ 11-20 ปี (PHQ-A)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#depressPhqaModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.depress_phqa_modal.set(Some(DepressPhqA::new(
                                                        page.depress_phqa.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.depress_phqa.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..5 => "inherit",
                                                        5..10 => "gold",
                                                        10..15 => "pink",
                                                        15..20 => "salmon",
                                                        20.. => "crimson",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.depress_phqa.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..5 => "ไม่มีภาวะซึมเศร้า",
                                                        5..10 => "ซึมเศร้าเล็กน้อย",
                                                        10..15 => "ซึมเศร้าปานกลาง",
                                                        15..20 => "ซึมเศร้ามาก",
                                                        20.. => "ซึมเศร้ารุนแรง",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                                .text_signal(page.depress_phqa.signal_ref(|concat| concat.split(',').nth(2).and_then(|s| s.parse::<u8>().ok()).map(|is_risk| {
                                                    if is_risk > 0 {
                                                        " ให้ประเมิน 8Q ต่อ"
                                                    } else {
                                                        ""
                                                    }
                                                }).unwrap_or_default()))
                                            }),
                                        ])
                                        .child_signal(page.depress_phqa.signal_ref(|concat| concat.split(',').nth(1).and_then(|s| s.parse::<u8>().ok()).and_then(|is_dx| {
                                            (is_dx > 0).then(|| {
                                                html!("div", {
                                                    .class("input-group-text")
                                                    .text("เข้าเกณฑ์วินิจฉัย")
                                                })
                                            })
                                        })))

                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินแนวโน้มฆ่าตัวตาย 8 คำถาม (8Q)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#suicide8QModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.suicide_8q_modal.set(Some(Suicide8Q::new(
                                                        page.suicide_8q.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.suicide_8q.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        0 => "inherit",
                                                        1..9 => "gold",
                                                        9..17 => "pink",
                                                        17.. => "salmon",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.suicide_8q.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        0 => "ไม่มีแนวโน้มฆ่าตัวตาย",
                                                        1..9 => "มีแนวโน้มฆ่าตัวตายเล็กน้อย",
                                                        9..17 => "มีแนวโน้มฆ่าตัวตายปานกลาง",
                                                        17.. => "มีแนวโน้มฆ่าตัวตายรุนแรง",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินภาวะเครียด (ST-5)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#stressST5Modal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.stress_st5_modal.set(Some(StressST5::new(
                                                        page.stress_st5.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.stress_st5.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..5 => "gold",
                                                        5..8 => "orange",
                                                        8..10 => "pink",
                                                        10.. => "salmon",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.stress_st5.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..5 => "มีความเครียดน้อย",
                                                        5..8 => "มีความเครียดปานกลาง",
                                                        8..10 => "มีความเครียดมาก",
                                                        10.. => "มีความเครียดมากที่สุด",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("PTSD Screening Test")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#ptsdScreenModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.ptsd_screen_modal.set(Some(PtsdScreen::new(
                                                        page.ptsd_screen.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.ptsd_screen.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..4 => "inherit",
                                                        4.. => "pink",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.ptsd_screen.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..4 => "ไม่น่ามี PTSD",
                                                        4.. => "อาจมี PTSD",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินผลกระทบทางจิตใจหลังเกิดเหตุการณ์สะเทือนขวัญ (PISCES-10)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#ptsdPiscesModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.ptsd_pisces_modal.set(Some(PtsdPisces10::new(
                                                        page.ptsd_pisces.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.ptsd_pisces.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..9 => "gold",
                                                        9..14 => "orange",
                                                        14..19 => "pink",
                                                        19.. => "salmon",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.ptsd_pisces.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..9 => "ปกติ",
                                                        9..14 => "มีเล็กน้อย",
                                                        14..19 => "มีมาก",
                                                        19.. => "มีรุนแรง",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("label", {
                                    .class(class::COL_SM5_R)
                                    .text("ประเมินผลกระทบจากเหตุการณ์ภัยพิบัติสำหรับเด็ก (CRIES-13)")
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#ptsdCriesModal")
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.ptsd_cries_modal.set(Some(PtsdCries13::new(
                                                        page.ptsd_cries.clone(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            }),
                                            html!("div", {
                                                .class("input-group-text")
                                                .style_signal("background-color", page.ptsd_cries.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    match score {
                                                        ..25 => "inherit",
                                                        25.. => "pink",
                                                    }
                                                }).unwrap_or("inherit")))
                                                .text_signal(page.ptsd_cries.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                                    let value = match score {
                                                        ..25 => "ไม่มีความเสี่ยง",
                                                        25.. => "มีความเสี่ยง",
                                                    };
                                                    [&score.to_string(), " : ", value].concat()
                                                }).unwrap_or(String::from("รอการประเมิน"))))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "addictAssistModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.addict_assist_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| AddictAssistV2::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "amphetamineAwqModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.amphetamine_awq_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| AmphetamineAwqV2::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "aggressionOasModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.aggression_oas_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| AggressionOAS::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "alcoholAuditModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.alcohol_audit_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| AlcoholAudit::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "alcoholCiwaModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.alcohol_ciwa_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| AlcoholCiwaAr::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "alcoholAwsModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.alcohol_aws_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| AlcoholAws::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "nicotinFtndModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.nicotin_ftnd_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| NicotinFtnd::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "depress2QModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.depress_2q_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| Depress2Q::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "depress9QModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.depress_9q_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| Depress9Q::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "depressCdiModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.depress_cdi_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| DepressCdi::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "depressCesdModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.depress_cesd_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| DepressCesD::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "depressPhqaModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.depress_phqa_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| DepressPhqA::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "suicide8QModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.suicide_8q_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| Suicide8Q::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "stressST5Modal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.stress_st5_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| StressST5::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "ptsdScreenModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.ptsd_screen_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| PtsdScreen::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "ptsdPiscesModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.ptsd_pisces_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| PtsdPisces10::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "ptsdCriesModal")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.ptsd_cries_modal.signal_cloned().map(|opt| {
                                opt.as_ref().map(|modal| PtsdCries13::render(modal.clone())).or(Some(blank_modal()))
                            }))
                        }),
                    ])
                })
            ])
        })
    }

    fn render_admit_hx(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("การเข้ารับการรักษาในโรงพยาบาล"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-2")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM2)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "h1")
                                    .attr("value", "ไม่เคย")
                                    .with_node!(element => {
                                        .future(map_ref!{
                                            let id = page.admission_note_id.signal_cloned(),
                                            let hx = page.inpatient_history.signal_cloned(),
                                            let old = page.old_regdatetime.signal_cloned() =>
                                            ((id.is_some() && hx == "ไม่เคย") || (id.is_none() && old.is_none()), old.is_some())
                                        }.for_each(clone!(element => move |(never, has_old)| {
                                            if never {
                                                element.set_checked(true);
                                            } else {
                                                element.set_checked(false);
                                            }
                                            if has_old {
                                                element.set_disabled(true);
                                            } else {
                                                element.set_disabled(false);
                                            }
                                            async {}
                                        })))
                                        .event(clone!(page => move |_: events::Click| {
                                            mixins::with_string(&element.value(), page.inpatient_history.clone(), page.changed.clone());
                                        }))
                                    })
                                }),
                                doms::label_check_for("h1", "ไม่เคย"),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-2")}),
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM1)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "radio")
                                    .class("form-check-input")
                                    .attr("id", "h2")
                                    .attr("value", "เคย")
                                    .with_node!(element => {
                                        .future(map_ref!{
                                            let id = page.admission_note_id.signal_cloned(),
                                            let hx = page.inpatient_history.signal_cloned(),
                                            let old = page.old_regdatetime.signal_cloned() =>
                                            (id.is_some() && hx == "เคย") || (id.is_none() && old.is_some())
                                        }.for_each(clone!(element => move |has| {
                                            if has {
                                                element.set_checked(true);
                                            } else {
                                                element.set_checked(false);
                                            }
                                            async {}
                                        })))
                                        .event(clone!(page => move |_: events::Click| {
                                            mixins::with_string(&element.value(), page.inpatient_history.clone(), page.changed.clone());
                                        }))
                                    })
                                }),
                                doms::label_check_for("h2", "เคย"),
                            ])
                        }),
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .text("ครั้งสุดท้ายเมื่อ")
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(doms::datetime_picker(
                                page.inpatient_last_date.clone(),
                                page.changed.clone(), always(false),
                                |d| d.style("min-width","175px"),
                                |d| d.class("form-control-sm"),
                                |d| d.class("form-control-sm"),
                                |s| s, always(None),
                            ))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM5_R)
                            .text("รพ.")
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.inpatient_location.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM5_R)
                            .text("เนื่องจาก")
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.inpatient_because.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_ros(page: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::CARD_GROUP_T)
            .child(html!("div", {
                .class("card")
                .child(html!("div", {
                    .class("card-body")
                    .style("overflow-y","auto")
                    .children([
                        html!("div", {
                            .class(class::ALERT_GREEN)
                            .class(class::COL_SM12_C)
                            .attr("role", "alert")
                            .text("Review of Systems")
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .child(html!("div", {
                                .class("col-sm-12")
                                .child(html!("div", {
                                    .class(class::CARD_BGREEN)
                                    .child(html!("div", {
                                        .class("card-body")
                                        .children([
                                            html!("div", {
                                                .class(class::ROW)
                                                .children([
                                                    html!("div", {.class("col-sm-10")}),
                                                    html!("div", {
                                                        .class("col-md-2")
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_SM_L_GRAY)
                                                            .text("ปฎิเสธทั้งหมด")
                                                            .event(clone!(page => move |_: events::Click| {
                                                                mixins::with_string(NOTHING, page.ros_eent.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_neuro.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_lung.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_tb.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_ht.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_heart.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_liver.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_gi.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_endocrine.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_kidney.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_tumour.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_hemato.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_rheumato.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_psychia.clone(), page.changed.clone());
                                                                mixins::with_string(NOTHING, page.ros_other.clone(), page.changed.clone());
                                                            }))
                                                        }))
                                                    }),
                                                ])
                                            }),
                                            render_ros_item("โรคเกี่ยวกับ ตา หู คอ จมูก", page.ros_eent.clone(), page.changed.clone()),
                                            render_ros_item("โรคระบบประสาท แขนขาอ่อนแรง", page.ros_neuro.clone(), page.changed.clone()),
                                            render_ros_item("โรคปอด ถุงลมโป่งพอง หอบหืด", page.ros_lung.clone(), page.changed.clone()),
                                            render_ros_item("วัณโรค", page.ros_tb.clone(), page.changed.clone()),
                                            render_ros_item("ความดันโลหิตสูง", page.ros_ht.clone(), page.changed.clone()),
                                            render_ros_item("หัวใจผิดปกติ", page.ros_heart.clone(), page.changed.clone()),
                                            render_ros_item("โรคตับและทางเดินน้ำดี", page.ros_liver.clone(), page.changed.clone()),
                                            render_ros_item("โรคกระเพาะอาหาร ลำไส้", page.ros_gi.clone(), page.changed.clone()),
                                            render_ros_item("คอพอก เบาหวาน", page.ros_endocrine.clone(), page.changed.clone()),
                                            render_ros_item("โรคไต นิ่วไต ไตวาย", page.ros_kidney.clone(), page.changed.clone()),
                                            render_ros_item("เนื้องอก มะเร็ง", page.ros_tumour.clone(), page.changed.clone()),
                                            render_ros_item("ความผิดปกติของเม็ดเลือด", page.ros_hemato.clone(), page.changed.clone()),
                                            render_ros_item("โรคข้อ เกาต์", page.ros_rheumato.clone(), page.changed.clone()),
                                            render_ros_item("โรคทางจิตเวช", page.ros_psychia.clone(), page.changed.clone()),
                                            render_ros_item("โรคอื่นๆ", page.ros_other.clone(), page.changed.clone()),
                                        ])
                                    }))
                                }))
                            }))
                        }),
                    ])
                }))
            }))
        })
    }

    fn render_pe(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_GROUP_T)
            .child(html!("div", {
                .class("card")
                .child(html!("div", {
                    .class("card-body")
                    .style("overflow-y","auto")
                    .children([
                        html!("div", {
                            .class(class::ALERT_GREEN)
                            .class(class::COL_SM12_C)
                            .attr("role", "alert")
                            .text("Physical examination")
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .child(html!("div", {
                                .class("col-sm-12")
                                .child(html!("div", {
                                    .class(class::CARD_BGREEN)
                                    .child(Self::render_pe_items(page.clone()))
                                }))
                            }))
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .child(html!("div", {
                                .class("col-sm-12")
                                .child(html!("div", {
                                    .class(class::CARD_BGREEN)
                                    .child(html!("div", {
                                        .class("card-body")
                                        .children([
                                            html!("h5", {.class("card-title")}),
                                            html!("div", {
                                                .class(class::FLEX_JCC)
                                                .child(html!("div", {
                                                    .class(class::ROUND_WHITE)
                                                    .style("border","2px dotted black")
                                                    .style("background-position","center")
                                                    .style("background-repeat","no-repeat")
                                                    .style("background-size","cover")
                                                    .style("position","relative")
                                                    .style("width","800px")
                                                    .style("height","600px")
                                                    .style_signal("background-image", page.all_body_image.signal_cloned().map(|image| svg_to_data_url(image.as_ref())))
                                                    .child(html!("canvas", {
                                                        .attr("id", "body_full")
                                                        .attr("width", "800")
                                                        .attr("height", "600")
                                                        .style("padding-left","0")
                                                        .style("padding-right","0")
                                                        .style("margin-left","auto")
                                                        .style("margin-right","auto")
                                                        .style("display","block")
                                                    }))
                                                }))
                                            }),
                                            html!("p"),
                                            html!("div", {
                                                .class(class::COL_SM11_OFSM1)
                                                .child(html!("div", {
                                                    .class(class::FLEX_JCC)
                                                    .children([
                                                        html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_L_REDO)
                                                            .child(html!("i", {.class(class::FA_L_ARROW_CIRCLE)}))
                                                            .text(" Undo")
                                                            .event(clone!(page => move |_action_row_div: events::Click| {
                                                                if let Some(canvas) = page.canvas.lock_ref().as_ref() {
                                                                    let mut objects = canvas.get_objects();
                                                                    if let Some(obj) = objects.pop() {
                                                                        canvas.remove(&obj);
                                                                        page.h.lock_mut().push_cloned(obj.into());
                                                                        canvas.render_all();
                                                                        page.changed.set_neq(true);
                                                                    }
                                                                }
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_L_REDO)
                                                            .child(html!("i", {.class(class::FA_R_ARROW_CIRCLE)}))
                                                            .text(" Redo")
                                                            .event(clone!(page => move |_: events::Click| {
                                                                if let Some(canvas) = page.canvas.lock_ref().as_ref() {
                                                                    if let Some(obj) = page.h.lock_mut().pop() {
                                                                        page.redoing.set_neq(true);
                                                                        canvas.add(&obj);
                                                                        page.changed.set_neq(true);
                                                                    }
                                                                }
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_L_REDO)
                                                            .child(html!("i", {.class(class::FA_ERASER)}))
                                                            .text(" Clear")
                                                            .event(clone!(page => move |_: events::Click| {
                                                                if let Some(canvas) = page.canvas.lock_ref().as_ref() {
                                                                    canvas.clear();
                                                                    page.changed.set_neq(true);
                                                                }
                                                            }))
                                                        }),
                                                        html!("input" => HtmlInputElement, {
                                                            .attr("type", "color")
                                                            .class("me-1")
                                                            .attr("list","color-list")
                                                            .prop_signal("value", page.pen_color.signal_cloned())
                                                            .with_node!(element => {
                                                                .event(clone!(page => move |_: events::Change| {
                                                                    if let Some(canvas) = page.canvas.lock_ref().as_ref() {
                                                                        let brush = canvas.get_brush();
                                                                        brush.set_color(&element.value());
                                                                    }
                                                                }))
                                                            })
                                                        }),
                                                        html!("input" => HtmlInputElement, {
                                                            .attr("type", "range")
                                                            .class("form-range")
                                                            .attr("min", "1")
                                                            .attr("max", "10")
                                                            .attr("step", "1")
                                                            .style("width","100px")
                                                            .prop_signal("value", page.pen_width.signal_cloned())
                                                            .with_node!(element => {
                                                                .event(clone!(page => move |_: events::Change| {
                                                                    if let Some(canvas) = page.canvas.lock_ref().as_ref() {
                                                                        let brush = canvas.get_brush();
                                                                        brush.set_width(element.value().parse::<u32>().unwrap_or(1));
                                                                    }
                                                                }))
                                                            })
                                                        }),
                                                        doms::color_picker(),
                                                    ])
                                                }))
                                            }),
                                        ])
                                    }))
                                }))
                            }))
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child_signal(page.admission_note_id.signal_cloned().map(clone!(app, page => move |opt| {
                                match opt {
                                    Some(admission_note_id) => {
                                        let is_signed_by_me = page.admission_note_doctors.lock_ref().iter().any(|d| {
                                            d.admission_note_doctor.as_ref().map(|doctor_code| doctor_code == &app.doctor_code().unwrap_or_default()).unwrap_or_default()
                                        });
                                        Some(ImageCpn::render("300px", ImageCpn::new_with_key(
                                            ImageUsage::IpdDrAdmissionNote,
                                            admission_note_id,
                                            is_signed_by_me,
                                            page.patient.lock_ref().patient.clone(),
                                            str_some(page.an.get_cloned()),
                                            "", // will use ImageUsage internally, so we add nothing here
                                        ), app.clone()))
                                    }
                                    None => {
                                        let an = page.an.get_cloned();
                                        let is_pre_admit = app.is_pre_admit(&an);
                                        app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, is_pre_admit).then(|| {
                                            // POST `EndPoint::ImageUsage`
                                            ImageCpn::render("300px", ImageCpn::new_returning(
                                                page.image_callback.clone(),
                                                page.patient.lock_ref().patient.clone(),
                                                str_some(an),
                                                "IPD-ADMISSION-NOTE-DOCTOR",
                                            ), app.clone())
                                        })
                                    }
                                }
                            })))
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .child(html!("div", {
                                .class("col-sm-12")
                                .child(html!("div", {
                                    .class("card")
                                    .child(html!("div", {
                                        .class("card-body")
                                        .child(html!("div", {
                                            .class(class::ROW)
                                            .child(html!("div", {
                                                .class("col-sm-11")
                                                .children([
                                                    html!("div", {
                                                        .class(class::ROW)
                                                        .children([
                                                            html!("label", {
                                                                .class(class::COL_SM3_R)
                                                                .text("Impression")
                                                            }),
                                                            html!("div", {
                                                                .class("col-sm-9")
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type", "text")
                                                                    .attr("maxlength", "150")
                                                                    .class(class::FORM_CTRL_SM)
                                                                    .apply(mixins::string_value(page.impression.clone(), page.changed.clone()))
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                    html!("div", {
                                                        .class(class::ROW)
                                                        .children([
                                                            html!("label", {
                                                                .class(class::COL_SM3_R)
                                                                .text("Diff. Dx")
                                                            }),
                                                            html!("div", {
                                                                .class("col-sm-9")
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type", "text")
                                                                    .attr("maxlength", "150")
                                                                    .class(class::FORM_CTRL_SM)
                                                                    .apply(mixins::string_value(page.diff_dx.clone(), page.changed.clone()))
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                    html!("div", {
                                                        .class(class::ROW)
                                                        .children([
                                                            html!("label", {
                                                                .class(class::COL_SM3_R)
                                                                .text("Plan Management")
                                                            }),
                                                            html!("div", {
                                                                .class("col-sm-9")
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type", "text")
                                                                    .attr("maxlength", "150")
                                                                    .class(class::FORM_CTRL_SM)
                                                                    .apply(mixins::string_value(page.plan_management.clone(), page.changed.clone()))
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                ])
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }),
                        Self::render_signing(page.clone(), app.clone()),
                        Self::render_export(page.clone(), app.clone()),
                    ])
                }))
            }))
        })
    }

    fn render_pe_items(page: Rc<Self>) -> Dom {
        html!("div", {
            .class("card-body")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-9")}),
                        html!("div", {
                            .class("col-sm-3")
                            .text("\u{00a0}\u{00a0}")
                            .child(html!("i", {.class(class::FA_USER)}))
                            .text(" ผู้ใหญ่\u{00a0}\u{00a0}\u{00a0}\u{00a0}\u{00a0}\u{00a0}")
                            .child(html!("i", {.class(class::FA_BABY)}))
                            .text(" เด็ก")
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {.class("col-sm-9")}),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal All")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_GENERAL, page.pe_general.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_SKIN, page.pe_skin.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_HEENT, page.pe_heent.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_NECK, page.pe_neck.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_THORAX, page.pe_breastthorax.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_HEART, page.pe_heart.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_LUNG, page.pe_lungs.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_ABDOMEN, page.pe_abdomen.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_GENITALIA, page.pe_rectalgenitalia.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_EXT, page.pe_extremities.clone(), page.changed.clone());
                                        mixins::with_string(PE_ADULT_NEURO, page.pe_neurological.clone(), page.changed.clone());
                                        if page.is_female() {
                                            mixins::with_string(PE_ADULT_OBGYN, page.pe_ob_gynexam.clone(), page.changed.clone());
                                        }
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal All")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_GENERAL, page.pe_general.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_SKIN, page.pe_skin.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_HEENT, page.pe_heent.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_NECK, page.pe_neck.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_THORAX, page.pe_breastthorax.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_HEART, page.pe_heart.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_LUNG, page.pe_lungs.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_ABDOMEN, page.pe_abdomen.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_GENITALIA, page.pe_rectalgenitalia.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_EXT, page.pe_extremities.clone(), page.changed.clone());
                                        mixins::with_string(PE_PED_NEURO, page.pe_neurological.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("General")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_general.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_GENERAL, page.pe_general.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_GENERAL, page.pe_general.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Skin")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_skin.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_SKIN, page.pe_skin.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_SKIN, page.pe_skin.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("HEENT")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_heent.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_HEENT, page.pe_heent.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_HEENT, page.pe_heent.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Neck")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_neck.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_NECK, page.pe_neck.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_NECK, page.pe_neck.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Breast & Thorax")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_breastthorax.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_THORAX, page.pe_breastthorax.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_THORAX, page.pe_breastthorax.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Heart")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_heart.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_HEART, page.pe_heart.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_HEART, page.pe_heart.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Lungs")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_lungs.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_LUNG, page.pe_lungs.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_LUNG, page.pe_lungs.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Abdomen")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_abdomen.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_ABDOMEN, page.pe_abdomen.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_ABDOMEN, page.pe_abdomen.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Rectum & Genitalia")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_rectalgenitalia.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_GENITALIA, page.pe_rectalgenitalia.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_GENITALIA, page.pe_rectalgenitalia.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Extremities")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_extremities.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_EXT, page.pe_extremities.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_EXT, page.pe_extremities.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Neuro")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_neurological.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-md-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_NEURO, page.pe_neurological.clone(), page.changed.clone());
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_BABY)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_PED_NEURO, page.pe_neurological.clone(), page.changed.clone());
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
            ])
            .child_signal(page.is_female_signal().map(clone!(page => move |is_female| {
                is_female.then(|| {
                    html!("div", {
                        .class(class::ROW)
                        .children([
                            html!("label", {
                                .class(class::COL_SM3_R)
                                .text("OB/Gyn exam")
                            }),
                            html!("div", {
                                .class("col-sm-6")
                                .child(html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .attr("maxlength", "200")
                                    .class(class::FORM_CTRL_SM)
                                    .apply(mixins::string_value(page.pe_ob_gynexam.clone(), page.changed.clone()))
                                }))
                            }),
                            html!("div", {
                                .class("col-md-3")
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_GRAY)
                                    .child(html!("i", {.class(class::FA_USER)}))
                                    .text(" Normal")
                                    .event(clone!(page => move |_: events::Click| {
                                        mixins::with_string(PE_ADULT_OBGYN, page.pe_ob_gynexam.clone(), page.changed.clone());
                                    }))
                                }))
                            }),
                        ])
                    })
                })
            })))
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("Other")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .attr("maxlength", "200")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.pe_other.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .text("PE Text")
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("rows", "6")
                                .apply(mixins::string_value(page.pe_text.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_signing(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("row")
            .children([
                html!("div", {
                    .class("col-md-4")
                    .child(html!("div", {
                        .class("mb-3")
                        .child(html!("label", {
                            //.attr("for", "action-person-dr-admission")
                            .class("my-3")
                            .text("ลงชื่อแพทย์")
                        }))
                        .apply_if(app.has_permission(Permission::DataTypeDoctorUse), |dom| {
                            dom.child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_R_GRAY)
                                .child(html!("i", {.class(class::FA_PLUS)}))
                                .text(" ลงชื่อ")
                                .event(clone!(app, page => move |_: events::Click| {
                                    if let Some(user) = app.user.lock_ref().as_ref() {
                                        let code = user.user.doctorcode.get_cloned();
                                        if !page.admission_note_doctors.lock_ref().iter().any(|d| d.admission_note_doctor.as_ref() == Some(&code)) {
                                            let doctor = Rc::new(AdmissionNoteDoctor {
                                                admission_note_item_id: 0,
                                                admission_note_doctor: Some(code),
                                                admission_note_doctorname: Some(user.user.name.get_cloned()),
                                                licenseno: match user.user.licenseno.lock_ref().as_str() {
                                                    "-99999"|"" => None,
                                                    s => Some(s.to_owned()),
                                                },
                                                entryposition: str_some(user.user.entryposition.get_cloned()),
                                            });
                                            page.admission_note_doctors.lock_mut().push_cloned(doctor);
                                            page.changed.set_neq(true);
                                        }
                                    }
                                }))
                            }))
                        })
                        .child(html!("div", {
                            .children_signal_vec(page.admission_note_doctors.signal_vec_cloned().map(|doctor| {
                                render_doctor(doctor)
                            }))
                        }))
                    }))
                }),
                html!("div", {
                    .class("col-md-7")
                    .child(html!("div", {
                        .class("mb-3")
                        .children([
                            html!("label", {
                                .class("my-3")
                                //.attr("for", "action-person-nurse")
                                .text("ลงชื่อพยาบาล")
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP)
                                .children([
                                    html!("input", {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .attr("readonly", "")
                                        .prop_signal("value", page.nurse_name.signal_cloned())
                                    }),
                                    html!("input", {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .attr("readonly", "")
                                        .prop_signal("value", page.nurse_pos.signal_cloned())
                                    }),
                                    html!("input", {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .attr("readonly", "")
                                        .prop_signal("value", page.nurse_licenseno.signal_cloned())
                                    }),
                                ])
                                .apply_if(app.has_permission(Permission::DataTypeNurseUse), |dom| {
                                    dom.child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_GRAY)
                                        .text("ลงชื่อ")
                                        .event(clone!(app, page => move |_: events::Click| {
                                            if let Some(user) = app.user.lock_ref().as_ref() {
                                                page.nurse_name.set_neq(user.user.name.get_cloned());
                                                page.nurse_pos.set_neq(user.user.entryposition.get_cloned());
                                                page.nurse_licenseno.set_neq(user.user.licenseno.get_cloned());
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                    }))
                                })
                            }),
                        ])
                    }))
                }),
            ])
        })
    }

    fn render_export(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::ROW)
            .child(html!("div", {
                .class(class::COL_SM12_R)
                .children([
                    html!("div", {
                        .class("float-start")
                        .children(PdfButtons::buttons(
                            PdfButtons::new(
                                TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteDr, &app.state().report_coercions()),
                                page.an.clone(),
                                page.admission_note_id.clone(),
                                page.changed.clone(),
                                clone!(page => move || {
                                    let patient = page.patient.lock_ref().patient.get_cloned();
                                    serde_json::json!({
                                        "id": page.an.get_cloned(),
                                        "patient": patient.clone(),
                                        "raw":  page.raw.get_cloned(),
                                        "patient_image": patient.as_ref().map(|pt| pt.image()),
                                    }).to_string()
                                })
                            ), "Print PDF", None, app.clone()
                        ))
                    }),
                    html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_L_BLUE)
                        .child(html!("i", {.class(class::FA_L_ARROW)}))
                        .text(" กลับ")
                        .event(clone!(app => move |_: events::Click| {
                            if app.go_back_else() {
                                for route in [Route::IpdSearchPatientDr, Route::IpdSearchPatientNurse, Route::IpdSearchPatientPharmacist, Route::IpdSearchPatientOther, Route::Info] {
                                    if route.has_permission(app.state()) {
                                        route.hard_redirect();
                                        break;
                                    }
                                }
                            }
                        }))
                    }),
                ])
                .child_signal(page.admission_note_id.signal_cloned().map(clone!(app, page => move |opt| {
                    let is_pre_admit = app.is_pre_admit(&page.an.lock_ref());
                    let ready = if opt.is_some() {
                        app.endpoint_is_allow(&Method::PUT, &EndPoint::IpdAdmissionNoteDr, is_pre_admit)
                    } else {
                        app.endpoint_is_allow(&Method::POST, &EndPoint::IpdAdmissionNoteDr, is_pre_admit)
                    };
                    ready.then(|| {
                        html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_R)
                            .class_signal("btn-primary", page.changed.signal())
                            .class_signal("btn-secondary", not(page.changed.signal()))
                            .text("บันทึก")
                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                Self::submit(page.clone(), app.clone());
                            }), not(page.changed.signal()), app.state()))
                        })
                    })
                })))
            }))
        })
    }

    fn submit(page: Rc<Self>, app: Rc<App>) {
        if let Some(note) = Self::finalized(page.clone(), app.clone()) {
            app.async_load(
                true,
                clone!(app => async move {
                    let method = if page.admission_note_id.get().is_some() {"PUT"} else {"POST"};
                    // update admission note in raw data
                    let saver = IpdAdmissionNoteDrSave {
                        admission_note: note.clone(),
                        admission_note_doctors: page.admission_note_doctors.lock_ref().iter().filter_map(|doctor| doctor.as_ref().admission_note_doctor.clone()).collect()
                    };
                    // POST `EndPoint::IpdAdmissionNoteDr`
                    // PUT `EndPoint::IpdAdmissionNoteDr`
                    match IpdAdmissionNoteDrSave::call_api_save(&saver, method, app.state()).await {
                        Ok(responses) => {
                            let admission_note_id = responses.first().map(|res| res.last_insert_id as u32).unwrap_or_default();
                            app.alert_execute_responses(&responses, clone!(app => async move {
                                if admission_note_id > 0 {
                                    // update images
                                    if method == "POST" {
                                        let is_pre_admit = app.is_pre_admit(&note.an);
                                        if app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, is_pre_admit) {
                                            // POST `EndPoint::ImageUsage`
                                            page.image_callback.lock_ref().post_images(ImageUsage::IpdDrAdmissionNote, admission_note_id, app.clone()).await;
                                        }
                                    }
                                    page.admission_note_id.set_neq(Some(admission_note_id));
                                }
                                let raw = page.raw.lock_ref();
                                if let Ok(mut guard) = raw.lock() {
                                    guard.admission_note = Some(note);
                                }
                                page.changed.set_neq(false);
                                if method == "POST" && page.is_set_this_hospital.get() {
                                    let route = Route::IpdMain {
                                        view_by: String::from("doctor"),
                                        an: page.an.get_cloned(),
                                        tab: Tab::MedReconcile.str().to_owned(),
                                        sub: String::new(),
                                        id: 0,
                                    };
                                    if route.has_permission(app.state()) {
                                        if app.confirm(&[
                                            "เนื่องจากมีการบันทึกข้อมูลโรคประจำตัวที่มีสถานพยาบาลที่รักษาเป็น",
                                            &app.app_status.lock_ref().as_ref().map(|status| status.hospital_name.clone()).unwrap_or_default(),
                                            " ท่านต้องการบันทึก Med Reconciliation ด้วยหรือไม่ ?"
                                        ].concat()).await {
                                            route.hard_redirect();
                                        }
                                    }
                                }
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn finalized(page: Rc<Self>, app: Rc<App>) -> Option<IpdDrAdmissionNote> {
        let concat_with_space = app.concat_with_space();

        page.hn.get_cloned().or(str_some(page.patient.lock_ref().hn.get_cloned())).map(|hn| {
            IpdDrAdmissionNote {
                hn,
                an: page.an.get_cloned(),
                receiver_medication_date: date_8601(&page.receiver_medication_date.lock_ref()),
                receiver_medication_time: time_8601(&page.receiver_medication_time.lock_ref()),

                take_medication_by: Some(page.take_medication_by.lock_ref().string()),
                arrive_by: Some(page.arrive_by.lock_ref().string()),
                taken_by_relative: str_some(page.taken_by_relative.get_cloned()),
                taken_by_nurse: str_some(page.taken_by_nurse.get_cloned()),
                taken_by_crib: str_some(page.taken_by_crib.get_cloned()),
                taken_by_etc: str_some(page.taken_by_etc.get_cloned()),
                taken_by: str_some(page.taken_by.get_cloned()),
                informant_patient: page.informant_patient.get_cloned().or(Some(String::from("ผู้ป่วย"))),
                informant_relatives: page.informant_relatives.get_cloned(),
                informant_deliverer: page.informant_deliverer.get_cloned(),
                informant_etc: page.informant_etc.get_cloned(),
                chief_complaints: str_some(page.chief_complaints.get_cloned()),
                medical_history: str_some(page.medical_history.get_cloned()),

                bp: str_some(page.bp.get_cloned()),
                t: Decimal::from_str_exact(&page.t.lock_ref()).ok(), // decimal(3,1) unsigned
                pr: page.pr.lock_ref().parse::<u32>().ok(),
                rr: page.rr.lock_ref().parse::<i32>().ok(),
                gcs: opt_zero_none(page.gcs.lock_ref().parse::<i32>().ok()),
                e: zero_str_none(page.e.get_cloned()),
                v: zero_str_none(page.v.get_cloned()),
                m: zero_str_none(page.m.get_cloned()),
                braden_scale: str_some(page.braden_scale.get_cloned()),

                disease: str_some(page.disease.get_cloned()),                                           //.or(Some(String::from("ไม่มี"))),
                disease_detail: str_some(concat_mutable_vec(&page.disease_details, concat_with_space)), // disease_name + ' ' + disease_year + ' ' + disease_hospital
                disease_etc: page.disease_etc.get_cloned(),                                             // unused
                last_dose_taken_time: datetime_8601(&page.last_dose_taken_time.lock_ref()),
                last_dose_taken_remark: opt_empty_none(page.last_dose_taken_remark.get_cloned()),
                operation_history: opt_empty_none(page.operation_history.get_cloned()), //.or(Some(String::from("ไม่มี"))),
                allergy_history: str_some(page.allergy_history.get_cloned()),           //.or(Some(String::from("ไม่มี"))),
                allergy_drug_history: str_some(concat_mutable_vec(&page.allergy_drugs, concat_with_space)), // agent + ' ' + symptom + ' ' + agent + ..
                allergy_drug_history_hosxp: str_some(page.allergy_drug_history_hosxp.get_cloned()),
                allergy_drug_pharmacy_check_person: page.allergy_drug_pharmacy_check_person.get_cloned(), // unused
                allergy_drug_pharmacy_check_datetime: page.allergy_drug_pharmacy_check_datetime.get_cloned(), // unused
                allergy_food_history: str_some(concat_mutable_vec(&page.allergy_foods, concat_with_space)), // agent + ' ' + symptom + ' ' + agent + ..
                allergy_etc_history: str_some(concat_mutable_vec(&page.allergy_etcs, concat_with_space)), // agent + ' ' + symptom + ' ' + agent + ..
                allergy_detail: page.allergy_detail.get_cloned(),                                         // unused
                family_medical_history: str_some(page.family_medical_history.get_cloned()),               //.or(Some(String::from("ไม่มี"))),
                family_medical_history_detail: str_some(concat_mutable_vec(&page.family_medicals, concat_with_space)), // disease + ' ' + relation + ' ' + disease + ..

                receives_immunisation_history_kid: opt_empty_none(page.receives_immunisation_history_kid.get_cloned()).or(Some(String::from("ครบตามวัย"))),
                developmentally_kid: opt_empty_none(page.developmentally_kid.get_cloned()), //.or(Some(String::from("ปกติ"))),
                g: page.g.lock_ref().parse::<i32>().ok(),                                   //.or(Some(0)),
                p: str_some(page.p.get_cloned()),                                           //.or(Some(String::from("0"))),
                anc: str_some(page.anc.get_cloned()),
                tt: opt_zero_none(page.tt.lock_ref().parse::<i32>().ok()),
                gestational_age: zero_str_none(page.gestational_age.get_cloned()),
                gestational_day: zero_str_none(page.gestational_day.get_cloned()),
                last_child: page.last_child.lock_ref().parse::<i32>().ok(),
                last_abort: str_some(page.last_abort.get_cloned()),
                curette: str_some(page.curette.get_cloned()),
                lmp: date_8601(&page.lmp.lock_ref()),
                edc: date_8601(&page.edc.lock_ref()),
                pb_no: str_some(page.pb_no.get_cloned()),
                giant_baby: str_some(page.giant_baby.get_cloned()),
                distocia: str_some(page.distocia.get_cloned()),
                extraction: page.extraction.get_cloned(),
                pph: str_some(page.pph.get_cloned()),
                pb_etc: page.pb_etc.get_cloned(),
                hf: page.hf.lock_ref().parse::<i32>().ok(),
                hf_position: str_some(page.hf_position.get_cloned()),
                mem_ruptured_hours: page.mem_ruptured_hours.lock_ref().as_ref().and_then(|s| s.parse::<u16>().ok()),

                lr_back_fetus: str_some(page.lr_back_fetus.get_cloned()),
                lr_presentation: str_some(page.lr_presentation.get_cloned()),
                lr_engagement: str_some(page.lr_engagement.get_cloned()),
                lr_prominence: str_some(page.lr_prominence.get_cloned()),
                lr_attitude: str_some(page.lr_attitude.get_cloned()),
                lr_fhr: page.lr_fhr.lock_ref().parse::<u16>().ok(),
                lr_fhr_irrigular: str_some(page.lr_fhr_irrigular.get_cloned()),
                lr_efw: page.lr_efw.lock_ref().parse::<u16>().ok(),
                lr_interval: str_some(lr_int_to_quote(&page.lr_interval_m.lock_ref(), &page.lr_interval_s.lock_ref())),
                lr_duration: page.lr_duration.lock_ref().parse::<u8>().ok(),
                lr_intensity: str_some(page.lr_intensity.get_cloned()),
                lr_pelvic_diagonal: Decimal::from_str_exact(&page.lr_pelvic_diagonal.lock_ref()).ok(),
                lr_pelvic_interspinous: Decimal::from_str_exact(&page.lr_pelvic_interspinous.lock_ref()).ok(),
                lr_pelvic_sidewall: str_some(page.lr_pelvic_sidewall.get_cloned()),
                lr_ischeal_spine: str_some(page.lr_ischeal_spine.get_cloned()),
                lr_sacral_curve: str_some(page.lr_sacral_curve.get_cloned()),
                lr_pubic_angle: page.lr_pubic_angle.lock_ref().parse::<u8>().ok(),
                lr_pelvic_ok: str_some(page.lr_pelvic_ok.get_cloned()),
                lr_cx_dilate: page.lr_cx_dilate.lock_ref().parse::<u8>().ok(),
                lr_cx_efface: page.lr_cx_efface.lock_ref().parse::<u8>().ok(),
                lr_cx_station: page.lr_cx_station.lock_ref().parse::<i8>().ok(),
                lr_cx_position: str_some(page.lr_cx_position.get_cloned()),
                lr_cx_consistency: str_some(page.lr_cx_consistency.get_cloned()),
                lr_cx_bishop: page.lr_cx_bishop.lock_ref().parse::<u8>().ok(),
                lr_cx_ok: str_some(page.lr_cx_ok.get_cloned()),
                lr_membrane: str_some(page.lr_membrane.get_cloned()),
                lr_amniotic_color: str_some(page.lr_amniotic_color.get_cloned()),
                lr_amniotic_smell: str_some(page.lr_amniotic_smell.get_cloned()),

                hiv: str_some(page.hiv.get_cloned()),
                vdrl: str_some(page.vdrl.get_cloned()),
                hbs_ag: str_some(page.hbs_ag.get_cloned()),
                hct: Decimal::from_str_exact(&page.hct.lock_ref()).ok(), // decimal(3,1)
                hiv2: str_some(page.hiv2.get_cloned()),
                vdrl2: str_some(page.vdrl2.get_cloned()),
                hbs_ag2: str_some(page.hbs_ag2.get_cloned()),
                hct2: Decimal::from_str_exact(&page.hct2.lock_ref()).ok(), // decimal(3,1)
                gr: str_some(page.gr.get_cloned()),
                thalassemia: str_some(page.thalassemia.get_cloned()),
                husband: str_some(page.husband.get_cloned()),
                condition_pregnant: opt_empty_none(page.condition_pregnant.get_cloned()), //.or(Some(String::from("ปกติ"))),
                deliver_anomalies: opt_empty_none(page.deliver_anomalies.get_cloned()),   //.or(Some(String::from("ปกติ"))),
                deliver_anomalies_means: str_some(page.deliver_anomalies_means.get_cloned()),
                deliver_location: str_some(page.deliver_location.get_cloned()),
                deliver_first_weight: Decimal::from_str_exact(&page.deliver_first_weight.lock_ref()).ok(), // decimal(5,0)
                deliver_first_health: str_some(page.deliver_first_health.get_cloned()),
                fant_breast_feeding_end_age_month: opt_zero_none(page.fant_breast_feeding_end_age_month.lock_ref().as_ref().and_then(|m| m.parse::<i32>().ok())),
                fant_artificial_feeding_start_age_month: opt_zero_none(page.fant_artificial_feeding_start_age_month.lock_ref().as_ref().and_then(|m| m.parse::<i32>().ok())),
                fant_feeding_etc: page.fant_feeding_etc.get_cloned(),
                supplementary_feeding: str_some(page.supplementary_feeding.get_cloned()), //.or(Some(String::from("ยังไม่ได้รับ"))),
                supplementary_feeding_start_age_month: opt_zero_none(page.supplementary_feeding_start_age_month.lock_ref().parse::<i32>().ok()),
                disease_operation_allergy: page.disease_operation_allergy.get_cloned(), // unused
                inpatient_history: str_some(page.inpatient_history.get_cloned()),       //.or(Some(String::from("ไม่เคย"))),
                inpatient_last_date: str_some(page.inpatient_last_date.get_cloned()),
                inpatient_location: str_some(page.inpatient_location.get_cloned()),
                inpatient_because: str_some(page.inpatient_because.get_cloned()),

                pe_general: str_some(page.pe_general.get_cloned()),
                pe_skin: str_some(page.pe_skin.get_cloned()),
                pe_heent: str_some(page.pe_heent.get_cloned()),
                pe_neck: str_some(page.pe_neck.get_cloned()),
                pe_breastthorax: str_some(page.pe_breastthorax.get_cloned()),
                pe_heart: str_some(page.pe_heart.get_cloned()),
                pe_lungs: str_some(page.pe_lungs.get_cloned()),
                pe_abdomen: str_some(page.pe_abdomen.get_cloned()),
                pe_rectalgenitalia: str_some(page.pe_rectalgenitalia.get_cloned()),
                pe_extremities: str_some(page.pe_extremities.get_cloned()),
                pe_neurological: str_some(page.pe_neurological.get_cloned()),
                pe_ob_gynexam: str_some(page.pe_ob_gynexam.get_cloned()),
                pe_other: str_some(page.pe_other.get_cloned()),
                pe_text: str_some(page.pe_text.get_cloned()),

                ros_eent: str_some(page.ros_eent.get_cloned()),
                ros_neuro: str_some(page.ros_neuro.get_cloned()),
                ros_lung: str_some(page.ros_lung.get_cloned()),
                ros_tb: str_some(page.ros_tb.get_cloned()),
                ros_ht: str_some(page.ros_ht.get_cloned()),
                ros_heart: str_some(page.ros_heart.get_cloned()),
                ros_liver: str_some(page.ros_liver.get_cloned()),
                ros_gi: str_some(page.ros_gi.get_cloned()),
                ros_endocrine: str_some(page.ros_endocrine.get_cloned()),
                ros_kidney: str_some(page.ros_kidney.get_cloned()),
                ros_tumour: str_some(page.ros_tumour.get_cloned()),
                ros_hemato: str_some(page.ros_hemato.get_cloned()),
                ros_rheumato: str_some(page.ros_rheumato.get_cloned()),
                ros_psychia: str_some(page.ros_psychia.get_cloned()),
                ros_other: str_some(page.ros_other.get_cloned()),

                addict: str_some(page.addict.get_cloned()),
                addict_assist: str_some(concat_mutable_vec(&page.addict_assists, concat_with_space)),
                addict_inj: str_some(page.addict_inj.get_cloned()),
                addict_inj_often: str_some(page.addict_inj_often.get_cloned()),
                amphetamine_awq: str_some(page.amphetamine_awq.get_cloned()),
                aggression_oas: str_some(page.aggression_oas.get_cloned()),
                motivation_scale: page.motivation_scale.lock_ref().parse::<u8>().ok(),
                craving_scale: page.craving_scale.lock_ref().parse::<u8>().ok(),
                stage_of_change_id: page.stage_of_change_id.lock_ref().parse::<u8>().ok(),
                alcohol_audit: str_some(page.alcohol_audit.get_cloned()),
                alcohol_aws: str_some(page.alcohol_aws.get_cloned()),
                alcohol_ciwa: str_some(page.alcohol_ciwa.get_cloned()),
                depress_2q: str_some(page.depress_2q.get_cloned()),
                depress_9q: str_some(page.depress_9q.get_cloned()),
                depress_cdi: str_some(page.depress_cdi.get_cloned()),
                depress_cesd: str_some(page.depress_cesd.get_cloned()),
                depress_phqa: str_some(page.depress_phqa.get_cloned()),
                nicotin_ftnd: str_some(page.nicotin_ftnd.get_cloned()),
                ptsd_screen: str_some(page.ptsd_screen.get_cloned()),
                ptsd_pisces: str_some(page.ptsd_pisces.get_cloned()),
                ptsd_cries: str_some(page.ptsd_cries.get_cloned()),
                suicide_8q: str_some(page.suicide_8q.get_cloned()),
                stress_st5: str_some(page.stress_st5.get_cloned()),

                svg_tag: page
                    .canvas
                    .lock_ref()
                    .as_ref()
                    .and_then(|c| (!c.get_objects().is_empty()).then(|| c.to_svg_suppress_preamble()).filter(|s| s.contains("path"))),

                impression: str_some(page.impression.get_cloned()),
                diff_dx: str_some(page.diff_dx.get_cloned()),
                plan_management: str_some(page.plan_management.get_cloned()),

                nurse_name: str_some(page.nurse_name.get_cloned()),
                nurse_pos: str_some(page.nurse_pos.get_cloned()),
                nurse_licenseno: str_some(page.nurse_licenseno.get_cloned()),
                doc_name: page.doc_name.get_cloned(), // unused
                doc_pos: page.doc_pos.get_cloned(),   // unused

                // skip when insert/update
                imgs: None,
                stage_of_change_name: None,

                admission_note_id: page.admission_note_id.get().unwrap_or_default(), // AUTO_INCREMENT
            }
        })
    }
}

#[derive(Clone, Default, PartialEq)]
enum ArriveBy {
    #[default]
    Walk, // include null // "เดินมา",
    WheelChair,     // "รถนั่ง",
    Stretcher,      // "รถนอน",
    Refer,          // "รถ Transfer",
    Others(String), // "อื่น ๆ",
}

impl ArriveBy {
    fn string(&self) -> String {
        match self {
            ArriveBy::Walk => String::from("เดินมา"),
            ArriveBy::WheelChair => String::from("รถนั่ง"),
            ArriveBy::Stretcher => String::from("รถนอน"),
            ArriveBy::Refer => String::from("รถ Transfer"),
            ArriveBy::Others(by) => String::from(by),
        }
    }
    fn from_str(text: &str) -> Self {
        match text {
            "เดินมา" => Self::Walk,
            "รถนั่ง" => Self::WheelChair,
            "รถนอน" => Self::Stretcher,
            "รถ Transfer" => Self::Refer,
            by => Self::Others(String::from(by)),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
enum TakeMedicationBy {
    #[default]
    YourSelf, //"มาเอง"
    Appointment,       //"แพทย์นัด"
    ReferFrom(String), // "ส่งตัวจาก"
}

impl TakeMedicationBy {
    fn string(&self) -> String {
        match self {
            TakeMedicationBy::YourSelf => String::from("มาเอง"),
            TakeMedicationBy::Appointment => String::from("แพทย์นัด"),
            TakeMedicationBy::ReferFrom(by) => String::from(by),
        }
    }

    fn from_str(text: &str) -> Self {
        match text {
            "มาเอง" => Self::YourSelf,
            "แพทย์นัด" => Self::Appointment,
            by => Self::ReferFrom(String::from(by)),
        }
    }
}

#[derive(Clone, Default)]
struct DiseaseDetail {
    id: u32,
    name: Mutable<String>,
    year: Mutable<String>,
    hospital: Mutable<String>,
}

impl PartialEq<DiseaseDetail> for DiseaseDetail {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Concat for DiseaseDetail {
    fn concat(&self, concat_with_space: bool) -> String {
        let name = self.name.lock_ref();
        if name.is_empty() {
            String::new()
        } else {
            let delimiter = if concat_with_space { " " } else { "^" };
            [name.as_str(), delimiter, self.year.lock_ref().as_str(), delimiter, self.hospital.lock_ref().as_str()].concat()
        }
    }
}

impl DiseaseDetail {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(dd: Rc<Self>, page: Rc<IpdAdmissionNoteDrPage>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::ROW)
            .children([
                html!("div", {.class("col-sm-2")}),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .class(class::FORM_CTRL_SM)
                        .attr("type", "text")
                        .apply(mixins::string_value(dd.name.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .class(class::FORM_CTRL_SM)
                        .attr("type", "text")
                        .apply(mixins::string_value(dd.year.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("div", {
                        .class(class::INPUT_GROUP)
                        .children([
                            html!("input" => HtmlInputElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("type", "text")
                                .apply(mixins::string_value(dd.hospital.clone(), page.changed.clone()))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_GRAY)
                                .text("รักษาที่นี่")
                                .event(clone!(app, page, dd => move |_:events::Click| {
                                    if let Some(item) = page.disease_details.lock_ref().iter().find(|raw| raw.id == dd.id) {
                                        item.hospital.set_neq(app.app_status.lock_ref().as_ref().map(|aps| aps.hospital_name.clone()).unwrap_or_default());
                                        page.is_set_this_hospital.set_neq(true);
                                    }
                                }))
                            })
                        ])
                    }))
                }),
                html!("div", {
                    .class(class::COL_SM1_PT1)
                    .child(html!("a", {
                        .attr("href","#")
                        .child(html!("i", {
                            .class(class::FA_TRASH)
                            .style("color","Tomato")
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                            event.prevent_default();
                            page.disease_details.lock_mut().retain(|x| **x != *dd);
                            if page.disease_details.lock_ref().is_empty() {
                                mixins::with_string(NOTHING, page.disease.clone(), page.changed.clone());
                            }
                        }))
                    }))
                }),
            ])
        })
    }
}

#[derive(Clone, Default)]
struct DrugAllergy {
    id: u32,
    agent: Mutable<String>,   // er_allergy_history_agent
    symptom: Mutable<String>, // er_allergy_history_symptom
}

impl Concat for DrugAllergy {
    fn concat(&self, concat_with_space: bool) -> String {
        let agent = self.agent.lock_ref();
        if agent.is_empty() {
            String::new()
        } else {
            let delimiter = if concat_with_space { " " } else { "^" };
            [agent.as_str(), delimiter, self.symptom.lock_ref().as_str()].concat()
        }
    }
}

impl PartialEq<DrugAllergy> for DrugAllergy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<&OpdErAllergyHistory> for DrugAllergy {
    fn from(item: &OpdErAllergyHistory) -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            agent: Mutable::new(item.er_allergy_history_agent.clone().unwrap_or_default()),
            symptom: Mutable::new(item.er_allergy_history_symptom.clone().unwrap_or_default()),
        }
    }
}

impl DrugAllergy {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(da: Rc<Self>, page: Rc<IpdAdmissionNoteDrPage>) -> Dom {
        html!("div", {
            .class(class::ROW)
            .children([
                html!("div", {.class("col-sm-5")}),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .style("color","#E60000")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(da.agent.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .style("color","#E60000")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(da.symptom.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class(class::COL_SM1_PT1)
                    .child(html!("a", {
                        .attr("href","#")
                        .child(html!("i", {
                            .class(class::FA_TRASH)
                            .style("color","Tomato")
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                            event.prevent_default();
                            page.allergy_drugs.lock_mut().retain(|x| **x != *da);
                            if page.allergy_drugs.lock_ref().is_empty() && page.allergy_foods.lock_ref().is_empty() && page.allergy_etcs.lock_ref().is_empty() {
                                mixins::with_string(NOTHING, page.allergy_history.clone(), page.changed.clone());
                            }
                        }))
                    }))
                }),
            ])
        })
    }
}

#[derive(Clone, Default)]
struct FoodAllergy {
    id: u32,
    agent: Mutable<String>,
    symptom: Mutable<String>,
}

impl Concat for FoodAllergy {
    fn concat(&self, concat_with_space: bool) -> String {
        let agent = self.agent.lock_ref();
        if agent.is_empty() {
            String::new()
        } else {
            let delimiter = if concat_with_space { " " } else { "^" };
            [agent.as_str(), delimiter, self.symptom.lock_ref().as_str()].concat()
        }
    }
}

impl PartialEq<FoodAllergy> for FoodAllergy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FoodAllergy {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(fa: Rc<Self>, page: Rc<IpdAdmissionNoteDrPage>) -> Dom {
        html!("div", {
            .class(class::ROW)
            .children([
                html!("div", {.class("col-sm-5")}),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .style("color","#E60000")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(fa.agent.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .style("color","#E60000")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(fa.symptom.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class(class::COL_SM1_PT1)
                    .child(html!("a", {
                        .attr("href","#")
                        .child(html!("i", {
                            .class(class::FA_TRASH)
                            .style("color","Tomato")
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                            event.prevent_default();
                            page.allergy_foods.lock_mut().retain(|x| **x != *fa);
                            if page.allergy_drugs.lock_ref().is_empty() && page.allergy_foods.lock_ref().is_empty() && page.allergy_etcs.lock_ref().is_empty() {
                                mixins::with_string(NOTHING, page.allergy_history.clone(), page.changed.clone());
                            }
                        }))
                    }))
                }),
            ])
        })
    }
}

#[derive(Clone, Default)]
struct EtcAllergy {
    id: u32,
    agent: Mutable<String>,
    symptom: Mutable<String>,
}

impl Concat for EtcAllergy {
    fn concat(&self, concat_with_space: bool) -> String {
        let agent = self.agent.lock_ref();
        if agent.is_empty() {
            String::new()
        } else {
            let delimiter = if concat_with_space { " " } else { "^" };
            [agent.as_str(), delimiter, self.symptom.lock_ref().as_str()].concat()
        }
    }
}

impl PartialEq<EtcAllergy> for EtcAllergy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl EtcAllergy {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(ea: Rc<Self>, page: Rc<IpdAdmissionNoteDrPage>) -> Dom {
        html!("div", {
            .class(class::ROW)
            .children([
                html!("div", {.class("col-sm-5")}),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .style("color","#E60000")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(ea.agent.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .style("color","#E60000")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(ea.symptom.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class(class::COL_SM1_PT1)
                    .child(html!("a", {
                        .attr("href","#")
                        .child(html!("i", {
                            .class(class::FA_TRASH)
                            .style("color","Tomato")
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                            event.prevent_default();
                            page.allergy_etcs.lock_mut().retain(|x| **x != *ea);
                            if page.allergy_drugs.lock_ref().is_empty() && page.allergy_foods.lock_ref().is_empty() && page.allergy_etcs.lock_ref().is_empty() {
                                mixins::with_string(NOTHING, page.allergy_history.clone(), page.changed.clone());
                            }
                        }))
                    }))
                }),
            ])
        })
    }
}

#[derive(Clone, Default)]
struct FamilyMedical {
    id: u32,
    disease: Mutable<String>,
    relation: Mutable<String>,
}

impl Concat for FamilyMedical {
    fn concat(&self, concat_with_space: bool) -> String {
        let disease = self.disease.lock_ref();
        if disease.is_empty() {
            String::new()
        } else {
            let delimiter = if concat_with_space { " " } else { "^" };
            [disease.as_str(), delimiter, self.relation.lock_ref().as_str()].concat()
        }
    }
}

impl PartialEq<FamilyMedical> for FamilyMedical {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FamilyMedical {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(fm: Rc<Self>, page: Rc<IpdAdmissionNoteDrPage>) -> Dom {
        html!("div", {
            .class(class::ROW)
            .children([
                html!("div", {.class("col-sm-3")}),
                html!("div", {
                    .class("col-sm-5")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(fm.disease.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(fm.relation.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class(class::COL_SM1_PT1)
                    .child(html!("a", {
                        .attr("href","#")
                        .child(html!("i", {
                            .class(class::FA_TRASH)
                            .style("color","Tomato")
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                            event.prevent_default();
                            page.family_medicals.lock_mut().retain(|x| **x != *fm);
                            if page.family_medicals.lock_ref().is_empty() && page.allergy_foods.lock_ref().is_empty() && page.allergy_etcs.lock_ref().is_empty() {
                                mixins::with_string(NOTHING, page.family_medical_history.clone(), page.changed.clone());
                            }
                        }))
                    }))
                }),
            ])
        })
    }
}

#[derive(Clone, Default)]
struct AddictAssist {
    id: u32,
    agent: Mutable<String>,
    score: Mutable<String>,
}

impl Concat for AddictAssist {
    fn concat(&self, concat_with_space: bool) -> String {
        let agent = self.agent.lock_ref();
        if agent.is_empty() {
            String::new()
        } else {
            let delimiter = if concat_with_space { " " } else { "^" };
            [agent.as_str(), delimiter, self.score.lock_ref().as_str()].concat()
        }
    }
}

impl PartialEq for AddictAssist {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl AddictAssist {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(fa: Rc<Self>, page: Rc<IpdAdmissionNoteDrPage>) -> Dom {
        html!("div", {
            .class(class::ROW)
            .children([
                html!("div", {.class("col-sm-2")}),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("input" => HtmlInputElement, {
                        .attr("type", "text")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(fa.agent.clone(), page.changed.clone()))
                    }))
                }),
                html!("div", {
                    .class("col-sm-3")
                    .child(html!("div", {
                        .class(class::INPUT_GROUP_SM)
                        .children([
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_GRAY)
                                .attr("data-bs-toggle", "modal")
                                .attr("data-bs-target", "#addictAssistModal")
                                .child(html!("i", {.class(class::FA_EDIT)}))
                                .event(clone!(page, fa => move |_:events::Click| {
                                    page.addict_assist_modal.set(Some(AddictAssistV2::new(
                                        fa.agent.clone(),
                                        fa.score.clone(),
                                        page.changed.clone(),
                                    )));
                                }))
                            }),
                            html!("div", {
                                .class("input-group-text")
                                .style_signal("background-color", fa.score.signal_ref(|s| s.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|assist| {
                                    match assist {
                                        0 => "inherit",
                                        1..4 => "gold",
                                        4..27 => "pink",
                                        27.. => "salmon",
                                    }
                                }).unwrap_or("inherit")))
                                .text_signal(fa.score.signal_ref(|s| s.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|assist| {
                                    let value = match assist {
                                        0 => "ไม่เคย",
                                        1..4 => "ผู้ใช้",
                                        4..27 => "ผู้เสพ",
                                        27.. => "ผู้ติด",
                                    };
                                    [&assist.to_string(), " : ", &value].concat()
                                }).unwrap_or(String::from("รอการประเมิน"))))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_REDO)
                                .child(html!("i", {.class(class::FA_TRASH)}))
                                .event(clone!(page => move |_: events::Click| {
                                    page.addict_assists.lock_mut().retain(|x| **x != *fa);
                                    if page.addict_assists.lock_ref().is_empty() {
                                        mixins::with_string(NOTHING, page.addict.clone(), page.changed.clone());
                                    }
                                }))
                            }),
                        ])
                    }))
                }),
                html!("div", {
                    .class(class::COL_SM1_PT1)
                    .child(html!("a", {
                        .attr("href","#")
                        .class("pt-1")

                    }))
                }),
            ])
        })
    }
}

fn render_ros_item(label: &str, ros: Mutable<String>, changed: Mutable<bool>) -> Dom {
    html!("div", {
        .class(class::ROW)
        .children([
            html!("label", {
                .class(class::COL_SM5_R)
                .text(label)
            }),
            html!("div", {
                .class("col-sm-5")
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "text")
                    .attr("maxlength", "200")
                    .class(class::FORM_CTRL_SM)
                    .apply(mixins::string_value(ros.clone(), changed.clone()))
                }))
            }),
            html!("div", {
                .class("col-md-2")
                .child(html!("button", {
                    .attr("type", "button")
                    .class(class::BTN_SM_L_GRAY)
                    .text(NOTHING)
                    .event(move |_: events::Click| {
                        mixins::with_string(NOTHING, ros.clone(), changed.clone());
                    })
                }))
            }),
        ])
    })
}

fn render_doctor(doctor: Rc<AdmissionNoteDoctor>) -> Dom {
    html!("div", {
        .class("dr_admission_input_div")
        .child(html!("div", {
            .class(class::INPUT_GROUP_T)
            .child(html!("input", {
                .attr("type", "text")
                .attr("readonly", "")
                .class("form-control")
                .attr("value", &doctor.admission_note_doctorname.clone().unwrap_or_default())
            }))
        }))
    })
}
