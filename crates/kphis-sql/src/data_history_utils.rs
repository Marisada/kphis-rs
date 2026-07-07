const CREATE_UPDATE_VERSION: &str = "\
    'create_user',create_user,\
    'create_datetime',create_datetime,\
    'update_user',update_user,\
    'update_datetime',update_datetime,\
    'version',version
";

/// if value is String, value MUST BE single-quoted like 'something'
pub struct KeyValue(pub &'static str, pub String);

impl KeyValue {
    fn string(&self) -> String {
        [self.0, "=", &self.1].concat()
    }
}

pub fn insert_history_log(
    table: &SourceTable,
    history_type: &str,
    loginname: &str,
    kvs: &[KeyValue],
    kphis: &str,
    kphis_log: &str,
) -> String {
    let where_clause = kvs.iter().map(|kv| kv.string()).collect::<Vec<String>>().join(" AND ");
    [
        "INSERT INTO ",kphis_log,".history_log (`history_datetime`,`history_table_name`,`history_type`,`history_user`, `data`) \
            SELECT NOW(),'",table.table_name(),"','",history_type,"','",loginname,"',",&table.data(&where_clause, kphis),
            " WHERE (SELECT EXISTS(SELECT * FROM ",kphis,".",table.table_name()," WHERE ",&where_clause,"));"
    ].concat()
}

pub enum SourceTable {
    IpdDrConsult,
    IpdDrConsultSignatureRequest,
    IpdDrConsultSignatureReply,

    IpdDoctorInCharge,

    IpdFocusList,
    IpdFocusListGoalItem,
    IpdFocusNote,
    IpdFocusNoteIntvtItem,
    IpdFocusNoteDlcItem,
    IpdIo,

    OpdErAllergyHistory,
    OpdErConsult,
    OpdErDocumentScan,
    OpdErDrPe,
    OpdErNurseScreening,

    OpdErFocusList,
    OpdErFocusListGoalItem,
    OpdErFocusNote,
    OpdErFocusNoteIntvtItem,
    OpdErFocusNoteDlcItem,
    OpdErSetFastTrack,
    OpdErIo,
}

impl SourceTable {
    pub fn table_name(&self) -> &'static str {
        match self {
            Self::IpdDrConsult => "ipd_dr_consult",
            Self::IpdDrConsultSignatureRequest => "ipd_dr_consult_signature_request",
            Self::IpdDrConsultSignatureReply => "ipd_dr_consult_signature_reply",

            Self::IpdDoctorInCharge => "ipd_doctor_in_charge",

            Self::IpdFocusList => "ipd_focus_list",
            Self::IpdFocusListGoalItem => "ipd_focus_list_goal_item",
            Self::IpdFocusNote => "ipd_focus_note",
            Self::IpdFocusNoteIntvtItem => "ipd_focus_note_intvt_item",
            Self::IpdFocusNoteDlcItem => "ipd_focus_note_dlc_item",

            Self::IpdIo => "ipd_io",

            Self::OpdErAllergyHistory => "opd_er_allergy_history",
            Self::OpdErConsult => "opd_er_consult",
            Self::OpdErDocumentScan => "opd_er_document_scan",
            Self::OpdErDrPe => "opd_er_dr_pe",
            Self::OpdErNurseScreening => "opd_er_nurse_screening",

            Self::OpdErFocusList => "opd_er_focus_list",
            Self::OpdErFocusListGoalItem => "opd_er_focus_list_goal_item",
            Self::OpdErFocusNote => "opd_er_focus_note",
            Self::OpdErFocusNoteIntvtItem => "opd_er_focus_note_intvt_item",
            Self::OpdErFocusNoteDlcItem => "opd_er_focus_note_dlc_item",

            Self::OpdErSetFastTrack => "opd_er_set_fast_track",

            Self::OpdErIo => "opd_er_io",
        }
    }
    /// (SELECT JSON_ARRAYAGG(JSON_OBJECT(k:v..) FROM XXX WHERE xxx_id=id)
    pub fn data(&self, where_clause: &str, kphis: &str) -> String {
        match self {
            Self::IpdDrConsult => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'consult_id',consult_id,\
                    'consult_type',consult_type,\
                    'consult_ward',consult_ward,\
                    'consult_emergency',consult_emergency,\
                    'consult_spclty',consult_spclty,\
                    'consult_doctorcode_mention',consult_doctorcode_mention,\
                    'consult_date',consult_date,\
                    'consult_time',consult_time,\
                    'consult_data',consult_data,\
                    'consult_datetime_create_reply',consult_datetime_create_reply,\
                    'consult_datetime_update_reply',consult_datetime_update_reply,\
                    'consult_finding',consult_finding,\
                    'consult_diagnosis',consult_diagnosis,\
                    'consult_recommendation',consult_recommendation,\
                    'consult_status',consult_status,\
                    'an',an,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_dr_consult WHERE ",where_clause,")"
            ].concat(),
            Self::IpdDrConsultSignatureRequest => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'consult_signature_id',consult_signature_id,\
                    'consult_id',consult_id,\
                    'consult_doctorcode_request',consult_doctorcode_request,\
                    'consult_doctorcode_request_person2',consult_doctorcode_request_person2,\
                    'an',an,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_dr_consult_signature_request WHERE ",where_clause,")"
            ].concat(),
            Self::IpdDrConsultSignatureReply => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'consult_reply_id',consult_reply_id,\
                    'consult_id',consult_id,\
                    'consult_doctorcode_reply',consult_doctorcode_reply,\
                    'consult_doctorcode_reply_person2',consult_doctorcode_reply_person2,\
                    'an',an,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_dr_consult_signature_reply WHERE ",where_clause,")"
            ].concat(),
            Self::IpdDoctorInCharge => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'doctor_in_charge_id',doctor_in_charge_id,\
                    'an',an,\
                    'hn',hn,\
                    'doctor',doctor,\
                    'spclty',spclty,\
                    'status',status,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_doctor_in_charge WHERE ",where_clause,")"
            ].concat(),
            Self::IpdFocusList => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fclist_id',fclist_id,\
                    'smp_id',smp_id,\
                    'focus_id',focus_id,\
                    'focus_text',focus_text,\
                    'goal_id',goal_id,\
                    'goal_text',goal_text,\
                    'fclist_stdate',fclist_stdate,\
                    'fclist_sttime',fclist_sttime,\
                    'fclist_enddate',fclist_enddate,\
                    'fclist_endtime',fclist_endtime,\
                    'fclist_status',fclist_status,\
                    'hn',hn,\
                    'an',an,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_focus_list WHERE ",where_clause,")"
            ].concat(),
            Self::IpdFocusListGoalItem => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fclist_item_id',fclist_item_id,\
                    'fclist_id',fclist_id,\
                    'goal_id',goal_id,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_focus_list_goal_item WHERE ",where_clause,")"
            ].concat(),
            Self::IpdFocusNote => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fcnote_id',fcnote_id,\
                    'general_symptoms',general_symptoms,\
                    'fclist_id',fclist_id,\
                    'assessment',assessment,\
                    'intvt_id',intvt_id,\
                    'intvt_text',intvt_text,\
                    'evalution',evalution,\
                    'dlc_id',dlc_id,\
                    'dlc_text',dlc_text,\
                    'other',other,\
                    'an',an,\
                    'hn',hn,\
                    'fcnote_date',fcnote_date,\
                    'fcnote_time',fcnote_time,\
                    'fcnote_patient_type',fcnote_patient_type,\
                    'ward',ward,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_focus_note WHERE ",where_clause,")"
            ].concat(),
            Self::IpdFocusNoteIntvtItem => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fcnote_intvt_item_id',fcnote_intvt_item_id,\
                    'fcnote_id',fcnote_id,\
                    'intvt_id',intvt_id,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_focus_note_intvt_item WHERE ",where_clause,")"
            ].concat(),
            Self::IpdFocusNoteDlcItem => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fcnote_dlc_item_id',fcnote_dlc_item_id,\
                    'fcnote_id',fcnote_id,\
                    'dlc_id',dlc_id,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_focus_note_dlc_item WHERE ",where_clause,")"
            ].concat(),
            Self::IpdIo => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'io_id',io_id,\
                    'io_date',io_date,\
                    'io_time',io_time,\
                    'io_parenteral_type',io_parenteral_type,\
                    'io_parenteral_name',io_parenteral_name,\
                    'io_parenteral_amount',io_parenteral_amount,\
                    'io_parenteral_absorb',io_parenteral_absorb,\
                    'io_parenteral_carry_forward',io_parenteral_carry_forward,\
                    'io_parenteral_remark',io_parenteral_remark,\
                    'io_oral_name',io_oral_name,\
                    'io_oral_amount',io_oral_amount,\
                    'io_oral_absorb',io_oral_absorb,\
                    'io_oral_carry_forward',io_oral_carry_forward,\
                    'io_oral_remark',io_oral_remark,\
                    'io_output_type',io_output_type,\
                    'io_output_amount',io_output_amount,\
                    'io_output_remark',io_output_remark,\
                    'an',an,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".ipd_io WHERE ",where_clause,")"
            ].concat(),

            Self::OpdErAllergyHistory => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'er_allergy_history_id',er_allergy_history_id,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'er_allergy_history_agent',er_allergy_history_agent,\
                    'er_allergy_history_symptom',er_allergy_history_symptom,\
                    'er_allergy_history_doctorcode',er_allergy_history_doctorcode,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_allergy_history WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErConsult => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'er_consult_id',er_consult_id,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'er_consult_ward',er_consult_ward,\
                    'er_consult_date',er_consult_date,\
                    'er_consult_time',er_consult_time,\
                    'er_consult_doctor_reply',er_consult_doctor_reply,\
                    'er_consult_date_reply',er_consult_date_reply,\
                    'er_consult_time_reply',er_consult_time_reply,\
                    'er_consult_doctorcode',er_consult_doctorcode,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_consult WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErDocumentScan => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'opd_er_document_scan_id',opd_er_document_scan_id,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'opd_er_document_scan',opd_er_document_scan,\
                    'opd_er_document_scan_doctorcode',opd_er_document_scan_doctorcode,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_document_scan WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErDrPe => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'opd_er_pe_id',opd_er_pe_id,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'arc',arc,\
                    'arc_npc_text',arc_npc_text,\
                    'breathing_chest_wall',breathing_chest_wall,\
                    'breathing_lung',breathing_lung,\
                    'circulation_shock',circulation_shock,\
                    'circulation_shock_text',circulation_shock_text,\
                    'circulation_other',circulation_other,\
                    'circulation_other_text',circulation_other_text,\
                    'circulation_efast_date',circulation_efast_date,\
                    'circulation_efast_time',circulation_efast_time,\
                    'circulation_doctor',circulation_doctor,\
                    'circulation',circulation,\
                    'circulation_positive_text',circulation_positive_text,\
                    'disability_e',disability_e,\
                    'disability_v',disability_v,\
                    'disability_m',disability_m,\
                    'disability_pupil_rt',disability_pupil_rt,\
                    'disability_pupil_lt',disability_pupil_lt,\
                    'disability_other',disability_other,\
                    'exposure',exposure,\
                    'doctor_pe',doctor_pe,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_dr_pe WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErNurseScreening => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'opd_er_screening_id',opd_er_screening_id,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'screening_emergency_level',screening_emergency_level,\
                    'screening_spclty',screening_spclty,\
                    'screening_arrive_date',screening_arrive_date,\
                    'screening_arrive_time',screening_arrive_time,\
                    'screening_date',screening_date,\
                    'screening_time',screening_time,\
                    'screening_report_date',screening_report_date,\
                    'screening_report_time',screening_report_time,\
                    'screening_see_doctor_date',screening_see_doctor_date,\
                    'screening_see_doctor_time',screening_see_doctor_time,\
                    'screening_doctor_doctorcode',screening_doctor_doctorcode,\
                    'screening_nurse_doctorcode',screening_nurse_doctorcode,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_nurse_screening WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErFocusList => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fclist_id',fclist_id,\
                    'smp_id',smp_id,\
                    'focus_id',focus_id,\
                    'focus_text',focus_text,\
                    'goal_id',goal_id,\
                    'goal_text',goal_text,\
                    'fclist_stdate',fclist_stdate,\
                    'fclist_sttime',fclist_sttime,\
                    'fclist_enddate',fclist_enddate,\
                    'fclist_endtime',fclist_endtime,\
                    'fclist_status',fclist_status,\
                    'opd_er_order_master_id',opd_er_order_master_id,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_focus_list WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErFocusListGoalItem => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fclist_item_id',fclist_item_id,\
                    'fclist_id',fclist_id,\
                    'goal_id',goal_id,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_focus_list_goal_item WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErFocusNote => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fcnote_id',fcnote_id,\
                    'general_symptoms',general_symptoms,\
                    'fclist_id',fclist_id,\
                    'assessment',assessment,\
                    'intvt_id',intvt_id,\
                    'intvt_text',intvt_text,\
                    'evalution',evalution,\
                    'dlc_id',dlc_id,\
                    'dlc_text',dlc_text,\
                    'other',other,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'fcnote_date',fcnote_date,\
                    'fcnote_time',fcnote_time,\
                    'fcnote_patient_type',fcnote_patient_type,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_focus_note WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErFocusNoteIntvtItem => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fcnote_intvt_item_id',fcnote_intvt_item_id,\
                    'fcnote_id',fcnote_id,\
                    'intvt_id',intvt_id,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_focus_note_intvt_item WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErFocusNoteDlcItem => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'fcnote_dlc_item_id',fcnote_dlc_item_id,\
                    'fcnote_id',fcnote_id,\
                    'dlc_id',dlc_id,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_focus_note_dlc_item WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErSetFastTrack => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'set_ft_id',set_ft_id,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'set_ft_date',set_ft_date,\
                    'set_ft_time',set_ft_time,\
                    'set_ft_doctorcode',set_ft_doctorcode,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_set_fast_track WHERE ",where_clause,")"
            ].concat(),
            Self::OpdErIo => [
                "(SELECT JSON_ARRAYAGG(JSON_OBJECT(\
                    'opd_er_io_id',opd_er_io_id,\
                    'opd_er_order_master_id',opd_er_order_master_id,\
                    'opd_er_io_date',opd_er_io_date,\
                    'opd_er_io_time',opd_er_io_time,\
                    'opd_er_io_parenteral_type',opd_er_io_parenteral_type,\
                    'opd_er_io_parenteral_name',opd_er_io_parenteral_name,\
                    'opd_er_io_parenteral_amount',opd_er_io_parenteral_amount,\
                    'opd_er_io_parenteral_absorb',opd_er_io_parenteral_absorb,\
                    'opd_er_io_parenteral_carry_forward',opd_er_io_parenteral_carry_forward,\
                    'opd_er_io_parenteral_remark',opd_er_io_parenteral_remark,\
                    'opd_er_io_oral_name',opd_er_io_oral_name,\
                    'opd_er_io_oral_amount',opd_er_io_oral_amount,\
                    'opd_er_io_oral_absorb',opd_er_io_oral_absorb,\
                    'opd_er_io_oral_carry_forward',opd_er_io_oral_carry_forward,\
                    'opd_er_io_oral_remark',opd_er_io_oral_remark,\
                    'opd_er_io_output_type',opd_er_io_output_type,\
                    'opd_er_io_output_amount',opd_er_io_output_amount,\
                    'opd_er_io_output_remark',opd_er_io_output_remark,",CREATE_UPDATE_VERSION,
                ")) FROM ",kphis,".opd_er_io WHERE ",where_clause,")"
            ].concat(),
        }
    }
}
