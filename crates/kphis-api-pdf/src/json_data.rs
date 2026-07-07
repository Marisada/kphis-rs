// All GET api that Typst can call 'get_file'
// via #let data = json("path")

use std::path::Path;
use strum::VariantArray;
use typst_library::diag::{EcoString, FileError};

use kphis_api_core::state::{ApiState, UserState};
use kphis_api_query::report::select_report_template_content;
use kphis_model::{
    DEFAULT_SVG_USER, PATH_PREFIX_PATIENT_IMAGE,
    endpoint::{EndPoint, QueryString, find_qs},
};
use kphis_util::error::{AppError, Source};

use crate::handler::get_config_template_inner;

// hook to handler functions in route.rs's Router provided by actor.rs
pub async fn get_json_data(path: &Path, app: &ApiState, user: &UserState) -> Result<Vec<u8>, FileError> {
    // dbg!(path.to_str().unwrap_or_default());
    let path_str = path.to_str().ok_or_else(|| FileError::InvalidUtf8)?;
    // we need to split query part first, because Path::new("/some?a=b").start_with("/some) is false
    let mut paths_params = path_str.split('?');
    // we need to convert to path again because Path::new("/some/thing").to_str() may be Some("\some\thing")
    // and "\some\thing".start_with("/some/thing") is false
    if let Some(path) = paths_params.next().map(Path::new) {
        // /customs/config.typ
        if path == Path::new("customs/config.typ") {
            Ok(get_config_template_inner(app).as_bytes().to_vec())
        // /customs/{template}
        } else if path.starts_with("customs/") {
            let paths = path.strip_prefix("customs/").unwrap_or(path).iter().map(|s| s.to_str().unwrap_or_default()).collect::<Vec<&str>>();
            if paths.len() == 1 {
                match select_report_template_content(paths[0], &app.db_pool, &app.kphis_extra()).await {
                    Ok(Some(report)) => Ok(report.as_bytes().to_vec()),
                    Ok(None) => Err(FileError::NotFound(path.to_path_buf())),
                    Err(e) => Err(FileError::Other(Some(EcoString::from(e.message)))),
                }
            } else {
                Err(FileError::NotFound(path.to_path_buf()))
            }
        // /img/patient/{hn}
        } else if path.starts_with(strip_prefix_slash(PATH_PREFIX_PATIENT_IMAGE)) {
            // dbg!(path.to_str().unwrap_or_default());
            let paths = path
                .strip_prefix(strip_prefix_slash(PATH_PREFIX_PATIENT_IMAGE))
                .unwrap_or(path)
                .iter()
                .map(|s| s.to_str().unwrap_or_default())
                .collect::<Vec<&str>>();
            if paths.len() == 1 {
                // dbg!(&paths[0]);
                match kphis_api_query::image::patient::get_patient_image(paths[0], &app.db_pool, &app.hosxp()).await {
                    Ok(Some(patient_image)) => Ok(patient_image),
                    Ok(None) => Ok(DEFAULT_SVG_USER.as_bytes().to_vec()),
                    Err(e) => Err(FileError::Other(Some(EcoString::from(e.message)))),
                }
            } else {
                Err(FileError::NotFound(path.to_path_buf()))
            }
        // /api/xxx
        } else if let Some(endpoint) = EndPoint::VARIANTS.iter().find(|ep| path.starts_with(strip_prefix_slash(&ep.base()))) {
            // dbg!(path.to_str().unwrap_or_default());
            let paths = path
                .strip_prefix(strip_prefix_slash(&endpoint.base()))
                .unwrap_or(path)
                .iter()
                .map(|s| s.to_str().unwrap_or_default())
                .collect::<Vec<&str>>();
            let params = paths_params
                .next()
                .map(|q| {
                    q.split('&')
                        .filter_map(|s| {
                            let kv = s.split('=').collect::<Vec<&str>>();
                            (kv.len() == 2).then(|| (kv[0].to_owned(), kv[1].to_owned()))
                        })
                        .collect::<Vec<(String, String)>>()
                })
                .unwrap_or_default();
            get_json_inner(endpoint, &paths, &params, app, user)
                .await
                .map(|s| s.as_bytes().to_vec())
                .map_err(|e| FileError::Other(Some(EcoString::from(e.string()))))
        } else {
            Err(FileError::NotFound(path.to_path_buf()))
        }
    } else {
        Err(FileError::NotFound(path.to_path_buf()))
    }
}

pub async fn get_json_inner(endpoint: &EndPoint, paths: &[&str], params: &[(String, String)], app: &ApiState, user: &UserState) -> Result<String, AppError> {
    // dbg!(endpoint);
    // dbg!(paths);
    // dbg!(params);
    let invalid = Source::App.to_error(500, "Invalid path or query data", "Get json data");
    let forbidden = AppError::app_403("Get json data");
    match endpoint {
        EndPoint::AvatarOpdEr => kphis_api_query::avatar::get_avatar_opd_er(&app.db_pool, &app.hosxp(), &app.kphis())
            .await
            .map(|s| serde_json::json!(s).to_string()),
        EndPoint::AvatarIpd => {
            if let Some(query) = kphis_model::avatar::AvatarParams::from_tuples(params) {
                kphis_api_query::avatar::get_avatar_ipd(&query, app.hosxp_hn_len(), app.hosxp_an_len(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::DrugUseDuration => Err(forbidden),
        EndPoint::ExistsKeyId => {
            if paths.len() == 2 {
                kphis_api_query::app::get_exists(paths[0], paths[1], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| s.to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::EmrDateHn => {
            if paths.len() == 1 {
                kphis_api_query::emr::get_emr_date(paths[0], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::EmrVisitVn => {
            if paths.len() == 1 {
                kphis_api_query::emr::get_emr_visit(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::HisIptDiagAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::his::get_ipt_diag(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::HisIptOprtAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::his::get_ipt_oprt(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::HisOperationAdmitAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::his::get_operation_admit(paths[0], &app.operation_success(), &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::HisMedPlanIpdAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::his::get_medplan_ipd_remains(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::HisReferOutVnan => {
            if paths.len() == 1 {
                kphis_api_query::refer_out::select_his_referout_data(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::Image => Err(forbidden),
        EndPoint::ImageUsage => Err(forbidden),
        EndPoint::ImageUsageId => {
            if paths.len() == 2 {
                kphis_api_query::image::file_path::get_image_usage_id(
                    paths[0].parse::<u32>().unwrap_or_default(),
                    paths[1].parse::<u32>().unwrap_or_default(),
                    &app.db_pool,
                    &app.hosxp(),
                    &app.kphis_extra(),
                )
                .await
                .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdAdmissionNoteDr => Err(forbidden),
        EndPoint::IpdAdmissionNoteDrAn => {
            if paths.len() == 1 {
                if paths[0].len() > app.hosxp_an_len() {
                    kphis_api_query::ipd::admission_note_dr::get_ipd_admission_note_dr_from_vn(paths[0], &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    kphis_api_query::ipd::admission_note_dr::get_ipd_admission_note_dr_from_an(paths[0], &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdAdmissionNoteNurse => Err(forbidden),
        EndPoint::IpdAdmissionNoteNurseAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::admission_note_nurse::get_ipd_admission_note_nurse(paths[0], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdAdmissionNoteDrPharmCheckAn => Err(forbidden),
        EndPoint::IpdConsult => {
            if let Some(query) = kphis_model::ipd::consult::IpdConsultListParams::from_tuples(params) {
                kphis_api_query::ipd::consult::get_ipd_consult_list(query, app.hosxp_hn_len(), app.hosxp_an_len(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdConsultAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::consult::get_ipd_consult_by_an(paths[0], &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdConsultId => {
            if paths.len() == 1 {
                kphis_api_query::ipd::consult::get_ipd_consult_by_id(paths[0].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDcPlanAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::dc_plan::get_dc_plan(paths[0], &app.db_pool, &app.hosxp(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDcPlanTmpDx => {
            if let Some(query) = kphis_model::ipd::dc_plan_tmp::DcPlanTmpParams::from_tuples(params) {
                kphis_api_query::ipd::dc_plan_tmp::get_dx(&query, &app.db_pool, &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDcPlanTmpMed => {
            if let Some(query) = kphis_model::ipd::dc_plan_tmp::DcPlanTmpParams::from_tuples(params) {
                kphis_api_query::ipd::dc_plan_tmp::get_med(&query, &app.db_pool, &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDcPlanTmpEnv => {
            if let Some(query) = kphis_model::ipd::dc_plan_tmp::DcPlanTmpParams::from_tuples(params) {
                kphis_api_query::ipd::dc_plan_tmp::get_env(&query, &app.db_pool, &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDcPlanTmpTx => {
            if let Some(query) = kphis_model::ipd::dc_plan_tmp::DcPlanTmpParams::from_tuples(params) {
                kphis_api_query::ipd::dc_plan_tmp::get_tx(&query, &app.db_pool, &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDcPlanTmpDiet => {
            if let Some(query) = kphis_model::ipd::dc_plan_tmp::DcPlanTmpParams::from_tuples(params) {
                kphis_api_query::ipd::dc_plan_tmp::get_diet(&query, &app.db_pool, &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDoctorInCharge => {
            if let Some(an) = find_qs(params, "an") {
                kphis_api_query::ipd::doctor_in_charge::get_doctor_in_charge(&an, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDocumentDatetimeAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::document::get_ipd_document_datetime(paths[0], &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDocumentListVnAn => {
            if paths.len() == 2 {
                kphis_api_query::ipd::document::get_ipd_document_list(paths[0], paths[1], &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdDocumentScanAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::document::get_ipd_document_types(paths[0], &app.db_pool, &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdFocusListAn => {
            if paths.len() == 1 {
                let query = kphis_model::focus_list::FocusListParams::from_tuples(params).unwrap_or_default();
                kphis_api_query::ipd::focus_list::get_focus_list(paths[0], &query, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdFocusNoteAn => {
            if paths.len() == 1 {
                let query = kphis_model::focus_note::FocusNoteParams::from_tuples(params).unwrap_or_default();
                kphis_api_query::ipd::focus_note::get_focus_note(paths[0], &query, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdIndexActionId => Err(forbidden),
        EndPoint::IpdIndexAction => Err(forbidden),
        EndPoint::IpdIndexMedPayAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::index_plan::get_index_med_pay(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdIndexMonitorId => Err(forbidden),
        EndPoint::IpdIndexMonitor => Err(forbidden),
        EndPoint::IpdIndexNoteId => Err(forbidden),
        EndPoint::IpdIndexNote => {
            if let Some(query) = kphis_model::ipd::index_note::IndexNoteParams::from_tuples(params) {
                kphis_api_query::ipd::index_note::get_index_note(&query, &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdIndexPlanDateAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::index_plan::get_index_plan_date(paths[0], &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdIndexPlanId => Err(forbidden),
        EndPoint::IpdIndexPlan => Err(forbidden), //{
        //     if let Some(query) = index_plan::IndexPlanParams::from_tuples(params) {
        //         kphis_api_query::ipd::index_plan::get_index_plan_plus(&query, app.hosxp_hn_len(), app.hosxp_an_len(), &app.db_pool, &app.hosxp(), &app.kphis())
        //             .await
        //             .map(|s| serde_json::json!(s).to_string())
        //     } else {
        //         Err(invalid)
        //     }
        // }
        EndPoint::IpdIo => {
            if let Some(query) = kphis_model::ipd::io::IoParams::from_tuples(params) {
                if query.an.is_some() {
                    kphis_api_query::ipd::io::get_io_shift(
                        &query,
                        app.app_config.shift_day_start,
                        app.app_config.shift_evening_start,
                        app.app_config.shift_night_start,
                        &app.db_pool,
                        &app.hosxp(),
                        &app.kphis(),
                    )
                    .await
                    .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdIoDateAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::io::get_io_date(paths[0], &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdMedReconcile => {
            if let Some(query) = kphis_model::med_reconcile::MedReconciliationParams::from_tuples(params) {
                if query.an.is_some() || query.med_reconciliation_id.is_some() || query.hn.is_some() {
                    kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile(&query, &None, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdMedReconcileHosxpAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile_hosxp(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdMedReconcileLastDoseAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile_last_dose(paths[0], &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdMedReconcileNoteId => {
            if paths.len() == 1 {
                kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile_note(paths[0].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdMedReconcileRemedVisitHn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile_remed_visit(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdMra => {
            if let Some(query) = kphis_model::ipd::mra::MraParams::from_tuples(params) {
                if query.an.is_some() {
                    kphis_api_query::ipd::mra::get_ipd_mra(&query, &app.db_pool, &app.kphis_extra())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdMedReconcileRemedMed => {
            if let Some(query) = kphis_model::med_reconcile::MedReconciliationParams::from_tuples(params) {
                if query.vn.is_some() || query.an.is_some() {
                    kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile_remed_med(&query, &app.db_pool, &app.hosxp())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderItem => {
            if let Some(query) = kphis_model::order::OrderParams::from_tuples(params) {
                if query.an.is_some() && query.view_by.is_some() {
                    kphis_api_handler::ipd::order::get_ipd_order_item_bundle(&query, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderPrevious => {
            if let Some(query) = kphis_model::order::OrderParams::from_tuples(params) {
                kphis_api_handler::ipd::order::get_ipd_order_previous_bundle(&query, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderOnedayPreviousAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::order::get_previous_one_day_order(paths[0], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderProgressPrevious => {
            if let (Some(an), Some(progress_note_owner_type)) = (find_qs(params, "an"), find_qs(params, "progress_note_owner_type")) {
                kphis_api_query::ipd::progress_note::get_previous_progress(&an, &progress_note_owner_type, &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderToHomeMedAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::order::get_home_med_from_cont(paths[0], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderOrderDateAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::order::get_order_date(paths[0], &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderOrderId => Err(forbidden),
        EndPoint::IpdOrderOrder => {
            if let Some(query) = kphis_model::order::OrderParams::from_tuples(params) {
                if query.an.is_some() && query.view_by.is_some() {
                    kphis_api_handler::ipd::order::get_ipd_order_bundle(
                        &query,
                        &user.user.doctorcode,
                        &app.app_config.doctor_intern_roles,
                        &app.db_pool,
                        &app.hosxp(),
                        &app.kphis(),
                        &app.kphis_extra(),
                    )
                    .await
                    .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderProgressNoteId => Err(forbidden),
        EndPoint::IpdOrderProgressNote => {
            if let Some(query) = kphis_model::progress_note::ProgressNoteParams::from_tuples(params) {
                kphis_api_handler::ipd::progress_note::get_ipd_progress_note_bundle(&query, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdOrderPharmacy => {
            if let Some(query) = kphis_model::ipd::pharmacy_monitor::IpdOrderPharmacyParams::from_tuples(params) {
                kphis_api_query::ipd::order::get_pharmacy_order(&query, app.hosxp_hn_len(), app.hosxp_an_len(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdPasscode => Err(forbidden),
        EndPoint::IpdPostAdmitList => Err(forbidden),
        EndPoint::IpdPostAdmitCount => Err(forbidden),
        EndPoint::IpdPreAdmit => Err(forbidden),
        EndPoint::IpdPreOrderMasterId => Err(forbidden),
        EndPoint::IpdPreOrderMaster => {
            if let Some(query) = kphis_model::pre_order::master::PreOrderMasterParams::from_tuples(params) {
                kphis_api_query::pre_order::master::get_pre_order_list(&query, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdPreOrderInto => Err(forbidden),
        EndPoint::IpdPreOrderOrderId => Err(forbidden),
        EndPoint::IpdPreOrderOrder => {
            if let Some(query) = kphis_model::pre_order::order::PreOrderParams::from_tuples(params) {
                kphis_api_handler::pre_order::get_ipd_pre_order_bundle(&query, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdPreOrderProgressNoteId => Err(forbidden),
        EndPoint::IpdPreOrderProgressNote => {
            if let Some(query) = kphis_model::pre_order::progress_note::PreProgressNoteParams::from_tuples(params) {
                kphis_api_handler::pre_order::get_ipd_pre_progress_note_bundle(&query, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdShowPatientMainAn => {
            if paths.len() == 1 {
                kphis_api_query::ipd::show_patient_main::get_show_patient_main(paths[0], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdSummary => {
            if let Some(query) = kphis_model::ipd::summary::SummaryParams::from_tuples(params) {
                if query.an.is_some() || query.summary_id.is_some() {
                    kphis_api_query::ipd::summary::get_ipd_summary(
                        &query,
                        &user.user.doctorcode,
                        &user.user.groupname,
                        &app.app_config.lab_alerts,
                        &app.operation_success(),
                        &app.db_pool,
                        &app.hosxp(),
                        &app.kphis(),
                    )
                    .await
                    .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdSummaryAudit => {
            if let Some(query) = kphis_model::ipd::summary_audit::SummaryAuditParams::from_tuples(params) {
                if query.an.is_some() {
                    kphis_api_query::ipd::summary_audit::get_ipd_summary_audit(&query, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdSummaryNoteId => Err(forbidden),
        EndPoint::IpdSummaryStatusId => Err(forbidden),
        EndPoint::IpdTmpGroup => {
            if let Some(query) = kphis_model::ipd::tmp::TmpParams::from_tuples(params) {
                kphis_api_query::ipd::tmp::get_group(&query, &app.db_pool, &app.kphis()).await.map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdTmpSubgroup => {
            if let Some(query) = kphis_model::ipd::tmp::TmpParams::from_tuples(params) {
                kphis_api_query::ipd::tmp::get_subgroup(&query, &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdTmpFocus => {
            if let Some(query) = kphis_model::ipd::tmp::TmpParams::from_tuples(params) {
                kphis_api_query::ipd::tmp::get_focus(&query, &app.db_pool, &app.kphis()).await.map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdTmpGoal => {
            if let Some(query) = kphis_model::ipd::tmp::TmpParams::from_tuples(params) {
                kphis_api_query::ipd::tmp::get_goal(&query, &app.db_pool, &app.kphis()).await.map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdTmpIntvt => {
            if let Some(query) = kphis_model::ipd::tmp::TmpParams::from_tuples(params) {
                kphis_api_query::ipd::tmp::get_intvt(&query, &app.db_pool, &app.kphis()).await.map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdTmpDlc => {
            if let Some(query) = kphis_model::ipd::tmp::TmpParams::from_tuples(params) {
                kphis_api_query::ipd::tmp::get_dlc(&query, &app.db_pool, &app.kphis()).await.map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdVitalSign => {
            if let Some(query) = kphis_model::vital_sign::VitalSignParams::from_tuples(params) {
                kphis_api_query::ipd::vital_sign::get_vital_sign(&query, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::IpdVitalSignId => Err(forbidden),
        EndPoint::LabHead => {
            if let Some(query) = kphis_model::lab::LabHeadParams::from_tuples(params) {
                kphis_api_query::lab::get_lab_head(&query, &user.user.doctorcode, &user.user.groupname, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::LabItem => {
            if let Some(query) = kphis_model::lab::LabItemParams::from_tuples(params) {
                kphis_api_query::lab::get_lab_item(&query, &user.user.doctorcode, &user.user.groupname, &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::LabReadId => Err(forbidden),
        EndPoint::LabWbcKeyValue => {
            if paths.len() == 2 {
                kphis_api_query::lab::get_wbc_band(paths[0], paths[1], app.app_config.hosxp_lab_wbc_code, app.app_config.hosxp_lab_band_code, &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::MedReconcileHn => {
            if paths.len() == 1 {
                kphis_api_query::med_reconciliation::get_med_reconciliation_header(paths[0], app.hosxp_an_len(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErDcPlanId => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::dc_plan::get_dc_plan(paths[0].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.hosxp(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErDocumentListVnId => {
            if paths.len() == 2 {
                kphis_api_query::opd_er::document::get_opd_er_document_list(paths[0], paths[1].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErDocumentScanId => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::document::get_opd_er_document_types(paths[0].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErFocusListId => {
            if paths.len() == 1 {
                let query = kphis_model::focus_list::FocusListParams::from_tuples(params).unwrap_or_default();
                kphis_api_query::opd_er::focus_list::get_focus_list(paths[0].parse::<u32>().unwrap_or_default(), &query, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErFocusNoteId => {
            if paths.len() == 1 {
                let query = kphis_model::focus_note::FocusNoteParams::from_tuples(params).unwrap_or_default();
                kphis_api_query::opd_er::focus_note::get_focus_note(paths[0].parse::<u32>().unwrap_or_default(), &query, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErHisMedVn => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::hosxp_med::get_opd_med(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErIndexActionId => Err(forbidden),
        EndPoint::OpdErIndexAction => Err(forbidden),
        EndPoint::OpdErIndexMonitorId => Err(forbidden),
        EndPoint::OpdErIndexMonitor => Err(forbidden),
        EndPoint::OpdErIndexPlanId => Err(forbidden),
        EndPoint::OpdErIndexPlan => Err(forbidden), //{
        //     if let Some(query) = index_plan::IndexPlanParams::from_tuples(params) {
        //         kphis_api_query::opd_er::index_plan::get_index_plan_plus(&query, app.hosxp_hn_len(), app.hosxp_vn_len(), &app.db_pool, &app.hosxp(), &app.kphis())
        //             .await
        //             .map(|s| serde_json::json!(s).to_string())
        //     } else {
        //         Err(invalid)
        //     }
        // }
        EndPoint::OpdErIo => {
            if let Some(query) = kphis_model::ipd::io::IoParams::from_tuples(params) {
                if query.opd_er_order_master_id.is_some() {
                    kphis_api_query::opd_er::io::get_io_shift(
                        &query,
                        app.app_config.shift_day_start,
                        app.app_config.shift_evening_start,
                        app.app_config.shift_night_start,
                        &app.db_pool,
                        &app.hosxp(),
                        &app.kphis(),
                    )
                    .await
                    .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErIoDateId => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::io::get_io_date(paths[0].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedicalHistory => {
            if let Some(query) = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams::from_tuples(params) {
                let query = query.clean();
                if query.valid() {
                    kphis_api_query::opd_er::medical_history::get_medical_history(&query, &app.app_config.hospital_name, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedicalHistoryTrauma => {
            if let Some(query) = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams::from_tuples(params) {
                if let Some(opd_er_order_master_id) = query.opd_er_order_master_id {
                    kphis_api_query::opd_er::medical_history::get_trauma_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedicalHistoryAllergy => {
            if let Some(query) = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams::from_tuples(params) {
                if let Some(opd_er_order_master_id) = query.opd_er_order_master_id {
                    kphis_api_query::opd_er::medical_history::get_allergy_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedicalHistoryScreen => {
            if let Some(query) = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams::from_tuples(params) {
                if let Some(opd_er_order_master_id) = query.opd_er_order_master_id {
                    kphis_api_query::opd_er::medical_history::get_screen_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedicalHistoryConsult => {
            if let Some(query) = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams::from_tuples(params) {
                if let Some(opd_er_order_master_id) = query.opd_er_order_master_id {
                    kphis_api_query::opd_er::medical_history::get_consult_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedicalHistoryScan => {
            if let Some(query) = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams::from_tuples(params) {
                if let Some(opd_er_order_master_id) = query.opd_er_order_master_id {
                    kphis_api_query::opd_er::medical_history::get_scan_history(opd_er_order_master_id, &app.db_pool, &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedicalHistoryFt => {
            if let Some(query) = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams::from_tuples(params) {
                if let Some(opd_er_order_master_id) = query.opd_er_order_master_id {
                    kphis_api_query::opd_er::medical_history::get_ft_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedReconcile => {
            if let Some(query) = kphis_model::med_reconcile::MedReconciliationParams::from_tuples(params) {
                if query.opd_er_order_master_id.is_some() || query.med_reconciliation_id.is_some() || query.hn.is_some() {
                    kphis_api_query::opd_er::med_reconcile::get_opd_er_med_reconcile(&query, &None, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErMedReconcileNoteId => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::med_reconcile::get_opd_er_med_reconcile_note(paths[0].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErOrderMasterCheckVn => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::order_master::get_order_master_check(paths[0], &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErOrderMasterId => {
            if paths.len() == 1 {
                if let Ok(opd_er_order_master_id) = paths[0].parse::<u32>() {
                    kphis_api_query::opd_er::order_master::get_order_master(opd_er_order_master_id, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErOrderMaster => {
            if let Some(query) = kphis_model::opd_er::order_master::OpdErOrderMasterParams::from_tuples(params) {
                kphis_api_query::opd_er::order_master::get_order_master_list(&query, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErOrderItem => {
            if let Some(query) = kphis_model::order::OrderParams::from_tuples(params) {
                if query.opd_er_order_master_id.is_some() || query.view_by.is_some() {
                    kphis_api_handler::opd_er::order::get_opd_er_order_item_bundle(&query, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                        .await
                        .map(|s| serde_json::json!(s).to_string())
                } else {
                    Err(invalid)
                }
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErOrderOrderId => Err(forbidden),
        EndPoint::OpdErOrderOrder => {
            if let Some(query) = kphis_model::order::OrderParams::from_tuples(params) {
                kphis_api_handler::opd_er::order::get_opd_er_order_bundle(
                    &query,
                    &user.user.doctorcode,
                    &app.app_config.doctor_intern_roles,
                    &app.db_pool,
                    &app.hosxp(),
                    &app.kphis(),
                    &app.kphis_extra(),
                )
                .await
                .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErOrderProgressNoteId => Err(forbidden),
        EndPoint::OpdErOrderProgressNote => {
            if let Some(query) = kphis_model::progress_note::ProgressNoteParams::from_tuples(params) {
                kphis_api_handler::opd_er::progress_note::get_opd_er_progress_note_bundle(&query, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErOrderPharmacy => {
            if let Some(query) = kphis_model::opd_er::pharmacy_monitor::OpdErOrderPharmacyParams::from_tuples(params) {
                kphis_api_query::opd_er::order::get_pharmacy_order(&query, app.hosxp_hn_len(), app.hosxp_vn_len(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErShowPatientMainId => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::show_patient_main::get_show_patient_main_id(paths[0].parse::<u32>().unwrap_or_default(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErShowPatientMainVn => {
            if paths.len() == 1 {
                kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(paths[0], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErVitalSign => {
            if let Some(query) = kphis_model::vital_sign::VitalSignParams::from_tuples(params) {
                kphis_api_query::opd_er::vital_sign::get_vital_sign(&query, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::OpdErVitalSignId => Err(forbidden),
        EndPoint::PrescrptionScreen => {
            if let Some(query) = kphis_model::prescription::PrescriptionScreenParams::from_tuples(params) {
                kphis_api_query::prescription::get_prescription_screen(
                    query,
                    app.hosxp_hn_len(),
                    app.hosxp_vn_len(),
                    &app.egfr_codes(),
                    &app.scr_codes(),
                    &app.lab_codes(),
                    &app.message_icodes(),
                    &app.message_egfr_icodes(),
                    &app.message_crcl_icodes(),
                    &app.db_pool,
                    &app.hosxp(),
                    &app.kphis_extra(),
                )
                .await
                .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::ReferNoteVnan => {
            if paths.len() == 1 {
                kphis_api_query::refer_note::select_refernote(paths[0], &app.db_pool, &app.hosxp(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::ReportCustom => Err(forbidden),
        EndPoint::ReportRawQuery => Err(forbidden),
        EndPoint::ReportRawTemplateTypeId => Err(forbidden),
        EndPoint::ReportTemplateTypeId => Err(forbidden),
        EndPoint::ScanHisImage => {
            if let Some(query) = kphis_model::image::scan_his::ScanImageParams::from_tuples(params) {
                kphis_api_query::image::scan_his::get_scan_his_image(&query, &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxHospText => {
            if paths.len() == 1 {
                kphis_api_query::search::searchbox::get_hosp_searchbox(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxMedDuplicate => {
            if let Some(query) = kphis_model::search::searchbox::DrugCheckParams::from_tuples(params) {
                kphis_api_query::search::searchbox::get_drug_duplicate_check(&query, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxMedInteraction => {
            if let Some(query) = kphis_model::search::searchbox::DrugCheckParams::from_tuples(params) {
                kphis_api_query::search::searchbox::get_drug_interaction_check(&query, &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxMedHnText => {
            if paths.len() == 2 {
                kphis_api_query::search::searchbox::get_med_searchbox(paths[0], paths[1], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxOpdVisitModeText => {
            if paths.len() == 2 {
                kphis_api_query::search::searchbox::get_opd_visit_searchbox(paths[0], paths[1], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxIvfluidText => {
            if paths.len() == 1 {
                kphis_api_query::search::searchbox::get_ivfluid_searchbox(paths[0], &app.ivfluid(), &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxLabText => {
            if paths.len() == 1 {
                kphis_api_query::search::searchbox::get_lab_searchbox(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxPatientText => {
            if paths.len() == 1 {
                kphis_api_query::search::searchbox::get_patient_searchbox(paths[0], app.hosxp_hn_len(), &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchBoxXrayText => {
            if paths.len() == 1 {
                kphis_api_query::search::searchbox::get_xray_searchbox(paths[0], &app.db_pool, &app.hosxp())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchDr => {
            if let Some(query) = kphis_model::search::ipd_search_patient_dr::IpdSearchPatientDrRequest::from_tuples(params) {
                kphis_api_query::search::ipd_search_patient_dr::get_ipd_dr_search_patient(query, app.hosxp_hn_len(), app.hosxp_an_len(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchNurse => {
            if let Some(query) = kphis_model::search::ipd_search_patient_nurse::IpdSearchPatientNurseRequest::from_tuples(params) {
                kphis_api_query::search::ipd_search_patient_nurse::get_ipd_nurse_search_patient(
                    query,
                    app.hosxp_hn_len(),
                    app.hosxp_an_len(),
                    &app.db_pool,
                    &app.hosxp(),
                    &app.kphis(),
                    &app.kphis_extra(),
                )
                .await
                .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchPharmacist => {
            if let Some(query) = kphis_model::search::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistRequest::from_tuples(params) {
                kphis_api_query::search::ipd_search_patient_pharmacist::get_ipd_pharmacist_search_patient(
                    query,
                    app.hosxp_hn_len(),
                    app.hosxp_an_len(),
                    &app.db_pool,
                    &app.hosxp(),
                    &app.kphis(),
                    &app.kphis_extra(),
                )
                .await
                .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::SearchOther => {
            if let Some(query) = kphis_model::search::ipd_search_patient_other::IpdSearchPatientOtherRequest::from_tuples(params) {
                kphis_api_query::search::ipd_search_patient_other::get_ipd_other_search_patient(query, app.hosxp_hn_len(), app.hosxp_an_len(), &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::Sse | EndPoint::SseGroup | EndPoint::SseMessage => Err(forbidden),
        EndPoint::User | EndPoint::UserConfig => Err(forbidden),
        EndPoint::UserRolePrelude => kphis_api_query::user::role::get_user_role_prelude(&app.db_pool, &app.hosxp(), &app.kphis())
            .await
            .map(|s| serde_json::json!(s).to_string()),
        EndPoint::UserRoleRole => {
            if let Some(query) = kphis_model::user::role::UserRoleParams::from_tuples(params) {
                kphis_api_query::user::role::get_role_permission_list(query, &app.db_pool, &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::UserRoleUser => {
            if let Some(query) = kphis_model::user::role::UserRoleParams::from_tuples(params) {
                kphis_api_query::user::role::get_user_role_list(query, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::XrayReportHn => {
            if paths.len() == 1 {
                kphis_api_query::xray::get_xray_report(paths[0], &app.db_pool, &app.hosxp(), &app.kphis())
                    .await
                    .map(|s| serde_json::json!(s).to_string())
            } else {
                Err(invalid)
            }
        }
        EndPoint::XrayReadId => Err(forbidden),
        EndPoint::XrayPacsXn => Err(forbidden),
        EndPoint::Unknown => Err(forbidden),
    }
}

fn strip_prefix_slash(s: &str) -> &str {
    s.trim_start_matches('/')
}
