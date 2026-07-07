// ipd-vital-sign-show-form.php
// opd-er-vital-sign-show-form.php

use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
};
use rust_decimal::Decimal;
use std::rc::Rc;
use time::PrimitiveDateTime;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    opd_er::medical_history::{OpdErMedicalHistory, OpdErMedicalHistoryParams},
    patient_info::PatientInfo,
    score::Scores,
    score::{CONCAT_EMPTY, ScoreDispatch},
    select_utils::SelectOption,
    vital_sign::{VitalSign, VitalSignParams, VitalSignSave},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, datetime_from_opt, js_now, time_8601},
    util::{lr_int_from_quote, lr_int_to_quote, str_some, zero_none},
};

use crate::modal::{
    blank_modal,
    lab_wbc::LabWbc,
    scoring::{
        aggression_oas::AggressionOAS, alcohol_aws::AlcoholAws, alcohol_ciwa_ar::AlcoholCiwaAr, amphetamine_awq::AmphetamineAwqV2, barthel_index::BarthelIndex, braden::Braden, depress_2q::Depress2Q,
        depress_9q::Depress9Q, motor_activity_maas::MotorActivityMaas, suicide_8q::Suicide8Q,
    },
};

#[derive(Clone, Default, PartialEq)]
enum Tab {
    #[default]
    VitalSign,
    NeuroSign,
    Score,
    Had,
    O2,
    Lr,
    Other,
}

/// - POST/PUT `EndPoint::IpdVitalSign` (guarded, remove 'บันทึก' btn)
/// - POST/PUT `EndPoint::OpdErVitalSign` (guarded, remove 'บันทึก' btn)
/// - DELETE `EndPoint::IpdVitalSignId` (guarded, remove 'ลบ' btn)
/// - DELETE `EndPoint::OpdErVitalSignId` (guarded, remove 'ลบ' btn)
/// - GET `EndPoint::OpdErMedicalHistory` (guarded, remove 'ดึงข้อมูลจาก HOSxP' btn)
/// - GET `EndPoint::LabWbcKeyValue` (LabWbc, guarded, remove lab btn)
#[derive(Default)]
pub struct VitalSignFormCpn {
    changed: Mutable<bool>,
    active_tab: Mutable<Tab>,
    wbc_modal: Mutable<Option<Rc<LabWbc>>>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    vs_id: Mutable<Option<u32>>,
    action_id: Mutable<Option<u32>>,

    vs_date: Mutable<String>,
    vs_time: Mutable<String>,

    bt: Mutable<String>,
    pr: Mutable<String>,
    rr: Mutable<String>,
    respirator: Mutable<String>,
    sbp: Mutable<String>,
    dbp: Mutable<String>,
    inotrope: Mutable<String>,
    map: Mutable<String>,
    sat: Mutable<String>,
    sat_room_air: Mutable<String>,
    cvp: Mutable<String>,
    end_co2: Mutable<String>,
    conscious_id: Mutable<String>,
    bw: Mutable<String>,
    height: Mutable<String>,
    urine: Mutable<String>,
    catheter: Mutable<String>,
    urine_amount: Mutable<String>,
    urine_duration: Mutable<String>,
    feces: Mutable<String>,
    head: Mutable<String>,
    t_inc: Mutable<String>,
    line_id: Mutable<String>,
    line_no: Mutable<String>,
    line_mark: Mutable<String>,
    braden: Mutable<String>,
    pain: Mutable<String>,
    eye: Mutable<String>,
    verbal: Mutable<String>,
    movement: Mutable<String>,
    right_pupil: Mutable<String>,
    right_cha_id: Mutable<String>,
    left_pupil: Mutable<String>,
    left_cha_id: Mutable<String>,
    va_id: Mutable<String>,
    mass_id: Mutable<String>,
    lt_arm: Mutable<String>,
    lt_leg: Mutable<String>,
    rt_arm: Mutable<String>,
    rt_leg: Mutable<String>,
    severity: Mutable<String>,
    had_name: Mutable<String>,
    had_drop: Mutable<String>,
    hct: Mutable<String>,
    dtx: Mutable<String>,
    bl: Mutable<String>,
    mcb: Mutable<String>,
    suction: Mutable<String>,
    nb: Mutable<String>,
    o2_id: Mutable<String>,
    o2_flow: Mutable<String>,
    tube_id: Mutable<String>,
    tube_no: Mutable<String>,
    tube_mark: Mutable<String>,
    ventilator_name: Mutable<String>,
    mode: Mutable<String>,
    tv: Mutable<String>,
    pip: Mutable<String>,
    r_rate: Mutable<String>,
    i_rate: Mutable<String>,
    e_rate: Mutable<String>,
    ti: Mutable<String>,
    ps: Mutable<String>,
    fio2: Mutable<String>,
    peep: Mutable<String>,
    ft: Mutable<String>,
    delta_p: Mutable<String>,
    o2_map: Mutable<String>,
    intake_id: Mutable<String>,
    intake_type: Mutable<String>,
    intake_amount: Mutable<String>,
    intake_absorb: Mutable<String>,
    other: Mutable<String>,
    output_id: Mutable<String>,
    output_amount: Mutable<String>,
    lr_int_m: Mutable<String>,
    lr_int_s: Mutable<String>,
    lr_dur: Mutable<String>,
    lr_fsh: Mutable<String>,
    lr_sev: Mutable<String>,
    lr_cer: Mutable<String>,
    lr_eff: Mutable<String>,
    lr_sta: Mutable<String>,
    lr_mem: Mutable<String>,
    lr_af: Mutable<String>,
    breathing_id: Mutable<String>,
    avpu_id: Mutable<String>,
    gut_feeling_id: Mutable<String>,
    pops_other_id: Mutable<String>,
    wbc: Mutable<String>,
    pleak_flow: Mutable<String>,
    // different from original KPHIS
    crt: Mutable<String>,
    band: Mutable<String>,
    lr_pos: Mutable<String>,
    lr_moulding: Mutable<String>,
    lr_oxytocin_unit: Mutable<String>,
    lr_oxytocin_rate: Mutable<String>,
    lr_urine_vol: Mutable<String>,
    urine_protein: Mutable<String>,
    urine_sugar: Mutable<String>,
    diet: Mutable<String>,
    barthel_index: Mutable<String>,
    aggression_oas: Mutable<String>,
    alcohol_ciwa: Mutable<String>,
    alcohol_aws: Mutable<String>,
    amphetamine_awq: Mutable<String>,
    motivation_scale: Mutable<String>,
    craving_scale: Mutable<String>,
    stage_of_change_id: Mutable<String>,
    depress_2q: Mutable<String>,
    depress_9q: Mutable<String>,
    suicide_8q: Mutable<String>,

    scores: Mutable<Option<Scores>>,
    aggression_oas_modal: Mutable<Option<Rc<AggressionOAS>>>,
    braden_modal: Mutable<Option<Rc<Braden>>>,
    maas_modal: Mutable<Option<Rc<MotorActivityMaas>>>,
    barthel_index_modal: Mutable<Option<Rc<BarthelIndex>>>,
    amphetamine_awq_modal: Mutable<Option<Rc<AmphetamineAwqV2>>>,
    alcohol_ciwa_modal: Mutable<Option<Rc<AlcoholCiwaAr>>>,
    alcohol_aws_modal: Mutable<Option<Rc<AlcoholAws>>>,
    depress_2q_modal: Mutable<Option<Rc<Depress2Q>>>,
    depress_9q_modal: Mutable<Option<Rc<Depress9Q>>>,
    suicide_8q_modal: Mutable<Option<Rc<Suicide8Q>>>,
}

impl VitalSignFormCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, vs_opt: Option<Rc<VitalSign>>, app: Rc<App>) -> Rc<Self> {
        let birthday = patient.lock_ref().as_ref().and_then(|pt| pt.birthday());
        Rc::new(match vs_opt {
            Some(vs) => {
                let (lr_int_m, lr_int_s) = if let Some(lr_int) = &vs.lr_int { lr_int_from_quote(lr_int) } else { (0, 0) };
                Self {
                    patient,
                    vs_id: Mutable::new(zero_none(vs.vs_id)),
                    action_id: Mutable::new(vs.action_id),
                    vs_date: Mutable::new(vs.vs_datetime.date().to_string()),
                    vs_time: Mutable::new(vs.vs_datetime.time().js_string()),
                    bt: Mutable::new(vs.bt.map(|d| d.to_string()).unwrap_or_default()),
                    pr: Mutable::new(vs.pr.map(|u| u.to_string()).unwrap_or_default()),
                    rr: Mutable::new(vs.rr.map(|u| u.to_string()).unwrap_or_default()),
                    respirator: Mutable::new(vs.respirator.to_owned().unwrap_or_default()),
                    sbp: Mutable::new(vs.sbp.map(|u| u.to_string()).unwrap_or_default()),
                    dbp: Mutable::new(vs.dbp.map(|u| u.to_string()).unwrap_or_default()),
                    inotrope: Mutable::new(vs.inotrope.to_owned().unwrap_or_default()),
                    map: Mutable::new(vs.map.map(|i| i.to_string()).unwrap_or_default()),
                    sat: Mutable::new(vs.sat.map(|u| u.to_string()).unwrap_or_default()),
                    sat_room_air: Mutable::new(vs.sat_room_air.map(|u| u.to_string()).unwrap_or_default()),
                    cvp: Mutable::new(vs.cvp.to_owned().unwrap_or_default()),
                    end_co2: Mutable::new(vs.end_co2.map(|u| u.to_string()).unwrap_or_default()),
                    conscious_id: Mutable::new(vs.conscious_id.map(|u| u.to_string()).unwrap_or_default()),
                    bw: Mutable::new(vs.bw.map(|d| d.to_string()).unwrap_or_default()),
                    height: Mutable::new(vs.height.map(|i| i.to_string()).unwrap_or_default()),
                    urine: Mutable::new(vs.urine.to_owned().unwrap_or_default()),
                    catheter: Mutable::new(vs.catheter.to_owned().unwrap_or_default()),
                    urine_amount: Mutable::new(vs.urine_amount.map(|u| u.to_string()).unwrap_or_default()),
                    urine_duration: Mutable::new(vs.urine_duration.map(|u| u.to_string()).unwrap_or_default()),
                    feces: Mutable::new(vs.feces.to_owned().unwrap_or_default()),
                    head: Mutable::new(vs.head.map(|d| d.to_string()).unwrap_or_default()),
                    t_inc: Mutable::new(vs.t_inc.map(|d| d.to_string()).unwrap_or_default()),
                    line_id: Mutable::new(vs.line_id.map(|u| u.to_string()).unwrap_or_default()),
                    line_no: Mutable::new(vs.line_no.map(|d| d.to_string()).unwrap_or_default()),
                    line_mark: Mutable::new(vs.line_mark.map(|d| d.to_string()).unwrap_or_default()),
                    braden: Mutable::new(vs.braden.to_owned().unwrap_or_default()),
                    pain: Mutable::new(vs.pain.map(|i| i.to_string()).unwrap_or_default()),
                    eye: Mutable::new(vs.eye.map(|i| i.to_string()).unwrap_or_default()),
                    verbal: Mutable::new(vs.verbal.to_owned().unwrap_or_default()),
                    movement: Mutable::new(vs.movement.map(|i| i.to_string()).unwrap_or_default()),
                    right_pupil: Mutable::new(vs.right_pupil.map(|d| d.to_string()).unwrap_or_default()),
                    right_cha_id: Mutable::new(vs.right_cha_id.map(|u| u.to_string()).unwrap_or_default()),
                    left_pupil: Mutable::new(vs.left_pupil.map(|d| d.to_string()).unwrap_or_default()),
                    left_cha_id: Mutable::new(vs.left_cha_id.map(|u| u.to_string()).unwrap_or_default()),
                    va_id: Mutable::new(vs.va_id.map(|u| u.to_string()).unwrap_or_default()),
                    mass_id: Mutable::new(vs.mass_id.map(|u| u.to_string()).unwrap_or_default()),
                    lt_arm: Mutable::new(vs.lt_arm.map(|u| u.to_string()).unwrap_or_default()),
                    lt_leg: Mutable::new(vs.lt_leg.map(|u| u.to_string()).unwrap_or_default()),
                    rt_arm: Mutable::new(vs.rt_arm.map(|u| u.to_string()).unwrap_or_default()),
                    rt_leg: Mutable::new(vs.rt_leg.map(|u| u.to_string()).unwrap_or_default()),
                    severity: Mutable::new(vs.severity.map(|i| i.to_string()).unwrap_or_default()),
                    had_name: Mutable::new(vs.had_name.to_owned().unwrap_or_default()),
                    had_drop: Mutable::new(vs.had_drop.to_owned().unwrap_or_default()),
                    hct: Mutable::new(vs.hct.map(|d| d.to_string()).unwrap_or_default()),
                    dtx: Mutable::new(vs.dtx.to_owned().unwrap_or_default()),
                    bl: Mutable::new(vs.bl.map(|d| d.to_string()).unwrap_or_default()),
                    mcb: Mutable::new(vs.mcb.map(|d| d.to_string()).unwrap_or_default()),
                    suction: Mutable::new(vs.suction.to_owned().unwrap_or_default()),
                    nb: Mutable::new(vs.nb.to_owned().unwrap_or_default()),
                    o2_id: Mutable::new(vs.o2_id.map(|u| u.to_string()).unwrap_or_default()),
                    o2_flow: Mutable::new(vs.o2_flow.map(|d| d.to_string()).unwrap_or_default()),
                    tube_id: Mutable::new(vs.tube_id.map(|u| u.to_string()).unwrap_or_default()),
                    tube_no: Mutable::new(vs.tube_no.map(|d| d.to_string()).unwrap_or_default()),
                    tube_mark: Mutable::new(vs.tube_mark.map(|d| d.to_string()).unwrap_or_default()),
                    ventilator_name: Mutable::new(vs.ventilator_name.clone().unwrap_or_default()),
                    mode: Mutable::new(vs.mode.clone().unwrap_or_default()),
                    tv: Mutable::new(vs.tv.map(|i| i.to_string()).unwrap_or_default()),
                    pip: Mutable::new(vs.pip.map(|i| i.to_string()).unwrap_or_default()),
                    r_rate: Mutable::new(vs.r_rate.map(|i| i.to_string()).unwrap_or_default()),
                    i_rate: Mutable::new(vs.i_rate.map(|i| i.to_string()).unwrap_or_default()),
                    e_rate: Mutable::new(vs.e_rate.map(|i| i.to_string()).unwrap_or_default()),
                    ti: Mutable::new(vs.ti.map(|d| d.to_string()).unwrap_or_default()),
                    ps: Mutable::new(vs.ps.map(|i| i.to_string()).unwrap_or_default()),
                    fio2: Mutable::new(vs.fio2.map(|d| d.to_string()).unwrap_or_default()),
                    peep: Mutable::new(vs.peep.map(|i| i.to_string()).unwrap_or_default()),
                    ft: Mutable::new(vs.ft.map(|d| d.to_string()).unwrap_or_default()),
                    delta_p: Mutable::new(vs.delta_p.map(|i| i.to_string()).unwrap_or_default()),
                    o2_map: Mutable::new(vs.o2_map.map(|i| i.to_string()).unwrap_or_default()),
                    intake_id: Mutable::new(vs.intake_id.map(|i| i.to_string()).unwrap_or_default()),
                    intake_type: Mutable::new(vs.intake_type.to_owned().unwrap_or_default()),
                    intake_amount: Mutable::new(vs.intake_amount.map(|i| i.to_string()).unwrap_or_default()),
                    intake_absorb: Mutable::new(vs.intake_absorb.map(|i| i.to_string()).unwrap_or_default()),
                    other: Mutable::new(vs.other.to_owned().unwrap_or_default()),
                    output_id: Mutable::new(vs.output_id.map(|u| u.to_string()).unwrap_or_default()),
                    output_amount: Mutable::new(vs.output_amount.map(|i| i.to_string()).unwrap_or_default()),
                    lr_int_m: Mutable::new(zero_none(lr_int_m).map(|u| u.to_string()).unwrap_or_default()),
                    lr_int_s: Mutable::new(zero_none(lr_int_s).map(|u| u.to_string()).unwrap_or_default()),
                    lr_dur: Mutable::new(vs.lr_dur.map(|i| i.to_string()).unwrap_or_default()),
                    lr_fsh: Mutable::new(vs.lr_fsh.map(|i| i.to_string()).unwrap_or_default()),
                    lr_sev: Mutable::new(vs.lr_sev.to_owned().unwrap_or_default()),
                    lr_cer: Mutable::new(vs.lr_cer.to_owned().unwrap_or_default()),
                    lr_eff: Mutable::new(vs.lr_eff.map(|i| i.to_string()).unwrap_or_default()),
                    lr_sta: Mutable::new(vs.lr_sta.map(|i| i.to_string()).unwrap_or_default()),
                    lr_mem: Mutable::new(vs.lr_mem.map(|i| i.to_string()).unwrap_or_default()),
                    lr_af: Mutable::new(vs.lr_af.to_owned().unwrap_or_default()),
                    breathing_id: Mutable::new(vs.breathing_id.map(|u| u.to_string()).unwrap_or_default()),
                    avpu_id: Mutable::new(vs.avpu_id.map(|u| u.to_string()).unwrap_or_default()),
                    gut_feeling_id: Mutable::new(vs.gut_feeling_id.map(|u| u.to_string()).unwrap_or_default()),
                    pops_other_id: Mutable::new(vs.pops_other_id.map(|u| u.to_string()).unwrap_or_default()),
                    wbc: Mutable::new(vs.wbc.map(|d| d.to_string()).unwrap_or_default()),
                    pleak_flow: Mutable::new(vs.pleak_flow.map(|i| i.to_string()).unwrap_or_default()),
                    // different from original KPHIS
                    crt: Mutable::new(vs.crt.map(|i| i.to_string()).unwrap_or_default()),
                    band: Mutable::new(vs.band.map(|i| i.to_string()).unwrap_or_default()),
                    lr_pos: Mutable::new(vs.lr_pos.to_owned().unwrap_or_default()),
                    lr_moulding: Mutable::new(vs.lr_moulding.map(|u| u.to_string()).unwrap_or_default()),
                    lr_oxytocin_unit: Mutable::new(vs.lr_oxytocin_unit.map(|u| u.to_string()).unwrap_or_default()),
                    lr_oxytocin_rate: Mutable::new(vs.lr_oxytocin_rate.map(|u| u.to_string()).unwrap_or_default()),
                    lr_urine_vol: Mutable::new(vs.lr_urine_vol.map(|u| u.to_string()).unwrap_or_default()),
                    urine_protein: Mutable::new(vs.urine_protein.map(|u| u.to_string()).unwrap_or_default()),
                    urine_sugar: Mutable::new(vs.urine_sugar.map(|u| u.to_string()).unwrap_or_default()),
                    diet: Mutable::new(vs.diet.to_owned().unwrap_or_default()),
                    barthel_index: Mutable::new(vs.barthel_index.to_owned().unwrap_or_default()),
                    aggression_oas: Mutable::new(vs.aggression_oas.to_owned().unwrap_or_default()),
                    alcohol_ciwa: Mutable::new(vs.alcohol_ciwa.to_owned().unwrap_or_default()),
                    alcohol_aws: Mutable::new(vs.alcohol_aws.to_owned().unwrap_or_default()),
                    amphetamine_awq: Mutable::new(vs.amphetamine_awq.to_owned().unwrap_or_default()),
                    motivation_scale: Mutable::new(vs.motivation_scale.map(|u| u.to_string()).unwrap_or_default()),
                    craving_scale: Mutable::new(vs.craving_scale.map(|u| u.to_string()).unwrap_or_default()),
                    stage_of_change_id: Mutable::new(vs.stage_of_change_id.map(|u| u.to_string()).unwrap_or_default()),
                    depress_2q: Mutable::new(vs.depress_2q.to_owned().unwrap_or_default()),
                    depress_9q: Mutable::new(vs.depress_9q.to_owned().unwrap_or_default()),
                    suicide_8q: Mutable::new(vs.suicide_8q.to_owned().unwrap_or_default()),
                    scores: Mutable::new(Scores::from_vs(&vs, birthday, app.state())),
                    ..Default::default()
                }
            }
            None => {
                let now = js_now();
                let empty_ews_concat = [&now.js_string(), CONCAT_EMPTY].concat();
                Self {
                    patient,
                    vs_date: Mutable::new(now.date().to_string()),
                    scores: Mutable::new(Scores::from_concat(&Some(empty_ews_concat), birthday, app.state())),
                    ..Default::default()
                }
            }
        })
    }

    // fn is_opd_er(&self) -> impl Signal<Item = Option<bool>> + use<> {
    //     self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| !pt.is_ipd()))
    // }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient
            .signal_cloned()
            .map(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    fn is_scorable(&self, item: &'static str) -> impl Signal<Item = bool> + use<> {
        self.scores.signal_ref(|opt| opt.as_ref().map(|sc| sc.contains(item)).unwrap_or_default())
    }

    fn finalized(&self) -> VitalSignSave {
        let now = js_now();
        VitalSignSave {
            vs_id: self.vs_id.get().unwrap_or_default(),
            action_id: None,
            vs_datetime: PrimitiveDateTime::new(date_8601(&self.vs_date.lock_ref()).unwrap_or(now.date()), time_8601(&self.vs_time.lock_ref()).unwrap_or(now.time())),
            bt: Decimal::from_str_exact(&self.bt.lock_ref()).ok(),
            pr: self.pr.lock_ref().parse::<u32>().ok(),
            rr: self.rr.lock_ref().parse::<u32>().ok(),
            respirator: str_some(self.respirator.get_cloned()),
            sbp: self.sbp.lock_ref().parse::<u32>().ok(),
            dbp: self.dbp.lock_ref().parse::<u32>().ok(),
            inotrope: str_some(self.inotrope.get_cloned()),
            map: self.map.lock_ref().parse::<i32>().ok(),
            sat: self.sat.lock_ref().parse::<u32>().ok(),
            sat_room_air: self.sat_room_air.lock_ref().parse::<u32>().ok(),
            cvp: str_some(self.cvp.get_cloned()),
            end_co2: self.end_co2.lock_ref().parse::<u32>().ok(),
            conscious_id: self.conscious_id.lock_ref().parse::<u32>().ok(),
            bw: Decimal::from_str_exact(&self.bw.lock_ref()).ok(),
            height: self.height.lock_ref().parse::<i32>().ok(),
            urine: str_some(self.urine.get_cloned()),
            catheter: str_some(self.catheter.get_cloned()),
            urine_amount: self.urine_amount.lock_ref().parse::<u32>().ok(),
            urine_duration: self.urine_duration.lock_ref().parse::<u32>().ok(),
            feces: str_some(self.feces.get_cloned()),
            head: Decimal::from_str_exact(&self.head.lock_ref()).ok(),
            t_inc: Decimal::from_str_exact(&self.t_inc.lock_ref()).ok(),
            line_id: self.line_id.lock_ref().parse::<u32>().ok(),
            line_no: Decimal::from_str_exact(&self.line_no.lock_ref()).ok(),
            line_mark: Decimal::from_str_exact(&self.line_mark.lock_ref()).ok(),
            braden: str_some(self.braden.get_cloned()),
            pain: self.pain.lock_ref().parse::<i32>().ok(),
            eye: self.eye.lock_ref().parse::<i32>().ok(),
            verbal: str_some(self.verbal.get_cloned()),
            movement: self.movement.lock_ref().parse::<i32>().ok(),
            right_pupil: Decimal::from_str_exact(&self.right_pupil.lock_ref()).ok(),
            right_cha_id: self.right_cha_id.lock_ref().parse::<u32>().ok(),
            left_pupil: Decimal::from_str_exact(&self.left_pupil.lock_ref()).ok(),
            left_cha_id: self.left_cha_id.lock_ref().parse::<u32>().ok(),
            va_id: self.va_id.lock_ref().parse::<u32>().ok(),
            mass_id: self.mass_id.lock_ref().parse::<u32>().ok(),
            lt_arm: self.lt_arm.lock_ref().parse::<u32>().ok(),
            lt_leg: self.lt_leg.lock_ref().parse::<u32>().ok(),
            rt_arm: self.rt_arm.lock_ref().parse::<u32>().ok(),
            rt_leg: self.rt_leg.lock_ref().parse::<u32>().ok(),
            severity: self.severity.lock_ref().parse::<i32>().ok(),
            had_name: str_some(self.had_name.get_cloned()),
            had_drop: str_some(self.had_drop.get_cloned()),
            hct: Decimal::from_str_exact(&self.hct.lock_ref()).ok(),
            dtx: str_some(self.dtx.get_cloned()),
            bl: Decimal::from_str_exact(&self.bl.lock_ref()).ok(),
            mcb: Decimal::from_str_exact(&self.mcb.lock_ref()).ok(),
            suction: str_some(self.suction.get_cloned()),
            nb: str_some(self.nb.get_cloned()),
            o2_id: self.o2_id.lock_ref().parse::<u32>().ok(),
            o2_flow: Decimal::from_str_exact(&self.o2_flow.lock_ref()).ok(),
            tube_id: self.tube_id.lock_ref().parse::<u32>().ok(),
            tube_no: Decimal::from_str_exact(&self.tube_no.lock_ref()).ok(),
            tube_mark: Decimal::from_str_exact(&self.tube_mark.lock_ref()).ok(),
            ventilator_name: str_some(self.ventilator_name.get_cloned()),
            mode: str_some(self.mode.get_cloned()),
            tv: self.tv.lock_ref().parse::<i32>().ok(),
            pip: self.pip.lock_ref().parse::<i32>().ok(),
            r_rate: self.r_rate.lock_ref().parse::<i32>().ok(),
            i_rate: self.i_rate.lock_ref().parse::<i32>().ok(),
            e_rate: self.e_rate.lock_ref().parse::<i32>().ok(),
            ti: Decimal::from_str_exact(&self.ti.lock_ref()).ok(),
            ps: self.ps.lock_ref().parse::<i32>().ok(),
            fio2: Decimal::from_str_exact(&self.fio2.lock_ref()).ok(),
            peep: self.peep.lock_ref().parse::<i32>().ok(),
            ft: Decimal::from_str_exact(&self.ft.lock_ref()).ok(),
            delta_p: self.delta_p.lock_ref().parse::<i32>().ok(),
            o2_map: self.o2_map.lock_ref().parse::<i32>().ok(),
            intake_id: self.intake_id.lock_ref().parse::<u32>().ok(),
            intake_type: str_some(self.intake_type.get_cloned()),
            intake_amount: self.intake_amount.lock_ref().parse::<i32>().ok(),
            intake_absorb: self.intake_absorb.lock_ref().parse::<i32>().ok(),
            other: str_some(self.other.get_cloned()),
            output_id: self.output_id.lock_ref().parse::<u32>().ok(),
            output_amount: self.output_amount.lock_ref().parse::<i32>().ok(),
            lr_int: str_some(lr_int_to_quote(&self.lr_int_m.lock_ref(), &self.lr_int_s.lock_ref())),
            lr_dur: self.lr_dur.lock_ref().parse::<i32>().ok(),
            lr_fsh: self.lr_fsh.lock_ref().parse::<i32>().ok(),
            lr_sev: str_some(self.lr_sev.get_cloned()),
            lr_cer: str_some(self.lr_cer.get_cloned()),
            lr_eff: self.lr_eff.lock_ref().parse::<i32>().ok(),
            lr_sta: self.lr_sta.lock_ref().parse::<u32>().ok(),
            lr_mem: self.lr_mem.lock_ref().parse::<u32>().ok(),
            lr_af: str_some(self.lr_af.get_cloned()),
            breathing_id: self.breathing_id.lock_ref().parse::<u32>().ok(),
            avpu_id: self.avpu_id.lock_ref().parse::<u32>().ok(),
            gut_feeling_id: self.gut_feeling_id.lock_ref().parse::<u32>().ok(),
            pops_other_id: self.pops_other_id.lock_ref().parse::<u32>().ok(),
            wbc: Decimal::from_str_exact(&self.wbc.lock_ref()).ok(),
            pleak_flow: self.pleak_flow.lock_ref().parse::<i32>().ok(),
            // different from original KPHIS
            crt: self.crt.lock_ref().parse::<i32>().ok(),
            band: self.band.lock_ref().parse::<i32>().ok(),
            lr_pos: str_some(self.lr_pos.get_cloned()),
            lr_moulding: self.lr_moulding.lock_ref().parse::<u32>().ok(),
            lr_oxytocin_unit: self.lr_oxytocin_unit.lock_ref().parse::<u32>().ok(),
            lr_oxytocin_rate: self.lr_oxytocin_rate.lock_ref().parse::<u32>().ok(),
            lr_urine_vol: self.lr_urine_vol.lock_ref().parse::<u32>().ok(),
            urine_protein: self.urine_protein.lock_ref().parse::<u32>().ok(),
            urine_sugar: self.urine_sugar.lock_ref().parse::<u32>().ok(),
            diet: str_some(self.diet.get_cloned()),
            barthel_index: str_some(self.barthel_index.get_cloned()),
            aggression_oas: str_some(self.aggression_oas.get_cloned()),
            alcohol_ciwa: str_some(self.alcohol_ciwa.get_cloned()),
            alcohol_aws: str_some(self.alcohol_aws.get_cloned()),
            amphetamine_awq: str_some(self.amphetamine_awq.get_cloned()),
            motivation_scale: self.motivation_scale.lock_ref().parse::<u8>().ok(),
            craving_scale: self.craving_scale.lock_ref().parse::<u8>().ok(),
            stage_of_change_id: self.stage_of_change_id.lock_ref().parse::<u8>().ok(),
            depress_2q: str_some(self.depress_2q.get_cloned()),
            depress_9q: str_some(self.depress_9q.get_cloned()),
            suicide_8q: str_some(self.suicide_8q.get_cloned()),
        }
    }

    fn calculate_map(&self) {
        let sbp_opt = self.sbp.lock_ref().parse::<u32>().ok();
        let dbp_opt = self.dbp.lock_ref().parse::<u32>().ok();
        if let (Some(sbp), Some(dbp)) = (sbp_opt, dbp_opt) {
            let map = ((dbp * 2) + sbp) / 3;
            self.map.set(map.to_string());
        }
    }

    fn submit(vs_id: Mutable<u32>, vs_changed: Mutable<bool>, page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            if page.vs_date.lock_ref().is_empty() {
                if let Some(elm) = app.get_id("vs_date").and_then(|elm| elm.dyn_into::<HtmlInputElement>().ok()) {
                    if let Err(e) = elm.focus() {
                        app.show_jsvalue_message(&e);
                    }
                }
            } else if page.vs_time.lock_ref().is_empty() {
                if let Some(elm) = app.get_id("vs_time").and_then(|elm| elm.dyn_into::<HtmlInputElement>().ok()) {
                    if let Err(e) = elm.focus() {
                        app.show_jsvalue_message(&e);
                    }
                }
            } else {
                app.async_load(
                    true,
                    clone!(app => async move {
                        let method = match page.vs_id.get() {
                            Some(_) => "PUT",
                            None => "POST",
                        };
                        let (params_opt, is_ipd) = match patient.visit_type() {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                (Some(VitalSignParams {
                                    hn: patient.hn(),
                                    an: Some(an),
                                    ..Default::default()
                                }), true)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                (Some(VitalSignParams {
                                    opd_er_order_master_id: Some(opd_er_order_master_id),
                                    ..Default::default()
                                }), false)
                            }
                            VisitTypeId::Visit(_) => {
                                (None, false)
                            }
                        };
                        if let Some(params) = params_opt {
                            let saver = page.finalized();
                            // POST `EndPoint::IpdVitalSign`
                            // PUT `EndPoint::IpdVitalSign`
                            // POST `EndPoint::OpdErVitalSign`
                            // PUT `EndPoint::OpdErVitalSign`
                            match saver.call_api_save(is_ipd, method, &params, app.state()).await {
                                Ok(response) => {
                                    app.alert_execute_response(&response, async move {
                                        // app.alert("บันทึกข้อมูลเรียบร้อยแล้วค่ะ");
                                        // update query will return last_insert_id == 0
                                        if let Some(id) = zero_none(response.last_insert_id as u32) {
                                            page.vs_id.set_neq(Some(id));
                                            vs_id.set_neq(id);
                                        }
                                        page.changed.set_neq(false);
                                        vs_changed.set(true);
                                    }).await;
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                    }),
                );
            }
        }
    }

    fn delete_vs(visit_type: VisitTypeId, id: u32, vs_id: Mutable<u32>, vs_changed: Mutable<bool>, form_rendered: Mutable<bool>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยันการลบข้อมูล").await {
                    // DELETE `EndPoint::IpdVitalSignId`
                    // DELETE `EndPoint::OpdErVitalSignId`
                    match VitalSign::call_api_delete(visit_type.is_ipd(), id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                // app.alert("ลบข้อมูลเรียบร้อยแล้วค่ะ");
                                vs_id.set(0);
                                vs_changed.set(true);
                                form_rendered.set(false);
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    fn get_vs_hosxp(page: Rc<Self>, app: Rc<App>) {
        let vn_opt = page.patient.lock_ref().as_ref().and_then(|pt| pt.vn());
        let birthday = page.patient.lock_ref().as_ref().and_then(|pt| pt.birthday());
        if let Some(vn) = vn_opt {
            if !vn.is_empty() {
                app.async_load(
                    true,
                    clone!(app, page => async move {

                        let params = OpdErMedicalHistoryParams {
                            vn: Some(vn),
                            only_opdscreen: Some(true),
                            ..Default::default()
                        };
                        // GET `EndPoint::OpdErMedicalHistory`
                        match OpdErMedicalHistory::call_api_get(&params, app.state()).await {
                            Ok(response) => {
                                if let Some(opdscreen) = response.opdscreen {
                                    page.vs_date.set_neq(opdscreen.vstdate.map(|d| d.to_string()).unwrap_or_default());
                                    page.vs_time.set_neq(opdscreen.vsttime.map(|t| t.js_string()).unwrap_or_default());

                                    let bt = opdscreen.temperature.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default();
                                    let pr = opdscreen.pulse.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default();
                                    let rr = opdscreen.rr.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default();
                                    let sbp = opdscreen.bps.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default();
                                    let eye = opdscreen.gcs_e.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default();
                                    let verbal = opdscreen.gcs_v.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default();
                                    let movement = opdscreen.gcs_m.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default();

                                    if let Some(vs_datetime) = datetime_from_opt(opdscreen.vstdate, opdscreen.vsttime) {
                                        let empty_ews_concat = [&vs_datetime.js_string(), CONCAT_EMPTY].concat();
                                        page.scores.set(Scores::from_concat(&Some(empty_ews_concat), birthday, app.state()))
                                    }
                                    {
                                        let mut lock = page.scores.lock_mut();
                                        if let Some(scs) = lock.as_mut() {
                                            scs.set_item("bt", &bt);
                                            scs.set_item("pr", &pr);
                                            scs.set_item("rr", &rr);
                                            scs.set_item("sbp", &sbp);
                                            scs.set_item("eye", &eye);
                                            scs.set_item("verbal", &verbal);
                                            scs.set_item("movement", &movement);
                                        }
                                    }
                                    page.bt.set_neq(bt);
                                    page.pr.set_neq(pr);
                                    page.rr.set_neq(rr);
                                    page.sbp.set_neq(sbp);
                                    page.dbp.set_neq(opdscreen.bpd.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default());
                                    page.bw.set_neq(opdscreen.bw.and_then(zero_none).map(|f| f.to_string()).unwrap_or_default());
                                    page.height.set_neq(opdscreen.height.and_then(zero_none).map(|i| i.to_string()).unwrap_or_default());
                                    page.pain.set_neq(opdscreen.pain_score.and_then(zero_none).map(|i| i.to_string()).unwrap_or_default());

                                    page.eye.set_neq(eye);
                                    page.verbal.set_neq(verbal);
                                    page.movement.set_neq(movement);

                                    page.calculate_map();

                                    page.changed.set(true);
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }),
                )
            }
        }
    }

    pub fn render(vs_id: Mutable<u32>, vs_changed: Mutable<bool>, form_rendered: Mutable<bool>, page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Vital Sign");

        html!("div", {
            //.class("container-fluid")
            .style("width", "420px")
            .child(html!("div", {
                .class(class::CARD_BBLUE)
                .child(html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class(class::COL_MD12_RT)
                            .child_signal(page.vs_id.signal_cloned().map(clone!(vs_id, form_rendered => move |id_opt| {
                                id_opt.is_some().then(|| {
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .text(" เพิ่ม")
                                        .event(clone!(vs_id, form_rendered => move |_: events::Click| {
                                            vs_id.set(0);
                                            form_rendered.set(false);
                                        }))
                                    })
                                })
                            })))
                            .child_signal(page.vs_id.signal().map(clone!(app, page => move |opt| {
                                (opt.is_none() && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistory, false)).then(|| {
                            // .child_signal(map_ref!{
                            //     let id_opt = page.vs_id.signal_cloned(),
                            //     let is_opd_er = page.is_opd_er().map(|opt| opt.unwrap_or_default()) =>
                            //     *is_opd_er && id_opt.is_none()
                            // }.map(clone!(app, page => move |from_hosxp| {
                            //     (from_hosxp && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistory, false)).then(|| {
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_DOWNLOAD)}))
                                        .text(" ดึงข้อมูลจาก HOSxP")
                                        .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                            Self::get_vs_hosxp(page.clone(), app.clone())
                                        }), app.state()))
                                        // getVitalSignFromHOSxP(event)
                                    })
                                })
                            })))
                        }))
                        .child(html!("div", {
                            .class("col-md-12")
                            .child(html!("div", {
                                //.attr("id", "vital-sign-form")
                                .children([
                                    doms::form_inline_group_sm(clone!(page => move |group| { group
                                        .children([
                                            doms::label_group_for("vs_date","วันที่"),
                                            doms::date_picker(
                                                page.vs_date.clone(),
                                                page.changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "vs_date"),
                                                |s| s, always(None),
                                            ),
                                            doms::label_group_for("vs_time","เวลา"),
                                            doms::time_picker(
                                                page.vs_time.clone(),
                                                page.changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "vs_time"),
                                                |s| s, always(None),
                                            ),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_GRAY)
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    let now = js_now();
                                                    page.vs_date.set_neq(now.date().to_string());
                                                    page.vs_time.set_neq(now.time().js_string());
                                                    page.changed.set_neq(true);
                                                }))
                                            }),

                                        ])
                                    })),
                                    // html!("hr"),
                                    html!("nav", {
                                        .class("mt-3")
                                        .child(html!("div", {
                                            .class(class::NAV_TABS_T)
                                            //.attr("id", "nav-tab")
                                            .attr("role", "tablist")
                                            .children([
                                                html!("a", {
                                                    .class(class::NAV_ITEM_LINK_ACTIVE_P2)
                                                    .attr("id", "nav-vitalsign-tab")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#")
                                                    .text("VS")
                                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                        event.prevent_default();
                                                        page.active_tab.set_neq(Tab::VitalSign);
                                                    }))
                                                }),
                                                html!("a", {
                                                    .class(class::NAV_ITEM_LINK_P2)
                                                    .attr("id", "nav-neuro-tab")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#")
                                                    .text("NS")
                                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                        event.prevent_default();
                                                        page.active_tab.set_neq(Tab::NeuroSign);
                                                    }))
                                                }),
                                                html!("a", {
                                                    .class(class::NAV_ITEM_LINK_P2)
                                                    .attr("id", "nav-score-tab")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#")
                                                    .text("Score")
                                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                        event.prevent_default();
                                                        page.active_tab.set_neq(Tab::Score);
                                                    }))
                                                }),
                                                html!("a", {
                                                    .class(class::NAV_ITEM_LINK_P2)
                                                    .attr("id", "nav-had-tab")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#")
                                                    .text("HAD")
                                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                        event.prevent_default();
                                                        page.active_tab.set_neq(Tab::Had);
                                                    }))
                                                }),
                                                html!("a", {
                                                    .class(class::NAV_ITEM_LINK_P2)
                                                    .attr("id", "nav-o2-tab")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#")
                                                    .text("O")
                                                    .child(html!("sub",{.text("2")}))
                                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                        event.prevent_default();
                                                        page.active_tab.set_neq(Tab::O2);
                                                    }))
                                                }),
                                                html!("a", {
                                                    .class(class::NAV_ITEM_LINK_P2)
                                                    .attr("id", "nav-lr-tab")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#")
                                                    .text("LR")
                                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                        event.prevent_default();
                                                        page.active_tab.set_neq(Tab::Lr);
                                                    }))
                                                }),
                                                html!("a", {
                                                    .class(class::NAV_ITEM_LINK_P2)
                                                    .attr("id", "nav-other-tab")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#")
                                                    .text("Other")
                                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                        event.prevent_default();
                                                        page.active_tab.set_neq(Tab::Other);
                                                    }))
                                                }),
                                            ])
                                        }))
                                    }),
                                    // html!("hr"),
                                    html!("div", {
                                        .class("tab-content")
                                        //.attr("id", "nav-vs-tabContent")
                                        .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                                            Some(match tab {
                                                Tab::VitalSign => Self::render_tab_vs(page.clone(), app.clone()),
                                                Tab::NeuroSign => Self::render_tab_neuro(page.clone(), app.clone()),
                                                Tab::Score => Self::render_tab_score(page.clone(), app.clone()),
                                                Tab::Had => Self::render_tab_had(page.clone()),
                                                Tab::O2 => Self::render_tab_o2(page.clone(), app.clone()),
                                                Tab::Lr => Self::render_tab_lr(page.clone(), app.clone()),
                                                Tab::Other => Self::render_tab_other(page.clone(), app.clone()),
                                            })
                                        })))
                                    }),
                                    html!("hr"),
                                ])
                                .child_signal(map_ref!{
                                    let id_opt = page.vs_id.signal_cloned(),
                                    let (is_ipd, is_pre_admit) = page.is_ipd_and_is_pre_admit() =>
                                    (*id_opt, *is_ipd, *is_pre_admit)
                                }.map(clone!(app, page, vs_id, vs_changed, form_rendered => move |(id_opt, is_ipd, is_pre_admit)| {
                                    id_opt.and_then(clone!(app, page, vs_id, vs_changed, form_rendered => move |id| {
                                        (if is_ipd {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdVitalSignId, is_pre_admit)
                                        } else {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErVitalSignId, false)
                                        }).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_RED)
                                                .child(html!("i", {.class(class::FA_TRASH)}))
                                                .text(" ลบ")
                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                    if let Some(patient) = page.patient.lock_ref().as_ref() {
                                                        Self::delete_vs(patient.visit_type(), id, vs_id.clone(), vs_changed.clone(), form_rendered.clone(), app.clone())
                                                    }
                                                }), app.state()))
                                            })
                                        })
                                    }))
                                })))
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_FR_GRAY)
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .text(" ยกเลิก")
                                    .event(clone!(form_rendered => move |_: events::Click| {
                                        form_rendered.set(false);
                                    }))
                                }))
                                .child_signal(map_ref!{
                                    let (is_ipd, is_pre_admit) = page.is_ipd_and_is_pre_admit(),
                                    let id_opt = page.vs_id.signal_cloned() =>
                                    (*id_opt, *is_ipd, *is_pre_admit)
                                }.map(clone!(app, page, vs_id, vs_changed => move |(id_opt, is_ipd, is_pre_admit)| {
                                    (match (is_ipd, id_opt.is_none()) {
                                        (true, true) => app.endpoint_is_allow(&Method::POST, &EndPoint::IpdVitalSign, is_pre_admit),
                                        (true, false) => app.endpoint_is_allow(&Method::PUT, &EndPoint::IpdVitalSign, is_pre_admit),
                                        (false, true) => app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErVitalSign, false),
                                        (false, false) => app.endpoint_is_allow(&Method::PUT, &EndPoint::OpdErVitalSign, false),
                                    }).then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_FR_R)
                                            .class_signal("btn-primary", page.changed.signal())
                                            .class_signal("btn-secondary", not(page.changed.signal()))
                                            .child(html!("i", {.class(class::FA_SAVE)}))
                                            .text(" บันทึก")
                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, vs_id, vs_changed => move || {
                                                Self::submit(vs_id.clone(), vs_changed.clone(), page.clone(), app.clone());
                                            }), not(page.changed.signal()), app.state()))
                                        })
                                    })
                                })))
                                .child_signal(page.action_id.signal().map(|action_id| {
                                    action_id.is_some().then(|| {
                                        html!("i", {
                                            .class(class::FA_ALERT_GOLD)
                                            .class(class::FLOAT_RB1)
                                            .style("font-size", "24px")
                                            .attr("title", "รายการนี้ ผูกกับ Action, ท่านต้องแก้ไขข้อมูลใน Action ให้ตรงกันด้วยตนเอง")
                                        })
                                    })
                                }))
                            }))
                            // modals
                            .children([
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "selectLabWBCModal")
                                    .attr("role", "dialog")
                                    .attr("tabindex", "-1")
                                    .child_signal(page.wbc_modal.signal_cloned().map(clone!(app => move |opt| {
                                        opt.as_ref().map(clone!(app => move |modal| LabWbc::render(modal.clone(), app))).or(Some(blank_modal()))
                                    })))
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "aggressionOASModal")
                                    .attr("role", "dialog")
                                    .attr("tabindex", "-1")
                                    .child_signal(page.aggression_oas_modal.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|modal| AggressionOAS::render(modal.clone())).or(Some(blank_modal()))
                                    }))
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
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "maasModal")
                                    .attr("role", "dialog")
                                    .attr("tabindex", "-1")
                                    .child_signal(page.maas_modal.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|modal| MotorActivityMaas::render(modal.clone())).or(Some(blank_modal()))
                                    }))
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "barthelIndexModal")
                                    .attr("role", "dialog")
                                    .attr("tabindex", "-1")
                                    .child_signal(page.barthel_index_modal.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|modal| BarthelIndex::render(modal.clone())).or(Some(blank_modal()))
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
                                    .attr("id", "alcoholAiwaArModal")
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
                                    .attr("id", "suicide8QModal")
                                    .attr("role", "dialog")
                                    .attr("tabindex", "-1")
                                    .child_signal(page.suicide_8q_modal.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|modal| Suicide8Q::render(modal.clone())).or(Some(blank_modal()))
                                    }))
                                }),
                            ])
                        }))
                    }))
                }))
            }))
        })
    }

    fn render_tab_vs(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (
            breathing_select_option,
            avpu_select_option,
            gut_feeling_select_option,
            pops_other_select_option,
            o2_select_option,
            conscious_select_option,
            urine_amount_select_option,
            urine_duration_select_option,
        ) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|a| {
                (
                    a.breathing_select_option.clone(),
                    a.avpu_select_option.clone(),
                    a.gut_feeling_select_option.clone(),
                    a.pops_other_select_option.clone(),
                    a.o2_select_option.clone(),
                    a.conscious_select_option.clone(),
                    a.urine_amount_select_option.clone(),
                    a.urine_duration_select_option.clone(),
                )
            })
            .unwrap_or_default();

        html!("div", {
            // .class(class::TAB_FADE_SHOW_ACTIVE)
            //.attr("id", "nav-vitalsign")
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-vitalsign-tab")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("bt", "Temperature", true),
                        input_number("bt", page.bt.clone(), page.changed.clone(), 1, Some("0"), Some("50"), Some("\u{00b0}C"), Some((page.scores.clone(), "bt"))),
                        ews_badges(page.scores.clone(), "bt"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("pr", "Pulse Rate", true),
                        input_number("pr", page.pr.clone(), page.changed.clone(), 0, Some("0"), Some("300"), Some("beat/min"), Some((page.scores.clone(), "pr"))),
                        ews_badges(page.scores.clone(), "pr"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("rr", "Respiratory Rate", true),
                        input_number("rr", page.rr.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("breath/min"), Some((page.scores.clone(), "rr"))),
                        ews_badges(page.scores.clone(), "rr"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("respirator"))
                    .children([
                        html!("div", {
                            .class(class::COL_SM5_OFSM4)
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    checkbox_y("respirator", page.respirator.clone(), page.changed.clone(), Some((page.scores.clone(), "respirator"))),
                                    doms::label_check_for("respirator", "Respirator"),
                                ])
                            }))
                        }),
                        ews_badges(page.scores.clone(), "respirator"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("sbp", "Systolic BP", true),
                        input_number("sbp", page.sbp.clone(), page.changed.clone(), 0, Some("0"), Some("300"), Some("mmHg"), Some((page.scores.clone(), "sbp"))),
                        ews_badges(page.scores.clone(), "sbp"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("dbp", "Diastolic BP", true),
                        input_number("dbp", page.dbp.clone(), page.changed.clone(), 0, Some("0"), Some("200"), Some("mmHg"), Some((Mutable::new(None),""))),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .attr("title", "Mean Arterial Pressure")
                    .children([
                        label_for("map", "MAP", true),
                        html!("div", {
                            .class(class::COL_SM5_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "map")
                                        .apply(mixins::string_value(page.map.clone(), page.changed.clone()))
                                    }),
                                    doms::span_group_text("mmHg"),
                                    html!("button", {
                                        .class(class::BTN_GRAY)
                                        .attr("type", "button")
                                        .child(html!("i", {.class(class::FA_CALCULATOR)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            page.calculate_map();
                                        }))
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("inotrope"))
                    .children([
                        html!("div", {
                            .class(class::COL_SM5_OFSM4)
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    checkbox_y("inotrope", page.inotrope.clone(), page.changed.clone(), Some((page.scores.clone(), "inotrope"))),
                                    doms::label_check_for("inotrope", "Inotrope"),
                                ])
                            }))
                        }),
                        ews_badges(page.scores.clone(), "inotrope"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("sat_ra", "O\u{2082} sat Room Air", true),
                        input_number("sat_room_air", page.sat_room_air.clone(), page.changed.clone(), 0, Some("0"), Some("100"), Some("%"), Some((Mutable::new(None),""))),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("sat", "O\u{2082} Saturation", true),
                        input_number("sat", page.sat.clone(), page.changed.clone(), 0, Some("0"), Some("100"), Some("%"), Some((page.scores.clone(), "sat"))),
                        ews_badges(page.scores.clone(), "sat"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    // .visible_signal(page.is_scorable("o2_id"))
                    .children([
                        label_for("o2_id_ews", "O\u{2082}", true),
                        select_options("o2_id_ews", page.o2_id.clone(), page.changed.clone(), &o2_select_option, "Room air", Some((page.scores.clone(), "o2_id"))),
                        ews_badges(page.scores.clone(), "o2_id"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("o2_flow_vs", "O\u{2082} Flow", true),
                        input_number("o2_flow_vs", page.o2_flow.clone(), page.changed.clone(), 1, Some("0"), Some("99"), Some("L/min"), Some((Mutable::new(None),""))),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("breathing_id"))
                    .children([
                        label_for("breathing_id", "Breathing", true),
                        select_options("breathing_id", page.breathing_id.clone(), page.changed.clone(), &breathing_select_option, "", Some((page.scores.clone(), "breathing_id"))),
                        ews_badges(page.scores.clone(), "breathing_id"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("gut_feeling_id"))
                    .children([
                        label_for("gut_feeling_id", "Gut Feeling", true),
                        select_options("gut_feeling_id", page.gut_feeling_id.clone(), page.changed.clone(), &gut_feeling_select_option, "", Some((page.scores.clone(), "gut_feeling_id"))),
                        ews_badges(page.scores.clone(), "gut_feeling_id"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("avpu_id"))
                    .children([
                        label_for("avpu_id", "AVPU", true),
                        select_options("avpu_id", page.avpu_id.clone(), page.changed.clone(), &avpu_select_option, "", Some((page.scores.clone(), "avpu_id"))),
                        ews_badges(page.scores.clone(), "avpu_id"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("pops_other_id"))
                    .children([
                        // POPS, PEWS
                        label_for("pops_other_id", "EWS Other", true),
                        select_options("pops_other_id", page.pops_other_id.clone(), page.changed.clone(), &pops_other_select_option, "", Some((page.scores.clone(), "pops_other_id"))),
                        ews_badges(page.scores.clone(), "pops_other_id"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("crt"))
                    .children([
                        label_for("crt_ews", "Capillary Refill Time", true),
                        input_number("crt_ews", page.crt.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("sec"), Some((page.scores.clone(), "crt"))),
                        ews_badges(page.scores.clone(), "crt"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("conscious_id"))
                    .children([
                        label_for("conscious_id_ews", "Conscious", true),
                        select_options("conscious_id_ews", page.conscious_id.clone(), page.changed.clone(), &conscious_select_option, "", Some((page.scores.clone(), "conscious_id"))),
                        ews_badges(page.scores.clone(), "conscious_id"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("eye"))
                    .children([
                        label_for("eye_ews", "Eye", true),
                        select_options("eye_ews", page.eye.clone(), page.changed.clone(), &[
                            SelectOption {key: String::from("1"), value: String::from("1")},
                            SelectOption {key: String::from("2"), value: String::from("2")},
                            SelectOption {key: String::from("3"), value: String::from("3")},
                            SelectOption {key: String::from("4"), value: String::from("4")},
                        ], "", Some((page.scores.clone(), "eye"))),
                        ews_badges(page.scores.clone(), "eye"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("verbal"))
                    .children([
                        label_for("verbal_ews", "Verbal", true),
                        select_options("verbal_ews", page.verbal.clone(), page.changed.clone(), &[
                            SelectOption {key: String::from("T"), value: String::from("T")},
                            SelectOption {key: String::from("1"), value: String::from("1")},
                            SelectOption {key: String::from("2"), value: String::from("2")},
                            SelectOption {key: String::from("3"), value: String::from("3")},
                            SelectOption {key: String::from("4"), value: String::from("4")},
                            SelectOption {key: String::from("5"), value: String::from("5")},
                        ], "", Some((page.scores.clone(), "verbal"))),
                        ews_badges(page.scores.clone(), "verbal"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("movement"))
                    .children([
                        label_for("movement_ews", "Movement", true),
                        select_options("movement_ews", page.movement.clone(), page.changed.clone(), &[
                            SelectOption {key: String::from("1"), value: String::from("1")},
                            SelectOption {key: String::from("2"), value: String::from("2")},
                            SelectOption {key: String::from("3"), value: String::from("3")},
                            SelectOption {key: String::from("4"), value: String::from("4")},
                            SelectOption {key: String::from("5"), value: String::from("5")},
                            SelectOption {key: String::from("6"), value: String::from("6")},
                        ], "", Some((page.scores.clone(), "movement"))),
                        ews_badges(page.scores.clone(), "movement"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("wbc", "WBC", true),
                        html!("div", {
                            .class(class::COL_SM5_P0S2)
                            .child(html!("div", {
                                .future(page.wbc.signal_cloned().for_each(clone!(page => move |wbc| {
                                    let mut lock = page.scores.lock_mut();
                                    if let Some(scores) = lock.as_mut() {
                                        scores.set_item("wbc", &wbc)
                                    }
                                    async {}
                                })))
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .attr("step", "0.01")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "wbc")
                                        .apply(mixins::string_value(page.wbc.clone(), page.changed.clone()))
                                        // we set scores argument to 'None' because we set wbc score with future above
                                        .apply(input_score_mixins(None))
                                    }),
                                    html!("span", {
                                        .class("input-group-text")
                                        .text("x 10")
                                        .child(html!("sup", {.text("3")}))
                                    }),
                                ])
                                .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabWbcKeyValue, false), |dom| dom
                                    .child(html!("button", {
                                        .class(class::BTN_GRAY)
                                        .attr("type", "button")
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#selectLabWBCModal")
                                        .child(html!("i", {.class(class::FA_FLASK)}))
                                        .event(clone!(page => move |_:events::Click| {
                                            if let Some(patient) = page.patient.lock_ref().as_ref() {
                                                // HosXp store `an` and `vn` in the same column `vn`
                                                page.wbc_modal.set(Some(LabWbc::new(
                                                    String::from("vn"),
                                                    patient.visit_type.vnan().to_owned(),
                                                    page.wbc.clone(),
                                                    page.band.clone(),
                                                    page.changed.clone(),
                                                )));
                                            }
                                        }))
                                    }))
                                )
                            }))
                        }),
                        ews_badges(page.scores.clone(), "wbc"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .future(page.band.signal_cloned().for_each(clone!(page => move |band| {
                        let mut lock = page.scores.lock_mut();
                        if let Some(scores) = lock.as_mut() {
                            scores.set_item("band", &band)
                        }
                        async {}
                    })))
                    .children([
                        label_for("band", "Band", true),
                        // we set scores argument to 'None' because we set band score with future above
                        input_number("band", page.band.clone(), page.changed.clone(), 0, Some("0"), Some("100"), Some("%"), Some((Mutable::new(None),""))),
                        ews_badges(page.scores.clone(), "band"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("urine_amount"))
                    .children([
                        label_for("urine_amount", "Urine (ปริมาณ)", true),
                        select_options("urine_amount", page.urine_amount.clone(), page.changed.clone(), &urine_amount_select_option, "", Some((page.scores.clone(), "urine_amount"))),
                        ews_badges(page.scores.clone(), "urine_amount"),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .visible_signal(page.is_scorable("urine_duration"))
                    .children([
                        label_for("urine_duration", "Urine (ระยะเวลา)", true),
                        select_options("urine_duration", page.urine_duration.clone(), page.changed.clone(), &urine_duration_select_option, "", Some((page.scores.clone(), "urine_duration"))),
                        ews_badges(page.scores.clone(), "urine_duration"),
                    ])
                }),
            ])
            .child_signal(page.scores.signal_ref(|opt| opt.as_ref().map(|sc| {
                html!("div", {
                    .class("row")
                    .children([
                        ews_total(sc.ews.label(), sc.ews.score(), &sc.ews.title(), sc.ews.color_total(), sc.ews.bg_color_total()),
                        ews_total(sc.qsofa.label(), sc.qsofa.score(), &sc.qsofa.title(), sc.qsofa.color_total(), sc.qsofa.bg_color_total()),
                        ews_total(sc.sirs.label(), sc.sirs.score(), &sc.sirs.title(), sc.sirs.color_total(), sc.sirs.bg_color_total()),
                    ])
                })
            })))
            .children([
                html!("hr"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("urine", "Urine (ครั้ง/12 ชม.)", false),
                        input_text("urine", page.urine.clone(), page.changed.clone(), Some(50), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .child(html!("div", {
                        .class(class::COL_SM6_OFSM5)
                        .child(html!("div", {
                            .class("form-check")
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .class("form-check-input")
                                    .attr("type", "checkbox")
                                    .attr("id", "catheter")
                                    .apply(mixins::checkbox_toggle(page.catheter.clone(), page.changed.clone(), "Y", ""))
                                    // .future(page.catheter.signal_cloned().for_each(clone!(page => move |cat| {
                                    //     if cat == "Y" {
                                    //         page.urine.set_neq(String::from("ใส่สายสวนฯ"));
                                    //     };
                                    //     async {}
                                    // })))
                                }),
                                doms::label_check_for("catheter", "ใส่สายสวนปัสสาวะ"),
                            ])
                        }))
                    }))
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("feces", "Feces (ครั้ง/12 ชม.)", false),
                        input_text("feces", page.feces.clone(), page.changed.clone(), Some(50), None, None),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("diet", "Diet", false),
                        input_text("diet", page.diet.clone(), page.changed.clone(), Some(20), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("bw", "Body Weight", false),
                        input_number("bw", page.bw.clone(), page.changed.clone(), 3, Some("1"), Some("999"), Some("kg"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("height", "Height", false),
                        input_number("height", page.height.clone(), page.changed.clone(), 0, Some("1"), Some("999"), Some("cm."), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("aggression_oas_vs", "OAS", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "aggression_oas_vs")
                                        .attr("min","0")
                                        .attr("max","3")
                                        .apply(mixins::string_value(page.aggression_oas.clone(), page.changed.clone()))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#aggressionOASModal")
                                        .child(html!("i", {.class(class::FA_EDIT)}))
                                        .event(clone!(page => move |_:events::Click| {
                                            page.aggression_oas_modal.set(Some(AggressionOAS::new(
                                                page.aggression_oas.clone(),
                                                page.changed.clone(),
                                            )));
                                        }))
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_tab_neuro(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (cha_select_option, motor_select_option, conscious_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|a| (a.cha_select_option.clone(), a.motor_select_option.clone(), a.conscious_select_option.clone()))
            .unwrap_or_default();

        html!("div", {
            // .class(class::TAB_FADE)
            //.attr("id", "nav-neuro")
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-neuro-tab")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("conscious_id", "Conscious", false),
                        select_options("conscious_id", page.conscious_id.clone(), page.changed.clone(), &conscious_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("eye", "Eye", false),
                        select_options("eye", page.eye.clone(), page.changed.clone(), &[
                            SelectOption {key: String::from("1"), value: String::from("1")},
                            SelectOption {key: String::from("2"), value: String::from("2")},
                            SelectOption {key: String::from("3"), value: String::from("3")},
                            SelectOption {key: String::from("4"), value: String::from("4")},
                        ], "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("verbal", "Verbal", false),
                        select_options("verbal", page.verbal.clone(), page.changed.clone(), &[
                            SelectOption {key: String::from("T"), value: String::from("T")},
                            SelectOption {key: String::from("1"), value: String::from("1")},
                            SelectOption {key: String::from("2"), value: String::from("2")},
                            SelectOption {key: String::from("3"), value: String::from("3")},
                            SelectOption {key: String::from("4"), value: String::from("4")},
                            SelectOption {key: String::from("5"), value: String::from("5")},
                        ], "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("movement", "Movement", false),
                        select_options("movement", page.movement.clone(), page.changed.clone(), &[
                            SelectOption {key: String::from("1"), value: String::from("1")},
                            SelectOption {key: String::from("2"), value: String::from("2")},
                            SelectOption {key: String::from("3"), value: String::from("3")},
                            SelectOption {key: String::from("4"), value: String::from("4")},
                            SelectOption {key: String::from("5"), value: String::from("5")},
                            SelectOption {key: String::from("6"), value: String::from("6")},
                        ], "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("right_pupil", "Right Pupil", false),
                        input_number("right_pupil", page.right_pupil.clone(), page.changed.clone(), 1, Some("0"), Some("9"), Some("mm."), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("right_cha_id", "Right Pupil Response", false),
                        select_options("right_cha_id", page.right_cha_id.clone(), page.changed.clone(), &cha_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("left_pupil", "Left Pupil", false),
                        input_number("left_pupil", page.left_pupil.clone(), page.changed.clone(), 1, Some("0"), Some("9"), Some("mm."), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("left_cha_id", "Left Pupil Response", false),
                        select_options("left_cha_id", page.left_cha_id.clone(), page.changed.clone(), &cha_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lt_arm", "Lt Arm Power", false),
                        select_options("lt_arm", page.lt_arm.clone(), page.changed.clone(), &motor_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lt_leg", "Lt Leg Power", false),
                        select_options("lt_leg", page.lt_leg.clone(), page.changed.clone(), &motor_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("rt_arm", "Rt Arm Power", false),
                        select_options("rt_arm", page.rt_arm.clone(), page.changed.clone(), &motor_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("rt_leg", "Rt Leg Power", false),
                        select_options("rt_leg", page.rt_leg.clone(), page.changed.clone(), &motor_select_option, "", None),
                    ])
                }),
            ])
        })
    }

    fn render_tab_score(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (va_select_option, mass_select_option, stage_of_change_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|a| (a.va_select_option.clone(), a.mass_select_option.clone(), a.stage_of_change_select_option.clone()))
            .unwrap_or_default();

        html!("div", {
            // .class(class::TAB_FADE)
            //.attr("id", "nav-score")
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-score-tab")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("pain", "Pain", false),
                        select_options("pain", page.pain.clone(), page.changed.clone(), &["0","1","2","3","4","5","6","7","8","9","10"].into_iter().map(|s| {
                            SelectOption {key: String::from(s), value: String::from(s)}
                        }).collect::<Vec<SelectOption>>(), "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    // .style("display", "none")
                    .children([
                        label_for("severity", "Severity Score", false),
                        input_number("severity", page.severity.clone(), page.changed.clone(), 0, None, None, None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        // TODO: What is VA? (scale with value A, B, C, D)
                        label_for("va_id", "VA", false),
                        select_options("va_id", page.va_id.clone(), page.changed.clone(), &va_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .attr("title", "Motor Activity Accessment Scale")
                    .children([
                        label_for("mass_id", "MAAS", false),
                        // select_options("mass_id", page.mass_id.clone(), page.changed.clone(), &mass_select_option, "", None),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .attr("id", "mass_id")
                                        .children(mass_select_option.iter().map(|option| doms::select_option(option, "")))
                                        .apply(mixins::string_value_select(page.mass_id.clone(), page.changed.clone()))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#maasModal")
                                        .child(html!("i", {.class(class::FA_EDIT)}))
                                        .event(clone!(page => move |_:events::Click| {
                                            page.maas_modal.set(Some(MotorActivityMaas::new(
                                                page.mass_id.clone(),
                                                page.changed.clone(),
                                            )));
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
                        label_not_for("Braden Scale", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
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
                                                page.braden.clone(),
                                                page.changed.clone(),
                                            )));
                                        }))
                                    }),
                                    html!("div", {
                                        .class("input-group-text")
                                        .style_signal("background-color", page.braden.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                            match score {
                                                ..10 => "crimson",
                                                10..13 => "salmon",
                                                13..15 => "pink",
                                                15..19 => "gold",
                                                19.. => "inherit"
                                            }
                                        })))
                                        .text_signal(page.braden.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                            let value = match score {
                                                ..10 => "เสี่ยงสูงมาก",
                                                10..13 => "เสี่ยงสูง",
                                                13..15 => "เสี่ยงปากกลาง",
                                                15..19 => "มีความเสี่ยง",
                                                19.. => "ไม่มีความเสี่ยง"
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
                        label_not_for("Barthel ADL", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#barthelIndexModal")
                                        .child(html!("i", {.class(class::FA_EDIT)}))
                                        .event(clone!(page => move |_:events::Click| {
                                            page.barthel_index_modal.set(Some(BarthelIndex::new(
                                                page.barthel_index.clone(),
                                                page.changed.clone(),
                                            )));
                                        }))
                                    }),
                                    html!("div", {
                                        .class("input-group-text")
                                        .style_signal("background-color", page.barthel_index.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                            match score {
                                                ..20 => "crimson",
                                                20..40 => "salmon",
                                                40..60 => "pink",
                                                60..80 => "gold",
                                                80.. => "inherit",
                                            }
                                        })))
                                        .text_signal(page.barthel_index.signal_ref(|concat| concat.split(',').nth(0).and_then(|s| s.parse::<u8>().ok()).map(|score| {
                                            let value = match score {
                                                ..20 => "พึ่งพาโดยสมบูรณ์",
                                                20..40 => "พึ่งพารุนแรง",
                                                40..60 => "พึ่งพาปานกลาง",
                                                60..80 => "พึ่งพาเล็กน้อย",
                                                80.. => "ไม่มีภาวะพึ่งพา",
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
                        label_not_for("AWQv2", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
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
                                    html!("div", {
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
                                    })
                                })))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_not_for("2Q", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
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
                        label_not_for("9Q", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
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
                        label_not_for("8Q", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
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
                        label_not_for("OAS", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#aggressionOASModal")
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
                        label_for("motivation_scale", "Motivation scale", false),
                        select_options("motivation_scale", page.motivation_scale.clone(), page.changed.clone(), &["0","1","2","3","4","5","6","7","8","9","10"].into_iter().map(|s| {
                            SelectOption {key: String::from(s), value: String::from(s)}
                        }).collect::<Vec<SelectOption>>(), "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("craving_scale", "Craving scale", false),
                        select_options("craving_scale", page.craving_scale.clone(), page.changed.clone(), &["0","1","2","3","4","5","6","7","8","9","10"].into_iter().map(|s| {
                            SelectOption {key: String::from(s), value: String::from(s)}
                        }).collect::<Vec<SelectOption>>(), "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("stage_of_change_id", "Stage of change", false),
                        select_options("stage_of_change_id", page.stage_of_change_id.clone(), page.changed.clone(), &stage_of_change_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_not_for("CIWA-Ar", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#alcoholAiwaArModal")
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
                                                ..8 => "อาการเล็กน้อย",
                                                8..15 => "อาการปานกลาง",
                                                15..20 => "อาการรุนแรง",
                                                20.. => "อาการรุนแรงมาก",
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
                        label_not_for("AWS", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
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
                                                ..5 => "อาการเล็กน้อย",
                                                5..10 => "อาการปานกลาง",
                                                10..15 => "อาการรุนแรง",
                                                15.. => "อาการรุนแรงมาก",
                                            };
                                            [&score.to_string(), " : ", value].concat()
                                        }).unwrap_or(String::from("รอการประเมิน"))))
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_tab_had(page: Rc<Self>) -> Dom {
        html!("div", {
            // .class(class::TAB_FADE)
            //.attr("id", "nav-had")
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-had-tab")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("had_name", "High Alert Drug", false),
                        input_text("had_name", page.had_name.clone(), page.changed.clone(), Some(200), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("had_drop", "HAD Drop Rate", false),
                        input_text("had_drop", page.had_drop.clone(), page.changed.clone(), Some(200), None, None),
                    ])
                }),
                // same as info-other
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("other-had", "Other", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("id", "other-had")
                                .attr("rows", "3")
                                .apply(mixins::textarea_value_auto_expand(page.other.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_tab_o2(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (o2_select_option, tube_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|a| (a.o2_select_option.clone(), a.tube_select_option.clone()))
            .unwrap_or_default();

        html!("div", {
            // .class(class::TAB_FADE)
            //.attr("id", "nav-o2")
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-o2-tab")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("o2_id", "O\u{2082}", false),
                        select_options("o2_id", page.o2_id.clone(), page.changed.clone(), &o2_select_option, "Room air", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("o2_flow", "O\u{2082} Flow", false),
                        input_number("o2_flow", page.o2_flow.clone(), page.changed.clone(), 1, Some("0"), Some("99"), Some("L/min"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("tube_id", "Tube", false),
                        select_options("tube_id", page.tube_id.clone(), page.changed.clone(), &tube_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("tube_no", "Tube No.", false),
                        input_number("tube_no", page.tube_no.clone(), page.changed.clone(), 1, Some("0"), Some("99"), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("tube_mark", "Tube Mark", false),
                        input_number("tube_mark", page.tube_mark.clone(), page.changed.clone(), 1, Some("0"), Some("99"), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("ventilator_name", "Ventilator Name", false),
                        input_text("ventilator_name", page.ventilator_name.clone(), page.changed.clone(), Some(50), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("mode", "Ventilator Mode", false),
                        input_text("mode", page.mode.clone(), page.changed.clone(), Some(50), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("tv", "Tidal Volume", false),
                        input_number("tv", page.tv.clone(), page.changed.clone(), 0, Some("0"), Some("9999"), Some("mL"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .attr("title", "Peak Inspiratory Pressure")
                    .children([
                        label_for("pip", "PIP", false),
                        input_number("pip", page.pip.clone(), page.changed.clone(), 0, Some("0"), Some("999"), Some("cmH\u{2082}O"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("r_rate", "Respiratory Rate", false),
                        input_number("r_rate", page.r_rate.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("/min"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("i_rate", "I:E Ratio", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "i_rate")
                                        .apply(mixins::string_value(page.i_rate.clone(), page.changed.clone()))
                                    }),
                                    doms::span_group_text(":"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "e_rate")
                                        .apply(mixins::string_value(page.e_rate.clone(), page.changed.clone()))
                                    }),
                                ])
                            }))
                        })
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("ti", "Inspiratory Time", false),
                        input_number("ti", page.ti.clone(), page.changed.clone(), 2, Some("0"), Some("99"), Some("sec"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("ps", "Pressure Support", false),
                        input_number("ps", page.ps.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("cmH\u{2082}O"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .attr("title", "Fraction of Inspired O\u{2082}")
                    .children([
                        label_for("fio2", "FiO\u{2082}", false),
                        input_number("fio2", page.fio2.clone(), page.changed.clone(), 1, Some("0"), Some("1"), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .attr("title", "Positive End-Expiratory Pressure")
                    .children([
                        label_for("peep", "PEEP", false),
                        input_number("peep", page.peep.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("cmH\u{2082}O"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("ft", "Flow Trigger", false),
                        input_number("ft", page.ft.clone(), page.changed.clone(), 1, Some("0"), Some("99"), Some("L/min"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("delta_p", "Delta Pressure", false),
                        input_number("delta_p", page.delta_p.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("cmH\u{2082}O"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("o2_map", "Mean Airway Pressure", false),
                        input_number("o2_map", page.o2_map.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("cmH\u{2082}O"), None),
                    ])
                }),
            ])
        })
    }

    fn render_tab_lr(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (lr_sta_select_option, lr_mem_select_option, lr_moulding_select_option, dipstick_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|a| {
                (
                    a.lr_sta_select_option.clone(),
                    a.lr_mem_select_option.clone(),
                    a.lr_moulding_select_option.clone(),
                    a.dipstick_select_option.clone(),
                )
            })
            .unwrap_or_default();

        html!("div", {
            // .class(class::TAB_FADE)
            //.attr("id", "nav-lr")
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-lr-tab")
            .children([

                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_int_m", "Interval", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "lr_int_m")
                                        .apply(mixins::string_value(page.lr_int_m.clone(), page.changed.clone()))
                                    }),
                                    doms::span_group_text("m"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "number")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "lr_int_s")
                                        .apply(mixins::string_value(page.lr_int_s.clone(), page.changed.clone()))
                                    }),
                                    doms::span_group_text("s"),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_dur", "Duration", false),
                        input_number("lr_dur", page.lr_dur.clone(), page.changed.clone(), 0, Some("0"), Some("999"), Some("sec"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_fsh", "Fetal Heart Sound", false),
                        input_number("lr_fsh", page.lr_fsh.clone(), page.changed.clone(), 0, Some("0"), Some("999"), Some("beat/min"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_sev", "Severity", false),
                        input_text("lr_sev", page.lr_sev.clone(), page.changed.clone(), Some(50), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_cer", "Cervix", false),
                        input_number("lr_cer", page.lr_cer.clone(), page.changed.clone(), 0, Some("0"), Some("10"), Some("cm."), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_eff", "Effacement", false),
                        input_number("lr_eff", page.lr_eff.clone(), page.changed.clone(), 0, Some("0"), Some("100"), Some("%"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_sta", "Station", false),
                        select_options("lr_sta", page.lr_sta.clone(), page.changed.clone(), &lr_sta_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_mem", "Membrane", false),
                        select_options("lr_mem", page.lr_mem.clone(), page.changed.clone(), &lr_mem_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_af", "Amniotic Fluid", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("select" => HtmlSelectElement, {
                                .class(class::FORM_SELECT_SM)
                                .attr("id", "lr_af")
                                .children([
                                    html!("option", {.attr("value", "")}),
                                    html!("option", {.attr("value", "C").text("Clear")}),
                                    html!("option", {.attr("value", "A").text("Absent")}),
                                    html!("option", {.attr("value", "B").text("Blood")}),
                                    html!("option", {.attr("value", "M₁").text("Meconium(lightly)")}),
                                    html!("option", {.attr("value", "M₂").text("Meconium(thick)")}),
                                    html!("option", {.attr("value", "M₃").text("Meconium(very thick)")}),
                                ])
                                .apply(mixins::string_value_select(page.lr_af.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_pos", "Fetal Position", false),
                        input_text("lr_pos", page.lr_pos.clone(), page.changed.clone(), Some(3), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_moulding", "Moulding", false),
                        select_options("lr_moulding", page.lr_moulding.clone(), page.changed.clone(), &lr_moulding_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_oxytocin_unit", "Oxytocin", false),
                        input_number("lr_oxytocin_unit", page.lr_oxytocin_unit.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("Unit/L"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_oxytocin_rate", "Oxytocin Rate", false),
                        input_number("lr_oxytocin_rate", page.lr_oxytocin_rate.clone(), page.changed.clone(), 0, Some("0"), Some("999"), Some("drops/min"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("urine_protein", "Urine Protein", false),
                        select_options("urine_protein", page.urine_protein.clone(), page.changed.clone(), &dipstick_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("urine_sugar", "Urine Sugar", false),
                        select_options("urine_sugar", page.urine_sugar.clone(), page.changed.clone(), &dipstick_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("lr_urine_vol", "Urine Volume", false),
                        input_number("lr_urine_vol", page.lr_urine_vol.clone(), page.changed.clone(), 0, Some("0"), Some("99999"), Some("mL (if void)"), None),
                    ])
                }),
            ])
        })
    }

    fn render_tab_other(page: Rc<Self>, app: Rc<App>) -> Dom {
        let line_select_option = app.app_asset.lock_ref().as_ref().map(|a| a.line_select_option.clone()).unwrap_or_default();

        html!("div", {
            // .class(class::TAB_FADE)
            //.attr("id", "nav-other")
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-other-tab")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("hct", "HCT", false),
                        input_number("hct", page.hct.clone(), page.changed.clone(), 1, Some("0"), Some("99"), Some("%"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("dtx", "DTX", false),
                        input_text("dtx", page.dtx.clone(), page.changed.clone(), Some(10), Some("mg/dl"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("bl", "Blood Lactate", false),
                        input_number("bl", page.bl.clone(), page.changed.clone(), 2, Some("0"), Some("99"), Some("mmol/L"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .attr("title", "Central Venous Pressure")
                    .children([
                        label_for("cvp", "CVP", false),
                        input_text("cvp", page.cvp.clone(), page.changed.clone(), Some(50), Some("mmHg"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("end_co2", "End Tidal CO\u{2082}", false),
                        input_number("end_co2", page.end_co2.clone(), page.changed.clone(), 0, Some("0"), Some("99"), Some("mmHg"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("head", "Head Circumference", false),
                        input_number("head", page.head.clone(), page.changed.clone(), 1, Some("0"), Some("99"), Some("cm."), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("t_inc", "Incubator Temp", false),
                        input_number("t_inc", page.t_inc.clone(), page.changed.clone(), 1, Some("0"), Some("99"), Some("\u{00b0}C"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("mcb", "Microbilirubin (MB)", false),
                        input_number("mcb", page.mcb.clone(), page.changed.clone(), 2, Some("0"), Some("999"), Some("mg/dl"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("peak-flow", "Peak Flow", false),
                        input_number("peak-flow", page.pleak_flow.clone(), page.changed.clone(), 2, Some("0"), Some("9999"), Some("L/min"), None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("suction", "Suction", false),
                        html!("div", {
                            .class(class::COL_SM5_B)
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    checkbox_y("suction", page.suction.clone(), page.changed.clone(), None),
                                    doms::label_check_for("suction", "ทำ"),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("nb", "NB", false),
                        html!("div", {
                            .class(class::COL_SM5_B)
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    checkbox_y("nb", page.nb.clone(), page.changed.clone(), None),
                                    doms::label_check_for("nb", "ทำ"),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("line_id", "Line", false),
                        select_options("line_id", page.line_id.clone(), page.changed.clone(), &line_select_option, "", None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("line_no", "Line No.", false),
                        input_number("line_no", page.line_no.clone(), page.changed.clone(), 1, Some("0"), Some("99"), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("line_mark", "Line Mark", false),
                        input_number("line_mark", page.line_mark.clone(), page.changed.clone(), 1, Some("0"), Some("99"), None, None),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .children([
                        label_for("other", "Other", false),
                        html!("div", {
                            .class(class::COL_SM6_P0S2)
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("id", "other")
                                .attr("rows", "3")
                                .apply(mixins::textarea_value_auto_expand(page.other.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }
}

fn label_for(id: &str, text: &str, with_scores: bool) -> Dom {
    let col = if with_scores { "col-sm-4" } else { "col-sm-5" };
    html!("label", {
        .class([col, "p-0", "pt-1", "text-end", "col-form-label"])
        .attr("for", id)
        .text(text)
    })
}

fn label_not_for(text: &str, with_scores: bool) -> Dom {
    let col = if with_scores { "col-sm-4" } else { "col-sm-5" };
    html!("label", {
        .class([col, "p-0", "pt-1", "text-end", "col-form-label"])
        .text(text)
    })
}

fn input_text(id: &str, text: Mutable<String>, changed: Mutable<bool>, max_len: Option<u32>, ending: Option<&str>, with_scores_item: Option<(Mutable<Option<Scores>>, &'static str)>) -> Dom {
    let has_ending = ending.is_some();
    html!("div", {
        .apply(|dom| if with_scores_item.is_some() {
            dom.class(class::COL_SM5_P0S2)
        } else {
            dom.class(class::COL_SM6_P0S2)
        })
        .child(html!("div", {
            .apply_if(has_ending, |dom| dom.class(class::INPUT_GROUP_SM))
            .child(html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class(class::FORM_CTRL_SM)
                .attr("id", id)
                .apply(|d| {
                    if let Some(max) = max_len {
                        d.attr("maxlength", &max.to_string())
                    } else {
                        d
                    }
                })
                .apply(mixins::string_value(text, changed))
                .apply(input_score_mixins(with_scores_item))
            }))
            .apply_if(has_ending, |dom| dom.child(
                doms::span_group_text(ending.unwrap_or_default())
            ))

        }))
    })
}

fn input_number(
    id: &str,
    text: Mutable<String>,
    changed: Mutable<bool>,
    decimal: u8,
    min: Option<&str>,
    max: Option<&str>,
    ending: Option<&str>,
    with_scores_item: Option<(Mutable<Option<Scores>>, &'static str)>,
) -> Dom {
    let has_ending = ending.is_some();
    let step = match decimal {
        1 => "0.1",
        2 => "0.01",
        3 => "0.001",
        _ => "1",
    };
    html!("div", {
        .apply(|dom| if with_scores_item.is_some() {
            dom.class(class::COL_SM5_P0S2)
        } else {
            dom.class(class::COL_SM6_P0S2)
        })
        .child(html!("div", {
            .apply_if(has_ending, |dom| dom.class(class::INPUT_GROUP_SM))
            .child(html!("input" => HtmlInputElement, {
                .attr("type", "number")
                .attr("step", step)
                .class(class::FORM_CTRL_SM)
                .attr("id", id)
                .attr("autocomplete","off")
                .apply(|d| {
                    if let Some(s) = min {
                        d.attr("min", s)
                    } else {
                        d
                    }
                })
                .apply(|d| {
                    if let Some(s) = max {
                        d.attr("max", s)
                    } else {
                        d
                    }
                })
                .apply(mixins::string_value(text, changed))
                .apply(input_score_mixins(with_scores_item))
            }))
            .apply_if(has_ending, |dom| dom.child(
                doms::span_group_text(ending.unwrap_or_default())
            ))
        }))
    })
}

fn checkbox_y(id: &str, text: Mutable<String>, changed: Mutable<bool>, with_scores_item: Option<(Mutable<Option<Scores>>, &'static str)>) -> Dom {
    html!("input" => HtmlInputElement, {
        .class("form-check-input")
        .attr("type", "checkbox")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(text, changed, "Y", ""))
        .with_node!(element => {
            .apply(|dom| {
                if let Some((scores, item)) = with_scores_item {
                    dom.event(move |_: events::Input| {
                        let mut lock = scores.lock_mut();
                        if let Some(scs) = lock.as_mut() {
                            let v = if element.checked() {"Y"} else {""};
                            scs.set_item(item, v);
                        }
                    })
                } else {
                    dom
                }
            })
        })
    })
}

fn select_options(id: &str, text: Mutable<String>, changed: Mutable<bool>, options: &[SelectOption], default_text: &str, with_scores_item: Option<(Mutable<Option<Scores>>, &'static str)>) -> Dom {
    // we use custom SelectOptions only ews scores
    // LqSofa's AVPU can use any EWS's AVPU option name
    // because abnormal LqSofa's AVPU always get 1 point
    let options = with_scores_item
        .as_ref()
        .and_then(|(scores_mutable, item)| scores_mutable.lock_ref().as_ref().and_then(|scores| scores.ews.custom_select_option(item)))
        .unwrap_or(options.to_vec());
    html!("div", {
        .apply(|dom| if with_scores_item.is_some() {
            dom.class(class::COL_SM5_P0S2)
        } else {
            dom.class(class::COL_SM6_P0S2)
        })
        .child(html!("select" => HtmlSelectElement, {
            .class(class::FORM_SELECT_SM)
            .attr("id", id)
            .child(html!("option", {.attr("value", "").text(default_text)}))
            .children(options.iter().map(|option| doms::select_option(option, "")))
            .apply(mixins::string_value_select(text, changed))
            .apply(input_score_mixins(with_scores_item))
        }))
    })
}

fn input_score_mixins<T>(with_scores_item: Option<(Mutable<Option<Scores>>, &'static str)>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: mixins::TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .apply(|d| {
                if let Some((scores, item)) = with_scores_item {
                    d.event(move |_: events::Input| {
                        let mut lock = scores.lock_mut();
                        if let Some(scs) = lock.as_mut() {
                            scs.set_item(item, &element.value());
                        }
                    })
                } else {
                    d
                }
            })
        })
    }
}

fn ews_badges(scores: Mutable<Option<Scores>>, item: &'static str) -> Dom {
    html!("div", {
        .class(class::COL_SM3_PX0)
        .child_signal(scores.signal_ref(|opt| opt.as_ref().map(|sc| {
            html!("div", {
                .apply(|dom| {
                    if sc.ews.contains(item) {
                        dom.child(ews_item(sc.ews.score_item(item), sc.ews.title_item(item), sc.ews.color_item(item), sc.ews.bg_color_item(item)))
                    } else {
                        dom.child(ews_space())
                    }
                })
                .apply(|dom| {
                    if sc.qsofa.contains(item) {
                        dom.child(ews_item(sc.qsofa.score_item(item), sc.qsofa.title_item(item), sc.qsofa.color_item(item), sc.qsofa.bg_color_item(item)))
                    } else {
                        dom.child(ews_space())
                    }
                })
                .apply(|dom| {
                    if sc.sirs.contains(item) {
                        dom.child(ews_item(sc.sirs.score_item(item), sc.sirs.title_item(item), sc.sirs.color_item(item), sc.sirs.bg_color_item(item)))
                    } else {
                        dom.child(ews_space())
                    }
                })
            })
        })))
    })
}

fn ews_item(score: Option<u32>, title: &str, color: &str, bg_color: &str) -> Dom {
    html!("div", {
        .style("font-size", "100%")
        .style("cursor", "help")
        .style("width", "28px")
        .attr("title", title)
        .apply(|dom| {
            if let Some(value) = score {
                dom.class(class::BADGE_RT_PX2).style("color", color).style("background-color", bg_color).style("cursor","default").text(&value.to_string())
            } else {
                dom.class(class::BADGE_RT_PX0_GRAY).child(html!("i", {.class(class::FA_X)}))
            }
        })
    })
}
fn ews_space() -> Dom {
    html!("div", {
        .class(class::BADGE_RT_PX2)
        .style("cursor","default")
        .style("width", "28px")
        .style("user-select", "none")
        .text("\u{2003}")
    })
}
fn ews_total(label: &str, score: Option<u32>, title: &str, color: &str, bg_color: &str) -> Dom {
    html!("div", {
        .class(class::COL_SM4_P0)
        .child(html!("div", {
            .class(class::ALERT_BOLD_C)
            .attr("title", title)
            .style("cursor", "help")
            .apply(|dom| {
                if let Some(value) = score {
                    dom.style("color", color).style("background-color", bg_color)
                    .children([
                        text(label), html!("br"), text(&["SCORE : ", &value.to_string()].concat())
                    ])
                } else {
                    dom.class("bg-secondary").text(label)
                }
            })
        }))
    })
}
