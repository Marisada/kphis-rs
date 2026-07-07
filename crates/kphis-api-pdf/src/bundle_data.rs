use kphis_api_core::state::{ApiState, UserState};
use kphis_api_query::report::select_raw_query_to_json_string;
use kphis_model::report::{CustomReport, SystemReport, TypstReport, params_and_ids_to_json};
use kphis_util::{
    datetime::{JsTime, datetime_from_opt},
    error::AppError,
    util::{str_some, zero_none},
};

/// PDF data list for Report Builder
/// - if coercion does't has custom_report, this will use system_report instead
pub async fn pdf_data(report: &TypstReport, ids: &str, app: &ApiState, user: &UserState) -> Result<String, AppError> {
    let data_json = match report {
        TypstReport::System(system) => system_pdf_data(system, ids, app, user).await?,
        TypstReport::Coercion((_, system, custom_opt)) => {
            if let Some(custom) = custom_opt {
                if let (Some(statement), Some(statement_params)) = (&custom.statement, &custom.statement_params) {
                    // has Query
                    select_raw_query_to_json_string(
                        statement,
                        statement_params,
                        ids,
                        &app.app_asset_cache,
                        &app.db_pool,
                        &app.hosxp(),
                        &app.kphis(),
                        &app.kphis_extra(),
                        &app.kphis_log(),
                    )
                    .await?
                } else {
                    // without Query, default to system
                    system_pdf_data(system, ids, app, user).await?
                }
            } else {
                system_pdf_data(system, ids, app, user).await?
            }
        }
        TypstReport::Custom(custom) => custom_pdf_data(custom, ids, app).await?,
    };

    Ok(data_json)
}

async fn system_pdf_data(system: &SystemReport, id: &str, app: &ApiState, user: &UserState) -> Result<String, AppError> {
    match system {
        SystemReport::DocumentImages => document_images(id, app).await,
        SystemReport::IpdAdmissionNoteDr => ipd_admission_note_dr(id, app).await,
        SystemReport::IpdAdmissionNoteNurse => ipd_admission_note_nurse(id, app).await,
        SystemReport::IpdConsult => ipd_consult(id, app).await,
        SystemReport::IpdDischargePlan => ipd_discharge_plan(id, app).await,
        SystemReport::IpdDocument => ipd_document(id, app).await,
        SystemReport::IpdEventLog => ipd_event_log(id, app, user).await,
        SystemReport::IpdFocusList => ipd_focus_list(id, app).await,
        SystemReport::IpdFocusNote => ipd_focus_note(id, app).await,
        SystemReport::IpdIndexPlan => ipd_index_plan(id, app).await,
        SystemReport::IpdIo => ipd_io(id, app).await,
        SystemReport::IpdMAR => ipd_mar(id, app).await,
        SystemReport::IpdMRA => ipd_mra(id, app).await,
        SystemReport::IpdMedReconciliation => ipd_med_reconciliation(id, app).await,
        SystemReport::IpdMedReconciliationHosXp => ipd_med_reconciliation_hosxp(id, app).await,
        SystemReport::IpdOrder => ipd_order(id, &user.user.doctorcode, app).await,
        SystemReport::IpdPartograph | SystemReport::IpdPartographWho => ipd_vital_sign(id, app).await,
        SystemReport::IpdSummary => ipd_summary_note(id, app).await,
        SystemReport::IpdSummaryAudit => ipd_summary_audit(id, app).await,
        SystemReport::IpdTPR => ipd_tpr_chart(id, app).await,
        SystemReport::IpdVitalSignGeneral | SystemReport::IpdVitalSignNeuro | SystemReport::IpdVitalSignLabour | SystemReport::IpdVitalSignPsychia => ipd_vital_sign(id, app).await,
        SystemReport::Lab => lab(true, id, app, user).await,
        SystemReport::LabSummary => lab(false, id, app, user).await,
        SystemReport::OpdErDischargePlan => opd_er_discharge_plan(id, app).await,
        SystemReport::OpdErDocument => opd_er_document(id, app).await,
        SystemReport::OpdErEventLog => opd_er_event_log(id, app, user).await,
        SystemReport::OpdErFocusList => opd_er_focus_list(id, app).await,
        SystemReport::OpdErFocusNote => opd_er_focus_note(id, app).await,
        SystemReport::OpdErIndexPlan => opd_er_index_plan(id, app).await,
        SystemReport::OpdErIo => opd_er_io(id, app).await,
        SystemReport::OpdErMedicalHistory => opd_er_medical_history(id, app).await,
        SystemReport::OpdErMedReconciliation => opd_er_med_reconciliation(id, app).await,
        SystemReport::OpdErOrder => opd_er_order(id, &user.user.doctorcode, app).await,
        SystemReport::OpdErVitalSignGeneral | SystemReport::OpdErVitalSignNeuro | SystemReport::OpdErVitalSignLabour | SystemReport::OpdErVitalSignPsychia => opd_er_vital_sign(id, app).await,
        SystemReport::ReferNote => refer_note(id, app).await,
        SystemReport::ReferOut => refer_out(id, app).await,
        SystemReport::ScanImages => scan_images(id, app).await,
    }
}

async fn custom_pdf_data(custom: &CustomReport, ids: &str, app: &ApiState) -> Result<String, AppError> {
    let result = if let (Some(statement), Some(statement_params)) = (&custom.statement, &custom.statement_params) {
        // has Query
        select_raw_query_to_json_string(
            statement,
            statement_params,
            ids,
            &app.app_asset_cache,
            &app.db_pool,
            &app.hosxp(),
            &app.kphis(),
            &app.kphis_extra(),
            &app.kphis_log(),
        )
        .await?
    } else if let Some(statement_params) = &custom.statement_params {
        // without Query
        params_and_ids_to_json(statement_params, ids)
    } else {
        ["{\"id\",\"", ids, "\"}"].concat()
    };

    Ok(result)
}

//===== ===== =====
//  DATA FUNCTIONS
//===== ===== =====

async fn document_images(vnan_id_perpage: &str, app: &ApiState) -> Result<String, AppError> {
    let triplets = vnan_id_perpage.split('|').collect::<Vec<&str>>();
    let len = triplets.len();
    if len != 3 || (len == 3 && triplets[0].is_empty()) {
        Err(AppError::app_400("Get DocumentImages"))
    } else {
        let vnan = triplets[0];
        let doc_type_id = triplets[1].parse::<u8>().unwrap_or(1);
        let per_page = triplets[2].parse::<u8>().unwrap_or(1);
        let data = if vnan.len() == app.hosxp_an_len() {
            let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
            let key_id_opt = kphis_api_query::ipd::document::get_ipd_document_types(&vnan, &app.db_pool, &app.kphis_extra())
                .await?
                .iter()
                .find(|doc| doc.document_type_id == kphis_model::image::file_path::DocumentType::new_from_u8(doc_type_id))
                .map(|d| d.document_id);
            let im_paths = if let Some(key_id) = key_id_opt {
                kphis_api_query::image::file_path::get_image_usage_id(11, key_id, &app.db_pool, &app.hosxp(), &app.kphis_extra()).await?
            } else {
                Vec::new()
            };
            serde_json::json!({
                "id": vnan,
                "doc_type_id": doc_type_id,
                "per_page": per_page,
                "patient": patient,
                "im_paths": im_paths,
            })
            .to_string()
        } else {
            let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
            let key_id_opt = if let Some(pt) = patient.as_ref() {
                if let Some(opd_er_order_master_id) = pt.opd_er_order_master_id {
                    kphis_api_query::opd_er::document::get_opd_er_document_types(opd_er_order_master_id, &app.db_pool, &app.kphis_extra())
                        .await?
                        .iter()
                        .find(|doc| doc.document_type_id == kphis_model::image::file_path::DocumentType::new_from_u8(doc_type_id))
                        .map(|d| d.document_id)
                } else {
                    None
                }
            } else {
                None
            };
            let im_paths = if let Some(key_id) = key_id_opt {
                kphis_api_query::image::file_path::get_image_usage_id(12, key_id, &app.db_pool, &app.hosxp(), &app.kphis_extra()).await?
            } else {
                Vec::new()
            };

            serde_json::json!({
                "id": vnan,
                "doc_type_id": doc_type_id,
                "per_page": per_page,
                "patient": patient,
                "im_paths": im_paths,
            })
            .to_string()
        };

        Ok(data)
    }
}

async fn ipd_admission_note_dr(an: &str, app: &ApiState) -> Result<String, AppError> {
    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let raw = kphis_api_query::ipd::admission_note_dr::get_ipd_admission_note_dr_from_an(an, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;
        let image = patient.as_ref().map(|pt| pt.image());

        serde_json::json!({
            "id": an,
            "patient": patient,
            "raw": raw,
            "patient_image": image,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let raw = kphis_api_query::ipd::admission_note_dr::get_ipd_admission_note_dr_from_vn(an, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;
        let image = patient.as_ref().map(|pt| pt.image());

        serde_json::json!({
            "id": an,
            "patient": patient,
            "raw": raw,
            "patient_image": image,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_admission_note_nurse(an: &str, app: &ApiState) -> Result<String, AppError> {
    let note = kphis_api_query::ipd::admission_note_nurse::get_ipd_admission_note_nurse(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "note": note,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "note": note,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_consult(an: &str, app: &ApiState) -> Result<String, AppError> {
    let consults = kphis_api_query::ipd::consult::get_ipd_consult_by_an(an, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;
    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "consults": consults,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "consults": consults,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_discharge_plan(an: &str, app: &ApiState) -> Result<String, AppError> {
    let dc_plans = kphis_api_query::ipd::dc_plan::get_dc_plan(an, &app.db_pool, &app.hosxp(), &app.kphis_extra()).await?;
    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "dc_plans": dc_plans,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "dc_plans": dc_plans,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_document(an: &str, app: &ApiState) -> Result<String, AppError> {
    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let count = kphis_api_query::ipd::document::get_ipd_document_list(
            &patient.as_ref().and_then(|pt| pt.vn.clone()).unwrap_or_default(),
            an,
            &app.db_pool,
            &app.hosxp(),
            &app.kphis(),
            &app.kphis_extra(),
        )
        .await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "count": count,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let count = kphis_api_query::ipd::document::get_ipd_document_list(
            &patient.as_ref().and_then(|pt| pt.vn.clone()).unwrap_or_default(),
            an,
            &app.db_pool,
            &app.hosxp(),
            &app.kphis(),
            &app.kphis_extra(),
        )
        .await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "count": count,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_event_log(an: &str, app: &ApiState, user: &UserState) -> Result<String, AppError> {
    let order_params = kphis_model::order::OrderParams {
        an: str_some(an.to_owned()),
        view_by: Some(String::from("doctor")),
        ..Default::default()
    };
    let order = kphis_api_handler::ipd::order::get_ipd_order_bundle(
        &order_params,
        &user.user.doctorcode,
        &app.app_config.doctor_intern_roles,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
        &app.kphis_extra(),
    )
    .await?;

    let note_params = kphis_model::progress_note::ProgressNoteParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let note =
        kphis_api_handler::ipd::progress_note::get_ipd_progress_note_bundle(&note_params, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;

    let consults = kphis_api_query::ipd::consult::get_ipd_consult_by_an(an, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;

    let vs_params = kphis_model::vital_sign::VitalSignParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let vs = kphis_api_query::ipd::vital_sign::get_vital_sign(&vs_params, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

    let io_params = kphis_model::ipd::io::IoParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let io = kphis_api_query::ipd::io::get_io_shift(
        &io_params,
        app.app_config.shift_day_start,
        app.app_config.shift_evening_start,
        app.app_config.shift_night_start,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
    )
    .await?;

    let lab = kphis_api_query::lab::get_lab_head(
        &kphis_model::lab::LabHeadParams {
            vn: Some(an.to_owned()),
            ..Default::default()
        },
        &user.user.doctorcode,
        &user.user.groupname,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
    )
    .await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "order": order,
            "note": note,
            "consult": consults,
            "vs": vs,
            "io": io,
            "lab": lab,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "order": order,
            "note": note,
            "consult": consults,
            "vs": vs,
            "io": io,
            "lab": lab,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_focus_list(an: &str, app: &ApiState) -> Result<String, AppError> {
    let params = kphis_model::focus_list::FocusListParams::default();
    let focus = kphis_api_query::ipd::focus_list::get_focus_list(an, &params, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "focus": focus,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "focus": focus,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_focus_note(an: &str, app: &ApiState) -> Result<String, AppError> {
    let params = kphis_model::focus_note::FocusNoteParams::default();
    let note = kphis_api_query::ipd::focus_note::get_focus_note(an, &params, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "note": note,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "note": note,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_index_plan(an: &str, app: &ApiState) -> Result<String, AppError> {
    let order_params = kphis_model::order::OrderParams {
        an: str_some(an.to_owned()),
        view_by: Some(String::from("doctor")),
        ..Default::default()
    };
    let order_item = kphis_api_handler::ipd::order::get_ipd_order_item_bundle(&order_params, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "order_item": order_item,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "order_item": order_item,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_io(an: &str, app: &ApiState) -> Result<String, AppError> {
    let params = kphis_model::ipd::io::IoParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let io = kphis_api_query::ipd::io::get_io_shift(
        &params,
        app.app_config.shift_day_start,
        app.app_config.shift_evening_start,
        app.app_config.shift_night_start,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
    )
    .await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "io": io,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "io": io,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_mar(an: &str, app: &ApiState) -> Result<String, AppError> {
    let order_params = kphis_model::order::OrderParams {
        an: str_some(an.to_owned()),
        view_by: Some(String::from("doctor")),
        ..Default::default()
    };
    let order_item = kphis_api_handler::ipd::order::get_ipd_order_item_bundle(&order_params, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;
    let med_pay = kphis_api_query::ipd::index_plan::get_index_med_pay(an, &app.db_pool, &app.hosxp()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "order_item": order_item,
            "med_pay": med_pay,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "order_item": order_item,
            "med_pay": med_pay,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_mra(an: &str, app: &ApiState) -> Result<String, AppError> {
    let mra_params = kphis_model::ipd::mra::MraParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let mra_item = kphis_api_query::ipd::mra::get_ipd_mra(&mra_params, &app.db_pool, &app.kphis_extra()).await?;

    let data = serde_json::json!({
        "id": an,
        "mra": mra_item,
    })
    .to_string();

    Ok(data)
}

async fn ipd_med_reconciliation(an: &str, app: &ApiState) -> Result<String, AppError> {
    let params = kphis_model::med_reconcile::MedReconciliationParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let recon = kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile(&params, &None, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "recon": recon,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "recon": recon,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_med_reconciliation_hosxp(an: &str, app: &ApiState) -> Result<String, AppError> {
    let recon = kphis_api_query::ipd::med_reconcile::get_ipd_med_reconcile_hosxp(an, &app.db_pool, &app.hosxp()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "recon": recon,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "recon": recon,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_order(an: &str, doctorcode: &Option<String>, app: &ApiState) -> Result<String, AppError> {
    let oneday_params = kphis_model::order::OrderParams {
        an: str_some(an.to_owned()),
        order_type: Some(String::from("oneday")),
        view_by: Some(String::from("doctor")),
        ..Default::default()
    };
    let oneday = kphis_api_handler::ipd::order::get_ipd_order_bundle(
        &oneday_params,
        doctorcode,
        &app.app_config.doctor_intern_roles,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
        &app.kphis_extra(),
    )
    .await?;
    let cont_params = kphis_model::order::OrderParams {
        an: str_some(an.to_owned()),
        order_type: Some(String::from("continuous")),
        view_by: Some(String::from("doctor")),
        ..Default::default()
    };
    let cont = kphis_api_handler::ipd::order::get_ipd_order_bundle(
        &cont_params,
        doctorcode,
        &app.app_config.doctor_intern_roles,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
        &app.kphis_extra(),
    )
    .await?;
    let note_params = kphis_model::progress_note::ProgressNoteParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let note =
        kphis_api_handler::ipd::progress_note::get_ipd_progress_note_bundle(&note_params, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;
    let doctor = kphis_api_query::ipd::doctor_in_charge::get_doctor_in_charge(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "oneday": oneday,
            "cont": cont,
            "note": note,
            "doctor": doctor,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "oneday": oneday,
            "cont": cont,
            "note": note,
            "doctor": doctor,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_summary_note(an: &str, app: &ApiState) -> Result<String, AppError> {
    let params = kphis_model::ipd::summary::SummaryParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let summary = kphis_api_query::ipd::summary::get_summary_data(&params, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let (dx_data, doctor_data) = kphis_api_query::ipd::summary::get_dx_and_doctor_data(&summary.as_ref().map(|s| s.summary_id), &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    // let or_data = kphis_api_query::ipd::his::get_operation_admit(an, &app.operation_success(), &app.db_pool, &app.hosxp()).await?;
    // let x34_data = kphis_api_query::ipd::summary::select_xray_with_groups(an, &[3, 4], &app.db_pool, &app.hosxp()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "summary": summary,
            "dx": dx_data,
            // "op": or_data,
            // "x34": x34_data,
            "doctor": doctor_data,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "summary": summary,
            "dx": dx_data,
            // "op": or_data,
            // "x34": x34_data,
            "doctor": doctor_data,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_summary_audit(an: &str, app: &ApiState) -> Result<String, AppError> {
    let params = kphis_model::ipd::summary_audit::SummaryAuditParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let audit = kphis_api_query::ipd::summary_audit::get_ipd_summary_audit(&params, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;

    let data = serde_json::json!({
        "id": an,
        "audit": audit,
    })
    .to_string();

    Ok(data)
}

async fn ipd_tpr_chart(an: &str, app: &ApiState) -> Result<String, AppError> {
    // vital_sign
    let vs_params = kphis_model::vital_sign::VitalSignParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let vs = kphis_api_query::ipd::vital_sign::get_vital_sign(&vs_params, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    // io
    let io_params = kphis_model::ipd::io::IoParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let io = kphis_api_query::ipd::io::get_io_shift(
        &io_params,
        app.app_config.shift_day_start,
        app.app_config.shift_evening_start,
        app.app_config.shift_night_start,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
    )
    .await?;
    // operations
    let op = kphis_api_query::ipd::his::get_operation_admit(an, &app.operation_success(), &app.db_pool, &app.hosxp()).await?;
    // doctor_in_charge
    let doctor = kphis_api_query::ipd::doctor_in_charge::get_doctor_in_charge(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let order_date = patient
            .as_ref()
            .and_then(|pt| pt.dchdate)
            .or(vs.iter().map(|vs| vs.vs_datetime.date()).max())
            .or(io.iter().filter_map(|io| io.shift_date).max());
        // food only contunuous order
        let diet = if order_date.is_some() {
            let order_params = kphis_model::order::OrderParams {
                an: str_some(an.to_owned()),
                current_date: order_date,
                order_item_types: Some(String::from("food")),
                with_offed: Some(String::from("Y")),
                ..Default::default()
            };
            kphis_api_query::ipd::order::get_previous_order(&order_params, &app.db_pool, &app.hosxp(), &app.kphis()).await?
        } else {
            Vec::new()
        };
        serde_json::json!({
            "id": an,
            "patient": patient,
            "vs": vs,
            "io": io,
            "op": op,
            "diet": diet,
            "doctor": doctor,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let last_date = vs.iter().map(|vs| vs.vs_datetime.date()).max().or(io.iter().filter_map(|io| io.shift_date).max());
        // food only contunuous order
        let diet = if last_date.is_some() {
            let order_params = kphis_model::order::OrderParams {
                an: str_some(an.to_owned()),
                current_date: last_date,
                order_item_types: Some(String::from("food")),
                with_offed: Some(String::from("Y")),
                ..Default::default()
            };
            kphis_api_query::ipd::order::get_previous_order(&order_params, &app.db_pool, &app.hosxp(), &app.kphis()).await?
        } else {
            Vec::new()
        };
        serde_json::json!({
            "id": an,
            "patient": patient,
            "vs": vs,
            "io": io,
            "op": op,
            "diet": diet,
            "doctor": doctor,
        })
        .to_string()
    };

    Ok(data)
}

async fn ipd_vital_sign(an: &str, app: &ApiState) -> Result<String, AppError> {
    let params = kphis_model::vital_sign::VitalSignParams {
        an: str_some(an.to_owned()),
        ..Default::default()
    };
    let vs = kphis_api_query::ipd::vital_sign::get_vital_sign(&params, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

    let data = if an.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "vs": vs,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(an, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": an,
            "patient": patient,
            "vs": vs,
        })
        .to_string()
    };

    Ok(data)
}

// vn can be 'vn' or 'an'
async fn lab(with_scan: bool, vnan: &str, app: &ApiState, user: &UserState) -> Result<String, AppError> {
    let lab = kphis_api_query::lab::get_lab_head(
        &kphis_model::lab::LabHeadParams {
            vn: Some(vnan.to_owned()),
            with_scan: Some(with_scan),
            ..Default::default()
        },
        &user.user.doctorcode,
        &user.user.groupname,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
    )
    .await?;

    let data = if vnan.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": vnan,
            "patient": patient,
            "lab": lab,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": vnan,
            "patient": patient,
            "lab": lab,
        })
        .to_string()
    };

    Ok(data)
}

async fn opd_er_discharge_plan(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let dc_plans = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        Some(kphis_api_query::opd_er::dc_plan::get_dc_plan(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis_extra()).await?)
    } else {
        None
    };
    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "dc_plans": dc_plans,
    })
    .to_string();

    Ok(data)
}

async fn opd_er_document(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let count = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        Some(kphis_api_query::opd_er::document::get_opd_er_document_list(vn, opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?)
    } else {
        None
    };

    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "count": count,
    })
    .to_string();

    Ok(data)
}

async fn opd_er_event_log(vn: &str, app: &ApiState, user: &UserState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

    let lab = kphis_api_query::lab::get_lab_head(
        &kphis_model::lab::LabHeadParams {
            vn: Some(vn.to_owned()),
            ..Default::default()
        },
        &user.user.doctorcode,
        &user.user.groupname,
        &app.db_pool,
        &app.hosxp(),
        &app.kphis(),
    )
    .await?;

    let data = if let Some(id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let opd_er_order_master_id = Some(id);

        let order_params = kphis_model::order::OrderParams {
            opd_er_order_master_id,
            view_by: Some(String::from("doctor")),
            ..Default::default()
        };
        let order = kphis_api_handler::opd_er::order::get_opd_er_order_bundle(
            &order_params,
            &user.user.doctorcode,
            &app.app_config.doctor_intern_roles,
            &app.db_pool,
            &app.hosxp(),
            &app.kphis(),
            &app.kphis_extra(),
        )
        .await?;

        let note_params = kphis_model::progress_note::ProgressNoteParams {
            opd_er_order_master_id,
            ..Default::default()
        };
        let note =
            kphis_api_handler::opd_er::progress_note::get_opd_er_progress_note_bundle(&note_params, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                .await?;

        let vs_params = kphis_model::vital_sign::VitalSignParams {
            opd_er_order_master_id: Some(id),
            ..Default::default()
        };
        let vs = kphis_api_query::opd_er::vital_sign::get_vital_sign(&vs_params, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

        let io_params = kphis_model::ipd::io::IoParams {
            opd_er_order_master_id,
            ..Default::default()
        };
        let io = kphis_api_query::ipd::io::get_io_shift(
            &io_params,
            app.app_config.shift_day_start,
            app.app_config.shift_evening_start,
            app.app_config.shift_night_start,
            &app.db_pool,
            &app.hosxp(),
            &app.kphis(),
        )
        .await?;

        serde_json::json!({
            "id": vn,
            "patient": patient,
            "order": order,
            "note": note,
            "vs": vs,
            "io": io,
            "lab": lab,
        })
        .to_string()
    } else {
        serde_json::json!({
            "id": vn,
            "patient": patient,
            "order_item": null,
            "note": null,
            "vs": null,
            "io": null,
            "lab": lab,
        })
        .to_string()
    };

    Ok(data)
}

async fn opd_er_focus_list(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let focus = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let params = kphis_model::focus_list::FocusListParams::default();
        Some(kphis_api_query::opd_er::focus_list::get_focus_list(opd_er_order_master_id, &params, &app.db_pool, &app.hosxp(), &app.kphis()).await?)
    } else {
        None
    };
    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "focus": focus,
    })
    .to_string();

    Ok(data)
}

async fn opd_er_focus_note(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let note = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let params = kphis_model::focus_note::FocusNoteParams::default();
        Some(kphis_api_query::opd_er::focus_note::get_focus_note(opd_er_order_master_id, &params, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?)
    } else {
        None
    };
    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "note": note,
    })
    .to_string();

    Ok(data)
}

async fn opd_er_index_plan(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let order_item = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let order_params = kphis_model::order::OrderParams {
            opd_er_order_master_id: Some(opd_er_order_master_id),
            view_by: Some(String::from("doctor")),
            ..Default::default()
        };
        Some(kphis_api_handler::opd_er::order::get_opd_er_order_item_bundle(&order_params, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?)
    } else {
        None
    };

    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "order_item": order_item,
    })
    .to_string();

    Ok(data)
}

async fn opd_er_io(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let io = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let params = kphis_model::ipd::io::IoParams {
            opd_er_order_master_id: zero_none(opd_er_order_master_id),
            ..Default::default()
        };
        Some(
            kphis_api_query::opd_er::io::get_io_shift(
                &params,
                app.app_config.shift_day_start,
                app.app_config.shift_evening_start,
                app.app_config.shift_night_start,
                &app.db_pool,
                &app.hosxp(),
                &app.kphis(),
            )
            .await?,
        )
    } else {
        None
    };
    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "io": io,
    })
    .to_string();

    Ok(data)
}

async fn opd_er_medical_history(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let data = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let visit_datetime = patient.as_ref().and_then(|pt| datetime_from_opt(pt.vstdate, pt.vsttime)).map(|dt| dt.js_string());
        let params = kphis_model::opd_er::medical_history::OpdErMedicalHistoryParams {
            opd_er_order_master_id: zero_none(opd_er_order_master_id),
            hn: patient.as_ref().and_then(|pt| pt.hn.clone()),
            vn: patient.as_ref().and_then(|pt| pt.vn.clone()),
            visit_datetime,
            ..Default::default()
        };
        let med = kphis_api_query::opd_er::medical_history::get_medical_history(&params, &app.app_config.hospital_name, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let trauma = kphis_api_query::opd_er::medical_history::get_trauma_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra()).await?;
        let allergy = kphis_api_query::opd_er::medical_history::get_allergy_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let screen = kphis_api_query::opd_er::medical_history::get_screen_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let consult = kphis_api_query::opd_er::medical_history::get_consult_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        let ft = kphis_api_query::opd_er::medical_history::get_ft_history(opd_er_order_master_id, &app.db_pool, &app.hosxp(), &app.kphis()).await?;

        serde_json::json!({
            "id": vn,
            "patient": patient,
            "med": med,
            "trauma": trauma,
            "allergy": allergy,
            "screen": screen,
            "consult": consult,
            "ft": ft,
        })
        .to_string()
    } else {
        serde_json::json!({
            "id": vn,
            "patient": patient,
            "med": null,
            "trauma": null,
            "allergy": null,
            "screen": null,
            "consult": null,
            "ft": null,
        })
        .to_string()
    };

    Ok(data)
}

async fn opd_er_med_reconciliation(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let recon = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let params = kphis_model::med_reconcile::MedReconciliationParams {
            opd_er_order_master_id: zero_none(opd_er_order_master_id),
            ..Default::default()
        };
        Some(kphis_api_query::opd_er::med_reconcile::get_opd_er_med_reconcile(&params, &None, &app.db_pool, &app.hosxp(), &app.kphis()).await?)
    } else {
        None
    };

    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "recon": recon,
    })
    .to_string();

    Ok(data)
}

async fn opd_er_order(vn: &str, doctorcode: &Option<String>, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let data = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let oneday_params = kphis_model::order::OrderParams {
            opd_er_order_master_id: zero_none(opd_er_order_master_id),
            order_type: Some(String::from("oneday")),
            view_by: Some(String::from("doctor")),
            ..Default::default()
        };
        let oneday = kphis_api_handler::opd_er::order::get_opd_er_order_bundle(
            &oneday_params,
            doctorcode,
            &app.app_config.doctor_intern_roles,
            &app.db_pool,
            &app.hosxp(),
            &app.kphis(),
            &app.kphis_extra(),
        )
        .await?;
        let cont_params = kphis_model::order::OrderParams {
            opd_er_order_master_id: zero_none(opd_er_order_master_id),
            order_type: Some(String::from("continuous")),
            view_by: Some(String::from("doctor")),
            ..Default::default()
        };
        let cont = kphis_api_handler::opd_er::order::get_opd_er_order_bundle(
            &cont_params,
            doctorcode,
            &app.app_config.doctor_intern_roles,
            &app.db_pool,
            &app.hosxp(),
            &app.kphis(),
            &app.kphis_extra(),
        )
        .await?;
        let note_params = kphis_model::progress_note::ProgressNoteParams {
            opd_er_order_master_id: zero_none(opd_er_order_master_id),
            ..Default::default()
        };
        let note =
            kphis_api_handler::opd_er::progress_note::get_opd_er_progress_note_bundle(&note_params, &app.app_config.doctor_intern_roles, &app.db_pool, &app.hosxp(), &app.kphis(), &app.kphis_extra())
                .await?;

        serde_json::json!({
            "id": vn,
            "patient": patient,
            "oneday": oneday,
            "cont": cont,
            "note": note,
        })
        .to_string()
    } else {
        serde_json::json!({
            "id": vn,
            "patient": patient,
            "oneday": null,
            "cont": null,
            "note": null,
        })
        .to_string()
    };

    Ok(data)
}

async fn opd_er_vital_sign(vn: &str, app: &ApiState) -> Result<String, AppError> {
    let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vn, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
    let vs = if let Some(opd_er_order_master_id) = patient.as_ref().and_then(|pt| pt.opd_er_order_master_id) {
        let params = kphis_model::vital_sign::VitalSignParams {
            opd_er_order_master_id: zero_none(opd_er_order_master_id),
            ..Default::default()
        };
        Some(kphis_api_query::opd_er::vital_sign::get_vital_sign(&params, &app.db_pool, &app.hosxp(), &app.kphis()).await?)
    } else {
        None
    };
    let data = serde_json::json!({
        "id": vn,
        "patient": patient,
        "vs": vs,
    })
    .to_string();

    Ok(data)
}

// vn can be 'vn' or 'an'
async fn refer_note(vnan: &str, app: &ApiState) -> Result<String, AppError> {
    let refernote = kphis_api_query::refer_note::select_refernote(vnan, &app.db_pool, &app.hosxp(), &app.kphis_extra()).await?;

    let data = if vnan.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": vnan,
            "patient": patient,
            "refernote": refernote,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": vnan,
            "patient": patient,
            "refernote": refernote,
        })
        .to_string()
    };

    Ok(data)
}

// vn can be 'vn' or 'an'
async fn refer_out(vnan: &str, app: &ApiState) -> Result<String, AppError> {
    let referout = kphis_api_query::refer_out::select_his_referout_data(vnan, &app.db_pool, &app.hosxp()).await?;

    let data = if vnan.len() == app.hosxp_an_len() {
        let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": vnan,
            "patient": patient,
            "referout": referout,
        })
        .to_string()
    } else {
        let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
        serde_json::json!({
            "id": vnan,
            "patient": patient,
            "referout": referout,
        })
        .to_string()
    };

    Ok(data)
}

async fn scan_images(vnan_key_perpage: &str, app: &ApiState) -> Result<String, AppError> {
    let triplets = vnan_key_perpage.split('|').collect::<Vec<&str>>();
    let len = triplets.len();
    if len != 3 || (len == 3 && triplets[0].is_empty()) {
        Err(AppError::app_400("Get DocumentImages"))
    } else {
        let vnan = triplets[0];
        let key = triplets[1];
        let per_page = triplets[2].parse::<u8>().unwrap_or(1);
        let data = if vnan.len() == app.hosxp_an_len() {
            let patient = kphis_api_query::ipd::show_patient_main::get_show_patient_main(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
            let vn_opt = patient.as_ref().and_then(|pt| pt.vn.clone());
            let images = match key {
                "pe" => {
                    if let Some(vn) = vn_opt.as_ref() {
                        kphis_api_query::image::scan_his::get_pe_image(&vn, &app.db_pool, &app.hosxp()).await?
                    } else {
                        Vec::new()
                    }
                }
                "er" => {
                    if let Some(vn) = vn_opt.as_ref() {
                        kphis_api_query::image::scan_his::get_er_image(&vn, &app.db_pool, &app.hosxp()).await?
                    } else {
                        Vec::new()
                    }
                }
                "lab" => kphis_api_query::image::scan_his::get_lab_image(&vnan, &vn_opt, &app.db_pool, &app.hosxp()).await?,
                "opd" => {
                    if let Some(vn) = vn_opt.as_ref() {
                        kphis_api_query::image::scan_his::get_opd_image(&vn, &app.db_pool, &app.hosxp()).await?
                    } else {
                        Vec::new()
                    }
                }
                _ => Err(AppError::app_400("Select ScanHisImage"))?,
            };
            serde_json::json!({
                "id": vnan,
                "key": key,
                "per_page": per_page,
                "patient": patient,
                "images": images,
            })
            .to_string()
        } else {
            let patient = kphis_api_query::opd_er::show_patient_main::get_show_patient_main_vn(vnan, &app.db_pool, &app.hosxp(), &app.kphis()).await?;
            let images = match key {
                "pe" => kphis_api_query::image::scan_his::get_pe_image(&vnan, &app.db_pool, &app.hosxp()).await?,
                "er" => kphis_api_query::image::scan_his::get_er_image(&vnan, &app.db_pool, &app.hosxp()).await?,
                "lab" => kphis_api_query::image::scan_his::get_lab_image(&vnan, &None, &app.db_pool, &app.hosxp()).await?,
                "opd" => kphis_api_query::image::scan_his::get_opd_image(&vnan, &app.db_pool, &app.hosxp()).await?,
                _ => Err(AppError::app_400("Select ScanHisImage"))?,
            };
            serde_json::json!({
                "id": vnan,
                "key": key,
                "per_page": per_page,
                "patient": patient,
                "images": images,
            })
            .to_string()
        };

        Ok(data)
    }
}
