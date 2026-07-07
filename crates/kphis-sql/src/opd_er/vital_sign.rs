use kphis_model::vital_sign::VitalSignParams;

// SELECT vs.*,create_opduser.name AS create_opduser_name,update_opduser.name AS update_opduser_name,
//     ipd_vs_conscious.conscious_name,ipd_vs_line.line_name,
//     left_cha.cha_name AS left_cha_name,right_cha.cha_name AS right_cha_name,
//     ipd_vs_va.va_name,ipd_vs_mass.mass_name,ipd_vs_lr_sta.lr_sta_name,ipd_vs_lr_mem.lr_mem_name,
//     ipd_vs_lt_arm.lt_arm_name,ipd_vs_lt_leg.lt_leg_name,ipd_vs_rt_arm.rt_arm_name,ipd_vs_rt_leg.rt_leg_name,
//     ipd_vs_o2.o2_name,ipd_vs_tube.tube_name,ipd_vs_intake.intake_name,ipd_vs_output.output_name,
//     ipd_vs_breathing.breathing_name,ipd_vs_avpu.avpu_name,ipd_vs_gut_feeling.gut_feeling_name,ipd_vs_pops_other.pops_other_name
// FROM kphis.opd_er_vs_vital_sign vs
//     LEFT JOIN kphis.ipd_vs_conscious ON vs.conscious_id=ipd_vs_conscious.conscious_id
//     LEFT JOIN kphis.ipd_vs_line ON vs.line_id=ipd_vs_line.line_id
//     LEFT JOIN kphis.ipd_vs_cha left_cha ON vs.left_cha_id=left_cha.cha_id
//     LEFT JOIN kphis.ipd_vs_cha right_cha ON vs.right_cha_id=right_cha.cha_id
//     LEFT JOIN kphis.ipd_vs_va ON vs.va_id=ipd_vs_va.va_id
//     LEFT JOIN kphis.ipd_vs_mass ON vs.mass_id=ipd_vs_mass.mass_id
//     LEFT JOIN kphis.ipd_vs_o2 ON vs.o2_id=ipd_vs_o2.o2_id
//     LEFT JOIN kphis.ipd_vs_tube ON vs.tube_id=ipd_vs_tube.tube_id
//     LEFT JOIN kphis.ipd_vs_intake ON vs.intake_id=ipd_vs_intake.intake_id
//     LEFT JOIN kphis.ipd_vs_output ON vs.output_id=ipd_vs_output.output_id
//     LEFT JOIN kphis.ipd_vs_lr_sta ON vs.lr_sta=ipd_vs_lr_sta.lr_sta_id
//     LEFT JOIN kphis.ipd_vs_lr_mem ON vs.lr_mem=ipd_vs_lr_mem.lr_mem_id
//     LEFT JOIN kphis.ipd_vs_lt_arm ON vs.lt_arm=ipd_vs_lt_arm.lt_arm
//     LEFT JOIN kphis.ipd_vs_lt_leg ON vs.lt_leg=ipd_vs_lt_leg.lt_leg
//     LEFT JOIN kphis.ipd_vs_rt_arm ON vs.rt_arm=ipd_vs_rt_arm.rt_arm
//     LEFT JOIN kphis.ipd_vs_rt_leg ON vs.rt_leg=ipd_vs_rt_leg.rt_leg
//     LEFT JOIN kphis.ipd_vs_breathing ON vs.breathing_id=ipd_vs_breathing.breathing_id
//     LEFT JOIN kphis.ipd_vs_avpu ON vs.avpu_id=ipd_vs_avpu.avpu_id
//     LEFT JOIN kphis.ipd_vs_gut_feeling ON vs.gut_feeling_id=ipd_vs_gut_feeling.gut_feeling_id
//     LEFT JOIN kphis.ipd_vs_pops_other ON vs.pops_other_id=ipd_vs_pops_other.pops_other_id
//     LEFT JOIN hos.opduser create_opduser ON vs.create_user=create_opduser.loginname
//     LEFT JOIN hos.opduser update_opduser ON vs.update_user=update_opduser.loginname
pub fn select_chart_data(params: &VitalSignParams, hosxp: &str, kphis: &str) -> String {
    let vs_id = if params.vs_id.is_some() {" AND vs.vs_id=? "} else {""};
    let opd_er_order_master_id = if params.opd_er_order_master_id.is_some() {"  AND vs.opd_er_order_master_id=? "} else {""};
    let start_date = if params.start_date.is_some() {" AND vs.vs_datetime >= CONCAT(?,' 00:00:00.000') "} else {""};
    let end_date = if params.end_date.is_some() {" AND vs.vs_datetime <= CONCAT(?,' 23:59:59') "} else {""};
    [
        "SELECT vs.*,create_opduser.name AS create_opduser_name,update_opduser.name AS update_opduser_name,\
            ipd_vs_conscious.conscious_name,ipd_vs_line.line_name,\
            left_cha.cha_name AS left_cha_name,right_cha.cha_name AS right_cha_name,\
            ipd_vs_va.va_name,ipd_vs_mass.mass_name,ipd_vs_lr_sta.lr_sta_name,ipd_vs_lr_mem.lr_mem_name,\
            ipd_vs_lr_moulding.lr_moulding_name,dpp.dipstick_name AS urine_protein_name,dps.dipstick_name AS urine_sugar_name,\
            ipd_vs_lt_arm.lt_arm_name,ipd_vs_lt_leg.lt_leg_name,ipd_vs_rt_arm.rt_arm_name,ipd_vs_rt_leg.rt_leg_name,\
            ipd_vs_o2.o2_name,ipd_vs_tube.tube_name,ipd_vs_intake.intake_name,ipd_vs_output.output_name,soc.stage_of_change_name \
        FROM ",kphis,".opd_er_vs_vital_sign vs \
            LEFT JOIN ",kphis,".ipd_vs_conscious ON vs.conscious_id=ipd_vs_conscious.conscious_id \
            LEFT JOIN ",kphis,".ipd_vs_line ON vs.line_id=ipd_vs_line.line_id \
            LEFT JOIN ",kphis,".ipd_vs_cha left_cha ON vs.left_cha_id=left_cha.cha_id \
            LEFT JOIN ",kphis,".ipd_vs_cha right_cha ON vs.right_cha_id=right_cha.cha_id \
            LEFT JOIN ",kphis,".ipd_vs_va ON vs.va_id=ipd_vs_va.va_id \
            LEFT JOIN ",kphis,".ipd_vs_mass ON vs.mass_id=ipd_vs_mass.mass_id \
            LEFT JOIN ",kphis,".ipd_vs_o2 ON vs.o2_id=ipd_vs_o2.o2_id \
            LEFT JOIN ",kphis,".ipd_vs_tube ON vs.tube_id=ipd_vs_tube.tube_id \
            LEFT JOIN ",kphis,".ipd_vs_intake ON vs.intake_id=ipd_vs_intake.intake_id \
            LEFT JOIN ",kphis,".ipd_vs_output ON vs.output_id=ipd_vs_output.output_id \
            LEFT JOIN ",kphis,".ipd_vs_lr_sta ON vs.lr_sta=ipd_vs_lr_sta.lr_sta_id \
            LEFT JOIN ",kphis,".ipd_vs_lr_mem ON vs.lr_mem=ipd_vs_lr_mem.lr_mem_id \
            LEFT JOIN ",kphis,".ipd_vs_lr_moulding ON vs.lr_moulding=ipd_vs_lr_moulding.lr_moulding_id \
            LEFT JOIN ",kphis,".ipd_vs_dipstick dpp ON vs.urine_protein=dpp.dipstick_id \
            LEFT JOIN ",kphis,".ipd_vs_dipstick dps ON vs.urine_sugar=dps.dipstick_id \
            LEFT JOIN ",kphis,".ipd_vs_lt_arm ON vs.lt_arm=ipd_vs_lt_arm.lt_arm \
            LEFT JOIN ",kphis,".ipd_vs_lt_leg ON vs.lt_leg=ipd_vs_lt_leg.lt_leg \
            LEFT JOIN ",kphis,".ipd_vs_rt_arm ON vs.rt_arm=ipd_vs_rt_arm.rt_arm \
            LEFT JOIN ",kphis,".ipd_vs_rt_leg ON vs.rt_leg=ipd_vs_rt_leg.rt_leg \
            LEFT JOIN ",kphis,".ipd_vs_stage_of_change soc ON vs.stage_of_change_id=soc.stage_of_change_id \
            LEFT JOIN ",hosxp,".opduser create_opduser ON vs.create_user=create_opduser.loginname \
            LEFT JOIN ",hosxp,".opduser update_opduser ON vs.update_user=update_opduser.loginname \
        WHERE 1=1 ",vs_id,opd_er_order_master_id,start_date,end_date," ORDER BY vs.vs_datetime DESC;"
    ].concat()
}

/// opd_er_order_master_id
pub fn select_vs_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_vs_vital_sign WHERE opd_er_order_master_id=? ORDER BY vs_datetime DESC;"
    ].concat()
}

// DELETE FROM kphis.opd_er_vs_vital_sign WHERE vs_id=?;
/// vs_id
pub fn delete_vital_sign(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_vs_vital_sign WHERE vs_id=?;"
    ].concat()
}
