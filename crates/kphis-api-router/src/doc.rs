use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use kphis_api_core::open_api::Binary;
use kphis_api_handler::{
    app, avatar, drug_use_duration, emr, image, ipd, lab, med_reconciliation, opd_er, post_admit, pre_admit, pre_order, prescription, refer_note, refer_out, report, search, sse, user, xray,
};
use kphis_api_pdf::handler as pdf;
use kphis_util::error;

// we mark where to test handler/query here
// if handler with all queries tested, no any side effect, only do unittest in kphis-query
// if handler that cannot create query's unittest, do handler's integration tests in this crate
// if handler using multipart, do any screening, bindling, forwarding or any complex things,
//   do handler's integration tests in this crate with query's unittest in kphis-query
// NOTE: check_an_can_execute() and check_an_opt_can_execute() already has Unittest, return 400 when NO AN
#[derive(OpenApi)]
#[openapi(
    paths(
        app::get_exists,                                                            // Unittest in query
        avatar::get_avatar_ipd,                                                     // Unittest in query
        avatar::get_avatar_opd_er,                                                  // Unittest in query
        drug_use_duration::get_drug_use_duration,                                   // Unittest in query
        drug_use_duration::post_drug_use_duration,                                  // Unittest in query
        emr::get_emr_date,                                                          // Unittest in query
        emr::get_emr_visit,                                                         // Bundle with Unittests in query
        image::file_path::post_image_file,                                          // Disk IO + Unittests in query     // image_path.rs
        image::file_path::patch_image_path,                                         // Unittest in query
        image::file_path::delete_image_file,                                        // Disk IO + Unittests in query     // image_path.rs
        image::file_path::post_image_usage,                                         // Unittest in query                // image_path.rs
        image::file_path::delete_image_usage,                                       // Unittest in query                // image_path.rs
        image::file_path::get_image_usage_id,                                       // Unittest in query
        image::scan_his::get_scan_his_image,                                        // Bundle with Unittests in query   // all_query_params.rs
        ipd::admission_note_dr::get_ipd_admission_note_dr,                          // Bundle with Unittests in query
        ipd::admission_note_dr::post_ipd_admission_note_dr,                         // Bundle with Unittests in query   // all_query_params.rs
        ipd::admission_note_dr::put_ipd_admission_note_dr,                          // Bundle with Unittests in query   // all_query_params.rs
        ipd::admission_note_dr::patch_ipd_pharmacy_check,                           // Unittest in query
        ipd::admission_note_nurse::get_ipd_admission_note_nurse,                    // Unittest in query
        ipd::admission_note_nurse::post_ipd_admission_note_nurse,                   // Unittest in query                // all_query_params.rs
        ipd::admission_note_nurse::put_ipd_admission_note_nurse,                    // Unittest in query                // all_query_params.rs
        ipd::consult::get_ipd_consult_by_an,                                        // Unittest in query
        ipd::consult::get_ipd_consult_by_id,                                        // Unittest in query
        ipd::consult::get_ipd_consult_list,                                         // Unittest in query
        ipd::consult::post_ipd_consult,                                             // Bundle with Unittests in query   // all_query_params.rs
        ipd::consult::delete_ipd_consult_by_id,                                     // Bundle with Unittests in query   // all_query_params.rs
        ipd::dc_plan::get_ipd_dc_plan,                                              // Unittest in query
        ipd::dc_plan::post_ipd_dc_plan,                                             // Unittest in query
        ipd::dc_plan::delete_ipd_dc_plan,                                           // Unittest in query                // all_query_params.rs
        ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_dx,                                   // Unittest in query
        ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_dx,                                  // Unittest in query                // all_query_params.rs
        ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_dx,                                // Unittest in query
        ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_med,                                  // Unittest in query
        ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_med,                                 // Unittest in query                // all_query_params.rs
        ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_med,                               // Unittest in query
        ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_env,                                  // Unittest in query
        ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_env,                                 // Unittest in query                // all_query_params.rs
        ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_env,                               // Unittest in query
        ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_tx,                                   // Unittest in query
        ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_tx,                                  // Unittest in query                // all_query_params.rs
        ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_tx,                                // Unittest in query
        ipd::dc_plan_tmp::get_ipd_dc_plan_tmp_diet,                                 // Unittest in query
        ipd::dc_plan_tmp::post_ipd_dc_plan_tmp_diet,                                // Unittest in query                // all_query_params.rs
        ipd::dc_plan_tmp::delete_ipd_dc_plan_tmp_diet,                              // Unittest in query
        ipd::doctor_in_charge::get_ipd_doctor_in_charge,                            // Unittest in query
        ipd::doctor_in_charge::post_ipd_doctor_in_charge,                           // Unittest in query                // all_query_params.rs
        ipd::doctor_in_charge::delete_ipd_doctor_in_charge,                         // Bundle with Unittests in query   // all_query_params.rs
        ipd::document::get_ipd_document_datetime,                                   // Unittest in query
        ipd::document::get_ipd_document_list,                                       // Unittest in query
        ipd::document::get_ipd_document_types,                                      // Unittest in query
        ipd::document::post_ipd_document_type,                                      // Unittest in query
        ipd::document::delete_ipd_document_type,                                    // Unittest in query
        ipd::focus_list::get_ipd_focus_list,                                        // Unittest in query
        ipd::focus_list::post_ipd_focus_list,                                       // Bundle with Unittests in query   // all_query_params.rs
        ipd::focus_list::delete_ipd_focus_list,                                     // Bundle with Unittests in query   // all_query_params.rs
        ipd::index_action::post_ipd_index_action,                                   // Bundle with Unittests in query   // all_query_params.rs
        ipd::index_action::delete_ipd_index_action,                                 // Unittest in query
        ipd::index_monitor::post_ipd_index_monitor,                                 // Bundle with Unittests in query   // all_query_params.rs
        ipd::index_monitor::delete_ipd_index_monitor,                               // Unittest in query
        ipd::focus_note::get_ipd_focus_note,                                        // Unittest in query
        ipd::focus_note::post_ipd_focus_note,                                       // Bundle with Unittests in query   // all_query_params.rs
        ipd::focus_note::delete_ipd_focus_note,                                     // Bundle with Unittests in query   // all_query_params.rs
        ipd::his::get_his_ipt_diag,                                                 // Unittest in query
        ipd::his::get_his_ipt_oprt,                                                 // Unittest in query
        ipd::his::get_ipd_his_opertion_admit,                                       // Unittest in query
        ipd::his::get_med_plan_ipd_remains,                                         // Unittest in query
        ipd::index_note::get_ipd_index_note,                                        // Unittest in query
        ipd::index_note::post_ipd_index_note,                                       // Bundle with Unittests in query   // all_query_params.rs
        ipd::index_note::delete_ipd_index_note,                                     // Unittest in query
        ipd::index_plan::get_index_plan_date,                                       // Unittest in query
        ipd::index_plan::get_ipd_index_med_pay,                                     // Unittest in query
        ipd::index_plan::post_ipd_index_plan,                                       // Bundle with Unittests in query   // all_query_params.rs
        ipd::index_plan::delete_ipd_index_plan,                                     // Unittest in query
        ipd::io::get_ipd_io_date,                                                   // Unittest in query
        ipd::io::get_ipd_io_shift,                                                  // Unittest in query
        ipd::io::post_ipd_io_shift,                                                 // Bundle with Unittests in query   // all_query_params.rs
        ipd::io::delete_ipd_io_shift,                                               // Bundle with Unittests in query   // all_query_params.rs
        ipd::med_reconcile::get_ipd_med_reconcile,                                  // Bundle with Unittests in query
        ipd::med_reconcile::post_ipd_med_reconcile,                                 // Bundle with Unittests in query   // all_query_params.rs
        ipd::med_reconcile::patch_ipd_med_reconcile,                                // Bundle with Unittests in query   // all_query_params.rs
        ipd::med_reconcile::delete_ipd_med_reconcile,                               // Bundle with Unittests in query   // all_query_params.rs
        ipd::med_reconcile::get_ipd_med_reconcile_hosxp,                            // Unittest in query
        ipd::med_reconcile::get_ipd_med_reconcile_last_dose,                        // Unittest in query
        ipd::med_reconcile::get_ipd_med_reconcile_note,                             // Unittest in query
        ipd::med_reconcile::post_ipd_med_reconcile_note,                            // Unittest in query
        ipd::med_reconcile::get_ipd_med_reconcile_remed_visit,                      // Unittest in query
        ipd::med_reconcile::get_ipd_med_reconcile_remed_med,                        // Unittest in query
        ipd::mra::get_ipd_mra,                                                      // Unittest in query
        ipd::mra::post_ipd_mra,                                                     // Unittest in query                // all_query_params.rs
        ipd::mra::put_ipd_mra,                                                      // Unittest in query                // all_query_params.rs
        ipd::mra::delete_ipd_mra,                                                   // Unittest in query                // all_query_params.rs
        ipd::order::get_ipd_order,                                                  // Unittest in query
        ipd::order::post_ipd_order,                                                 // Bundle with Unittests in query   // all_query_params.rs
        ipd::order::patch_ipd_order,                                                // Unittest in query
        ipd::order::delete_ipd_order,                                               // Unittest in query
        ipd::order::get_ipd_order_date,                                             // Unittest in query
        ipd::order::get_ipd_order_item,                                             // Bundle with Unittests in query
        ipd::order::patch_ipd_order_item,                                           // Unittest in query
        ipd::order::get_ipd_order_pharmacy,                                         // Unittest in query
        ipd::order::get_ipd_order_previous,                                         // Bundle with Unittests in query
        ipd::order::get_ipd_order_one_day_previous,                                 // Unittest in query
        ipd::order::get_ipd_home_med_from_cont,                                     // Unittest in query
        ipd::progress_note::get_ipd_progress_note,                                  // Bundle with Unittests in query
        ipd::progress_note::post_ipd_progress_note,                                 // Bundle with Unittests in query   // all_query_params.rs
        ipd::progress_note::delete_ipd_progress_note,                               // Unittest in query
        ipd::progress_note::get_ipd_progress_previous,                              // Unittest in query
        ipd::passcode::get_ipd_ward_passcode,                                       // Unittest in query
        ipd::passcode::post_ipd_ward_passcode,                                      // Bundle with Unittests in query
        ipd::show_patient_main::get_ipd_show_patient_main,                          // Unittest in query
        ipd::summary::get_ipd_summary,                                              // Bundle with Unittests in query   // all_query_params.rs
        ipd::summary::post_ipd_summary,                                             // Bundle with Unittests in query   // all_query_params.rs
        ipd::summary::patch_ipd_summary,                                            // Unittest in query                // all_query_params.rs
        ipd::summary::get_ipd_summary_note,                                         // Unittest in query
        ipd::summary::post_ipd_summary_note,                                        // Unittest in query
        ipd::summary::patch_ipd_summary_note,                                       // Unittest in query
        ipd::summary::delete_ipd_summary_note,                                      // Unittest in query
        ipd::summary::get_ipd_summary_status,                                       // Unittest in query
        ipd::summary::put_ipd_summary_status,                                        // Unittest in query
        ipd::summary_audit::get_ipd_summary_audit,                                  // Unittest in query
        ipd::summary_audit::post_ipd_summary_audit,                                 // Unittest in query                // all_query_params.rs
        ipd::summary_audit::delete_ipd_summary_audit,                               // Unittest in query                // all_query_params.rs
        ipd::tmp::get_ipd_tmp_group,                                                // Unittest in query
        ipd::tmp::post_ipd_tmp_group,                                               // Bundle with Unittests in query   // all_query_params.rs
        ipd::tmp::delete_ipd_tmp_group,                                             // Unittest in query                // all_query_params.rs
        ipd::tmp::get_ipd_subgroup,                                                 // Unittest in query
        ipd::tmp::post_ipd_subgroup,                                                // Bundle with Unittests in query   // all_query_params.rs
        ipd::tmp::delete_ipd_subgroup,                                              // Unittest in query                // all_query_params.rs
        ipd::tmp::get_ipd_focus,                                                    // Unittest in query
        ipd::tmp::post_ipd_focus,                                                   // Bundle with Unittests in query   // all_query_params.rs
        ipd::tmp::delete_ipd_focus,                                                 // Unittest in query                // all_query_params.rs
        ipd::tmp::get_ipd_goal,                                                     // Unittest in query
        ipd::tmp::post_ipd_goal,                                                    // Bundle with Unittests in query   // all_query_params.rs
        ipd::tmp::delete_ipd_goal,                                                  // Unittest in query                // all_query_params.rs
        ipd::tmp::get_ipd_intvt,                                                    // Unittest in query
        ipd::tmp::post_ipd_intvt,                                                   // Bundle with Unittests in query   // all_query_params.rs
        ipd::tmp::delete_ipd_intvt,                                                 // Unittest in query                // all_query_params.rs
        ipd::tmp::get_ipd_dlc,                                                      // Unittest in query
        ipd::tmp::post_ipd_dlc,                                                     // Bundle with Unittests in query   // all_query_params.rs
        ipd::tmp::delete_ipd_dlc,                                                   // Unittest in query                // all_query_params.rs
        ipd::vital_sign::get_ipd_vital_sign,                                        // Unittest in query
        ipd::vital_sign::post_ipd_vital_sign,                                       // Unittest in query                // all_query_params.rs
        ipd::vital_sign::put_ipd_vital_sign,                                        // Unittest in query                // all_query_params.rs
        ipd::vital_sign::delete_ipd_vital_sign,                                     // Unittest in query
        lab::get_lab_head,                                                          // Unittest in query
        lab::get_lab_item,                                                          // Unittest in query
        lab::get_wbc_band,                                                          // Unittest in query
        lab::post_lab_read,                                                         // Unittest in query
        lab::delete_lab_read,                                                       // Unittest in query
        med_reconciliation::get_med_reconciliation_head,                            // Unittest in query
        opd_er::dc_plan::get_opd_er_dc_plan,                                        // Unittest in query
        opd_er::dc_plan::post_opd_er_dc_plan,                                       // Unittest in query
        opd_er::dc_plan::delete_opd_er_dc_plan,                                     // Unittest in query                // all_query_params.rs
        opd_er::document::get_opd_er_document_list,                                 // Unittest in query
        opd_er::document::get_opd_er_document_types,                                // Unittest in query
        opd_er::document::post_opd_er_document_type,                                // Unittest in query
        opd_er::document::delete_opd_er_document_type,                              // Unittest in query
        opd_er::focus_list::get_opd_er_focus_list,                                  // Unittest in query
        opd_er::focus_list::post_opd_er_focus_list,                                 // Bundle with Unittests in query
        opd_er::focus_list::delete_opd_er_focus_list,                               // Bundle with Unittests in query   // all_query_params.rs
        opd_er::focus_note::get_opd_er_focus_note,                                  // Unittest in query
        opd_er::focus_note::post_opd_er_focus_note,                                 // Bundle with Unittests in query
        opd_er::focus_note::delete_opd_er_focus_note,                               // Bundle with Unittests in query   // all_query_params.rs
        opd_er::hosxp_med::get_opd_med,                                             // Unittest in query
        opd_er::index_action::post_opd_er_index_action,                             // Bundle with Unittests in query
        opd_er::index_action::delete_opd_er_index_action,                           // Unittest in query
        opd_er::index_monitor::post_opd_er_index_monitor,                           // Bundle with Unittests in query
        opd_er::index_monitor::delete_opd_er_index_monitor,                         // Unittest in query
        // opd_er::index_plan::get_opd_er_index_plan,                               // Bundle with Unittests in query
        opd_er::index_plan::post_opd_er_index_plan,                                 // Bundle with Unittests in query
        opd_er::index_plan::delete_opd_er_index_plan,                               // Unittest in query
        opd_er::io::get_opd_er_io_date,                                             // Unittest in query
        opd_er::io::get_opd_er_io_shift,                                            // Unittest in query
        opd_er::io::post_opd_er_io_shift,                                           // Bundle with Unittests in query   // all_query_params.rs
        opd_er::io::delete_opd_er_io_shift,                                         // Bundle with Unittests in query   // all_query_params.rs
        opd_er::medical_history::get_opd_er_medical_history,                        // Bundle with Unittests in query   // all_query_params.rs
        opd_er::medical_history::get_opd_er_allergy_history,                        // Unittest in query
        opd_er::medical_history::post_opd_er_allergy_history,                       // Bundle with Unittests in query   // all_query_params.rs
        opd_er::medical_history::get_opd_er_consult_history,                        // Unittest in query
        opd_er::medical_history::post_opd_er_consult_history,                       // Bundle with Unittests in query   // all_query_params.rs
        opd_er::medical_history::get_opd_er_ft_history,                             // Unittest in query
        opd_er::medical_history::post_opd_er_ft_history,                            // Bundle with Unittests in query
        opd_er::medical_history::get_opd_er_scan_history,                           // Unittest in query
        opd_er::medical_history::post_opd_er_scan_history,                          // Bundle with Unittests in query
        opd_er::medical_history::get_opd_er_screen_history,                         // Unittest in query
        opd_er::medical_history::post_opd_er_screen_history,                        // Bundle with Unittests in query   // all_query_params.rs
        opd_er::medical_history::get_opd_er_trauma_history,                         // Unittest in query
        opd_er::medical_history::post_opd_er_trauma_history,                        // Bundle with Unittests in query
        opd_er::med_reconcile::get_opd_er_med_reconcile,                            // Bundle with Unittests in query
        opd_er::med_reconcile::post_opd_er_med_reconcile,                           // Bundle with Unittests in query   // all_query_params.rs
        opd_er::med_reconcile::patch_opd_er_med_reconcile,                          // Bundle with Unittests in query   // all_query_params.rs
        opd_er::med_reconcile::delete_opd_er_med_reconcile,                         // Bundle with Unittests in query   // all_query_params.rs
        opd_er::med_reconcile::get_opd_er_med_reconcile_note,                       // Unittest in query
        opd_er::med_reconcile::post_opd_er_med_reconcile_note,                      // Unittest in query
        opd_er::order::get_opd_er_order,                                            // Bundle with Unittests in query
        opd_er::order::post_opd_er_order,                                           // Bundle with Unittests in query
        opd_er::order::patch_opd_er_order,                                          // Unittest in query
        opd_er::order::delete_opd_er_order,                                         // Unittest in query
        opd_er::order::get_opd_er_order_item,                                       // Bundle with Unittests in query
        opd_er::order::patch_opd_er_order_item,                                     // Unittest in query
        opd_er::order::get_opd_er_order_pharmacy,                                   // Unittest in query
        opd_er::order_master::get_opd_er_order_master,                              // Unittest in query
        opd_er::order_master::get_opd_er_order_master_check,                        // Unittest in query
        opd_er::order_master::get_opd_er_order_master_list,                         // Unittest in query
        opd_er::order_master::post_opd_er_order_master,                             // Bundle with Unittests in query
        opd_er::progress_note::get_opd_er_progress_note,                            // Bundle with Unittests in query
        opd_er::progress_note::post_opd_er_progress_note,                           // Bundle with Unittests in query
        opd_er::progress_note::delete_opd_er_progress_note,                         // Unittest in query
        opd_er::show_patient_main::get_opd_er_show_patient_main_id,                 // Unittest in query
        opd_er::show_patient_main::get_opd_er_show_patient_main_vn,                 // Unittest in query
        opd_er::vital_sign::get_opd_er_vital_sign,                                  // Unittest in query
        opd_er::vital_sign::post_opd_er_vital_sign,                                 // Unittest in query
        opd_er::vital_sign::put_opd_er_vital_sign,                                  // Unittest in query
        opd_er::vital_sign::delete_opd_er_vital_sign,                               // Unittest in query
        pdf::get_single_pdf,                                                        // Disk IO
        pdf::get_raw_single_template,                                               // Disk IO
        post_admit::get_ipd_post_admit_list,                                        // Unittest in query
        pre_admit::get_ipd_pre_admit_list,                                          // Unittest in query
        pre_admit::post_ipd_pre_admit,                                              // Unittest in query
        pre_admit::patch_ipd_pre_admit,                                             // Unittest in query
        pre_order::get_ipd_pre_order_list,                                          // Unittest in query
        pre_order::post_ipd_pre_order_master,                                       // Bundle with Unittests in query
        pre_order::delete_ipd_pre_order_master,                                     // Bundle with Unittests in query
        pre_order::get_ipd_pre_order,                                               // Unittest in query
        pre_order::post_ipd_pre_order,                                              // Bundle with Unittests in query
        pre_order::delete_ipd_pre_order,                                            // Unittest in query
        pre_order::post_ipd_pre_order_into,                                         // Bundle with Unittests in query
        pre_order::get_ipd_pre_progress_note,                                       // Bundle with Unittests in query
        pre_order::post_ipd_pre_progress_note,                                      // Bundle with Unittests in query
        pre_order::delete_ipd_pre_progress_note,                                    // Unittest in query
        prescription::get_prescription_screen,                                      // Bundle with Unittests in query
        prescription::post_prescription_screen,                                     // Bundle with Unittests in query   // all_query_params.rs
        prescription::patch_prescription_screen,                                    // Bundle with Unittests in query   // all_query_params.rs
        refer_note::get_refernote,                                                  // Bundle with Unittests in query
        refer_note::post_refernote,                                                 // Unittest in query
        refer_out::get_his_referout_data,                                           // Bundle with Unittests in query
        refer_out::post_his_referout,                                               // Unittest in query
        report::get_custom_report,                                                  // Bundle with Unittests in query
        report::post_custom_report,                                                 // Unittest in query
        report::delete_custom_report,                                               // Unittest in query                // all_query_params.rs
        report::post_query_to_json_string,                                          // Unittest in query
        search::searchbox::get_drug_duplication_check,                              // Unittest in query
        search::searchbox::get_drug_interaction_check,                              // Unittest in query
        search::searchbox::get_hosp_searchbox,                                      // Unittest in query
        search::searchbox::get_ivfluid_searchbox,                                   // Unittest in query
        search::searchbox::get_lab_searchbox,                                       // Unittest in query
        search::searchbox::get_med_searchbox,                                       // Unittest in query
        search::searchbox::get_opd_visit_searchbox,                                 // Unittest in query
        search::searchbox::get_patient_searchbox,                                   // Unittest in query
        search::searchbox::get_xray_searchbox,                                      // Unittest in query
        search::ipd_search_patient_dr::get_ipd_dr_search_patient,                   // Unittest in query
        search::ipd_search_patient_nurse::get_ipd_nurse_search_patient,             // Unittest in query
        search::ipd_search_patient_other::get_ipd_other_search_patient,             // Unittest in query
        search::ipd_search_patient_pharmacist::get_ipd_pharmacist_search_patient,   // Unittest in query
        // sse::get_sse,
        sse::logout,                                                                                                    // user.rs
        // sse::get_sse_by_id,
        sse::get_sse_message,                                                       // Unittest in query
        sse::post_sse_group,
        sse::post_sse_message,                                                      // Unittest in query
        sse::patch_sse_messages,                                                    // Unittest in query
        user::config::post_user_config,                                             // Bundle with Unittests in query
        user::config::patch_user_config,                                            // Bundle with Unittests in query
        user::role::get_user_role_list,                                             // Bundle with Unittests in query
        user::role::get_user_role_prelude,                                          // Bundle with Unittests in query
        user::role::post_user_role,                                                 // Bundle with Unittests in query
        user::role::get_role_permission_list,                                       // Bundle with Unittests in query
        user::role::post_role_permission,                                           // Bundle with Unittests in query
        user::role::delete_role_permission,                                         // Bundle with Unittests in query
        user::his::check_login,                                                                                       // user.rs
        user::his::check_totp,                                                                                        // user.rs
        user::his::refresh_cookie,                                                                                    // user.rs
        user::his::refresh_token,                                                                                     // user.rs                                                                                           // user.rs
        xray::get_xray_report,                                                      // Bundle with Unittests in query
        xray::post_xray_read,                                                       // Bundle with Unittests in query
        xray::delete_xray_read,                                                     // Bundle with Unittests in query
    ),
    components(
        // NOTE: Struct/Enum name MUST NOT DUPLICATE
        schemas(
            Binary,
            kphis_model::app::AppStatus, kphis_model::app::AppAsset, kphis_model::app::VisitTypeId,
            error::AppError, error::Source,
            kphis_model::fetch::ExecuteResponse,
            kphis_model::avatar::AvatarOpdEr, kphis_model::avatar::AvatarWard,
            kphis_model::dc_plan::DischargePlan, kphis_model::dc_plan::DischargePlanSave,
            kphis_model::drug_use_duration::DrugUseDuration,
            kphis_model::emr::EmrDate,
            kphis_model::emr::EmrVisit,
            kphis_model::focus_list::FocusList, kphis_model::focus_list::FocusListSave,
            kphis_model::focus_note::FocusNote, kphis_model::focus_note::FocusNoteSave,
            kphis_model::image::ImageBase64,
            kphis_model::image::file_path::ImagePath, kphis_model::image::file_path::ImageSave, kphis_model::image::file_path::ImageUsage,
            kphis_model::image::scan_his::ScanImage,
            kphis_model::index_action::IndexAction,
            kphis_model::index_monitor::IndexMonitor,
            kphis_model::index_plan::IndexPlanDate, kphis_model::index_plan::IndexPlan, kphis_model::index_plan::IndexPlanSave, kphis_model::index_plan::IpdIndexMedPay,
            kphis_model::ipd::admission_note_dr::IpdAdmissionNoteDrRaw, kphis_model::ipd::admission_note_dr::IpdAdmissionNoteDrSave, kphis_model::ipd::admission_note_dr::IpdDrAdmissionNote,
            kphis_model::ipd::admission_note_dr::AdmissionNoteDoctor, kphis_model::ipd::admission_note_dr::OpdscreenPe, kphis_model::ipd::admission_note_dr::Vs,
            kphis_model::ipd::admission_note_dr::Period, kphis_model::ipd::admission_note_dr::OpdErAllergyHistory,
            kphis_model::ipd::admission_note_nurse::IpdNurseAdmissionNote,
            kphis_model::ipd::consult::IpdConsultList, kphis_model::ipd::consult::ConsultWithName, kphis_model::ipd::consult::Consult, kphis_model::ipd::consult::ConsultSave, kphis_model::ipd::consult::DoctorCodeSave,
            kphis_model::ipd::dc_plan_tmp::DcPlanTmpDx, kphis_model::ipd::dc_plan_tmp::DcPlanTmpMed, kphis_model::ipd::dc_plan_tmp::DcPlanTmpEnv, kphis_model::ipd::dc_plan_tmp::DcPlanTmpTx, kphis_model::ipd::dc_plan_tmp::DcPlanTmpDiet,
            kphis_model::ipd::doctor_in_charge::IpdDoctorInCharge,
            kphis_model::ipd::document::IpdDocumentExists, kphis_model::ipd::document::IpdDocumentDatetime, kphis_model::ipd::document::DocumentScan,
            kphis_model::ipd::his::HisIptDiag, kphis_model::ipd::his::HisIptOprt, kphis_model::ipd::his::HisOperationAdmit, kphis_model::ipd::his::HisMedPlanIpd,
            kphis_model::ipd::index_note::IndexNote,
            kphis_model::ipd::io::IoDate, kphis_model::ipd::io::IoShift,
            kphis_model::ipd::mra::IpdMra,
            kphis_model::ipd::pharmacy_monitor::IpdOrderPharmacyMonitor, kphis_model::ipd::pharmacy_monitor::IpdOrderPharmacy, kphis_model::ipd::pharmacy_monitor::PharmacyIpt,
            kphis_model::ipd::passcode::ConfigIpdWardPasscode, kphis_model::ipd::passcode::PasscodeGenRequest, kphis_model::ipd::passcode::PasscodeGenRequestMode, kphis_model::ipd::passcode::PasscodeGenResponse,
            kphis_model::ipd::summary::Summary, kphis_model::ipd::summary::SummaryData, kphis_model::ipd::summary::DxData, kphis_model::ipd::summary::DoctorData, kphis_model::ipd::summary::XRayData, kphis_model::ipd::summary::DchData, kphis_model::ipd::summary::LabAlertData,
            kphis_model::ipd::summary::SummarySave, kphis_model::ipd::summary::SummaryDataSave, kphis_model::ipd::summary::SummaryCodeSave, kphis_model::ipd::summary::SummaryNote, kphis_model::ipd::summary::SummaryNoteSave, kphis_model::ipd::summary::SummaryStatus,
            kphis_model::ipd::summary_audit::SummaryAudit, kphis_model::ipd::summary_audit::SummaryAuditItem,
            kphis_model::ipd::tmp::TmpGroup, kphis_model::ipd::tmp::TmpSubGroup, kphis_model::ipd::tmp::TmpFocus, kphis_model::ipd::tmp::TmpGoal, kphis_model::ipd::tmp::TmpIntvt, kphis_model::ipd::tmp::TmpDlc,
            kphis_model::lab::LabHead, kphis_model::lab::LabItemsGroup, kphis_model::lab::LabItem, kphis_model::lab::LabWbcBand,
            kphis_model::med_reconcile::MedReconciliationHeader, kphis_model::med_reconcile::MedReconciliation, kphis_model::med_reconcile::MedReconciliationDetail, kphis_model::med_reconcile::MedReconciliationItemSave, kphis_model::med_reconcile::MedReconciliationItemPatch,
            kphis_model::med_reconcile::MedReconciliationNote, kphis_model::med_reconcile::AdmissionNoteLastDose, kphis_model::med_reconcile::ReMedVisit, kphis_model::med_reconcile::ReMedMedication,
            kphis_model::opd_er::document::OpdErDocumentExists,
            kphis_model::opd_er::hosxp_med::OpdMed,
            kphis_model::opd_er::medical_history::OpdErMedicalHistory, kphis_model::opd_er::medical_history::OpdScreenHistory, kphis_model::opd_er::medical_history::VitalSignHistory, kphis_model::opd_er::medical_history::TraumaHistory, kphis_model::opd_er::medical_history::AllergyHistory,
            kphis_model::opd_er::medical_history::NurseScreeningHistory, kphis_model::opd_er::medical_history::ConsultHistory, kphis_model::opd_er::medical_history::ScanHistory, kphis_model::opd_er::medical_history::SetFtHistory,
            kphis_model::opd_er::order_master::OpdErOrderMaster, kphis_model::opd_er::order_master::OpdErOrderMasterCheck, kphis_model::opd_er::order_master::OpdErOrderMasterList, kphis_model::opd_er::order_master::OpdErOrderMasterSave,
            kphis_model::opd_er::pharmacy_monitor::OpdErOrderPharmacyMonitor, kphis_model::opd_er::pharmacy_monitor::OpdErOrderPharmacy,
            kphis_model::order::OrderDate, kphis_model::order::OrderTypeName, kphis_model::order::OrderSave, kphis_model::order::OrderPatch, kphis_model::order::OrderItemPatch, kphis_model::order::OrderPatchAction, kphis_model::order::OrderItemPatchAction,
            kphis_model::order::Order, kphis_model::order::OrderItemType,  kphis_model::order::MedOrderItem, kphis_model::order::OrderItem,
            kphis_model::patient_info::PatientInfo,
            kphis_model::post_admit::PostAdmitList,
            kphis_model::pre_admit::PreAdmitList, kphis_model::pre_admit::PreAdmitSave, kphis_model::pre_admit::PreAdmitPatch,
            kphis_model::pre_order::master::PreOrderMaster, kphis_model::pre_order::master::PreOrderMasterSave,
            kphis_model::pre_order::order::PreOrder, kphis_model::pre_order::order::PreOrderItemType, kphis_model::pre_order::order::PreOrderIntoCommand,
            kphis_model::pre_order::progress_note::PreProgressNote, kphis_model::pre_order::progress_note::PreProgressNoteItemType,
            kphis_model::prescription::PrescriptionScreen, kphis_model::prescription::PrescriptionInfo, kphis_model::prescription::VisitDate, kphis_model::prescription::PtNote, kphis_model::prescription::Lab,
            kphis_model::prescription::PrescriptionVn, kphis_model::prescription::Medicine, kphis_model::prescription::DrugInteraction, kphis_model::prescription::NextAppointment,
            kphis_model::prescription::PrescriptionScreenPatch, kphis_model::prescription::PostalPatch, kphis_model::prescription::TelemedPatch,
            kphis_model::progress_note::ProgressNote, kphis_model::progress_note::ProgressNoteItem, kphis_model::progress_note::ProgressNoteItemType, kphis_model::progress_note::ProgressNoteTypeName, kphis_model::progress_note::ProgressNoteSave,
            kphis_model::refer_note::ReferNote, kphis_model::refer_note::ReferNoteSave,
            kphis_model::refer_out::HisReferOutData, kphis_model::refer_out::HisReferOut, kphis_model::refer_out::HisReferVitalSign, kphis_model::refer_out::HisReferOutSave,
            kphis_model::report::TypstRaw, kphis_model::report::CustomReport, kphis_model::report::ReportQuery,
            kphis_model::route::Route,
            kphis_model::score::SupportedScore,
            kphis_model::search::ipd_search_patient_dr::IpdSearchPatientDrResponse,
            kphis_model::search::ipd_search_patient_nurse::IpdSearchPatientNurseResponse,
            kphis_model::search::ipd_search_patient_other::IpdSearchPatientOtherResponse,
            kphis_model::search::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistResponse,
            kphis_model::search::searchbox::DrugDuplicateCheck, kphis_model::search::searchbox::DrugInteractionCheck, kphis_model::search::searchbox::IvfluidSearchbox, kphis_model::search::searchbox::LabSearchbox, kphis_model::search::searchbox::MedSearchbox,
            kphis_model::search::searchbox::OpdVisitSearchbox, kphis_model::search::searchbox::PatientSearchbox, kphis_model::search::searchbox::XraySearchbox, kphis_model::search::searchbox::HospSearchBox,
            kphis_model::select_utils::ColorSelectOption, kphis_model::select_utils::SelectOption,
            kphis_model::shift::NurseShift,
            kphis_model::sse::SsePostMessage, kphis_model::sse::SseGroup, kphis_model::sse::SseMessage, kphis_model::sse::SseData,
            kphis_model::user::config::UserConfig, kphis_model::user::config::UserConfigResponse, kphis_model::user::config::UserConfigCommand,
            kphis_model::user::permission::Permission,
            kphis_model::user::role::Role, kphis_model::user::role::RolePermission, kphis_model::user::role::UserRole, kphis_model::user::role::UserRoleOptions,
            kphis_model::user::role::UserRoleList, kphis_model::user::role::UserRoleSave, kphis_model::user::role::RolePermissionList, kphis_model::user::role::RolePermissionSave,
            kphis_model::user::his::User, kphis_model::user::his::CurrentUserRole, kphis_model::user::his::UserRequest, kphis_model::user::his::UserRequest2fa, kphis_model::user::his::UserRequestFull, kphis_model::user::his::LoginResponse,
            kphis_model::vital_sign::VitalSign, kphis_model::vital_sign::VitalSignSave,
            kphis_model::xray::XrayReport,
        ),
    )
)]
struct KphisApi;

#[derive(OpenApi)]
#[openapi(paths(app::get_app_asset), components(schemas(kphis_model::app::AppAsset)))]
struct AssetsApi;

#[derive(OpenApi)]
#[openapi(paths(image::patient::get_patient_image,))]
struct ImageApi;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Access Token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("PASETO")
                        .description(Some("[PASETO](https://paseto.io/) v.4 public access token"))
                        .build(),
                ),
            );
            components.add_security_scheme(
                "Refresh Token",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::with_description("REFRESH", "[PASETO](https://paseto.io/) v.4 public refresh token"))),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    nest(
        (path = "/assets", api = AssetsApi),
        (path = "/img", api = ImageApi),
        (path = "/api", api = KphisApi),
    ),
    tags(
        (name = "kphis", description = "KPHIS backend API"),
    )
)]
pub struct ApiDoc;
