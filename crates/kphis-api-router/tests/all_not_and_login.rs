mod common;

use axum::http::StatusCode;
use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};
use strum::IntoEnumIterator;
use tokio::sync::broadcast;
use ulid::Ulid;

use kphis_api_pdf::test_state::new_test_state;
use kphis_model::{
    ASSETS_PREFIX, IMG_PREFIX, PATH_PREFIX_IMAGE, PATH_PREFIX_THUMB,
    app::VisitTypeId,
    dc_plan, drug_use_duration,
    endpoint::{EndPoint, QueryString},
    focus_list, focus_note, image, index_action, index_monitor, index_plan, ipd, med_reconcile, opd_er, order, pre_admit, pre_order, prescription, progress_note, refer_note, refer_out, report, sse,
    user, vital_sign,
};
use kphis_sqlx_tester::MySqlMocker;

use common::{login, new_test_app};

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore]
async fn api_all_not_and_login() {
    // mocker will crate/insert all databases and tables, so we try to run many test as possible
    let mocker = MySqlMocker::new_all().await;
    let (shutdown_sender, _shutdown_recv) = broadcast::channel(5);
    let state = new_test_state(mocker.db_pool.clone(), shutdown_sender).await;
    let mut server = new_test_app(&state).await;

    // test all NOT LOG-IN
    api_all_run(&server, StatusCode::BAD_REQUEST).await;

    let user = login(&server).await;
    server.add_header("Authorization", &["bearer ", &user.token].concat());

    // test all LOG-IN
    api_all_run(&server, StatusCode::OK).await;
}

// ALL APIs excepts
// - GET event-stream test (GET /api/sse, GET /api/sse-id/{state_id})
// - All method of /api/user API (test at tests/user.rs)
#[rustfmt::skip]
async fn api_all_run(server: &TestServer, status_code: StatusCode) {
    // app::get_app_asset
    // GET /assets
    let _ = server.get(ASSETS_PREFIX).expect_success().await;

    // image::patient::get_patient_image
    // GET /img/patient/{hn}
    let _ = server.get(&[IMG_PREFIX,"/patient/0001234"].concat()).expect_success().await;

    for ep in EndPoint::iter() {
        api_all_match(ep, &server, status_code).await;
    }

    // ** THIS TEST WILL LOGOUT USER, ALWAYS KEEP THIS THE LAST TEST **
    // user::role::post_user_role,
    // POST /api/user-role/user
    assert_eq!(server
        .post(&EndPoint::UserRoleUser.base())
        .json(&user::role::UserRoleSave { loginname: String::from("user"), roles: vec![String::from("MSO")] })
        .await.status_code(), status_code);
}

#[rustfmt::skip]
async fn api_all_match(ep: EndPoint, server: &TestServer, status_code: StatusCode) {
    match ep {
        EndPoint::AvatarOpdEr => {
            // avatar::get_avatar_opd_er
            // GET /api/avatar/opd-er
            assert_eq!(server
                .get(&EndPoint::AvatarOpdEr.base())
                .await.status_code(), status_code);
        }
        EndPoint::AvatarIpd => {
            // avatar::get_avatar_in_ward
            // GET /api/avatar/ward-ward/{ward}
            assert_eq!(server
                .get(&EndPoint::AvatarIpd.base())
                .await.status_code(), status_code);
        }
        EndPoint::DrugUseDuration => {
            // drug_use_duration
            // GET /api/drug-use-duration
            assert_eq!(server
                .get(&EndPoint::DrugUseDuration.base())
                .await.status_code(), status_code);
            // drug_use_duration
            // POST /api/drug-use-duration
            assert_eq!(server
                .post(&EndPoint::DrugUseDuration.base())
                .json(&drug_use_duration::DrugUseDuration::demo())
                .await.status_code(), status_code);
        }
        EndPoint::ExistsKeyId => {
            // app::get_exists
            // GET /api/exists-key-id/{key}/{id}
            assert_eq!(server
                .get(&[&EndPoint::ExistsKeyId.base(),"opd-er-med-reconcile-dr-unconfirm/1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::EmrDateHn => {
            // emr::get_emr_date
            // GET /api/emr/date-hn/{hn}
            assert_eq!(server
                .get(&[&EndPoint::EmrDateHn.base(),"0001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::EmrVisitVn => {
            // emr::get_emr_visit
            // GET /api/emr/visit-vn/{vn}
            assert_eq!(server
                .get(&[&EndPoint::EmrVisitVn.base(),"661231235959"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::HisIptDiagAn => {
            // ipd::his::get_his_ipt_diag,
            // GET /api/his/ipt-diag-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::HisIptDiagAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::HisIptOprtAn => {
            // ipd::his::get_his_ipt_oprt,
            // GET /api/his/ipt-oprt-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::HisIptOprtAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::HisMedPlanIpdAn => {
            // ipd::his::get_med_plan_ipd_remains
            // GET /api/his/med-plan-ipd-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::HisMedPlanIpdAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::HisOperationAdmitAn => {
            // ipd::his::get_ipd_his_opertion_admit,
            // GET /api/his/operation-admit-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::HisOperationAdmitAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::HisReferOutVnan => {
            // refer_out::get_his_referout_data
            // GET /api/his/refer-out-vnan/{vnan}
            assert_eq!(server
                .get(&[&EndPoint::HisReferOutVnan.base(), "660001234"].concat())
                .await.status_code(), status_code);
            // refer_out::post_his_referout
            // POST /api/his/refer-out-vnan/{vnan}
            assert_eq!(server
                .post(&[&EndPoint::HisReferOutVnan.base(), "660001234"].concat())
                .json(&refer_out::HisReferOutSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::Image => {
            // image::file_path::post_image_file
            // POST api/image
            let post_api_image_filename = Ulid::new().to_string();
            let post_api_image_multipart = MultipartForm::new()
                .add_part(PATH_PREFIX_IMAGE, Part::bytes(b"test_image".as_slice()).file_name(&post_api_image_filename).mime_type("image/webp"))
                .add_part(PATH_PREFIX_THUMB, Part::bytes(b"test_thumb".as_slice()).file_name(&post_api_image_filename).mime_type("image/webp"));
            assert_eq!(server
                .post(&EndPoint::Image.base())
                .multipart(post_api_image_multipart)
                .await.status_code(), status_code);
            // image::file_path::patch_image_path
            // PATCH /api/image
            assert_eq!(server
                .patch(&EndPoint::Image.base())
                .json(&image::file_path::ImagePath::demo())
                .await.status_code(), status_code);
            // image::file_path::delete_image_file
            // DELETE /api/image
            assert_eq!(server
                .delete(&EndPoint::Image.base())
                .json(&vec![1u32])
                .await.status_code(), status_code);
        }
        EndPoint::ImageUsage => {
            // image::file_path::post_image_usage
            // POST /api/image-usage
            assert_eq!(server
                .post(&EndPoint::ImageUsage.base())
                .json(&vec![image::file_path::ImagePath::demo()])
                .await.status_code(), status_code);
            // image::file_path::delete_image_usage
            // DELETE /api/image-usage
            assert_eq!(server
                .delete(&EndPoint::ImageUsage.base())
                .json(&vec![1u32])
                .await.status_code(), status_code);
        }
        EndPoint::ImageUsageId => {
            // image::file_path::get_image_usage_id
            // GET /api/image-usage-id/{usage_id}/{usage_key_id}
            assert_eq!(server
                .get(&[&EndPoint::ImageUsageId.base(),"1/1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdAdmissionNoteDrAn => {
            // ipd::admission_note_dr::get_ipd_admission_note_dr
            // GET /api/ipd/admission-note-dr-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdAdmissionNoteDrAn.base(),"660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdAdmissionNoteDrPharmCheckAn => {
            // ipd::admission_note_dr::patch_ipd_pharmacy_check
            // PATCH /api/ipd/admission-note-dr/pharmacy-check-an/{an}
            assert_eq!(server
                .patch(&[&EndPoint::IpdAdmissionNoteDrPharmCheckAn.base(),"660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdAdmissionNoteDr => {
            // ipd::admission_note_dr::post_ipd_admission_note_dr
            // POST /api/ipd/admission-note-dr
            let mut post_api_ipd_admission_note_dr_saver = ipd::admission_note_dr::IpdAdmissionNoteDrSave::demo();
            post_api_ipd_admission_note_dr_saver.admission_note.an = String::from("670001234");
            assert_eq!(server
                .post(&EndPoint::IpdAdmissionNoteDr.base())
                .json(&post_api_ipd_admission_note_dr_saver)
                .await.status_code(), status_code);
            // ipd::admission_note_dr::put_ipd_admission_note_dr
            // PUT /api/ipd/admission-note-dr
            assert_eq!(server
                .put(&EndPoint::IpdAdmissionNoteDr.base())
                .json(&ipd::admission_note_dr::IpdAdmissionNoteDrSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdAdmissionNoteNurseAn => {
            // ipd::admission_note_nurse::get_ipd_admission_note_nurse
            // GET /api/ipd/admission-note-nurse-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdAdmissionNoteNurseAn.base(),"660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdAdmissionNoteNurse => {
            // ipd::admission_note_nurse::post_ipd_admission_note_nurse
            // POST /api/ipd/admission-note-nurse
            let mut post_api_ipd_admission_note_nurse_saver = ipd::admission_note_nurse::IpdNurseAdmissionNote::demo();
            post_api_ipd_admission_note_nurse_saver.an = String::from("670001234");
            assert_eq!(server
                .post(&EndPoint::IpdAdmissionNoteNurse.base())
                .json(&post_api_ipd_admission_note_nurse_saver)
                .await.status_code(), status_code);
            // ipd::admission_note_nurse::put_ipd_admission_note_nurse
            // PUT /api/ipd/admission-note-nurse
            assert_eq!(server
                .put(&EndPoint::IpdAdmissionNoteNurse.base())
                .json(&ipd::admission_note_nurse::IpdNurseAdmissionNote::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdConsult => {
            // ipd::consult::get_ipd_consult_list
            // GET /api/ipd/consult
            assert_eq!(server
                .get(&EndPoint::IpdConsult.base())
                .await.status_code(), status_code);
            // ipd::consult::post_ipd_consult
            // POST /api/ipd/consult
            assert_eq!(server
                .post(&EndPoint::IpdConsult.base())
                .json(&ipd::consult::ConsultSave::demo())
                .await.status_code(), status_code);
            // ipd::consult::delete_ipd_consult_by_id
            // DELETE /api/ipd/consult
            assert_eq!(server
                .delete(&[EndPoint::IpdConsult.base(), ipd::consult::ConsultParams {consult_id: Some(1), version: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdConsultAn => {
            // ipd::consult::get_ipd_consult_by_an
            // GET /api/ipd/consult-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdConsultAn.base(),"660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdConsultId => {
            // ipd::consult::get_ipd_consult_by_id
            // GET /api/ipd/consult-id/{consult_id}
            assert_eq!(server
                .get(&[&EndPoint::IpdConsultId.base(),"1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDcPlanAn => {
            // ipd::dc_plan::get_ipd_dc_plan,
            // GET /api/ipd/dc-plan-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdDcPlanAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
            // ipd::dc_plan::post_ipd_dc_plan,
            // POST /api/ipd/dc-plan-an/{an}
            assert_eq!(server
                .post(&[&EndPoint::IpdDcPlanAn.base(), "660001234"].concat())
                .json(&dc_plan::DischargePlanSave::demo())
                .await.status_code(), status_code);
            // ipd::dc_plan::delete_ipd_dc_plan,
            // DELETE /api/ipd/dc-plan-an/{an}
            assert_eq!(server
                .delete(&[&EndPoint::IpdDcPlanAn.base(), "660001234", &dc_plan::DischargePlanParams {dc_plan_id: Some(1), version: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDcPlanTmpDx => {
            // ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_dx
            // GET /api/ipd/dc-plan-tmp/dx
            assert_eq!(server
                .get(&[EndPoint::IpdDcPlanTmpDx.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_dx
            // POST /api/ipd/dc-plan-tmp/dx
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpDx.base())
                .json(&ipd::dc_plan_tmp::DcPlanTmpDx::demo())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_dx
            // DELETE /api/ipd/dc-plan-tmp/dx
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpDx.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDcPlanTmpMed => {
            // ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_med
            // GET /api/ipd/dc-plan-tmp/med
            assert_eq!(server
                .get(&[EndPoint::IpdDcPlanTmpMed.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_med
            // POST /api/ipd/dc-plan-tmp/med
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpMed.base())
                .json(&ipd::dc_plan_tmp::DcPlanTmpMed::demo())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_med
            // DELETE /api/ipd/dc-plan-tmp/med
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpMed.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDcPlanTmpEnv => {
            // ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_env
            // GET /api/ipd/dc-plan-tmp/env
            assert_eq!(server
                .get(&[EndPoint::IpdDcPlanTmpEnv.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_env
            // POST /api/ipd/dc-plan-tmp/env
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpEnv.base())
                .json(&ipd::dc_plan_tmp::DcPlanTmpEnv::demo())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_env
            // DELETE /api/ipd/dc-plan-tmp/env
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpEnv.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDcPlanTmpTx => {
            // ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_tx
            // GET /api/ipd/dc-plan-tmp/tx
            assert_eq!(server
                .get(&[EndPoint::IpdDcPlanTmpTx.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_tx
            // POST /api/ipd/dc-plan-tmp/tx
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpTx.base())
                .json(&ipd::dc_plan_tmp::DcPlanTmpTx::demo())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_tx
            // DELETE /api/ipd/dc-plan-tmp/tx
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpTx.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDcPlanTmpDiet => {
            // ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_diet
            // GET /api/ipd/dc-plan-tmp/diet
            assert_eq!(server
                .get(&[EndPoint::IpdDcPlanTmpDiet.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_diet
            // POST /api/ipd/dc-plan-tmp/diet
            assert_eq!(server
                .post(&EndPoint::IpdDcPlanTmpDiet.base())
                .json(&ipd::dc_plan_tmp::DcPlanTmpDiet::demo())
                .await.status_code(), status_code);
            // ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_diet
            // DELETE /api/ipd/dc-plan-tmp/diet
            assert_eq!(server
                .delete(&[EndPoint::IpdDcPlanTmpDiet.base(), ipd::dc_plan_tmp::DcPlanTmpParams {id: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDoctorInCharge => {
            // ipd::doctor_in_charge::get_ipd_doctor_in_charge
            // GET /api/ipd/doctor-in-charge
            assert_eq!(server
                .get(&[EndPoint::IpdDoctorInCharge.base(), ipd::doctor_in_charge::DoctorInChargeParams {an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::doctor_in_charge::post_ipd_doctor_in_charge
            // POST /api/ipd/doctor-in-charge
            assert_eq!(server
                .post(&EndPoint::IpdDoctorInCharge.base())
                .json(&ipd::doctor_in_charge::IpdDoctorInCharge::demo())
                .await.status_code(), status_code);
            // ipd::doctor_in_charge::delete_ipd_doctor_in_charge
            // DELETE /api/ipd/doctor-in-charge
            assert_eq!(server
                .delete(&[EndPoint::IpdDoctorInCharge.base(), ipd::doctor_in_charge::DoctorInChargeParams {an: Some(String::from("660001234")), doctor_in_charge_id: Some(1), version: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDocumentDatetimeAn => {
            // ipd::document::get_ipd_document_datetime
            // GET /api/ipd/document/datetime-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdDocumentDatetimeAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDocumentListVnAn => {
            // ipd::document::get_ipd_document_list,
            // GET /api/ipd/document/list-vn-an/{vn}/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdDocumentListVnAn.base(), "661231235959/660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdDocumentScanAn => {
            // ipd::document::get_ipd_document_types,
            // GET /api/ipd/document/scan-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdDocumentScanAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
            // ipd::document::post_ipd_document_type,
            // POST /api/ipd/document/scan-an/{an}
            assert_eq!(server
                .post(&[&EndPoint::IpdDocumentScanAn.base(), "660001234"].concat())
                .json(&1)
                .await.status_code(), status_code);
            // ipd::document::post_ipd_document_type,
            // DELETE /api/ipd/document/scan-an/{an}
            assert_eq!(server
                .delete(&[&EndPoint::IpdDocumentScanAn.base(), "660001234"].concat())
                .json(&1)
                .await.status_code(), status_code);
        }
        EndPoint::IpdFocusListAn => {
            // ipd::focus_list::get_ipd_focus_list,
            // GET /api/ipd/focus-list-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdFocusListAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
            // ipd::focus_list::post_ipd_focus_list,
            // POST /api/ipd/focus-list-an/{an}
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusListAn.base(), "660001234", &focus_list::FocusListSaveParams {hn: Some(String::from("0001234"))}.query_string()].concat())
                .json(&focus_list::FocusListSave::demo())
                .await.status_code(), status_code);
            // ipd::focus_list::delete_ipd_focus_list,
            // DELETE /api/ipd/focus-list-an/{an}
            assert_eq!(server
                .delete(&[&EndPoint::IpdFocusListAn.base(), "660001234", &focus_list::FocusListParams {fclist_id: Some(1), version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdFocusNoteAn => {
            // ipd::focus_note::get_ipd_focus_note,
            // GET /api/ipd/focus-note-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdFocusNoteAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
            // ipd::focus_note::post_ipd_focus_note,
            // POST /api/ipd/focus-note-an/{an}
            assert_eq!(server
                .post(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteSaveParams {hn: Some(String::from("0001234")), ward: Some(String::from("01"))}.query_string()].concat())
                .json(&focus_note::FocusNoteSave::demo())
                .await.status_code(), status_code);
            // ipd::focus_note::delete_ipd_focus_note,
            // DELETE /api/ipd/focus-note-an/{an}
            assert_eq!(server
                .delete(&[&EndPoint::IpdFocusNoteAn.base(), "660001234", &focus_note::FocusNoteParams {fcnote_id: Some(1), version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexActionId => {
            // ipd::index_action::delete_ipd_index_action,
            // DELETE /api/ipd/index-action-id/{action_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdIndexActionId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexAction => {
            // ipd::index_action::post_ipd_index_action,
            // POST /api/ipd/index-action
            assert_eq!(server
                .post(&EndPoint::IpdIndexAction.base())
                .json(&index_action::IndexAction::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexMedPayAn => {
            // ipd::index_plan::get_ipd_index_med_pay,
            // GET /api/ipd/index-med-pay-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdIndexMedPayAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexMonitorId => {
            // ipd::index_monitor::delete_ipd_index_monitor,
            // DELETE /api/ipd/index-monitor-id/{monitor_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdIndexMonitorId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexMonitor => {
            // ipd::index_monitor::post_ipd_index_monitor,
            // POST /api/ipd/index-monitor
            assert_eq!(server
                .post(&EndPoint::IpdIndexMonitor.base())
                .json(&index_monitor::IndexMonitor::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexNoteId => {
            // ipd::index_note::delete_ipd_index_note,
            // DELETE /api/ipd/index-note-id/{nurse_index_note_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdIndexNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexNote => {
            // ipd::index_note::get_ipd_index_note,
            // GET /api/ipd/index-note
            assert_eq!(server
                .get(&EndPoint::IpdIndexNote.base())
                .await.status_code(), status_code);
            // ipd::index_note::post_ipd_index_note,
            // POST /api/ipd/index-note
            assert_eq!(server
                .post(&EndPoint::IpdIndexNote.base())
                .json(&ipd::index_note::IndexNote::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexPlanDateAn => {
            // ipd::index_plan::get_index_plan_date,
            // GET /api/ipd/index-plan-date-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdIndexPlanDateAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexPlanId => {
            // ipd::index_plan::delete_ipd_index_plan,
            // DELETE /api/ipd/index-plan-id/{plan_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdIndexPlanId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIndexPlan => {
            // ipd::index_plan::post_ipd_index_plan,
            // POST /api/ipd/index-plan
            assert_eq!(server
                .post(&EndPoint::IpdIndexPlan.base())
                .json(&index_plan::IndexPlanSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIo => {
            // ipd::io::get_ipd_io_shift,
            // GET /api/ipd/io
            assert_eq!(server
                .get(&[EndPoint::IpdIo.base(), ipd::io::IoParams {an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::io::post_ipd_io_shift,
            // POST /api/ipd/io
            assert_eq!(server
                .post(&EndPoint::IpdIo.base())
                .json(&ipd::io::IoShift::demo())
                .await.status_code(), status_code);
            // ipd::io::delete_ipd_io_shift,
            // DELETE /api/ipd/io
            assert_eq!(server
                .delete(&[EndPoint::IpdIo.base(), ipd::io::IoParams {an: Some(String::from("660001234")), io_id: Some(1), version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdIoDateAn => {
            // ipd::io::get_ipd_io_date,
            // GET /api/ipd/io-date-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdIoDateAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdMedReconcile => {
            // ipd::med_reconcile::get_ipd_med_reconcile,
            // GET /api/ipd/med-reconcile
            assert_eq!(server
                .get(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {an: Some(String::from("660001234")), hn: Some(String::from("0001234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::med_reconcile::post_ipd_med_reconcile,
            // POST /api/ipd/med-reconcile
            assert_eq!(server
                .post(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemSave::demo()])
                .await.status_code(), status_code);
            // ipd::med_reconcile::patch_ipd_med_reconcile,
            // PATCH /api/ipd/med-reconcile
            assert_eq!(server
                .patch(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {an: Some(String::from("660001234")), med_reconciliation_id: Some(1), patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), status_code);
            // ipd::med_reconcile::delete_ipd_med_reconcile,
            // DELETE /api/ipd/med-reconcile
            assert_eq!(server
                .delete(&[EndPoint::IpdMedReconcile.base(), med_reconcile::MedReconciliationParams {an: Some(String::from("660001234")), med_reconciliation_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdMedReconcileHosxpAn => {
            // ipd::med_reconcile::get_ipd_med_reconcile_hosxp,
            // GET /api/ipd/med-reconcile-hosxp-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdMedReconcileHosxpAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdMedReconcileLastDoseAn => {
            // ipd::med_reconcile::get_ipd_med_reconcile_last_dose,
            // GET /api/ipd/med-reconcile-last-dose-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdMedReconcileLastDoseAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdMedReconcileNoteId => {
            // ipd::med_reconcile::get_ipd_med_reconcile_note,
            // GET /api/ipd/med-reconcile-note-id/{med_reconciliation_id}
            assert_eq!(server
                .get(&[&EndPoint::IpdMedReconcileNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
            // ipd::med_reconcile::post_ipd_med_reconcile_note,
            // POST /api/ipd/med-reconcile-note-id/{med_reconciliation_id}
            assert_eq!(server
                .post(&[&EndPoint::IpdMedReconcileNoteId.base(), "1"].concat())
                .json("Note Text")
                .await.status_code(), status_code);
        }
        EndPoint::IpdMedReconcileRemedVisitHn => {
            // ipd::med_reconcile::get_ipd_med_reconcile_remed_visit,
            // GET /api/ipd/med-reconcile-remed-visit-hn/{hn}
            assert_eq!(server
                .get(&[&EndPoint::IpdMedReconcileRemedVisitHn.base(), "0001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdMedReconcileRemedMed => {
            // ipd::med_reconcile::get_ipd_med_reconcile_remed_med,
            // GET /api/ipd/med-reconcile-remed-med
            assert_eq!(server
                .get(&[EndPoint::IpdMedReconcileRemedMed.base(), med_reconcile::MedReconciliationParams {an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdMra => {
            // ipd::mra::get_ipd_mra,
            // GET /api/ipd/mra
            assert_eq!(server
                .get(&EndPoint::IpdMra.base())
                .await.status_code(), status_code);
            // ipd::mra::post_ipd_mra,
            // POST /api/ipd/mra
            assert_eq!(server
                .post(&EndPoint::IpdMra.base())
                .json(&ipd::mra::IpdMra::demo())
                .await.status_code(), status_code);
            // ipd::mra::put_ipd_mra,
            // PUT /api/ipd/mra
            assert_eq!(server
                .put(&EndPoint::IpdMra.base())
                .json(&ipd::mra::IpdMra::demo())
                .await.status_code(), status_code);
            // ipd::mra::delete_ipd_mra,
            // DELETE /api/ipd/mra
            assert_eq!(server
                .delete(&[EndPoint::IpdMra.base(), ipd::mra::MraParams {mra_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderItem => {
            // ipd::order::get_ipd_order_item,
            // GET /api/ipd/order/item
            assert_eq!(server
                .get(&[EndPoint::IpdOrderItem.base(), order::OrderParams {an: Some(String::from("660001234")), view_by: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::order::patch_ipd_order_item,
            // PATCH /api/ipd/order/item
            assert_eq!(server
                .patch(&EndPoint::IpdOrderItem.base())
                .json(&order::OrderItemPatch::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderPrevious => {
            // ipd::order::get_ipd_order_previous,
            // GET /api/ipd/order/previous
            assert_eq!(server
                .get(&EndPoint::IpdOrderPrevious.base())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderOnedayPreviousAn => {
            // ipd::order::get_ipd_order_one_day_previous,
            // GET /api/ipd/order/one-day-previous-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdOrderOnedayPreviousAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderProgressPrevious => {
            // ipd::progress_note::get_ipd_progress_previous,
            // GET /api/ipd/order/progress-previous
            assert_eq!(server
                .get(&[EndPoint::IpdOrderProgressPrevious.base(), progress_note::ProgressNoteParams {an: Some(String::from("660001234")), progress_note_owner_type: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderToHomeMedAn => {
            // ipd::order::get_ipd_home_med_from_cont,
            // GET /api/ipd/order/to-home-med-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdOrderToHomeMedAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderOrderDateAn => {
            // ipd::order::get_ipd_order_date,
            // GET /api/ipd/order/order-date-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdOrderOrderDateAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderOrderId => {
            // ipd::order::delete_ipd_order,
            // DELETE /api/ipd/order/order-id/{order_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdOrderOrderId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderOrder => {
            // ipd::order::get_ipd_order,
            // GET /api/ipd/order/order
            assert_eq!(server
                .get(&EndPoint::IpdOrderOrder.base())
                .await.status_code(), status_code);
            // ipd::order::post_ipd_order,
            // POST /api/ipd/order/order
            let mut post_api_ipd_order_order_saver = order::OrderSave::demo();
            post_api_ipd_order_order_saver.order_id = None;
            assert_eq!(server
                .post(&EndPoint::IpdOrderOrder.base())
                .json(&post_api_ipd_order_order_saver)
                .await.status_code(), status_code);
            // ipd::order::patch_ipd_order,
            // PATCH /api/ipd/order/order
            assert_eq!(server
                .patch(&EndPoint::IpdOrderOrder.base())
                .json(&order::OrderPatch::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderProgressNoteId => {
            // ipd::progress_note::delete_ipd_progress_note,
            // DELETE /api/ipd/order/progress-note-id/{progress_note_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdOrderProgressNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderProgressNote => {
            // ipd::progress_note::get_ipd_progress_note,
            // GET /api/ipd/order/progress-note
            assert_eq!(server
                .get(&EndPoint::IpdOrderProgressNote.base())
                .await.status_code(), status_code);
            // ipd::progress_note::post_ipd_progress_note,
            // POST /api/ipd/order/progress-note
            assert_eq!(server
                .post(&EndPoint::IpdOrderProgressNote.base())
                .json(&progress_note::ProgressNoteSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdOrderPharmacy => {
            // ipd::order::get_ipd_order_pharmacy,
            // GET /api/ipd/order/pharmacy
            assert_eq!(server
                .get(&EndPoint::IpdOrderPharmacy.base())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPasscode => {
            // ipd::passcode::get_ipd_ward_passcode,
            // GET /api/ipd/passcode
            assert_eq!(server
                .get(&EndPoint::IpdPasscode.base())
                .await.status_code(), status_code);
            // ipd::passcode::post_ipd_ward_passcode,
            // POST /api/ipd/passcode
            assert_eq!(server
                .post(&EndPoint::IpdPasscode.base())
                .json(&ipd::passcode::PasscodeGenRequest::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPostAdmitCount => {
            // post_admit::get_ipd_post_admit_count,
            // GET /api/ipd/post-admit/count
            assert_eq!(server
                .get(&EndPoint::IpdPostAdmitCount.base())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPostAdmitList => {
            // post_admit::get_ipd_post_admit_list,
            // GET /api/ipd/post-admit/list
            assert_eq!(server
                .get(&EndPoint::IpdPostAdmitList.base())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreAdmit => {
            // pre_admit::get_ipd_pre_admit_list,
            // GET /api/ipd/pre-admit
            assert_eq!(server
                .get(&EndPoint::IpdPreAdmit.base())
                .await.status_code(), status_code);
            // pre_admit::post_ipd_pre_admit,
            // POST /api/ipd/pre-admit
            assert_eq!(server
                .post(&EndPoint::IpdPreAdmit.base())
                .json(&pre_admit::PreAdmitSave {vn: String::from("671231235959")})
                .await.status_code(), status_code);
            // pre_admit::patch_ipd_pre_admit,
            // PATCH /api/ipd/pre-admit
            assert_eq!(server
                .patch(&EndPoint::IpdPreAdmit.base())
                .json(&pre_admit::PreAdmitPatch::demo_sync_an(String::from("660001234")))
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreOrderMasterId => {
            // pre_order::delete_ipd_pre_order_master,
            // DELETE /api/ipd/pre-order/master-id/{pre_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdPreOrderMasterId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreOrderMaster => {
            // pre_order::get_ipd_pre_order_list,
            // GET /api/ipd/pre-order/master
            assert_eq!(server
                .get(&EndPoint::IpdPreOrderMaster.base())
                .await.status_code(), status_code);
            // pre_order::post_ipd_pre_order_master,
            // POST /api/ipd/pre-order/master
            assert_eq!(server
                .post(&EndPoint::IpdPreOrderMaster.base())
                .json(&pre_order::master::PreOrderMasterSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreOrderInto => {
            // pre_order::post_ipd_pre_order_into,
            // POST /api/ipd/pre-order/into
            assert_eq!(server
                .post(&EndPoint::IpdPreOrderInto.base())
                .json(&pre_order::order::PreOrderIntoCommand::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreOrderOrderId => {
            // pre_order::delete_ipd_pre_order,
            // DELETE /api/ipd/pre-order/order-id/{order_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdPreOrderOrderId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreOrderOrder => {
            // pre_order::get_ipd_pre_order,
            // GET /api/ipd/pre-order/order
            assert_eq!(server
                .get(&[EndPoint::IpdPreOrderOrder.base(), pre_order::order::PreOrderParams {order_type: Some(String::from("oneday")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // pre_order::post_ipd_pre_order,
            // POST /api/ipd/pre-order/order
            assert_eq!(server
                .post(&EndPoint::IpdPreOrderOrder.base())
                .json(&pre_order::order::PreOrderSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreOrderProgressNoteId => {
            // pre_order::delete_ipd_pre_progress_note,
            // DELETE /api/ipd/pre-order/progress-note-id/{progress_note_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdPreOrderProgressNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdPreOrderProgressNote => {
            // pre_order::get_ipd_pre_progress_note,
            // GET /api/ipd/pre-order/progress-note
            assert_eq!(server
                .get(&EndPoint::IpdPreOrderProgressNote.base())
                .await.status_code(), status_code);

            // pre_order::post_ipd_pre_progress_note,
            // POST /api/ipd/pre-order/progress-note
            assert_eq!(server
                .post(&EndPoint::IpdPreOrderProgressNote.base())
                .json(&pre_order::progress_note::PreProgressNoteSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdShowPatientMainAn => {
            // ipd::show_patient_main::get_ipd_show_patient_main,
            // GET /api/ipd/show-patient-main-an/{an}
            assert_eq!(server
                .get(&[&EndPoint::IpdShowPatientMainAn.base(), "660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdSummary => {
            // ipd::summary::get_ipd_summary,
            // GET /api/ipd/summary
            assert_eq!(server
                .get(&[EndPoint::IpdSummary.base(), ipd::summary::SummaryParams {an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::summary::post_ipd_summary,
            // POST /api/ipd/summary
            assert_eq!(server
                .post(&EndPoint::IpdSummary.base())
                .json(&ipd::summary::SummarySave::demo())
                .await.status_code(), status_code);
            // ipd::summary::patch_ipd_summary,
            // PATCH /api/ipd/summary
            assert_eq!(server
                .patch(&EndPoint::IpdSummary.base())
                .json(&ipd::summary::SummaryCodeSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdSummaryAudit => {
            // ipd::summary_audit::get_ipd_summary_audit,
            // GET /api/ipd/summary-audit
            assert_eq!(server
                .get(&[EndPoint::IpdSummaryAudit.base(), ipd::summary_audit::SummaryAuditParams {an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // ipd::summary_audit::post_ipd_summary_audit,
            // POST /api/ipd/summary-audit
            assert_eq!(server
                .post(&EndPoint::IpdSummaryAudit.base())
                .json(&ipd::summary_audit::SummaryAudit::demo())
                .await.status_code(), status_code);
            // ipd::summary_audit::delete_ipd_summary_audit,
            // DELETE /api/ipd/summary-audit
            assert_eq!(server
                .delete(&[EndPoint::IpdSummaryAudit.base(), ipd::summary_audit::SummaryAuditParams {summary_audit_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdSummaryNoteId => {
            // ipd::summary::get_ipd_summary_note,
            // GET /api/ipd/summary-note-id/{summary_id}
            assert_eq!(server
                .get(&[&EndPoint::IpdSummaryNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
            // ipd::summary::post_ipd_summary_note,
            // POST /api/ipd/summary-note-id/{summary_id}
            assert_eq!(server
                .post(&[&EndPoint::IpdSummaryNoteId.base(), "1"].concat())
                .json(&ipd::summary::SummaryNoteSave::demo())
                .await.status_code(), status_code);
            // ipd::summary::patch_ipd_summary_note,
            // PATCH /api/ipd/summary-note-id/{summary_id}
            assert_eq!(server
                .patch(&[&EndPoint::IpdSummaryNoteId.base(), "1"].concat())
                .json(&ipd::summary::SummaryNoteSave::demo())
                .await.status_code(), status_code);
            // ipd::summary::delete_ipd_summary_note,
            // DELETE /api/ipd/summary-note-id/{summary_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdSummaryNoteId.base(), "1"].concat())
                .json(&ipd::summary::SummaryNoteSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::IpdSummaryStatusId => {
            // ipd::summary::get_ipd_summary_status,
            // GET /api/ipd/summary-status-id/{summary_id}
            assert_eq!(server
                .get(&[&EndPoint::IpdSummaryStatusId.base(), "1"].concat())
                .await.status_code(), status_code);
            // ipd::summary::post_ipd_summary_status,
            // PUT /api/ipd/summary-status-id/{summary_id}
            assert_eq!(server
                .put(&[&EndPoint::IpdSummaryStatusId.base(), "1"].concat())
                .json(&ipd::summary::SummaryStatus { status: Some(String::from("review")) })
                .await.status_code(), status_code);
        }
        EndPoint::IpdTmpGroup => {
            // ipd::tmp::get_ipd_tmp_group,
            // GET /api/ipd/tmp/group
            assert_eq!(server
                .get(&EndPoint::IpdTmpGroup.base())
                .await.status_code(), status_code);
            // ipd::tmp::post_ipd_tmp_group,
            // POST /api/ipd/tmp/group
            assert_eq!(server
                .post(&EndPoint::IpdTmpGroup.base())
                .json(&ipd::tmp::TmpGroup::demo())
                .await.status_code(), status_code);
            // ipd::tmp::delete_ipd_tmp_group,
            // DELETE /api/ipd/tmp/group
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpGroup.base(), ipd::tmp::TmpParams {smp_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdTmpSubgroup => {
            // ipd::tmp::get_ipd_subgroup,
            // GET /api/ipd/tmp/subgroup
            assert_eq!(server
                .get(&EndPoint::IpdTmpSubgroup.base())
                .await.status_code(), status_code);
            // ipd::tmp::post_ipd_subgroup,
            // POST /api/ipd/tmp/subgroup
            assert_eq!(server
                .post(&EndPoint::IpdTmpSubgroup.base())
                .json(&ipd::tmp::TmpSubGroup::demo())
                .await.status_code(), status_code);
            // ipd::tmp::delete_ipd_subgroup,
            // DELETE /api/ipd/tmp/subgroup
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpSubgroup.base(), ipd::tmp::TmpParams {smp_id: Some(1), subgroup: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdTmpFocus => {
            // ipd::tmp::get_ipd_focus,
            // GET /api/ipd/tmp/focus
            assert_eq!(server
                .get(&EndPoint::IpdTmpFocus.base())
                .await.status_code(), status_code);
            // ipd::tmp::post_ipd_focus,
            // POST /api/ipd/tmp/focus
            assert_eq!(server
                .post(&EndPoint::IpdTmpFocus.base())
                .json(&ipd::tmp::TmpFocus::demo())
                .await.status_code(), status_code);
            // ipd::tmp::delete_ipd_focus,
            // DELETE /api/ipd/tmp/focus
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpFocus.base(), ipd::tmp::TmpParams {id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdTmpGoal => {
            // ipd::tmp::get_ipd_goal,
            // GET /api/ipd/tmp/goal
            assert_eq!(server
                .get(&EndPoint::IpdTmpGoal.base())
                .await.status_code(), status_code);
            // ipd::tmp::post_ipd_goal,
            // POST /api/ipd/tmp/goal
            assert_eq!(server
                .post(&EndPoint::IpdTmpGoal.base())
                .json(&ipd::tmp::TmpGoal::demo())
                .await.status_code(), status_code);
            // ipd::tmp::delete_ipd_goal,
            // DELETE /api/ipd/tmp/goal
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpGoal.base(), ipd::tmp::TmpParams {id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdTmpIntvt => {
            // ipd::tmp::get_ipd_intvt,
            // GET /api/ipd/tmp/intvt
            assert_eq!(server
                .get(&EndPoint::IpdTmpIntvt.base())
                .await.status_code(), status_code);
            // ipd::tmp::post_ipd_intvt,
            // POST /api/ipd/tmp/intvt
            assert_eq!(server
                .post(&EndPoint::IpdTmpIntvt.base())
                .json(&ipd::tmp::TmpIntvt::demo())
                .await.status_code(), status_code);
            // ipd::tmp::delete_ipd_intvt,
            // DELETE /api/ipd/tmp/intvt
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpIntvt.base(), ipd::tmp::TmpParams {id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdTmpDlc => {
            // ipd::tmp::get_ipd_dlc,
            // GET /api/ipd/tmp/dlc
            assert_eq!(server
                .get(&EndPoint::IpdTmpDlc.base())
                .await.status_code(), status_code);
            // ipd::tmp::post_ipd_dlc,
            // POST /api/ipd/tmp/dlc
            assert_eq!(server
                .post(&EndPoint::IpdTmpDlc.base())
                .json(&ipd::tmp::TmpDlc::demo())
                .await.status_code(), status_code);
            // ipd::tmp::delete_ipd_dlc,
            // DELETE /api/ipd/tmp/dlc
            assert_eq!(server
                .delete(&[EndPoint::IpdTmpDlc.base(), ipd::tmp::TmpParams {id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdVitalSignId => {
            // ipd::vital_sign::delete_ipd_vital_sign,
            // DELETE /api/ipd/vital-sign-id/{vs_id}
            assert_eq!(server
                .delete(&[&EndPoint::IpdVitalSignId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::IpdVitalSign => {
            // ipd::vital_sign::get_ipd_vital_sign,
            // GET /api/ipd/vital-sign
            assert_eq!(server
                .get(&EndPoint::IpdVitalSign.base())
                .await.status_code(), status_code);
            // ipd::vital_sign::post_ipd_vital_sign,
            // POST /api/ipd/vital-sign
            assert_eq!(server
                .post(&[EndPoint::IpdVitalSign.base(), vital_sign::VitalSignParams {hn: Some(String::from("0001234")), an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .json(&vital_sign::VitalSignSave::demo())
                .await.status_code(), status_code);
            // ipd::vital_sign::put_ipd_vital_sign,
            // PUT /api/ipd/vital-sign
            assert_eq!(server
                .put(&[EndPoint::IpdVitalSign.base(), vital_sign::VitalSignParams {hn: Some(String::from("0001234")), an: Some(String::from("660001234")), ..Default::default()}.query_string()].concat())
                .json(&vital_sign::VitalSignSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::LabHead => {
            // lab::get_lab_head,
            // GET /api/lab/head
            assert_eq!(server
                .get(&EndPoint::LabHead.base())
                .await.status_code(), status_code);
        }
        EndPoint::LabItem => {
            // lab::get_lab_item,
            // GET /api/lab/item
            assert_eq!(server
                .get(&EndPoint::LabItem.base())
                .await.status_code(), status_code);
        }
        EndPoint::LabReadId => {
            // lab::post_lab_read,
            // POST /api/lab/read-id/{lab_order_number}
            assert_eq!(server
                .post(&[&EndPoint::LabReadId.base(), "1"].concat())
                .await.status_code(), status_code);
            // lab::delete_lab_read,
            // DELETE /api/lab/read-id/{lab_order_number}
            assert_eq!(server
                .delete(&[&EndPoint::LabReadId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::LabWbcKeyValue => {
            // lab::get_wbc_band,
            // GET /api/lab/wbc-key-value/{key}/{value}
            assert_eq!(server
                .get(&[&EndPoint::LabWbcKeyValue.base(), "hn/0001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::MedReconcileHn => {
            // med_reconciliation::get_med_reconciliation_head,
            // GET /api/med-reconcile-hn/{hn}
            assert_eq!(server
                .get(&[&EndPoint::MedReconcileHn.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErDcPlanId => {
            // opd_er::dc_plan::get_opd_er_dc_plan,
            // GET /api/opd-er/dc-plan-id/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErDcPlanId.base(), "1"].concat())
                .await.status_code(), status_code);
            // opd_er::dc_plan::post_opd_er_dc_plan,
            // POST /api/opd-er/dc-plan-id/{opd_er_order_master_id}
            assert_eq!(server
                .post(&[&EndPoint::OpdErDcPlanId.base(), "1"].concat())
                .json(&dc_plan::DischargePlanSave::demo())
                .await.status_code(), status_code);
            // opd_er::dc_plan::delete_opd_er_dc_plan,
            // DELETE /api/opd-er/dc-plan-id/{opd_er_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErDcPlanId.base(), "1", &dc_plan::DischargePlanParams {dc_plan_id: Some(1), version: Some(1)}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErDocumentListVnId => {
            // opd_er::document::get_opd_er_document_list,
            // GET /api/opd-er/document/list-vn-id/{vn}/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErDocumentListVnId.base(), "661231235959/1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErDocumentScanId => {
            // opd_er::document::get_opd_er_document_types,
            // GET /api/opd-er/document/scan-id/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErDocumentScanId.base(), "1"].concat())
                .await.status_code(), status_code);
            // ipd::document::post_opd_er_document_type,
            // POST /api/opd-er/document/scan-id/{opd_er_order_master_id}
            assert_eq!(server
                .post(&[&EndPoint::OpdErDocumentScanId.base(), "1"].concat())
                .json(&1)
                .await.status_code(), status_code);
            // ipd::document::post_opd_er_document_type,
            // DELETE /api/opd-er/document/scan-id/{opd_er_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErDocumentScanId.base(), "1"].concat())
                .json(&1)
                .await.status_code(), status_code);
        }
        EndPoint::OpdErFocusListId => {
            // opd_er::focus_list::get_opd_er_focus_list,
            // GET /api/opd-er/focus-list-id/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErFocusListId.base(), "1"].concat())
                .await.status_code(), status_code);
            // opd_er::focus_list::post_opd_er_focus_list,
            // POST /api/opd-er/focus-list-id/{opd_er_order_master_id}
            assert_eq!(server
                .post(&[&EndPoint::OpdErFocusListId.base(), "1"].concat())
                .json(&focus_list::FocusListSave::demo())
                .await.status_code(), status_code);
            // opd_er::focus_list::delete_opd_er_focus_list,
            // DELETE /api/opd-er/focus-list-id/{opd_er_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErFocusListId.base(), "1", &focus_list::FocusListParams {fclist_id: Some(1), version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErFocusNoteId => {
            // opd_er::focus_note::get_opd_er_focus_note,
            // GET /api/opd-er/focus-note-id/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErFocusNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
            // opd_er::focus_note::post_opd_er_focus_note,
            // POST /api/opd-er/focus-note-id/{opd_er_order_master_id}
            assert_eq!(server
                .post(&[&EndPoint::OpdErFocusNoteId.base(), "1"].concat())
                .json(&focus_note::FocusNoteSave::demo())
                .await.status_code(), status_code);
            // opd_er::focus_note::delete_opd_er_focus_note,
            // DELETE /api/opd-er/focus-note-id/{opd_er_order_master_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErFocusNoteId.base(), "1", &focus_note::FocusNoteParams {fcnote_id: Some(1), version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErHisMedVn => {
            // opd_er::hosxp_med::get_opd_med,
            // GET /api/opd-er/his-med-vn/{vn}
            assert_eq!(server
                .get(&[&EndPoint::OpdErHisMedVn.base(), "661231235959"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIndexActionId => {
            // opd_er::index_action::delete_opd_er_index_action,
            // DELETE /api/opd-er/index-action-id/{action_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErIndexActionId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIndexAction => {
            // opd_er::index_action::post_opd_er_index_action,
            // POST /api/opd-er/index-action
            assert_eq!(server
                .post(&EndPoint::OpdErIndexAction.base())
                .json(&index_action::IndexAction::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIndexMonitorId => {
            // opd_er::index_monitor::delete_opd_er_index_monitor,
            // DELETE /api/opd-er/index-monitor-id/{monitor_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErIndexMonitorId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIndexMonitor => {
            // opd_er::index_monitor::post_opd_er_index_monitor,
            // POST /api/opd-er/index-monitor
            assert_eq!(server
                .post(&EndPoint::OpdErIndexMonitor.base())
                .json(&index_monitor::IndexMonitor::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIndexPlanId => {
            // opd_er::index_plan::delete_opd_er_index_plan,
            // DELETE /api/opd-er/index-plan-id/{plan_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErIndexPlanId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIndexPlan => {
            // opd_er::index_plan::post_opd_er_index_plan,
            // POST /api/opd-er/index-plan
            assert_eq!(server
                .post(&EndPoint::OpdErIndexPlan.base())
                .json(&index_plan::IndexPlanSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIo => {
            // opd_er::io::get_opd_er_io_shift,
            // GET /api/opd-er/io
            assert_eq!(server
                .get(&[EndPoint::OpdErIo.base(), ipd::io::IoParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::io::post_opd_er_io_shift,
            // POST /api/opd-er/io
            assert_eq!(server
                .post(&EndPoint::OpdErIo.base())
                .json(&ipd::io::IoShift::demo())
                .await.status_code(), status_code);
            // opd_er::io::delete_opd_er_io_shift,
            // DELETE /api/opd-er/io
            assert_eq!(server
                .delete(&[EndPoint::OpdErIo.base(), ipd::io::IoParams {io_id: Some(1), version: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErIoDateId => {
            // opd_er::io::get_opd_er_io_date,
            // GET /api/opd-er/io-date-id/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErIoDateId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedicalHistory => {
            // opd_er::medical_history::get_opd_er_medical_history,
            // GET /api/opd-er/medical-history
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistory.base(), opd_er::medical_history::OpdErMedicalHistoryParams {only_opdscreen: Some(true), vn: Some(String::from("661231235959")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedicalHistoryTrauma => {
            // opd_er::medical_history::get_opd_er_trauma_history,
            // GET /api/opd-er/medical-history-trauma
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistoryTrauma.base(), opd_er::medical_history::OpdErMedicalHistoryParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::medical_history::post_opd_er_trauma_history,
            // POST /api/opd-er/medical-history-trauma
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryTrauma.base())
                .json(&opd_er::medical_history::TraumaHistory::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedicalHistoryAllergy => {
            // opd_er::medical_history::get_opd_er_allergy_history,
            // GET /api/opd-er/medical-history-allergy
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistoryAllergy.base(), opd_er::medical_history::OpdErMedicalHistoryParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::medical_history::post_opd_er_allergy_history,
            // POST /api/opd-er/medical-history-allergy
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryAllergy.base())
                .json(&vec![opd_er::medical_history::AllergyHistory::demo()])
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedicalHistoryScreen => {
            // opd_er::medical_history::get_opd_er_screen_history,
            // GET /api/opd-er/medical-history-screen
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistoryScreen.base(), opd_er::medical_history::OpdErMedicalHistoryParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::medical_history::post_opd_er_screen_history,
            // POST /api/opd-er/medical-history-screen
            assert_eq!(server
                .post(&[EndPoint::OpdErMedicalHistoryScreen.base(), opd_er::medical_history::OpdErMedicalHistoryParams {view_by: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&opd_er::medical_history::NurseScreeningHistory::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedicalHistoryConsult => {
            // opd_er::medical_history::get_opd_er_consult_history,
            // GET /api/opd-er/medical-history-consult
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistoryConsult.base(), opd_er::medical_history::OpdErMedicalHistoryParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::medical_history::post_opd_er_consult_history,
            // POST /api/opd-er/medical-history-consult
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryConsult.base())
                .json(&vec![opd_er::medical_history::ConsultHistory::demo()])
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedicalHistoryScan => {
            // opd_er::medical_history::get_opd_er_scan_history,
            // GET /api/opd-er/medical-history-scan
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistoryScan.base(), opd_er::medical_history::OpdErMedicalHistoryParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::medical_history::post_opd_er_scan_history,
            // POST /api/opd-er/medical-history-scan
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryScan.base())
                .json(&opd_er::medical_history::ScanHistory::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedicalHistoryFt => {
            // opd_er::medical_history::get_opd_er_ft_history,
            // GET /api/opd-er/medical-history-ft
            assert_eq!(server
                .get(&[EndPoint::OpdErMedicalHistoryFt.base(), opd_er::medical_history::OpdErMedicalHistoryParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::medical_history::post_opd_er_ft_history,
            // POST /api/opd-er/medical-history-ft
            assert_eq!(server
                .post(&EndPoint::OpdErMedicalHistoryFt.base())
                .json(&opd_er::medical_history::SetFtHistory::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedReconcile => {
            // opd_er::med_reconcile::get_opd_er_med_reconcile,
            // GET /api/opd-er/med-reconcile
            assert_eq!(server
                .get(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {opd_er_order_master_id: Some(1), hn: Some(String::from("0001234")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::med_reconcile::post_opd_er_med_reconcile,
            // POST /api/opd-er/med-reconcile
            assert_eq!(server
                .post(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemSave::demo()])
                .await.status_code(), status_code);
            // opd_er::med_reconcile::patch_opd_er_med_reconcile,
            // PATCH /api/opd-er/med-reconcile
            assert_eq!(server
                .patch(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), patch: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .json(&vec![med_reconcile::MedReconciliationItemPatch::demo()])
                .await.status_code(), status_code);
            // opd_er::med_reconcile::delete_opd_er_med_reconcile,
            // DELETE /api/opd-er/med-reconcile
            assert_eq!(server
                .delete(&[EndPoint::OpdErMedReconcile.base(), med_reconcile::MedReconciliationParams {med_reconciliation_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErMedReconcileNoteId => {
            // opd_er::med_reconcile::get_opd_er_med_reconcile_note,
            // GET /api/opd-er/med-reconcile-note-id/{med_reconciliation_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErMedReconcileNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
            // opd_er::med_reconcile::post_opd_er_med_reconcile_note,
            // POST /api/opd-er/med-reconcile-note-id/{med_reconciliation_id}
            assert_eq!(server
                .post(&[&EndPoint::OpdErMedReconcileNoteId.base(), "1"].concat())
                .json("Note Text")
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderMasterCheckVn => {
            // opd_er::order_master::get_opd_er_order_master_check,
            // GET /api/opd-er/order/master/check-vn/{vn}
            assert_eq!(server
                .get(&[&EndPoint::OpdErOrderMasterCheckVn.base(), "670111111111"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderMasterId => {
            // opd_er::order_master::get_opd_er_order_master,
            // GET /api/opd-er/order/master-id/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErOrderMasterId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderMaster => {
            // opd_er::order_master::get_opd_er_order_master_list,
            // GET /api/opd-er/order/master
            assert_eq!(server
                .get(&EndPoint::OpdErOrderMaster.base())
                .await.status_code(), status_code);
            // opd_er::order_master::post_opd_er_order_master,
            // POST /api/opd-er/order/master
            assert_eq!(server
                .post(&EndPoint::OpdErOrderMaster.base())
                .json(&opd_er::order_master::OpdErOrderMasterSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderItem => {
            // opd_er::order::get_opd_er_order_item,
            // GET /api/opd-er/order/item
            assert_eq!(server
                .get(&[EndPoint::OpdErOrderItem.base(), order::OrderParams {opd_er_order_master_id: Some(1), view_by: Some(String::from("doctor")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // opd_er::order::patch_opd_er_order_item,
            // PATCH /api/opd-er/order/item
            assert_eq!(server
                .patch(&EndPoint::OpdErOrderItem.base())
                .json(&order::OrderItemPatch::demo())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderOrderId => {
            // opd_er::order::delete_opd_er_order,
            // DELETE /api/opd-er/order/order-id/{order_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErOrderOrderId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderOrder => {
            // opd_er::order::get_opd_er_order,
            // GET /api/opd-er/order/order
            assert_eq!(server
                .get(&EndPoint::OpdErOrderOrder.base())
                .await.status_code(), status_code);
            // opd_er::order::post_opd_er_order,
            // POST /api/opd-er/order/order
            let mut post_api_opd_er_order_order_saver = order::OrderSave::demo();
            post_api_opd_er_order_order_saver.visit_type = VisitTypeId::OpdEr(String::from("661231235959"), 1);
            post_api_opd_er_order_order_saver.order_id = None;
            assert_eq!(server
                .post(&EndPoint::OpdErOrderOrder.base())
                .json(&post_api_opd_er_order_order_saver)
                .await.status_code(), status_code);
            // opd_er::order::patch_opd_er_order,
            // PATCH /api/opd-er/order/order
            assert_eq!(server
                .patch(&EndPoint::OpdErOrderOrder.base())
                .json(&order::OrderPatch::demo())
                .await.status_code(), status_code);
                }
        EndPoint::OpdErOrderProgressNoteId => {
            // opd_er::progress_note::delete_opd_er_progress_note,
            // DELETE /api/opd-er/order/progress-note-id/{progress_note_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErOrderProgressNoteId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderProgressNote => {
            // opd_er::progress_note::get_opd_er_progress_note,
            // GET /api/opd-er/order/progress-note
            assert_eq!(server
                .get(&EndPoint::OpdErOrderProgressNote.base())
                .await.status_code(), status_code);
            // opd_er::progress_note::post_opd_er_progress_note,
            // POST /api/opd-er/order/progress-note
            let mut post_api_opd_er_order_progress_note_saver = progress_note::ProgressNoteSave::demo();
            post_api_opd_er_order_progress_note_saver.visit_type = VisitTypeId::OpdEr(String::from("661231235959"), 1);
            assert_eq!(server
                .post(&EndPoint::OpdErOrderProgressNote.base())
                .json(&post_api_opd_er_order_progress_note_saver)
                .await.status_code(), status_code);
        }
        EndPoint::OpdErOrderPharmacy => {
            // opd_er::order::get_opd_er_order_pharmacy,
            // GET /api/opd-er/order/pharmacy
            assert_eq!(server
                .get(&EndPoint::OpdErOrderPharmacy.base())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErShowPatientMainId => {
            // opd_er::show_patient_main::get_opd_er_show_patient_main_id,
            // GET /api/opd-er/show-patient-main-id/{opd_er_order_master_id}
            assert_eq!(server
                .get(&[&EndPoint::OpdErShowPatientMainId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErShowPatientMainVn => {
            // opd_er::show_patient_main::get_opd_er_show_patient_main_vn,
            // GET /api/opd-er/show-patient-main-vn/{vn}
            assert_eq!(server
                .get(&[&EndPoint::OpdErShowPatientMainVn.base(), "661231235959"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErVitalSignId => {
            // opd_er::vital_sign::delete_opd_er_vital_sign,
            // DELETE /api/opd-er/vital-sign-id/{vs_id}
            assert_eq!(server
                .delete(&[&EndPoint::OpdErVitalSignId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::OpdErVitalSign => {
            // opd_er::vital_sign::get_opd_er_vital_sign,
            // GET /api/opd-er/vital-sign
            assert_eq!(server
                .get(&EndPoint::OpdErVitalSign.base())
                .await.status_code(), status_code);
            // opd_er::vital_sign::post_opd_er_vital_sign,
            // POST /api/opd-er/vital-sign
            assert_eq!(server
                .post(&[EndPoint::OpdErVitalSign.base(), vital_sign::VitalSignParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .json(&vital_sign::VitalSignSave::demo())
                .await.status_code(), status_code);
            // opd_er::vital_sign::put_opd_er_vital_sign,
            // PUT /api/opd-er/vital-sign
            assert_eq!(server
                .put(&[EndPoint::OpdErVitalSign.base(), vital_sign::VitalSignParams {opd_er_order_master_id: Some(1), ..Default::default()}.query_string()].concat())
                .json(&vital_sign::VitalSignSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::PrescrptionScreen => {
            // prescription::get_prescription_screen,
            // GET /api/prescription/screen
            assert_eq!(server
                .get(&EndPoint::PrescrptionScreen.base())
                .await.status_code(), status_code);
            // prescription::post_prescription_screen,
            // POST /api/prescription/screen
            assert_eq!(server
                .post(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
            // prescription::patch_prescription_screen,
            // PATCH /api/prescription/screen
            assert_eq!(server
                .patch(&[EndPoint::PrescrptionScreen.base(), prescription::PrescriptionScreenParams{vn: Some(String::from("661231235959")), action: Some(String::from("check")), ..Default::default()}.query_string()].concat())
                .json(&prescription::PrescriptionScreenPatch::demo())
                .await.status_code(), status_code);
        }
        EndPoint::ReferNoteVnan => {
            // refer_note::get_refernote
            // GET /api/refer-note-vnan/{vnan}
            assert_eq!(server
                .get(&[&EndPoint::ReferNoteVnan.base(), "660001234"].concat())
                .await.status_code(), status_code);
            // refer_note::post_refernote
            // POST /api/refer-note-vnan/{vnan}
            assert_eq!(server
                .post(&[&EndPoint::ReferNoteVnan.base(), "660001234"].concat())
                .json(&refer_note::ReferNoteSave::demo())
                .await.status_code(), status_code);
        }
        EndPoint::ReportCustom => {
            // report::get_custom_report,
            // GET /api/report/custom
            assert_eq!(server
                .get(&EndPoint::ReportCustom.base())
                .await.status_code(), status_code);
            // report::post_custom_report,
            // POST /api/report/custom
            assert_eq!(server
                .post(&EndPoint::ReportCustom.base())
                .json(&report::CustomReport::demo())
                .await.status_code(), status_code);
            // report::delete_custom_report,
            // DELETE /api/report/custom
            assert_eq!(server
                .delete(&[EndPoint::ReportCustom.base(), report::ReportTemplateParams{template_id: Some(1), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::ReportRawQuery => {
            // report::post_query_to_json_string,
            // POST /api/report/raw-query
            assert_eq!(server
                .post(&EndPoint::ReportRawQuery.base())
                .json(&report::ReportQuery::demo())
                .await.status_code(), status_code);
        }
        EndPoint::ReportRawTemplateTypeId => {
            // pdf::get_raw_single_template,
            // GET /api/report/raw-template-type-id/{template}/{type}/{id}  
            assert_eq!(server
                .get(&[&EndPoint::ReportRawTemplateTypeId.base(), "ipd-consult/system/660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::ReportTemplateTypeId => {
            // pdf::get_single_template,
            // GET /api/report/template-type-id/{template}/{type}/{id}
            assert_eq!(server
                .get(&[&EndPoint::ReportTemplateTypeId.base(), "ipd-consult/system/660001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::ScanHisImage => {
            // image::scan_his::get_scan_his_image
            // GET /api/scan/his/image
            assert_eq!(server
                .get(&[EndPoint::ScanHisImage.base(), image::scan_his::ScanImageParams {key: Some(String::from("pe")), vn: Some(String::from("651231235959")), ..Default::default()}.query_string()].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxHospText => {
            // search::searchbox::get_hosp_searchbox,
            // GET /api/search/box/hosp-text/{search_text}
            assert_eq!(server
                .get(&[&EndPoint::SearchBoxHospText.base(), "what"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxMedDuplicate => {
            // search::searchbox::get_drug_duplication_check,
            // GET /api/search/box/med/duplicate
            assert_eq!(server
                .get(&EndPoint::SearchBoxMedDuplicate.base())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxMedInteraction => {
            // search::searchbox::get_drug_interaction_check,
            // GET /api/search/box/med/interaction
            assert_eq!(server
                .get(&EndPoint::SearchBoxMedInteraction.base())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxMedHnText => {
            // search::searchbox::get_med_searchbox,
            // GET /api/search/box/med-hn-text/{hn}/{search_text}
            assert_eq!(server
                .get(&[&EndPoint::SearchBoxMedHnText.base(), "0001234/what"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxOpdVisitModeText => {
            // search::searchbox::get_opd_visit_searchbox,
            // GET /api/search/box/opd-visit-mode-text/{mode}/{search_text}
            assert_eq!(server
                .get(&[&EndPoint::SearchBoxOpdVisitModeText.base(), "hn/0001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxIvfluidText => {
            // search::searchbox::get_ivfluid_searchbox,
            // GET /api/search/box/ivfluid-text/{search_text}
            assert_eq!(server
                .get(&[&EndPoint::SearchBoxIvfluidText.base(), "what"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxLabText => {
            // search::searchbox::get_lab_searchbox,
            // GET /api/search/box/lab-text/{search_text}
            assert_eq!(server
                .get(&[&EndPoint::SearchBoxLabText.base(), "what"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxPatientText => {
            // search::searchbox::get_patient_searchbox,
            // GET /api/search/box/patient-text/{search_text}
            assert_eq!(server
                .get(&[&EndPoint::SearchBoxPatientText.base(), "what"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchBoxXrayText => {
            // search::searchbox::get_xray_searchbox,
            // GET /api/search/box/xray-text/{search_text}
            assert_eq!(server
                .get(&[&EndPoint::SearchBoxXrayText.base(), "what"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::SearchDr => {
            // search::ipd_search_patient_dr::get_ipd_dr_search_patient,
            // GET /api/search/dr
            assert_eq!(server
                .get(&EndPoint::SearchDr.base())
                .await.status_code(), status_code);
        }
        EndPoint::SearchNurse => {
            // search::ipd_search_patient_nurse::get_ipd_nurse_search_patient,
            // GET /api/search/nurse
            assert_eq!(server
                .get(&EndPoint::SearchNurse.base())
                .await.status_code(), status_code);
        }
        EndPoint::SearchPharmacist => {
            // search::ipd_search_patient_pharmacist::get_ipd_pharmacist_search_patient,
            // GET /api/search/pharmacist
            assert_eq!(server
                .get(&EndPoint::SearchPharmacist.base())
                .await.status_code(), status_code);
        }
        EndPoint::SearchOther => {
            // search::ipd_search_patient_other::get_ipd_other_search_patient,
            // GET /api/search/other
            assert_eq!(server
                .get(&EndPoint::SearchOther.base())
                .await.status_code(), status_code);
        }
        EndPoint::Sse => {}
        EndPoint::SseGroup => {
            // sse::post_sse_group,
            // POST /api/sse-group
            assert_eq!(server
                .post(&EndPoint::SseGroup.base())
                .json(&sse::SseGroup::demo())
                .await.status_code(), status_code);
        }
        EndPoint::SseMessage => {
            // sse::get_sse_message,
            // GET /api/sse-message
            assert_eq!(server
                .get(&EndPoint::SseMessage.base())
                .await.status_code(), status_code);
            // sse::post_sse_message,
            // POST /api/sse-message
            assert_eq!(server
                .post(&EndPoint::SseMessage.base())
                .json(&sse::SsePostMessage::demo())
                .await.status_code(), status_code);
            // sse::patch_sse_message_by_person,
            // PATCH /api/sse-message
            assert_eq!(server
                .patch(&EndPoint::SseMessage.base())
                .json(&Vec::<u32>::new())
                .await.status_code(), status_code);
        }
        EndPoint::User => {} // has seperated test
        EndPoint::UserConfig => {
            // user::config::post_user_config
            // POST /api/user-config
            assert_eq!(server
                .post(&EndPoint::UserConfig.base())
                .json(&user::config::UserConfig::demo())
                .await.status_code(), status_code);
            // user::config::patch_user_config
            // PATCH /api/user-config
            assert_eq!(server
                .patch(&EndPoint::UserConfig.base())
                .json(&user::config::UserConfigCommand::demo_clear2fa(String::from("user")))
                .await.status_code(), status_code);
        }
        EndPoint::UserRolePrelude => {
            // user::role::get_user_role_prelude,
            // GET /api/user-role/prelude
            assert_eq!(server
                .get(&EndPoint::UserRolePrelude.base())
                .await.status_code(), status_code);
        }
        EndPoint::UserRoleRole => {
            // user::role::get_role_permission_list,
            // GET /api/user-role/role
            assert_eq!(server
                .get(&EndPoint::UserRoleRole.base())
                .await.status_code(), status_code);
            // user::role::post_role_permission,
            // POST /api/user-role/role
            assert_eq!(server
                .post(&EndPoint::UserRoleRole.base())
                .json(&user::role::RolePermissionSave::demo())
                .await.status_code(), status_code);
            // user::role::delete_role_permission,
            // DELETE /api/user-role/role
            assert_eq!(server
                .delete(&EndPoint::UserRoleRole.base())
                .await.status_code(), status_code);
        }
        EndPoint::UserRoleUser => {
            // user::role::get_user_role_list,
            // GET /api/user-role/user
            assert_eq!(server
                .get(&EndPoint::UserRoleUser.base())
                .await.status_code(), status_code);
            }
            // user::role::post_user_role,
            // POST /api/user-role/user
            // ** THIS TEST WILL LOGOUT USER, ALWAYS KEEP THIS THE LAST TEST **
        EndPoint::XrayReportHn => {
            // xray::get_xray_report,
            // GET /api/xray/report-hn/{hn}
            assert_eq!(server
                .get(&[&EndPoint::XrayReportHn.base(), "0001234"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::XrayReadId => {
            // xray::post_xray_read,
            // POST /api/xray/read-id/{xn}
            assert_eq!(server
                .post(&[&EndPoint::XrayReadId.base(), "1"].concat())
                .await.status_code(), status_code);
            // xray::delete_xray_read,
            // DELETE /api/xray/read-id/{xn}
            assert_eq!(server
                .delete(&[&EndPoint::XrayReadId.base(), "1"].concat())
                .await.status_code(), status_code);
        }
        EndPoint::XrayPacsXn => {} // need Mock PACs
        EndPoint::Unknown => {}
    }
}
