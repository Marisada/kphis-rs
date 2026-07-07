use http::Method;
use std::ops::Deref;
use strum::{EnumIter, EnumString, IntoStaticStr, VariantArray};

use crate::user::permission::Permission;

pub trait QueryString {
    // new from "key=value" array
    fn from_tuples(params: &[(String, String)]) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn query_string(&self) -> String;
}

pub fn find_qs(params: &[(String, String)], key: &str) -> Option<String> {
    params.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
}

// NOTE: we use path style like `/some/../name-first-second/{first}/{second}` syntax
// because we need to re-construct endpoint from [first, secone] with base() + [first, secone].join("/")
// NOTE: NOT USE `/some` and `/some/thing` together
//       because `/some/thing` will matched by `/some` and not hit, USE `/some-thing` instead
/// Server API endpoints
#[derive(Debug, PartialEq, Eq, Hash, EnumString, EnumIter, IntoStaticStr, VariantArray)]
// #[strum(prefix = "/api")]
pub enum EndPoint {
    #[strum(to_string = "/avatar/opd-er")]
    AvatarOpdEr,

    #[strum(to_string = "/avatar/ipd")]
    AvatarIpd,

    #[strum(to_string = "/drug-use-duration")]
    DrugUseDuration,

    #[strum(to_string = "/exists-key-id/{key}/{id}")]
    ExistsKeyId,

    #[strum(to_string = "/emr/date-hn/{hn}")]
    EmrDateHn,

    #[strum(to_string = "/emr/visit-vn/{vn}")]
    EmrVisitVn,

    #[strum(to_string = "/his/ipt-diag-an/{an}")]
    HisIptDiagAn,

    #[strum(to_string = "/his/ipt-oprt-an/{an}")]
    HisIptOprtAn,

    #[strum(to_string = "/his/med-plan-ipd-an/{an}")]
    HisMedPlanIpdAn,

    #[strum(to_string = "/his/operation-admit-an/{an}")]
    HisOperationAdmitAn,

    #[strum(to_string = "/his/refer-out-vnan/{vnan}")]
    HisReferOutVnan,

    #[strum(to_string = "/image")]
    Image,

    #[strum(to_string = "/image-usage")]
    ImageUsage,

    #[strum(to_string = "/image-usage-id/{usage_id}/{usage_key_id}")]
    ImageUsageId,

    #[strum(to_string = "/ipd/admission-note-dr-an/{an}")]
    IpdAdmissionNoteDrAn,

    #[strum(to_string = "/ipd/admission-note-dr/pharmacy-check-an/{an}")]
    IpdAdmissionNoteDrPharmCheckAn,

    #[strum(to_string = "/ipd/admission-note-dr")]
    IpdAdmissionNoteDr,

    #[strum(to_string = "/ipd/admission-note-nurse-an/{an}")]
    IpdAdmissionNoteNurseAn,

    #[strum(to_string = "/ipd/admission-note-nurse")]
    IpdAdmissionNoteNurse,

    #[strum(to_string = "/ipd/consult")]
    IpdConsult,

    #[strum(to_string = "/ipd/consult-an/{an}")]
    IpdConsultAn,

    #[strum(to_string = "/ipd/consult-id/{consult_id}")]
    IpdConsultId,

    #[strum(to_string = "/ipd/dc-plan-an/{an}")]
    IpdDcPlanAn,

    #[strum(to_string = "/ipd/dc-plan-tmp/dx")]
    IpdDcPlanTmpDx,

    #[strum(to_string = "/ipd/dc-plan-tmp/med")]
    IpdDcPlanTmpMed,

    #[strum(to_string = "/ipd/dc-plan-tmp/env")]
    IpdDcPlanTmpEnv,

    #[strum(to_string = "/ipd/dc-plan-tmp/tx")]
    IpdDcPlanTmpTx,

    #[strum(to_string = "/ipd/dc-plan-tmp/diet")]
    IpdDcPlanTmpDiet,

    #[strum(to_string = "/ipd/doctor-in-charge")]
    IpdDoctorInCharge,

    #[strum(to_string = "/ipd/document/datetime-an/{an}")]
    IpdDocumentDatetimeAn,

    #[strum(to_string = "/ipd/document/list-vn-an/{vn}/{an}")]
    IpdDocumentListVnAn,

    #[strum(to_string = "/ipd/document/scan-an/{an}")]
    IpdDocumentScanAn,

    #[strum(to_string = "/ipd/focus-list-an/{an}")]
    IpdFocusListAn,

    #[strum(to_string = "/ipd/focus-note-an/{an}")]
    IpdFocusNoteAn,

    #[strum(to_string = "/ipd/index-action-id/{action_id}")]
    IpdIndexActionId,

    #[strum(to_string = "/ipd/index-action")]
    IpdIndexAction,

    #[strum(to_string = "/ipd/index-med-pay-an/{an}")]
    IpdIndexMedPayAn,

    #[strum(to_string = "/ipd/index-monitor-id/{monitor_id}")]
    IpdIndexMonitorId,

    #[strum(to_string = "/ipd/index-monitor")]
    IpdIndexMonitor,

    #[strum(to_string = "/ipd/index-note-id/{nurse_index_note_id}")]
    IpdIndexNoteId,

    #[strum(to_string = "/ipd/index-note")]
    IpdIndexNote,

    #[strum(to_string = "/ipd/index-plan-date-an/{an}")]
    IpdIndexPlanDateAn,

    #[strum(to_string = "/ipd/index-plan-id/{plan_id}")]
    IpdIndexPlanId,

    #[strum(to_string = "/ipd/index-plan")]
    IpdIndexPlan,

    #[strum(to_string = "/ipd/io")]
    IpdIo,

    #[strum(to_string = "/ipd/io-date-an/{an}")]
    IpdIoDateAn,

    #[strum(to_string = "/ipd/med-reconcile")]
    IpdMedReconcile,

    #[strum(to_string = "/ipd/med-reconcile-hosxp-an/{an}")]
    IpdMedReconcileHosxpAn,

    #[strum(to_string = "/ipd/med-reconcile-last-dose-an/{an}")]
    IpdMedReconcileLastDoseAn,

    #[strum(to_string = "/ipd/med-reconcile-note-id/{med_reconciliation_id}")]
    IpdMedReconcileNoteId,

    #[strum(to_string = "/ipd/med-reconcile-remed-visit-hn/{hn}")]
    IpdMedReconcileRemedVisitHn,

    #[strum(to_string = "/ipd/med-reconcile-remed-med")]
    IpdMedReconcileRemedMed,

    #[strum(to_string = "/ipd/mra")]
    IpdMra,

    #[strum(to_string = "/ipd/order/item")]
    IpdOrderItem,

    #[strum(to_string = "/ipd/order/previous")]
    IpdOrderPrevious,

    #[strum(to_string = "/ipd/order/one-day-previous-an/{an}")]
    IpdOrderOnedayPreviousAn,

    #[strum(to_string = "/ipd/order/progress-previous")]
    IpdOrderProgressPrevious,

    #[strum(to_string = "/ipd/order/to-home-med-an/{an}")]
    IpdOrderToHomeMedAn,

    #[strum(to_string = "/ipd/order/order-date-an/{an}")]
    IpdOrderOrderDateAn,

    #[strum(to_string = "/ipd/order/order-id/{order_id}")]
    IpdOrderOrderId,

    #[strum(to_string = "/ipd/order/order")]
    IpdOrderOrder,

    #[strum(to_string = "/ipd/order/progress-note-id/{progress_note_id}")]
    IpdOrderProgressNoteId,

    #[strum(to_string = "/ipd/order/progress-note")]
    IpdOrderProgressNote,

    #[strum(to_string = "/ipd/order/pharmacy")]
    IpdOrderPharmacy,

    #[strum(to_string = "/ipd/passcode")]
    IpdPasscode,

    #[strum(to_string = "/ipd/post-admit/list")]
    IpdPostAdmitList,

    #[strum(to_string = "/ipd/post-admit/count")]
    IpdPostAdmitCount,

    #[strum(to_string = "/ipd/pre-admit")]
    IpdPreAdmit,

    #[strum(to_string = "/ipd/pre-order/master-id/{pre_order_master_id}")]
    IpdPreOrderMasterId,

    #[strum(to_string = "/ipd/pre-order/master")]
    IpdPreOrderMaster,

    #[strum(to_string = "/ipd/pre-order/into")]
    IpdPreOrderInto,

    #[strum(to_string = "/ipd/pre-order/order-id/{order_id}")]
    IpdPreOrderOrderId,

    #[strum(to_string = "/ipd/pre-order/order")]
    IpdPreOrderOrder,

    #[strum(to_string = "/ipd/pre-order/progress-note-id/{progress_note_id}")]
    IpdPreOrderProgressNoteId,

    #[strum(to_string = "/ipd/pre-order/progress-note")]
    IpdPreOrderProgressNote,

    #[strum(to_string = "/ipd/show-patient-main-an/{an}")]
    IpdShowPatientMainAn,

    #[strum(to_string = "/ipd/summary")]
    IpdSummary,

    #[strum(to_string = "/ipd/summary-audit")]
    IpdSummaryAudit,

    #[strum(to_string = "/ipd/summary-note-id/{summary_id}")]
    IpdSummaryNoteId,

    #[strum(to_string = "/ipd/summary-status-id/{summary_id}")]
    IpdSummaryStatusId,

    #[strum(to_string = "/ipd/tmp/group")]
    IpdTmpGroup,

    #[strum(to_string = "/ipd/tmp/subgroup")]
    IpdTmpSubgroup,

    #[strum(to_string = "/ipd/tmp/focus")]
    IpdTmpFocus,

    #[strum(to_string = "/ipd/tmp/goal")]
    IpdTmpGoal,

    #[strum(to_string = "/ipd/tmp/intvt")]
    IpdTmpIntvt,

    #[strum(to_string = "/ipd/tmp/dlc")]
    IpdTmpDlc,

    #[strum(to_string = "/ipd/vital-sign-id/{vs_id}")]
    IpdVitalSignId,

    #[strum(to_string = "/ipd/vital-sign")]
    IpdVitalSign,

    #[strum(to_string = "/lab/head")]
    LabHead,

    #[strum(to_string = "/lab/item")]
    LabItem,

    #[strum(to_string = "/lab/read-id/{lab_order_number}")]
    LabReadId,

    #[strum(to_string = "/lab/wbc-key-value/{key}/{value}")]
    LabWbcKeyValue,

    #[strum(to_string = "/med-reconcile-hn/{hn}")]
    MedReconcileHn,

    #[strum(to_string = "/opd-er/dc-plan-id/{opd_er_order_master_id}")]
    OpdErDcPlanId,

    #[strum(to_string = "/opd-er/document/list-vn-id/{vn}/{opd_er_order_master_id}")]
    OpdErDocumentListVnId,

    #[strum(to_string = "/opd-er/document/scan-id/{opd_er_order_master_id}")]
    OpdErDocumentScanId,

    #[strum(to_string = "/opd-er/focus-list-id/{opd_er_order_master_id}")]
    OpdErFocusListId,

    #[strum(to_string = "/opd-er/focus-note-id/{opd_er_order_master_id}")]
    OpdErFocusNoteId,

    #[strum(to_string = "/opd-er/his-med-vn/{vn}")]
    OpdErHisMedVn,

    #[strum(to_string = "/opd-er/index-action-id/{action_id}")]
    OpdErIndexActionId,

    #[strum(to_string = "/opd-er/index-action")]
    OpdErIndexAction,

    #[strum(to_string = "/opd-er/index-plan-id/{plan_id}")]
    OpdErIndexPlanId,

    #[strum(to_string = "/opd-er/index-plan")]
    OpdErIndexPlan,

    #[strum(to_string = "/opd-er/index-monitor-id/{monitor_id}")]
    OpdErIndexMonitorId,

    #[strum(to_string = "/opd-er/index-monitor")]
    OpdErIndexMonitor,

    #[strum(to_string = "/opd-er/io")]
    OpdErIo,

    #[strum(to_string = "/opd-er/io-date-id/{opd_er_order_master_id}")]
    OpdErIoDateId,

    #[strum(to_string = "/opd-er/medical-history")]
    OpdErMedicalHistory,

    #[strum(to_string = "/opd-er/medical-history-trauma")]
    OpdErMedicalHistoryTrauma,

    #[strum(to_string = "/opd-er/medical-history-allergy")]
    OpdErMedicalHistoryAllergy,

    #[strum(to_string = "/opd-er/medical-history-screen")]
    OpdErMedicalHistoryScreen,

    #[strum(to_string = "/opd-er/medical-history-consult")]
    OpdErMedicalHistoryConsult,

    #[strum(to_string = "/opd-er/medical-history-scan")]
    OpdErMedicalHistoryScan,

    #[strum(to_string = "/opd-er/medical-history-ft")]
    OpdErMedicalHistoryFt,

    #[strum(to_string = "/opd-er/med-reconcile")]
    OpdErMedReconcile,

    #[strum(to_string = "/opd-er/med-reconcile-note-id/{med_reconciliation_id}")]
    OpdErMedReconcileNoteId,

    #[strum(to_string = "/opd-er/order/master/check-vn/{vn}")]
    OpdErOrderMasterCheckVn,

    #[strum(to_string = "/opd-er/order/master-id/{opd_er_order_master_id}")]
    OpdErOrderMasterId,

    #[strum(to_string = "/opd-er/order/master")]
    OpdErOrderMaster,

    #[strum(to_string = "/opd-er/order/item")]
    OpdErOrderItem,

    #[strum(to_string = "/opd-er/order/order-id/{order_id}")]
    OpdErOrderOrderId,

    #[strum(to_string = "/opd-er/order/order")]
    OpdErOrderOrder,

    #[strum(to_string = "/opd-er/order/progress-note-id/{progress_note_id}")]
    OpdErOrderProgressNoteId,

    #[strum(to_string = "/opd-er/order/progress-note")]
    OpdErOrderProgressNote,

    #[strum(to_string = "/opd-er/order/pharmacy")]
    OpdErOrderPharmacy,

    #[strum(to_string = "/opd-er/show-patient-main-id/{opd_er_order_master_id}")]
    OpdErShowPatientMainId,

    #[strum(to_string = "/opd-er/show-patient-main-vn/{vn}")]
    OpdErShowPatientMainVn,

    #[strum(to_string = "/opd-er/vital-sign-id/{vs_id}")]
    OpdErVitalSignId,

    #[strum(to_string = "/opd-er/vital-sign")]
    OpdErVitalSign,

    #[strum(to_string = "/prescription/screen")]
    PrescrptionScreen,

    #[strum(to_string = "/refer-note-vnan/{vnan}")]
    ReferNoteVnan,

    #[strum(to_string = "/report/custom")]
    ReportCustom,

    #[strum(to_string = "/report/raw-query")]
    ReportRawQuery,

    #[strum(to_string = "/report/raw-template-type-id/{template}/{type}/{id}")]
    ReportRawTemplateTypeId,

    #[strum(to_string = "/report/template-type-id/{template}/{type}/{id}")]
    ReportTemplateTypeId,

    #[strum(to_string = "/scan/his/image")]
    ScanHisImage,

    #[strum(to_string = "/search/box/hosp-text/{search_text}")]
    SearchBoxHospText,

    #[strum(to_string = "/search/box/med/duplicate")]
    SearchBoxMedDuplicate,

    #[strum(to_string = "/search/box/med/interaction")]
    SearchBoxMedInteraction,

    #[strum(to_string = "/search/box/med-hn-text/{hn}/{search_text}")]
    SearchBoxMedHnText,

    #[strum(to_string = "/search/box/opd-visit-mode-text/{mode}/{search_text}")]
    SearchBoxOpdVisitModeText,

    #[strum(to_string = "/search/box/ivfluid-text/{search_text}")]
    SearchBoxIvfluidText,

    #[strum(to_string = "/search/box/lab-text/{search_text}")]
    SearchBoxLabText,

    #[strum(to_string = "/search/box/patient-text/{search_text}")]
    SearchBoxPatientText,

    #[strum(to_string = "/search/box/xray-text/{search_text}")]
    SearchBoxXrayText,

    #[strum(to_string = "/search/dr")]
    SearchDr,

    #[strum(to_string = "/search/nurse")]
    SearchNurse,

    #[strum(to_string = "/search/pharmacist")]
    SearchPharmacist,

    #[strum(to_string = "/search/other")]
    SearchOther,

    #[strum(to_string = "/sse")]
    Sse,

    #[strum(to_string = "/sse-group")]
    SseGroup,

    #[strum(to_string = "/sse-message")]
    SseMessage,

    #[strum(to_string = "/user")]
    User,

    #[strum(to_string = "/user-config")]
    UserConfig,

    #[strum(to_string = "/user-role/prelude")]
    UserRolePrelude,

    #[strum(to_string = "/user-role/role")]
    UserRoleRole,

    #[strum(to_string = "/user-role/user")]
    UserRoleUser,

    #[strum(to_string = "/xray/report-hn/{hn}")]
    XrayReportHn,

    #[strum(to_string = "/xray/read-id/{xn}")]
    XrayReadId,

    #[strum(to_string = "/xray/pacs/xn/{xn}")]
    XrayPacsXn,

    #[strum(to_string = "/unknown")]
    Unknown,
}

impl EndPoint {
    // please follow backend/src/route.rs
    /// is_pre_admit == use VN as AN and check OPD-ER permission instead (if not exists will bypass checking)
    pub fn is_allow(&self, method: &Method, permissions: &[Permission], is_pre_admit: bool) -> bool {
        match self {
            Self::AvatarOpdEr | Self::AvatarIpd => {
                matches!(method, &Method::GET)
            }
            Self::DrugUseDuration => match *method {
                Method::GET => {
                    permissions.contains(&Permission::IpdDoctorMainProgramAccess)
                        || permissions.contains(&Permission::IpdNurseMainProgramAccess)
                        || permissions.contains(&Permission::IpdPharmacyOrderMainProgramAccess)
                        || permissions.contains(&Permission::OpdErPharmacyOrderProgramAccess)
                }
                Method::POST => permissions.contains(&Permission::IpdPharmacyOrderMainProgramAccess) || permissions.contains(&Permission::OpdErPharmacyOrderProgramAccess),
                _ => false,
            },
            Self::EmrDateHn | Self::EmrVisitVn => match *method {
                Method::GET => permissions.contains(&Permission::EmrView),
                _ => false,
            },
            Self::ExistsKeyId => matches!(method, &Method::GET),
            Self::HisOperationAdmitAn | Self::HisMedPlanIpdAn | Self::HisIptDiagAn | Self::HisIptOprtAn => matches!(method, &Method::GET),
            Self::HisReferOutVnan => match *method {
                Method::GET => permissions.contains(&Permission::IpdOrderView) || permissions.contains(&Permission::OpdErOrderView),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IpdOrderAdd)
                        || permissions.contains(&Permission::IpdOrderEdit)
                        || permissions.contains(&Permission::OpdErOrderAdd)
                        || permissions.contains(&Permission::OpdErOrderEdit)
                }
                _ => false,
            },
            Self::Image => matches!(method, &Method::POST | &Method::PUT | &Method::DELETE),
            Self::ImageUsage => matches!(method, &Method::POST | &Method::DELETE),
            Self::ImageUsageId => matches!(method, &Method::GET),
            Self::IpdAdmissionNoteDrAn => {
                is_pre_admit
                    || match *method {
                        Method::GET => permissions.contains(&Permission::AdmissionNoteView),
                        _ => false,
                    }
            }
            Self::IpdAdmissionNoteDr => {
                is_pre_admit
                    || match *method {
                        Method::POST => permissions.contains(&Permission::AdmissionNoteAdd),
                        Method::PUT => permissions.contains(&Permission::AdmissionNoteEdit),
                        _ => false,
                    }
            }
            Self::IpdAdmissionNoteNurseAn => {
                is_pre_admit
                    || match *method {
                        Method::GET => permissions.contains(&Permission::IpdNurseAdmissionNoteView),
                        _ => false,
                    }
            }
            Self::IpdAdmissionNoteNurse => {
                is_pre_admit
                    || match *method {
                        Method::POST => permissions.contains(&Permission::IpdNurseAdmissionNoteAdd),
                        Method::PUT => permissions.contains(&Permission::IpdNurseAdmissionNoteEdit),
                        _ => false,
                    }
            }
            Self::IpdAdmissionNoteDrPharmCheckAn => {
                is_pre_admit
                    || match *method {
                        Method::PATCH => permissions.contains(&Permission::AdmissionNoteDrugAllergyCheck),
                        _ => false,
                    }
            }
            Self::IpdConsult => {
                is_pre_admit
                    || match *method {
                        Method::GET => permissions.contains(&Permission::IpdDoctorConsultView),
                        // TODO PUT
                        Method::POST => permissions.contains(&Permission::IpdDoctorConsultAdd) || permissions.contains(&Permission::IpdDoctorConsultEdit),
                        Method::DELETE => permissions.contains(&Permission::IpdDoctorConsultRemove),
                        _ => false,
                    }
            }
            Self::IpdConsultAn => {
                is_pre_admit
                    || match *method {
                        Method::GET => permissions.contains(&Permission::IpdDoctorConsultView),
                        _ => false,
                    }
            }
            Self::IpdConsultId => {
                is_pre_admit
                    || match *method {
                        Method::GET => permissions.contains(&Permission::IpdDoctorConsultView),
                        _ => false,
                    }
            }
            Self::IpdDcPlanAn => match *method {
                Method::GET => permissions.contains(&Permission::IpdNurseNoteView) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseNoteView)),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IpdNurseNoteAdd)
                        || permissions.contains(&Permission::IpdNurseNoteEdit)
                        || (is_pre_admit && (permissions.contains(&Permission::OpdErNurseNoteAdd) || permissions.contains(&Permission::OpdErNurseNoteEdit)))
                }
                Method::DELETE => permissions.contains(&Permission::IpdNurseNoteRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseNoteRemove)),
                _ => false,
            },
            Self::IpdDcPlanTmpDx | Self::IpdDcPlanTmpMed | Self::IpdDcPlanTmpEnv | Self::IpdDcPlanTmpTx | Self::IpdDcPlanTmpDiet => {
                is_pre_admit
                    || match *method {
                        Method::GET => permissions.contains(&Permission::IpdNurseNoteView) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseNoteView)),
                        // TODO PUT
                        Method::POST => permissions.contains(&Permission::NursingProgressnoteTemplateAdd) || permissions.contains(&Permission::NursingProgressnoteTemplateEdit),
                        Method::DELETE => permissions.contains(&Permission::NursingProgressnoteTemplateRemove),
                        _ => false,
                    }
            }
            Self::IpdDoctorInCharge => {
                is_pre_admit
                    || match *method {
                        // EmrView for TPR report
                        Method::GET => permissions.contains(&Permission::EmrView) || permissions.contains(&Permission::IpdDoctorInchargeView),
                        // TODO PUT
                        Method::POST => permissions.contains(&Permission::IpdDoctorInchargeAdd) || permissions.contains(&Permission::IpdDoctorInchargeEdit),
                        Method::DELETE => permissions.contains(&Permission::IpdDoctorInchargeRemove),
                        _ => false,
                    }
            }
            Self::IpdDocumentDatetimeAn | Self::IpdDocumentListVnAn => {
                matches!(method, &Method::GET)
            }
            Self::IpdDocumentScanAn => {
                matches!(method, &Method::GET | &Method::POST | &Method::DELETE)
            }
            Self::IpdFocusListAn | Self::IpdFocusNoteAn => match *method {
                Method::GET => permissions.contains(&Permission::IpdNurseNoteView) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseNoteView)),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IpdNurseNoteAdd)
                        || permissions.contains(&Permission::IpdNurseNoteEdit)
                        || (is_pre_admit && (permissions.contains(&Permission::OpdErNurseNoteAdd) || permissions.contains(&Permission::OpdErNurseNoteEdit)))
                }
                Method::DELETE => permissions.contains(&Permission::IpdNurseNoteRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseNoteRemove)),
                _ => false,
            },
            Self::IpdIndexActionId | Self::IpdIndexMonitorId | Self::IpdIndexPlanId => match *method {
                Method::DELETE => permissions.contains(&Permission::IpdNurseIndexRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseIndexRemove)),
                _ => false,
            },
            Self::IpdIndexAction | Self::IpdIndexMonitor | Self::IpdIndexPlan => match *method {
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IpdNurseIndexAdd)
                        || permissions.contains(&Permission::IpdNurseIndexEdit)
                        || (is_pre_admit && (permissions.contains(&Permission::OpdErNurseIndexAdd) || permissions.contains(&Permission::OpdErNurseIndexEdit)))
                }
                _ => false,
            },
            Self::IpdIndexMedPayAn => {
                matches!(method, &Method::GET)
            }
            Self::IpdIndexNoteId => match *method {
                Method::DELETE => permissions.contains(&Permission::IpdNurseIndexNoteRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseIndexNoteRemove)),
                _ => false,
            },
            Self::IpdIndexNote => match *method {
                Method::GET => permissions.contains(&Permission::IpdNurseIndexNoteView) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseIndexNoteView)),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IpdNurseIndexNoteAdd)
                        || permissions.contains(&Permission::IpdNurseIndexNoteEdit)
                        || (is_pre_admit && (permissions.contains(&Permission::OpdErNurseIndexNoteAdd) || permissions.contains(&Permission::OpdErNurseIndexNoteEdit)))
                }
                _ => false,
            },
            Self::IpdIndexPlanDateAn => match *method {
                Method::GET => permissions.contains(&Permission::IpdNurseIndexView) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseIndexView)),
                _ => false,
            },
            Self::IpdIo => match *method {
                // EmrView for TPR report
                Method::GET => permissions.contains(&Permission::EmrView) || permissions.contains(&Permission::IoView) || (is_pre_admit && permissions.contains(&Permission::OpdErIoView)),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IoAdd)
                        || permissions.contains(&Permission::IoEdit)
                        || (is_pre_admit && (permissions.contains(&Permission::OpdErIoAdd) || permissions.contains(&Permission::OpdErIoEdit)))
                }
                Method::DELETE => permissions.contains(&Permission::IoRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErIoRemove)),
                _ => false,
            },
            Self::IpdIoDateAn => match *method {
                Method::GET => permissions.contains(&Permission::IoView) || (is_pre_admit && permissions.contains(&Permission::OpdErIoView)),
                _ => false,
            },
            Self::IpdMedReconcile => match *method {
                Method::GET => permissions.contains(&Permission::MedReconciliationView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::MedReconciliationAdd) || permissions.contains(&Permission::MedReconciliationEdit),
                Method::PATCH => permissions.contains(&Permission::MedReconciliationEdit),
                Method::DELETE => permissions.contains(&Permission::MedReconciliationRemove),
                _ => false,
            },
            Self::IpdMedReconcileHosxpAn | Self::IpdMedReconcileLastDoseAn => match *method {
                Method::GET => permissions.contains(&Permission::MedReconciliationView),
                _ => false,
            },
            Self::IpdMedReconcileNoteId => match *method {
                Method::GET => permissions.contains(&Permission::EmrView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::MedReconciliationEdit),
                _ => false,
            },
            Self::IpdMedReconcileRemedVisitHn | Self::IpdMedReconcileRemedMed => match *method {
                Method::GET => permissions.contains(&Permission::EmrView),
                _ => false,
            },
            Self::IpdMra => match *method {
                Method::GET | Method::POST | Method::PUT | Method::DELETE => permissions.contains(&Permission::DataTypeAuditorUse),
                _ => false,
            },
            Self::IpdOrderItem => match *method {
                Method::GET => permissions.contains(&Permission::IpdOrderView) || (is_pre_admit && permissions.contains(&Permission::OpdErOrderView)),
                Method::PATCH => permissions.contains(&Permission::IpdNurseIndexEdit) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseIndexEdit)), // edit nurse_assign in index_plan modal
                _ => false,
            },
            Self::IpdOrderOnedayPreviousAn | Self::IpdOrderOrderDateAn | Self::IpdOrderPrevious | Self::IpdOrderProgressPrevious | Self::IpdOrderToHomeMedAn => match *method {
                Method::GET => permissions.contains(&Permission::IpdOrderView) || (is_pre_admit && permissions.contains(&Permission::OpdErOrderView)),
                _ => false,
            },
            Self::IpdOrderOrderId => match *method {
                Method::DELETE => permissions.contains(&Permission::IpdOrderRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErOrderRemove)),
                _ => false,
            },
            Self::IpdOrderOrder => match *method {
                Method::GET => permissions.contains(&Permission::IpdOrderView) || (is_pre_admit && permissions.contains(&Permission::OpdErOrderView)),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IpdOrderAdd)
                        || permissions.contains(&Permission::IpdOrderEdit)
                        || (is_pre_admit && (permissions.contains(&Permission::OpdErOrderAdd) || permissions.contains(&Permission::OpdErOrderEdit)))
                }
                Method::PATCH => {
                    permissions.contains(&Permission::IpdOrderConfirm)
                        || permissions.contains(&Permission::IpdOrderCheck)
                        || permissions.contains(&Permission::IpdOrderAccept)
                        || permissions.contains(&Permission::IpdOrderDone)
                        || (is_pre_admit
                            && (permissions.contains(&Permission::OpdErOrderConfirm)
                                || permissions.contains(&Permission::OpdErOrderCheck)
                                || permissions.contains(&Permission::OpdErOrderAccept)
                                || permissions.contains(&Permission::OpdErOrderDone)))
                }
                _ => false,
            },
            Self::IpdOrderPharmacy => match *method {
                Method::GET => permissions.contains(&Permission::IpdPharmacyOrderMainProgramAccess) || (is_pre_admit && permissions.contains(&Permission::OpdErPharmacyOrderProgramAccess)),
                _ => false,
            },
            Self::IpdOrderProgressNoteId => match *method {
                Method::DELETE => permissions.contains(&Permission::ProgressNoteRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErProgressNoteRemove)),
                _ => false,
            },
            Self::IpdOrderProgressNote => match *method {
                Method::GET => permissions.contains(&Permission::ProgressNoteView) || (is_pre_admit && permissions.contains(&Permission::OpdErProgressNoteView)),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::ProgressNoteAdd)
                        || permissions.contains(&Permission::ProgressNoteEdit)
                        || (is_pre_admit && (permissions.contains(&Permission::OpdErProgressNoteAdd) || permissions.contains(&Permission::OpdErProgressNoteEdit)))
                }
                _ => false,
            },
            Self::IpdPasscode => matches!(*method, Method::GET | Method::POST),
            Self::IpdPostAdmitList => match *method {
                Method::GET => permissions.contains(&Permission::IpdOrderView) || (is_pre_admit && permissions.contains(&Permission::OpdErOrderView)),
                _ => false,
            },
            Self::IpdPostAdmitCount => matches!(*method, Method::GET),
            Self::IpdPreAdmit => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView) || permissions.contains(&Permission::IpdOrderView),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::OpdErOrderAdd)
                        || permissions.contains(&Permission::OpdErOrderEdit)
                        || permissions.contains(&Permission::IpdOrderAdd)
                        || permissions.contains(&Permission::IpdOrderEdit)
                }
                Method::PATCH => permissions.contains(&Permission::OpdErOrderEdit) || permissions.contains(&Permission::IpdOrderEdit),
                // Method::DELETE => permissions.contains(&Permission::OpdErOrderRemove) || permissions.contains(&Permission::IpdOrderRemove),
                _ => false,
            },
            Self::IpdPreOrderMasterId => match *method {
                Method::DELETE => permissions.contains(&Permission::OpdErOrderRemove) || permissions.contains(&Permission::IpdOrderRemove),
                _ => false,
            },
            Self::IpdPreOrderMaster => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView) || permissions.contains(&Permission::IpdOrderView),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::OpdErOrderAdd)
                        || permissions.contains(&Permission::OpdErOrderEdit)
                        || permissions.contains(&Permission::IpdOrderAdd)
                        || permissions.contains(&Permission::IpdOrderEdit)
                }
                _ => false,
            },
            Self::IpdPreOrderInto => match *method {
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::OpdErOrderAdd)
                        || permissions.contains(&Permission::OpdErOrderEdit)
                        || permissions.contains(&Permission::IpdOrderAdd)
                        || permissions.contains(&Permission::IpdOrderEdit)
                }
                _ => false,
            },
            Self::IpdPreOrderOrderId => match *method {
                Method::DELETE => permissions.contains(&Permission::OpdErOrderRemove) || permissions.contains(&Permission::IpdOrderRemove),
                _ => false,
            },
            Self::IpdPreOrderOrder => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView) || permissions.contains(&Permission::IpdOrderView),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::OpdErOrderAdd)
                        || permissions.contains(&Permission::OpdErOrderEdit)
                        || permissions.contains(&Permission::IpdOrderAdd)
                        || permissions.contains(&Permission::IpdOrderEdit)
                }
                _ => false,
            },
            Self::IpdPreOrderProgressNoteId => match *method {
                Method::DELETE => permissions.contains(&Permission::OpdErProgressNoteRemove) || permissions.contains(&Permission::ProgressNoteRemove),
                _ => false,
            },
            Self::IpdPreOrderProgressNote => match *method {
                Method::GET => permissions.contains(&Permission::OpdErProgressNoteView) || permissions.contains(&Permission::ProgressNoteView),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::OpdErProgressNoteAdd)
                        || permissions.contains(&Permission::OpdErProgressNoteEdit)
                        || permissions.contains(&Permission::ProgressNoteAdd)
                        || permissions.contains(&Permission::ProgressNoteEdit)
                }
                _ => false,
            },
            Self::IpdShowPatientMainAn => matches!(method, &Method::GET),
            Self::IpdSummary => {
                match *method {
                    Method::GET => permissions.contains(&Permission::IpdDischargeSummaryView),
                    // TODO PUT
                    Method::POST => permissions.contains(&Permission::IpdDischargeSummaryAdd) || permissions.contains(&Permission::IpdDischargeSummaryEdit),
                    Method::PATCH => permissions.contains(&Permission::DataTypeAuditorUse),
                    _ => false,
                }
            }
            Self::IpdSummaryAudit => match *method {
                Method::GET | Method::POST | Method::DELETE => permissions.contains(&Permission::DataTypeAuditorUse),
                _ => false,
            },
            Self::IpdSummaryNoteId => {
                match *method {
                    Method::GET => permissions.contains(&Permission::IpdDischargeSummaryView) || permissions.contains(&Permission::DataTypeAuditorUse),
                    // TODO PUT
                    Method::POST | Method::PATCH => {
                        permissions.contains(&Permission::IpdDischargeSummaryAdd) || permissions.contains(&Permission::IpdDischargeSummaryEdit) || permissions.contains(&Permission::DataTypeAuditorUse)
                    }
                    Method::DELETE => permissions.contains(&Permission::IpdDischargeSummaryRemove) || permissions.contains(&Permission::DataTypeAuditorUse),
                    _ => false,
                }
            }
            Self::IpdSummaryStatusId => match *method {
                Method::GET => permissions.contains(&Permission::IpdDischargeSummaryView) || permissions.contains(&Permission::DataTypeAuditorUse),
                Method::PUT => permissions.contains(&Permission::IpdDischargeSummaryEdit) || permissions.contains(&Permission::DataTypeAuditorUse),
                _ => false,
            },
            Self::IpdTmpGroup | Self::IpdTmpSubgroup | Self::IpdTmpFocus | Self::IpdTmpGoal | Self::IpdTmpIntvt | Self::IpdTmpDlc => match *method {
                Method::GET => permissions.contains(&Permission::IpdNurseNoteView) || (is_pre_admit && permissions.contains(&Permission::OpdErNurseNoteView)),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::NursingProgressnoteTemplateAdd) || permissions.contains(&Permission::NursingProgressnoteTemplateEdit),
                Method::DELETE => permissions.contains(&Permission::NursingProgressnoteTemplateRemove),
                _ => false,
            },
            Self::IpdVitalSignId => match *method {
                Method::DELETE => permissions.contains(&Permission::VitalSignRemove) || (is_pre_admit && permissions.contains(&Permission::OpdErVitalSignRemove)),
                _ => false,
            },
            Self::IpdVitalSign => match *method {
                Method::GET => permissions.contains(&Permission::VitalSignView) || (is_pre_admit && permissions.contains(&Permission::OpdErVitalSignView)),
                Method::POST => permissions.contains(&Permission::VitalSignAdd) || (is_pre_admit && permissions.contains(&Permission::OpdErVitalSignAdd)),
                Method::PUT => permissions.contains(&Permission::VitalSignEdit) || (is_pre_admit && permissions.contains(&Permission::OpdErVitalSignEdit)),
                _ => false,
            },
            Self::LabHead | Self::LabItem | Self::LabWbcKeyValue => match *method {
                Method::GET => permissions.contains(&Permission::LabView),
                _ => false,
            },
            Self::LabReadId => match *method {
                Method::POST => permissions.contains(&Permission::IpdLabReadAdd),
                Method::DELETE => permissions.contains(&Permission::IpdLabReadRemove),
                _ => false,
            },
            Self::MedReconcileHn => match *method {
                Method::GET => permissions.contains(&Permission::EmrView),
                _ => false,
            },
            Self::OpdErDcPlanId => match *method {
                Method::GET => permissions.contains(&Permission::OpdErNurseNoteView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErNurseNoteAdd) || permissions.contains(&Permission::OpdErNurseNoteEdit),
                Method::DELETE => permissions.contains(&Permission::OpdErNurseNoteRemove),
                _ => false,
            },
            Self::OpdErDocumentListVnId => matches!(method, &Method::GET),
            Self::OpdErDocumentScanId => {
                matches!(method, &Method::GET | &Method::POST | &Method::DELETE)
            }
            Self::OpdErFocusListId | Self::OpdErFocusNoteId => match *method {
                Method::GET => permissions.contains(&Permission::OpdErNurseNoteView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErNurseNoteAdd) || permissions.contains(&Permission::OpdErNurseNoteEdit),
                Method::DELETE => permissions.contains(&Permission::OpdErNurseNoteRemove),
                _ => false,
            },
            Self::OpdErHisMedVn => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView),
                _ => false,
            },
            Self::OpdErIndexActionId | Self::OpdErIndexMonitorId | Self::OpdErIndexPlanId => match *method {
                Method::DELETE => permissions.contains(&Permission::OpdErNurseIndexRemove),
                _ => false,
            },
            Self::OpdErIndexAction | Self::OpdErIndexMonitor | Self::OpdErIndexPlan => match *method {
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErNurseIndexAdd) || permissions.contains(&Permission::OpdErNurseIndexEdit),
                _ => false,
            },
            Self::OpdErIo => match *method {
                // EmrView for TPR report
                Method::GET => permissions.contains(&Permission::EmrView) || permissions.contains(&Permission::OpdErIoView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErIoAdd) || permissions.contains(&Permission::OpdErIoEdit),
                Method::DELETE => permissions.contains(&Permission::OpdErIoRemove),
                _ => false,
            },
            Self::OpdErIoDateId => match *method {
                Method::GET => permissions.contains(&Permission::OpdErIoView),
                _ => false,
            },
            Self::OpdErMedicalHistory
            | Self::OpdErMedicalHistoryAllergy
            | Self::OpdErMedicalHistoryConsult
            | Self::OpdErMedicalHistoryFt
            | Self::OpdErMedicalHistoryScan
            | Self::OpdErMedicalHistoryScreen
            | Self::OpdErMedicalHistoryTrauma => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErOrderAdd) || permissions.contains(&Permission::OpdErOrderEdit),
                _ => false,
            },
            Self::OpdErMedReconcile => match *method {
                Method::GET => permissions.contains(&Permission::MedReconciliationView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::MedReconciliationAdd) || permissions.contains(&Permission::MedReconciliationEdit),
                Method::PATCH => permissions.contains(&Permission::MedReconciliationEdit),
                Method::DELETE => permissions.contains(&Permission::MedReconciliationRemove),
                _ => false,
            },
            Self::OpdErMedReconcileNoteId => match *method {
                Method::GET => permissions.contains(&Permission::EmrView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::MedReconciliationEdit),
                _ => false,
            },
            Self::OpdErOrderMasterCheckVn | Self::OpdErOrderMasterId => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView),
                _ => false,
            },
            Self::OpdErOrderItem => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView),
                Method::PATCH => permissions.contains(&Permission::OpdErNurseIndexEdit), // edit nurse_assign in index_plan modal
                _ => false,
            },
            Self::OpdErOrderMaster => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErOrderAdd) || permissions.contains(&Permission::OpdErOrderEdit),
                _ => false,
            },
            Self::OpdErOrderOrderId => match *method {
                Method::DELETE => permissions.contains(&Permission::OpdErOrderRemove),
                _ => false,
            },
            Self::OpdErOrderOrder => match *method {
                Method::GET => permissions.contains(&Permission::OpdErOrderView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErOrderAdd) || permissions.contains(&Permission::OpdErOrderEdit),
                Method::PATCH => {
                    permissions.contains(&Permission::OpdErOrderConfirm)
                        || permissions.contains(&Permission::OpdErOrderCheck)
                        || permissions.contains(&Permission::OpdErOrderAccept)
                        || permissions.contains(&Permission::OpdErOrderDone)
                }
                _ => false,
            },
            Self::OpdErOrderProgressNoteId => match *method {
                Method::DELETE => permissions.contains(&Permission::OpdErProgressNoteRemove),
                _ => false,
            },
            Self::OpdErOrderProgressNote => match *method {
                Method::GET => permissions.contains(&Permission::OpdErProgressNoteView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::OpdErProgressNoteAdd) || permissions.contains(&Permission::OpdErProgressNoteEdit),
                _ => false,
            },
            Self::OpdErOrderPharmacy => match *method {
                Method::GET => permissions.contains(&Permission::OpdErPharmacyOrderProgramAccess),
                _ => false,
            },
            Self::OpdErShowPatientMainId | Self::OpdErShowPatientMainVn => {
                matches!(method, &Method::GET)
            }
            Self::OpdErVitalSignId => match *method {
                Method::DELETE => permissions.contains(&Permission::OpdErVitalSignRemove),
                _ => false,
            },
            Self::OpdErVitalSign => match *method {
                Method::GET => permissions.contains(&Permission::OpdErVitalSignView),
                Method::POST => permissions.contains(&Permission::OpdErVitalSignAdd),
                Method::PUT => permissions.contains(&Permission::OpdErVitalSignEdit),
                _ => false,
            },
            Self::PrescrptionScreen => match *method {
                Method::GET => permissions.contains(&Permission::EmrView),
                Method::POST | Method::PATCH => permissions.contains(&Permission::IpdPharmacyOrderMainProgramAccess),
                _ => false,
            },
            Self::ReferNoteVnan => match *method {
                Method::GET => permissions.contains(&Permission::IpdOrderView) || permissions.contains(&Permission::OpdErOrderView),
                // TODO PUT
                Method::POST => {
                    permissions.contains(&Permission::IpdOrderAdd)
                        || permissions.contains(&Permission::IpdOrderEdit)
                        || permissions.contains(&Permission::OpdErOrderAdd)
                        || permissions.contains(&Permission::OpdErOrderEdit)
                }
                _ => false,
            },
            Self::ReportCustom => match *method {
                Method::GET => permissions.contains(&Permission::EmrView) || permissions.contains(&Permission::SystemAcReportView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::SystemAcReportAdd) || permissions.contains(&Permission::SystemAcReportEdit),
                Method::DELETE => permissions.contains(&Permission::SystemAcReportRemove),
                _ => false,
            },
            Self::ReportRawQuery => match *method {
                Method::POST => permissions.contains(&Permission::SystemAcReportView),
                _ => false,
            },
            Self::ReportRawTemplateTypeId => match *method {
                Method::GET => permissions.contains(&Permission::EmrView) || permissions.contains(&Permission::SystemAcReportView),
                _ => false,
            },
            Self::ReportTemplateTypeId => match *method {
                Method::GET => permissions.contains(&Permission::IpdDocumentPrint) || permissions.contains(&Permission::OpdErDocumentPrint) || permissions.contains(&Permission::SystemAcReportView),
                _ => false,
            },
            Self::ScanHisImage => matches!(method, &Method::GET),
            Self::SearchBoxHospText
            | Self::SearchBoxMedDuplicate
            | Self::SearchBoxMedInteraction
            | Self::SearchBoxMedHnText
            | Self::SearchBoxOpdVisitModeText
            | Self::SearchBoxIvfluidText
            | Self::SearchBoxLabText
            | Self::SearchBoxPatientText
            | Self::SearchBoxXrayText => matches!(method, &Method::GET),
            Self::SearchDr => match *method {
                Method::GET => permissions.contains(&Permission::IpdDoctorMainProgramAccess),
                _ => false,
            },
            Self::SearchNurse => match *method {
                Method::GET => permissions.contains(&Permission::IpdNurseMainProgramAccess),
                _ => false,
            },
            Self::SearchPharmacist => match *method {
                Method::GET => permissions.contains(&Permission::IpdPharmacyOrderMainProgramAccess),
                _ => false,
            },
            Self::SearchOther => match *method {
                Method::GET => permissions.contains(&Permission::IpdOtherOrderMainProgramAccess),
                _ => false,
            },
            Self::Sse => matches!(*method, Method::GET | Method::DELETE),
            Self::SseGroup => matches!(*method, Method::POST),
            Self::SseMessage => matches!(*method, Method::GET | Method::POST | Method::PATCH),
            Self::User => matches!(*method, Method::GET | Method::POST | Method::PUT | Method::PATCH),
            Self::UserConfig => match *method {
                Method::POST => true,
                Method::PATCH => permissions.contains(&Permission::SystemAcRoleUserEdit),
                _ => false,
            },
            Self::UserRolePrelude => match *method {
                Method::GET => permissions.contains(&Permission::SystemAcRoleView),
                _ => false,
            },
            Self::UserRoleRole => match *method {
                Method::GET => permissions.contains(&Permission::SystemAcRolePermissionView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::SystemAcRolePermissionAdd) || permissions.contains(&Permission::SystemAcRolePermissionEdit),
                Method::DELETE => permissions.contains(&Permission::SystemAcRolePermissionRemove),
                _ => false,
            },
            Self::UserRoleUser => match *method {
                Method::GET => permissions.contains(&Permission::SystemAcRoleUserView),
                // TODO PUT
                Method::POST => permissions.contains(&Permission::SystemAcRoleUserAdd) || permissions.contains(&Permission::SystemAcRoleUserEdit),
                _ => false,
            },
            Self::XrayReportHn => matches!(*method, Method::GET),
            Self::XrayReadId => matches!(*method, Method::POST | Method::DELETE),
            Self::XrayPacsXn => matches!(*method, Method::GET),
            Self::Unknown => false,
        }
    }

    pub fn base(&self) -> String {
        [crate::API_PREFIX, self.deref().split('{').next().unwrap_or_default()].concat()
    }

    /// create '/base/sub1/sub2'
    pub fn path<S>(&self, sub: &[S]) -> String
    where
        S: std::string::ToString,
    {
        self.path_query_inner(sub, "")
    }

    /// create '/base?param1=xx&param2=yy'
    pub fn query<Q>(&self, params: &Q) -> String
    where
        Q: QueryString + ?Sized,
    {
        [self.base(), params.query_string()].concat()
    }

    /// create '/base/sub1/sub2?param1=xx&param2=yy'
    pub fn path_query<S, Q>(&self, sub: &[S], params: Option<&Q>) -> String
    where
        Q: QueryString + ?Sized,
        S: std::string::ToString,
    {
        let query = params.map(|q| q.query_string()).unwrap_or_default();
        self.path_query_inner(sub, &query)
    }

    /// create '/base/sub1/sub2?param1=xx&param2=yy'
    pub fn path_query_string<S>(&self, sub: &[S], params: Option<S>) -> String
    where
        S: std::string::ToString,
    {
        let query = params.map(|s| s.to_string()).unwrap_or_default();
        self.path_query_inner(sub, &query)
    }

    fn path_query_inner<S>(&self, sub: &[S], query: &str) -> String
    where
        S: std::string::ToString,
    {
        let base = self.base();
        let base_slash = base.ends_with('/');
        [
            base,
            if sub.is_empty() {
                String::new()
            } else {
                let must_slash = if base_slash { "" } else { "/" };
                let sub_concat = sub.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("/");
                [must_slash, &sub_concat].concat()
            },
            query.to_owned(),
        ]
        .concat()
    }
}

impl std::ops::Deref for EndPoint {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        std::convert::Into::<&'static str>::into(self)
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;
    use strum::VariantArray;

    use super::*;
    use crate::user::permission::Permission;

    #[test]
    fn test_endpoint_is_allow() {
        // method related only
        assert!(EndPoint::SseMessage.is_allow(&Method::GET, &[], false));
        assert!(!EndPoint::SseMessage.is_allow(&Method::PUT, &[], false));
        // method and permission
        assert!(EndPoint::IpdAdmissionNoteDr.is_allow(&Method::POST, &[Permission::AdmissionNoteAdd], false));
        assert!(!EndPoint::IpdAdmissionNoteDr.is_allow(&Method::DELETE, &[Permission::AdmissionNoteAdd], false));
        assert!(EndPoint::IpdOrderOrder.is_allow(&Method::GET, &[Permission::OpdErOrderView], true));
        assert!(EndPoint::IpdOrderOrder.is_allow(&Method::GET, &[Permission::IpdOrderView], true));
    }

    fn create_endpoints_allow(permissions: &[Permission]) -> HashSet<(Method, &EndPoint)> {
        let mut allows = HashSet::new();
        for endpoint in EndPoint::VARIANTS {
            for method in [Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE] {
                if endpoint.is_allow(&method, permissions, false) {
                    allows.insert((method, endpoint));
                }
            }
        }
        allows
    }

    #[test]
    fn test_endpoints_allow_with_permission() {
        // settings here
        // describe all permissions you want to check endpoints allowance
        // and run test with `verbose = true`
        let permissions = [Permission::AdmissionNoteAdd];
        let verbose = false;
        // end setting

        let base_allows = create_endpoints_allow(&[]);
        let permissions_allow = create_endpoints_allow(&permissions);
        let permissions_extra_allow = permissions_allow.difference(&base_allows);

        if verbose {
            println!("\nBase allows:\n============");
            for (method, endpoint) in &base_allows {
                println!("{:?} {:?}", method, endpoint);
            }

            println!("\nExtra allows:\n=============");
            for (method, endpoint) in permissions_extra_allow {
                println!("{:?} {:?}", method, endpoint);
            }
        }

        assert_eq!(base_allows.len(), 54);
    }
}
