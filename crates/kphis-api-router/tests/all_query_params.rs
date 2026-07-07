mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use strum::IntoEnumIterator;
use tokio::sync::broadcast;

use kphis_api_pdf::test_state::new_test_state;
use kphis_model::{
    app::VisitTypeId,
    dc_plan,
    endpoint::{EndPoint, QueryString},
    focus_list, focus_note, image, index_action, index_monitor, index_plan, ipd, med_reconcile, opd_er, order, pre_order, prescription, progress_note, refer_note, refer_out, report, vital_sign,
};
use kphis_sqlx_tester::MySqlMocker;

use common::new_test_app_login;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore]
#[rustfmt::skip]
async fn api_all_query_params() {
    // mocker will crate/insert all databases and tables, so we try to run many test as possible
    let mocker = MySqlMocker::new_all().await;
    let (shutdown_sender, _shutdown_recv) = broadcast::channel(5);
    let state = new_test_state(mocker.db_pool.clone(), shutdown_sender).await;
    let server = new_test_app_login(&state).await;

    // NOTE: GET, DELETE are OK without `str_some` or `zero_none`
    // but POST, PUT, PATCH need `str_some` or `zero_none` to prevent Some("") or Some(0) to be used
    for ep in EndPoint::iter() {
        api_all_query_params_match(ep, &server).await;
    }
}

// list all APIs that can 400 Bad Request or 405 Method Not Allowed
// check_an_can_execute() already has an.is_empty() check internally
// NOTE: Empty AN in path argument will return 405, not hit check_an_can_execute()
#[rustfmt::skip]
async fn api_all_query_params_match(ep: EndPoint, server: &TestServer) {
    match ep {
        EndPoint::AvatarOpdEr => {}
        EndPoint::AvatarIpd => {}
        EndPoint::DrugUseDuration => {}
        EndPoint::ExistsKeyId => {}
        EndPoint::EmrDateHn => {}
        EndPoint::EmrVisitVn => {}
        EndPoint::HisIptDiagAn => {}
        EndPoint::HisIptOprtAn => {}
        EndPoint::HisMedPlanIpdAn => {}
        EndPoint::HisOperationAdmitAn => {}
        EndPoint::HisReferOutVnan => {
            // test vnan MUST EQUAL to vn in payload
            // refer_out::post_his_referout
            // POST /api/his/refer-out-vnan/{vnan}
            assert_eq!(server
                .post(&[&EndPoint::HisReferOutVnan.base(), "990001234"].concat())
                .json(&refer_out::HisReferOutSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::Image => {} // POST multipart
        EndPoint::ImageUsage => {}
        EndPoint::ImageUsageId => {}
        EndPoint::IpdAdmissionNoteDrAn => {}
        EndPoint::IpdAdmissionNoteDrPharmCheckAn => {
            // check_an_can_execute(): AN in path, null AN will return 405 and not hit check_an_can_execute()
            // kphis_api_handler::ipd::admission_note_dr::patch_ipd_pharmacy_check
            // PATCH /api/ipd/admission-note-dr/pharmacy-check-an/{an}
            assert_eq!(server
                .patch(&EndPoint::IpdAdmissionNoteDrPharmCheckAn.base())
                .await.status_code(), StatusCode::METHOD_NOT_ALLOWED);
        }
        EndPoint::IpdAdmissionNoteDr => {
            let mut post_api_ipd_admission_note_dr_saver = ipd::admission_note_dr::IpdAdmissionNoteDrSave::demo();
            post_api_ipd_admission_note_dr_saver.admission_note.an = String::new();
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::admission_note_dr::post_ipd_admission_note_dr
            // POST /api/ipd/admission-note-dr
            assert_eq!(server
                .post(&EndPoint::IpdAdmissionNoteDr.base())
                .json(&post_api_ipd_admission_note_dr_saver)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::admission_note_dr::put_ipd_admission_note_dr
            // PUT /api/ipd/admission-note-dr
            assert_eq!(server
                .put(&EndPoint::IpdAdmissionNoteDr.base())
                .json(&post_api_ipd_admission_note_dr_saver)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdAdmissionNoteNurseAn => {}
        EndPoint::IpdAdmissionNoteNurse => {
            let mut post_api_ipd_admission_note_nurse_saver = ipd::admission_note_nurse::IpdNurseAdmissionNote::demo();
            post_api_ipd_admission_note_nurse_saver.an = String::new();
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::admission_note_nurse::post_ipd_admission_note_nurse
            // POST /api/ipd/admission-note-nurse
            assert_eq!(server
                .post(&EndPoint::IpdAdmissionNoteNurse.base())
                .json(&post_api_ipd_admission_note_nurse_saver)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::admission_note_nurse::put_ipd_admission_note_nurse
            // PUT /api/ipd/admission-note-nurse
            assert_eq!(server
                .put(&EndPoint::IpdAdmissionNoteNurse.base())
                .json(&post_api_ipd_admission_note_nurse_saver)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdConsult => {
            let mut post_api_ipd_consult= ipd::consult::ConsultSave::demo();
            post_api_ipd_consult.an = String::new();
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::consult::post_ipd_consult
            // POST /api/ipd/consult
            assert_eq!(server
                .post(&EndPoint::IpdConsult.base())
                .json(&post_api_ipd_consult)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: `consult_id` and `version` is Some
            // kphis_api_handler::ipd::consult::delete_ipd_consult_by_id
            // DELETE /api/ipd/consult
            assert_eq!(server
                .delete(&[EndPoint::IpdConsult.base(), ipd::consult::ConsultParams {consult_id: None, version: Some(1)}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[EndPoint::IpdConsult.base(), ipd::consult::ConsultParams {consult_id: Some(1), version: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdConsultAn => {}
        EndPoint::IpdConsultId => {}
        EndPoint::IpdDcPlanAn => {
            // check_an_can_execute(): AN in path, null AN will return 405 and not hit check_an_can_execute()
            // kphis_api_handler::ipd::dc_plan::post_ipd_dc_plan
            // POST /api/ipd/dc-plan-an/{an}
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanAn.base())
                .json(&dc_plan::DischargePlanSave::demo())
                .await.status_code(), StatusCode::METHOD_NOT_ALLOWED);
            // check query params: dc_plan_id and version is Some
            // kphis_api_handler::ipd::dc_plan::delete_ipd_dc_plan
            // DELETE /api/ipd/dc-plan-an/{an}
            assert_eq!(server
                .delete(&[&EndPoint::IpdDcPlanAn.base(), "660001234", &dc_plan::DischargePlanParams {dc_plan_id: None, version: Some(1)}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[&EndPoint::IpdDcPlanAn.base(), "660001234", &dc_plan::DischargePlanParams {dc_plan_id: Some(1), version: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdDcPlanTmpDx => {
            // check payload field: `dx_name` is Some(not empty string)
            // kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_dx
            // POST /api/ipd/dc-plan-tmp/dx
            let mut post_api_ipd_dc_plan_tmp_dx= ipd::dc_plan_tmp::DcPlanTmpDx::demo();
            post_api_ipd_dc_plan_tmp_dx.dx_name = None;
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpDx.base())
                .json(&post_api_ipd_dc_plan_tmp_dx)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_dc_plan_tmp_dx.dx_name = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpDx.base())
                .json(&post_api_ipd_dc_plan_tmp_dx)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_dx
            // DELETE /api/ipd/dc-plan-tmp/dx
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpDx.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdDcPlanTmpMed => {
            // check payload field: `med_text` is Some(not empty string)
            // kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_med
            // POST /api/ipd/dc-plan-tmp/med
            let mut post_api_ipd_dc_plan_tmp_med= ipd::dc_plan_tmp::DcPlanTmpMed::demo();
            post_api_ipd_dc_plan_tmp_med.med_text = None;
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpMed.base())
                .json(&post_api_ipd_dc_plan_tmp_med)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_dc_plan_tmp_med.med_text = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpMed.base())
                .json(&post_api_ipd_dc_plan_tmp_med)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_med
            // DELETE /api/ipd/dc-plan-tmp/med
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpMed.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdDcPlanTmpEnv => {
            // check payload field: `env_text` is Some(not empty string)
            // kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_env
            // POST /api/ipd/dc-plan-tmp/env
            let mut post_api_ipd_dc_plan_tmp_env= ipd::dc_plan_tmp::DcPlanTmpEnv::demo();
            post_api_ipd_dc_plan_tmp_env.env_text = None;
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpEnv.base())
                .json(&post_api_ipd_dc_plan_tmp_env)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_dc_plan_tmp_env.env_text = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpEnv.base())
                .json(&post_api_ipd_dc_plan_tmp_env)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_env
            // DELETE /api/ipd/dc-plan-tmp/env
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpEnv.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdDcPlanTmpTx => {
            // check payload field: `tx_text` is Some(not empty string)
            // kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_tx
            // POST /api/ipd/dc-plan-tmp/tx
            let mut post_api_ipd_dc_plan_tmp_tx= ipd::dc_plan_tmp::DcPlanTmpTx::demo();
            post_api_ipd_dc_plan_tmp_tx.tx_text = None;
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpTx.base())
                .json(&post_api_ipd_dc_plan_tmp_tx)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_dc_plan_tmp_tx.tx_text = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpTx.base())
                .json(&post_api_ipd_dc_plan_tmp_tx)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_tx
            // DELETE /api/ipd/dc-plan-tmp/tx
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpTx.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdDcPlanTmpDiet => {
            // check payload field: `diet_text` is Some(not empty string)
            // kphis_api_handler::ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_diet
            // POST /api/ipd/dc-plan-tmp/diet
            let mut post_api_ipd_dc_plan_tmp_diet= ipd::dc_plan_tmp::DcPlanTmpDiet::demo();
            post_api_ipd_dc_plan_tmp_diet.diet_text = None;
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpDiet.base())
                .json(&post_api_ipd_dc_plan_tmp_diet)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_dc_plan_tmp_diet.diet_text = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpDiet.base())
                .json(&post_api_ipd_dc_plan_tmp_diet)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_diet
            // DELETE /api/ipd/dc-plan-tmp/diet
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpDiet.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdDoctorInCharge => {
            // check_an_opt_can_execute(): AN in payload
            // kphis_api_handler::ipd::doctor_in_charge::post_ipd_doctor_in_charge
            // POST /api/ipd/doctor-in-charge
            let mut post_api_ipd_doctor_in_charge = ipd::doctor_in_charge::IpdDoctorInCharge::demo();
            post_api_ipd_doctor_in_charge.an = None;
            assert_eq!(server
                .post(&EndPoint::IpdDoctorInCharge.base())
                .json(&post_api_ipd_doctor_in_charge)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_doctor_in_charge.an = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdDoctorInCharge.base())
                .json(&post_api_ipd_doctor_in_charge)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: doctor_in_charge_id and version is Some
            // kphis_api_handler::ipd::doctor_in_charge::delete_ipd_doctor_in_charge
            // DELETE /api/ipd/doctor-in-charge
            assert_eq!(server
                .delete(&[EndPoint::IpdDoctorInCharge.base(), ipd::doctor_in_charge::DoctorInChargeParams {doctor_in_charge_id: None, version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[EndPoint::IpdDoctorInCharge.base(), ipd::doctor_in_charge::DoctorInChargeParams {doctor_in_charge_id: Some(1), version: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdDocumentDatetimeAn => {}
        EndPoint::IpdDocumentListVnAn => {}
        EndPoint::IpdDocumentScanAn => {
            // check_an_can_execute(): AN in path, null AN will return 405 and not hit check_an_can_execute()
            // kphis_api_handler::ipd::document::post_ipd_document_type
            // POST /ipd/document/scan-an/{an}
            assert_eq!(server
                .post(&EndPoint::IpdDocumentScanAn.base())
                .json(&1)
                .await.status_code(), StatusCode::METHOD_NOT_ALLOWED);
            // check_an_can_execute(): AN in path, null AN will return 405 and not hit check_an_can_execute()
            // kphis_api_handler::ipd::document::delete_ipd_document_type
            // DELETE /ipd/document/scan-an/{an}
            assert_eq!(server
                .delete(&EndPoint::IpdDocumentScanAn.base())
                .json(&1)
                .await.status_code(), StatusCode::METHOD_NOT_ALLOWED);
        }
        EndPoint::IpdFocusListAn => {
            // check_an_can_execute(): AN in path, null AN will return 405 and not hit check_an_can_execute()
            // check query params: hn is Some(not empty string)
            // kphis_api_handler::ipd::focus_list::post_ipd_focus_list
            // POST /api/ipd/focus-list-an/{an}
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusListAn.base(), "660001234", &focus_list::FocusListSaveParams {hn: None}.query_string()].concat())
                .json(&focus_list::FocusListSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusListAn.base(), "660001234", &focus_list::FocusListSaveParams {hn: Some(String::new())}.query_string()].concat())
                .json(&focus_list::FocusListSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: fclist_id and version is Some
            // kphis_api_handler::ipd::focus_list::delete_ipd_focus_list
            // DELETE /api/ipd/focus-list-an/{an}
            assert_eq!(server
                .delete(&[&EndPoint::IpdFocusListAn.base(), "660001234", &focus_list::FocusListParams {fclist_id: None, version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[&EndPoint::IpdFocusListAn.base(), "660001234", &focus_list::FocusListParams {fclist_id: Some(1), version: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdFocusNoteAn => {
            // check_an_can_execute(): AN in path, null AN will return 405 and not hit check_an_can_execute()
            // check query params: hn and ward is Some(not empty string)
            // kphis_api_handler::ipd::focus_note::post_ipd_focus_note
            // POST /api/ipd/focus-note-an/{an}
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteSaveParams {hn: None, ward: Some(String::from("01"))}.query_string()].concat())
                .json(&focus_note::FocusNoteSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteSaveParams {hn: Some(String::new()), ward: Some(String::from("01"))}.query_string()].concat())
                .json(&focus_note::FocusNoteSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteSaveParams {hn: Some(String::from("0001234")), ward: None}.query_string()].concat())
                .json(&focus_note::FocusNoteSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteSaveParams {hn: Some(String::from("0001234")), ward: Some(String::new())}.query_string()].concat())
                .json(&focus_note::FocusNoteSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: fcnote_id and version is Some
            // kphis_api_handler::ipd::focus_note::delete_ipd_focus_note
            // DELETE /api/ipd/focus-note-an/{an}
            assert_eq!(server
                .delete(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteParams {fcnote_id: None, version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteParams {fcnote_id: Some(1), version: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdIndexActionId => {}
        EndPoint::IpdIndexAction => {
            // check_an_can_execute(): AN in payload, visit_type is VisitTypeId::Ipd(an) or VisitTypeId::PreAdmit(an)
            // kphis_api_handler::ipd::index_action::post_ipd_index_action
            // POST /api/ipd/index-action
            let mut post_api_ipd_index_action = index_action::IndexAction::demo();
            post_api_ipd_index_action.visit_type = VisitTypeId::OpdEr(String::new(), 0);
            assert_eq!(server
                .post(&EndPoint::IpdIndexAction.base())
                .json(&post_api_ipd_index_action)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_index_action.visit_type = VisitTypeId::Visit(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdIndexAction.base())
                .json(&post_api_ipd_index_action)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdIndexMedPayAn => {}
        EndPoint::IpdIndexMonitorId => {}
        EndPoint::IpdIndexMonitor => {
            // check_an_can_execute(): AN in payload, visit_type is VisitTypeId::Ipd(an) or VisitTypeId::PreAdmit(an)
            // kphis_api_handler::ipd::index_monitor::post_ipd_index_monitor
            // POST /api/ipd/index-monitor
            let mut post_api_ipd_index_monitor = index_monitor::IndexMonitor::demo();
            post_api_ipd_index_monitor.visit_type = VisitTypeId::OpdEr(String::new(), 0);
            assert_eq!(server
                .post(&EndPoint::IpdIndexMonitor.base())
                .json(&post_api_ipd_index_monitor)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_index_monitor.visit_type = VisitTypeId::Visit(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdIndexMonitor.base())
                .json(&post_api_ipd_index_monitor)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdIndexNoteId => {}
        EndPoint::IpdIndexNote => {
            // check_an_opt_can_execute(): AN in payload
            // kphis_api_handler::ipd::index_note::post_ipd_index_note
            // POST /api/ipd/index-note
            let mut post_api_ipd_index_note = ipd::index_note::IndexNote::demo();
            post_api_ipd_index_note.an = None;
            assert_eq!(server
                .post(&EndPoint::IpdIndexNote.base())
                .json(&post_api_ipd_index_note)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_index_note.an = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdIndexNote.base())
                .json(&post_api_ipd_index_note)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdIndexPlanDateAn => {}
        EndPoint::IpdIndexPlanId => {}
        EndPoint::IpdIndexPlan => {
            // check_an_can_execute(): AN in payload, visit_type is VisitTypeId::Ipd(an) or VisitTypeId::PreAdmit(an)
            // kphis_api_handler::ipd::index_plan::post_ipd_index_plan
            // POST /api/ipd/index-plan
            let mut post_api_ipd_index_plan = index_plan::IndexPlanSave::demo();
            post_api_ipd_index_plan.visit_type = VisitTypeId::OpdEr(String::new(), 0);
            assert_eq!(server
                .post(&EndPoint::IpdIndexPlan.base())
                .json(&post_api_ipd_index_plan)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_index_plan.visit_type = VisitTypeId::Visit(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdIndexPlan.base())
                .json(&post_api_ipd_index_plan)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdIo => {
            // check_an_opt_can_execute(): AN in payload
            // kphis_api_handler::ipd::io::post_ipd_io_shift
            // POST /api/ipd/io
            let mut post_api_ipd_io = ipd::io::IoShift::demo();
            post_api_ipd_io.an = None;
            assert_eq!(server
                .post(&EndPoint::IpdIo.base())
                .json(&post_api_ipd_io)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_io.an = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdIo.base())
                .json(&post_api_ipd_io)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: io_id and version is Some
            // kphis_api_handler::ipd::io::delete_ipd_io_shift
            // DELETE /api/ipd/io
            assert_eq!(server
                .delete(&[EndPoint::IpdIo.base(), ipd::io::IoParams {io_id: None, version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[EndPoint::IpdIo.base(), ipd::io::IoParams {io_id: Some(1), version: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdIoDateAn => {}
        EndPoint::IpdMedReconcile => {
            // check_an_opt_can_execute(): AN in query
            // kphis_api_handler::ipd::med_reconcile::post_ipd_med_reconcile
            // POST /api/ipd/med-reconcile
            assert_eq!(server
                .post(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {an: None, ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemSave::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {an: Some(String::new()), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemSave::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: med_reconciliation_id is Some(not zero) and patch is Some(doctor|pharm|unconfirm|last)
            // kphis_api_handler::ipd::med_reconcile::patch_ipd_med_reconcile
            // PATCH /api/ipd/med-reconcile
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: None, patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(0), patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: None, ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("xxx")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("pharm")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("unconfirm")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("last")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            // check query params: med_reconciliation_id is Some or med_reconciliation_item_id is Some
            // kphis_api_handler::ipd::med_reconcile::delete_ipd_med_reconcile
            // DELETE /api/ipd/med-reconcile
            assert_eq!(server
                .delete(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: None, med_reconciliation_item_id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdMedReconcileHosxpAn => {}
        EndPoint::IpdMedReconcileLastDoseAn => {}
        EndPoint::IpdMedReconcileNoteId => {}
        EndPoint::IpdMedReconcileRemedVisitHn => {}
        EndPoint::IpdMedReconcileRemedMed => {}
        EndPoint::IpdMra => {
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::mra::post_ipd_mra
            // POST /api/ipd/mra
            let mut post_api_ipd_mra = ipd::mra::IpdMra::demo();
            post_api_ipd_mra.an = String::new();
            assert_eq!(server
                .post(&EndPoint::IpdMra.base())
                .json(&post_api_ipd_mra)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::mra::put_ipd_mra
            // PUT /api/ipd/mra
            assert_eq!(server
                .put(&EndPoint::IpdMra.base())
                .json(&post_api_ipd_mra)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: mra_id is Some
            // kphis_api_handler::ipd::mra::delete_ipd_mra
            // DELETE /api/ipd/mra
            assert_eq!(server
                .delete(&[EndPoint::IpdMra.base(), ipd::mra::MraParams {mra_id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdOrderItem => {}
        EndPoint::IpdOrderPrevious => {}
        EndPoint::IpdOrderOnedayPreviousAn => {}
        EndPoint::IpdOrderProgressPrevious => {}
        EndPoint::IpdOrderToHomeMedAn => {}
        EndPoint::IpdOrderOrderDateAn => {}
        EndPoint::IpdOrderOrderId => {}
        EndPoint::IpdOrderOrder => {
            // check_an_can_execute(): AN in payload, visit_type is VisitTypeId::Ipd(an) or VisitTypeId::PreAdmit(an)
            // kphis_api_handler::ipd::order::post_ipd_order
            // POST /api/ipd/order/order
            let mut post_api_ipd_order_order_saver = order::OrderSave::demo();
            post_api_ipd_order_order_saver.visit_type = VisitTypeId::OpdEr(String::new(), 0);
            assert_eq!(server
                .post(&EndPoint::IpdOrderOrder.base())
                .json(&post_api_ipd_order_order_saver)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_order_order_saver.visit_type = VisitTypeId::Visit(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdOrderOrder.base())
                .json(&post_api_ipd_order_order_saver)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdOrderProgressNoteId => {}
        EndPoint::IpdOrderProgressNote => {
            // check_an_can_execute(): AN in payload, visit_type is VisitTypeId::Ipd(an) or VisitTypeId::PreAdmit(an)
            // kphis_api_handler::ipd::progress_note::post_ipd_progress_note
            // POST /api/ipd/order/progress-note
            let mut post_api_ipd_order_progress_note = progress_note::ProgressNoteSave::demo();
            post_api_ipd_order_progress_note.visit_type = VisitTypeId::OpdEr(String::new(), 0);
            assert_eq!(server
                .post(&EndPoint::IpdOrderProgressNote.base())
                .json(&post_api_ipd_order_progress_note)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_order_progress_note.visit_type = VisitTypeId::Visit(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdOrderProgressNote.base())
                .json(&post_api_ipd_order_progress_note)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdOrderPharmacy => {}
        EndPoint::IpdPasscode => {}
        EndPoint::IpdPostAdmitCount => {}
        EndPoint::IpdPostAdmitList => {}
        EndPoint::IpdPreAdmit => {}
        EndPoint::IpdPreOrderMasterId => {}
        EndPoint::IpdPreOrderMaster => {}
        EndPoint::IpdPreOrderInto => {
            // check_an_opt_can_execute(): AN in payload only when `into` is Some(order)
            // check payload field: from, into, from_id, into_id is Some
            // kphis_api_handler::pre_order::post_ipd_pre_order_into
            // POST /api/ipd/pre-order/into
            let mut post_api_ipd_pre_order_into = pre_order::order::PreOrderIntoCommand::demo();
            post_api_ipd_pre_order_into.into_id = None;
            assert_eq!(server
                .post(&EndPoint::IpdPreOrderInto.base())
                .json(&post_api_ipd_pre_order_into)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_pre_order_into.into_id = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdPreOrderInto.base())
                .json(&post_api_ipd_pre_order_into)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdPreOrderOrderId => {}
        EndPoint::IpdPreOrderOrder => {}
        EndPoint::IpdPreOrderProgressNoteId => {}
        EndPoint::IpdPreOrderProgressNote => {}
        EndPoint::IpdShowPatientMainAn => {}
        EndPoint::IpdSummary => {
            // check query params: an or summary_id is Some
            // kphis_api_handler::ipd::summary::get_ipd_summary
            // GET /api/ipd/summary
            assert_eq!(server
                .get(&[EndPoint::IpdSummary.base(), ipd::summary::SummaryParams::default().query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check_an_can_execute(): AN in payload
            // check payload field: `an` is not empty, `status` is not Some(claim|done)
            // kphis_api_handler::ipd::summary::post_ipd_summary
            // POST /api/ipd/summary
            let mut post_api_ipd_summary = ipd::summary::SummarySave::demo();
            post_api_ipd_summary.summary.status = Some(String::from("claim"));
            assert_eq!(server
                .post(&EndPoint::IpdSummary.base())
                .json(&post_api_ipd_summary)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_summary.summary.status = Some(String::from("done"));
            assert_eq!(server
                .post(&EndPoint::IpdSummary.base())
                .json(&post_api_ipd_summary)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_summary.summary.status = Some(String::from("review"));
            post_api_ipd_summary.summary.an = String::new();
            assert_eq!(server
                .post(&EndPoint::IpdSummary.base())
                .json(&post_api_ipd_summary)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check_an_can_execute(): AN in payload
            // kphis_api_handler::ipd::summary::patch_ipd_summary
            // PATCH /api/ipd/summary
            let mut patch_api_ipd_summary = ipd::summary::SummaryCodeSave::demo();
            patch_api_ipd_summary.an = String::new();
            assert_eq!(server
                .patch(&EndPoint::IpdSummary.base())
                .json(&patch_api_ipd_summary)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdSummaryAudit => {
            // check query params: summary_audit_id is Some
            // kphis_api_handler::ipd::summaty_audit::delete_ipd_summary_audit
            // DELETE /api/ipd/summary-audit
            assert_eq!(server
                .delete(&[EndPoint::IpdSummaryAudit.base(), ipd::summary_audit::SummaryAuditParams {summary_audit_id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdSummaryNoteId => {
            // check payload field: note_id is Some
            // kphis_api_handler::ipd::summary::patch_ipd_summary_note
            // PATCH /api/ipd/summary-note-id/{summary_id}
            let api_upd_summary_note_no_note_id = ipd::summary::SummaryNoteSave { note_id: None, ..Default::default() };
            assert_eq!(server
                .patch(&[&EndPoint::IpdSummaryNoteId.base(), "1"].concat())
                .json(&api_upd_summary_note_no_note_id)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check payload field: note_id is Some
            // kphis_api_handler::ipd::summary::delete_ipd_summary_note
            // DELETE /api/ipd/summary-note-id/{summary_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdSummaryNoteId.base(), "1"].concat())
                .json(&api_upd_summary_note_no_note_id)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdSummaryStatusId => {}
        EndPoint::IpdTmpGroup => {
            // check payload field: smp_name is Some(not empty string)
            // kphis_api_handler::ipd::tmp::post_ipd_tmp_group
            // POST /api/ipd/tmp/group
            let mut post_api_ipd_tmp_group = ipd::tmp::TmpGroup::demo();
            post_api_ipd_tmp_group.smp_name = None;
            assert_eq!(server
                .post(&EndPoint::IpdTmpGroup.base())
                .json(&post_api_ipd_tmp_group)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_group.smp_name = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdTmpGroup.base())
                .json(&post_api_ipd_tmp_group)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: smp_id is Some
            // kphis_api_handler::ipd::tmp::delete_ipd_tmp_group
            // DELETE /api/ipd/tmp/group
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpGroup.base(), ipd::tmp::TmpParams {smp_id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdTmpSubgroup => {
            // check payload field: smp_id > 0 and subgroup_name is Some(not empty string)
            // kphis_api_handler::ipd::tmp::post_ipd_subgroup
            // POST /api/ipd/tmp/subgroup
            let mut post_api_ipd_tmp_subgroup = ipd::tmp::TmpSubGroup::demo();
            post_api_ipd_tmp_subgroup.smp_id = 0;
            assert_eq!(server
                .post(&EndPoint::IpdTmpSubgroup.base())
                .json(&post_api_ipd_tmp_subgroup)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_subgroup.smp_id = 1;
            post_api_ipd_tmp_subgroup.subgroup_name = None;
            assert_eq!(server
                .post(&EndPoint::IpdTmpSubgroup.base())
                .json(&post_api_ipd_tmp_subgroup)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_subgroup.subgroup_name = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdTmpSubgroup.base())
                .json(&post_api_ipd_tmp_subgroup)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: smp_id and subgroup is Some
            // kphis_api_handler::ipd::tmp::delete_ipd_subgroup
            // DELETE /api/ipd/tmp/subgroup
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpSubgroup.base(), ipd::tmp::TmpParams {smp_id: None, subgroup: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpSubgroup.base(), ipd::tmp::TmpParams {smp_id: Some(1), subgroup: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdTmpFocus => {
            // check payload field: smp_id > 0 and focus_name is Some(not empty string)
            // kphis_api_handler::ipd::tmp::post_ipd_focus
            // POST /api/ipd/tmp/focus
            let mut post_api_ipd_tmp_focus = ipd::tmp::TmpFocus::demo();
            post_api_ipd_tmp_focus.smp_id = 0;
            assert_eq!(server
                .post(&EndPoint::IpdTmpFocus.base())
                .json(&post_api_ipd_tmp_focus)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_focus.smp_id = 1;
            post_api_ipd_tmp_focus.focus_name = None;
            assert_eq!(server
                .post(&EndPoint::IpdTmpFocus.base())
                .json(&post_api_ipd_tmp_focus)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_focus.focus_name = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdTmpFocus.base())
                .json(&post_api_ipd_tmp_focus)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::tmp::delete_ipd_focus
            // DELETE /api/ipd/tmp/focus
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpFocus.base(), ipd::tmp::TmpParams {id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdTmpGoal => {
            // check payload field: smp_id > 0 and goal_name is Some(not empty string)
            // kphis_api_handler::ipd::tmp::post_ipd_goal
            // POST /api/ipd/tmp/goal
            let mut post_api_ipd_tmp_goal = ipd::tmp::TmpGoal::demo();
            post_api_ipd_tmp_goal.smp_id = 0;
            assert_eq!(server
                .post(&EndPoint::IpdTmpGoal.base())
                .json(&post_api_ipd_tmp_goal)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_goal.smp_id = 1;
            post_api_ipd_tmp_goal.goal_name = None;
            assert_eq!(server
                .post(&EndPoint::IpdTmpGoal.base())
                .json(&post_api_ipd_tmp_goal)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_goal.goal_name = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdTmpGoal.base())
                .json(&post_api_ipd_tmp_goal)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::tmp::delete_ipd_goal
            // DELETE /api/ipd/tmp/goal
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpGoal.base(), ipd::tmp::TmpParams {id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdTmpIntvt => {
            // check payload field: smp_id > 0 and intvt_name is Some(not empty string)
            // kphis_api_handler::ipd::tmp::post_ipd_intvt
            // POST /api/ipd/tmp/intvt
            let mut post_api_ipd_tmp_intvt = ipd::tmp::TmpIntvt::demo();
            post_api_ipd_tmp_intvt.smp_id = 0;
            assert_eq!(server
                .post(&EndPoint::IpdTmpIntvt.base())
                .json(&post_api_ipd_tmp_intvt)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_intvt.smp_id = 1;
            post_api_ipd_tmp_intvt.intvt_name = None;
            assert_eq!(server
                .post(&EndPoint::IpdTmpIntvt.base())
                .json(&post_api_ipd_tmp_intvt)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_ipd_tmp_intvt.intvt_name = Some(String::new());
            assert_eq!(server
                .post(&EndPoint::IpdTmpIntvt.base())
                .json(&post_api_ipd_tmp_intvt)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::tmp::delete_ipd_intvt
            // DELETE /api/ipd/tmp/intvt
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpIntvt.base(), ipd::tmp::TmpParams {id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdTmpDlc => {
            // check payload field: dlc_name is not empty
            // kphis_api_handler::ipd::tmp::post_ipd_dlc
            // POST /api/ipd/tmp/dlc
            let mut post_api_ipd_tmp_dlc = ipd::tmp::TmpDlc::demo();
            post_api_ipd_tmp_dlc.dlc_name = String::new();
            assert_eq!(server
                .post(&EndPoint::IpdTmpDlc.base())
                .json(&post_api_ipd_tmp_dlc)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: id is Some
            // kphis_api_handler::ipd::tmp::delete_ipd_dlc
            // DELETE /api/ipd/tmp/dlc
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpDlc.base(), ipd::tmp::TmpParams {id: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::IpdVitalSignId => {}
        EndPoint::IpdVitalSign => {
            // check_an_can_execute(): AN in query
            // check query params: hn and an is Some(not empty string)
            // kphis_api_handler::ipd::vital_sign::post_ipd_vital_sign
            // POST /api/ipd/vital-sign
            let post_api_ipd_vital_sign = vital_sign::VitalSignSave::demo();
            assert_eq!(server
                .post(&[EndPoint::IpdVitalSign.base(), vital_sign::VitalSignParams {hn: None, an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .json(&post_api_ipd_vital_sign)
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[EndPoint::IpdVitalSign.base(), vital_sign::VitalSignParams {hn: Some(String::from("0001234")), an: None, ..Default::default()}.query_string()].concat())
                .json(&post_api_ipd_vital_sign)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check_an_can_execute(): AN in query
            // check query params: hn and an is Some(not empty string)
            // kphis_api_handler::ipd::vital_sign::put_ipd_vital_sign
            // PUT /api/ipd/vital-sign
            assert_eq!(server
                .put(&[EndPoint::IpdVitalSign.base(), vital_sign::VitalSignParams {hn: None, an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .json(&post_api_ipd_vital_sign)
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .put(&[EndPoint::IpdVitalSign.base(), vital_sign::VitalSignParams {hn: Some(String::from("0001234")), an: None, ..Default::default()}.query_string()].concat())
                .json(&post_api_ipd_vital_sign)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::LabHead => {}
        EndPoint::LabItem => {}
        EndPoint::LabReadId => {}
        EndPoint::LabWbcKeyValue => {}
        EndPoint::MedReconcileHn => {}
        EndPoint::OpdErDcPlanId => {
            // check query params: dc_plan_id and version is Some
            // kphis_api_handler::opd_er::dc_plan::delete_opd_er_dc_plan
            // DELETE /api/opd-er/dc-plan-id/{opd_er_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErDcPlanId.base(), "1", &dc_plan::DischargePlanParams {dc_plan_id: None, version: Some(1)}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[&EndPoint::OpdErDcPlanId.base(), "1", &dc_plan::DischargePlanParams {dc_plan_id: Some(1), version: None}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErDocumentListVnId => {}
        EndPoint::OpdErDocumentScanId => {}
        EndPoint::OpdErFocusListId => {
            // check query params: fclist_id and version is Some
            // kphis_api_handler::opd_er::focus_list::delete_opd_er_focus_list
            // DELETE /api/opd-er/focus-list-id/{opd_er_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErFocusListId.base(), "1", &focus_list::FocusListParams {fclist_id: None, version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[&EndPoint::OpdErFocusListId.base(), "1", &focus_list::FocusListParams {fclist_id: Some(1), version: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErFocusNoteId => {
            // check query params: fcnote_id and version is Some
            // kphis_api_handler::opd_er::focus_note::delete_opd_er_focus_note
            // DELETE /api/opd-er/focus-note-id/{opd_er_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErFocusNoteId.base(), "1", &focus_note::FocusNoteParams {fcnote_id: None, version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[&EndPoint::OpdErFocusNoteId.base(), "1", &focus_note::FocusNoteParams {fcnote_id: Some(1), version: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErHisMedVn => {}
        EndPoint::OpdErIndexActionId => {}
        EndPoint::OpdErIndexAction => {}
        EndPoint::OpdErIndexMonitorId => {}
        EndPoint::OpdErIndexMonitor => {}
        EndPoint::OpdErIndexPlanId => {}
        EndPoint::OpdErIndexPlan => {}
        EndPoint::OpdErIo => {
            // check payload field: opd_er_order_master_id is Some(not zero)
            // kphis_api_handler::opd_er::io::post_opd_er_io_shift
            // POST /api/opd-er/io
            let mut post_api_opd_er_io = ipd::io::IoShift::demo();
            post_api_opd_er_io.opd_er_order_master_id = None;
            assert_eq!(server
                .post(&EndPoint::OpdErIo.base())
                .json(&post_api_opd_er_io)
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_opd_er_io.opd_er_order_master_id = Some(0);
            assert_eq!(server
                .post(&EndPoint::OpdErIo.base())
                .json(&post_api_opd_er_io)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: io_id and version is Some
            // kphis_api_handler::opd_er::io::delete_opd_er_io_shift
            // DELETE /api/opd-er/io
            assert_eq!(server
                .delete(&[EndPoint::OpdErIo.base(), ipd::io::IoParams {io_id: None, version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .delete(&[EndPoint::OpdErIo.base(), ipd::io::IoParams {io_id: Some(1), version: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErIoDateId => {}
        EndPoint::OpdErMedicalHistory => {
            // check query params: [only_opdscreen is Some(true) + vn is Some(not empty string)]
            // or [only_opdscreen is Some(false) or None + opd_er_order_master_id and hn and vn and visit_datetime is Some]
            // kphis_api_handler::opd_er::medical_history::get_opd_er_medical_history
            // GET /api/opd-er/medical-history
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistory.base(), opd_er::medical_history::OpdErMedicalHistoryParams {only_opdscreen: Some(true), vn: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistory.base(), opd_er::medical_history::OpdErMedicalHistoryParams {only_opdscreen: Some(false), vn: None, hn: Some(String::from("0001234")), opd_er_order_master_id: Some(1), visit_datetime: Some(String::from("2023-12-31")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistory.base(), opd_er::medical_history::OpdErMedicalHistoryParams {only_opdscreen: Some(false), vn: Some(String::from("661231235959")), hn: None, opd_er_order_master_id: Some(1), visit_datetime: Some(String::from("2023-12-31")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistory.base(), opd_er::medical_history::OpdErMedicalHistoryParams {only_opdscreen: None, vn: Some(String::from("661231235959")), hn: Some(String::from("0001234")), opd_er_order_master_id: None, visit_datetime: Some(String::from("2023-12-31")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistory.base(), opd_er::medical_history::OpdErMedicalHistoryParams {only_opdscreen: None, vn: Some(String::from("661231235959")), hn: Some(String::from("0001234")), opd_er_order_master_id: Some(1), visit_datetime: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErMedicalHistoryTrauma => {}
        EndPoint::OpdErMedicalHistoryAllergy => {
            // check payload field: `opd_er_order_master_id` and `version` has the same value
            // kphis_api_handler::opd_er::medical_history::post_opd_er_allergy_history
            // POST /api/opd-er/medical-history-allergy
            let post_api_opd_er_medical_history_allergy_1 =  opd_er::medical_history::AllergyHistory::demo();
            let mut post_api_opd_er_medical_history_allergy_2 = opd_er::medical_history::AllergyHistory::demo();
            post_api_opd_er_medical_history_allergy_2.er_allergy_history_id = 2;
            post_api_opd_er_medical_history_allergy_2.opd_er_order_master_id = Some(2);
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryAllergy.base())
                .json(&vec![post_api_opd_er_medical_history_allergy_1.clone(), post_api_opd_er_medical_history_allergy_2.clone()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_opd_er_medical_history_allergy_2.opd_er_order_master_id = Some(1);
            post_api_opd_er_medical_history_allergy_2.version = 2;
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryAllergy.base())
                .json(&vec![post_api_opd_er_medical_history_allergy_1, post_api_opd_er_medical_history_allergy_2])
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErMedicalHistoryScreen => {
            // check query params: view_by is Some(doctor|nurse)
            // kphis_api_handler::opd_er::medical_history::post_opd_er_screen_history
            // POST /api/opd-er/medical-history-screen
            let post_api_opd_er_medical_history_screen = opd_er::medical_history::NurseScreeningHistory::demo();
            assert_eq!(server
                .post(&[EndPoint::OpdErMedicalHistoryScreen.base(), opd_er::medical_history::OpdErMedicalHistoryParams {view_by: None, ..Default::default()}.query_string()].concat())
                .json(&post_api_opd_er_medical_history_screen)
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[EndPoint::OpdErMedicalHistoryScreen.base(), opd_er::medical_history::OpdErMedicalHistoryParams {view_by: Some(String::from("xxx")), ..Default::default()}.query_string()].concat())
                .json(&post_api_opd_er_medical_history_screen)
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[EndPoint::OpdErMedicalHistoryScreen.base(), opd_er::medical_history::OpdErMedicalHistoryParams {view_by: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&post_api_opd_er_medical_history_screen)
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .post(&[EndPoint::OpdErMedicalHistoryScreen.base(), opd_er::medical_history::OpdErMedicalHistoryParams {view_by: Some(String::from("nurse")), ..Default::default()}.query_string()].concat())
                .json(&post_api_opd_er_medical_history_screen)
                .await.status_code(), StatusCode::OK);
        }
        EndPoint::OpdErMedicalHistoryConsult => {
            // check payload field: `opd_er_order_master_id` and `version` has the same value
            // kphis_api_handler::opd_er::medical_history::post_opd_er_consult_history
            // POST /api/opd-er/medical-history-consult
            let post_api_opd_er_medical_history_consult_1 =  opd_er::medical_history::ConsultHistory::demo();
            let mut post_api_opd_er_medical_history_consult_2 = opd_er::medical_history::ConsultHistory::demo();
            post_api_opd_er_medical_history_consult_2.er_consult_id = 2;
            post_api_opd_er_medical_history_consult_2.opd_er_order_master_id = Some(2);
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryConsult.base())
                .json(&vec![post_api_opd_er_medical_history_consult_1.clone(), post_api_opd_er_medical_history_consult_2.clone()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            post_api_opd_er_medical_history_consult_2.opd_er_order_master_id = Some(1);
            post_api_opd_er_medical_history_consult_2.version = 2;
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryConsult.base())
                .json(&vec![post_api_opd_er_medical_history_consult_1, post_api_opd_er_medical_history_consult_2])
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErMedicalHistoryScan => {}
        EndPoint::OpdErMedicalHistoryFt => {}
        EndPoint::OpdErMedReconcile => {
            // check query params: opd_er_order_master_id is Some(not zero)
            // kphis_api_handler::opd_er::med_reconcile::post_opd_er_med_reconcile
            // POST /api/opd-er/med-reconcile
            assert_eq!(server
                .post(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {opd_er_order_master_id: None, ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemSave::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .post(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {opd_er_order_master_id: Some(0), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemSave::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: med_reconciliation_id is Some(not zero)
            // and patch is Some(doctor|pharm|unconfirm|last)
            // kphis_api_handler::opd_er::med_reconcile::patch_opd_er_med_reconcile
            // PATCH /api/opd-er/med-reconcile
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: None, patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(0), patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: None, ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("xxx")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("pharm")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("unconfirm")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("last")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), StatusCode::OK);
            // check query params: med_reconciliation_id or med_reconciliation_item_id is Some
            // kphis_api_handler::opd_er::med_reconcile::delete_opd_er_med_reconcile
            // DELETE /api/opd-er/med-reconcile
            assert_eq!(server
                .delete(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams::default().query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::OpdErMedReconcileNoteId => {}
        EndPoint::OpdErOrderMasterCheckVn => {}
        EndPoint::OpdErOrderMasterId => {}
        EndPoint::OpdErOrderMaster => {}
        EndPoint::OpdErOrderItem => {}
        EndPoint::OpdErOrderOrderId => {}
        EndPoint::OpdErOrderOrder => {}
        EndPoint::OpdErOrderProgressNoteId => {}
        EndPoint::OpdErOrderProgressNote => {}
        EndPoint::OpdErOrderPharmacy => {}
        EndPoint::OpdErShowPatientMainId => {}
        EndPoint::OpdErShowPatientMainVn => {}
        EndPoint::OpdErVitalSignId => {}
        EndPoint::OpdErVitalSign => {
            // check query params: opd_er_order_master_id is Some
            // kphis_api_handler::opd_er::vital_sign::post_opd_er_vital_sign
            // POST /api/opd_er/vital-sign
            let post_api_opd_er_vital_sign = vital_sign::VitalSignSave::demo();
            assert_eq!(server
                .post(&[EndPoint::OpdErVitalSign.base(), vital_sign::VitalSignParams {opd_er_order_master_id: None, ..Default::default()}.query_string()].concat())
                .json(&post_api_opd_er_vital_sign)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: opd_er_order_master_id is Some
            // kphis_api_handler::opd_er::vital_sign::put_opd_er_vital_sign
            // PUT /api/opd_er/vital-sign
            assert_eq!(server
                .put(&[EndPoint::OpdErVitalSign.base(), vital_sign::VitalSignParams {opd_er_order_master_id: None, ..Default::default()}.query_string()].concat())
                .json(&post_api_opd_er_vital_sign)
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::PrescrptionScreen => {
            // check query params: vn is Some
            // kphis_api_handler::prescription::post_prescription_screen
            // POST /api/prescription/screen
            assert_eq!(server
                .post(&EndPoint::PrescrptionScreen.base())
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: vn is Some and action is Some(check|done)
            // kphis_api_handler::prescription::patch_prescription_screen
            // PATCH /api/prescription/screen
            let mut patch_prescription = prescription::PrescriptionScreenPatch::default();
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: None, ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: None, action: Some(String::from("check")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("xxx")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("check")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("done")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::OK);
            // postal need 'postal' in payload
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("postal")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::BAD_REQUEST);
            patch_prescription.postal = Some(prescription::PostalPatch::demo());
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("postal")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::OK);
            // telemed need 'telemed' in payload
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("telemed")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::BAD_REQUEST);
            patch_prescription.telemed = Some(prescription::TelemedPatch::demo());
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("telemed")), ..Default::default()}.query_string()].concat())
                .json(&patch_prescription)
                .await.status_code(), StatusCode::OK);
        }
        EndPoint::ReferNoteVnan => {
            // test vnan MUST EQUAL to vn in payload
            // refer_note::post_referout
            // POST /api/refer-note-vnan/{vnan}
            assert_eq!(server
                .post(&[&EndPoint::ReferNoteVnan.base(), "990001234"].concat())
                .json(&refer_note::ReferNoteSave::demo())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::ReportCustom => {
            // check payload field: `template_name` is unique
            // kphis_api_handler::report::post_custom_report
            // POST /api/report/custom
            let mut post_report_custom = report::CustomReport::demo();
            post_report_custom.template_id = 0;
            assert_eq!(server
                .post(&EndPoint::ReportCustom.base())
                .json(&post_report_custom)
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .post(&EndPoint::ReportCustom.base())
                .json(&post_report_custom)
                .await.status_code(), StatusCode::BAD_REQUEST);
            // check query params: template_id is Some
            // kphis_api_handler::report::delete_custom_report
            // DELETE /api/report/custom
            assert_eq!(server
                .delete(&EndPoint::ReportCustom.base())
                .await.status_code(), StatusCode::BAD_REQUEST);
        }
        EndPoint::ReportRawQuery => {}
        EndPoint::ReportRawTemplateTypeId => {}
        EndPoint::ReportTemplateTypeId => {}
        EndPoint::ScanHisImage => {
            // check query params: key is Some(pe|opd|er|lab) and vn is Some(not empty string)
            // kphis_api_handler::image::scan_his::get_scan_his_image
            // GET /api/scan/his/image
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: None, vn: Some(String::from("651231235959")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: Some(String::from("pe")), vn: None, ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: Some(String::from("xxx")), vn: Some(String::from("651231235959")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::BAD_REQUEST);
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: Some(String::from("pe")), vn: Some(String::from("1234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: Some(String::from("opd")), vn: Some(String::from("1234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: Some(String::from("er")), vn: Some(String::from("1234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::OK);
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: Some(String::from("lab")), vn: Some(String::from("1234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), StatusCode::OK);
        }
        EndPoint::SearchBoxHospText => {}
        EndPoint::SearchBoxMedDuplicate => {}
        EndPoint::SearchBoxMedInteraction => {}
        EndPoint::SearchBoxMedHnText => {}
        EndPoint::SearchBoxOpdVisitModeText => {}
        EndPoint::SearchBoxIvfluidText => {}
        EndPoint::SearchBoxLabText => {}
        EndPoint::SearchBoxPatientText => {}
        EndPoint::SearchBoxXrayText => {}
        EndPoint::SearchDr => {}
        EndPoint::SearchNurse => {}
        EndPoint::SearchPharmacist => {}
        EndPoint::SearchOther => {}
        EndPoint::Sse => {}
        EndPoint::SseGroup => {}
        EndPoint::SseMessage => {}
        EndPoint::User => {}
        EndPoint::UserConfig => {}
        EndPoint::UserRolePrelude => {}
        EndPoint::UserRoleRole => {}
        EndPoint::UserRoleUser => {}
        EndPoint::XrayReportHn => {}
        EndPoint::XrayReadId => {}
        EndPoint::XrayPacsXn => {}
        EndPoint::Unknown => {}
    }
}
