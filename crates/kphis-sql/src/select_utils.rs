// SelectUtils::getAllDoctorSelectOption
// SELECT code AS `key`, `name` AS `value` FROM hos.doctor WHERE active = 'Y' ORDER BY name;
pub fn get_all_doctor_select_option(hosxp: &str) -> String {
    ["SELECT code AS `key`, `name` AS `value` FROM ",hosxp,".doctor WHERE active = 'Y' ORDER BY name;"].concat()
}

// SelectUtils::getDoctorSelectOption
// SELECT code AS `key`, `name` AS `value` FROM hos.doctor
// WHERE active = 'Y' AND provider_type_code IN ('01', '02') AND licenseno IS NOT NULL AND trim(licenseno) <> '' AND licenseno <> '-99999' ORDER BY NAME;
pub fn get_doctor_select_option(hosxp: &str) -> String {
    [
        "SELECT code AS `key`, `name` AS `value` FROM ",hosxp,".doctor \
        WHERE active = 'Y' AND provider_type_code IN ('01', '02') AND licenseno IS NOT NULL AND trim(licenseno) <> '' AND licenseno <> '-99999' ORDER BY NAME;"
    ].concat()
}

// SelectUtils::getErBedSelectOption
// SELECT b.opd_er_bed_id AS `key`, CONCAT(t.bed_type_name,' ',b.bedno) AS `value`, t.bed_type_color AS color
// FROM kphis.opd_er_bed b LEFT JOIN kphis.opd_er_bed_type t ON b.bed_type = t.bed_type
// WHERE b.active <> 'N' ORDER BY t.display_order, b.display_order, b.bedno
pub fn get_er_bed_select_option(kphis: &str) -> String {
    [
        "SELECT b.opd_er_bed_id AS `key`, CONCAT(t.bed_type_name,' ',b.bedno) AS `value`, t.bed_type_color AS color \
        FROM ",kphis,".opd_er_bed b LEFT JOIN ",kphis,".opd_er_bed_type t ON b.bed_type = t.bed_type \
        WHERE b.active <> 'N' ORDER BY t.display_order, b.display_order, b.bedno;"
    ].concat()
}

// SelectUtils::getErPatientStatusSelectOption
// SELECT er_patient_status_id AS `key`, er_patient_status_name AS `value` FROM kphis.opd_er_patient_status ORDER BY display_order;
pub fn get_er_patient_status_select_option(kphis: &str) -> String {
    ["SELECT er_patient_status_id AS `key`, er_patient_status_name AS `value` FROM ",kphis,".opd_er_patient_status ORDER BY display_order;"].concat()
}

// SelectUtils::getErDchTypeSelectOption
// SELECT er_dch_type_id AS `key`, er_dch_type_name AS `value` FROM kphis.opd_er_dch_type ORDER BY display_order;
pub fn get_er_dch_type_select_option(kphis: &str) -> String {
    ["SELECT er_dch_type_id AS `key`, er_dch_type_name AS `value` FROM ",kphis,".opd_er_dch_type ORDER BY display_order;"].concat()
}

// SelectUtils::getWardSelectOption
// SELECT ward AS `key`, `name` AS `value` FROM hos.ward ORDER BY name;
pub fn get_ward_select_option(hosxp: &str) -> String {
    ["SELECT ward AS `key`, `name` AS `value` FROM ",hosxp,".ward ORDER BY name;"].concat()
}

// SelectUtils::getKphisSpcltySelectOption
// SELECT spclty_id AS `key`, spclty_name AS `value` FROM kphis.kphis_spclty ORDER BY spclty_order;
pub fn get_kphis_spclty_select_option(kphis: &str) -> String {
    ["SELECT spclty_id AS `key`, spclty_name AS `value` FROM ",kphis,".kphis_spclty ORDER BY spclty_order;"].concat()
}

// SelectUtils::getSpcltySelectOption
// SELECT spclty AS `key`, `name` AS `value` FROM hos.spclty ORDER BY name;"
pub fn get_spclty_select_option(hosxp: &str) -> String {
    ["SELECT spclty AS `key`, `name` AS `value` FROM ",hosxp,".spclty ORDER BY name;"].concat()
}

// SELECT inscl_code AS `key`, inscl_name AS `value` FROM hos.nhso_inscl_code ORDER BY inscl_name;
pub fn get_inscl_select_option(hosxp: &str) -> String {
    ["SELECT inscl_code AS `key`, inscl_name AS `value` FROM ",hosxp,".nhso_inscl_code ORDER BY inscl_name;"].concat()
}

// SelectUtil::getEmergencySelectOption
// SELECT emergency_id AS `key`, emergency_name AS `value` FROM kphis.ipd_emergency ORDER BY emergency_id;
pub fn get_emergency_select_option(kphis: &str) -> String {
    ["SELECT emergency_id AS `key`, emergency_name AS `value` FROM ",kphis,".ipd_emergency ORDER BY emergency_id;"].concat()
}

// SelectUtil::getEmergencyLevelSelectOption
// SELECT er_emergency_level_id AS `key`, er_emergency_level_name AS `value` FROM hos.er_emergency_level ORDER BY er_emergency_level_id;
pub fn get_emergency_level_select_option(hosxp: &str) -> String {
    ["SELECT er_emergency_level_id AS `key`, er_emergency_level_name AS `value` FROM ",hosxp,".er_emergency_level ORDER BY er_emergency_level_id;"].concat()
}

// SelectUtil::getConsultTypeSelectOption
// SELECT consult_type_id AS `key`, `consult_type_name` AS `value` FROM kphis.ipd_dr_consult_type ORDER BY consult_type_order;
pub fn get_consult_type_select_option(kphis: &str) -> String {
    ["SELECT consult_type_id AS `key`, `consult_type_name` AS `value` FROM ",kphis,".ipd_dr_consult_type ORDER BY consult_type_order;"].concat()
}

// SelectUtil::getConsciousSelectOption u32
// SELECT conscious_id AS `key`, `conscious_name` AS `value` FROM kphis.ipd_vs_conscious ORDER BY conscious_id;
pub fn get_conscious_select_option(kphis: &str) -> String {
    ["SELECT conscious_id AS `key`, `conscious_name` AS `value` FROM ",kphis,".ipd_vs_conscious ORDER BY conscious_id;"].concat()
}

// SelectUtil::getUrinAmountSelectOption u32
// SELECT urine_amount_id AS `key`, `urine_amount_name` AS `value` FROM kphis.ipd_vs_urine_amount ORDER BY urine_amount_id;
pub fn get_urine_amount_select_option(kphis: &str) -> String {
    ["SELECT urine_amount_id AS `key`, `urine_amount_name` AS `value` FROM ",kphis,".ipd_vs_urine_amount ORDER BY urine_amount_id;"].concat()
}

// SelectUtil::getUrinDurationSelectOption u32
// SELECT urine_d_id AS `key`, `urine_d_name` AS `value` FROM kphis.ipd_vs_urine_duration ORDER BY urine_d_id;
pub fn get_urine_duration_select_option(kphis: &str) -> String {
    ["SELECT urine_d_id AS `key`, `urine_d_name` AS `value` FROM ",kphis,".ipd_vs_urine_duration ORDER BY urine_d_id;"].concat()
}

// SelectUtil::getLineSelectOption u32
// SELECT line_id AS `key`, `line_name` AS `value` FROM kphis.ipd_vs_line ORDER BY line_id;
pub fn get_line_select_option(kphis: &str) -> String {
    ["SELECT line_id AS `key`, `line_name` AS `value` FROM ",kphis,".ipd_vs_line ORDER BY line_id;"].concat()
}

// SelectUtil::getChaSelectOption u32
// SELECT cha_id AS `key`, `cha_name` AS `value` FROM kphis.ipd_vs_cha ORDER BY cha_id;
pub fn get_cha_select_option(kphis: &str) -> String {
    ["SELECT cha_id AS `key`, `cha_name` AS `value` FROM ",kphis,".ipd_vs_cha ORDER BY cha_id;"].concat()
}

// SelectUtil::getVaSelectOption u32
// SELECT va_id AS `key`, `va_name` AS `value` FROM kphis.ipd_vs_va ORDER BY va_id;
pub fn get_va_select_option(kphis: &str) -> String {
    ["SELECT va_id AS `key`, `va_name` AS `value` FROM ",kphis,".ipd_vs_va ORDER BY va_id;"].concat()
}

// SelectUtil::getMassSelectOption u32
// SELECT mass_id AS `key`, `mass_name` AS `value` FROM kphis.ipd_vs_mass ORDER BY mass_id;
pub fn get_mass_select_option(kphis: &str) -> String {
    ["SELECT mass_id AS `key`, `mass_name` AS `value` FROM ",kphis,".ipd_vs_mass ORDER BY mass_id;"].concat()
}

// SelectUtil::getKphisLtArmSelectOption
// SELECT lt_arm AS `key`, lt_arm_name AS `value` FROM kphis.ipd_vs_lt_arm ORDER BY lt_arm;
pub fn get_lt_arm_select_option(kphis: &str) -> String {
    ["SELECT lt_arm AS `key`, lt_arm_name AS `value` FROM ",kphis,".ipd_vs_lt_arm ORDER BY lt_arm;"].concat()
}

// SelectUtil::getO2SelectOption u32
// SELECT o2_id AS `key`, `o2_name` AS `value` FROM kphis.ipd_vs_o2 ORDER BY o2_id;
pub fn get_o2_select_option(kphis: &str) -> String {
    ["SELECT o2_id AS `key`, `o2_name` AS `value` FROM ",kphis,".ipd_vs_o2 ORDER BY o2_id;"].concat()
}

// SelectUtil::getTubeSelectOption u32
// SELECT tube_id AS `key`, `tube_name` AS `value` FROM kphis.ipd_vs_tube ORDER BY tube_id;
pub fn get_tube_select_option(kphis: &str) -> String {
    ["SELECT tube_id AS `key`, `tube_name` AS `value` FROM ",kphis,".ipd_vs_tube ORDER BY tube_id;"].concat()
}

// SelectUtils::getIntakeSelectOption
// SELECT intake_id AS `key`, `intake_name` AS `value` FROM kphis.ipd_vs_intake ORDER BY intake_id;
pub fn get_intake_select_option(kphis: &str) -> String {
    ["SELECT intake_id AS `key`, `intake_name` AS `value` FROM ",kphis,".ipd_vs_intake ORDER BY intake_id;"].concat()
}

// SelectUtils::getOutputSelectOption
// SELECT output_id AS `key`, `output_name` AS `value` FROM kphis.ipd_vs_output ORDER BY output_id;
pub fn get_output_select_option(kphis: &str) -> String {
    ["SELECT output_id AS `key`, `output_name` AS `value` FROM ",kphis,".ipd_vs_output ORDER BY output_id;"].concat()
}

// SelectUtil::getLRstaSelectOption u32
// SELECT lr_sta_id AS `key`, `lr_sta_name` AS `value` FROM kphis.ipd_vs_lr_sta ORDER BY lr_sta_id;
pub fn get_lr_sta_select_option(kphis: &str) -> String {
    ["SELECT lr_sta_id AS `key`, `lr_sta_name` AS `value` FROM ",kphis,".ipd_vs_lr_sta ORDER BY lr_sta_id;"].concat()
}

// SelectUtil::getLRmemSelectOption u32
// SELECT lr_mem_id AS `key`, `lr_mem_name` AS `value` FROM kphis.ipd_vs_lr_mem ORDER BY lr_mem_id;
pub fn get_lr_mem_select_option(kphis: &str) -> String {
    ["SELECT lr_mem_id AS `key`, `lr_mem_name` AS `value` FROM ",kphis,".ipd_vs_lr_mem ORDER BY lr_mem_id;"].concat()
}

// SELECT lr_moulding_id AS `key`, `lr_moulding_name` AS `value` FROM kphis.ipd_vs_lr_moulding ORDER BY lr_moulding_id;
pub fn get_lr_moulding_select_option(kphis: &str) -> String {
    ["SELECT lr_moulding_id AS `key`, `lr_moulding_name` AS `value` FROM ",kphis,".ipd_vs_lr_moulding ORDER BY lr_moulding_id;"].concat()
}

// SELECT dipstick_id AS `key`, `dipstick_name` AS `value` FROM kphis.ipd_vs_dipstick ORDER BY dipstick_id;
pub fn get_dipstick_select_option(kphis: &str) -> String {
    ["SELECT dipstick_id AS `key`, `dipstick_name` AS `value` FROM ",kphis,".ipd_vs_dipstick ORDER BY dipstick_id;"].concat()
}

// SelectUtil::getKphisBreathingSelectOption u32
// SELECT breathing_id AS `key`, breathing_name AS `value` FROM kphis.ipd_vs_breathing ORDER BY breathing_id;
pub fn get_breathing_select_option(kphis: &str) -> String {
    ["SELECT breathing_id AS `key`, breathing_name AS `value` FROM ",kphis,".ipd_vs_breathing ORDER BY breathing_id;"].concat()
}

// SelectUtil::getKphisAVPUSelectOption u32
// SELECT avpu_id AS `key`, avpu_name AS `value` FROM kphis.ipd_vs_avpu ORDER BY avpu_id;
pub fn get_avpu_select_option(kphis: &str) -> String {
    ["SELECT avpu_id AS `key`, avpu_name AS `value` FROM ",kphis,".ipd_vs_avpu ORDER BY avpu_id;"].concat()
}

// SelectUtil::getKphisGUSFeelingSelectOption u32
// SELECT gut_feeling_id AS `key`, gut_feeling_name AS `value` FROM kphis.ipd_vs_gut_feeling ORDER BY gut_feeling_id;
pub fn get_gut_feeling_select_option(kphis: &str) -> String {
    ["SELECT gut_feeling_id AS `key`, gut_feeling_name AS `value` FROM ",kphis,".ipd_vs_gut_feeling ORDER BY gut_feeling_id;"].concat()
}

// SelectUtil::getKphisPOPSOtherSelectOption u32
// SELECT pops_other_id AS `key`, pops_other_name AS `value` FROM kphis.ipd_vs_pops_other ORDER BY pops_other_id;
pub fn get_pops_other_select_option(kphis: &str) -> String {
    ["SELECT pops_other_id AS `key`, pops_other_name AS `value` FROM ",kphis,".ipd_vs_pops_other ORDER BY pops_other_id;"].concat()
}

// SELECT stage_of_change_id AS `key`, stage_of_change_name AS `value` FROM kphis.ipd_vs_stage_of_change ORDER BY stage_of_change_id;
pub fn get_stage_of_change_select_option(kphis: &str) -> String {
    ["SELECT stage_of_change_id AS `key`, stage_of_change_name AS `value` FROM ",kphis,".ipd_vs_stage_of_change ORDER BY stage_of_change_id;"].concat()
}

// SELECT refer_type AS `key`, refer_type_name AS `value` FROM hos.refer_type ORDER BY refer_type;
pub fn get_refer_type_select_option(hosxp: &str) -> String {
    ["SELECT refer_type AS `key`, refer_type_name AS `value` FROM ",hosxp,".refer_type ORDER BY refer_type;"].concat()
}

// SELECT id AS `key`, `name` AS `value` FROM hos.refer_cause ORDER BY id;
pub fn get_refer_cause_select_option(hosxp: &str) -> String {
    ["SELECT id AS `key`, `name` AS `value` FROM ",hosxp,".refer_cause ORDER BY id;"].concat()
}

// SELECT `name` AS `key`, `name` AS `value` FROM hos.refer_point_list ORDER BY `name`;
pub fn get_refer_point_select_option(hosxp: &str) -> String {
    ["SELECT `name` AS `key`, `name` AS `value` FROM ",hosxp,".refer_point_list ORDER BY `name`;"].concat()
}

// SELECT moph_refer_expire_type_id AS `key`, moph_refer_expire_type_name AS `value` FROM hos.moph_refer_expire_type ORDER BY moph_refer_expire_type_id;
pub fn get_moph_refer_expire_type_select_option(hosxp: &str) -> String {
    ["SELECT moph_refer_expire_type_id AS `key`, moph_refer_expire_type_name AS `value` FROM ",hosxp,".moph_refer_expire_type ORDER BY moph_refer_expire_type_id;"].concat()
}