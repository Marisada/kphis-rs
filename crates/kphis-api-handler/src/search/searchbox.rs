use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::search::searchbox;
use kphis_model::search::searchbox::{
    DrugCheckParams, DrugDuplicateCheck, DrugInteractionCheck, HospSearchBox, IvfluidSearchbox, LabSearchbox, MedSearchbox, OpdVisitSearchbox, PatientSearchbox, XraySearchbox,
};
use kphis_util::error::{AppError, Source};

// common-searchbox-lab-data.php
/// /api/search/box/lab-text/{search_text}
///
/// Get Lab Search-Box by text, return a list of Lab Search-Box
#[utoipa::path(
    get,
    path = "/search/box/lab-text/{search_text}",
    responses(DocVec<LabSearchbox>),
    params(
        ("search_text" = String, Path, description = "Search text", example = "hct")
    ),
)]
pub async fn get_lab_searchbox(Path(search_text): Path<String>, ctx: RequestState) -> Result<Json<Vec<LabSearchbox>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let decoded_text = urlencoding::decode(&search_text).map_err(|e| Source::UrlEncoding.to_error(500, e, "Get LabSearchBox"))?;
    let response = searchbox::get_lab_searchbox(&decoded_text, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}

// common-searchbox-xray-data.php
/// /api/search/box/xray-text/{search_text}
///
/// Get X-Ray Search-Box by text, return a list of X-Ray Search-Box
#[utoipa::path(
    get,
    path = "/search/box/xray-text/{search_text}",
    responses(DocVec<XraySearchbox>),
    params(
        ("search_text" = String, Path, description = "Search text", example = "cxr")
    ),
)]
pub async fn get_xray_searchbox(Path(search_text): Path<String>, ctx: RequestState) -> Result<Json<Vec<XraySearchbox>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let decoded_text = urlencoding::decode(&search_text).map_err(|e| Source::UrlEncoding.to_error(500, e, "Get XraySearchBox"))?;
    let response = searchbox::get_xray_searchbox(&decoded_text, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}

// common-searchbox-ivfluid-data.php
/// /api/search/box/ivfluid-text/{search_text}
///
/// Get IV-Fluid Search-Box by text, return a list of IV-Fluid Search-Box
#[utoipa::path(
    get,
    path = "/search/box/ivfluid-text/{search_text}",
    responses(DocVec<IvfluidSearchbox>),
    params(
        ("search_text" = String, Path, description = "Search text", example = "acetar")
    ),
)]
pub async fn get_ivfluid_searchbox(Path(search_text): Path<String>, ctx: RequestState) -> Result<Json<Vec<IvfluidSearchbox>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let decoded_text = urlencoding::decode(&search_text).map_err(|e| Source::UrlEncoding.to_error(500, e, "Get IvFluidSearchBox"))?;
    let response = searchbox::get_ivfluid_searchbox(&decoded_text, &ctx.api_state.ivfluid(), &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}

// common-searchbox-med-data.php
/// /api/search/box/med-hn-text/{hn}/{search_text}
///
/// Get Medication Search-Box by HN and text, return a list of Medication Search-Box
///
/// HN can be '-' but drug allergy will not detected
#[utoipa::path(
    get,
    path = "/search/box/med-hn-text/{hn}/{search_text}",
    responses(DocVec<MedSearchbox>),
    params(
        ("hn" = String, Path, description = "Hospital Number: HN", example = "0001234"),
        ("search_text" = String, Path, description = "Search text", example = "amoxy")
    ),
)]
pub async fn get_med_searchbox(Path((hn, search_text)): Path<(String, String)>, ctx: RequestState) -> Result<Json<Vec<MedSearchbox>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let decoded_text = urlencoding::decode(&search_text).map_err(|e| Source::UrlEncoding.to_error(500, e, "Get MedSearchBox"))?;
    let response = if hn.as_str() == "-" {
        searchbox::get_med_searchbox_without_hn(&decoded_text, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?
    } else {
        searchbox::get_med_searchbox(&hn, &decoded_text, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?
    };

    Ok(Json(response))
}

// // ipd-dr-order-item-drug-duplication-check.php
/// /api/search/box/med/duplicate
///
/// Get Drug-Duplication Checker by PARAMS, return a list of Drug-Duplication
#[utoipa::path(
    get,
    path = "/search/box/med/duplicate",
    responses(DocVec<DrugDuplicateCheck>),
    params(DrugCheckParams),
)]
pub async fn get_drug_duplication_check(Query(params): Query<DrugCheckParams>, ctx: RequestState) -> Result<Json<Vec<DrugDuplicateCheck>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let duplications = searchbox::get_drug_duplicate_check(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(duplications))
}

// // ipd-dr-order-item-drug-interaction-check.php
/// /api/search/box/med/interaction
///
/// Get Drug-Interaction Checker by PARAMS, return a list of Drug-Interaction
#[utoipa::path(
    get,
    path = "/search/box/med/interaction",
    responses(DocVec<DrugInteractionCheck>),
    params(DrugCheckParams),
)]
pub async fn get_drug_interaction_check(Query(params): Query<DrugCheckParams>, ctx: RequestState) -> Result<Json<Vec<DrugInteractionCheck>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let interactions = searchbox::get_drug_interaction_check(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(interactions))
}

// common-searchbox-patient-data.php
/// /api/search/box/patient-text/{search_text}
///
/// Get Patient Search-Box by text, return a list of Patient Search-Box
#[utoipa::path(
    get,
    path = "/search/box/patient-text/{search_text}",
    responses(DocVec<PatientSearchbox>),
    params(
        ("search_text" = String, Path, description = "Search text", example = "01234")
    ),
)]
pub async fn get_patient_searchbox(Path(search_text): Path<String>, ctx: RequestState) -> Result<Json<Vec<PatientSearchbox>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let decoded_text = urlencoding::decode(&search_text).map_err(|e| Source::UrlEncoding.to_error(500, e, "Get PatientSearchBox"))?;
    let response = searchbox::get_patient_searchbox(&decoded_text, ctx.api_state.hosxp_hn_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}

// common-searchbox-opd_visit-data.php
/// /api/search/box/opd-visit-mode-text/{mode}/{search_text}
///
/// Get OPD Visit Search-Box by mode and text, return a list of OPD Visit Search-Box
#[utoipa::path(
    get,
    path = "/search/box/opd-visit-mode-text/{mode}/{search_text}",
    responses(DocVec<OpdVisitSearchbox>),
    params(
        ("mode" = String, Path, description = "[hn, qn, vn, pt-name, cid, all]", example = "hn"),
        ("search_text" = String, Path, description = "Search text", example = "01234")
    ),
)]
pub async fn get_opd_visit_searchbox(Path((mode, search_text)): Path<(String, String)>, ctx: RequestState) -> Result<Json<Vec<OpdVisitSearchbox>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let decoded_text = urlencoding::decode(&search_text).map_err(|e| Source::UrlEncoding.to_error(500, e, "Get OpdVisitSearchBox"))?;
    let response = searchbox::get_opd_visit_searchbox(&mode, &decoded_text, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}

// ipd-summary-2-hospcode-data.php
/// /api/search/box/hosp-text/{search_text}
///
/// Get Hospital Search-Box by text, return a list of Hospital Search-Box
#[utoipa::path(
    get,
    path = "/search/box/hosp-text/{search_text}",
    responses(DocVec<HospSearchBox>),
    params(
        ("search_text" = String, Path, description = "Search text", example = "นาดูน")
    ),
)]
pub async fn get_hosp_searchbox(Path(search_text): Path<String>, ctx: RequestState) -> Result<Json<Vec<HospSearchBox>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let decoded_text = urlencoding::decode(&search_text).map_err(|e| Source::UrlEncoding.to_error(500, e, "Get HospSearchBox"))?;
    let response = searchbox::get_hosp_searchbox(&decoded_text, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}
