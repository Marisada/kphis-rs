use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use utoipa::ToSchema;
use wasm_bindgen::JsCast;
use web_sys::Url;

use crate::{
    app::{AppState, WINDOW},
    endpoint::EndPoint,
    fetch::Method,
    tab::Tab,
    user::permission::Permission,
};

/// Client routes
#[derive(Clone, Debug, Demo, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(Self::demo_info()))]
pub enum Route {
    Root,
    NotFound {
        path: String,
    },
    UnAuthorized {
        hash: String,
    },
    Image,
    Index,
    Info,

    IpdIndexPlan,
    IpdOrderPharmacy,
    IpdSearchPatientDr,
    IpdSearchPatientNurse,
    IpdSearchPatientPharmacist,
    IpdSearchPatientOther,
    IpdVitalSign,

    OpdErIndexPlan,
    OpdErOrderPharmacy,
    OpdErVitalSign,

    DrugUseDuration,
    SettingTemplateNurseNote,
    SettingTemplateDcPlan,

    UserList,
    PermissionList,
    ReportDesigner,

    ReportViewer,

    IpdAdmissionNoteDr {
        an: String,
    },
    IpdAdmissionNoteNurse {
        an: String,
    },
    IpdConsultList {
        view_by: String,
    },
    IpdMain {
        view_by: String,
        an: String,
        tab: String,
        sub: String,
        id: u32,
    },
    IpdMra {
        an: String,
    },
    IpdPostAdmitList {
        view_by: String,
    },
    IpdPreAdmitList {
        view_by: String,
    },
    IpdPreOrder {
        view_by: String,
        pre_order_master_id: u32,
    },
    IpdPreOrderList {
        view_by: String,
    },
    IpdSummaryAudit {
        an: String,
    },

    OpdErMain {
        view_by: String,
        opd_er_order_master_id: u32,
        tab: String,
        id: u32,
    },
    OpdErOrderList {
        view_by: String,
    },

    PrescriptionScreen {
        hn: String,
    },

    Summary {
        view_by: String,
        an: String,
    },
    /// valid path ex: http://some/thing
    External {
        path: String,
    },
}

impl Route {
    pub fn string(&self) -> String {
        match self {
            Self::Root => "#".to_owned(),
            Self::NotFound { path } => ["#/notfound/", path].concat(),
            Self::UnAuthorized { hash } => ["#/unauthorized/", hash].concat(),
            Self::Image => "#/image".to_owned(),
            Self::Index => "#/index".to_owned(),
            Self::Info => "#/info".to_owned(),

            Self::IpdIndexPlan => "#/ipd-index-plan".to_owned(),
            Self::IpdOrderPharmacy => "#/ipd-order-pharmacy".to_owned(),
            Self::IpdSearchPatientDr => "#/ipd-search-patient-dr".to_owned(),
            Self::IpdSearchPatientNurse => "#/ipd-search-patient-nurse".to_owned(),
            Self::IpdSearchPatientPharmacist => "#/ipd-search-patient-pharmacist".to_owned(),
            Self::IpdSearchPatientOther => "#/ipd-search-patient-other".to_owned(),
            Self::IpdVitalSign => "#/ipd-vital-sign".to_owned(),

            Self::OpdErIndexPlan => "#/opd-er-index-plan".to_owned(),
            Self::OpdErOrderPharmacy => "#/opd-er-order-pharmacy".to_owned(),
            Self::OpdErVitalSign => "#/opd-er-vital-sign".to_owned(),

            Self::IpdAdmissionNoteDr { an } => ["#/ipd-admission-note-dr/", an].concat(),
            Self::IpdAdmissionNoteNurse { an } => ["#/ipd-admission-note-nurse/", an].concat(),
            Self::Summary { view_by, an } => ["#/summary", view_by, an].join("/"),

            Self::IpdConsultList { view_by } => ["#/ipd-consult-list/", view_by].concat(),
            Self::IpdMain { view_by, an, tab, sub, id } => ["#/ipd-main", view_by, an, tab, sub, &id.to_string()].join("/"),
            Self::IpdMra { an } => ["#/ipd-mra/", an].concat(),
            Self::IpdPostAdmitList { view_by } => ["#/ipd-post-admit-list/", view_by].concat(),
            Self::IpdPreAdmitList { view_by } => ["#/ipd-pre-admit-list/", view_by].concat(),
            Self::IpdPreOrder { view_by, pre_order_master_id } => ["#/ipd-pre-order", view_by, &pre_order_master_id.to_string()].join("/"),
            Self::IpdPreOrderList { view_by } => ["#/ipd-pre-order-list/", view_by].concat(),
            Self::IpdSummaryAudit { an } => ["#/ipd-summary-audit/", an].concat(),

            Self::OpdErMain {
                view_by,
                opd_er_order_master_id,
                tab,
                id,
            } => ["#/opd-er-main", view_by, &opd_er_order_master_id.to_string(), tab, &id.to_string()].join("/"),
            Self::OpdErOrderList { view_by } => ["#/opd-er-order-list/", view_by].concat(),

            Self::PrescriptionScreen { hn } => ["#/prescription-screen/", hn].concat(),

            Self::DrugUseDuration => "#/drug-use-duration".to_owned(),
            Self::SettingTemplateNurseNote => "#/setting-template-nurse-note".to_owned(),
            Self::SettingTemplateDcPlan => "#/setting-template-dc-plan".to_owned(),

            Self::UserList => "#/user-list".to_owned(),
            Self::PermissionList => "#/permission-list".to_owned(),
            Self::ReportDesigner => "#/report-designer".to_owned(),

            Self::ReportViewer => "#/report-viewer".to_owned(),

            Self::External { path } => path.to_owned(),
        }
    }

    pub fn hard_redirect(&self) {
        WINDOW.with(|w| {
            if let Self::External { path } = self {
                if let Err(e) = w.open_with_url_and_target(path, "_blank") {
                    if let Some(message) = e.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)) {
                        w.document().unwrap().get_element_by_id("errormessage").unwrap().set_text_content(Some(&message));
                        log::error!("{}", message);
                    }
                }
            } else {
                let location = w.location();
                if let Err(e) = location.assign(&self.string()) {
                    if let Some(message) = e.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)) {
                        w.document().unwrap().get_element_by_id("errormessage").unwrap().set_text_content(Some(&message));
                        log::error!("{}", message);
                    }
                }
            }
        })
    }

    // NOTE: the gold standard in url naming is using kebab-case
    /// support<br>
    /// - #/some/thing <br>
    /// - http://host/#/some/thing
    pub fn from_url(url: &str, host: &str) -> Self {
        if url.is_empty() {
            return Self::NotFound { path: String::new() };
        }
        if url.starts_with("#/") {
            return Self::from_hash(url);
        }

        match Url::new(url).map(|u| {
            let h = u.hash();
            // NOTE "http://xxx/#" has pathname() == "/" and hash() == ""
            (h.clone(), (u.host().as_str() == host && u.pathname().as_str() == "/") && (h.starts_with("#/") || h.is_empty()))
        }) {
            Ok((hash, is_accepted)) => {
                if is_accepted {
                    Self::from_hash(&hash)
                } else {
                    Self::External { path: url.to_owned() }
                }
            }
            Err(_) => Self::External { path: url.to_owned() },
        }
    }

    pub fn from_hash(hash: &str) -> Self {
        if hash.starts_with("#/notfound/") {
            return Self::NotFound {
                path: hash.strip_prefix("#/notfound/").unwrap_or(hash).to_owned(),
            };
        }
        let url_parts = hash.split('/').collect::<Vec<&str>>();
        match url_parts.len() {
            // #
            1 => Self::Root,
            // #/xxx
            2 => match url_parts[1] {
                "" => Self::Root,
                "drug-use-duration" => Self::DrugUseDuration,
                "image" => Self::Image,
                "index" => Self::Index,
                "info" => Self::Info,
                "ipd-index-plan" => Self::IpdIndexPlan,
                "ipd-order-pharmacy" => Self::IpdOrderPharmacy,
                "ipd-search-patient-dr" => Self::IpdSearchPatientDr,
                "ipd-search-patient-nurse" => Self::IpdSearchPatientNurse,
                "ipd-search-patient-pharmacist" => Self::IpdSearchPatientPharmacist,
                "ipd-search-patient-other" => Self::IpdSearchPatientOther,
                "ipd-vital-sign" => Self::IpdVitalSign,
                "opd-er-index-plan" => Self::OpdErIndexPlan,
                "opd-er-order-pharmacy" => Self::OpdErOrderPharmacy,
                "opd-er-vital-sign" => Self::OpdErVitalSign,
                "permission-list" => Self::PermissionList,
                "report-viewer" => Self::ReportViewer,
                "report-designer" => Self::ReportDesigner,
                "setting-template-dc-plan" => Self::SettingTemplateDcPlan,
                "setting-template-nurse-note" => Self::SettingTemplateNurseNote,
                "user-list" => Self::UserList,
                _ => Self::NotFound { path: hash.to_owned() },
            },
            // #/xxx/yyy
            3 => match url_parts[1] {
                "ipd-admission-note-dr" => Self::IpdAdmissionNoteDr { an: url_parts[2].to_owned() },
                "ipd-admission-note-nurse" => Self::IpdAdmissionNoteNurse { an: url_parts[2].to_owned() },
                "ipd-consult-list" => Self::IpdConsultList { view_by: url_parts[2].to_owned() },
                "ipd-mra" => Self::IpdMra { an: url_parts[2].to_owned() },
                "ipd-post-admit-list" => Self::IpdPostAdmitList { view_by: url_parts[2].to_owned() },
                "ipd-pre-admit-list" => Self::IpdPreAdmitList { view_by: url_parts[2].to_owned() },
                "ipd-pre-order-list" => Self::IpdPreOrderList { view_by: url_parts[2].to_owned() },
                "ipd-summary-audit" => Self::IpdSummaryAudit { an: url_parts[2].to_owned() },
                "opd-er-order-list" => Self::OpdErOrderList { view_by: url_parts[2].to_owned() },
                "prescription-screen" => Self::PrescriptionScreen { hn: url_parts[2].to_owned() },
                _ => Self::NotFound { path: hash.to_owned() },
            },
            4 => match url_parts[1] {
                // #/unauthorized/#/xxx
                "unauthorized" => Self::UnAuthorized { hash: ["#/", url_parts[3]].concat() },
                "ipd-pre-order" => Self::IpdPreOrder {
                    view_by: url_parts[2].to_owned(),
                    pre_order_master_id: url_parts[3].parse::<u32>().unwrap_or_default(),
                },
                "summary" => Self::Summary {
                    view_by: url_parts[2].to_owned(),
                    an: url_parts[3].to_owned(),
                },
                _ => Self::NotFound { path: hash.to_owned() },
            },
            // #/xxx/yyy/zzz/aaa/bbb
            6 => match url_parts[1] {
                "opd-er-main" => Self::OpdErMain {
                    view_by: url_parts[2].to_owned(),
                    opd_er_order_master_id: url_parts[3].parse::<u32>().unwrap_or_default(),
                    tab: url_parts[4].to_owned(),
                    id: url_parts[5].parse::<u32>().unwrap_or_default(),
                },
                _ => Self::NotFound { path: hash.to_owned() },
            },
            7 => match url_parts[1] {
                "ipd-main" => Self::IpdMain {
                    view_by: url_parts[2].to_owned(),
                    an: url_parts[3].to_owned(),
                    tab: url_parts[4].to_owned(),
                    sub: url_parts[5].to_owned(),
                    id: url_parts[6].parse::<u32>().unwrap_or_default(),
                },
                _ => Self::NotFound { path: hash.to_owned() },
            },
            _ => Self::NotFound { path: hash.to_owned() },
        }
    }

    pub fn has_permission(&self, app: Rc<AppState>) -> bool {
        if app.is_production() {
            match self {
                Self::IpdAdmissionNoteDr { an } => {
                    let is_pre_admit = app.is_pre_admit(an);
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdAdmissionNoteDrAn, is_pre_admit)
                        && if is_pre_admit {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, true)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false)
                        }
                }
                Self::IpdAdmissionNoteNurse { an } => {
                    let is_pre_admit = app.is_pre_admit(an);
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdAdmissionNoteNurseAn, is_pre_admit)
                        && if is_pre_admit {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, true)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false)
                        }
                }
                Self::IpdConsultList { view_by } => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdConsult, false) && check_permission_view_by(&view_by, app.clone()),
                Self::IpdIndexPlan => {
                    app.has_permission(Permission::IpdNurseMainProgramAccess)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::AvatarIpd, false)
                        && (app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderItem, true) || app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderItem, false))
                        && (app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false) || app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, false))
                }
                Self::IpdMain { view_by, an, tab, .. } => {
                    let is_pre_admit = app.is_pre_admit(an);
                    (if is_pre_admit {
                        app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, true)
                    } else {
                        app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false)
                    }) && check_permission_view_by(&view_by, app.clone())
                        && check_permission_tab_ipd(&tab, is_pre_admit, app.clone())
                }
                Self::IpdMra { an } => {
                    let is_pre_admit = app.is_pre_admit(an);
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMra, is_pre_admit)
                        && if is_pre_admit {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, true)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false)
                        }
                }
                Self::IpdOrderPharmacy => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderPharmacy, false),
                Self::IpdPostAdmitList { view_by } => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPostAdmitList, false) && check_permission_view_by(&view_by, app.clone()),
                Self::IpdPreAdmitList { view_by } => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreAdmit, false) && check_permission_view_by(&view_by, app.clone()),
                Self::IpdPreOrder { view_by, .. } => {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderMaster, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxPatientText, false)
                        && check_permission_view_by(&view_by, app.clone())
                }
                Self::IpdPreOrderList { view_by } => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderMaster, false) && check_permission_view_by(&view_by, app.clone()),
                Self::IpdSearchPatientDr => {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::SearchDr, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPasscode, false)
                        && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPasscode, false)
                }
                Self::IpdSearchPatientNurse => {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::SearchNurse, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPasscode, false)
                        && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPasscode, false)
                }
                Self::IpdSearchPatientOther => app.endpoint_is_allow(&Method::GET, &EndPoint::SearchOther, false),
                Self::IpdSearchPatientPharmacist => app.endpoint_is_allow(&Method::GET, &EndPoint::SearchPharmacist, false),
                Self::IpdSummaryAudit { an } => {
                    let is_pre_admit = app.is_pre_admit(an);
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdSummaryAudit, is_pre_admit)
                        && if is_pre_admit {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, true)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false)
                        }
                }
                Self::IpdVitalSign => {
                    app.has_permission(Permission::IpdNurseMainProgramAccess)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::AvatarIpd, false)
                        && (app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false) || app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, true))
                        && (app.endpoint_is_allow(&Method::GET, &EndPoint::IpdVitalSign, true) || app.endpoint_is_allow(&Method::GET, &EndPoint::IpdVitalSign, false))
                }
                Self::OpdErIndexPlan => {
                    app.has_permission(Permission::OpdErNurseProgramAccess)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::AvatarOpdEr, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderItem, false)
                        && (app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, false) || app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainId, false))
                }
                Self::OpdErMain { view_by, tab, .. } => {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderMasterId, false)
                        && (app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, false) || app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainId, false))
                        && check_permission_view_by(&view_by, app.clone())
                        && check_permission_tab_opd_er(&tab, app.clone())
                }
                Self::OpdErOrderList { view_by } => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderMaster, false) && check_permission_view_by(&view_by, app.clone()),
                Self::OpdErOrderPharmacy => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderPharmacy, false),
                Self::OpdErVitalSign => {
                    app.has_permission(Permission::OpdErNurseProgramAccess)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::AvatarOpdEr, false)
                        && (app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, false) || app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainId, false))
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErVitalSign, false)
                }
                Self::PermissionList => app.endpoint_is_allow(&Method::GET, &EndPoint::UserRolePrelude, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::UserRoleRole, false),
                Self::PrescriptionScreen { .. } => {
                    (app.has_permission(Permission::IpdPharmacyOrderMainProgramAccess) || app.has_permission(Permission::OpdErPharmacyOrderProgramAccess))
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::PrescrptionScreen, false)
                }
                Self::DrugUseDuration => app.endpoint_is_allow(&Method::POST, &EndPoint::DrugUseDuration, false),
                Self::ReportViewer => {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::ReportCustom, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::AvatarIpd, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::AvatarOpdEr, false)
                }
                Self::ReportDesigner => {
                    app.is_production()
                        && app.endpoint_is_allow(&Method::POST, &EndPoint::ReportRawQuery, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::ReportCustom, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false)
                }
                Self::SettingTemplateDcPlan => {
                    app.has_permission(Permission::NursingProgressnoteTemplateView)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpDx, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpMed, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpEnv, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpTx, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpDiet, false)
                }
                Self::SettingTemplateNurseNote => {
                    app.has_permission(Permission::NursingProgressnoteTemplateView)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpGroup, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpSubgroup, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpFocus, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpGoal, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpIntvt, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpDlc, false)
                }
                Self::Summary { view_by, an } => {
                    let is_pre_admit = app.is_pre_admit(an);
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdSummary, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdSummaryNoteId, is_pre_admit)
                        && if is_pre_admit {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, true)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false)
                        }
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxHospText, false)
                        && check_permission_view_by(&view_by, app.clone())
                }
                Self::UserList => app.endpoint_is_allow(&Method::GET, &EndPoint::UserRolePrelude, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::UserRoleUser, false),
                Self::Image | Self::Index | Self::Info | Self::Root | Self::NotFound { .. } | Self::UnAuthorized { .. } | Self::External { .. } => true,
            }
        } else {
            true
        }
    }
}

pub fn check_permission_view_by(view_by: &str, app: Rc<AppState>) -> bool {
    match view_by {
        "doctor" => app.has_permission(Permission::DataTypeDoctorUse),
        "nurse" => app.has_permission(Permission::DataTypeNurseUse),
        "pharmacist" => app.has_permission(Permission::DataTypePharmacyUse),
        "other" => app.has_permission(Permission::DataTypeOtherUse),
        _ => false,
    }
}

fn check_permission_tab_ipd(tab: &str, is_pre_admit: bool, app: Rc<AppState>) -> bool {
    match Tab::from_string(tab) {
        Tab::MedReconcile => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit),
        Tab::Order => {
            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrderDateAn, is_pre_admit)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, is_pre_admit)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderPrevious, is_pre_admit)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderProgressNote, is_pre_admit)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit)
        }
        Tab::VitalSign => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdVitalSign, is_pre_admit),
        Tab::Io => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIoDateAn, is_pre_admit) && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIo, is_pre_admit),
        Tab::NurseNote => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdFocusNoteAn, is_pre_admit),
        Tab::NursePlan => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIndexPlanDateAn, is_pre_admit) && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderItem, is_pre_admit),
        Tab::Lab => app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false),
        Tab::XRay => app.has_pacs_host() && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayReportHn, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayPacsXn, false),
        Tab::Emr => app.endpoint_is_allow(&Method::GET, &EndPoint::EmrDateHn, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::EmrVisitVn, false),
        Tab::Document => app.has_permission(Permission::EmrView),
        // TabIpd::Operation => app.has_permission(Permission::OperationView),
        Tab::Doctor => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDoctorInCharge, is_pre_admit),
        Tab::Consult => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdConsultAn, is_pre_admit) && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdConsultId, is_pre_admit),
        Tab::ReferOut => app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, is_pre_admit),
        Tab::MedHx => false,
    }
}

fn check_permission_tab_opd_er(tab: &str, app: Rc<AppState>) -> bool {
    match Tab::from_string(tab) {
        Tab::MedHx => {
            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistory, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryTrauma, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryAllergy, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryScreen, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryScan, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryConsult, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryFt, false)
        }
        Tab::MedReconcile => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false),
        Tab::Order => {
            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderMasterCheckVn, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderOrder, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderProgressNote, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErHisMedVn, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false)
        }
        Tab::NursePlan => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderItem, false),
        Tab::Lab => app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false),
        Tab::XRay => app.has_pacs_host() && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayReportHn, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayPacsXn, false),
        Tab::NurseNote => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErFocusNoteId, false),
        Tab::VitalSign => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErVitalSign, false),
        Tab::Io => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErIoDateId, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErIo, false),
        Tab::Emr => app.endpoint_is_allow(&Method::GET, &EndPoint::EmrDateHn, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::EmrVisitVn, false),
        Tab::Document => app.has_permission(Permission::EmrView),
        Tab::ReferOut => app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderOrder, false),
        Tab::Doctor | Tab::Consult => false,
    }
}

// Integration tests at /tests/src/route.rs
