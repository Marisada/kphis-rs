use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use js_sys::Array;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    A4_HEIGHT, A4_WIDTH,
    app::{AppState, VisitTypeId},
    avatar::{AvatarEnum, AvatarOpdEr, AvatarParams, AvatarWard},
    endpoint::EndPoint,
    fetch::Method,
    report::{CustomReport, ReportParam, ReportTemplateParams, SystemReport, TypstRaw, TypstReport, TypstSvg},
    timer::Timeout,
};
use kphis_ui_app::App;
use kphis_ui_component::modal::report::param_input::ReportParamInput;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins, pannable::PanState};
use kphis_util::{
    datetime::datetime_th_relative,
    error::CONTACT_ADMIN,
    util::{str_some, zoom_step},
};

/// - GET `EndPoint::ReportCustom`
/// - GET `EndPoint::ReportRawTemplateTypeId`
/// - GET `EndPoint::AvatarIpd`
/// - GET `EndPoint::AvatarOpdEr`
/// - GET `EndPoint::ReportTemplateTypeId` (guarded, remove `Signed PDF` btn)
#[derive(Clone, Default)]
pub struct ReportViewerPage {
    report_type: Mutable<ReportType>,
    search: Mutable<String>,
    vnan: Mutable<String>,

    loaded_custom_templates_compact: Mutable<bool>,
    renew_system_templates_select_box: Mutable<bool>,
    renew_ward_select_box: Mutable<bool>,

    system_templates: MutableVec<SystemReport>,
    selected_system_template: Mutable<Option<SystemReport>>,

    custom_templates_compact: MutableVec<CustomReport>,
    selected_custom_template_compact: Mutable<String>,
    custom_template_changed: Mutable<bool>,
    renew_custom_templates_select_box: Mutable<bool>,
    selected_custom_template: Mutable<Option<CustomReport>>,

    ids: MutableVec<Rc<ReportParamInput>>,
    ids_changed: Mutable<bool>,

    data_json: Mutable<String>,

    load_and_render_svg: Mutable<bool>,
    report_svg: MutableVec<Rc<TypstSvg>>,
    report_width_percent: Mutable<f64>,

    load_and_render_pdf: Mutable<bool>,
    load_signed_pdf: Mutable<bool>,

    viewer_position_percent: Mutable<f64>,
    viewer_top: Mutable<u32>,
    viewer_top_renew: Mutable<bool>,
    // Pannable
    pan_state: Rc<PanState>,

    search_result: MutableVec<Rc<AvatarEnum>>,
    search_changed: Mutable<bool>,
    search_selected: Mutable<Option<VisitTypeId>>,
}

impl ReportViewerPage {
    pub fn new(app: Rc<App>) -> Rc<Self> {
        let selected_system_template = SystemReport::new(&app.report_select.lock_ref());
        let search_changed = selected_system_template.is_some();
        Rc::new(Self {
            // default report_type is ReportType::Ipd
            system_templates: MutableVec::new_with_values(SystemReport::ipd_set()),
            selected_system_template: Mutable::new(selected_system_template),
            search_changed: Mutable::new(search_changed),
            ..Default::default()
        })
    }

    fn load_custom_templates_compact(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                let params = ReportTemplateParams {
                    disabled: Some(false),
                    compact: Some(true),
                    ..Default::default()
                };
                // GET `EndPoint::ReportCustom`
                match CustomReport::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.custom_templates_compact.lock_mut();
                        lock.replace_cloned(responses);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                        page.custom_templates_compact.lock_mut().clear();
                    }
                }
            }),
        )
    }

    fn load_custom_template(page: Rc<Self>, app: Rc<App>) {
        let template_id = page.selected_custom_template_compact.lock_ref().parse::<u32>().ok();
        if template_id.is_some() {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let params = ReportTemplateParams {
                        template_id,
                        ..Default::default()
                    };
                    // GET `EndPoint::ReportCustom`
                    match CustomReport::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            if let Some(template) = responses.first() {
                                let ids_opt = template.statement_params.as_ref().map(|cap_pipe| {
                                    ReportParam::from_cap_pipe(cap_pipe).iter().map(|param| ReportParamInput::new(param)).collect::<Vec<Rc<ReportParamInput>>>()
                                });
                                page.selected_custom_template.set(Some(template.to_owned()));
                                // create spaces for all params in ids
                                if let Some(ids) = ids_opt {
                                    page.ids.lock_mut().replace_cloned(ids);
                                } else {
                                    page.ids.lock_mut().clear();
                                }
                            } else {
                                page.selected_custom_template.set(None);
                                page.selected_custom_template_compact.set(String::new());
                                page.ids.lock_mut().clear();
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.selected_custom_template.set(None);
                            page.selected_custom_template_compact.set(String::new());
                            page.ids.lock_mut().clear();
                        }
                    }
                    page.data_json.set_neq(String::new());
                }),
            )
        }
    }

    fn load_and_render_svg(page: Rc<Self>, app: Rc<App>) {
        if let (Some(template), Some(ids)) = match *page.report_type.lock_ref() {
            ReportType::Ipd | ReportType::OpdEr => (
                page.selected_system_template
                    .get_cloned()
                    .map(|selected| TypstReport::from_system_with_coercion(selected, &app.state().report_coercions())),
                str_some(page.vnan.get_cloned()),
            ),
            ReportType::Custom => (
                page.selected_custom_template.get_cloned().map(|selected| TypstReport::Custom(selected)),
                str_some(page.ids.lock_ref().iter().map(|param| param.to_request_id()).collect::<Vec<String>>().join("|")),
            ),
        } {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ReportRawTemplateTypeId`
                    match TypstRaw::call_api_get(template.template_name(), template.report_type(), &ids, app.state()).await {
                        Ok(response) => {
                            page.data_json.set(response.data_json.clone());
                            let bytes = app.typst_worker().await.svg(response.typ, response.data_json, app.token().unwrap_or_default()).await;
                            let reports = if bytes.is_empty() {
                                vec![TypstSvg::default()]
                            } else {
                                match bitcode::decode(&bytes) {
                                    Ok(pages) => pages,
                                    Err(e) => {
                                        app.alert_error_with_clipboard(CONTACT_ADMIN, &["BitcodeError: ", &e.to_string()].concat()).await;
                                        vec![TypstSvg::default()]
                                    }
                                }
                            };
                            {
                                let mut lock = page.report_svg.lock_mut();
                                lock.clear();
                                lock.extend(reports.into_iter().map(Rc::new));
                            }
                            page.set_fit(app);
                            page.viewer_position_percent.set(0.0);
                            page.set_viewer_offset_and_inner_height();
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.data_json.set_neq(String::new());
                        }
                    }
                }),
            )
        } else {
            page.data_json.set_neq(String::new());
        }
    }

    fn cannot_pdf(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let report_type = self.report_type.signal_cloned(),
            let no_system_template = self.selected_system_template.signal_ref(|opt| opt.is_none()),
            let no_custom_template = self.selected_custom_template.signal_ref(|opt| opt.is_none()),
            let no_vnan = self.vnan.signal_ref(|vnan| vnan.is_empty()),
            let not_all_ids = not(self.ids.signal_vec_cloned().filter_signal_cloned(|param| {
                param.is_empty_signal()
            }).is_empty()) =>
            match report_type {
                ReportType::Ipd | ReportType::OpdEr => *no_system_template || *no_vnan,
                ReportType::Custom => *no_custom_template || *not_all_ids
            }
        }
    }

    fn prepare_report_parts(&self, app: Rc<AppState>) -> Option<ReportParts> {
        match self.report_type.get_cloned() {
            ReportType::Ipd | ReportType::OpdEr => str_some(self.vnan.get_cloned()).and_then(|ids| {
                self.selected_system_template.get_cloned().map(|selected| ReportParts {
                    file_name: selected.download_file_name(&ids),
                    title: selected.title_with_ids(&ids),
                    template: TypstReport::from_system_with_coercion(selected, &app.report_coercions()),
                    ids: ids,
                })
            }),
            ReportType::Custom => str_some(self.ids.lock_ref().iter().map(|param| param.to_request_id()).collect::<Vec<String>>().join("|")).and_then(|ids| {
                self.selected_custom_template.get_cloned().map(|selected| ReportParts {
                    file_name: selected.download_file_name(&ids),
                    title: selected.title_with_ids(&ids),
                    template: TypstReport::Custom(selected),
                    ids: ids,
                })
            }),
        }
    }

    fn load_and_render_pdf(page: Rc<Self>, app: Rc<App>) {
        let author = app.app_status.lock_ref().as_ref().map(|status| status.hospital_name.clone()).unwrap_or_default();
        let user = app.user.lock_ref().as_ref().map(|user| user.user.name.get_cloned()).unwrap_or_default();
        if let Some(parts) = page.prepare_report_parts(app.state()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ReportRawTemplateTypeId`
                    match TypstRaw::call_api_get(parts.template.template_name(), parts.template.report_type(), &parts.ids, app.state()).await {
                        Ok(response) => {
                            page.data_json.set(response.data_json.clone());
                            let bytes = app.typst_worker().await.pdf(
                                response.typ,
                                response.data_json,
                                parts.title,
                                author,
                                user,
                                app.token().unwrap_or_default(),
                            ).await;
                            if !bytes.is_empty() {
                                app.open_file_with_mime(&bytes, &parts.file_name, "application/pdf");
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.data_json.set_neq(String::new());
                        }
                    }
                }),
            )
        }
    }

    fn load_signed_pdf(page: Rc<Self>, app: Rc<App>) {
        if let Some(parts) = page.prepare_report_parts(app.state()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::ReportTemplateTypeId`
                    match AppState::call_api_get_pdf_report(parts.template.template_name(), parts.template.report_type(), &parts.ids, app.state()).await {
                        Ok(blob) => {
                            app.open_response_blob(blob, &parts.template.download_file_name(&parts.ids));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn submit_search(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                match *page.report_type.lock_ref() {
                    ReportType::Ipd => {
                        let params = AvatarParams {
                            ward: str_some(app.ward_select.get_cloned()),
                            search: str_some(page.search.get_cloned()),
                        };
                        if params.is_empty() {
                            page.search_result.lock_mut().clear();
                        } else {
                            // GET `EndPoint::AvatarIpd`
                            match AvatarWard::call_api_get(&params, app.state()).await {
                                Ok(items) => {
                                    let mut lock = page.search_result.lock_mut();
                                    lock.clear();
                                    lock.extend(items.iter().map(|im| Rc::new(AvatarEnum::from(im))));
                                    if items.len() == 1 {
                                        page.search_selected.set(items.first().map(|i| i.visit_type(app.hosxp_an_len())));
                                    } else {
                                        page.search_selected.set(None);
                                    }
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                    }
                    ReportType::OpdEr => {
                        // GET `EndPoint::AvatarOpdEr`
                        match AvatarOpdEr::call_api_get(app.state()).await {
                            Ok(items) => {
                                let mut lock = page.search_result.lock_mut();
                                lock.clear();
                                lock.extend(items.iter().map(|im| Rc::new(AvatarEnum::from(im))));
                                if items.len() == 1 {
                                    page.search_selected.set(items.first().map(|i| i.visit_type()));
                                } else {
                                    page.search_selected.set(None);
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                    ReportType::Custom => {

                    }
                }
            }),
        );
    }

    fn set_fit(&self, app: Rc<App>) {
        if let Some(viewer) = app.get_id("report-right-panel") {
            let viewer_width = viewer.client_width() as f64;
            let reports_width = self.report_svg.lock_ref().iter().max_by_key(|i| i.width as u64).map(|i| i.width).unwrap_or(A4_WIDTH);
            let percent = (viewer_width * 100.0) / reports_width;
            self.report_width_percent.set(percent);
        }
    }

    fn set_viewer_offset_and_inner_height(&self) {
        let percent = self.report_width_percent.get();
        let viewer_position_percent = self.viewer_position_percent.get();

        let report_count = self.report_svg.lock_ref().len();
        let reports_raw_height = self.report_svg.lock_ref().iter().map(|i| i.height).sum::<f64>();
        let reports_exact_height = if reports_raw_height > 0.0 { reports_raw_height } else { A4_HEIGHT };
        let gaps = ((report_count - 1) * 32) as f64;
        let reports_adjusted_height = (reports_exact_height * percent / 100.0) - gaps;

        self.viewer_top.set((reports_adjusted_height * viewer_position_percent / 100.0) as u32);
        self.viewer_top_renew.set(true);
    }

    fn set_viewer_position_percent(&self, app: Rc<App>) {
        if let Some(elm) = app.get_id("designer-viewer-gut") {
            let gut = elm.dyn_into::<HtmlElement>().unwrap();
            let content_height = gut.scroll_height() as u32;
            let scroll_top = gut.scroll_top() as u32;
            let content_position_percent = if content_height > 0 { scroll_top as f64 / content_height as f64 * 100.0 } else { 0.0 };
            self.viewer_position_percent.set(content_position_percent);
        }
    }

    fn renew_system_templates_select_box(page: Rc<Self>, app: Rc<App>) {
        if let Some(elm) = app.get_id("system_templates") {
            Timeout::new(
                0,
                clone!(page => move || {
                    if let Some(template) = page.selected_system_template.get_cloned() {
                        if page.system_templates.lock_ref().contains(&template) {
                            NiceSelect::new_default_with_value(&elm, template.template_name());
                        } else {
                            page.selected_system_template.set(None);
                            NiceSelect::new_default(&elm);
                        }
                    } else {
                        NiceSelect::new_default(&elm);
                    }
                }),
            )
            .forget();
        }
    }

    fn renew_custom_templates_select_box(page: Rc<Self>, app: Rc<App>) {
        if let Some(elm) = app.get_id("custom_templates") {
            Timeout::new(
                0,
                clone!(page => move || {
                    let template = page.selected_custom_template_compact.lock_ref();
                    if !template.is_empty() {
                            NiceSelect::new_default_with_value(&elm, &template);
                    } else {
                        NiceSelect::new_default(&elm);
                    }
                }),
            )
            .forget();
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Report Viewer");

        let ward_select_option = app.app_asset.lock_ref().as_ref().map(|asset| asset.ward_select_option.clone()).unwrap_or_default();

        let allow_signed_pdf = app.endpoint_is_allow(&Method::GET, &EndPoint::ReportTemplateTypeId, false) && app.can_sign_pdf();

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::renew_system_templates_select_box(page.clone(), app.clone());
                }
                async{}
            })))
            .future(page.renew_system_templates_select_box.signal().for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::renew_system_templates_select_box(page.clone(), app.clone());
                    page.renew_system_templates_select_box.set(false);
                }
                async{}
            })))
            .future(page.renew_custom_templates_select_box.signal().for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::renew_custom_templates_select_box(page.clone(), app.clone());
                    page.renew_custom_templates_select_box.set(false);
                }
                async{}
            })))
            .future(page.renew_ward_select_box.signal().for_each(clone!(app, page => move |ready| {
                if ready {
                    if let Some(elm) = app.get_id("wards") {
                        let ward = app.ward_select.lock_ref();
                        if !ward.is_empty() {
                            NiceSelect::new_default_with_value(&elm, &ward);
                        } else {
                            NiceSelect::new_default(&elm);
                        }
                    }
                    page.renew_ward_select_box.set(false);
                }
                async{}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_custom_templates_compact.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_custom_templates_compact(page.clone(), app.clone());
                    page.loaded_custom_templates_compact.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.search_changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit_search(page.clone(), app.clone());
                    page.search_changed.set(false);
                }
                async {}
            })))
            .future(page.search_selected.signal_cloned().for_each(clone!(page => move |opt| {
                if let Some(visit_type) = opt {
                    page.vnan.set(visit_type.vnan().to_owned());
                    page.load_and_render_svg.set(true);
                }
                async{}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.custom_template_changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_custom_template(page.clone(), app.clone());
                    page.custom_template_changed.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let render = page.load_and_render_svg.signal() =>
                !busy && *render
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_and_render_svg(page.clone(), app.clone());
                    page.load_and_render_svg.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let render = page.load_and_render_pdf.signal() =>
                !busy && *render
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_and_render_pdf(page.clone(), app.clone());
                    page.load_and_render_pdf.set(false);
                }
                async {}
            })))
            .apply_if(allow_signed_pdf, |dom| dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let render = page.load_signed_pdf.signal() =>
                    !busy && *render
                ).for_each(clone!(app, page => move |ready| {
                    if ready {
                        Self::load_signed_pdf(page.clone(), app.clone());
                        page.load_signed_pdf.set(false);
                    }
                    async {}
                })))
            )
            .global_event(clone!(page => move |e: events::MouseMove| {
                PanState::on_mouse_move(&e, page.pan_state.clone());
            }))
            .global_event(clone!(page => move |_: events::MouseUp| {
                PanState::on_mouse_up(page.pan_state.clone());
            }))
            .class("container-fluid")
            .child(html!("div", {
                .class(class::ROW_NOWRAP)
                .style("height", "calc(100vh - 74px)")
                .children([
                    // left panel
                    html!("div", {
                        .class("mt-3")
                        .style("width","450px")
                        .style("height", "100%")
                        .children([
                            html!("div", {
                                .class(class::INPUT_GROUP_T)
                                .children([
                                    html!("button", {
                                        .class(class::BTN_BLUEO)
                                        .class_signal("active", page.report_type.signal_cloned().map(|report_type| matches!(report_type, ReportType::Ipd)))
                                        .attr("type", "button")
                                        .attr("data-bs-toggle", "button")
                                        .text("IPD")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.report_type.set(ReportType::Ipd);
                                            {
                                                let mut lock = page.system_templates.lock_mut();
                                                lock.replace_cloned(SystemReport::ipd_set());
                                            }
                                            page.report_svg.lock_mut().clear();
                                            page.vnan.set_neq(String::new());
                                            page.ids.lock_mut().clear();
                                            page.data_json.set_neq(String::new());
                                            page.search_changed.set(true);
                                        }))
                                    }),
                                    html!("button", {
                                        .class(class::BTN_BLUEO)
                                        .class_signal("active", page.report_type.signal_cloned().map(|report_type| matches!(report_type, ReportType::OpdEr)))
                                        .attr("type", "button")
                                        .attr("data-bs-toggle", "button")
                                        .text("OPD-ER")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.report_type.set(ReportType::OpdEr);
                                            {
                                                let mut lock = page.system_templates.lock_mut();
                                                lock.replace_cloned(SystemReport::opd_er_set());
                                            }
                                            page.report_svg.lock_mut().clear();
                                            page.vnan.set_neq(String::new());
                                            page.ids.lock_mut().clear();
                                            page.data_json.set_neq(String::new());
                                            page.search_changed.set(true);
                                        }))
                                    }),
                                ])
                                .child_signal(page.custom_templates_compact.signal_vec_cloned().is_empty().map(clone!(page => move |is_empty| {
                                    (!is_empty).then(|| {
                                        html!("button", {
                                            .class(class::BTN_BLUEO)
                                            .class_signal("active", page.report_type.signal_cloned().map(|report_type| matches!(report_type, ReportType::Custom)))
                                            .attr("type", "button")
                                            .attr("data-bs-toggle", "button")
                                            .text("Custom")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.report_type.set(ReportType::Custom);
                                                page.report_svg.lock_mut().clear();
                                                page.vnan.set_neq(String::new());
                                                page.ids.lock_mut().clear();
                                                page.data_json.set_neq(String::new());
                                                page.search_result.lock_mut().clear();
                                                page.selected_custom_template.set(None);
                                                page.selected_custom_template_compact.set(String::new());
                                            }))
                                        })
                                    })
                                })))
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .child(doms::span_group_text("รายงาน"))
                                .child_signal(page.report_type.signal_cloned().map(clone!(app, page => move |report_type| {
                                    match report_type {
                                        ReportType::Ipd
                                        | ReportType::OpdEr => {
                                            page.renew_system_templates_select_box.set(true);
                                            Some(html!("div", {
                                                .class(class::FLEX_W100)
                                                .child(html!("select" => HtmlSelectElement, {
                                                    .class(class::FORM_CTRL_SM_WRAP)
                                                    .attr("id", "system_templates")
                                                    .children_signal_vec(page.system_templates.signal_vec_cloned().map(|template| {
                                                        html!("option", {
                                                            .attr("value", template.template_name())
                                                            .text(template.title())
                                                        })
                                                    }))
                                                    .prop_signal("value", page.selected_system_template.signal_cloned().map(|opt| opt.as_ref().map(|selected| selected.template_name()).unwrap_or_default()))
                                                    .with_node!(element => {
                                                        .event(clone!(app, page => move |_: events::Change| {
                                                            let v = element.value();
                                                            page.selected_system_template.set(SystemReport::new(&v));
                                                            app.report_select.set(v);
                                                            app.to_local_storage();
                                                            page.load_and_render_svg.set(true);
                                                        }))
                                                    })
                                                }))
                                            }))
                                        }
                                        ReportType::Custom => {
                                            page.renew_custom_templates_select_box.set(true);
                                            Some(html!("div", {
                                                .class(class::FLEX_W100)
                                                .child(html!("select" => HtmlSelectElement, {
                                                    .class(class::FORM_CTRL_SM_WRAP)
                                                    .attr("id", "custom_templates")
                                                    // .child(html!("option", {.attr("value","").text("เลือกรายงาน")}))
                                                    .children_signal_vec(page.custom_templates_compact.signal_vec_cloned().map(|template| {
                                                        html!("option", {
                                                            .attr("value", &template.template_id.to_string())
                                                            .text(&template.template_name)
                                                        })
                                                    }))
                                                    .apply(mixins::string_value_select(page.selected_custom_template_compact.clone(), page.custom_template_changed.clone()))
                                                }))
                                            }))
                                        }
                                    }

                                })))
                                .child_signal(app.loader_is_loading().map(clone!(page => move |is_loading| {
                                    Some(if is_loading {
                                        html!("div", {
                                            .style("font-size","24px")
                                            .style("line-height","30px")
                                            .style("padding-left","5px")
                                            .child(html!("i", {.class(class::FA_SPIN)}))
                                        })
                                    } else {
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_GRAY)
                                            .child(html!("i", {.class(class::FA_SYNC)}))
                                            .event(clone!(page => move |_:events::Click| {
                                                page.load_and_render_svg.set(true);
                                            }))
                                        })
                                    })
                                })))
                            }),
                        ])
                        .children_signal_vec(page.report_type.signal_cloned().map(clone!(app, page => move |report_type| {
                            match report_type {
                                ReportType::Ipd => {
                                    page.renew_ward_select_box.set(true);
                                    vec![
                                        html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .class("mt-2")
                                            .children([
                                                doms::span_group_text("Ward"),
                                                html!("div", {
                                                    .class(class::FLEX_W100)
                                                    .child(html!("select" => HtmlSelectElement, {
                                                        .class(class::FORM_CTRL_SM_WRAP)
                                                        .attr("id", "wards")
                                                        .children(ward_select_option.iter().map(|option| {
                                                            doms::select_option(option, &app.ward_select.lock_ref())
                                                        }))
                                                        .prop_signal("value", app.ward_select.signal_cloned())
                                                        .with_node!(element => {
                                                            .event(clone!(app, page, element => move |_: events::Change| {
                                                                app.ward_select.set_neq(element.value());
                                                                app.to_local_storage();
                                                                page.search_changed.set_neq(true);
                                                            }))
                                                        })
                                                    }))
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_RED)
                                                    .child(html!("i", {.class(class::FA_X)}))
                                                    .event(clone!(app, page => move |_:events::Click| {
                                                        let no_ward = app.ward_select.lock_ref().is_empty();
                                                        if !no_ward {
                                                            app.ward_select.set(String::new());
                                                            if let Some(elm) = app.get_id("wards") {
                                                                NiceSelect::new_default(&elm);
                                                            }
                                                            page.search_changed.set_neq(true);
                                                        }
                                                    }))
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .class("mt-2")
                                            .children([
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class(class::FORM_CTRL_SM)
                                                    .focused(true)
                                                    .attr("placeholder", "HN/AN/ชื่อ-สกุล")
                                                    .prop_signal("value", page.search.signal_cloned())
                                                    .with_node!(element => {
                                                        .event_with_options(&EventOptions::preventable(), clone!(page, element => move |event: events::KeyDown| {
                                                            if event.key() == "Enter" {
                                                                event.prevent_default();
                                                                page.search.set_neq(element.value());
                                                                page.search_changed.set_neq(true);
                                                            }
                                                        }))
                                                        .event(clone!(page => move |_: events::Change| {
                                                            page.search.set_neq(element.value());
                                                        }))
                                                    })
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_RED)
                                                    .child(html!("i", {.class(class::FA_X)}))
                                                    .event(clone!(page => move |_:events::Click| {
                                                        let no_search = page.search.lock_ref().is_empty();
                                                        if !no_search {
                                                            page.search.set(String::new());
                                                            page.search_changed.set_neq(true);
                                                        }
                                                    }))
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_BLUE)
                                                    .text("ค้นหา")
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.search_changed.set_neq(true);
                                                    }))
                                                }),
                                            ])
                                        }),
                                        Self::render_patient_list(true, page.clone(), app.clone()),
                                    ]
                                }
                                ReportType::OpdEr => {
                                    vec![
                                        Self::render_patient_list(false, page.clone(), app.clone()),
                                    ]
                                }
                                ReportType::Custom => {
                                    vec![
                                        Self::render_custom_params(page.clone(), app.clone()),
                                    ]
                                }

                            }
                        })).to_signal_vec())
                    }),
                    // right panel
                    html!("div", {
                        .attr("id", "report-right-panel")
                        .class("p-0")
                        .style("width", "calc(100vw - 450px)")
                        .style("height", "calc(100% + 15px)")
                        .child(html!("div", {
                            .attr("id", "report-right-gut")
                            .style("background-color","#eee")
                            .apply(PanState::pan_container_mixins(page.pan_state.clone()))
                            .child(html!("div", {
                                .apply(mixins::typst_svg_mixins(page.report_width_percent.clone(), page.report_svg.clone()))
                            }))
                            .future(page.viewer_top_renew.signal().for_each(clone!(app, page => move |set_top| {
                                if set_top {
                                    page.viewer_top_renew.set(false);
                                    if let Some(gut) = app.get_id("report-right-gut") {
                                        gut.set_scroll_top(page.viewer_top.get() as i32);
                                    }
                                }
                                async {}
                            })))
                        }))
                    }),
                ])
            }))
            // tools
            .child(html!("div", {
                .style("position","fixed")
                .style("bottom","25px")
                .style("right","45px")
                .style("z-index","1")
                .child(html!("div", {
                    .class("d-flex")
                    .children([
                        html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_L_GRAY)
                            .child(html!("i", {.class(class::FA_ARROW_LR)}))
                            .event(clone!(app, page => move |_:events::Click| {
                                page.set_fit(app.clone());
                                page.set_viewer_position_percent(app.clone());
                                page.set_viewer_offset_and_inner_height();
                            }))
                        }),
                        html!("div", {
                            .class(class::INPUT_GROUP)
                            .style("max-width","170px")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .child(html!("i", {.class(class::FA_MINUS)}))
                                    .event(clone!(app, page => move |_:events::Click| {
                                        let zoom = page.report_width_percent.get();
                                        page.report_width_percent.set(zoom_step(zoom, false));
                                        page.set_viewer_position_percent(app.clone());
                                        page.set_viewer_offset_and_inner_height();
                                    }))
                                }),
                                html!("input" => HtmlInputElement, {
                                    .class(class::FORM_CTRL_C)
                                    .prop_signal("value", page.report_width_percent.signal_cloned().map(|u| [&u.round().to_string(), "%"].concat()))
                                    .with_node!(element => {
                                        .event(clone!(app, page => move |_:events::Change| {
                                            if let Ok(value) = element.value().trim_end_matches('%').parse::<f64>() {
                                                page.report_width_percent.set(value);
                                                page.set_viewer_position_percent(app.clone());
                                                page.set_viewer_offset_and_inner_height();
                                            }
                                        }))
                                    })
                                    // .text_signal(page.report_width_percent.signal_cloned().map(|u| [&u.round().to_string(), "%"].concat()))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                    .event(clone!(app, page => move |_:events::Click| {
                                        let zoom = page.report_width_percent.get();
                                        page.report_width_percent.set(zoom_step(zoom, true));
                                        page.set_viewer_position_percent(app.clone());
                                        page.set_viewer_offset_and_inner_height();
                                    }))
                                }),
                            ])
                        }),
                        html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_R_BLUE)
                            .text("PDF")
                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(page => move || {
                                page.load_and_render_pdf.set(true);
                            }), page.cannot_pdf(), app.state()))
                            // .event(clone!(page => move |_:events::Click| {
                            //     page.load_and_render_pdf.set(true);
                            // }))
                        }),
                    ])
                    .apply_if(allow_signed_pdf, |dom| dom
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_R_CYAN)
                            .style("white-space","nowrap")
                            .text("Signed PDF")
                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(page => move || {
                                page.load_signed_pdf.set(true);
                            }), page.cannot_pdf(), app.state()))
                            // .event(clone!(page => move |_:events::Click| {
                            //     page.load_signed_pdf.set(true);
                            // }))
                        }))
                    )
                    .child_signal(page.data_json.signal_cloned().map(clone!(app => move |json| {
                        (!json.is_empty()).then(|| {
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_R_BLUE)
                                .text("JSON")
                                .event(clone!(app => move |_:events::Click| {
                                    let blob = Blob::new_with_str_sequence(&Array::of1(&JsValue::from(&json))).unwrap();
                                    app.open_response_blob(blob, "data.json");
                                }))
                            })
                        })
                    })))
                }))
            }))
        })
    }

    fn render_patient_list(is_ipd: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        let height = if is_ipd { "calc(100vh - 237px)" } else { "calc(100vh - 159px)" };

        html!("div", {
            .class("mt-2")
            .style("height", height)
            .style("width", "100%")
            .style("box-sizing","border-box")
            .style("overflow-y","auto")
            .children([
                html!("table", {
                    .class(class::TABLE_STRIP)
                    .children([
                        html!("thead", {
                            .child(html!("tr", {
                                .child(html!("th", {
                                    .attr("scope", "col")
                                    .class("th-sm")
                                    .text("รายชื่อผู้ป่วย")
                                }))
                            }))
                        }),
                        html!("tbody", {
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app, page => move |row| {
                                doms::render_avatar(row, page.search_selected.clone(), app.state())
                            })))
                        }),
                    ])
                })
            ])
        })
    }

    fn render_custom_params(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("mt-2")
            .child_signal(page.selected_custom_template.signal_cloned().map(clone!(app, page => move |opt| {
                opt.map(|template| {
                    html!("div", {
                        .class("card")
                        .children([
                            html!("div", {
                                .class(class::CARD_HEAD_CYANS)
                                .child(html!("div", {
                                    .children([
                                        html!("span", {.class("fw-bold").text(&template.title)}),
                                        html!("br"),
                                        html!("span", {.text(&template.template_name)})
                                    ])
                                }))
                                .apply(|dom| {
                                    if let (Some(update_username), Some(update_datetime)) = (&template.update_username, &template.update_datetime) {
                                        dom.children([
                                            html!("span", {.class("fw-bold").text("โดย: ")}),
                                            html!("span", {.text(update_username)}),
                                            html!("br"),
                                            html!("span", {.class("fw-bold").text(" ปรับปรุง: ")}),
                                            html!("span", {.text(&datetime_th_relative(update_datetime))}),
                                        ])
                                    } else {
                                        dom
                                    }
                                })
                            }),
                            html!("div", {
                                .class("card-body")
                                .apply(|dom| {
                                    if let Some(info) = &template.info {
                                        dom.child(html!("span", {
                                            .style("white-space", "pre-wrap")
                                            .text(info)
                                        }))
                                    } else {
                                        dom
                                    }
                                })
                            }),
                            html!("div", {
                                .class("card-footer")
                                .children_signal_vec(page.ids.signal_vec_cloned().map(clone!(app, page => move |param| {
                                    ReportParamInput::render(param.clone(), page.ids_changed.clone(), app.clone())
                                })))
                                .child(html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_FR_T_BLUEO)
                                    .text("สร้างรายงาน")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(
                                        clone!(page => move || {
                                            page.load_and_render_svg.set(true);
                                        }),
                                        not(page.ids.signal_vec_cloned().filter_signal_cloned(|param| {
                                            param.is_empty_signal()
                                        }).is_empty()),
                                        app.state(),
                                    ))
                                }))
                            }),
                        ])
                    })
                })
            })))
        })
    }
}

#[derive(Clone, Default)]
enum ReportType {
    #[default]
    Ipd,
    OpdEr,
    Custom,
}

struct ReportParts {
    file_name: String,
    title: String,
    template: TypstReport,
    ids: String,
}
