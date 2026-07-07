use axum::{
    Router,
    body::Body,
    extract::DefaultBodyLimit,
    http::{
        Request,
        StatusCode,
        // header::{self, HeaderValue},
    },
    routing::{delete, get, patch, post},
};
use std::time::Duration;
use tower_cookies::CookieManagerLayer;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    compression::{
        CompressionLayer,
        predicate::{NotForContentType, Predicate, SizeAbove},
    },
    // set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::Level;

use kphis_api_core::state::ApiState;
use kphis_model::{PATH_API_PATIENT_IMAGE, PATH_API_XRAY_IMAGE, PATH_API_XRAY_THUMBNAIL, endpoint::EndPoint};

// we use handler name from kphis's php function name
#[rustfmt::skip]
pub fn api_router(state: ApiState) -> Router {
    let request_body_limited_mb = state.app_config.request_body_limited_mb as usize;
    let compression_predicate = SizeAbove::new(1024)
        // SSE *MUST NOT* COMPRESS, if compressed, data will send once when closed
        .and(NotForContentType::const_new("text/event-stream"))
        // image and woff file already compressed
        .and(NotForContentType::IMAGES)
        .and(NotForContentType::const_new("font/woff"))
        .and(NotForContentType::const_new("font/woff2"));

    // Create rate-limiting layer
    let governor_config = GovernorConfigBuilder::default()
        .burst_size(state.app_config.rate_limit_burst_size) // quota of xxx per IP address
        .per_millisecond(state.app_config.rate_limit_replenish_every_millisecond) // replenish 1 every xxx millisecond
        .finish()
        .expect("Cannot create Rate Limit Config");
    let governor_layer = GovernorLayer::new(governor_config);

    // *NOTE* please sort similar endpoint from long to short: ex IpdNameId before IpdName before Ipd
    // Naming with
    // - no pleural form
    // - verbose parameter-name-in-path ex: money-id/{id} or person-hn/{hn}
    Router::new()
        .route(&EndPoint::AvatarOpdEr,
            get(kphis_api_handler::avatar::get_avatar_opd_er))
        .route(&EndPoint::AvatarIpd,
            get(kphis_api_handler::avatar::get_avatar_ipd))
        .route(&EndPoint::DrugUseDuration,
            get(kphis_api_handler::drug_use_duration::get_drug_use_duration)
            .post(kphis_api_handler::drug_use_duration::post_drug_use_duration))
        .route(&EndPoint::EmrDateHn,
            get(kphis_api_handler::emr::get_emr_date))
        .route(&EndPoint::EmrVisitVn,
            get(kphis_api_handler::emr::get_emr_visit))
        .route(&EndPoint::ExistsKeyId,
            get(kphis_api_handler::app::get_exists))
        .route(&EndPoint::HisIptDiagAn,
            get(kphis_api_handler::ipd::his::get_his_ipt_diag))
        .route(&EndPoint::HisIptOprtAn,
            get(kphis_api_handler::ipd::his::get_his_ipt_oprt))
        .route(&EndPoint::HisOperationAdmitAn,
            get(kphis_api_handler::ipd::his::get_ipd_his_opertion_admit))
        .route(&EndPoint::HisMedPlanIpdAn,
            get(kphis_api_handler::ipd::his::get_med_plan_ipd_remains))
        .route(&EndPoint::HisReferOutVnan,
            get(kphis_api_handler::refer_out::get_his_referout_data)
            .post(kphis_api_handler::refer_out::post_his_referout))
        .route(&EndPoint::Image,
            post(kphis_api_handler::image::file_path::post_image_file)
            .patch(kphis_api_handler::image::file_path::patch_image_path)
            .delete(kphis_api_handler::image::file_path::delete_image_file))
        .route(&EndPoint::ImageUsage,
            post(kphis_api_handler::image::file_path::post_image_usage)
            .delete(kphis_api_handler::image::file_path::delete_image_usage))
        .route(&EndPoint::ImageUsageId,
            get(kphis_api_handler::image::file_path::get_image_usage_id))
        .route(&EndPoint::IpdAdmissionNoteDrAn,
            get(kphis_api_handler::ipd::admission_note_dr::get_ipd_admission_note_dr))
        .route(&EndPoint::IpdAdmissionNoteDr,
            post(kphis_api_handler::ipd::admission_note_dr::post_ipd_admission_note_dr)
            .put(kphis_api_handler::ipd::admission_note_dr::put_ipd_admission_note_dr))
        .route(&EndPoint::IpdAdmissionNoteNurseAn,
            get(kphis_api_handler::ipd::admission_note_nurse::get_ipd_admission_note_nurse))
        .route(&EndPoint::IpdAdmissionNoteNurse,
            post(kphis_api_handler::ipd::admission_note_nurse::post_ipd_admission_note_nurse)
            .put(kphis_api_handler::ipd::admission_note_nurse::put_ipd_admission_note_nurse))
        .route(&EndPoint::IpdAdmissionNoteDrPharmCheckAn,
            patch(kphis_api_handler::ipd::admission_note_dr::patch_ipd_pharmacy_check))
        .route(&EndPoint::IpdConsult,
            get(kphis_api_handler::ipd::consult::get_ipd_consult_list)
            .post(kphis_api_handler::ipd::consult::post_ipd_consult)
            .delete(kphis_api_handler::ipd::consult::delete_ipd_consult_by_id))
        .route(&EndPoint::IpdConsultAn,
            get(kphis_api_handler::ipd::consult::get_ipd_consult_by_an))
        .route(&EndPoint::IpdConsultId,
            get(kphis_api_handler::ipd::consult::get_ipd_consult_by_id))
        .route(&EndPoint::IpdDcPlanAn,
            get(kphis_api_handler::ipd::dc_plan::get_ipd_dc_plan)
            .post(kphis_api_handler::ipd::dc_plan::post_ipd_dc_plan)
            .delete(kphis_api_handler::ipd::dc_plan::delete_ipd_dc_plan))
        .route(&EndPoint::IpdDcPlanTmpDx,
            get(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_dx)
            .post(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_dx)
            .delete(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_dx))
        .route(&EndPoint::IpdDcPlanTmpMed,
            get(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_med)
            .post(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_med)
            .delete(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_med))
        .route(&EndPoint::IpdDcPlanTmpEnv,
            get(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_env)
            .post(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_env)
            .delete(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_env))
        .route(&EndPoint::IpdDcPlanTmpTx,
            get(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_tx)
            .post(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_tx)
            .delete(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_tx))
        .route(&EndPoint::IpdDcPlanTmpDiet,
            get(kphis_api_handler::ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_diet)
            .post(kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_diet)
            .delete(kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_diet))
        .route(&EndPoint::IpdDoctorInCharge,
            get(kphis_api_handler::ipd::doctor_in_charge::get_ipd_doctor_in_charge)
            .post(kphis_api_handler::ipd::doctor_in_charge::post_ipd_doctor_in_charge)
            .delete(kphis_api_handler::ipd::doctor_in_charge::delete_ipd_doctor_in_charge))
        .route(&EndPoint::IpdDocumentDatetimeAn,
            get(kphis_api_handler::ipd::document::get_ipd_document_datetime))
        .route(&EndPoint::IpdDocumentListVnAn,
            get(kphis_api_handler::ipd::document::get_ipd_document_list))
        .route(&EndPoint::IpdDocumentScanAn,
            get(kphis_api_handler::ipd::document::get_ipd_document_types)
            .post(kphis_api_handler::ipd::document::post_ipd_document_type)
            .delete(kphis_api_handler::ipd::document::delete_ipd_document_type))
        .route(&EndPoint::IpdFocusListAn,
            get(kphis_api_handler::ipd::focus_list::get_ipd_focus_list)
            .post(kphis_api_handler::ipd::focus_list::post_ipd_focus_list)
            .delete(kphis_api_handler::ipd::focus_list::delete_ipd_focus_list))
        .route(&EndPoint::IpdFocusNoteAn,
            get(kphis_api_handler::ipd::focus_note::get_ipd_focus_note)
            .post(kphis_api_handler::ipd::focus_note::post_ipd_focus_note)
            .delete(kphis_api_handler::ipd::focus_note::delete_ipd_focus_note))
        .route(&EndPoint::IpdIndexActionId,
            delete(kphis_api_handler::ipd::index_action::delete_ipd_index_action))
        .route(&EndPoint::IpdIndexAction,
            post(kphis_api_handler::ipd::index_action::post_ipd_index_action))
        .route(&EndPoint::IpdIndexMedPayAn,
            get(kphis_api_handler::ipd::index_plan::get_ipd_index_med_pay))
        .route(&EndPoint::IpdIndexMonitorId,
            delete(kphis_api_handler::ipd::index_monitor::delete_ipd_index_monitor))
        .route(&EndPoint::IpdIndexMonitor,
            post(kphis_api_handler::ipd::index_monitor::post_ipd_index_monitor))
        .route(&EndPoint::IpdIndexNoteId,
            delete(kphis_api_handler::ipd::index_note::delete_ipd_index_note))
        .route(&EndPoint::IpdIndexNote,
            get(kphis_api_handler::ipd::index_note::get_ipd_index_note)
            .post(kphis_api_handler::ipd::index_note::post_ipd_index_note))
        .route(&EndPoint::IpdIndexPlan,
            post(kphis_api_handler::ipd::index_plan::post_ipd_index_plan))
        .route(&EndPoint::IpdIndexPlanDateAn,
            get(kphis_api_handler::ipd::index_plan::get_index_plan_date))
        .route(&EndPoint::IpdIndexPlanId,
            delete(kphis_api_handler::ipd::index_plan::delete_ipd_index_plan))
        .route(&EndPoint::IpdIo,
            get(kphis_api_handler::ipd::io::get_ipd_io_shift)
            .post(kphis_api_handler::ipd::io::post_ipd_io_shift)
            .delete(kphis_api_handler::ipd::io::delete_ipd_io_shift))
        .route(&EndPoint::IpdIoDateAn,
            get(kphis_api_handler::ipd::io::get_ipd_io_date))
        .route(&EndPoint::IpdMedReconcile,
            get(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile)
            .post(kphis_api_handler::ipd::med_reconcile::post_ipd_med_reconcile)
            .patch(kphis_api_handler::ipd::med_reconcile::patch_ipd_med_reconcile)
            .delete(kphis_api_handler::ipd::med_reconcile::delete_ipd_med_reconcile))
        .route(&EndPoint::IpdMedReconcileHosxpAn,
            get(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_hosxp))
        .route(&EndPoint::IpdMedReconcileLastDoseAn,
            get(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_last_dose))
        .route(&EndPoint::IpdMedReconcileNoteId,
            get(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_note)
            .post(kphis_api_handler::ipd::med_reconcile::post_ipd_med_reconcile_note))
        .route(&EndPoint::IpdMedReconcileRemedVisitHn,
            get(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_remed_visit))
        .route(&EndPoint::IpdMedReconcileRemedMed,
            get(kphis_api_handler::ipd::med_reconcile::get_ipd_med_reconcile_remed_med))
        .route(&EndPoint::IpdMra,
            get(kphis_api_handler::ipd::mra::get_ipd_mra)
            .post(kphis_api_handler::ipd::mra::post_ipd_mra)
            .put(kphis_api_handler::ipd::mra::put_ipd_mra)
            .delete(kphis_api_handler::ipd::mra::delete_ipd_mra))
        .route(&EndPoint::IpdOrderItem,
            get(kphis_api_handler::ipd::order::get_ipd_order_item)
            .patch(kphis_api_handler::ipd::order::patch_ipd_order_item))
        .route(&EndPoint::IpdOrderOnedayPreviousAn,
            get(kphis_api_handler::ipd::order::get_ipd_order_one_day_previous))
        .route(&EndPoint::IpdOrderOrderDateAn,
            get(kphis_api_handler::ipd::order::get_ipd_order_date))
        .route(&EndPoint::IpdOrderOrderId,
            delete(kphis_api_handler::ipd::order::delete_ipd_order))
        .route(&EndPoint::IpdOrderOrder,
            get(kphis_api_handler::ipd::order::get_ipd_order)
            .post(kphis_api_handler::ipd::order::post_ipd_order)
            .patch(kphis_api_handler::ipd::order::patch_ipd_order))
        .route(&EndPoint::IpdOrderPharmacy,
            get(kphis_api_handler::ipd::order::get_ipd_order_pharmacy))
        .route(&EndPoint::IpdOrderPrevious,
            get(kphis_api_handler::ipd::order::get_ipd_order_previous))
        .route(&EndPoint::IpdOrderProgressNoteId,
            delete(kphis_api_handler::ipd::progress_note::delete_ipd_progress_note))
        .route(&EndPoint::IpdOrderProgressNote,
            get(kphis_api_handler::ipd::progress_note::get_ipd_progress_note)
            .post(kphis_api_handler::ipd::progress_note::post_ipd_progress_note))
        .route(&EndPoint::IpdOrderProgressPrevious,
            get(kphis_api_handler::ipd::progress_note::get_ipd_progress_previous))
        .route(&EndPoint::IpdOrderToHomeMedAn,
            get(kphis_api_handler::ipd::order::get_ipd_home_med_from_cont))
        .route(&EndPoint::IpdPasscode,
            get(kphis_api_handler::ipd::passcode::get_ipd_ward_passcode)
            .post(kphis_api_handler::ipd::passcode::post_ipd_ward_passcode))
        .route(&EndPoint::IpdPostAdmitCount,
            get(kphis_api_handler::post_admit::get_ipd_post_admit_count))
        .route(&EndPoint::IpdPostAdmitList,
            get(kphis_api_handler::post_admit::get_ipd_post_admit_list))
        .route(&EndPoint::IpdPreAdmit,
            get(kphis_api_handler::pre_admit::get_ipd_pre_admit_list)
            .post(kphis_api_handler::pre_admit::post_ipd_pre_admit)
            .patch(kphis_api_handler::pre_admit::patch_ipd_pre_admit))
        .route(&EndPoint::IpdPreOrderMasterId,
            delete(kphis_api_handler::pre_order::delete_ipd_pre_order_master))
        .route(&EndPoint::IpdPreOrderMaster,
            get(kphis_api_handler::pre_order::get_ipd_pre_order_list)
            .post(kphis_api_handler::pre_order::post_ipd_pre_order_master))
        .route(&EndPoint::IpdPreOrderInto,
            post(kphis_api_handler::pre_order::post_ipd_pre_order_into))
        .route(&EndPoint::IpdPreOrderOrderId,
            delete(kphis_api_handler::pre_order::delete_ipd_pre_order))
        .route(&EndPoint::IpdPreOrderOrder,
            get(kphis_api_handler::pre_order::get_ipd_pre_order)
            .post(kphis_api_handler::pre_order::post_ipd_pre_order))
        .route(&EndPoint::IpdPreOrderProgressNoteId,
            delete(kphis_api_handler::pre_order::delete_ipd_pre_progress_note))
        .route(&EndPoint::IpdPreOrderProgressNote,
            get(kphis_api_handler::pre_order::get_ipd_pre_progress_note)
            .post(kphis_api_handler::pre_order::post_ipd_pre_progress_note))
        .route(&EndPoint::IpdShowPatientMainAn,
            get(kphis_api_handler::ipd::show_patient_main::get_ipd_show_patient_main))
        .route(&EndPoint::IpdSummary,
            get(kphis_api_handler::ipd::summary::get_ipd_summary)
            .post(kphis_api_handler::ipd::summary::post_ipd_summary)
            .patch(kphis_api_handler::ipd::summary::patch_ipd_summary))
        .route(&EndPoint::IpdSummaryAudit,
            get(kphis_api_handler::ipd::summary_audit::get_ipd_summary_audit)
            .post(kphis_api_handler::ipd::summary_audit::post_ipd_summary_audit)
            .delete(kphis_api_handler::ipd::summary_audit::delete_ipd_summary_audit))
        .route(&EndPoint::IpdSummaryNoteId,
            get(kphis_api_handler::ipd::summary::get_ipd_summary_note)
            .post(kphis_api_handler::ipd::summary::post_ipd_summary_note)
            .patch(kphis_api_handler::ipd::summary::patch_ipd_summary_note)
            .delete(kphis_api_handler::ipd::summary::delete_ipd_summary_note))
        .route(&EndPoint::IpdSummaryStatusId,
            get(kphis_api_handler::ipd::summary::get_ipd_summary_status)
            .put(kphis_api_handler::ipd::summary::put_ipd_summary_status))
        .route(&EndPoint::IpdTmpGroup,
            get(kphis_api_handler::ipd::tmp::get_ipd_tmp_group)
            .post(kphis_api_handler::ipd::tmp::post_ipd_tmp_group)
            .delete(kphis_api_handler::ipd::tmp::delete_ipd_tmp_group))
        .route(&EndPoint::IpdTmpSubgroup,
            get(kphis_api_handler::ipd::tmp::get_ipd_subgroup)
            .post(kphis_api_handler::ipd::tmp::post_ipd_subgroup)
            .delete(kphis_api_handler::ipd::tmp::delete_ipd_subgroup))
        .route(&EndPoint::IpdTmpFocus,
            get(kphis_api_handler::ipd::tmp::get_ipd_focus)
            .post(kphis_api_handler::ipd::tmp::post_ipd_focus)
            .delete(kphis_api_handler::ipd::tmp::delete_ipd_focus))
        .route(&EndPoint::IpdTmpGoal,
            get(kphis_api_handler::ipd::tmp::get_ipd_goal)
            .post(kphis_api_handler::ipd::tmp::post_ipd_goal)
            .delete(kphis_api_handler::ipd::tmp::delete_ipd_goal))
        .route(&EndPoint::IpdTmpIntvt,
            get(kphis_api_handler::ipd::tmp::get_ipd_intvt)
            .post(kphis_api_handler::ipd::tmp::post_ipd_intvt)
            .delete(kphis_api_handler::ipd::tmp::delete_ipd_intvt))
        .route(&EndPoint::IpdTmpDlc,
            get(kphis_api_handler::ipd::tmp::get_ipd_dlc)
            .post(kphis_api_handler::ipd::tmp::post_ipd_dlc)
            .delete(kphis_api_handler::ipd::tmp::delete_ipd_dlc))
        .route(&EndPoint::IpdVitalSign,
            get(kphis_api_handler::ipd::vital_sign::get_ipd_vital_sign)
            .post(kphis_api_handler::ipd::vital_sign::post_ipd_vital_sign)
            .put(kphis_api_handler::ipd::vital_sign::put_ipd_vital_sign))
        .route(&EndPoint::IpdVitalSignId,
            delete(kphis_api_handler::ipd::vital_sign::delete_ipd_vital_sign))
        .route(&EndPoint::LabHead,
            get(kphis_api_handler::lab::get_lab_head))
        .route(&EndPoint::LabItem,
            get(kphis_api_handler::lab::get_lab_item))
        .route(&EndPoint::LabReadId,
            post(kphis_api_handler::lab::post_lab_read)
            .delete(kphis_api_handler::lab::delete_lab_read))
        .route(&EndPoint::LabWbcKeyValue,
            get(kphis_api_handler::lab::get_wbc_band))
        .route(&EndPoint::MedReconcileHn,
            get(kphis_api_handler::med_reconciliation::get_med_reconciliation_head))
        .route(&EndPoint::OpdErDcPlanId,
            get(kphis_api_handler::opd_er::dc_plan::get_opd_er_dc_plan)
            .post(kphis_api_handler::opd_er::dc_plan::post_opd_er_dc_plan)
            .delete(kphis_api_handler::opd_er::dc_plan::delete_opd_er_dc_plan))
        .route(&EndPoint::OpdErDocumentListVnId,
            get(kphis_api_handler::opd_er::document::get_opd_er_document_list))
        .route(&EndPoint::OpdErDocumentScanId,
            get(kphis_api_handler::opd_er::document::get_opd_er_document_types)
            .post(kphis_api_handler::opd_er::document::post_opd_er_document_type)
            .delete(kphis_api_handler::opd_er::document::delete_opd_er_document_type))
        .route(&EndPoint::OpdErFocusListId,
            get(kphis_api_handler::opd_er::focus_list::get_opd_er_focus_list)
            .post(kphis_api_handler::opd_er::focus_list::post_opd_er_focus_list)
            .delete(kphis_api_handler::opd_er::focus_list::delete_opd_er_focus_list))
        .route(&EndPoint::OpdErFocusNoteId,
            get(kphis_api_handler::opd_er::focus_note::get_opd_er_focus_note)
            .post(kphis_api_handler::opd_er::focus_note::post_opd_er_focus_note)
            .delete(kphis_api_handler::opd_er::focus_note::delete_opd_er_focus_note))
        .route(&EndPoint::OpdErHisMedVn,
            get(kphis_api_handler::opd_er::hosxp_med::get_opd_med))
        .route(&EndPoint::OpdErIndexActionId,
            delete(kphis_api_handler::opd_er::index_action::delete_opd_er_index_action))
        .route(&EndPoint::OpdErIndexAction,
            post(kphis_api_handler::opd_er::index_action::post_opd_er_index_action))
        .route(&EndPoint::OpdErIndexMonitorId,
            delete(kphis_api_handler::opd_er::index_monitor::delete_opd_er_index_monitor))
        .route(&EndPoint::OpdErIndexMonitor,
            post(kphis_api_handler::opd_er::index_monitor::post_opd_er_index_monitor))
        .route(&EndPoint::OpdErIndexPlanId,
            delete(kphis_api_handler::opd_er::index_plan::delete_opd_er_index_plan))
        .route(&EndPoint::OpdErIndexPlan,
            post(kphis_api_handler::opd_er::index_plan::post_opd_er_index_plan))
        .route(&EndPoint::OpdErIo,
            get(kphis_api_handler::opd_er::io::get_opd_er_io_shift)
            .post(kphis_api_handler::opd_er::io::post_opd_er_io_shift)
            .delete(kphis_api_handler::opd_er::io::delete_opd_er_io_shift))
        .route(&EndPoint::OpdErIoDateId,
            get(kphis_api_handler::opd_er::io::get_opd_er_io_date))
        .route(&EndPoint::OpdErMedicalHistory,
            get(kphis_api_handler::opd_er::medical_history::get_opd_er_medical_history))
        .route(&EndPoint::OpdErMedicalHistoryAllergy,
            get(kphis_api_handler::opd_er::medical_history::get_opd_er_allergy_history)
            .post(kphis_api_handler::opd_er::medical_history::post_opd_er_allergy_history))
        .route(&EndPoint::OpdErMedicalHistoryConsult,
            get(kphis_api_handler::opd_er::medical_history::get_opd_er_consult_history)
            .post(kphis_api_handler::opd_er::medical_history::post_opd_er_consult_history))
        .route(&EndPoint::OpdErMedicalHistoryFt,
            get(kphis_api_handler::opd_er::medical_history::get_opd_er_ft_history)
            .post(kphis_api_handler::opd_er::medical_history::post_opd_er_ft_history))
        .route(&EndPoint::OpdErMedicalHistoryScan,
            get(kphis_api_handler::opd_er::medical_history::get_opd_er_scan_history)
            .post(kphis_api_handler::opd_er::medical_history::post_opd_er_scan_history))
        .route(&EndPoint::OpdErMedicalHistoryScreen,
            get(kphis_api_handler::opd_er::medical_history::get_opd_er_screen_history)
            .post(kphis_api_handler::opd_er::medical_history::post_opd_er_screen_history))
        .route(&EndPoint::OpdErMedicalHistoryTrauma,
            get(kphis_api_handler::opd_er::medical_history::get_opd_er_trauma_history)
            .post(kphis_api_handler::opd_er::medical_history::post_opd_er_trauma_history))
        .route(&EndPoint::OpdErMedReconcile,
            get(kphis_api_handler::opd_er::med_reconcile::get_opd_er_med_reconcile)
            .post(kphis_api_handler::opd_er::med_reconcile::post_opd_er_med_reconcile)
            .patch(kphis_api_handler::opd_er::med_reconcile::patch_opd_er_med_reconcile)
            .delete(kphis_api_handler::opd_er::med_reconcile::delete_opd_er_med_reconcile))
        .route(&EndPoint::OpdErMedReconcileNoteId,
            get(kphis_api_handler::opd_er::med_reconcile::get_opd_er_med_reconcile_note)
            .post(kphis_api_handler::opd_er::med_reconcile::post_opd_er_med_reconcile_note))
        .route(&EndPoint::OpdErOrderMasterCheckVn,
            get(kphis_api_handler::opd_er::order_master::get_opd_er_order_master_check))
        .route(&EndPoint::OpdErOrderMasterId,
            get(kphis_api_handler::opd_er::order_master::get_opd_er_order_master))
        .route(&EndPoint::OpdErOrderMaster,
            get(kphis_api_handler::opd_er::order_master::get_opd_er_order_master_list)
            .post(kphis_api_handler::opd_er::order_master::post_opd_er_order_master))
        .route(&EndPoint::OpdErOrderItem,
            get(kphis_api_handler::opd_er::order::get_opd_er_order_item)
            .patch(kphis_api_handler::opd_er::order::patch_opd_er_order_item))
        .route(&EndPoint::OpdErOrderOrderId,
            delete(kphis_api_handler::opd_er::order::delete_opd_er_order))
        .route(&EndPoint::OpdErOrderOrder,
            get(kphis_api_handler::opd_er::order::get_opd_er_order)
            .post(kphis_api_handler::opd_er::order::post_opd_er_order)
            .patch(kphis_api_handler::opd_er::order::patch_opd_er_order))
        .route(&EndPoint::OpdErOrderProgressNoteId,
            delete(kphis_api_handler::opd_er::progress_note::delete_opd_er_progress_note))
        .route(&EndPoint::OpdErOrderProgressNote,
            get(kphis_api_handler::opd_er::progress_note::get_opd_er_progress_note)
            .post(kphis_api_handler::opd_er::progress_note::post_opd_er_progress_note))
        .route(&EndPoint::OpdErOrderPharmacy,
            get(kphis_api_handler::opd_er::order::get_opd_er_order_pharmacy))
        .route(&EndPoint::OpdErShowPatientMainId,
            get(kphis_api_handler::opd_er::show_patient_main::get_opd_er_show_patient_main_id))
        .route(&EndPoint::OpdErShowPatientMainVn,
            get(kphis_api_handler::opd_er::show_patient_main::get_opd_er_show_patient_main_vn))
        .route(&EndPoint::OpdErVitalSignId,
            delete(kphis_api_handler::opd_er::vital_sign::delete_opd_er_vital_sign))
        .route(&EndPoint::OpdErVitalSign,
            get(kphis_api_handler::opd_er::vital_sign::get_opd_er_vital_sign)
            .post(kphis_api_handler::opd_er::vital_sign::post_opd_er_vital_sign)
            .put(kphis_api_handler::opd_er::vital_sign::put_opd_er_vital_sign))
        .route(&EndPoint::PrescrptionScreen,
            get(kphis_api_handler::prescription::get_prescription_screen)
            .post(kphis_api_handler::prescription::post_prescription_screen)
            .patch(kphis_api_handler::prescription::patch_prescription_screen))
        .route(&EndPoint::ReferNoteVnan,
            get(kphis_api_handler::refer_note::get_refernote)
            .post(kphis_api_handler::refer_note::post_refernote))
        .route(&EndPoint::ReportRawTemplateTypeId,
            get(kphis_api_pdf::handler::get_raw_single_template))
        .route(&EndPoint::ReportTemplateTypeId,
            get(kphis_api_pdf::handler::get_single_pdf))
        .route(&EndPoint::ScanHisImage,
            get(kphis_api_handler::image::scan_his::get_scan_his_image))
        .route(&EndPoint::SearchBoxHospText,
            get(kphis_api_handler::search::searchbox::get_hosp_searchbox))
        .route(&EndPoint::SearchBoxMedDuplicate,
            get(kphis_api_handler::search::searchbox::get_drug_duplication_check))
        .route(&EndPoint::SearchBoxMedInteraction,
            get(kphis_api_handler::search::searchbox::get_drug_interaction_check))
        .route(&EndPoint::SearchBoxMedHnText,
            get(kphis_api_handler::search::searchbox::get_med_searchbox))
        .route(&EndPoint::SearchBoxOpdVisitModeText,
            get(kphis_api_handler::search::searchbox::get_opd_visit_searchbox))
        .route(&EndPoint::SearchBoxIvfluidText,
            get(kphis_api_handler::search::searchbox::get_ivfluid_searchbox))
        .route(&EndPoint::SearchBoxLabText,
            get(kphis_api_handler::search::searchbox::get_lab_searchbox))
        .route(&EndPoint::SearchBoxPatientText,
            get(kphis_api_handler::search::searchbox::get_patient_searchbox))
        .route(&EndPoint::SearchBoxXrayText,
            get(kphis_api_handler::search::searchbox::get_xray_searchbox))
        .route(&EndPoint::SearchDr,
            get(kphis_api_handler::search::ipd_search_patient_dr::get_ipd_dr_search_patient))
        .route(&EndPoint::SearchNurse,
            get(kphis_api_handler::search::ipd_search_patient_nurse::get_ipd_nurse_search_patient))
        .route(&EndPoint::SearchPharmacist,
            get(kphis_api_handler::search::ipd_search_patient_pharmacist::get_ipd_pharmacist_search_patient))
        .route(&EndPoint::SearchOther,
            get(kphis_api_handler::search::ipd_search_patient_other::get_ipd_other_search_patient))
        .route(&EndPoint::Sse,
            // GET moved out to prevent 429
            delete(kphis_api_handler::sse::logout))
        .route(&EndPoint::SseGroup,
            post(kphis_api_handler::sse::post_sse_group))
        .route(&EndPoint::SseMessage,
            get(kphis_api_handler::sse::get_sse_message)
            .post(kphis_api_handler::sse::post_sse_message)
            .patch(kphis_api_handler::sse::patch_sse_messages))
        .route(&EndPoint::User,
            get(kphis_api_handler::user::his::refresh_token)
            .post(kphis_api_handler::user::his::check_login)
            .put(kphis_api_handler::user::his::refresh_cookie)
            .patch(kphis_api_handler::user::his::check_totp))
        .route(&EndPoint::UserConfig,
            post(kphis_api_handler::user::config::post_user_config)
            .patch(kphis_api_handler::user::config::patch_user_config))
        .route(&EndPoint::XrayReportHn,
            get(kphis_api_handler::xray::get_xray_report))
        .route(&EndPoint::XrayReadId,
            post(kphis_api_handler::xray::post_xray_read)
            .delete(kphis_api_handler::xray::delete_xray_read))
        .route(&EndPoint::XrayPacsXn,
            get(kphis_api_handler::pacs::get_pacs_xn))
        .route(&EndPoint::ReportCustom,
            get(kphis_api_handler::report::get_custom_report)
            .post(kphis_api_handler::report::post_custom_report)
            .delete(kphis_api_handler::report::delete_custom_report))
        .route(&EndPoint::ReportRawQuery,
            post(kphis_api_handler::report::post_query_to_json_string))
        .route(&EndPoint::UserRolePrelude,
            get(kphis_api_handler::user::role::get_user_role_prelude))
        .route(&EndPoint::UserRoleRole,
            get(kphis_api_handler::user::role::get_role_permission_list)
            .post(kphis_api_handler::user::role::post_role_permission)
            .delete(kphis_api_handler::user::role::delete_role_permission))
        .route(&EndPoint::UserRoleUser,
            get(kphis_api_handler::user::role::get_user_role_list)
            .post(kphis_api_handler::user::role::post_user_role))
        .with_state(state)
        // we do not use Authorization layer because we need UserStage in handlers
        // so we authorized in UserState extraction (see.. impl<S> FromRequestParts<S> for UserState)
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(CookieManagerLayer::new())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                tracing::span!(
                    Level::DEBUG,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(ulid::Ulid::new()),
                )
            }),
        )
        .layer(DefaultBodyLimit::max(request_body_limited_mb * 1024 * 1024))
        .layer(CompressionLayer::new().compress_when(compression_predicate))
        .layer(governor_layer)
}

#[rustfmt::skip]
pub fn sse_get_router(state: ApiState) -> Router {
    Router::new()
        .route("/any", get(kphis_api_handler::sse::get_sse))
        .route("/id/{state_id}", get(kphis_api_handler::sse::get_sse_by_id))
        .with_state(state)
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
}

#[rustfmt::skip]
pub fn assets_router(state: ApiState) -> Router {
    let compression_predicate = SizeAbove::new(1024);
    // let etag_opt = state.app_asset_cache_etag.read().ok();
    // let etag = etag_opt.map(|lock| (*lock).clone()).unwrap_or(String::from("AAAAAAdbzRU="));
    Router::new()
        .route("/",get(kphis_api_handler::app::get_app_asset).patch(kphis_api_handler::app::patch_assets))
        .with_state(state)
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        // .layer(SetResponseHeaderLayer::if_not_present(header::CACHE_CONTROL,HeaderValue::from_static("max-age=86400"))) // 24 hr
        // .layer(SetResponseHeaderLayer::if_not_present(header::ETAG,HeaderValue::from_str(&etag).unwrap_or(HeaderValue::from_static("AAAAAAdbzRU="))))
        .layer(CompressionLayer::new().compress_when(compression_predicate))
}

#[rustfmt::skip]
pub fn custom_template_router(state: ApiState) -> Router {
    let compression_predicate = SizeAbove::new(1024);
    Router::new()
        .route("/config.typ",get(kphis_api_pdf::handler::get_config_template))
        .route("/{template}",get(kphis_api_pdf::handler::get_custom_template))
        .with_state(state)
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(CompressionLayer::new().compress_when(compression_predicate))
}

#[rustfmt::skip]
pub fn img_router(state: ApiState) -> Router {
    Router::new()
        .route(PATH_API_PATIENT_IMAGE,get(kphis_api_handler::image::patient::get_patient_image))
        .route(PATH_API_XRAY_THUMBNAIL, get(kphis_api_handler::pacs::get_pacs_thumbnail))
        .route(PATH_API_XRAY_IMAGE, get(kphis_api_handler::pacs::get_pacs_image))
        .with_state(state)
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
}
