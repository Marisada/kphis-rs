use derive_demo::Demo;
use js_sys::JsString;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{collections::HashMap, rc::Rc, sync::Arc};
use strum::EnumIter;
use time::{PrimitiveDateTime, macros::datetime};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::{date_8601, datetime_8601, time_8601},
    error::{AppError, Source},
    util::explode_cap_pipe_iter,
};

use crate::{
    A4_HEIGHT, A4_WIDTH, DEFAULT_SVG_REPORT,
    app::{AppAsset, AppState},
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_text, fetch_json_api},
    ipd::document::IpdDocumentExists,
    select_utils::{ColorSelectOption, SelectOption},
};

#[derive(bitcode::Encode, bitcode::Decode)]
pub struct TypstSvg {
    pub width: f64,
    pub height: f64,
    pub svg: String,
}

impl Default for TypstSvg {
    fn default() -> Self {
        Self {
            width: A4_WIDTH,
            height: A4_HEIGHT,
            svg: DEFAULT_SVG_REPORT.to_owned(),
        }
    }
}

/// Typst template with data.json
#[derive(Clone, Demo, Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(TypstRaw::demo()))]
pub struct TypstRaw {
    #[Demo(value = r#"String::from("ipd-order")"#)]
    pub typ: String,
    #[Demo(value = r#"String::from("{\"id\":\"1\"}")"#)]
    pub data_json: String,
}

impl TypstRaw {
    /// GET `EndPoint::ReportRawTemplateTypeId`
    /// - template_name of coercion MUST BE `system template name`
    pub async fn call_api_get(template_name: &str, report_type: &str, ids: &str, app: Rc<AppState>) -> Result<Self, AppError> {
        let path = [&EndPoint::ReportRawTemplateTypeId.base(), template_name, "/", report_type, "/", ids].concat();
        match fetch_json_api(&path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PdfRaw"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PdfRaw"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

#[derive(Clone, Eq)]
pub enum TypstReport {
    System(SystemReport),
    /// custom_name, base_system, target_custom
    Coercion((String, SystemReport, Option<CustomReport>)),
    Custom(CustomReport),
}

impl PartialEq for TypstReport {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::System(a), Self::System(b)) => a.eq(b),
            (Self::Coercion((a, _, _)), Self::Coercion((b, _, _))) => a.eq(b),
            (Self::Custom(a), Self::Custom(b)) => a.eq(b),
            _ => false,
        }
    }
}

impl TypstReport {
    pub fn new_system_with_coercion(text: &str, report_coercions: Option<Arc<HashMap<String, String>>>) -> Option<Self> {
        SystemReport::new(text).map(|system| Self::from_system_with_coercion(system, &report_coercions))
    }

    pub fn from_system_with_coercion(system: SystemReport, report_coercions: &Option<Arc<HashMap<String, String>>>) -> Self {
        if let Some(custom) = report_coercions.as_ref().and_then(|coercion| coercion.get(system.template_name())) {
            Self::Coercion((custom.to_owned(), system, None))
        } else {
            Self::System(system)
        }
    }

    pub fn full_report(full_type: &str, exists: IpdDocumentExists, an: &str, report_coercions: &Option<Arc<HashMap<String, String>>>) -> Vec<(Self, String)> {
        match full_type {
            "full-labour" => Self::full_report_labour(exists, an, report_coercions),
            "full-psychia" => Self::full_report_psychia(exists, an, report_coercions),
            // "full-general"
            _ => Self::full_report_general(exists, an, report_coercions),
        }
    }

    // without IpdEventLog, IpdMRA, IpdPartographWho, IpdPartograph, IpdVitalSignLabour, IpdVitalSignPsychia
    /// return (template, id)
    fn full_report_general(exists: IpdDocumentExists, an: &str, report_coercions: &Option<Arc<HashMap<String, String>>>) -> Vec<(Self, String)> {
        let scan_template = TypstReport::from_system_with_coercion(SystemReport::DocumentImages, report_coercions);
        let mut reports = Vec::with_capacity(36);
        if exists.has_data_summary2 {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdSummary, report_coercions), an.to_owned()));
        }
        if exists.has_scan_consent {
            reports.push((scan_template.clone(), [an, "|1|1"].concat()));
        }
        if exists.has_scan_refer_in {
            reports.push((scan_template.clone(), [an, "|3|1"].concat()));
        }
        if exists.has_data_refer_out {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::ReferOut, report_coercions), an.to_owned()));
        }
        if exists.has_scan_refer_out {
            reports.push((scan_template.clone(), [an, "|4|1"].concat()));
        }
        if exists.has_data_dr_admission_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteDr, report_coercions), an.to_owned()));
        }
        if exists.has_data_nurse_admission_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteNurse, report_coercions), an.to_owned()));
        }
        if exists.has_data_med_reconciliation {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliation, report_coercions), an.to_owned()));
        }
        if exists.has_data_order || exists.has_data_progress_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdOrder, report_coercions), an.to_owned()));
        }
        if exists.has_data_dr_consult {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdConsult, report_coercions), an.to_owned()));
        }
        if exists.has_scan_oper {
            reports.push((scan_template.clone(), [an, "|12|1"].concat()));
        }
        if exists.has_scan_anes {
            reports.push((scan_template.clone(), [an, "|13|1"].concat()));
        }
        if exists.has_scan_labour {
            reports.push((scan_template.clone(), [an, "|14|1"].concat()));
        }
        if exists.has_scan_physio {
            reports.push((scan_template.clone(), [an, "|15|1"].concat()));
        }
        if exists.has_scan_culture {
            reports.push((scan_template.clone(), [an, "|5|1"].concat()));
        }
        if exists.has_data_lab {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::LabSummary, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::Lab, report_coercions), an.to_owned()));
        }
        if exists.has_scan_ekg {
            reports.push((scan_template.clone(), [an, "|8|1"].concat()));
        }
        if exists.has_scan_xray {
            reports.push((scan_template.clone(), [an, "|9|1"].concat()));
        }
        if exists.has_scan_ct {
            reports.push((scan_template.clone(), [an, "|10|1"].concat()));
        }
        if exists.has_scan_mri {
            reports.push((scan_template.clone(), [an, "|11|1"].concat()));
        }
        if exists.has_scan_special {
            reports.push((scan_template.clone(), [an, "|7|1"].concat()));
        }
        if exists.has_data_focus_list {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdFocusList, report_coercions), an.to_owned()));
        }
        if exists.has_data_focus_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdFocusNote, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdDischargePlan, report_coercions), an.to_owned()));
        }
        if exists.has_data_index_plan {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdIndexPlan, report_coercions), an.to_owned()));
        }
        if exists.has_data_vital_sign {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignGeneral, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignNeuro, report_coercions), an.to_owned()));
        }
        if exists.has_data_vital_sign {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdTPR, report_coercions), an.to_owned()));
        }
        if exists.has_data_io {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdIo, report_coercions), an.to_owned()));
        }
        if exists.has_data_index_plan {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdMAR, report_coercions), an.to_owned()));
        }
        if exists.has_scan_blood {
            reports.push((scan_template.clone(), [an, "|6|1"].concat()));
        }
        if exists.has_scan_opd_card {
            reports.push((scan_template.clone(), [an, "|20|1"].concat()));
        }
        if exists.has_scan_insure {
            reports.push((scan_template.clone(), [an, "|2|1"].concat()));
        }
        if exists.has_scan_alt_med {
            reports.push((scan_template.clone(), [an, "|16|1"].concat()));
        }
        if exists.has_scan_nutrition {
            reports.push((scan_template.clone(), [an, "|17|1"].concat()));
        }
        if exists.has_scan_other_sp_clinic {
            reports.push((scan_template.clone(), [an, "|19|1"].concat()));
        }
        if exists.has_scan_others {
            reports.push((scan_template.clone(), [an, "|18|1"].concat()));
        }
        if exists.has_scan_finance {
            reports.push((scan_template.clone(), [an, "|21|1"].concat()));
        }
        reports
    }

    // without IpdEventLog, IpdMRA, IpdPartograph, IpdVitalSignPsychia
    /// return (template, id)
    fn full_report_labour(exists: IpdDocumentExists, an: &str, report_coercions: &Option<Arc<HashMap<String, String>>>) -> Vec<(Self, String)> {
        let scan_template = TypstReport::from_system_with_coercion(SystemReport::DocumentImages, report_coercions);
        let mut reports = Vec::with_capacity(38);
        if exists.has_data_summary2 {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdSummary, report_coercions), an.to_owned()));
        }
        if exists.has_scan_consent {
            reports.push((scan_template.clone(), [an, "|1|1"].concat()));
        }
        if exists.has_scan_refer_in {
            reports.push((scan_template.clone(), [an, "|3|1"].concat()));
        }
        if exists.has_data_refer_out {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::ReferOut, report_coercions), an.to_owned()));
        }
        if exists.has_scan_refer_out {
            reports.push((scan_template.clone(), [an, "|4|1"].concat()));
        }
        if exists.has_data_dr_admission_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteDr, report_coercions), an.to_owned()));
        }
        if exists.has_data_nurse_admission_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteNurse, report_coercions), an.to_owned()));
        }
        if exists.has_data_med_reconciliation {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliation, report_coercions), an.to_owned()));
        }
        if exists.has_data_order || exists.has_data_progress_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdOrder, report_coercions), an.to_owned()));
        }
        if exists.has_data_dr_consult {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdConsult, report_coercions), an.to_owned()));
        }
        if exists.has_scan_oper {
            reports.push((scan_template.clone(), [an, "|12|1"].concat()));
        }
        if exists.has_scan_anes {
            reports.push((scan_template.clone(), [an, "|13|1"].concat()));
        }
        if exists.has_data_vital_sign {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdPartographWho, report_coercions), an.to_owned()));
        }
        if exists.has_scan_labour {
            reports.push((scan_template.clone(), [an, "|14|1"].concat()));
        }
        if exists.has_scan_physio {
            reports.push((scan_template.clone(), [an, "|15|1"].concat()));
        }
        if exists.has_scan_culture {
            reports.push((scan_template.clone(), [an, "|5|1"].concat()));
        }
        if exists.has_data_lab {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::LabSummary, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::Lab, report_coercions), an.to_owned()));
        }
        if exists.has_scan_ekg {
            reports.push((scan_template.clone(), [an, "|8|1"].concat()));
        }
        if exists.has_scan_xray {
            reports.push((scan_template.clone(), [an, "|9|1"].concat()));
        }
        if exists.has_scan_ct {
            reports.push((scan_template.clone(), [an, "|10|1"].concat()));
        }
        if exists.has_scan_mri {
            reports.push((scan_template.clone(), [an, "|11|1"].concat()));
        }
        if exists.has_scan_special {
            reports.push((scan_template.clone(), [an, "|7|1"].concat()));
        }
        if exists.has_data_focus_list {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdFocusList, report_coercions), an.to_owned()));
        }
        if exists.has_data_focus_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdFocusNote, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdDischargePlan, report_coercions), an.to_owned()));
        }
        if exists.has_data_index_plan {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdIndexPlan, report_coercions), an.to_owned()));
        }
        if exists.has_data_vital_sign {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignGeneral, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignNeuro, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignLabour, report_coercions), an.to_owned()));
        }
        if exists.has_data_vital_sign {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdTPR, report_coercions), an.to_owned()));
        }
        if exists.has_data_io {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdIo, report_coercions), an.to_owned()));
        }
        if exists.has_data_index_plan {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdMAR, report_coercions), an.to_owned()));
        }
        if exists.has_scan_blood {
            reports.push((scan_template.clone(), [an, "|6|1"].concat()));
        }
        if exists.has_scan_opd_card {
            reports.push((scan_template.clone(), [an, "|20|1"].concat()));
        }
        if exists.has_scan_insure {
            reports.push((scan_template.clone(), [an, "|2|1"].concat()));
        }
        if exists.has_scan_alt_med {
            reports.push((scan_template.clone(), [an, "|16|1"].concat()));
        }
        if exists.has_scan_nutrition {
            reports.push((scan_template.clone(), [an, "|17|1"].concat()));
        }
        if exists.has_scan_other_sp_clinic {
            reports.push((scan_template.clone(), [an, "|19|1"].concat()));
        }
        if exists.has_scan_others {
            reports.push((scan_template.clone(), [an, "|18|1"].concat()));
        }
        if exists.has_scan_finance {
            reports.push((scan_template.clone(), [an, "|21|1"].concat()));
        }
        // Financial summary
        reports
    }

    // without IpdEventLog, IpdMRA, IpdPartographWho, IpdPartograph, IpdVitalSignLabour
    /// return (template, id)
    fn full_report_psychia(exists: IpdDocumentExists, an: &str, report_coercions: &Option<Arc<HashMap<String, String>>>) -> Vec<(Self, String)> {
        let scan_template = TypstReport::from_system_with_coercion(SystemReport::DocumentImages, report_coercions);
        let mut reports = Vec::with_capacity(37);
        if exists.has_data_summary2 {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdSummary, report_coercions), an.to_owned()));
        }
        if exists.has_scan_consent {
            reports.push((scan_template.clone(), [an, "|1|1"].concat()));
        }
        if exists.has_scan_refer_in {
            reports.push((scan_template.clone(), [an, "|3|1"].concat()));
        }
        if exists.has_data_refer_out {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::ReferOut, report_coercions), an.to_owned()));
        }
        if exists.has_scan_refer_out {
            reports.push((scan_template.clone(), [an, "|4|1"].concat()));
        }
        if exists.has_data_dr_admission_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteDr, report_coercions), an.to_owned()));
        }
        if exists.has_data_nurse_admission_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteNurse, report_coercions), an.to_owned()));
        }
        if exists.has_data_med_reconciliation {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliation, report_coercions), an.to_owned()));
        }
        if exists.has_data_order || exists.has_data_progress_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdOrder, report_coercions), an.to_owned()));
        }
        if exists.has_data_dr_consult {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdConsult, report_coercions), an.to_owned()));
        }
        if exists.has_scan_oper {
            reports.push((scan_template.clone(), [an, "|12|1"].concat()));
        }
        if exists.has_scan_anes {
            reports.push((scan_template.clone(), [an, "|13|1"].concat()));
        }
        if exists.has_scan_labour {
            reports.push((scan_template.clone(), [an, "|14|1"].concat()));
        }
        if exists.has_scan_physio {
            reports.push((scan_template.clone(), [an, "|15|1"].concat()));
        }
        if exists.has_scan_culture {
            reports.push((scan_template.clone(), [an, "|5|1"].concat()));
        }
        if exists.has_data_lab {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::LabSummary, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::Lab, report_coercions), an.to_owned()));
        }
        if exists.has_scan_ekg {
            reports.push((scan_template.clone(), [an, "|8|1"].concat()));
        }
        if exists.has_scan_xray {
            reports.push((scan_template.clone(), [an, "|9|1"].concat()));
        }
        if exists.has_scan_ct {
            reports.push((scan_template.clone(), [an, "|10|1"].concat()));
        }
        if exists.has_scan_mri {
            reports.push((scan_template.clone(), [an, "|11|1"].concat()));
        }
        if exists.has_scan_special {
            reports.push((scan_template.clone(), [an, "|7|1"].concat()));
        }
        if exists.has_data_focus_list {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdFocusList, report_coercions), an.to_owned()));
        }
        if exists.has_data_focus_note {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdFocusNote, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdDischargePlan, report_coercions), an.to_owned()));
        }
        if exists.has_data_index_plan {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdIndexPlan, report_coercions), an.to_owned()));
        }
        if exists.has_data_vital_sign {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignGeneral, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignNeuro, report_coercions), an.to_owned()));
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignPsychia, report_coercions), an.to_owned()));
        }
        if exists.has_data_vital_sign {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdTPR, report_coercions), an.to_owned()));
        }
        if exists.has_data_io {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdIo, report_coercions), an.to_owned()));
        }
        if exists.has_data_index_plan {
            reports.push((TypstReport::from_system_with_coercion(SystemReport::IpdMAR, report_coercions), an.to_owned()));
        }
        if exists.has_scan_blood {
            reports.push((scan_template.clone(), [an, "|6|1"].concat()));
        }
        if exists.has_scan_opd_card {
            reports.push((scan_template.clone(), [an, "|20|1"].concat()));
        }
        if exists.has_scan_insure {
            reports.push((scan_template.clone(), [an, "|2|1"].concat()));
        }
        if exists.has_scan_alt_med {
            reports.push((scan_template.clone(), [an, "|16|1"].concat()));
        }
        if exists.has_scan_nutrition {
            reports.push((scan_template.clone(), [an, "|17|1"].concat()));
        }
        if exists.has_scan_other_sp_clinic {
            reports.push((scan_template.clone(), [an, "|19|1"].concat()));
        }
        if exists.has_scan_others {
            reports.push((scan_template.clone(), [an, "|18|1"].concat()));
        }
        if exists.has_scan_finance {
            reports.push((scan_template.clone(), [an, "|21|1"].concat()));
        }
        reports
    }

    pub fn template_name(&self) -> &str {
        match self {
            Self::System(r) => r.template_name(),
            Self::Coercion((_, s, _)) => s.template_name(),
            Self::Custom(r) => r.template_name.as_str(),
        }
    }

    pub fn file_name(&self) -> &str {
        match self {
            Self::System(r) => r.template_name(),
            Self::Coercion((_, r, _)) => r.template_name(),
            Self::Custom(r) => r.template_name.as_str(),
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Self::System(r) => r.title(),
            Self::Coercion((_, r, _)) => r.title(),
            Self::Custom(r) => r.title.as_str(),
        }
    }

    pub fn key_names(&self) -> String {
        match self {
            Self::System(r) => r.key_names().to_owned(),
            Self::Coercion((_, r, _)) => r.key_names().to_owned(),
            Self::Custom(r) => r
                .statement_params
                .as_ref()
                .map(|s| explode_cap_pipe_iter(s, 2).map(|v| v[0]).collect::<Vec<&str>>().join("|"))
                .unwrap_or_default(),
        }
    }

    /// for get Typst file from client side
    pub fn typ_path_client(&self) -> String {
        match self {
            TypstReport::System(r) => ["/templates/", r.template_name(), ".typ"].concat(),
            TypstReport::Coercion((template_name, _, _)) => ["/customs/", template_name, ".typ"].concat(),
            TypstReport::Custom(r) => ["/customs/", &r.template_name, ".typ"].concat(),
        }
    }

    pub fn report_type(&self) -> &'static str {
        match self {
            TypstReport::System(_) => "system",
            TypstReport::Coercion((_, _, _)) => "coercion",
            TypstReport::Custom(_) => "custom",
        }
    }

    pub fn title_with_ids(&self, ids: &str) -> String {
        let title = self.title();
        let key_names = self.key_names();
        if ids.is_empty() || key_names.is_empty() {
            title.to_owned()
        } else {
            let kvs = key_names.split("|").zip(ids.split("|")).map(|(k, v)| [k, ": ", v].concat()).collect::<Vec<String>>().join(", ");
            [title, " [", &kvs, "]"].concat()
        }
    }

    pub fn download_file_name(&self, ids: &str) -> String {
        [&ids.replace('|', "-"), "-", &self.template_name().to_uppercase()].concat()
    }
}

#[derive(Clone, EnumIter, PartialEq, Eq)]
pub enum SystemReport {
    DocumentImages,
    IpdAdmissionNoteDr,
    IpdAdmissionNoteNurse,
    IpdConsult,
    IpdDischargePlan,
    IpdDocument,
    IpdEventLog,
    IpdFocusList,
    IpdFocusNote,
    IpdIndexPlan,
    IpdIo,
    IpdMAR,
    IpdMRA,
    IpdMedReconciliation,
    IpdMedReconciliationHosXp,
    IpdOrder,
    IpdPartograph,
    IpdPartographWho,
    IpdSummary,
    IpdSummaryAudit,
    IpdTPR,
    IpdVitalSignGeneral,
    IpdVitalSignNeuro,
    IpdVitalSignLabour,
    IpdVitalSignPsychia,
    Lab,
    LabSummary,
    OpdErDischargePlan,
    OpdErDocument,
    OpdErEventLog,
    OpdErFocusList,
    OpdErFocusNote,
    OpdErIndexPlan,
    OpdErIo,
    OpdErMedicalHistory,
    OpdErMedReconciliation,
    OpdErOrder,
    OpdErVitalSignGeneral,
    OpdErVitalSignNeuro,
    OpdErVitalSignLabour,
    OpdErVitalSignPsychia,
    ReferNote,
    ReferOut,
    ScanImages,
}

impl SystemReport {
    pub fn ipd_set() -> Vec<Self> {
        vec![
            SystemReport::IpdAdmissionNoteDr,
            SystemReport::IpdAdmissionNoteNurse,
            SystemReport::IpdMedReconciliation,
            SystemReport::Lab,
            SystemReport::LabSummary,
            SystemReport::IpdOrder,
            SystemReport::IpdConsult,
            SystemReport::IpdTPR,
            SystemReport::IpdPartograph,
            SystemReport::IpdPartographWho,
            SystemReport::IpdVitalSignGeneral,
            SystemReport::IpdVitalSignNeuro,
            SystemReport::IpdVitalSignLabour,
            SystemReport::IpdVitalSignPsychia,
            SystemReport::IpdIo,
            SystemReport::IpdFocusList,
            SystemReport::IpdFocusNote,
            SystemReport::IpdIndexPlan,
            SystemReport::IpdMAR,
            SystemReport::IpdDischargePlan,
            SystemReport::IpdSummary,
            SystemReport::IpdMRA,
            SystemReport::ReferNote,
            SystemReport::ReferOut,
            // SystemReport::IpdEventLog,
            // SystemReport::IpdDocument,
            // SystemReport::IpdMedReconciliationHosXp,
        ]
    }

    pub fn opd_er_set() -> Vec<Self> {
        vec![
            SystemReport::OpdErMedicalHistory,
            SystemReport::OpdErMedReconciliation,
            SystemReport::OpdErOrder,
            SystemReport::OpdErVitalSignGeneral,
            SystemReport::OpdErVitalSignNeuro,
            SystemReport::OpdErVitalSignLabour,
            SystemReport::OpdErVitalSignPsychia,
            SystemReport::OpdErIo,
            SystemReport::OpdErFocusList,
            SystemReport::OpdErFocusNote,
            SystemReport::OpdErIndexPlan,
            SystemReport::Lab,
            SystemReport::LabSummary,
            SystemReport::ReferNote,
            SystemReport::ReferOut,
            // SystemReport::OpdErDocument,
            // SystemReport::OpdErEventLog,
        ]
    }

    pub fn new(text: &str) -> Option<Self> {
        match text {
            "document-images" => Some(Self::DocumentImages),
            "ipd-admission-note-dr" => Some(Self::IpdAdmissionNoteDr),
            "ipd-admission-note-nurse" => Some(Self::IpdAdmissionNoteNurse),
            "ipd-consult" => Some(Self::IpdConsult),
            "ipd-discharge-plan" => Some(Self::IpdDischargePlan),
            "ipd-document" => Some(Self::IpdDocument),
            "ipd-event-log" => Some(Self::IpdEventLog),
            "ipd-focus-list" => Some(Self::IpdFocusList),
            "ipd-focus-note" => Some(Self::IpdFocusNote),
            "ipd-index-plan" => Some(Self::IpdIndexPlan),
            "ipd-io" => Some(Self::IpdIo),
            "ipd-mar" => Some(Self::IpdMAR),
            "ipd-mra" => Some(Self::IpdMRA),
            "ipd-med-reconciliation" => Some(Self::IpdMedReconciliation),
            "ipd-med-reconciliation-hosxp" => Some(Self::IpdMedReconciliationHosXp),
            "ipd-order" => Some(Self::IpdOrder),
            "ipd-partograph" => Some(Self::IpdPartograph),
            "ipd-partograph-who" => Some(Self::IpdPartographWho),
            "ipd-summary" => Some(Self::IpdSummary),
            "ipd-summary-audit" => Some(Self::IpdSummaryAudit),
            "ipd-tpr" => Some(Self::IpdTPR),
            "ipd-vital-sign-general" => Some(Self::IpdVitalSignGeneral),
            "ipd-vital-sign-neuro" => Some(Self::IpdVitalSignNeuro),
            "ipd-vital-sign-labour" => Some(Self::IpdVitalSignLabour),
            "ipd-vital-sign-psychia" => Some(Self::IpdVitalSignPsychia),
            "lab" => Some(Self::Lab),
            "lab-summary" => Some(Self::LabSummary),
            "opd-er-discharge-plan" => Some(Self::OpdErDischargePlan),
            "opd-er-document" => Some(Self::OpdErDocument),
            "opd-er-event-log" => Some(Self::OpdErEventLog),
            "opd-er-focus-list" => Some(Self::OpdErFocusList),
            "opd-er-focus-note" => Some(Self::OpdErFocusNote),
            "opd-er-index-plan" => Some(Self::OpdErIndexPlan),
            "opd-er-io" => Some(Self::OpdErIo),
            "opd-er-medical-history" => Some(Self::OpdErMedicalHistory),
            "opd-er-med-reconciliation" => Some(Self::OpdErMedReconciliation),
            "opd-er-order" => Some(Self::OpdErOrder),
            "opd-er-vital-sign-general" => Some(Self::OpdErVitalSignGeneral),
            "opd-er-vital-sign-neuro" => Some(Self::OpdErVitalSignNeuro),
            "opd-er-vital-sign-labour" => Some(Self::OpdErVitalSignLabour),
            "opd-er-vital-sign-psychia" => Some(Self::OpdErVitalSignPsychia),
            "refer-note" => Some(Self::ReferNote),
            "refer-out" => Some(Self::ReferOut),
            "scan-images" => Some(Self::ScanImages),
            _ => None,
        }
    }

    pub fn template_name(&self) -> &'static str {
        match self {
            Self::DocumentImages => "document-images",
            Self::IpdAdmissionNoteDr => "ipd-admission-note-dr",
            Self::IpdAdmissionNoteNurse => "ipd-admission-note-nurse",
            Self::IpdConsult => "ipd-consult",
            Self::IpdDischargePlan => "ipd-discharge-plan",
            Self::IpdDocument => "ipd-document",
            Self::IpdEventLog => "ipd-event-log",
            Self::IpdFocusList => "ipd-focus-list",
            Self::IpdFocusNote => "ipd-focus-note",
            Self::IpdIndexPlan => "ipd-index-plan",
            Self::IpdIo => "ipd-io",
            Self::IpdMAR => "ipd-mar",
            Self::IpdMRA => "ipd-mra",
            Self::IpdMedReconciliation => "ipd-med-reconciliation",
            Self::IpdMedReconciliationHosXp => "ipd-med-reconciliation-hosxp",
            Self::IpdOrder => "ipd-order",
            Self::IpdPartograph => "ipd-partograph",
            Self::IpdPartographWho => "ipd-partograph-who",
            Self::IpdSummary => "ipd-summary",
            Self::IpdSummaryAudit => "ipd-summary-audit",
            Self::IpdTPR => "ipd-tpr",
            Self::IpdVitalSignGeneral => "ipd-vital-sign-general",
            Self::IpdVitalSignNeuro => "ipd-vital-sign-neuro",
            Self::IpdVitalSignLabour => "ipd-vital-sign-labour",
            Self::IpdVitalSignPsychia => "ipd-vital-sign-psychia",
            Self::Lab => "lab",
            Self::LabSummary => "lab-summary",
            Self::OpdErDischargePlan => "opd-er-discharge-plan",
            Self::OpdErDocument => "opd-er-document",
            Self::OpdErEventLog => "opd-er-event-log",
            Self::OpdErFocusList => "opd-er-focus-list",
            Self::OpdErFocusNote => "opd-er-focus-note",
            Self::OpdErIndexPlan => "opd-er-index-plan",
            Self::OpdErIo => "opd-er-io",
            Self::OpdErMedicalHistory => "opd-er-medical-history",
            Self::OpdErMedReconciliation => "opd-er-med-reconciliation",
            Self::OpdErOrder => "opd-er-order",
            Self::OpdErVitalSignGeneral => "opd-er-vital-sign-general",
            Self::OpdErVitalSignNeuro => "opd-er-vital-sign-neuro",
            Self::OpdErVitalSignLabour => "opd-er-vital-sign-labour",
            Self::OpdErVitalSignPsychia => "opd-er-vital-sign-psychia",
            Self::ReferNote => "refer-note",
            Self::ReferOut => "refer-out",
            Self::ScanImages => "scan-images",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::DocumentImages => "Document Images",
            Self::IpdAdmissionNoteDr => "IPD Admission Note (Doctor)",
            Self::IpdAdmissionNoteNurse => "IPD Admission Note (Nurse)",
            Self::IpdConsult => "IPD Consult",
            Self::IpdDischargePlan => "IPD Discharge Plan",
            Self::IpdDocument => "IPD Document",
            Self::IpdEventLog => "IPD Event Logs",
            Self::IpdFocusList => "IPD Focus List",
            Self::IpdFocusNote => "IPD Focus Note",
            Self::IpdIndexPlan => "IPD Index Plan",
            Self::IpdIo => "IPD IO",
            Self::IpdMAR => "IPD Medication Administration Record (eMAR)",
            Self::IpdMRA => "IPD Medical Record Audit (MRA)",
            Self::IpdMedReconciliation => "IPD Medical Reconciliation",
            Self::IpdMedReconciliationHosXp => "IPD Medical Reconciliation (HIS)",
            Self::IpdOrder => "IPD Order",
            Self::IpdPartograph => "IPD Partograph",
            Self::IpdPartographWho => "IPD Partograph (WHO)",
            Self::IpdSummary => "IPD Summary",
            Self::IpdSummaryAudit => "IPD Summary/Coding Audit",
            Self::IpdTPR => "IPD T.P.R. Chart",
            Self::IpdVitalSignGeneral => "IPD Vital Sign",
            Self::IpdVitalSignNeuro => "IPD Neurologic Vital Sign",
            Self::IpdVitalSignLabour => "IPD Labour Vital Sign",
            Self::IpdVitalSignPsychia => "IPD Psychiatric Vital Sign",
            Self::Lab => "Lab",
            Self::LabSummary => "Lab Summary",
            Self::OpdErDischargePlan => "OPD/ER Discharge Plan",
            Self::OpdErDocument => "OPD/ER Document",
            Self::OpdErEventLog => "OPD/ER Event Logs",
            Self::OpdErFocusList => "OPD/ER Focus List",
            Self::OpdErFocusNote => "OPD/ER Focus Note",
            Self::OpdErIndexPlan => "OPD/ER Index Plan",
            Self::OpdErIo => "OPD/ER IO",
            Self::OpdErMedicalHistory => "OPD/ER Medical History",
            Self::OpdErMedReconciliation => "OPD/ER Medical Reconciliation",
            Self::OpdErOrder => "OPD/ER Order",
            Self::OpdErVitalSignGeneral => "OPD/ER Vital Sign",
            Self::OpdErVitalSignNeuro => "OPD/ER Neurologic Vital Sign",
            Self::OpdErVitalSignLabour => "OPD/ER Labour Vital Sign",
            Self::OpdErVitalSignPsychia => "OPD/ER Psychiatric Vital Sign",
            Self::ReferNote => "Refer Note",
            Self::ReferOut => "Refer Out",
            Self::ScanImages => "Scanned Images",
        }
    }

    pub fn key_names(&self) -> &'static str {
        match self {
            Self::IpdAdmissionNoteDr
            | Self::IpdAdmissionNoteNurse
            | Self::IpdConsult
            | Self::IpdDischargePlan
            | Self::IpdDocument
            | Self::IpdEventLog
            | Self::IpdFocusList
            | Self::IpdFocusNote
            | Self::IpdIndexPlan
            | Self::IpdIo
            | Self::IpdMAR
            | Self::IpdMRA
            | Self::IpdMedReconciliation
            | Self::IpdMedReconciliationHosXp
            | Self::IpdOrder
            | Self::IpdPartograph
            | Self::IpdPartographWho
            | Self::IpdSummary
            | Self::IpdSummaryAudit
            | Self::IpdTPR
            | Self::IpdVitalSignGeneral
            | Self::IpdVitalSignNeuro
            | Self::IpdVitalSignLabour
            | Self::IpdVitalSignPsychia => "AN",
            Self::Lab | Self::LabSummary | Self::ReferNote | Self::ReferOut => "VN/AN",
            Self::OpdErDischargePlan
            | Self::OpdErDocument
            | Self::OpdErEventLog
            | Self::OpdErFocusList
            | Self::OpdErFocusNote
            | Self::OpdErIndexPlan
            | Self::OpdErIo
            | Self::OpdErMedicalHistory
            | Self::OpdErMedReconciliation
            | Self::OpdErOrder
            | Self::OpdErVitalSignGeneral
            | Self::OpdErVitalSignNeuro
            | Self::OpdErVitalSignLabour
            | Self::OpdErVitalSignPsychia => "VN",
            Self::DocumentImages => "VN/AN|DOC-TYPE-ID|PER-PAGE",
            Self::ScanImages => "VN/AN|KEY|PER-PAGE",
        }
    }

    pub fn key_param(&self) -> &'static str {
        match self {
            Self::IpdAdmissionNoteDr
            | Self::IpdAdmissionNoteNurse
            | Self::IpdConsult
            | Self::IpdDischargePlan
            | Self::IpdDocument
            | Self::IpdEventLog
            | Self::IpdFocusList
            | Self::IpdFocusNote
            | Self::IpdIndexPlan
            | Self::IpdIo
            | Self::IpdMAR
            | Self::IpdMRA
            | Self::IpdMedReconciliation
            | Self::IpdMedReconciliationHosXp
            | Self::IpdOrder
            | Self::IpdPartograph
            | Self::IpdPartographWho
            | Self::IpdSummary
            | Self::IpdSummaryAudit
            | Self::IpdTPR
            | Self::IpdVitalSignGeneral
            | Self::IpdVitalSignNeuro
            | Self::IpdVitalSignLabour
            | Self::IpdVitalSignPsychia => "id^AN^str",
            Self::Lab | Self::LabSummary | Self::ReferNote | Self::ReferOut => "id^VN/AN^str",
            Self::OpdErDischargePlan
            | Self::OpdErDocument
            | Self::OpdErEventLog
            | Self::OpdErFocusList
            | Self::OpdErFocusNote
            | Self::OpdErIndexPlan
            | Self::OpdErIo
            | Self::OpdErMedicalHistory
            | Self::OpdErMedReconciliation
            | Self::OpdErOrder
            | Self::OpdErVitalSignGeneral
            | Self::OpdErVitalSignNeuro
            | Self::OpdErVitalSignLabour
            | Self::OpdErVitalSignPsychia => "id^VN^str",
            Self::DocumentImages => "id^VN/AN^str|doc_type_id^Document^(doc-type)|per_page^Images/Page^(u8,1,1,2,2,4,4)",
            Self::ScanImages => "id^VN/AN^str|key^KEY^(str,opd,OPD,er,ER,pe,PE,lab,Lab)|per_page^Images/Page^(u8,1,1,2,2,4,4)",
        }
    }

    /// for get Typst file from server side
    pub fn typ_path_server(&self) -> String {
        ["volume/pwa/templates/", self.template_name(), ".typ"].concat()
    }

    pub fn title_with_ids(&self, ids: &str) -> String {
        let title = self.title();
        let key_names = self.key_names();
        if ids.is_empty() || key_names.is_empty() {
            title.to_owned()
        } else {
            let kvs = key_names.split("|").zip(ids.split("|")).map(|(k, v)| [k, ": ", v].concat()).collect::<Vec<String>>().join(", ");
            [title, " [", &kvs, "]"].concat()
        }
    }

    pub fn download_file_name(&self, ids: &str) -> String {
        [&ids.replace('|', "-"), "-", &self.template_name().to_uppercase()].concat()
    }
}

/// Custom report from database
#[derive(Clone, Debug, Default, Demo, Deserialize, Eq, Serialize, FromRow, ToSchema)]
#[schema(example = json!(CustomReport::demo()))]
pub struct CustomReport {
    #[Demo(value = "1")]
    pub template_id: u32,
    #[Demo(value = r#"String::from("ipd-order-v2")"#)]
    pub template_name: String,
    #[Demo(value = r#"String::from("IPD Order")"#)]
    pub title: String,
    #[Demo(value = r##"String::from("= Test report#linebreak()#h(10pt)Hello!!")"##)]
    pub content: String,
    /// System will replace `__HOSXP__`, `__KPHIS__`, `__KPHIS_EXTRA__` and `__KPHIS_LOG__` with database name in config<br>
    /// and replace statement_params(label^placeholder) (seperated by `|`) with user's ids (seperated by `|`)
    #[Demo(value = r#"Some(String::from("SELECT * FROM __HOSXP__.ipt WHERE hn='__HN__' AND dchtype='__DCHTYPE__';"))"#)]
    pub statement: Option<String>,
    /// System will replace statement with statement-params(label^placeholder) (seperated by `|`) with user's ids (seperated by `|`)
    #[Demo(value = r#"Some(String::from("hn^__HN__|dchtype^__DCHTYPE__"))"#)]
    pub statement_params: Option<String>,
    #[Demo(value = r#"Some(String::from("This is test report"))"#)]
    pub info: Option<String>,
    #[Demo(value = "Some(false)")]
    pub disabled: Option<bool>,
    #[Demo(value = r#"Some(String::from("Mr.User"))"#)]
    pub update_username: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub update_datetime: Option<PrimitiveDateTime>,
}

impl PartialEq for CustomReport {
    fn eq(&self, other: &Self) -> bool {
        self.template_id == other.template_id
    }
}

impl CustomReport {
    pub fn title_with_ids(&self, ids: &str) -> String {
        let kvs_opt = self.statement_params.as_ref().map(|s| {
            explode_cap_pipe_iter(s, 2)
                .map(|v| v[0])
                .zip(ids.split("|"))
                .map(|(k, v)| [k, ": ", v].concat())
                .collect::<Vec<String>>()
                .join(", ")
        });
        if let Some(kvs) = kvs_opt {
            [&self.title, " [", &kvs, "]"].concat()
        } else {
            self.title.to_owned()
        }
    }

    pub fn download_file_name(&self, ids: &str) -> String {
        [&ids.replace('|', "-"), "-", &self.template_name.to_uppercase()].concat()
    }

    /// GET `EndPoint::ReportCustom`
    pub async fn call_api_get(params: &ReportTemplateParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::ReportCustom.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch CustomReport"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch CustomReport"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch CustomReport")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::ReportCustom`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send CustomReport"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send CustomReport"))?;

        execute_fetch(&EndPoint::ReportCustom.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::ReportCustom`
    pub async fn call_api_delete(params: &ReportTemplateParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::ReportCustom.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct ReportTemplateParams {
    pub template_id: Option<u32>,
    pub template_name: Option<String>,
    pub disabled: Option<bool>,
    pub compact: Option<bool>,
}

impl QueryString for ReportTemplateParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            template_id: find_qs(params, "template_id").and_then(|s| s.parse::<u32>().ok()),
            template_name: find_qs(params, "template_name"),
            disabled: find_qs(params, "disabled").and_then(|s| s.parse::<bool>().ok()),
            compact: find_qs(params, "compact").and_then(|s| s.parse::<bool>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(4);
        if let Some(template_id) = &self.template_id {
            queries.push(["template_id=", &template_id.to_string()].concat());
        }
        if let Some(template_name) = &self.template_name {
            queries.push(["template_name=", template_name].concat());
        }
        if let Some(disabled) = &self.disabled {
            queries.push(["disabled=", &disabled.to_string()].concat());
        }
        if let Some(compact) = &self.compact {
            queries.push(["compact=", &compact.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Report query sending to server
#[derive(Clone, Debug, Demo, Default, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ReportQuery::demo()))]
pub struct ReportQuery {
    /// System will replace `__HOSXP__`, `__KPHIS__`, `__KPHIS_EXTRA__` and `__KPHIS_LOG__` with database name in config
    #[Demo(value = r#"String::from("SELECT * FROM __HOSXP__.ipt WHERE hn=? AND dchtype=?;")"#)]
    pub statement: String,
    /// params syntax separated by `|`<br>
    /// we parameterized `?` in SQL with `ids` separated by `|` from left to right (skip `value` type)<br>
    /// - Simple: `id^title^type`
    /// - List: `id^title^(type,k1,v1,k2,v2,..)`
    /// - System list: `id^title^(system-type)`
    /// - Array: `id^title^[type]`
    /// - Array of List: `id^title^[(type,k1,v1,k2,v2,..)]`
    /// - Array of System list: `id^title^[(system-type)]`
    #[Demo(value = r#"String::from("hn^HN^str|dchtype^Discharge Type^(str,02,Recover,04,Refer)")"#)]
    pub statement_params: String,
    /// Values for `?` in SQL, separated by `|`<br>
    /// Array parameter ex. `0001234|02,04,06` will multiply 2nd `?` from `..IN (?)..` to `..IN (?,?,?)..`
    #[Demo(value = r#"String::from("0001234|04")"#)]
    pub ids: String,
}

impl ReportQuery {
    /// POST `EndPoint::ReportRawQuery`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<String, AppError> {
        let path = EndPoint::ReportRawQuery.base();

        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send ReportQuery"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send ReportQuery"))?;

        execute_fetch_text(&path, "POST", Some(&body), app).await
    }
}

pub fn params_and_ids_to_json(custom_params: &str, ids: &str) -> String {
    let mid = explode_cap_pipe_iter(custom_params, 2)
        .map(|v| v[0])
        .zip(ids.split('|'))
        .map(|(label, id)| ["\"", label, "\": \"", id, "\""].concat())
        .collect::<Vec<String>>()
        .join(",");
    ["{", &mid, ",\"data\":[]}"].concat()
}

#[derive(Clone)]
pub struct ReportParam {
    pub id: String,
    pub title: String,
    pub ty: ParamType,
}

impl ReportParam {
    pub fn from_cap_pipe(cap_pipe: &str) -> Vec<Self> {
        explode_cap_pipe_iter(cap_pipe, 3)
            .map(|v| {
                let id = v[0].to_owned();
                let title = v[1].to_owned();
                // (system-list) or (ty,k1,v1,k2,v2,..)
                let ty = if v[2].starts_with('(') {
                    let mut v_iter = v[2].trim_start_matches('(').trim_end_matches(')').split(',').map(str::trim);
                    if let Some(first) = v_iter.next() {
                        let items = v_iter
                            .collect::<Vec<&str>>()
                            .chunks_exact(2)
                            .map(|two| KeyLabel {
                                key: two[0].to_owned(),
                                label: two[1].to_owned(),
                            })
                            .collect::<Vec<KeyLabel>>();
                        if items.is_empty() {
                            ParamType::ListSystem(SystemListType::from(first))
                        } else {
                            ParamType::List(BasicType::from(first), items)
                        }
                    } else {
                        ParamType::Basic(BasicType::Str)
                    }
                // [(system-list)] or [(ty,k1,v1,k2,v2,..)]
                } else if v[2].starts_with("[(") {
                    let mut v_iter = v[2].trim_start_matches("[(").trim_end_matches(")]").split(',').map(str::trim);
                    if let Some(first) = v_iter.next() {
                        let items = v_iter
                            .collect::<Vec<&str>>()
                            .chunks_exact(2)
                            .map(|two| KeyLabel {
                                key: two[0].to_owned(),
                                label: two[1].to_owned(),
                            })
                            .collect::<Vec<KeyLabel>>();
                        if items.is_empty() {
                            ParamType::ArrayListSystem(SystemListType::from(first))
                        } else {
                            ParamType::ArrayList(BasicType::from(first), items)
                        }
                    } else {
                        ParamType::Basic(BasicType::Str)
                    }
                // [ty]
                } else if v[2].starts_with('[') {
                    ParamType::Array(BasicType::from(v[2].trim_start_matches("[").trim_end_matches("]")))
                // ty
                } else {
                    ParamType::Basic(BasicType::from(v[2]))
                };
                Self { id, title, ty }
            })
            .collect()
    }

    pub fn to_cap_pipe(items: &[Self]) -> String {
        items
            .iter()
            .map(|item| [&item.id, "^", &item.title, "^", &item.ty.to_str()].concat())
            .collect::<Vec<String>>()
            .join("|")
    }

    pub fn is_array(&self) -> bool {
        matches!(self.ty, ParamType::Array(_) | ParamType::ArrayList(_, _) | ParamType::ArrayListSystem(_))
    }
}

#[derive(Clone)]
pub enum VarType {
    Basic,
    List,
    System,
}

#[derive(Clone)]
pub enum ParamType {
    Basic(BasicType),
    List(BasicType, Vec<KeyLabel>),
    ListSystem(SystemListType),
    Array(BasicType),
    ArrayList(BasicType, Vec<KeyLabel>),
    ArrayListSystem(SystemListType),
}

impl ParamType {
    pub fn to_str(&self) -> String {
        match self {
            Self::Basic(inner) => inner.to_str().to_owned(),
            Self::List(ty, items) => {
                let values = items.iter().flat_map(|item| [item.key.as_str(), item.label.as_str()]).collect::<Vec<&str>>().join(",");
                ["(", ty.to_str(), ",", &values, ")"].concat()
            }
            Self::ListSystem(sty) => ["(", sty.to_str(), ")"].concat(),
            Self::Array(ty) => ["[", ty.to_str(), "]"].concat(),
            Self::ArrayList(ty, items) => {
                let values = items.iter().flat_map(|item| [item.key.as_str(), item.label.as_str()]).collect::<Vec<&str>>().join(",");
                ["[(", ty.to_str(), ",", &values, ")]"].concat()
            }
            Self::ArrayListSystem(sty) => ["[(", sty.to_str(), ")]"].concat(),
        }
    }

    pub fn is_list(&self) -> bool {
        matches!(&self, Self::List(_, _) | Self::ListSystem(_) | Self::ArrayList(_, _) | Self::ArrayListSystem(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(&self, Self::Array(_) | Self::ArrayList(_, _) | Self::ArrayListSystem(_))
    }

    pub fn get_var_type(&self) -> VarType {
        match self {
            Self::Basic(_) | Self::Array(_) => VarType::Basic,
            Self::List(_, _) | Self::ArrayList(_, _) => VarType::List,
            Self::ListSystem(_) | Self::ArrayListSystem(_) => VarType::System,
        }
    }

    pub fn get_basic_type(&self) -> BasicType {
        match self {
            Self::Basic(ty) | Self::List(ty, _) | Self::Array(ty) | Self::ArrayList(ty, _) => ty.clone(),
            Self::ListSystem(sty) | Self::ArrayListSystem(sty) => sty.key_type(),
        }
    }

    pub fn get_system_list_type(&self) -> Option<SystemListType> {
        match self {
            Self::Basic(_) | Self::List(_, _) | Self::Array(_) | Self::ArrayList(_, _) => None,
            Self::ListSystem(sty) | Self::ArrayListSystem(sty) => Some(sty.to_owned()),
        }
    }

    pub fn get_items(&self, assets: &AppAsset) -> Vec<KeyLabel> {
        match self {
            Self::Basic(_) | Self::Array(_) => Vec::new(),
            Self::List(_, v) | Self::ArrayList(_, v) => v.to_owned(),
            Self::ListSystem(list) | Self::ArrayListSystem(list) => list.get_items(assets),
        }
    }
}

#[derive(Clone, EnumIter)]
pub enum BasicType {
    Str,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Decimal,
    Date,
    Time,
    DateTime,
    Value,
}

impl BasicType {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Str => "str",
            Self::Bool => "bool",
            Self::Int8 => "i8",
            Self::Int16 => "i16",
            Self::Int32 => "i32",
            Self::Int64 => "i64",
            Self::Uint8 => "u8",
            Self::Uint16 => "u16",
            Self::Uint32 => "u32",
            Self::Uint64 => "u64",
            Self::Float32 => "f32",
            Self::Float64 => "f64",
            Self::Decimal => "decimal",
            Self::Date => "date",
            Self::Time => "time",
            Self::DateTime => "datetime",
            Self::Value => "value",
        }
    }

    pub fn to_label(&self) -> &'static str {
        match self {
            Self::Str => "VARCHAR/CHAR/TEXT",
            Self::Bool => "BOOL",
            Self::Int8 => "TINYINT",
            Self::Int16 => "SMALLINT",
            Self::Int32 => "INT",
            Self::Int64 => "BIGINT",
            Self::Uint8 => "TINYINT UNSIGNED",
            Self::Uint16 => "SMALLINT UNSIGNED",
            Self::Uint32 => "INT UNSIGNED",
            Self::Uint64 => "BIGINT UNSIGNED",
            Self::Float32 => "FLOAT",
            Self::Float64 => "DOUBLE",
            Self::Decimal => "DECIMAL",
            Self::Date => "DATE",
            Self::Time => "TIME",
            Self::DateTime => "DATETIME",
            Self::Value => "NOT QUERY",
        }
    }

    pub fn is_value_parsable(&self, value: &str) -> bool {
        match self {
            Self::Str => true,
            Self::Bool => value.parse::<bool>().is_ok(),
            Self::Int8 => value.parse::<i8>().is_ok(),
            Self::Int16 => value.parse::<i16>().is_ok(),
            Self::Int32 => value.parse::<i32>().is_ok(),
            Self::Int64 => value.parse::<i64>().is_ok(),
            Self::Uint8 => value.parse::<u8>().is_ok(),
            Self::Uint16 => value.parse::<u16>().is_ok(),
            Self::Uint32 => value.parse::<u32>().is_ok(),
            Self::Uint64 => value.parse::<u64>().is_ok(),
            Self::Float32 => value.parse::<f32>().is_ok(),
            Self::Float64 => value.parse::<f64>().is_ok(),
            Self::Decimal => value.parse::<Decimal>().is_ok(),
            Self::Date => date_8601(value).is_some(),
            Self::Time => time_8601(value).is_some(),
            Self::DateTime => datetime_8601(value).is_some(),
            Self::Value => true,
        }
    }
}

impl std::convert::From<&str> for BasicType {
    fn from(item: &str) -> Self {
        match item {
            "bool" => Self::Bool,
            "i8" => Self::Int8,
            "i16" => Self::Int16,
            "i32" => Self::Int32,
            "i64" => Self::Int64,
            "u8" => Self::Uint8,
            "u16" => Self::Uint16,
            "u32" => Self::Uint32,
            "u64" => Self::Uint64,
            "f32" => Self::Float32,
            "f64" => Self::Float64,
            "decimal" => Self::Decimal,
            "date" => Self::Date,
            "time" => Self::Time,
            "datetime" => Self::DateTime,
            "value" => Self::Value,
            _ => Self::Str,
        }
    }
}

#[derive(Clone, EnumIter)]
pub enum SystemListType {
    // fcnote_patient_type_select_options, ประเภทผู้ป่วย, str, color, fcnote_patient_type
    PatientType,
    // er_bed_select_options, เลขเตียง ER, str, color, kphis.opd_er_bed.opd_er_bed_id
    ErBed,
    // er_patient_status_select_options, สถานะผู้ป่วย ER , u32, kphis.opd_er_patient_status.er_patient_status_id
    ErPatientStatus,
    // er_dch_type_select_options, ประเภทการ Discharge ER, u32, kphis.opd_er_dch_type.er_dch_type_id
    ErDchType,
    // ward_select_option, Ward, str, hos.ward.ward
    Ward,
    // doctor_select_option, Doctor, str, hos.doctor.code
    Doctor,
    // all_doctor_select_option, Doctor (All), str, hos.doctor.code
    DoctorAll,
    // spclty_select_option, แผนก (HOSxP), str, hos.spclty.spclty
    Spclty,
    // spclty_kphis_select_option, แผนก (KPHIS), u32, kphis.kphis_spclty.spclty_id
    SpcltyKphis,
    // inscl_select_option, สิทธิ์, str, hos.nhso_inscl_code.inscl_code
    Inscl,
    // emergency_select_option, เร่งด่วน ER, u32, kphis.ipd_emergency.emergency_id
    Emergency,
    // emergency_level_select_option, ระดับความฉุกเฉิน, i32, hos.er_emergency_level.er_emergency_level_id
    EmergencyLevel,
    // consult_type_select_option, ชนิดใบ Consult, u32, kphis.ipd_dr_consult_type.consult_type_id
    ConsultType,
    // conscious_select_option, VS-Conscious, u32, kphis.ipd_vs_conscious.conscious_id
    VsConscious,
    // urine_amount_select_option, VS-Urine Amount, u32, kphis.ipd_vs_urine_amount.urine_amount_id
    VsUrineAmount,
    // urine_duration_select_option, VS-Urine Duration, u32, kphis.ipd_vs_urine_duration.urine_d_id
    VsUrineDuration,
    // line_select_option, VS-Line, u32, kphis.ipd_vs_line.line_id
    VsLine,
    // cha_select_option, VS-Pupil Response, u32, kphis.ipd_vs_cha.cha_id
    VsCha,
    // va_select_option, Vs-VA, u32, kphis.ipd_vs_va.va_id
    VsVA,
    // mass_select_option, VS-MAAS, u32, kphis.ipd_vs_mass.mass_id
    VsMaas,
    // motor_select_option, VS-Motor Power, u32, kphis.ipd_vs_lt_arm.lt_arm
    VsMotor,
    // o2_select_option, VS-O2, u32, kphis.ipd_vs_o2.o2_id
    VsO2,
    // tube_select_option, VS-Tube, u32, kphis.ipd_vs_tube.tube_id
    VsTube,
    // // intake_select_option, Intake, u32, kphis.ipd_vs_intake.intake_id
    // VsIntake,
    // // output_select_option, Output, u32, kphis.ipd_vs_output.output_id
    // VsOutput,
    // lr_sta_select_option, VS-Cx Station, u32, kphis.ipd_vs_lr_sta.lr_sta_id
    VsLrStation,
    // lr_mem_select_option, VS-Membrane, u32, kphis.ipd_vs_lr_mem.lr_mem_id
    VsLrMembrane,
    // lr_moulding_select_option, VS-Moulding, u32, kphis.ipd_vs_lr_moulding.lr_moulding_id
    VsLrMoulding,
    // dipstick_select_option, VS-Urine Dipstick, u32, kphis.ipd_vs_dipstick.dipstick_id
    VsDipstick,
    // breathing_select_option, VS-Breathing, u32, kphis.ipd_vs_breathing.breathing_id
    VsBreathing,
    // avpu_select_option, VS-AVPU, u32. kphis.ipd_vs_avpu.avpu_id
    VsAvpu,
    // gut_feeling_select_option, VS-Gut Feeling, u32, kphis.ipd_vs_gut_feeling.gut_feeling_id
    VsGutFeeling,
    // pops_other_select_option, VS-POPs Other, u32, kphis.ipd_vs_pops_other.pops_other_id
    VsPopsOther,
    // stage_of_change_select_option, VS-Stage-Of-Change, u32, kphis.ipd_vs_stage_of_change.stage_of_change_id
    StageOfChange,
    // refer_type_select_option, Refer type, i32, hos.refer_type.refer_type
    ReferType,
    // refer_cause_select_option, Refer cause, i32, hos.refer_cause.id
    ReferCause,
    // refer_point_select_option, Refer point list, str, hos.refer_point_list.name
    ReferPoint,
    // moph_refer_expire_type_select_option, Moph Refer Expire type, i32, hos.moph_refer_expire_type.moph_refer_expire_type_id
    MophReferExpType,
    // document_type_select_option, Document type, u8, Static code
    DocumentType,
    #[strum(disabled)]
    Unknown,
}

impl std::convert::From<&str> for SystemListType {
    fn from(item: &str) -> Self {
        match item {
            "patient-type" => Self::PatientType,
            "er-bed" => Self::ErBed,
            "er-patient-status" => Self::ErPatientStatus,
            "er-dch-type" => Self::ErDchType,
            "ward" => Self::Ward,
            "doctor" => Self::Doctor,
            "doctor-all" => Self::DoctorAll,
            "spclty" => Self::Spclty,
            "spclty-id" => Self::SpcltyKphis,
            "inscl" => Self::Inscl,
            "emergency" => Self::Emergency,
            "emergency-level" => Self::EmergencyLevel,
            "consult-type" => Self::ConsultType,
            "conscious" => Self::VsConscious,
            "urine-amount" => Self::VsUrineAmount,
            "urine-duration" => Self::VsUrineDuration,
            "line" => Self::VsLine,
            "cha" => Self::VsCha,
            "va" => Self::VsVA,
            "mass" => Self::VsMaas,
            "motor" => Self::VsMotor,
            "o2" => Self::VsO2,
            "tube" => Self::VsTube,
            // "intake" => Self::VsIntake,
            // "output" => Self::VsOutput,
            "lr-sta" => Self::VsLrStation,
            "lr-mem" => Self::VsLrMembrane,
            "lr-moulding" => Self::VsLrMoulding,
            "dipstick" => Self::VsDipstick,
            "breathing" => Self::VsBreathing,
            "avpu" => Self::VsAvpu,
            "gut-feeling" => Self::VsGutFeeling,
            "pops-other" => Self::VsPopsOther,
            "stage-of-change" => Self::StageOfChange,
            "refer-type" => Self::ReferType,
            "refer-cause" => Self::ReferCause,
            "refer-point" => Self::ReferPoint,
            "moph-refer-expire-type" => Self::MophReferExpType,
            "doc-type" => Self::DocumentType,
            _ => Self::Unknown,
        }
    }
}

impl SystemListType {
    pub fn key_type(&self) -> BasicType {
        match self {
            Self::PatientType => BasicType::Str,
            Self::ErBed => BasicType::Str,
            Self::ErPatientStatus => BasicType::Uint32,
            Self::ErDchType => BasicType::Uint32,
            Self::Ward => BasicType::Str,
            Self::Doctor => BasicType::Str,
            Self::DoctorAll => BasicType::Str,
            Self::Spclty => BasicType::Str,
            Self::SpcltyKphis => BasicType::Uint32,
            Self::Inscl => BasicType::Str,
            Self::Emergency => BasicType::Uint32,
            Self::EmergencyLevel => BasicType::Int32,
            Self::ConsultType => BasicType::Uint32,
            Self::VsConscious => BasicType::Uint32,
            Self::VsUrineAmount => BasicType::Uint32,
            Self::VsUrineDuration => BasicType::Uint32,
            Self::VsLine => BasicType::Uint32,
            Self::VsCha => BasicType::Uint32,
            Self::VsVA => BasicType::Uint32,
            Self::VsMaas => BasicType::Uint32,
            Self::VsMotor => BasicType::Uint32,
            Self::VsO2 => BasicType::Uint32,
            Self::VsTube => BasicType::Uint32,
            // Self::VsIntake => BasicType::Uint32,
            // Self::VsOutput => BasicType::Uint32,
            Self::VsLrStation => BasicType::Uint32,
            Self::VsLrMembrane => BasicType::Uint32,
            Self::VsLrMoulding => BasicType::Uint32,
            Self::VsDipstick => BasicType::Uint32,
            Self::VsBreathing => BasicType::Uint32,
            Self::VsAvpu => BasicType::Uint32,
            Self::VsGutFeeling => BasicType::Uint32,
            Self::VsPopsOther => BasicType::Uint32,
            Self::StageOfChange => BasicType::Uint32,
            Self::ReferType => BasicType::Int32,
            Self::ReferCause => BasicType::Int32,
            Self::ReferPoint => BasicType::Str,
            Self::MophReferExpType => BasicType::Int32,
            Self::DocumentType => BasicType::Uint8,
            Self::Unknown => BasicType::Str,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::PatientType => "patient-type",
            Self::ErBed => "er-bed",
            Self::ErPatientStatus => "er-patient-status",
            Self::ErDchType => "er-dch-type",
            Self::Ward => "ward",
            Self::Doctor => "doctor",
            Self::DoctorAll => "doctor-all",
            Self::Spclty => "spclty",
            Self::SpcltyKphis => "spclty-id",
            Self::Inscl => "inscl",
            Self::Emergency => "emergency",
            Self::EmergencyLevel => "emergency-level",
            Self::ConsultType => "consult-type",
            Self::VsConscious => "conscious",
            Self::VsUrineAmount => "urine-amount",
            Self::VsUrineDuration => "urine-duration",
            Self::VsLine => "line",
            Self::VsCha => "cha",
            Self::VsVA => "va",
            Self::VsMaas => "mass",
            Self::VsMotor => "motor",
            Self::VsO2 => "o2",
            Self::VsTube => "tube",
            // Self::VsIntake => "intake",
            // Self::VsOutput => "output",
            Self::VsLrStation => "lr-sta",
            Self::VsLrMembrane => "lr-mem",
            Self::VsLrMoulding => "lr-moulding",
            Self::VsDipstick => "dipstick",
            Self::VsBreathing => "breathing",
            Self::VsAvpu => "avpu",
            Self::VsGutFeeling => "gut-feeling",
            Self::VsPopsOther => "pops-other",
            Self::StageOfChange => "stage-of-change",
            Self::ReferType => "refer-type",
            Self::ReferCause => "refer-cause",
            Self::ReferPoint => "refer-point",
            Self::MophReferExpType => "moph-refer-expire-type",
            Self::DocumentType => "doc-type",
            Self::Unknown => "unknown",
        }
    }

    pub fn to_label(&self) -> &'static str {
        match self {
            Self::PatientType => "ประเภทผู้ป่วย",
            Self::ErBed => "เลขเตียง ER",
            Self::ErPatientStatus => "สถานะผู้ป่วย ER",
            Self::ErDchType => "ประเภทการ Discharge ER",
            Self::Ward => "Ward",
            Self::Doctor => "แพทย์",
            Self::DoctorAll => "เจ้าหน้าที่",
            Self::Spclty => "แผนก (HOSxP)",
            Self::SpcltyKphis => "แผนก (KPHIS)",
            Self::Inscl => "สิทธิ์",
            Self::Emergency => "ความเร่งด่วน",
            Self::EmergencyLevel => "ระดับความฉุกเฉิน",
            Self::ConsultType => "ชนิดใบ Consult",
            Self::VsConscious => "Conscious",
            Self::VsUrineAmount => "Urine Amount",
            Self::VsUrineDuration => "Urine Duration",
            Self::VsLine => "Line",
            Self::VsCha => "Pupil Response",
            Self::VsVA => "VA",
            Self::VsMaas => "MAAS",
            Self::VsMotor => "Motor Power",
            Self::VsO2 => "O2",
            Self::VsTube => "Tube",
            // Self::VsIntake => "Intake",
            // Self::VsOutput => "Output",
            Self::VsLrStation => "Station",
            Self::VsLrMembrane => "Membrane",
            Self::VsLrMoulding => "Moulding",
            Self::VsDipstick => "Dipstick",
            Self::VsBreathing => "Breathing",
            Self::VsAvpu => "AVPU",
            Self::VsGutFeeling => "Gut Feeling",
            Self::VsPopsOther => "POPs Other",
            Self::StageOfChange => "Stage-Of-Change",
            Self::ReferType => "Refer Type",
            Self::ReferCause => "Refer Cause",
            Self::ReferPoint => "Refer Point",
            Self::MophReferExpType => "Moph-Refer Expire Type",
            Self::DocumentType => "Document Type",
            Self::Unknown => "As TEXT type",
        }
    }

    pub fn source_hint(&self) -> &'static str {
        match self {
            Self::PatientType => "Config: fcnote-patient-types",
            Self::ErBed => "kphis.opd_er_bed.opd_er_bed_id",
            Self::ErPatientStatus => "kphis.opd_er_patient_status.er_patient_status_id",
            Self::ErDchType => "kphis.opd_er_dch_type.er_dch_type_id",
            Self::Ward => "hos.ward.ward",
            Self::Doctor => "hos.doctor.code",
            Self::DoctorAll => "hos.doctor.code",
            Self::Spclty => "hos.spclty.spclty",
            Self::SpcltyKphis => "kphis.kphis_spclty.spclty_id",
            Self::Inscl => "hos.nhso_inscl_code.inscl_code",
            Self::Emergency => "kphis.ipd_emergency.emergency_id",
            Self::EmergencyLevel => "hos.er_emergency_level.er_emergency_level_id",
            Self::ConsultType => "kphis.ipd_dr_consult_type.consult_type_id",
            Self::VsConscious => "kphis.ipd_vs_conscious.conscious_id",
            Self::VsUrineAmount => "kphis.ipd_vs_urine_amount.urine_amount_id",
            Self::VsUrineDuration => "kphis.ipd_vs_urine_duration.urine_d_id",
            Self::VsLine => "kphis.ipd_vs_line.line_id",
            Self::VsCha => "kphis.ipd_vs_cha.cha_id",
            Self::VsVA => "kphis.ipd_vs_va.va_id",
            Self::VsMaas => "kphis.ipd_vs_mass.mass_id",
            Self::VsMotor => "kphis.ipd_vs_lt_arm.lt_arm",
            Self::VsO2 => "kphis.ipd_vs_o2.o2_id",
            Self::VsTube => "kphis.ipd_vs_tube.tube_id",
            // Self::VsIntake => "kphis.ipd_vs_intake.intake_id",
            // Self::VsOutput => "kphis.ipd_vs_output.output_id",
            Self::VsLrStation => "kphis.ipd_vs_lr_sta.lr_sta_id",
            Self::VsLrMembrane => "kphis.ipd_vs_lr_mem.lr_mem_id",
            Self::VsLrMoulding => "kphis.ipd_vs_lr_moulding.lr_moulding_id",
            Self::VsDipstick => "kphis.ipd_vs_dipstick.dipstick_id",
            Self::VsBreathing => "kphis.ipd_vs_breathing.breathing_id",
            Self::VsAvpu => "kphis.ipd_vs_avpu.avpu_id",
            Self::VsGutFeeling => "kphis.ipd_vs_gut_feeling.gut_feeling_id",
            Self::VsPopsOther => "kphis.ipd_vs_pops_other.pops_other_id",
            Self::StageOfChange => "kphis.ipd_vs_stage_of_change.stage_of_change_id",
            Self::ReferType => "hos.refer_type.refer_type",
            Self::ReferCause => "hos.refer_cause.id",
            Self::ReferPoint => "hos.refer_point_list.name",
            Self::MophReferExpType => "hos.moph_refer_expire_type.moph_refer_expire_type_id",
            Self::DocumentType => "Static code",
            Self::Unknown => "Use as TEXT type",
        }
    }

    pub fn get_items(&self, assets: &AppAsset) -> Vec<KeyLabel> {
        match self {
            Self::PatientType => assets.fcnote_patient_type_select_options.iter().map(KeyLabel::from).collect(),
            Self::ErBed => assets.er_bed_select_options.iter().map(KeyLabel::from).collect(),
            Self::ErPatientStatus => assets.er_patient_status_select_options.iter().map(KeyLabel::from).collect(),
            Self::ErDchType => assets.er_dch_type_select_options.iter().map(KeyLabel::from).collect(),
            Self::Ward => assets.ward_select_option.iter().map(KeyLabel::from).collect(),
            Self::Doctor => assets.doctor_select_option.iter().map(KeyLabel::from).collect(),
            Self::DoctorAll => assets.all_doctor_select_option.iter().map(KeyLabel::from).collect(),
            Self::Spclty => assets.spclty_select_option.iter().map(KeyLabel::from).collect(),
            Self::SpcltyKphis => assets.spclty_kphis_select_option.iter().map(KeyLabel::from).collect(),
            Self::Inscl => assets.inscl_select_option.iter().map(KeyLabel::from).collect(),
            Self::Emergency => assets.emergency_select_option.iter().map(KeyLabel::from).collect(),
            Self::EmergencyLevel => assets.emergency_level_select_option.iter().map(KeyLabel::from).collect(),
            Self::ConsultType => assets.consult_type_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsConscious => assets.conscious_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsUrineAmount => assets.urine_amount_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsUrineDuration => assets.urine_duration_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsLine => assets.line_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsCha => assets.cha_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsVA => assets.va_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsMaas => assets.mass_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsMotor => assets.motor_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsO2 => assets.o2_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsTube => assets.tube_select_option.iter().map(KeyLabel::from).collect(),
            // Self::VsIntake => assets.intake_select_option.iter().map(KeyLabel::from).collect(),
            // Self::VsOutput => assets.output_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsLrStation => assets.lr_sta_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsLrMembrane => assets.lr_mem_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsLrMoulding => assets.lr_moulding_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsDipstick => assets.dipstick_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsBreathing => assets.breathing_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsAvpu => assets.avpu_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsGutFeeling => assets.gut_feeling_select_option.iter().map(KeyLabel::from).collect(),
            Self::VsPopsOther => assets.pops_other_select_option.iter().map(KeyLabel::from).collect(),
            Self::StageOfChange => assets.stage_of_change_select_option.iter().map(KeyLabel::from).collect(),
            Self::ReferType => assets.refer_type_select_option.iter().map(KeyLabel::from).collect(),
            Self::ReferCause => assets.refer_cause_select_option.iter().map(KeyLabel::from).collect(),
            Self::ReferPoint => assets.refer_point_select_option.iter().map(KeyLabel::from).collect(),
            Self::MophReferExpType => assets.moph_refer_expire_type_select_option.iter().map(KeyLabel::from).collect(),
            Self::DocumentType => assets.document_type_select_option.iter().map(KeyLabel::from).collect(),
            Self::Unknown => Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct KeyLabel {
    pub key: String,
    pub label: String,
}

impl std::convert::From<&ColorSelectOption> for KeyLabel {
    fn from(item: &ColorSelectOption) -> Self {
        Self {
            key: item.key.to_owned(),
            label: item.value.to_owned(),
        }
    }
}

impl std::convert::From<&SelectOption> for KeyLabel {
    fn from(item: &SelectOption) -> Self {
        Self {
            key: item.key.to_owned(),
            label: item.value.to_owned(),
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use super::*;

    #[test]
    fn test_parse_report_params() {
        for i in [
            "k1^Ant^bool|k2^Bee^i8|k3^Cat^i16|k4^Dog^i32|k5^Egg^i64|k6^Fish^u8|k7^Iris^u16|k8^Jar^u32|k9^King^u64|k10^Lama^f32|k11^Monk^f64|k12^Nut^decimal|k13^Ox^date|k14^Pig^time|k15^Queen^datetime|v16^Rat^value",
            "k1^Audit^(str,I,Internal,E,External)|k2^Key^(u32,1,Internal,2,External)"
        ] {
            assert_eq!(ReportParam::to_cap_pipe(&ReportParam::from_cap_pipe(i)), i);
        }
    }
}
