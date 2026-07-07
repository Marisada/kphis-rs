use std::rc::Rc;
use time::Date;

use kphis_model::{
    score::{ScoreDispatch, Scores},
    vital_sign::VitalSign,
};
use kphis_ui_app::App;
use kphis_util::util::decimal_rescale;

// no urines, feces, bw, height, head-circum
/// with_datetime will set `new line` + `- [DateTime] ` before contents
pub fn full_text(row: Rc<VitalSign>, birth_day: Option<Date>, app: Rc<App>) -> (String, Option<Scores>) {
    let scores_opt = Scores::from_vs(&row, birth_day, app.state());

    let mut values = Vec::with_capacity(91);
    // V/S
    if let Some(bt) = row.bt {
        values.push(["T: ", &bt.to_string(), " °C"].concat());
    }
    if let Some(pr) = row.pr {
        values.push(["P: ", &pr.to_string(), " /min"].concat());
    }
    if let Some(rr) = row.rr {
        values.push(["R: ", &rr.to_string(), " /min"].concat());
    }
    if let (Some(sbp), Some(dbp)) = (row.sbp, row.dbp) {
        values.push(["BP: ", &sbp.to_string(), "/", &dbp.to_string(), " mmHg"].concat());
    }
    if let Some(map) = row.map {
        values.push(["MAP: ", &map.to_string(), " mmHg"].concat());
    }
    if let Some(sat_room_air) = row.sat_room_air {
        values.push(["O\u{2082}sat RA: ", &sat_room_air.to_string(), " %"].concat());
    }
    if let Some(sat) = row.sat {
        values.push(["O\u{2082}sat: ", &sat.to_string(), " %"].concat());
    }
    if let Some(pain) = row.pain {
        values.push(["Pain: ", &pain.to_string(), "/10"].concat());
    }
    if let Some(dtx) = row.dtx.as_ref() {
        values.push(["Dtx: ", dtx, " mg%"].concat());
    }
    if let Some(hct) = row.hct {
        values.push(["Hct: ", &hct.to_string(), " %"].concat());
    }
    if let Some(crt) = row.crt {
        values.push(["CRT: ", &crt.to_string(), " sec"].concat());
    }
    if let Some(peak_flow) = row.pleak_flow {
        values.push(["Peak Flow: ", &peak_flow.to_string(), " L/min"].concat());
    }

    // Neuro
    if let Some(conscious_name) = row.conscious_name.as_ref() {
        values.push(conscious_name.to_owned());
    }
    if let (Some(eye), Some(verbal), Some(movement)) = (row.eye, row.verbal.as_ref(), row.movement) {
        values.push(["GCS: E", &eye.to_string(), "V", verbal, "M", &movement.to_string()].concat());
    }
    if let (Some(right_pupil), Some(left_pupil), Some(right_cha_id), Some(left_cha_id)) = (row.right_pupil, row.left_pupil, row.right_cha_id, row.left_cha_id) {
        if (right_pupil == left_pupil) && (right_cha_id == left_cha_id) {
            values.push(["Pupils: ", &right_pupil.to_string(), " mm.", cha_id_to_short(right_cha_id), " BE"].concat());
        } else {
            values.push(
                [
                    "Pupils: Rt ",
                    &right_pupil.to_string(),
                    " mm.",
                    cha_id_to_short(right_cha_id),
                    " Lt ",
                    &left_pupil.to_string(),
                    " mm.",
                    cha_id_to_short(left_cha_id),
                ]
                .concat(),
            );
        }
    } else if let (Some(right_pupil), Some(right_cha_id)) = (row.right_pupil, row.right_cha_id) {
        values.push(["Pupil: Rt ", &right_pupil.to_string(), " mm.", cha_id_to_short(right_cha_id)].concat());
    } else if let (Some(left_pupil), Some(left_cha_id)) = (row.left_pupil, row.left_cha_id) {
        values.push(["Pupil: Lt ", &left_pupil.to_string(), " mm.", cha_id_to_short(left_cha_id)].concat());
    }
    if let (Some(rt_arm_name), Some(lt_arm_name), Some(lt_leg_name), Some(rt_leg_name)) = (row.rt_arm_name.as_ref(), row.lt_arm_name.as_ref(), row.lt_leg_name.as_ref(), row.rt_leg_name.as_ref()) {
        if (rt_arm_name == lt_arm_name) && (rt_leg_name == lt_leg_name) && (rt_arm_name == rt_leg_name) {
            values.push(["Motor power: gr.", rt_arm_name, " all"].concat());
        } else {
            values.push(["Motor power: Rt arm ", rt_arm_name, ", Lt arm ", lt_arm_name, ", Rt leg ", rt_leg_name, ", Lt leg ", lt_leg_name].concat());
        }
    } else if row.rt_arm_name.is_some() || row.lt_arm_name.is_some() || row.rt_leg_name.is_some() || row.lt_leg_name.is_some() {
        let mut motors = Vec::with_capacity(5);
        motors.push("Motor power:".to_owned());
        if let Some(rt_arm_name) = row.rt_arm_name.as_ref() {
            motors.push(["Rt arm ", rt_arm_name].concat());
        }
        if let Some(lt_arm_name) = row.lt_arm_name.as_ref() {
            motors.push(["Lt arm ", lt_arm_name].concat());
        }
        if let Some(rt_leg_name) = row.rt_leg_name.as_ref() {
            motors.push(["Rt leg ", rt_leg_name].concat());
        }
        if let Some(lt_leg_name) = row.lt_leg_name.as_ref() {
            motors.push(["Lt leg ", lt_leg_name].concat());
        }
        values.push(motors.join(" "))
    }

    // Scores
    if let Some(scores) = &scores_opt {
        if let Some(ews) = scores.ews.score() {
            values.push([scores.ews.label(), ": ", &ews.to_string()].concat());
        }
        if let Some(qsofa) = scores.qsofa.score() {
            values.push([scores.qsofa.label(), ": ", &qsofa.to_string()].concat());
        }
        if let Some(sirs) = scores.sirs.score() {
            values.push([scores.sirs.label(), ": ", &sirs.to_string()].concat());
        }
    }
    if let Some(braden) = row.braden.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["Braden: ", braden].concat());
    }
    if let Some(va_name) = row.va_name.as_ref() {
        // TODO: What is VA scale with value A, B, C, D
        values.push(["VA: ", va_name].concat());
    }
    if let Some(mass_name) = row.mass_name.as_ref() {
        values.push(["MAAS: ", mass_name].concat());
    }
    if let Some(severity) = row.severity {
        values.push(["Severity: ", &severity.to_string()].concat());
    }
    if let Some(barthel) = row.barthel_index.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["Barthel Index: ", barthel].concat());
    }

    if let Some(concat) = row.amphetamine_awq.as_ref() {
        let mut iter = concat.split(',');
        let awq = iter.next();
        let awq_h = iter.next();
        let awq_a = iter.next();
        let awq_r = iter.next();
        values.push(["AWQv2: ", awq.unwrap_or_default()].concat());
        if let (Some(h), Some(a), Some(r)) = (awq_h, awq_a, awq_r) {
            values.push([" (H:", h, ",A:", a, ",R:", r, ")"].concat());
        }
    }
    if let Some(depress_2q) = row.depress_2q.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["2Q: ", depress_2q].concat());
    }
    if let Some(depress_9q) = row.depress_9q.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["9Q: ", depress_9q].concat());
    }
    if let Some(suicide_8q) = row.suicide_8q.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["8Q: ", suicide_8q].concat());
    }
    if let Some(motivation_scale) = row.motivation_scale {
        values.push(["Motivation scale: ", &motivation_scale.to_string()].concat());
    }
    if let Some(craving_scale) = row.craving_scale {
        values.push(["Craving scale: ", &craving_scale.to_string()].concat());
    }
    if let Some(stage_of_change_name) = &row.stage_of_change_name {
        values.push(["Stage of change: ", &stage_of_change_name].concat());
    }
    if let Some(oas) = row.aggression_oas.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["OAS: ", oas].concat());
    }

    if let Some(ciwa) = row.alcohol_ciwa.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["CIWA-Ar: ", ciwa].concat());
    }
    if let Some(aws) = row.alcohol_aws.as_ref().and_then(|concat| concat.split(',').nth(0)) {
        values.push(["AWS: ", aws].concat());
    }

    // O2 + Airway
    if let Some(o2_name) = row.o2_name.as_ref() {
        let mut o2s = Vec::with_capacity(3);
        o2s.push(["O\u{2082}: ", o2_name].concat());
        if let Some(o2_flow) = row.o2_flow {
            o2s.push([&decimal_rescale(o2_flow, 0).to_string(), " L/min"].concat());
        }
        if let Some(fio2) = row.fio2 {
            o2s.push(["FiO\u{2082} ", &decimal_rescale(fio2, 1).to_string()].concat());
        }
        values.push(o2s.join(" "))
    }

    if let Some(tube_name) = row.tube_name.as_ref() {
        let mut tubes = Vec::with_capacity(3);
        tubes.push(["On ", tube_name].concat());
        if let Some(tube_no) = row.tube_no {
            tubes.push(["No.", &tube_no.to_string()].concat());
        }
        if let Some(tube_mark) = row.tube_mark {
            tubes.push(["depth ", &tube_mark.to_string()].concat());
        }
        values.push(tubes.join(" "))
    }

    // Respirator
    if let Some(ventilator_name) = row.ventilator_name.as_ref() {
        let mut ventilators = Vec::with_capacity(13);
        ventilators.push(["Ventilator: ", ventilator_name].concat());
        if let Some(mode) = row.mode.as_ref() {
            ventilators.push(["on ", mode, " mode"].concat());
        }
        if let Some(tv) = row.tv {
            ventilators.push(["TV ", &tv.to_string(), " mL"].concat());
        }
        if let Some(pip) = row.pip {
            ventilators.push(["PIP ", &pip.to_string(), " cmH\u{2082}O"].concat());
        }
        if let Some(r_rate) = row.r_rate {
            ventilators.push(["RR ", &r_rate.to_string(), " /min"].concat());
        }
        if let (Some(i_rate), Some(e_rate)) = (row.i_rate, row.e_rate) {
            ventilators.push(["I:E ", &i_rate.to_string(), ":", &e_rate.to_string()].concat());
        }
        if let Some(ti) = row.ti {
            ventilators.push(["Ti ", &ti.to_string(), " secs"].concat());
        }
        if let Some(ps) = row.ps {
            ventilators.push(["PS ", &ps.to_string(), " cmH\u{2082}O"].concat());
        }
        if let Some(fio2) = row.fio2 {
            ventilators.push(["FiO\u{2082} ", &fio2.to_string()].concat());
        }
        if let Some(peep) = row.peep {
            ventilators.push(["PEEP ", &peep.to_string(), " cmH\u{2082}O"].concat());
        }
        if let Some(ft) = row.ft {
            ventilators.push(["FT ", &ft.to_string(), " L/min"].concat());
        }
        if let Some(delta_p) = row.delta_p {
            ventilators.push(["∆P ", &delta_p.to_string(), " cmH\u{2082}O"].concat());
        }
        if let Some(o2_map) = row.o2_map {
            ventilators.push(["MAP ", &o2_map.to_string(), " cmH\u{2082}O"].concat());
        }
        values.push(ventilators.join(" "))
    }

    // ICU
    if let Some(line_name) = row.line_name.as_ref() {
        values.push(["Line: ", line_name].concat());
    }
    if let Some(line_no) = row.line_no {
        values.push(["Line No.: ", &line_no.to_string()].concat());
    }
    if let Some(line_mark) = row.line_mark {
        values.push(["Line Mark: ", &line_mark.to_string()].concat());
    }
    if let Some(cvp) = row.cvp.as_ref() {
        values.push(["CVP: ", cvp, " mmHg"].concat());
    }
    if let Some(end_co2) = row.end_co2 {
        values.push(["EtCO\u{2082}: ", &end_co2.to_string(), " mmHg"].concat());
    }
    if let Some(t_inc) = row.t_inc {
        values.push(["Incubator T: ", &t_inc.to_string(), " °C"].concat());
    }

    // LR
    if let Some(lr_int) = row.lr_int.as_ref() {
        values.push(["I ", lr_int].concat());
    }
    if let Some(lr_dur) = row.lr_dur {
        values.push(["D ", &lr_dur.to_string(), "\""].concat());
    }
    if let Some(lr_fsh) = row.lr_fsh {
        values.push(["FHS ", &lr_fsh.to_string(), " /min"].concat());
    }
    if let Some(lr_sev) = row.lr_sev.as_ref() {
        values.push(["S ", lr_sev].concat());
    }
    if let Some(lr_cer) = row.lr_cer.as_ref() {
        values.push(["Cx ", lr_cer, " cm."].concat());
    }
    if let Some(lr_eff) = row.lr_eff {
        values.push(["Eff ", &lr_eff.to_string(), " %"].concat());
    }
    if let Some(lr_sta_name) = row.lr_sta_name.as_ref() {
        values.push(["St ", lr_sta_name].concat());
    }
    if let Some(lr_mem_name) = row.lr_mem_name.as_ref() {
        values.push(["M ", lr_mem_name].concat());
    }
    if let Some(lr_af) = row.lr_af.as_ref() {
        values.push(["AF ", lr_af].concat());
    }
    if let Some(lr_pos) = row.lr_pos.as_ref() {
        values.push(lr_pos.to_owned());
    }
    if let Some(lr_moulding_name) = row.lr_moulding_name.as_ref() {
        values.push(["Moulding ", lr_moulding_name].concat());
    }
    if let Some(lr_oxytocin_unit) = row.lr_oxytocin_unit {
        values.push(
            [
                "Oxytocin ",
                &lr_oxytocin_unit.to_string(),
                " U/L",
                if row.lr_oxytocin_rate.is_some() { " rate " } else { "" },
                &row.lr_oxytocin_rate.map(|u| u.to_string()).unwrap_or_default(),
                if row.lr_oxytocin_rate.is_some() { " drops/min" } else { "" },
            ]
            .concat(),
        );
    }
    if let Some(urine_protein_name) = row.urine_protein_name.as_ref() {
        values.push(["Protein ", urine_protein_name].concat());
    }
    if let Some(urine_sugar_name) = row.urine_sugar_name.as_ref() {
        values.push(["Sugar ", urine_sugar_name].concat());
    }
    if let Some(lr_urine_vol) = row.lr_urine_vol {
        values.push(["Void ", &lr_urine_vol.to_string(), " mL"].concat());
    }

    // Other
    if let Some(had_name) = row.had_name.as_ref()
        && !had_name.trim().is_empty()
    {
        values.push(["HAD: ", had_name, if row.had_drop.is_some() { " rate " } else { "" }, &row.had_drop.clone().unwrap_or_default()].concat());
    }
    if let Some(bl) = row.bl {
        values.push(["Lactate: ", &bl.to_string()].concat());
    }

    if let Some(mcb) = row.mcb {
        values.push(["MCB: ", &mcb.to_string()].concat());
    }
    if row.suction == Some(String::from("Y")) {
        values.push("Suction: done".to_owned());
    }
    if row.nb == Some(String::from("Y")) {
        values.push("NB: done".to_owned());
    }
    if let Some(other) = row.other.as_ref() {
        values.push(["Other: ", other].concat());
    }

    (values.join(", "), scores_opt)
}

pub fn cha_id_to_short(cha_id: u32) -> &'static str {
    match cha_id {
        1 => "RTL",
        2 => "sRTL",
        3 => "Fixed",
        4 => "Dilated",
        _ => "",
    }
}
