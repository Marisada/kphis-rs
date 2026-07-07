// ipd-dr-admission-note-form.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;
use time::{Date, Time};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    route::Route,
    {
        ipd::admission_note_nurse::IpdNurseAdmissionNote,
        report::{SystemReport, TypstReport},
    },
};
use kphis_ui_app::App;
use kphis_ui_component::{
    gadget::pdf_button::PdfButtons,
    modal::{blank_modal, vs_selector::VsSelector},
    show_patient_main::ShowPatientMainCpn,
};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::util::{str_some, zero_none};

/// - GET `EndPoint::IpdAdmissionNoteNurseAn`
/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - GET `EndPoint::IpdVitalSign` (VsSelector, guarded, remove v/s btn)
/// - GET `EndPoint::OpdErVitalSign` (VsSelector, guarded, remove v/s btn)
/// - POST/PUT `EndPoint::IpdAdmissionNoteNurse` (guarded, remove 'บันทึก' btn)
#[derive(Clone, Default)]
pub struct IpdAdmissionNoteNursePage {
    // page mechanic
    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    // patient main
    patient: Mutable<Rc<ShowPatientMainCpn>>,

    vs_selector_modal: Mutable<Option<Rc<VsSelector>>>,

    // database data
    nurse_admission_note_id: Mutable<Option<u32>>, // row
    hn: Mutable<String>,
    an: Mutable<String>,
    info_patient: Mutable<String>,
    info_parent: Mutable<String>,
    info_spouse: Mutable<String>,
    info_child: Mutable<String>,
    info_relatives: Mutable<String>,
    info_sender: Mutable<String>,
    chief_complaints: Mutable<String>,
    medical_history: Mutable<String>,
    vs_admit: Mutable<String>,
    concious: Mutable<String>,
    normal_breath: Mutable<String>,
    kussmaul: Mutable<String>,
    tachypnea: Mutable<String>,
    dyspnea: Mutable<String>,
    apnea: Mutable<String>,
    tube: Mutable<String>,
    normal_hr: Mutable<String>,
    arregular: Mutable<String>,
    weakness: Mutable<String>,
    arrhythmia: Mutable<String>,
    chestpain: Mutable<String>,
    pacemaker: Mutable<String>,
    cardio_other: Mutable<String>,
    cardio_other_text: Mutable<String>,
    normal_cir: Mutable<String>,
    pale: Mutable<String>,
    cyanosis: Mutable<String>,
    generalized_edema: Mutable<String>,
    localized_edema: Mutable<String>,
    localized_edema_text: Mutable<String>,
    pitting_edema: Mutable<String>,
    pitting_edema_text: Mutable<String>,
    circulation_orther: Mutable<String>,
    circulation_orther_text: Mutable<String>,
    normal_skin: Mutable<String>,
    dry: Mutable<String>,
    bruise: Mutable<String>,
    erythema: Mutable<String>,
    abscess: Mutable<String>,
    joudice: Mutable<String>,
    skin_other: Mutable<String>,
    skin_other_text: Mutable<String>,
    pain: Mutable<String>,
    location: Mutable<String>,
    pain_charac: Mutable<String>,
    pain_charac_text: Mutable<String>,
    pain_score: Mutable<String>,
    normal_behav: Mutable<String>,
    agitate: Mutable<String>,
    aggressive: Mutable<String>,
    depression: Mutable<String>,
    madness: Mutable<String>,
    behaviour_other: Mutable<String>,
    behaviour_other_text: Mutable<String>,
    normal_emotional: Mutable<String>,
    angry: Mutable<String>,
    moody: Mutable<String>,
    anxiety: Mutable<String>,
    fear: Mutable<String>,
    emotional_other: Mutable<String>,
    emotional_other_text: Mutable<String>,
    no_anxiety: Mutable<String>,
    study: Mutable<String>,
    family: Mutable<String>,
    economy: Mutable<String>,
    habitation: Mutable<String>,
    illness: Mutable<String>,
    spiritual_no: Mutable<String>,
    spiritual_back_home: Mutable<String>,
    spiritual_need_family: Mutable<String>,
    spiritual_other: Mutable<String>,
    spiritual_other_text: Mutable<String>,
    spiritual_cant_rated: Mutable<String>,
    spiritual_cant_rated_text: Mutable<String>,
    no_mental_state: Mutable<String>,
    no_mental_state_text: Mutable<String>,
    education: Mutable<String>,
    education_result: Mutable<String>,
    occupation: Mutable<String>,
    income: Mutable<String>,
    self_value: Mutable<String>,
    person_family: Mutable<String>,
    neighbor: Mutable<String>,
    assistant_other: Mutable<String>,
    assistant_other_text: Mutable<String>,
    assistant_occupation: Mutable<String>,
    clinic: Mutable<String>,
    buy_medicine: Mutable<String>,
    no_risk: Mutable<String>,
    smoking: Mutable<String>,
    smoke_year: Mutable<String>,
    smoke_frequency: Mutable<String>,
    smoke_stopped: Mutable<String>,
    alcohol: Mutable<String>,
    alc_year: Mutable<String>,
    alc_frequency: Mutable<String>,
    alc_stopped: Mutable<String>,
    medication_used: Mutable<String>,
    med_name: Mutable<String>,
    med_year: Mutable<String>,
    med_frequency: Mutable<String>,
    med_stopped: Mutable<String>,
    diet_regular: Mutable<String>,
    diet_spec: Mutable<String>,
    nutrition_risk: Mutable<String>,
    loss_appetite: Mutable<String>,
    dysphagia: Mutable<String>,
    loss_gustation: Mutable<String>,
    denture: Mutable<String>,
    nutrition_risk_other: Mutable<String>,
    nutrition_risk_other_text: Mutable<String>,
    normal_urine: Mutable<String>,
    dysuria: Mutable<String>,
    incontinence: Mutable<String>,
    staining: Mutable<String>,
    hematuria: Mutable<String>,
    catheter: Mutable<String>,
    normal_feces: Mutable<String>,
    constipation: Mutable<String>,
    diarrhea: Mutable<String>,
    bowel_incontinence: Mutable<String>,
    hemorrhoid: Mutable<String>,
    colostomy: Mutable<String>,
    activity1: Mutable<String>,
    activity2: Mutable<String>,
    activity3: Mutable<String>,
    activity4: Mutable<String>,
    o_p_use: Mutable<String>,
    sleep_per_day: Mutable<String>,
    sleep_hour: Mutable<String>,
    sleep_problems: Mutable<String>,
    sleep_problems_detail: Mutable<String>,
    sleep_med_name: Mutable<String>,
    sleep_med_name_detail: Mutable<String>,
    cognitive: Mutable<String>,
    memory: Mutable<String>,
    memory_detail: Mutable<String>,
    hearing: Mutable<String>,
    hearing_detail: Mutable<String>,
    eartone: Mutable<String>,
    vision: Mutable<String>,
    vision_detail: Mutable<String>,
    vision_eyeglasses: Mutable<String>,
    vision_contactlens: Mutable<String>,
    speech: Mutable<String>,
    speech_detail: Mutable<String>,
    self_image: Mutable<String>,
    self_image_detail: Mutable<String>,
    self_activity: Mutable<String>,
    self_activity_detail: Mutable<String>,
    sickness_effect: Mutable<String>,
    sickness_family: Mutable<String>,
    sickness_occupation: Mutable<String>,
    sickness_education: Mutable<String>,
    sickness_other: Mutable<String>,
    sickness_other_text: Mutable<String>,
    period: Mutable<String>,
    period_normal: Mutable<String>,
    period_disorders: Mutable<String>,
    period_lmp: Mutable<String>,
    period_menopause: Mutable<String>,
    breast: Mutable<String>,
    breast_disorders: Mutable<String>,
    consult: Mutable<String>,
    seclude: Mutable<String>,
    medication: Mutable<String>,
    medication_detail: Mutable<String>,
    religion: Mutable<String>,
    coping_stress_other: Mutable<String>,
    coping_stress_other_detail: Mutable<String>,
    belief_sickness_behave: Mutable<String>,
    belief_sickness_age: Mutable<String>,
    belief_sickness_destiny: Mutable<String>,
    belief_sickness_other: Mutable<String>,
    belief_sickness_other_text: Mutable<String>,
    belief_believe: Mutable<String>,
    belief_believe_text: Mutable<String>,
    religious_activity: Mutable<String>,
    religious_activity_text: Mutable<String>,

    nurse_name: Mutable<String>,
    nurse_pos: Mutable<String>,
    nurse_licenseno: Mutable<String>,
    receiver_medication_date: Mutable<Option<Date>>,
    receiver_medication_time: Mutable<Option<Time>>,
}

impl IpdAdmissionNoteNursePage {
    pub fn new(an: String, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            an: Mutable::new(an),
            info_patient: Mutable::new(String::from("Y")),
            concious: Mutable::new(String::from("รู้สึกตัวดี")),
            normal_breath: Mutable::new(String::from("Y")),
            normal_hr: Mutable::new(String::from("Y")),
            normal_cir: Mutable::new(String::from("Y")),
            normal_skin: Mutable::new(String::from("Y")),
            pain: Mutable::new(String::from("ไม่มี")),
            normal_behav: Mutable::new(String::from("Y")),
            normal_emotional: Mutable::new(String::from("Y")),
            no_anxiety: Mutable::new(String::from("Y")),
            spiritual_no: Mutable::new(String::from("Y")),
            education: Mutable::new(String::from("ไม่ได้รับ")),
            income: Mutable::new(String::from("เพียงพอ")),
            self_value: Mutable::new(String::from("Y")),
            clinic: Mutable::new(String::from("Y")),
            no_risk: Mutable::new(String::from("Y")),
            diet_regular: Mutable::new(String::from("อาหารทั่วไป")),
            nutrition_risk: Mutable::new(String::from("Y")),
            normal_urine: Mutable::new(String::from("Y")),
            normal_feces: Mutable::new(String::from("Y")),
            activity1: Mutable::new(String::from("Y")),
            sleep_med_name: Mutable::new(String::from("ไม่เคย")),
            cognitive: Mutable::new(String::from("ตรง")),
            memory: Mutable::new(String::from("ปกติ")),
            hearing: Mutable::new(String::from("ปกติ")),
            vision: Mutable::new(String::from("ปกติ")),
            speech: Mutable::new(String::from("ปกติ")),
            self_image: Mutable::new(String::from("ไม่มี")),
            self_activity: Mutable::new(String::from("ไม่มี")),
            sickness_effect: Mutable::new(String::from("ไม่มี")),
            period: Mutable::new(String::from("ยังไม่มี")),
            breast: Mutable::new(String::from("ปกติ")),
            consult: Mutable::new(String::from("Y")),
            belief_sickness_behave: Mutable::new(String::from("Y")),
            belief_believe: Mutable::new(String::from("ไม่มี")),
            religious_activity: Mutable::new(String::from("ไม่ต้องการ")),
            nurse_name: Mutable::new(app.doctor_name().unwrap_or_default()),
            nurse_pos: Mutable::new(app.doctor_entryposition().unwrap_or_default()),
            nurse_licenseno: Mutable::new(app.doctor_licenseno().unwrap_or_default()),
            ..Default::default()
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::IpdAdmissionNoteNurseAn`
                match IpdNurseAdmissionNote::call_api_get(&page.an.lock_ref(), app.state()).await {
                    Ok(response) => {
                        let mut nurse_admission_note_id = None;
                        if let Some(note) = response {
                            nurse_admission_note_id = zero_none(note.nurse_admission_note_id);
                            page.nurse_admission_note_id.set_neq(nurse_admission_note_id);
                            page.hn.set_neq(note.hn);
                            page.info_patient.set_neq(note.info_patient.unwrap_or_default());
                            page.info_parent.set_neq(note.info_parent.unwrap_or_default());
                            page.info_spouse.set_neq(note.info_spouse.unwrap_or_default());
                            page.info_child.set_neq(note.info_child.unwrap_or_default());
                            page.info_relatives.set_neq(note.info_relatives.unwrap_or_default());
                            page.info_sender.set_neq(note.info_sender.unwrap_or_default());
                            page.chief_complaints.set_neq(note.chief_complaints.unwrap_or_default());
                            page.medical_history.set_neq(note.medical_history.unwrap_or_default());
                            page.vs_admit.set_neq(note.vs_admit.unwrap_or_default());
                            page.concious.set_neq(note.concious.unwrap_or_default());
                            page.normal_breath.set_neq(note.normal_breath.unwrap_or_default());
                            page.kussmaul.set_neq(note.kussmaul.unwrap_or_default());
                            page.tachypnea.set_neq(note.tachypnea.unwrap_or_default());
                            page.dyspnea.set_neq(note.dyspnea.unwrap_or_default());
                            page.apnea.set_neq(note.apnea.unwrap_or_default());
                            page.tube.set_neq(note.tube.unwrap_or_default());
                            page.normal_hr.set_neq(note.normal_hr.unwrap_or_default());
                            page.arregular.set_neq(note.arregular.unwrap_or_default());
                            page.weakness.set_neq(note.weakness.unwrap_or_default());
                            page.arrhythmia.set_neq(note.arrhythmia.unwrap_or_default());
                            page.chestpain.set_neq(note.chestpain.unwrap_or_default());
                            page.pacemaker.set_neq(note.pacemaker.unwrap_or_default());
                            page.cardio_other.set_neq(note.cardio_other.unwrap_or_default());
                            page.cardio_other_text.set_neq(note.cardio_other_text.unwrap_or_default());
                            page.normal_cir.set_neq(note.normal_cir.unwrap_or_default());
                            page.pale.set_neq(note.pale.unwrap_or_default());
                            page.cyanosis.set_neq(note.cyanosis.unwrap_or_default());
                            page.generalized_edema.set_neq(note.generalized_edema.unwrap_or_default());
                            page.localized_edema.set_neq(note.localized_edema.unwrap_or_default());
                            page.localized_edema_text.set_neq(note.localized_edema_text.unwrap_or_default());
                            page.pitting_edema.set_neq(note.pitting_edema.unwrap_or_default());
                            page.pitting_edema_text.set_neq(note.pitting_edema_text.unwrap_or_default());
                            page.circulation_orther.set_neq(note.circulation_orther.unwrap_or_default());
                            page.circulation_orther_text.set_neq(note.circulation_orther_text.unwrap_or_default());
                            page.normal_skin.set_neq(note.normal_skin.unwrap_or_default());
                            page.dry.set_neq(note.dry.unwrap_or_default());
                            page.bruise.set_neq(note.bruise.unwrap_or_default());
                            page.erythema.set_neq(note.erythema.unwrap_or_default());
                            page.abscess.set_neq(note.abscess.unwrap_or_default());
                            page.joudice.set_neq(note.joudice.unwrap_or_default());
                            page.skin_other.set_neq(note.skin_other.unwrap_or_default());
                            page.skin_other_text.set_neq(note.skin_other_text.unwrap_or_default());
                            page.pain.set_neq(note.pain.unwrap_or_default());
                            page.location.set_neq(note.location.unwrap_or_default());
                            page.pain_charac.set_neq(note.pain_charac.unwrap_or_default());
                            page.pain_charac_text.set_neq(note.pain_charac_text.unwrap_or_default());
                            page.pain_score.set_neq(note.pain_score.unwrap_or_default());
                            page.normal_behav.set_neq(note.normal_behav.unwrap_or_default());
                            page.agitate.set_neq(note.agitate.unwrap_or_default());
                            page.aggressive.set_neq(note.aggressive.unwrap_or_default());
                            page.depression.set_neq(note.depression.unwrap_or_default());
                            page.madness.set_neq(note.madness.unwrap_or_default());
                            page.behaviour_other.set_neq(note.behaviour_other.unwrap_or_default());
                            page.behaviour_other_text.set_neq(note.behaviour_other_text.unwrap_or_default());
                            page.normal_emotional.set_neq(note.normal_emotional.unwrap_or_default());
                            page.angry.set_neq(note.angry.unwrap_or_default());
                            page.moody.set_neq(note.moody.unwrap_or_default());
                            page.anxiety.set_neq(note.anxiety.unwrap_or_default());
                            page.fear.set_neq(note.fear.unwrap_or_default());
                            page.emotional_other.set_neq(note.emotional_other.unwrap_or_default());
                            page.emotional_other_text.set_neq(note.emotional_other_text.unwrap_or_default());
                            page.no_anxiety.set_neq(note.no_anxiety.unwrap_or_default());
                            page.study.set_neq(note.study.unwrap_or_default());
                            page.family.set_neq(note.family.unwrap_or_default());
                            page.economy.set_neq(note.economy.unwrap_or_default());
                            page.habitation.set_neq(note.habitation.unwrap_or_default());
                            page.illness.set_neq(note.illness.unwrap_or_default());
                            page.spiritual_no.set_neq(note.spiritual_no.unwrap_or_default());
                            page.spiritual_back_home.set_neq(note.spiritual_back_home.unwrap_or_default());
                            page.spiritual_need_family.set_neq(note.spiritual_need_family.unwrap_or_default());
                            page.spiritual_other.set_neq(note.spiritual_other.unwrap_or_default());
                            page.spiritual_other_text.set_neq(note.spiritual_other_text.unwrap_or_default());
                            page.spiritual_cant_rated.set_neq(note.spiritual_cant_rated.unwrap_or_default());
                            page.spiritual_cant_rated_text.set_neq(note.spiritual_cant_rated_text.unwrap_or_default());
                            page.no_mental_state.set_neq(note.no_mental_state.unwrap_or_default());
                            page.no_mental_state_text.set_neq(note.no_mental_state_text.unwrap_or_default());
                            page.education.set_neq(note.education.unwrap_or_default());
                            page.education_result.set_neq(note.education_result.unwrap_or_default());
                            page.occupation.set_neq(note.occupation.unwrap_or_default());
                            page.income.set_neq(note.income.unwrap_or_default());
                            page.self_value.set_neq(note.self_value.unwrap_or_default());
                            page.person_family.set_neq(note.person_family.unwrap_or_default());
                            page.neighbor.set_neq(note.neighbor.unwrap_or_default());
                            page.assistant_other.set_neq(note.assistant_other.unwrap_or_default());
                            page.assistant_other_text.set_neq(note.assistant_other_text.unwrap_or_default());
                            page.assistant_occupation.set_neq(note.assistant_occupation.unwrap_or_default());
                            page.clinic.set_neq(note.clinic.unwrap_or_default());
                            page.buy_medicine.set_neq(note.buy_medicine.unwrap_or_default());
                            page.no_risk.set_neq(note.no_risk.unwrap_or_default());
                            page.smoking.set_neq(note.smoking.unwrap_or_default());
                            page.smoke_year.set_neq(note.smoke_year.unwrap_or_default());
                            page.smoke_frequency.set_neq(note.smoke_frequency.unwrap_or_default());
                            page.smoke_stopped.set_neq(note.smoke_stopped.unwrap_or_default());
                            page.alcohol.set_neq(note.alcohol.unwrap_or_default());
                            page.alc_year.set_neq(note.alc_year.unwrap_or_default());
                            page.alc_frequency.set_neq(note.alc_frequency.unwrap_or_default());
                            page.alc_stopped.set_neq(note.alc_stopped.unwrap_or_default());
                            page.medication_used.set_neq(note.medication_used.unwrap_or_default());
                            page.med_name.set_neq(note.med_name.unwrap_or_default());
                            page.med_year.set_neq(note.med_year.unwrap_or_default());
                            page.med_frequency.set_neq(note.med_frequency.unwrap_or_default());
                            page.med_stopped.set_neq(note.med_stopped.unwrap_or_default());
                            page.diet_regular.set_neq(note.diet_regular.unwrap_or_default());
                            page.diet_spec.set_neq(note.diet_spec.unwrap_or_default());
                            page.nutrition_risk.set_neq(note.nutrition_risk.unwrap_or_default());
                            page.loss_appetite.set_neq(note.loss_appetite.unwrap_or_default());
                            page.dysphagia.set_neq(note.dysphagia.unwrap_or_default());
                            page.loss_gustation.set_neq(note.loss_gustation.unwrap_or_default());
                            page.denture.set_neq(note.denture.unwrap_or_default());
                            page.nutrition_risk_other.set_neq(note.nutrition_risk_other.unwrap_or_default());
                            page.nutrition_risk_other_text.set_neq(note.nutrition_risk_other_text.unwrap_or_default());
                            page.normal_urine.set_neq(note.normal_urine.unwrap_or_default());
                            page.dysuria.set_neq(note.dysuria.unwrap_or_default());
                            page.incontinence.set_neq(note.incontinence.unwrap_or_default());
                            page.staining.set_neq(note.staining.unwrap_or_default());
                            page.hematuria.set_neq(note.hematuria.unwrap_or_default());
                            page.catheter.set_neq(note.catheter.unwrap_or_default());
                            page.normal_feces.set_neq(note.normal_feces.unwrap_or_default());
                            page.constipation.set_neq(note.constipation.unwrap_or_default());
                            page.diarrhea.set_neq(note.diarrhea.unwrap_or_default());
                            page.bowel_incontinence.set_neq(note.bowel_incontinence.unwrap_or_default());
                            page.hemorrhoid.set_neq(note.hemorrhoid.unwrap_or_default());
                            page.colostomy.set_neq(note.colostomy.unwrap_or_default());
                            page.activity1.set_neq(note.activity1.unwrap_or_default());
                            page.activity2.set_neq(note.activity2.unwrap_or_default());
                            page.activity3.set_neq(note.activity3.unwrap_or_default());
                            page.activity4.set_neq(note.activity4.unwrap_or_default());
                            page.o_p_use.set_neq(note.o_p_use.unwrap_or_default());
                            page.sleep_per_day.set_neq(note.sleep_per_day.unwrap_or_default());
                            page.sleep_hour.set_neq(note.sleep_hour.unwrap_or_default());
                            page.sleep_problems.set_neq(note.sleep_problems.unwrap_or_default());
                            page.sleep_problems_detail.set_neq(note.sleep_problems_detail.unwrap_or_default());
                            page.sleep_med_name.set_neq(note.sleep_med_name.unwrap_or_default());
                            page.sleep_med_name_detail.set_neq(note.sleep_med_name_detail.unwrap_or_default());
                            page.cognitive.set_neq(note.cognitive.unwrap_or_default());
                            page.memory.set_neq(note.memory.unwrap_or_default());
                            page.memory_detail.set_neq(note.memory_detail.unwrap_or_default());
                            page.hearing.set_neq(note.hearing.unwrap_or_default());
                            page.hearing_detail.set_neq(note.hearing_detail.unwrap_or_default());
                            page.eartone.set_neq(note.eartone.unwrap_or_default());
                            page.vision.set_neq(note.vision.unwrap_or_default());
                            page.vision_detail.set_neq(note.vision_detail.unwrap_or_default());
                            page.vision_eyeglasses.set_neq(note.vision_eyeglasses.unwrap_or_default());
                            page.vision_contactlens.set_neq(note.vision_contactlens.unwrap_or_default());
                            page.speech.set_neq(note.speech.unwrap_or_default());
                            page.speech_detail.set_neq(note.speech_detail.unwrap_or_default());
                            page.self_image.set_neq(note.self_image.unwrap_or_default());
                            page.self_image_detail.set_neq(note.self_image_detail.unwrap_or_default());
                            page.self_activity.set_neq(note.self_activity.unwrap_or_default());
                            page.self_activity_detail.set_neq(note.self_activity_detail.unwrap_or_default());
                            page.sickness_effect.set_neq(note.sickness_effect.unwrap_or_default());
                            page.sickness_family.set_neq(note.sickness_family.unwrap_or_default());
                            page.sickness_occupation.set_neq(note.sickness_occupation.unwrap_or_default());
                            page.sickness_education.set_neq(note.sickness_education.unwrap_or_default());
                            page.sickness_other.set_neq(note.sickness_other.unwrap_or_default());
                            page.sickness_other_text.set_neq(note.sickness_other_text.unwrap_or_default());
                            page.period.set_neq(note.period.unwrap_or_default());
                            page.period_normal.set_neq(note.period_normal.unwrap_or_default());
                            page.period_disorders.set_neq(note.period_disorders.unwrap_or_default());
                            page.period_lmp.set_neq(note.period_lmp.unwrap_or_default());
                            page.period_menopause.set_neq(note.period_menopause.unwrap_or_default());
                            page.breast.set_neq(note.breast.unwrap_or_default());
                            page.breast_disorders.set_neq(note.breast_disorders.unwrap_or_default());
                            page.consult.set_neq(note.consult.unwrap_or_default());
                            page.seclude.set_neq(note.seclude.unwrap_or_default());
                            page.medication.set_neq(note.medication.unwrap_or_default());
                            page.medication_detail.set_neq(note.medication_detail.unwrap_or_default());
                            page.religion.set_neq(note.religion.unwrap_or_default());
                            page.coping_stress_other.set_neq(note.coping_stress_other.unwrap_or_default());
                            page.coping_stress_other_detail.set_neq(note.coping_stress_other_detail.unwrap_or_default());
                            page.belief_sickness_behave.set_neq(note.belief_sickness_behave.unwrap_or_default());
                            page.belief_sickness_age.set_neq(note.belief_sickness_age.unwrap_or_default());
                            page.belief_sickness_destiny.set_neq(note.belief_sickness_destiny.unwrap_or_default());
                            page.belief_sickness_other.set_neq(note.belief_sickness_other.unwrap_or_default());
                            page.belief_sickness_other_text.set_neq(note.belief_sickness_other_text.unwrap_or_default());
                            page.belief_believe.set_neq(note.belief_believe.unwrap_or_default());
                            page.belief_believe_text.set_neq(note.belief_believe_text.unwrap_or_default());
                            page.religious_activity.set_neq(note.religious_activity.unwrap_or_default());
                            page.religious_activity_text.set_neq(note.religious_activity_text.unwrap_or_default());

                            page.nurse_name.set_neq(note.nurse_name.or(app.doctor_name()).unwrap_or_default());
                            page.nurse_pos.set_neq(note.nurse_pos.or(app.doctor_entryposition()).unwrap_or_default());
                            page.nurse_licenseno.set_neq(note.nurse_licenseno.or(app.doctor_licenseno()).unwrap_or_default());
                            page.receiver_medication_date.set_neq(note.receiver_medication_date);
                            page.receiver_medication_time.set_neq(note.receiver_medication_time);
                        }
                        if nurse_admission_note_id.is_none() {
                            page.changed.set_neq(true);
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn sex(&self) -> impl Signal<Item = Option<String>> + use<> {
        self.patient.lock_ref().as_ref().patient.signal_cloned().map(|opt| opt.as_ref().and_then(|patient| patient.sex()))
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let show_patient_main = ShowPatientMainCpn::new_with_an(page.an.get_cloned());
        let patient_main = ShowPatientMainCpn::render(false, show_patient_main.clone(), app.clone());
        page.patient.set(show_patient_main);

        html!("div", {
            .children([
                patient_main,
                Self::render_form(page.clone(), app.clone()),
            ])
        })
    }

    fn render_form(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    // load patient data
                    Self::load(page.clone(), app.clone());
                    page.loaded.set(true);
                }
                async {}
            })))
            //.attr("id", "nurse_admission_note")
            .children([
                html!("div", {
                    .class(class::CONF_B)
                    .children([
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class("col-md-1")
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
                                    .class("col-md-11")
                                    .child(html!("h4", {.text("การประเมินสภาพผู้ป่วยแรกรับและแบบแผนสุขภาพ (ยกเว้นผู้ป่วยเด็กอายุ < 1 ปี)")}))
                                }),
                            ])
                        }),
                        html!("p"),
                        html!("div", {
                            .class("row")
                            .child(html!("div", {
                                .class("col-md-12")
                                .child(html!("div", {
                                    .class("card")
                                    .child(html!("div", {
                                        .class("card-body")
                                        .children([
                                            Self::render_hx(page.clone(), app.clone()),
                                            html!("hr"),
                                            Self::render_body(page.clone()),
                                            html!("hr"),
                                            Self::render_mind(page.clone()),
                                            html!("hr"),
                                            Self::render_social(page.clone()),
                                            html!("hr"),
                                            Self::render_tradition(page.clone()),
                                            html!("hr"),
                                            html!("div", {
                                                .class(class::INPUT_GROUP)
                                                .children([
                                                    html!("div", {.class("col-sm-6")}),
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
                                            }),
                                            html!("hr"),
                                            html!("div", {
                                                .class("row")
                                                .child(html!("div", {
                                                    .class(class::COL_SM12_R)
                                                    .children([
                                                        html!("div", {
                                                            .class("float-start")
                                                            .children(PdfButtons::buttons(
                                                                PdfButtons::new(
                                                                    TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteNurse, &app.state().report_coercions()),
                                                                    page.an.clone(),
                                                                    page.nurse_admission_note_id.clone(),
                                                                    page.changed.clone(),
                                                                    clone!(page => move || {serde_json::json!({
                                                                        "id": page.an.get_cloned(),
                                                                        "patient": page.patient.lock_ref().patient.get_cloned(),
                                                                        "note": Self::finalized(page.clone()),
                                                                    }).to_string()})
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
                                                    // (SessionManager::checkPermission('IPD_NURSE_ADDMISSION_NOTE','ADD') && ($nurse_admission_note_id == null)) ||
                                                    // (SessionManager::checkPermission('IPD_NURSE_ADDMISSION_NOTE','EDIT') && ($nurse_admission_note_id != null))
                                                    .child_signal(page.nurse_admission_note_id.signal_cloned().map(clone!(app, page => move |opt| {
                                                        let is_pre_admit = app.is_pre_admit(&page.an.lock_ref());
                                                        let ready = if opt.is_some() {
                                                            app.endpoint_is_allow(&Method::PUT, &EndPoint::IpdAdmissionNoteNurse, is_pre_admit)
                                                        } else {
                                                            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdAdmissionNoteNurse, is_pre_admit)
                                                        };
                                                        ready.then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_R)
                                                                .class_signal("btn-primary", page.changed.signal())
                                                                .class_signal("btn-secondary", not(page.changed.signal()))
                                                                .child(html!("i", {.class(class::FA_SAVE)}))
                                                                .text(" บันทึก")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::submit(page.clone(), app.clone());
                                                                }), not(page.changed.signal()), app.state()))
                                                            })
                                                        })
                                                    })))
                                                }))
                                            }),
                                        ])
                                    }))
                                }))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_hx(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL_SM12_BOLD_C_FS5_T)
                        .child(html!("i", {.class(class::FA_WHEELCHAIR)}))
                        .text(" ประวัติผู้ป่วยแรกรับ")
                    }))
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ผู้ให้ข้อมูล")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.info_patient.clone(), page.changed.clone(), "info_patient"),
                            doms::label_check_for("info_patient","ผู้ป่วย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.info_parent.clone(), page.changed.clone(), "info_parent"),
                            doms::label_check_for("info_parent","บิดา/มารดา"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.info_spouse.clone(), page.changed.clone(), "info_spouse"),
                            doms::label_check_for("info_spouse","สามี/ภรรยา"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.info_child.clone(), page.changed.clone(), "info_child"),
                            doms::label_check_for("info_child","บุตร"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.info_relatives.clone(), page.changed.clone(), "info_relatives"),
                            doms::label_check_for("info_relatives","ญาติ/ผู้ดูแล"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_container(page.info_sender.clone(), page.changed.clone(), "info_sender"),
                            doms::label_check_for("info_sender","ผู้นำส่ง"),
                        ])}),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("อาการสำคัญ")}))
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class("form-control")
                                .attr("rows","3")
                                .apply(mixins::textarea_value_auto_expand(page.chief_complaints.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ประวัติการเจ็บป่วยปัจจุบัน")}))
                        }),
                        html!("div", {
                            .class("col-sm-4")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class("form-control")
                                .attr("rows","3")
                                .apply(mixins::textarea_value_auto_expand(page.medical_history.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("สัญญาณชีพแรกรับ")}))
                            .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderOrder, app.is_pre_admit(&page.an.lock_ref())), |dom| dom
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RT_BLUE)
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", "#vsSelectorModal")
                                    .child(html!("i", {.class(class::FA_HEARTBEAT)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        page.vs_selector_modal.set(Some(VsSelector::new(
                                            false,
                                            page.patient.lock_ref().patient.clone(),
                                            page.vs_admit.clone(),
                                            page.changed.clone(),
                                        )));
                                    }))
                                }))
                            )
                        }),
                        html!("div", {
                            .class("col-sm-10")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class("form-control")
                                .attr("rows","2")
                                .apply(mixins::textarea_value_auto_expand(page.vs_admit.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
            ])
            .child(html!("div", {
                .class("modal")
                .attr("id", "vsSelectorModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.vs_selector_modal.signal_cloned().map(clone!(app => move |opt| {
                    opt.as_ref().map(clone!(app => move |modal| {
                        VsSelector::render(modal.clone(), app)
                    })).or(Some(blank_modal()))
                })))
            }))
        })
    }

    fn render_body(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL_SM12_BOLD_C_FS5_T)
                        .child(html!("i", {.class(class::FA_CHILD)}))
                        .text(" สภาพร่างกายแรกรับ")
                    }))
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ความรู้สึกตัว")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.concious.clone(), page.changed.clone(), "concious1", "รู้สึกตัวดี"),
                            doms::label_check_for("concious1","รู้สึกตัวดี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.concious.clone(), page.changed.clone(), "concious2", "สับสน"),
                            doms::label_check_for("concious2","สับสน"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.concious.clone(), page.changed.clone(), "concious3", "ง่วงซึม"),
                            doms::label_check_for("concious3","ง่วงซึม"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.concious.clone(), page.changed.clone(), "concious4", "ไม่รู้สึกตัว"),
                            doms::label_check_for("concious4","ไม่รู้สึกตัว"),
                        ])}),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ลักษณะการหายใจ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(page.normal_breath.clone(), vec![
                                page.kussmaul.clone(),
                                page.tachypnea.clone(),
                                page.dyspnea.clone(),
                                page.apnea.clone(),
                                page.tube.clone(),
                            ], page.changed.clone(), "normal_breath"),
                            doms::label_check_for("normal_breath","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.kussmaul.clone(),
                                vec![page.normal_breath.clone()],
                                page.changed.clone(), "kussmaul",
                            ),
                            doms::label_check_for("kussmaul","หอบลึก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.tachypnea.clone(),
                                vec![page.normal_breath.clone()],
                                page.changed.clone(), "tachypnea",
                            ),
                            doms::label_check_for("tachypnea","เร็วตื้น"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.dyspnea.clone(),
                                vec![page.normal_breath.clone()],
                                page.changed.clone(), "dyspnea",
                            ),
                            doms::label_check_for("dyspnea","ลำบาก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.apnea.clone(),
                                vec![page.normal_breath.clone()],
                                page.changed.clone(), "apnea",
                            ),
                            doms::label_check_for("apnea","ไม่หายใจ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.tube.clone(),
                                vec![page.normal_breath.clone()],
                                page.changed.clone(), "tube",
                            ),
                            doms::label_check_for("tube","ใส่ท่อช่วยหายใจ"),
                        ])}),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ระบบหัวใจ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(page.normal_hr.clone(), vec![
                                page.arregular.clone(),
                                page.weakness.clone(),
                                page.arrhythmia.clone(),
                                page.chestpain.clone(),
                                page.pacemaker.clone(),
                                page.cardio_other.clone()
                            ], page.changed.clone(), "normal_hr"),
                            doms::label_check_for("normal_hr","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.arregular.clone(),
                                vec![page.normal_hr.clone()],
                                page.changed.clone(), "arregular",
                            ),
                            doms::label_check_for("arregular","อัตราการเต้นไม่สม่ำเสมอ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.weakness.clone(),
                                vec![page.normal_hr.clone()],
                                page.changed.clone(), "weakness",
                            ),
                            doms::label_check_for("weakness","ชีพจรเบา"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.arrhythmia.clone(),
                                vec![page.normal_hr.clone()],
                                page.changed.clone(), "arrhythmia",
                            ),
                            doms::label_check_for("arrhythmia","ใจสั่น"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.chestpain.clone(),
                                vec![page.normal_hr.clone()],
                                page.changed.clone(), "chestpain",
                            ),
                            doms::label_check_for("chestpain","เจ็บหน้าอก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.pacemaker.clone(),
                                vec![page.normal_hr.clone()],
                                page.changed.clone(), "pacemaker",
                            ),
                            doms::label_check_for("pacemaker","ใส่เครื่องกระตุ้นหัวใจ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_toggle_texts_container(
                                page.cardio_other.clone(),
                                vec![page.cardio_other_text.clone()],
                                vec![page.normal_hr.clone()],
                                page.changed.clone(), "cardio_other",
                            ),
                            doms::label_check_for("cardio_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.cardio_other_text.clone(),
                            page.cardio_other.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การไหลเวียนโลหิต")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(page.normal_cir.clone(), vec![
                                page.pale.clone(),
                                page.cyanosis.clone(),
                                page.generalized_edema.clone(),
                                page.localized_edema.clone(),
                                page.localized_edema_text.clone(),
                                page.pitting_edema.clone(),
                                page.pitting_edema_text.clone(),
                                page.circulation_orther.clone(),
                                page.circulation_orther_text.clone(),
                            ], page.changed.clone(), "normal_cir"),
                            doms::label_check_for("normal_cir","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.pale.clone(),
                                vec![page.normal_cir.clone()],
                                page.changed.clone(), "pale",
                            ),
                            doms::label_check_for("pale","ซีด"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.cyanosis.clone(),
                                vec![page.normal_cir.clone()],
                                page.changed.clone(), "cyanosis",
                            ),
                            doms::label_check_for("cyanosis","เขียวปลายมือ-เท้า"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.generalized_edema.clone(),
                                vec![page.normal_cir.clone()],
                                page.changed.clone(), "generalized_edema",
                            ),
                            doms::label_check_for("generalized_edema","บวมทั่วตัว"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_binding_toggle_texts_container(
                                page.localized_edema.clone(),
                                vec![page.localized_edema_text.clone()],
                                vec![page.normal_cir.clone()],
                                page.changed.clone(), "localized_edema",
                            ),
                            doms::label_check_for("localized_edema","บวมเฉพาะที่"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.localized_edema_text.clone(),
                            page.localized_edema.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_toggle_texts_container(
                                page.pitting_edema.clone(),
                                vec![page.pitting_edema_text.clone()],
                                vec![page.normal_cir.clone()],
                                page.changed.clone(), "pitting_edema",
                            ),
                            doms::label_check_for("pitting_edema","บวมกดบุ๋ม"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.pitting_edema_text.clone(),
                            page.pitting_edema.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_toggle_texts_container(
                                page.circulation_orther.clone(),
                                vec![page.circulation_orther_text.clone()],
                                vec![page.normal_cir.clone()],
                                page.changed.clone(), "circulation_orther",
                            ),
                            doms::label_check_for("circulation_orther","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.circulation_orther_text.clone(),
                            page.circulation_orther.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("สภาพผิวหนัง")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(page.normal_skin.clone(), vec![
                                page.dry.clone(),
                                page.bruise.clone(),
                                page.erythema.clone(),
                                page.abscess.clone(),
                                page.joudice.clone(),
                                page.skin_other.clone(),
                                page.skin_other_text.clone(),
                            ], page.changed.clone(), "normal_skin"),
                            doms::label_check_for("normal_skin","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.dry.clone(),
                                vec![page.normal_skin.clone()],
                                page.changed.clone(), "dry",
                            ),
                            doms::label_check_for("dry","แห้งแตก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.bruise.clone(),
                                vec![page.normal_skin.clone()],
                                page.changed.clone(), "bruise",
                            ),
                            doms::label_check_for("bruise","บาง ช้ำ หลุดลอกง่าย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.erythema.clone(),
                                vec![page.normal_skin.clone()],
                                page.changed.clone(), "erythema",
                            ),
                            doms::label_check_for("erythema","ผื่นแดง"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.abscess.clone(),
                                vec![page.normal_skin.clone()],
                                page.changed.clone(), "abscess",
                            ),
                            doms::label_check_for("abscess","แผล ฝี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.joudice.clone(),
                                vec![page.normal_skin.clone()],
                                page.changed.clone(), "joudice",
                            ),
                            doms::label_check_for("joudice","เหลือง"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_toggle_texts_container(
                                page.skin_other.clone(),
                                vec![page.skin_other_text.clone()],
                                vec![page.normal_skin.clone()],
                                page.changed.clone(), "skin_other",
                            ),
                            doms::label_check_for("skin_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.skin_other_text.clone(),
                            page.skin_other.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ความเจ็บปวด")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_toggle_texts_container(page.pain.clone(), vec![
                                page.pain_charac.clone(),
                                page.pain_charac_text.clone(),
                                page.pain_score.clone(),
                            ], page.changed.clone(), "pain_n","ไม่มี"),
                            doms::label_check_for("pain_n","ไม่มี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_binding_texts_container(
                                page.pain.clone(),
                                vec![page.location.clone()],
                                page.changed.clone(), "pain_y", "มี"),
                            doms::label_check_for("pain_y","มี บริเวณ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.location.clone(),
                            page.pain.clone(), "มี",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_OFS3)
                            .child(html!("b", {.text("ลักษณะการเจ็บปวด")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_disable_by_not_container(
                                page.pain_charac.clone(),
                                page.pain.clone(), "มี",
                                page.changed.clone(), "pain_charac_s","ครั้งคราว",
                            ),
                            doms::label_check_for("pain_charac_s","ครั้งคราว"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .child(html!("div", {.class(class::FORM_CHK_COL_SM1_OFS5).children([
                        doms::radio_disable_by_not_container(
                            page.pain_charac.clone(),
                            page.pain.clone(), "มี",
                            page.changed.clone(), "pain_charac_a", "ตลอดเวลา",
                        ),
                        doms::label_check_for("pain_charac_a","ตลอดเวลา"),
                    ])}))
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS5).children([
                            doms::radio_binding_texts_disable_by_not_container(
                                page.pain_charac.clone(),
                                vec![page.pain_charac_text.clone()],
                                page.pain.clone(), "มี",
                                page.changed.clone(), "pain_charac_o", "อื่นๆ",
                            ),
                            doms::label_check_for("pain_charac_o","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.pain_charac_text.clone(),
                            page.pain_charac.clone(), "อื่นๆ",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM1_OFS5)
                            .child(html!("b", {.text("Pain Score")}))
                        }),
                        doms::text_disable_by_not_container(
                            page.pain_score.clone(),
                            page.pain.clone(), "มี",
                            page.changed.clone(), "col-sm-1",
                            Some(4),
                        ),
                        html!("div", {
                            .class("col-sm-3")
                            .child(html!("label", {.text("คะแนน")}))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_mind(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL_SM12_BOLD_C_FS5_T)
                        .child(html!("i", {.class(class::FA_HEART)}))
                        .text(" สภาพจิตใจแรกรับ")
                    }))
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {
                            .class(class::FORM_CHK_COL_SM3_OFS1)
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "no_mental_state")
                                    .apply(mixins::checkbox_toggle(page.no_mental_state.clone(), page.changed.clone(), "Y", ""))
                                    .future(page.no_mental_state.signal_cloned().for_each(clone!(page => move |yes| {
                                        if yes.is_empty() {
                                            page.no_mental_state_text.set_neq(String::new());
                                            page.normal_behav.set_neq(String::from("Y"));
                                            page.normal_emotional.set_neq(String::from("Y"));
                                            page.no_anxiety.set_neq(String::from("Y"));
                                            page.spiritual_no.set_neq(String::from("Y"));
                                        } else {
                                            page.normal_behav.set_neq(String::new());
                                            page.agitate.set_neq(String::new());
                                            page.aggressive.set_neq(String::new());
                                            page.depression.set_neq(String::new());
                                            page.madness.set_neq(String::new());
                                            page.behaviour_other.set_neq(String::new());
                                            //page.behaviour_other_text.set_neq(String::new());
                                            page.normal_emotional.set_neq(String::new());
                                            page.angry.set_neq(String::new());
                                            page.moody.set_neq(String::new());
                                            page.anxiety.set_neq(String::new());
                                            page.fear.set_neq(String::new());
                                            page.emotional_other.set_neq(String::new());
                                            //page.emotional_other_text.set_neq(String::new());
                                            page.no_anxiety.set_neq(String::new());
                                            page.study.set_neq(String::new());
                                            page.family.set_neq(String::new());
                                            page.economy.set_neq(String::new());
                                            page.habitation.set_neq(String::new());
                                            page.illness.set_neq(String::new());
                                            page.spiritual_no.set_neq(String::new());
                                            page.spiritual_back_home.set_neq(String::new());
                                            page.spiritual_need_family.set_neq(String::new());
                                            page.spiritual_other.set_neq(String::new());
                                            //page.spiritual_other_text.set_neq(String::new());
                                            page.spiritual_cant_rated.set_neq(String::new());
                                            //page.spiritual_cant_rated_text.set_neq(String::new());
                                        }
                                        async {}
                                    })))
                                }),
                                doms::label_check_for("no_mental_state","ประเมินสภาพจิตใจไม่ได้เนื่องจาก"),
                            ])
                            // .attr("onchange","onchange_no_mental_state()")
                        }),
                        doms::textarea_disable_by_not_container(
                            page.no_mental_state_text.clone(),
                            page.no_mental_state.clone(), "Y",
                            page.changed.clone(), "col-sm-7",
                            Some(400),
                        ),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ด้านพฤติกรรม")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.normal_behav.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "normal_behav",
                            ),
                            doms::label_check_for("normal_behav","ร่วมมือดี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_disable_by_not_container(
                                page.agitate.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "agitate",
                            ),
                            doms::label_check_for("agitate","กระวนกระวาย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.aggressive.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "aggressive",
                            ),
                            doms::label_check_for("aggressive","ก้าวร้าว"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.depression.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "depression",
                            ),
                            doms::label_check_for("depression","ซึมเศร้า"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_disable_by_not_container(
                                page.madness.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "madness",
                            ),
                            doms::label_check_for("madness","เอะอะโวยวาย"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_texts_disable_by_not_container(
                                page.behaviour_other.clone(),
                                vec![page.behaviour_other_text.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "behaviour_other",
                            ),
                            doms::label_check_for("behaviour_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.behaviour_other_text.clone(),
                            page.behaviour_other.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ด้านอารมณ์")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.normal_emotional.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "normal_emotional",
                            ),
                            doms::label_check_for("normal_emotional","สงบ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.angry.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "angry",
                            ),
                            doms::label_check_for("angry","โกรธ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.moody.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "moody",
                            ),
                            doms::label_check_for("moody","หงุดหงิด"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.anxiety.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "anxiety",
                            ),
                            doms::label_check_for("anxiety","กังวลใจ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.fear.clone(),
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "fear",
                            ),
                            doms::label_check_for("fear","หวาดกลัว"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_texts_disable_by_not_container(
                                page.emotional_other.clone(),
                                vec![page.emotional_other_text.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "emotional_other",
                            ),
                            doms::label_check_for("emotional_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.emotional_other_text.clone(),
                            page.emotional_other.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ความกังวลใจ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(page.no_anxiety.clone(), vec![
                                page.study.clone(),
                                page.family.clone(),
                                page.economy.clone(),
                                page.habitation.clone(),
                                page.illness.clone(),
                            ], page.no_mental_state.clone(), "", page.changed.clone(), "no_anxiety"),
                            doms::label_check_for("no_anxiety","ปฎิเสธ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(
                                page.study.clone(),
                                vec![page.no_anxiety.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "study",
                            ),
                            doms::label_check_for("study","การเรียน"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(
                                page.family.clone(),
                                vec![page.no_anxiety.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "family",
                            ),
                            doms::label_check_for("family","ครอบครัว"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(
                                page.economy.clone(),
                                vec![page.no_anxiety.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "economy",
                            ),
                            doms::label_check_for("economy","ค่าใช้จ่าย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(
                                page.habitation.clone(),
                                vec![page.no_anxiety.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "habitation",
                            ),
                            doms::label_check_for("habitation","ที่อยู่อาศัย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(
                                page.illness.clone(),
                                vec![page.no_anxiety.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "illness",
                            ),
                            doms::label_check_for("illness","ความเจ็บป่วย"),
                        ])}),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .child(html!("b", {.text("ความต้องการด้านจิตวิญญาณ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(page.spiritual_no.clone(), vec![
                                page.spiritual_back_home.clone(),
                                page.spiritual_need_family.clone(),
                                page.spiritual_other.clone(),
                                page.spiritual_other_text.clone(),
                                page.spiritual_cant_rated.clone(),
                                page.spiritual_cant_rated_text.clone(),
                            ], page.no_mental_state.clone(), "", page.changed.clone(), "spiritual_no"),
                            doms::label_check_for("spiritual_no","ไม่ต้องการ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(
                                page.spiritual_back_home.clone(),
                                vec![page.spiritual_no.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "spiritual_back_home",
                            ),
                            doms::label_check_for("spiritual_back_home","บ่นอยากกลับบ้านมาก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM3).children([
                            doms::checkbox_toggle_texts_disable_by_not_container(
                                page.spiritual_need_family.clone(),
                                vec![page.spiritual_no.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "spiritual_need_family",
                            ),
                            doms::label_check_for("spiritual_need_family","ถามถึงบุคคลในครอบครัวบ่อยๆ"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM2_OFS3).children([
                            doms::checkbox_binding_toggle_texts_disable_by_not_container(
                                page.spiritual_other.clone(),
                                vec![page.spiritual_other_text.clone()],
                                vec![page.spiritual_no.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "spiritual_other"),
                            doms::label_check_for("spiritual_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.spiritual_other_text.clone(),
                            page.spiritual_other.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM2_OFS3).children([
                            doms::checkbox_binding_toggle_texts_disable_by_not_container(
                                page.spiritual_cant_rated.clone(),
                                vec![page.spiritual_cant_rated_text.clone()],
                                vec![page.spiritual_no.clone()],
                                page.no_mental_state.clone(), "",
                                page.changed.clone(), "spiritual_cant_rated"),
                            doms::label_check_for("spiritual_cant_rated","ประเมินไม่ได้")
                        ])}),
                        doms::text_disable_by_not_container(
                            page.spiritual_cant_rated_text.clone(),
                            page.spiritual_cant_rated.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            None,
                        ),
                    ])
                }),
            ])
        })
    }
    fn render_social(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL_SM12_BOLD_C_FS5_T)
                        .child(html!("i", {.class(class::FA_USERS)}))
                        .text(" สภาพสังคมและเศรษฐานะ")
                    }))
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การศึกษา")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_container(page.education.clone(), page.changed.clone(), "education_n", "ไม่ได้รับ"),
                            doms::label_check_for("education_n","ไม่ได้รับ/ยังไม่ได้รับ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_binding_texts_container(
                                page.education.clone(),
                                vec![page.education_result.clone()],
                                page.changed.clone(), "education_y", "ได้รับ",
                            ),
                            doms::label_check_for("education_y","ได้รับ(ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.education_result.clone(),
                            page.education.clone(), "ได้รับ",
                            page.changed.clone(), "col-sm-3",
                            Some(50),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("อาชีพ(ระบุ)")}))
                        }),
                        doms::texts_container(
                            page.occupation.clone(),
                            page.changed.clone(),
                            "col-sm-6",
                            Some(70),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("รายได้")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.income.clone(), page.changed.clone(), "income_y","เพียงพอ"),
                            doms::label_check_for("income_y","เพียงพอ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_container(page.income.clone(), page.changed.clone(), "income_n","ไม่เพียงพอ"),
                            doms::label_check_for("income_n","ไม่เพียงพอ"),
                        ])}),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ผู้ให้ความช่วยเหลือดูแล")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.self_value.clone(), page.changed.clone(), "self"),
                            doms::label_check_for("self","ตนเอง"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_container(page.person_family.clone(), page.changed.clone(), "person_family"),
                            doms::label_check_for("person_family","บุคคลในครอบครัว"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.neighbor.clone(), page.changed.clone(), "neighbor"),
                            doms::label_check_for("neighbor","เพื่อนบ้าน"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_texts_container(
                                page.assistant_other.clone(),
                                vec![page.assistant_other_text.clone()],
                                page.changed.clone(), "assistant_other",
                            ),
                            doms::label_check_for("assistant_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.assistant_other_text.clone(),
                            page.assistant_other.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        )
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_OFS2)
                            .child(html!("b", {.text("อาชีพผู้ดูแล(ระบุ)")}))
                        }),
                        doms::texts_container(
                            page.assistant_occupation.clone(),
                            page.changed.clone(),
                            "col-sm-6",
                            Some(70),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_tradition(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL_SM12_BOLD_C_FS5_T)
                        .child(html!("i", {.class(class::FA_PASTE)}))
                        .text(" แบบแผนสุขภาพ")
                    }))
                }),
                html!("p"),
                Self::render_health(page.clone()),
                html!("hr"),
                Self::render_nutrition(page.clone()),
                html!("hr"),
                Self::render_elimination(page.clone()),
                html!("hr"),
                Self::render_activity(page.clone()),
                html!("hr"),
                Self::render_sleep(page.clone()),
                html!("hr"),
                Self::render_perception(page.clone()),
                html!("hr"),
                Self::render_self(page.clone()),
                html!("hr"),
                Self::render_relationship(page.clone()),
                html!("hr"),
                Self::render_reproduction(page.clone()),
                html!("hr"),
                Self::render_stress(page.clone()),
                html!("hr"),
                Self::render_value(page.clone()),
            ])
        })
    }

    fn render_health(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("การรับรู้สุขภาพ และการดูแลสุขภาพ"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การดูแลตนเอง")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_container(page.clinic.clone(), page.changed.clone(), "clinic"),
                            doms::label_check_for("clinic","ไป รพ./คลินิก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM4).children([
                            doms::checkbox_container(page.buy_medicine.clone(), page.changed.clone(), "buy_medicine"),
                            doms::label_check_for("buy_medicine","ซื้อยารับประทานเอง"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("พฤติกรรมเสี่ยง")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_toggle_texts_container(page.no_risk.clone(), vec![
                                page.smoking.clone(),
                                page.alcohol.clone(),
                                page.medication_used.clone(),
                            ], page.changed.clone(), "no_risk", "Y"),
                            doms::label_check_for("no_risk","ปฏิเสธ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_container(page.no_risk.clone(), page.changed.clone(), "have_risk", ""),
                            doms::label_check_for("have_risk","มี"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_texts_disable_by_not_container(page.smoking.clone(), vec![
                                page.smoke_year.clone(),
                                page.smoke_frequency.clone(),
                                page.smoke_stopped.clone(),
                            ], page.no_risk.clone(), "", page.changed.clone(), "smoking"),
                            doms::label_check_for("smoking","สูบบุหรี่"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.smoke_year.clone(),
                            page.smoking.clone(), "Y",
                            page.changed.clone(), "col-sm-1",
                            Some(20),
                        ),
                        html!("label", {.class("col-sm-1").text("ปี ปริมาณ")}),
                        doms::text_disable_by_not_container(
                            page.smoke_frequency.clone(),
                            page.smoking.clone(), "Y",
                            page.changed.clone(), "col-sm-1",
                            Some(20),
                        ),
                        html!("label", {.class("col-sm-1").text("/วัน เลิกเมื่อ")}),
                        doms::text_disable_by_not_container(
                            page.smoke_stopped.clone(),
                            page.smoking.clone(), "Y",
                            page.changed.clone(), "col-sm-2",
                            Some(30),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_texts_disable_by_not_container(page.alcohol.clone(), vec![
                                page.alc_year.clone(),
                                page.alc_frequency.clone(),
                                page.alc_stopped.clone(),
                            ], page.no_risk.clone(), "", page.changed.clone(), "alcohol"),
                            doms::label_check_for("alcohol","ดื่มสุรา"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.alc_year.clone(),
                            page.alcohol.clone(), "Y",
                            page.changed.clone(), "col-sm-1",
                            Some(20),
                        ),
                        html!("label", {.class("col-sm-1").text("ปี ปริมาณ")}),
                        doms::text_disable_by_not_container(
                            page.alc_frequency.clone(),
                            page.alcohol.clone(), "Y",
                            page.changed.clone(), "col-sm-1",
                            Some(20),
                        ),
                        html!("label", {.class("col-sm-1").text("/วัน เลิกเมื่อ")}),
                        doms::text_disable_by_not_container(
                            page.alc_stopped.clone(),
                            page.alcohol.clone(), "Y",
                            page.changed.clone(), "col-sm-2",
                            Some(30),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_texts_disable_by_not_container(page.medication_used.clone(), vec![
                                page.med_name.clone(),
                                page.med_year.clone(),
                                page.med_frequency.clone(),
                                page.med_stopped.clone(),
                            ], page.no_risk.clone(), "", page.changed.clone(), "medication_used"),
                            doms::label_check_for("medication_used","ยา (ระบุ)"),
                        ])}),
                        doms::textarea_disable_by_not_container(
                            page.med_name.clone(),
                            page.medication_used.clone(), "Y",
                            page.changed.clone(), "col-sm-6",
                            None,
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {.class(class::COL_SM1_OFS2_R).text("ระยะเวลาที่ใช้")}),
                        doms::text_disable_by_not_container(
                            page.med_year.clone(),
                            page.medication_used.clone(), "Y",
                            page.changed.clone(), "col-sm-1",
                            Some(30),
                        ),
                        html!("label", {.class(class::COL_SM1_R).text("ปริมาณ")}),
                        doms::text_disable_by_not_container(
                            page.med_frequency.clone(),
                            page.medication_used.clone(), "Y",
                            page.changed.clone(), "col-sm-1",
                            Some(20),
                        ),
                        html!("label", {.class("col-sm-1").text("/วัน เลิกเมื่อ")}),
                        doms::text_disable_by_not_container(
                            page.med_stopped.clone(),
                            page.medication_used.clone(), "Y",
                            page.changed.clone(), "col-sm-2",
                            Some(30),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_nutrition(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("อาหาร และการเผาผลาญอาหาร"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .child(html!("div", {.class(class::FORM_CHK_COL_SM2_OFS2).children([
                        doms::radio_container(page.diet_regular.clone(), page.changed.clone(), "diet_regu","อาหารทั่วไป"),
                        doms::label_check_for("diet_regu","อาหารทั่วไป"),
                    ])}))
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM2_OFS2).children([
                            doms::radio_binding_texts_container(
                                page.diet_regular.clone(),
                                vec![page.diet_spec.clone()],
                                page.changed.clone(), "diet_sp", "อาหารเฉพาะโรค",
                            ),
                            doms::label_check_for("diet_sp","อาหารเฉพาะโรค (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.diet_spec.clone(),
                            page.diet_regular.clone(), "อาหารเฉพาะโรค",
                            page.changed.clone(), "col-sm-5",
                            Some(100),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM3_R)
                            .child(html!("b", {.text("ปัญหาการรับประทานอาหาร")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(page.nutrition_risk.clone(), vec![
                                page.loss_appetite.clone(),
                                page.dysphagia.clone(),
                                page.loss_gustation.clone(),
                                page.denture.clone(),
                                page.nutrition_risk_other.clone(),
                            ], page.changed.clone(), "nutrition_risk"),
                            doms::label_check_for("nutrition_risk","ไม่มี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.loss_appetite.clone(),
                                vec![page.nutrition_risk.clone()],
                                page.changed.clone(), "loss_appetite",
                            ),
                            doms::label_check_for("loss_appetite","เบื่ออาหาร"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.dysphagia.clone(),
                                vec![page.nutrition_risk.clone()],
                                page.changed.clone(), "dysphagia",
                            ),
                            doms::label_check_for("dysphagia","เคี้ยว/กลืนลำบาก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.loss_gustation.clone(),
                                vec![page.nutrition_risk.clone()],
                                page.changed.clone(), "loss_gustation",
                            ),
                            doms::label_check_for("loss_gustation","ไม่รู้รสกลิ่น"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.denture.clone(),
                                vec![page.nutrition_risk.clone()],
                                page.changed.clone(), "denture",
                            ),
                            doms::label_check_for("denture","ใส่ฟันปลอม"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS3).children([
                            doms::checkbox_binding_toggle_texts_container(
                                page.nutrition_risk_other.clone(),
                                vec![page.nutrition_risk_other_text.clone()],
                                vec![page.nutrition_risk.clone()],
                                page.changed.clone(), "nutrition_risk_other",
                            ),
                            doms::label_check_for("nutrition_risk_other","อื่นๆ (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.nutrition_risk_other_text.clone(),
                            page.nutrition_risk_other.clone(), "Y",
                            page.changed.clone(), "col-sm-5",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_elimination(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("การขับถ่าย"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ปัสสาวะ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(page.normal_urine.clone(), vec![
                                page.dysuria.clone(),
                                page.incontinence.clone(),
                                page.staining.clone(),
                                page.hematuria.clone(),
                                page.catheter.clone(),
                            ], page.changed.clone(), "normal_urine"),
                            doms::label_check_for("normal_urine","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.dysuria.clone(),
                                vec![page.normal_urine.clone()],
                                page.changed.clone(), "dysuria",
                            ),
                            doms::label_check_for("dysuria","แสบขัด"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.incontinence.clone(),
                                vec![page.normal_urine.clone()],
                                page.changed.clone(), "incontinence",
                            ),
                            doms::label_check_for("incontinence","กลั้นไม่ได้"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.staining.clone(),
                                vec![page.normal_urine.clone()],
                                page.changed.clone(), "staining",
                            ),
                            doms::label_check_for("staining","ลำบาก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.hematuria.clone(),
                                vec![page.normal_urine.clone()],
                                page.changed.clone(), "hematuria",
                            ),
                            doms::label_check_for("hematuria","เป็นเลือด"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.catheter.clone(),
                                vec![page.normal_urine.clone()],
                                page.changed.clone(), "catheter",
                            ),
                            doms::label_check_for("catheter","สายสวนปัสสาวะ"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("อุจจาระ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(page.normal_feces.clone(), vec![
                                page.constipation.clone(),
                                page.diarrhea.clone(),
                                page.bowel_incontinence.clone(),
                                page.hemorrhoid.clone(),
                                page.colostomy.clone(),
                            ], page.changed.clone(), "normal_feces"),
                            doms::label_check_for("normal_feces","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.constipation.clone(),
                                vec![page.normal_feces.clone()],
                                page.changed.clone(), "constipation",
                            ),
                            doms::label_check_for("constipation","ท้องผูก"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.diarrhea.clone(),
                                vec![page.normal_feces.clone()],
                                page.changed.clone(), "diarrhea",
                            ),
                            doms::label_check_for("diarrhea","ท้องเสียบ่อย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_toggle_texts_container(
                                page.bowel_incontinence.clone(),
                                vec![page.normal_feces.clone()],
                                page.changed.clone(), "bowel_incontinence",
                            ),
                            doms::label_check_for("bowel_incontinence","กลั้นไม่ได้"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.hemorrhoid.clone(),
                                vec![page.normal_feces.clone()],
                                page.changed.clone(), "hemorrhoid",
                            ),
                            doms::label_check_for("hemorrhoid","ริดสีดวงทวาร"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_toggle_texts_container(
                                page.colostomy.clone(),
                                vec![page.normal_feces.clone()],
                                page.changed.clone(), "colostomy",
                            ),
                            doms::label_check_for("colostomy","ถ่ายทางหน้าท้อง"),
                        ])}),
                    ])
                }),
            ])
        })
    }

    fn render_activity(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("กิจกรรมและออกกำลังกาย"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_container(page.activity1.clone(), page.changed.clone(), "activity1"),
                            doms::label_check_for("activity1","ทำได้เอง"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.activity2.clone(), page.changed.clone(), "activity2"),
                            doms::label_check_for("activity2","ต้องมีคนช่วย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.activity3.clone(), page.changed.clone(), "activity3"),
                            doms::label_check_for("activity3","ทำเองไม่ได้"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_binding_texts_container(
                                page.activity4.clone(),
                                vec![page.o_p_use.clone()],
                                page.changed.clone(), "activity4",
                            ),
                            doms::label_check_for("activity4","ใช้กายอุปกรณ์"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.o_p_use.clone(),
                            page.activity4.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_sleep(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("การพักผ่อนนอนหลับ"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                            doms::checkbox_binding_texts_container(
                                page.sleep_per_day.clone(),
                                vec![page.sleep_hour.clone()],
                                page.changed.clone(), "sleep_per_day",
                            ),
                            doms::label_check_for("sleep_per_day","ปกติ วันละ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.sleep_hour.clone(),
                            page.sleep_per_day.clone(), "Y",
                            page.changed.clone(), "col-sm-1",
                            Some(20),
                        ),
                        html!("div", {.class("col-sm-1").child(html!("label", {.text("ชม.")}))}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_binding_texts_container(
                                page.sleep_problems.clone(),
                                vec![page.sleep_problems_detail.clone()],
                                page.changed.clone(), "sleep_problems",
                            ),
                            doms::label_check_for("sleep_problems","ปัญหาการนอน (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.sleep_problems_detail.clone(),
                            page.sleep_problems.clone(), "Y",
                            page.changed.clone(), "col-sm-4",
                            Some(100),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การใช้ยานอนหลับ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.sleep_med_name.clone(), page.changed.clone(), "sleep_med_name1","ไม่เคย"),
                            doms::label_check_for("sleep_med_name1","ไม่เคย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_container(page.sleep_med_name.clone(), page.changed.clone(), "sleep_med_name2","เป็นครั้งคราว"),
                            doms::label_check_for("sleep_med_name2","เป็นครั้งคราว"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_binding_texts_container(
                                page.sleep_med_name.clone(),
                                vec![page.sleep_med_name_detail.clone()],
                                page.changed.clone(), "sleep_med_name3","เป็นประจำ",
                            ),
                            doms::label_check_for("sleep_med_name3","เป็นประจำ ยา (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.sleep_med_name_detail.clone(),
                            page.sleep_med_name.clone(), "เป็นประจำ",
                            page.changed.clone(), "col-sm-4",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_perception(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("สติปัญญาและการรับรู้"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การรับรู้")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.cognitive.clone(), page.changed.clone(), "cognitive1","ตรง"),
                            doms::label_check_for("cognitive1","ตรง"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_container(page.cognitive.clone(), page.changed.clone(), "cognitive2","ไม่ตรง"),
                            doms::label_check_for("cognitive2","ไม่ตรง"),
                        ])}),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ความจำ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.memory.clone(), page.changed.clone(), "memory1","ปกติ"),
                            doms::label_check_for("memory1","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_binding_texts_container(
                                page.memory.clone(),
                                vec![page.memory_detail.clone()],
                                page.changed.clone(), "memory2", "ผิดปกติ",
                            ),
                            doms::label_check_for("memory2","ผิดปกติ (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.memory_detail.clone(),
                            page.memory.clone(), "ผิดปกติ",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การได้ยิน")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "radio")
                                .class("form-check-input")
                                .attr("id", "hearing1")
                                .apply(mixins::radio_match(page.hearing.clone(), page.changed.clone(), "ปกติ"))
                                .future(page.hearing.signal_cloned().for_each(clone!(page => move |yes| {
                                    if yes == "ปกติ" {
                                        page.hearing_detail.set_neq(String::new());
                                        page.eartone.set_neq(String::new());
                                    }
                                    async {}
                                })))
                            }),
                            doms::label_check_for("hearing1","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_binding_texts_container(
                                page.hearing.clone(),
                                vec![page.hearing_detail.clone()],
                                page.changed.clone(), "hearing2", "ผิดปกติ",
                            ),
                            doms::label_check_for("hearing2","ผิดปกติ (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.hearing_detail.clone(),
                            page.hearing.clone(), "ผิดปกติ",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                        html!("div", {.class(class::FORM_CHK_COL_SM2_R).children([
                            doms::checkbox_disable_by_not_container(
                                page.eartone.clone(),
                                page.hearing.clone(), "ผิดปกติ",
                                page.changed.clone(), "eartone",
                            ),
                            doms::label_check_for("eartone","ใช้เครื่องช่วยฟัง"),
                        ])}),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การมองเห็น")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "radio")
                                .class("form-check-input")
                                .attr("id", "vision1")
                                .apply(mixins::radio_match(page.vision.clone(), page.changed.clone(), "ปกติ"))
                                .future(page.vision.signal_cloned().for_each(clone!(page => move |yes| {
                                    if yes == "ปกติ" {
                                        page.vision_detail.set_neq(String::new());
                                        page.vision_eyeglasses.set_neq(String::new());
                                        page.vision_contactlens.set_neq(String::new());
                                    }
                                    async {}
                                })))
                            }),
                            doms::label_check_for("vision1","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_binding_texts_container(
                                page.vision.clone(),
                                vec![page.vision_detail.clone()],
                                page.changed.clone(), "vision2", "ผิดปกติ",
                            ),
                            doms::label_check_for("vision2","ผิดปกติ (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.vision_detail.clone(),
                            page.vision.clone(), "ผิดปกติ",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                        html!("div", {.class(class::FORM_CHK_COL_SM1_R).children([
                            doms::checkbox_disable_by_not_container(
                                page.vision_eyeglasses.clone(),
                                page.vision.clone(), "ผิดปกติ",
                                page.changed.clone(), "vision_eyeglasses",
                            ),
                            doms::label_check_for("vision_eyeglasses","แว่นตา"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_disable_by_not_container(
                                page.vision_contactlens.clone(),
                                page.vision.clone(), "ผิดปกติ",
                                page.changed.clone(), "vision_contactlens",
                            ),
                            doms::label_check_for("vision_contactlens","คอนแทคเลนส์"),
                        ])}),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("การพูด")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.speech.clone(), page.changed.clone(), "speech1","ปกติ"),
                            doms::label_check_for("speech1","ปกติ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_binding_texts_container(
                                page.speech.clone(),
                                vec![page.speech_detail.clone()],
                                page.changed.clone(), "speech2", "ผิดปกติ",
                            ),
                            doms::label_check_for("speech2","ผิดปกติ (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.speech_detail.clone(),
                            page.speech.clone(), "ผิดปกติ",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_self(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("การรับรู้ตนเองและอัตมโนทัศน์"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("กระทบต่อภาพลักษณ์")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.self_image.clone(), page.changed.clone(), "self_image1","ไม่มี"),
                            doms::label_check_for("self_image1","ไม่มี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_binding_texts_container(
                                page.self_image.clone(),
                                vec![page.self_image_detail.clone()],
                                page.changed.clone(), "self_image2", "มี",
                            ),
                            doms::label_check_for("self_image2","มี"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.self_image_detail.clone(),
                            page.self_image.clone(), "มี",
                            page.changed.clone(), "col-sm-4",
                            Some(100),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("กระทบต่อความสามารถ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.self_activity.clone(), page.changed.clone(), "self_activity1","ไม่มี"),
                            doms::label_check_for("self_activity1","ไม่มี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_binding_texts_container(
                                page.self_activity.clone(),
                                vec![page.self_activity_detail.clone()],
                                page.changed.clone(), "self_activity2", "มี",
                            ),
                            doms::label_check_for("self_activity2","มี"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.self_activity_detail.clone(),
                            page.self_activity.clone(), "มี",
                            page.changed.clone(), "col-sm-4",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_relationship(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("บทบาทและสัมพันธภาพ"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("ความเจ็บป่วยมีผลกระทบ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "radio")
                                .class("form-check-input")
                                .attr("id", "sickness_effect1")
                                .apply(mixins::radio_match(page.sickness_effect.clone(), page.changed.clone(), "ไม่มี"))
                                .future(page.sickness_effect.signal_cloned().for_each(clone!(page => move |yes| {
                                    if yes == "ไม่มี" {
                                        page.sickness_family.set_neq(String::new());
                                        page.sickness_occupation.set_neq(String::new());
                                        page.sickness_education.set_neq(String::new());
                                        page.sickness_other.set_neq(String::new());
                                    }
                                    async {}
                                })))
                            }),
                            doms::label_check_for("sickness_effect1","ไม่มี"),
                        ])}),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM2_OFS2).children([
                            doms::radio_container(page.sickness_effect.clone(), page.changed.clone(), "sickness_effect2","มีผลกระทบต่อ"),
                            doms::label_check_for("sickness_effect2","มีผลกระทบต่อ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.sickness_family.clone(),
                                page.sickness_effect.clone(), "มีผลกระทบต่อ",
                                page.changed.clone(), "sickness_family",
                            ),
                            doms::label_check_for("sickness_family","ครอบครัว"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.sickness_occupation.clone(),
                                page.sickness_effect.clone(), "มีผลกระทบต่อ",
                                page.changed.clone(), "sickness_occupation",
                            ),
                            doms::label_check_for("sickness_occupation","อาชีพ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_disable_by_not_container(
                                page.sickness_education.clone(),
                                page.sickness_effect.clone(), "มีผลกระทบต่อ",
                                page.changed.clone(), "sickness_education",
                            ),
                            doms::label_check_for("sickness_education","การศึกษา"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_binding_texts_disable_by_not_container(
                                page.sickness_other.clone(),
                                vec![page.sickness_other_text.clone()],
                                page.sickness_effect.clone(), "มีผลกระทบต่อ",
                                page.changed.clone(), "sickness_other",
                            ),
                            doms::label_check_for("sickness_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.sickness_other_text.clone(),
                            page.sickness_other.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_reproduction(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("เพศและการเจริญพันธุ์"),
                html!("p"),
                html!("div", {
                    //.attr("id", "CheckSexFormInput")
                    .visible_signal(page.sex().map(|sex| sex == Some(String::from("2"))))
                    .children([
                        html!("div", {
                            .class("row")
                            .children([
                                html!("label", {
                                    .class(class::COL_SM2_R)
                                    .child(html!("b", {.text("ประจำเดือน")}))
                                }),
                                html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                                    doms::radio_toggle_texts_container(page.period.clone(), vec![
                                        page.period_normal.clone(),
                                        page.period_disorders.clone(),
                                        page.period_lmp.clone(),
                                        page.period_menopause.clone(),
                                    ], page.changed.clone(), "period1","ยังไม่มี"),
                                    doms::label_check_for("period1","ยังไม่มี"),
                                ])}),
                            ])
                        }),
                        html!("div", {
                            .class("row")
                            .child(html!("div", {.class(class::FORM_CHK_COL_SM1_OFS2).children([
                                doms::radio_binding_toggle_texts_container(
                                    page.period.clone(),
                                    vec![page.period_normal.clone()],
                                    vec![page.period_menopause.clone()],
                                    page.changed.clone(), "period2", "มี",
                                ),
                                doms::label_check_for("period2","มี"),
                            ])}))
                        }),
                        html!("div", {
                            .class("row")
                            .child(html!("div", {.class(class::FORM_CHK_COL_SM1_OFS3).children([
                                doms::radio_disable_by_not_container(
                                    page.period_normal.clone(),
                                    page.period.clone(), "มี",
                                    page.changed.clone(), "period_normal1", "ปกติ",
                                ),
                                doms::label_check_for("period_normal1","ปกติ"),
                            ])}))
                        }),
                        html!("p"),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {.class(class::FORM_CHK_COL_SM1_OFS3).children([
                                    doms::radio_binding_texts_disable_by_not_container(
                                        page.period_normal.clone(),
                                        vec![page.period_disorders.clone()],
                                        page.period.clone(), "มี",
                                        page.changed.clone(), "period_normal2", "ผิดปกติ",
                                    ),
                                    doms::label_check_for("period_normal2","ผิดปกติ"),
                                ])}),
                                doms::text_disable_by_not_container(
                                    page.period_disorders.clone(),
                                    page.period_normal.clone(), "ผิดปกติ",
                                    page.changed.clone(), "col-sm-4",
                                    Some(100),
                                ),
                            ])
                        }),
                        html!("p"),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("label", {
                                    .class(class::FORM_CHK_COL_SM1_OFS3)
                                    .child(html!("b", {.text("LMP")}))
                                }),
                                doms::text_disable_by_not_container(
                                    page.period_lmp.clone(),
                                    page.period.clone(), "มี",
                                    page.changed.clone(), "col-sm-4",
                                    Some(100),
                                ),
                            ])
                        }),
                        html!("p"),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {.class(class::FORM_CHK_COL_SM2_OFS2).children([
                                    doms::radio_binding_toggle_texts_container(page.period.clone(), vec![page.period_menopause.clone()], vec![
                                        page.period_normal.clone(),
                                        page.period_disorders.clone(),
                                        page.period_lmp.clone(),
                                    ], page.changed.clone(), "period3", "หมดประจำเดือน"),
                                    doms::label_check_for("period3","หมดประจำเดือน เมื่ออายุ"),
                                ])}),
                                doms::text_disable_by_not_container(
                                    page.period_menopause.clone(),
                                    page.period.clone(), "หมดประจำเดือน",
                                    page.changed.clone(), "col-sm-1",
                                    Some(2),
                                ),
                                html!("div", {.class("col-sm-1").child(html!("label", {.text("ปี")}))}),
                            ])
                        }),
                        html!("p"),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM2_R)
                            .child(html!("b", {.text("เต้านม")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.breast.clone(), page.changed.clone(), "breast1","ปกติ"),
                            doms::label_check_for("breast1","ปกติ"),
                        ])}),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM2_OFS2).children([
                            doms::radio_binding_texts_container(
                                page.breast.clone(),
                                vec![page.breast_disorders.clone()],
                                page.changed.clone(), "breast2", "ผิดปกติ",
                            ),
                            doms::label_check_for("breast2","ผิดปกติ(ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.breast_disorders.clone(),
                            page.breast.clone(), "ผิดปกติ",
                            page.changed.clone(), "col-sm-4",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_stress(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("การปรับตัวและทนต่อความเครียด"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM4_R)
                            .child(html!("b", {.text("วิธีแก้ไขความไม่สบายใจ/กังวล/เครียด/อื่นๆ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.consult.clone(), page.changed.clone(), "consult"),
                            doms::label_check_for("consult","ปรึกษา"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.seclude.clone(), page.changed.clone(), "seclude"),
                            doms::label_check_for("seclude","แยกตัว"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_binding_texts_container(
                                page.medication.clone(),
                                vec![page.medication_detail.clone()],
                                page.changed.clone(), "medication",
                            ),
                            doms::label_check_for("medication","ใช้ยา"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.medication_detail.clone(),
                            page.medication.clone(), "Y",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS4).children([
                            doms::checkbox_container(page.religion.clone(), page.changed.clone(), "religion"),
                            doms::label_check_for("religion","ศาสนา"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_binding_texts_container(
                                page.coping_stress_other.clone(),
                                vec![page.coping_stress_other_detail.clone()],
                                page.changed.clone(), "coping_stress_other",
                            ),
                            doms::label_check_for("coping_stress_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.coping_stress_other_detail.clone(),
                            page.coping_stress_other.clone(), "Y",
                            page.changed.clone(), "col-sm-4",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn render_value(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                doms::label_row_12("คุณค่าและความเชื่อ"),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM4_R)
                            .child(html!("b", {.text("เชื่อว่าการเจ็บป่วยครั้งนี้มีสาเหตุจาก")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_container(page.belief_sickness_behave.clone(), page.changed.clone(), "belief_sickness_behave"),
                            doms::label_check_for("belief_sickness_behave","ปฏิบัติตัวไม่ถูกต้อง"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::checkbox_container(page.belief_sickness_age.clone(), page.changed.clone(), "belief_sickness_age"),
                            doms::label_check_for("belief_sickness_age","ตามวัย"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::checkbox_container(page.belief_sickness_destiny.clone(), page.changed.clone(), "belief_sickness_destiny"),
                            doms::label_check_for("belief_sickness_destiny","เคราะห์กรรม"),
                        ])}),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("div", {.class(class::FORM_CHK_COL_SM1_OFS4).children([
                            doms::checkbox_binding_texts_container(
                                page.belief_sickness_other.clone(),
                                vec![page.belief_sickness_other_text.clone()],
                                page.changed.clone(), "belief_sickness_other",
                            ),
                            doms::label_check_for("belief_sickness_other","อื่นๆ"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.belief_sickness_other_text.clone(),
                            page.belief_sickness_other.clone(), "Y",
                            page.changed.clone(), "col-sm-5",
                            Some(100),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM4_R)
                            .child(html!("b", {.text("สิ่งยึดเหนี่ยวด้านจิตใจ")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.belief_believe.clone(), page.changed.clone(), "belief_believe1","ไม่มี"),
                            doms::label_check_for("belief_believe1","ไม่มี"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_binding_texts_container(
                                page.belief_believe.clone(),
                                vec![page.belief_believe_text.clone()],
                                page.changed.clone(), "belief_believe2", "มี",
                            ),
                            doms::label_check_for("belief_believe2","มี"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.belief_believe_text.clone(),
                            page.belief_believe.clone(), "มี",
                            page.changed.clone(), "col-sm-4",
                            Some(100),
                        ),
                    ])
                }),
                html!("p"),
                html!("div", {
                    .class("row")
                    .children([
                        html!("label", {
                            .class(class::COL_SM4_R)
                            .child(html!("b", {.text("ความต้องการปฏิบัติกิจกรรมทางศาสนา")}))
                        }),
                        html!("div", {.class(class::FORM_CHK_COL_SM1).children([
                            doms::radio_container(page.religious_activity.clone(), page.changed.clone(), "religious_activity1","ไม่ต้องการ"),
                            doms::label_check_for("religious_activity1","ไม่ต้องการ"),
                        ])}),
                        html!("div", {.class(class::FORM_CHK_COL_SM2).children([
                            doms::radio_binding_texts_container(
                                page.religious_activity.clone(),
                                vec![page.religious_activity_text.clone()],
                                page.changed.clone(), "religious_activity2", "ต้องการ",
                            ),
                            doms::label_check_for("religious_activity2","ต้องการ (ระบุ)"),
                        ])}),
                        doms::text_disable_by_not_container(
                            page.religious_activity_text.clone(),
                            page.religious_activity.clone(), "ต้องการ",
                            page.changed.clone(), "col-sm-3",
                            Some(100),
                        ),
                    ])
                }),
            ])
        })
    }

    fn submit(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let method = match page.nurse_admission_note_id.get() {
                    Some(_) => "PUT",
                    None => "POST",
                };
                // update admission note in raw data
                let note = Self::finalized(page.clone());
                // POST `EndPoint::IpdAdmissionNoteNurse`
                // PUT `EndPoint::IpdAdmissionNoteNurse`
                match note.call_api_save(method, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            // app.alert("บันทึกข้อมูลสำเร็จ");
                            page.nurse_admission_note_id.set_neq(Some(response.last_insert_id as u32));
                            page.changed.set_neq(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn finalized(page: Rc<Self>) -> IpdNurseAdmissionNote {
        IpdNurseAdmissionNote {
            nurse_admission_note_id: page.nurse_admission_note_id.get().unwrap_or_default(), //  AUTO_INCREMENT,
            hn: page.patient.lock_ref().patient.lock_ref().as_ref().and_then(|pt| pt.hn()).unwrap_or_default(),
            an: page.an.get_cloned(),
            info_patient: str_some(page.info_patient.get_cloned()),
            info_parent: str_some(page.info_parent.get_cloned()),
            info_spouse: str_some(page.info_spouse.get_cloned()),
            info_child: str_some(page.info_child.get_cloned()),
            info_relatives: str_some(page.info_relatives.get_cloned()),
            info_sender: str_some(page.info_sender.get_cloned()),
            chief_complaints: str_some(page.chief_complaints.get_cloned()),
            medical_history: str_some(page.medical_history.get_cloned()),
            vs_admit: str_some(page.vs_admit.get_cloned()),
            concious: str_some(page.concious.get_cloned()),
            normal_breath: str_some(page.normal_breath.get_cloned()),
            kussmaul: str_some(page.kussmaul.get_cloned()),
            tachypnea: str_some(page.tachypnea.get_cloned()),
            dyspnea: str_some(page.dyspnea.get_cloned()),
            apnea: str_some(page.apnea.get_cloned()),
            tube: str_some(page.tube.get_cloned()),
            normal_hr: str_some(page.normal_hr.get_cloned()),
            arregular: str_some(page.arregular.get_cloned()),
            weakness: str_some(page.weakness.get_cloned()),
            arrhythmia: str_some(page.arrhythmia.get_cloned()),
            chestpain: str_some(page.chestpain.get_cloned()),
            pacemaker: str_some(page.pacemaker.get_cloned()),
            cardio_other: str_some(page.cardio_other.get_cloned()),
            cardio_other_text: str_some(page.cardio_other_text.get_cloned()),
            normal_cir: str_some(page.normal_cir.get_cloned()),
            pale: str_some(page.pale.get_cloned()),
            cyanosis: str_some(page.cyanosis.get_cloned()),
            generalized_edema: str_some(page.generalized_edema.get_cloned()),
            localized_edema: str_some(page.localized_edema.get_cloned()),
            localized_edema_text: str_some(page.localized_edema_text.get_cloned()),
            pitting_edema: str_some(page.pitting_edema.get_cloned()),
            pitting_edema_text: str_some(page.pitting_edema_text.get_cloned()),
            circulation_orther: str_some(page.circulation_orther.get_cloned()),
            circulation_orther_text: str_some(page.circulation_orther_text.get_cloned()),
            normal_skin: str_some(page.normal_skin.get_cloned()),
            dry: str_some(page.dry.get_cloned()),
            bruise: str_some(page.bruise.get_cloned()),
            erythema: str_some(page.erythema.get_cloned()),
            abscess: str_some(page.abscess.get_cloned()),
            joudice: str_some(page.joudice.get_cloned()),
            skin_other: str_some(page.skin_other.get_cloned()),
            skin_other_text: str_some(page.skin_other_text.get_cloned()),
            pain: str_some(page.pain.get_cloned()),
            location: str_some(page.location.get_cloned()),
            pain_charac: str_some(page.pain_charac.get_cloned()),
            pain_charac_text: str_some(page.pain_charac_text.get_cloned()),
            pain_score: str_some(page.pain_score.get_cloned()),
            normal_behav: str_some(page.normal_behav.get_cloned()),
            agitate: str_some(page.agitate.get_cloned()),
            aggressive: str_some(page.aggressive.get_cloned()),
            depression: str_some(page.depression.get_cloned()),
            madness: str_some(page.madness.get_cloned()),
            behaviour_other: str_some(page.behaviour_other.get_cloned()),
            behaviour_other_text: str_some(page.behaviour_other_text.get_cloned()),
            normal_emotional: str_some(page.normal_emotional.get_cloned()),
            angry: str_some(page.angry.get_cloned()),
            moody: str_some(page.moody.get_cloned()),
            anxiety: str_some(page.anxiety.get_cloned()),
            fear: str_some(page.fear.get_cloned()),
            emotional_other: str_some(page.emotional_other.get_cloned()),
            emotional_other_text: str_some(page.emotional_other_text.get_cloned()),
            no_anxiety: str_some(page.no_anxiety.get_cloned()),
            study: str_some(page.study.get_cloned()),
            family: str_some(page.family.get_cloned()),
            economy: str_some(page.economy.get_cloned()),
            habitation: str_some(page.habitation.get_cloned()),
            illness: str_some(page.illness.get_cloned()),
            spiritual_no: str_some(page.spiritual_no.get_cloned()),
            spiritual_back_home: str_some(page.spiritual_back_home.get_cloned()),
            spiritual_need_family: str_some(page.spiritual_need_family.get_cloned()),
            spiritual_other: str_some(page.spiritual_other.get_cloned()),
            spiritual_other_text: str_some(page.spiritual_other_text.get_cloned()),
            spiritual_cant_rated: str_some(page.spiritual_cant_rated.get_cloned()),
            spiritual_cant_rated_text: str_some(page.spiritual_cant_rated_text.get_cloned()),
            no_mental_state: str_some(page.no_mental_state.get_cloned()),
            no_mental_state_text: str_some(page.no_mental_state_text.get_cloned()),
            education: str_some(page.education.get_cloned()),
            education_result: str_some(page.education_result.get_cloned()),
            occupation: str_some(page.occupation.get_cloned()),
            income: str_some(page.income.get_cloned()),
            self_value: str_some(page.self_value.get_cloned()),
            person_family: str_some(page.person_family.get_cloned()),
            neighbor: str_some(page.neighbor.get_cloned()),
            assistant_other: str_some(page.assistant_other.get_cloned()),
            assistant_other_text: str_some(page.assistant_other_text.get_cloned()),
            assistant_occupation: str_some(page.assistant_occupation.get_cloned()),
            clinic: str_some(page.clinic.get_cloned()),
            buy_medicine: str_some(page.buy_medicine.get_cloned()),
            no_risk: str_some(page.no_risk.get_cloned()),
            smoking: str_some(page.smoking.get_cloned()),
            smoke_year: str_some(page.smoke_year.get_cloned()),
            smoke_frequency: str_some(page.smoke_frequency.get_cloned()),
            smoke_stopped: str_some(page.smoke_stopped.get_cloned()),
            alcohol: str_some(page.alcohol.get_cloned()),
            alc_year: str_some(page.alc_year.get_cloned()),
            alc_frequency: str_some(page.alc_frequency.get_cloned()),
            alc_stopped: str_some(page.alc_stopped.get_cloned()),
            medication_used: str_some(page.medication_used.get_cloned()),
            med_name: str_some(page.med_name.get_cloned()),
            med_year: str_some(page.med_year.get_cloned()),
            med_frequency: str_some(page.med_frequency.get_cloned()),
            med_stopped: str_some(page.med_stopped.get_cloned()),
            diet_regular: str_some(page.diet_regular.get_cloned()),
            diet_spec: str_some(page.diet_spec.get_cloned()),
            nutrition_risk: str_some(page.nutrition_risk.get_cloned()),
            loss_appetite: str_some(page.loss_appetite.get_cloned()),
            dysphagia: str_some(page.dysphagia.get_cloned()),
            loss_gustation: str_some(page.loss_gustation.get_cloned()),
            denture: str_some(page.denture.get_cloned()),
            nutrition_risk_other: str_some(page.nutrition_risk_other.get_cloned()),
            nutrition_risk_other_text: str_some(page.nutrition_risk_other_text.get_cloned()),
            normal_urine: str_some(page.normal_urine.get_cloned()),
            dysuria: str_some(page.dysuria.get_cloned()),
            incontinence: str_some(page.incontinence.get_cloned()),
            staining: str_some(page.staining.get_cloned()),
            hematuria: str_some(page.hematuria.get_cloned()),
            catheter: str_some(page.catheter.get_cloned()),
            normal_feces: str_some(page.normal_feces.get_cloned()),
            constipation: str_some(page.constipation.get_cloned()),
            diarrhea: str_some(page.diarrhea.get_cloned()),
            bowel_incontinence: str_some(page.bowel_incontinence.get_cloned()),
            hemorrhoid: str_some(page.hemorrhoid.get_cloned()),
            colostomy: str_some(page.colostomy.get_cloned()),
            activity1: str_some(page.activity1.get_cloned()),
            activity2: str_some(page.activity2.get_cloned()),
            activity3: str_some(page.activity3.get_cloned()),
            activity4: str_some(page.activity4.get_cloned()),
            o_p_use: str_some(page.o_p_use.get_cloned()),
            sleep_per_day: str_some(page.sleep_per_day.get_cloned()),
            sleep_hour: str_some(page.sleep_hour.get_cloned()),
            sleep_problems: str_some(page.sleep_problems.get_cloned()),
            sleep_problems_detail: str_some(page.sleep_problems_detail.get_cloned()),
            sleep_med_name: str_some(page.sleep_med_name.get_cloned()),
            sleep_med_name_detail: str_some(page.sleep_med_name_detail.get_cloned()),
            cognitive: str_some(page.cognitive.get_cloned()),
            memory: str_some(page.memory.get_cloned()),
            memory_detail: str_some(page.memory_detail.get_cloned()),
            hearing: str_some(page.hearing.get_cloned()),
            hearing_detail: str_some(page.hearing_detail.get_cloned()),
            eartone: str_some(page.eartone.get_cloned()),
            vision: str_some(page.vision.get_cloned()),
            vision_detail: str_some(page.vision_detail.get_cloned()),
            vision_eyeglasses: str_some(page.vision_eyeglasses.get_cloned()),
            vision_contactlens: str_some(page.vision_contactlens.get_cloned()),
            speech: str_some(page.speech.get_cloned()),
            speech_detail: str_some(page.speech_detail.get_cloned()),
            self_image: str_some(page.self_image.get_cloned()),
            self_image_detail: str_some(page.self_image_detail.get_cloned()),
            self_activity: str_some(page.self_activity.get_cloned()),
            self_activity_detail: str_some(page.self_activity_detail.get_cloned()),
            sickness_effect: str_some(page.sickness_effect.get_cloned()),
            sickness_family: str_some(page.sickness_family.get_cloned()),
            sickness_occupation: str_some(page.sickness_occupation.get_cloned()),
            sickness_education: str_some(page.sickness_education.get_cloned()),
            sickness_other: str_some(page.sickness_other.get_cloned()),
            sickness_other_text: str_some(page.sickness_other_text.get_cloned()),
            period: str_some(page.period.get_cloned()),
            period_normal: str_some(page.period_normal.get_cloned()),
            period_disorders: str_some(page.period_disorders.get_cloned()),
            period_lmp: str_some(page.period_lmp.get_cloned()),
            period_menopause: str_some(page.period_menopause.get_cloned()),
            breast: str_some(page.breast.get_cloned()),
            breast_disorders: str_some(page.breast_disorders.get_cloned()),
            consult: str_some(page.consult.get_cloned()),
            seclude: str_some(page.seclude.get_cloned()),
            medication: str_some(page.medication.get_cloned()),
            medication_detail: str_some(page.medication_detail.get_cloned()),
            religion: str_some(page.religion.get_cloned()),
            coping_stress_other: str_some(page.coping_stress_other.get_cloned()),
            coping_stress_other_detail: str_some(page.coping_stress_other_detail.get_cloned()),
            belief_sickness_behave: str_some(page.belief_sickness_behave.get_cloned()),
            belief_sickness_age: str_some(page.belief_sickness_age.get_cloned()),
            belief_sickness_destiny: str_some(page.belief_sickness_destiny.get_cloned()),
            belief_sickness_other: str_some(page.belief_sickness_other.get_cloned()),
            belief_sickness_other_text: str_some(page.belief_sickness_other_text.get_cloned()),
            belief_believe: str_some(page.belief_believe.get_cloned()),
            belief_believe_text: str_some(page.belief_believe_text.get_cloned()),
            religious_activity: str_some(page.religious_activity.get_cloned()),
            religious_activity_text: str_some(page.religious_activity_text.get_cloned()),
            // not save
            nurse_name: str_some(page.nurse_name.get_cloned()),
            nurse_pos: str_some(page.nurse_pos.get_cloned()),
            nurse_licenseno: str_some(page.nurse_licenseno.get_cloned()),
            receiver_medication_date: page.receiver_medication_date.get(),
            receiver_medication_time: page.receiver_medication_time.get(),
        }
    }
}
