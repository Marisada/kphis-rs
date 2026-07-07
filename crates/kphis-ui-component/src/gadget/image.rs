use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, pseudo, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use js_sys::{Array, JsString};
use std::{
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Blob, CanvasRenderingContext2d, ConstrainDomStringParameters, Event, File, FilePropertyBag, HtmlButtonElement, HtmlCanvasElement, HtmlInputElement, HtmlVideoElement, MediaDeviceInfo,
    MediaDeviceKind, MediaDevices, MediaStream, MediaStreamConstraints, MediaStreamTrack, MediaTrackConstraints,
};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::{ImagePath, ImageUsage},
    patient_info::PatientInfo,
    report::SystemReport,
    report::TypstReport,
    timer::Timeout,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::Viewer, class, mixins};
use kphis_util::{datetime::datetime_th_relative, util::str_some};

use crate::modal::{blank_modal, report::preview::ReportPreview};

static IMAGE_CPN_ID: AtomicUsize = AtomicUsize::new(1);

/// - GET `EndPoint::ImageUsageId` (guarded, invisible)
/// - POST `EndPoint::ImageUsage` (guarded-insided, remove เพิ่มรูป/camera btn in with_key mechanism, remove paste btn)<br>
/// **NOTE**: Not guarded when call `ImagePath::post_images()` outside this component
/// - DELETE `EndPoint::ImageUsage` (guarded, remove delete btn in with-key mechanism)
/// - POST `EndPoint::Image` (guarded, remove เพิ่มรูป/camera btn)
/// - PATCH `EndPoint::Image` (guarded, remove image title elms)
/// - GET `EndPoint::ReportRawTemplateTypeId` (ReportPreview, guarded, remove pdf btns)
#[derive(Default)]
pub struct ImageCpn {
    is_editable: bool,
    id: usize,
    patient: Mutable<Option<Rc<PatientInfo>>>,
    vnan: Option<String>,
    pdf_title: String,
    mechanic: ImageCpnMechanic,
    select_mode: Mutable<bool>,
    loaded: Mutable<bool>,
    is_edit_title: Mutable<bool>,

    viewer: Mutable<Option<Rc<Viewer>>>,
    viewer_width: Mutable<i32>,

    image_paths: MutableVec<Rc<ImagePath>>,
    images_redraw: Mutable<bool>,

    selected: Mutable<Vec<Rc<ImagePath>>>,
    edited: Mutable<Option<ImagePath>>,
    old_title: Mutable<String>,
    edited_title: Mutable<String>,

    is_delete_ids: Mutable<bool>,
    delete_ids: Mutable<Option<Vec<u32>>>,

    devices_loaded: Mutable<bool>,
    media_devices_info: MutableVec<MediaDeviceInfo>,
    selected_info: Mutable<Option<MediaDeviceInfo>>,

    show_capture_modal: Mutable<bool>,
    file_list: Mutable<Vec<File>>,
}

impl ImageCpn {
    /// pdf report title (vnan-title) will use patient.vnan()/page.vnan and pdf_title if provided
    pub fn new_with_key(usage_id: ImageUsage, usage_key_id: u32, is_editable: bool, patient: Mutable<Option<Rc<PatientInfo>>>, vnan: Option<String>, pdf_title: &str) -> Rc<Self> {
        Rc::new(Self {
            is_editable,
            id: IMAGE_CPN_ID.fetch_add(1, Ordering::SeqCst),
            patient,
            vnan,
            pdf_title: pdf_title.to_owned(),
            mechanic: ImageCpnMechanic::WithKey(usage_id, usage_key_id),
            ..Default::default()
        })
    }

    /// pdf report title (vnan-title) will use patient.vnan()/page.vnan and pdf_title if provided<br>
    /// parent MUST provides (ImageUsage + key) and call ImagePath::post_images() later<br>
    /// POST `EndPoint::ImageUsage`
    pub fn new_returning(callback: Mutable<ImagePaths>, patient: Mutable<Option<Rc<PatientInfo>>>, vnan: Option<String>, pdf_title: &str) -> Rc<Self> {
        Rc::new(Self {
            is_editable: true,
            id: IMAGE_CPN_ID.fetch_add(1, Ordering::SeqCst),
            patient,
            vnan,
            pdf_title: pdf_title.to_owned(),
            mechanic: ImageCpnMechanic::ReturnImages(callback.clone()),
            ..Default::default()
        })
    }

    /// pdf report title will be "CACHED-REPORT"
    pub fn new_using_local_storage() -> Rc<Self> {
        Rc::new(Self {
            is_editable: true,
            id: IMAGE_CPN_ID.fetch_add(1, Ordering::SeqCst),
            patient: Mutable::new(None),
            vnan: None,
            pdf_title: String::from("CACHED-REPORT"),
            mechanic: ImageCpnMechanic::LocalStorage,
            ..Default::default()
        })
    }

    pub fn is_empty_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.image_paths.signal_vec_cloned().is_empty()
    }

    fn has_selected_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.selected.signal_cloned().map(|v| !v.is_empty())
    }

    fn is_with_key(&self) -> bool {
        matches!(self.mechanic, ImageCpnMechanic::WithKey(_, _))
    }

    fn title_id(&self) -> String {
        ["image-title-", &self.id.to_string()].concat()
    }

    fn viewer_id(&self) -> String {
        ["images-list-", &self.id.to_string()].concat()
    }

    fn set_viewer_width(&self, app: Rc<App>) {
        let width = match app.get_id(&self.viewer_id()) {
            Some(element) => element.client_width(),
            None => 0,
        };
        self.viewer_width.set_neq(width);
    }

    fn viewer_render(page: Rc<Self>, app: Rc<App>) {
        match page.viewer.get_cloned() {
            Some(viewer) => {
                viewer.update();
            }
            None => {
                if let Some(elm) = app.get_id(&page.viewer_id()) {
                    let viewer = Viewer::new_with_data_url(&elm);
                    page.viewer.set(Some(Rc::new(viewer)));
                }
            }
        }
    }

    fn viewer_destroy(&self) {
        if let Some(viewer) = self.viewer.get_cloned() {
            viewer.destroy();
            self.viewer.set(None);
        }
    }

    async fn get_and_set_image_usages(&self, app: Rc<App>) {
        match &self.mechanic {
            // GET `EndPoint::ImageUsageId`
            ImageCpnMechanic::WithKey(usage_id, usage_key_id) => match ImagePath::call_api_get(usage_id, *usage_key_id, app.state()).await {
                Ok(image_paths) => self.set_image_usages(&image_paths.into_iter().map(Rc::new).collect::<Vec<Rc<ImagePath>>>()),
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            },
            ImageCpnMechanic::LocalStorage => {
                let lock = app.uploaded_images.lock_ref();
                self.set_image_usages(&lock);
            }
            ImageCpnMechanic::ReturnImages(_) | ImageCpnMechanic::Nothing => {}
        }
    }

    /// replace all images with new images
    fn set_image_usages(&self, image_paths: &[Rc<ImagePath>]) {
        let mut lock = self.image_paths.lock_mut();
        lock.replace_cloned(image_paths.to_vec());
    }

    fn delete_image_usages(page: Rc<Self>, app: Rc<App>) {
        if let Some(ids) = page.delete_ids.get_cloned() {
            app.loader_load(clone!(app, page => async move {
                // DELETE `EndPoint::ImageUsage`
                match ImagePath::call_api_delete(&ids, app.state()).await {
                    Ok(_response) => {
                        // if response.rows_affected > 0 {
                        //     app.alert("บันทึกข้อมูลสำเร็จ");
                        // }
                        // change to view mode, need to redraw images
                        page.get_and_set_image_usages(app).await;
                        page.images_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }));
        }
    }

    fn edit_title(page: Rc<Self>, app: Rc<App>) {
        if let Some(mut edited) = page.edited.get_cloned() {
            edited.title = str_some(page.edited_title.get_cloned());
            match &page.mechanic {
                ImageCpnMechanic::WithKey(_, _) => {
                    app.loader_load(clone!(app, page => async move {
                        // PATCH `EndPoint::Image`
                        match edited.call_api_patch(app.state()).await {
                            Ok(_response) => {
                                // if response.rows_affected > 0 {
                                //     app.alert("บันทึกข้อมูลสำเร็จ");
                                // }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                        // still in select mode, no redraw images
                        page.get_and_set_image_usages(app).await;
                    }));
                }
                ImageCpnMechanic::ReturnImages(callback) => {
                    let mut lock = callback.lock_mut();
                    lock.replace(edited);
                    let image_paths = lock.to_vec_rc();
                    page.set_image_usages(&image_paths);
                }
                ImageCpnMechanic::LocalStorage => {
                    // scope for dropping lock
                    {
                        let mut lock = app.uploaded_images.lock_mut();
                        if let Some(pos) = lock.iter().position(|im| **im == edited) {
                            lock.set_cloned(pos, Rc::new(edited));
                        }
                        page.set_image_usages(&lock);
                    }
                    app.to_local_storage();
                }
                ImageCpnMechanic::Nothing => {}
            }
            page.edited.set(None);
        }
    }

    /// header height=48px, title input height=36px, the rest is images_max_height<br>
    /// each thumbnail has 128px height (130px with margin)
    pub fn render(images_max_height: &'static str, page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_pre_admit = page
            .patient
            .lock_ref()
            .as_ref()
            .map(|pt| pt.visit_type.is_pre_admit())
            .or(page.vnan.as_ref().map(|vnan| app.is_pre_admit(vnan)))
            .unwrap_or_default();
        let allow_get_usage_id = app.endpoint_is_allow(&Method::GET, &EndPoint::ImageUsageId, is_pre_admit);
        let allow_post_image = app.endpoint_is_allow(&Method::POST, &EndPoint::Image, is_pre_admit);
        let allow_patch_image = app.endpoint_is_allow(&Method::PATCH, &EndPoint::Image, is_pre_admit);
        let allow_post_usage = app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, is_pre_admit);
        let allow_delete_usage = app.endpoint_is_allow(&Method::DELETE, &EndPoint::ImageUsage, is_pre_admit);

        html!("div", {
            .apply_if(if page.is_with_key() {allow_get_usage_id} else {true}, |dom| dom
                .future(map_ref! {
                    let busy = app.loader_is_loading(),
                    let loaded = page.loaded.signal() =>
                    !busy && !loaded
                }.for_each(clone!(app, page => move |ready| {
                    if ready {
                        page.loaded.set(true);
                        app.loader_load(clone!(app, page => async move {
                            // this function do nothing in callback mode
                            page.get_and_set_image_usages(app.clone()).await;
                            page.images_redraw.set(true);
                        }))
                    }
                    async {}
                })))
            )
            .future(map_ref! {
                let busy = app.loader_is_loading(),
                let edit = page.is_edit_title.signal() =>
                !busy && *edit
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    page.is_edit_title.set(false);
                    Self::edit_title(page.clone(), app.clone());
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let delete = page.is_delete_ids.signal() =>
                !busy && *delete
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    page.is_delete_ids.set(false);
                    Self::delete_image_usages(page.clone(), app.clone());
                }
                async {}
            })))
            .future(page.images_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    page.images_redraw.set(false);
                    page.set_viewer_width(app.clone());
                    Self::viewer_render(page.clone(), app.clone());
                }
                async {}
            })))
            .future(window_size().dedupe().for_each(clone!(app, page => move |ws| {
                if ws.width > 0.0 {
                    page.set_viewer_width(app.clone());
                }
                async {}
            })))
            .apply(|dom| {
                if allow_get_usage_id {
                    if page.is_editable {
                        dom
                    } else {
                        dom.visible_signal(not(page.is_empty_signal()))
                    }
                } else {
                    dom.visible(false)
                }
            })
            .class("card")
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BB0_P2)
                    .style("min-height","47px")
                    .child(html!("div", {
                        .class(class::ABSOLUTE_RT)
                        .class("overflow-hidden")
                        .style("width", "80px")
                        .style("pointer-events", "none")
                        .style("height","47px")
                        .child(html!("i", {
                            .class(class::FA_IMAGE)
                            .class(class::ABSOLUTE_RT)
                            // select mode never empty
                            .class_signal("text-danger", page.select_mode.signal())
                            .class_signal("text-success", map_ref!{
                                let is_select = page.select_mode.signal(),
                                let is_empty = page.is_empty_signal() =>
                                !is_select && !is_empty
                            })
                            .style("font-size","70px")
                            .style("opacity","0.3")
                            .style("rotate","-15deg")
                            .class_signal("icon-sway", app.loader_is_loading())
                        }))
                    }))
                    .child_signal(page.select_mode.signal_cloned().map(clone!(app, page => move |select_mode| {
                        (!select_mode).then(|| {
                            html!("div", {
                                .apply_if(page.is_editable && allow_post_image && if page.is_with_key() {allow_post_usage} else {true}, |dom| { dom
                                    .children([
                                        html!("label", {
                                            .class(class::BTN_SM_L_BLUE)
                                            .style_signal("opacity", app.loader_is_loading().map(|loading| {
                                                if loading {"0.7"} else {"1"}
                                            }))
                                            .text("เพิ่มรูป")
                                            .child(html!("input" => HtmlInputElement, {
                                                .attr("type", "file")
                                                .attr("accept","image/*,.pdf")
                                                .attr("capture","environment")
                                                .attr("multiple","")
                                                .visible(false)
                                                .apply(mixins::other_true_signal_disable(app.loader_is_loading()))
                                                .apply(Self::file_action_mixin(page.clone(), app.clone()))
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type","button")
                                            .class(class::BTN_SM_L_BLUE)
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", &["#camera-capture-modal-", &page.id.to_string()].concat())
                                            .style_signal("opacity", app.loader_is_loading().map(|loading| {
                                                if loading {"0.7"} else {"1"}
                                            }))
                                            .child(html!("i", {.class(class::FA_CAMERA)}))
                                            .event(clone!(page => move |_:events::Click| {
                                                page.show_capture_modal.set_neq(true);
                                            }))
                                        }),
                                        html!("div", {
                                            .class("modal")
                                            .attr("id", &["camera-capture-modal-", &page.id.to_string()].concat())
                                            .attr("role", "dialog")
                                            .attr("tabindex", "-1")
                                            .child(html!("div", {
                                                .class(class::MODAL_DIALOG_LG)
                                                .attr("role", "document")
                                                .child_signal(page.show_capture_modal.signal().map(clone!(page, app => move |is_show| {
                                                    is_show.then(|| {
                                                        Self::render_capture_modal(page.clone(), app.clone())
                                                    })
                                                })))
                                            }))
                                        }),
                                    ])
                                })
                                .child_signal(page.is_empty_signal().map(clone!(app, page => move |is_empty| {
                                    (!is_empty).then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_L_BLUE)
                                            .text("เลือก")
                                            .apply(mixins::other_true_signal_disable(app.loader_is_loading()))
                                            .event(clone!(page => move |_: events::Click| {
                                                page.viewer_destroy();
                                                page.select_mode.set(true);
                                            }))
                                        })
                                    })
                                })))
                                .child_signal(app.clipboard_images.signal_vec_cloned().is_empty().map(clone!(app, page => move |is_empty| {
                                    (page.is_editable && !is_empty && allow_post_usage).then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_BLUE)
                                            .text("วาง")
                                            .apply(mixins::other_true_signal_disable(app.loader_is_loading()))
                                            .apply(|dom| {
                                                let post_mut = Mutable::new(None);
                                                dom.event(clone!(app, page, post_mut => move |_: events::Click| {
                                                    // save new of local_storage
                                                    let new_of_local_storage = find_new_inserting_image(&app.uploaded_images, &app.clipboard_images);
                                                    {
                                                        let mut lock = app.uploaded_images.lock_mut();
                                                        lock.reverse();
                                                        lock.extend(new_of_local_storage);
                                                        lock.reverse();
                                                    }
                                                    app.to_local_storage();
                                                    // prevent duplicate image_data
                                                    let pasted = find_new_inserting_image(&page.image_paths, &app.clipboard_images);
                                                    let images = pasted.iter().map(|im| im.as_ref().to_owned()).collect::<Vec<ImagePath>>();
                                                    // update
                                                    if !images.is_empty() {
                                                        let image_path_s = ImagePaths::from_vec(images);
                                                        match page.mechanic.clone() {
                                                            ImageCpnMechanic::WithKey(usage_id, usage_key_id) => {
                                                                post_mut.set(Some((image_path_s, usage_id, usage_key_id)));
                                                            }
                                                            ImageCpnMechanic::ReturnImages(callback) => {
                                                                let mut lock = callback.lock_mut();
                                                                lock.extend(image_path_s);
                                                                let image_paths = lock.to_vec_rc();
                                                                page.set_image_usages(&image_paths);
                                                                page.images_redraw.set(true);
                                                            }
                                                            ImageCpnMechanic::LocalStorage => {
                                                                let mut lock = page.image_paths.lock_mut();
                                                                lock.extend(pasted);
                                                                page.images_redraw.set(true);
                                                            }
                                                            ImageCpnMechanic::Nothing => {}
                                                        }
                                                    }
                                                }))
                                                .future(map_ref!{
                                                    let busy = app.loader_is_loading(),
                                                    let opt = post_mut.signal_cloned() =>
                                                    (*busy, opt.clone())
                                                }.for_each(clone!(app, page, post_mut => move |(busy, triple_opt)| {
                                                    if !busy {
                                                        if let Some((image_path_s, usage_id, usage_key_id)) = triple_opt {
                                                            post_mut.set(None);
                                                            app.loader_load(clone!(app, page, image_path_s => async move {
                                                                image_path_s.post_images(usage_id, usage_key_id, app).await;
                                                                page.loaded.set(false);
                                                            }));
                                                        }
                                                    }
                                                    async {}
                                                })))
                                            })
                                        })
                                    })
                                })))
                            })
                        })
                    })))
                    .child_signal(page.select_mode.signal_cloned().map(clone!(app, page => move |select_mode| {
                        select_mode.then(|| {
                            html!("div", {
                                .child_signal(page.has_selected_signal().map(clone!(app, page => move |has_selected| {
                                    (has_selected).then(|| {
                                        let modal_id = ["reportModal-img-",&page.id.to_string()].concat();
                                        let report_modal: Mutable<Option<Rc<ReportPreview>>> = Mutable::new(None);
                                        html!("span", {
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_L_BLUE)
                                                .text("สำเนา")
                                                .event(clone!(app, page => move |_: events::Click| {
                                                    page.select_mode.set(false);
                                                    page.edited.set_neq(None);
                                                    let mut selected_lock = page.selected.lock_mut();
                                                    if !selected_lock.is_empty() {
                                                        if matches!(page.mechanic, ImageCpnMechanic::LocalStorage) {
                                                            selected_lock.reverse();
                                                        }
                                                        let mut clipboard_lock = app.clipboard_images.lock_mut();
                                                        clipboard_lock.clear();
                                                        clipboard_lock.extend(selected_lock.drain(0..));
                                                    }
                                                    Self::viewer_render(page.clone(), app.clone());
                                                }))
                                            }))
                                            .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false), |dom| dom
                                                .children([
                                                    Self::pdf_btn(1, &modal_id, report_modal.clone(), page.clone(), app.clone()),
                                                    Self::pdf_btn(2, &modal_id, report_modal.clone(), page.clone(), app.clone()),
                                                    Self::pdf_btn(4, &modal_id, report_modal.clone(), page.clone(), app.clone()),
                                                ])
                                                .child(html!("div", {
                                                    .class("modal")
                                                    .attr("id", &modal_id)
                                                    .attr("role", "dialog")
                                                    .attr("tabindex", "-1")
                                                    .child_signal(report_modal.signal_cloned().map(clone!(app => move |opt| {
                                                        opt.as_ref().map(clone!(app => move |modal| {
                                                            ReportPreview::render(modal.clone(), app)
                                                        })).or(Some(blank_modal()))
                                                    })))
                                                }))
                                            )
                                            .apply_if(page.is_editable && if page.is_with_key() {allow_delete_usage} else {true}, |dom| { dom
                                                .child(html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_L_RED)
                                                    .text("ลบ")
                                                    .visible_signal(page.selected.signal_cloned().map(clone!(app => move |selected| selected.iter().all(|i| app.user_name() == i.create_username))))
                                                    .apply(mixins::other_true_signal_disable(app.loader_is_loading()))
                                                    .apply(|dom| {
                                                        dom.event(clone!(app, page => move |_: events::Click| {
                                                            page.select_mode.set(false);
                                                            page.edited.set_neq(None);
                                                            match &page.mechanic {
                                                                ImageCpnMechanic::WithKey(_, _) => {
                                                                    let image_usage_ids;
                                                                    {
                                                                        let mut selected_lock = page.selected.lock_mut();
                                                                        image_usage_ids = selected_lock.iter().filter_map(|image| image.image_usage_id).collect::<Vec<u32>>();
                                                                        selected_lock.clear();
                                                                    }
                                                                    if !image_usage_ids.is_empty() {
                                                                        page.delete_ids.set(Some(image_usage_ids));
                                                                        page.is_delete_ids.set(true);
                                                                    } else {
                                                                        Self::viewer_render(page.clone(), app.clone());
                                                                    }
                                                                }
                                                                ImageCpnMechanic::ReturnImages(callback) => {
                                                                    let image_ids;
                                                                    {
                                                                        let mut selected_lock = page.selected.lock_mut();
                                                                        image_ids = selected_lock.iter().map(|image| image.image_id).collect::<Vec<u32>>();
                                                                        selected_lock.clear();
                                                                    }
                                                                    if !image_ids.is_empty() {
                                                                        let mut lock = callback.lock_mut();
                                                                        lock.remove(&image_ids);
                                                                        let image_paths = lock.to_vec_rc();
                                                                        page.set_image_usages(&image_paths);
                                                                    }
                                                                }
                                                                ImageCpnMechanic::LocalStorage => {
                                                                    let image_ids;
                                                                    {
                                                                        let mut selected_lock = page.selected.lock_mut();
                                                                        image_ids = selected_lock.iter().map(|image| image.image_id).collect::<Vec<u32>>();
                                                                        selected_lock.clear();
                                                                    }
                                                                    if !image_ids.is_empty() {
                                                                        {
                                                                            let mut lock = app.uploaded_images.lock_mut();
                                                                            lock.retain(|im| !image_ids.contains(&im.image_id));
                                                                            let image_paths = lock.to_vec();
                                                                            page.set_image_usages(&image_paths);
                                                                        }
                                                                        app.to_local_storage();
                                                                    }
                                                                }
                                                                ImageCpnMechanic::Nothing => {}
                                                            }
                                                        }))
                                                    })
                                                }))
                                            })
                                        })
                                    })
                                })))
                                .child_signal(page.has_selected_signal().map(clone!(page => move |has_selected| {
                                    (!has_selected).then(|| {
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_L_BLUE)
                                            .text("ทั้งหมด")
                                            .event(clone!(page => move |_: events::Click| {
                                                let mut lock = page.selected.lock_mut();
                                                lock.clear();
                                                lock.extend(page.image_paths.lock_ref().to_vec());
                                                let is_local_storage_mode = matches!(page.mechanic, ImageCpnMechanic::LocalStorage);
                                                if is_local_storage_mode {
                                                    lock.reverse();
                                                }
                                                if lock.len() == 1 && !is_local_storage_mode {
                                                    page.edited.set(lock.first().map(|im| im.as_ref().to_owned()));
                                                } else {
                                                    page.edited.set_neq(None);
                                                }
                                            }))
                                        })
                                    })
                                })))
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_L_GRAY)
                                    .text("ยกเลิก")
                                    .event(clone!(app, page => move |_: events::Click| {
                                        page.edited.set_neq(None);
                                        page.select_mode.set(false);
                                        page.selected.lock_mut().clear();
                                        Self::viewer_render(page.clone(), app.clone());
                                    }))
                                }))
                            })
                        })
                    })))
                }),
                html!("div", {
                    .class(class::CARD_BODY_P0)
                    .apply_if(page.is_editable && if page.is_with_key() {allow_patch_image} else {true}, |dom| { dom
                        .child(html!("div", {
                            .visible_signal(page.edited.signal_cloned().map(|opt| opt.is_some()))
                            .class(class::INPUT_GROUP_SM)
                            .child_signal(page.viewer_width.signal_cloned().map(|viewer_width| {
                                (viewer_width > 333).then(|| {
                                    html!("span", {
                                        .class(class::INPUT_GROUP_TEXT_SQ_BS0)
                                        .text("ข้อความ")
                                    })
                                })
                            }))
                            .children([
                                // prevent implicit submission (https://www.w3.org/TR/html5/sec-forms.html#implicit-submission)
                                // will happen when press `enter` key on single input, se we create hidden 2nd input
                                html!("input", {.visible(false)}),
                                html!("input" => HtmlInputElement, {
                                    .attr("id", &page.title_id())
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .class_signal("rounded-0", page.viewer_width.signal_cloned().map(|viewer_width| viewer_width <= 500))
                                    .prop_signal("value", page.edited.signal_ref(|opt| opt.as_ref().and_then(|image| image.title.clone()).unwrap_or_default()))
                                    .with_node!(element => {
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |e: events::KeyUp| {
                                            if e.key() == "Enter" {
                                                e.prevent_default();
                                                if page.edited_title.lock_ref().as_str() != page.old_title.lock_ref().as_str() {
                                                    page.is_edit_title.set(true);
                                                }
                                                page.selected.lock_mut().clear();
                                            } else {
                                                page.edited_title.set_neq(element.value());
                                            }
                                        }))
                                    })
                                }),
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_BLUE)
                                    .child(html!("i", {
                                        .class(class::FA_CHECK)
                                    }))
                                    .apply(mixins::other_true_signal_disable(map_ref!{
                                        let old = page.old_title.signal_cloned(),
                                        let changed = page.edited_title.signal_cloned() =>
                                        old == changed
                                    }))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.is_edit_title.set(true);
                                        page.selected.lock_mut().clear();
                                    }))
                                }),
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RED)
                                    .class("rounded-0")
                                    .child(html!("i", {
                                        .class(class::FA_X)
                                    }))
                                    .apply(mixins::other_true_signal_disable(map_ref!{
                                        let old = page.old_title.signal_cloned(),
                                        let changed = page.edited_title.signal_cloned() =>
                                        old.is_empty() || changed.is_empty()
                                    }))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.edited_title.set(String::new());
                                        page.is_edit_title.set(true);
                                        page.selected.lock_mut().clear();
                                    }))
                                }),
                            ])
                        }))
                    })
                    .child(html!("div", {
                        .child(html!("div", {
                            .style("overflow-y","auto")
                            .style("max-height", images_max_height)
                            .child(html!("ul", {
                                .attr("id", &page.viewer_id())
                                .class(class::FLEX_WRAP)
                                .class(class::M0_P0)
                                .style("background-color","rgba(128,128,128,.3)")
                                .style("list-style-type","none")
                                .children_signal_vec(page.image_paths.signal_vec_cloned().map(clone!(app, page => move |image_data| {
                                    html!("li", {
                                        .class("position-relative")
                                        .style("margin","1px")
                                        .style("flex-grow","1")
                                        .child(html!("img", {
                                            .class("w-100")
                                            .style_signal("cursor", page.select_mode.signal_cloned().map(|is_select| {
                                                if is_select {
                                                    "pointer"
                                                } else {
                                                    "zoom-in"
                                                }
                                            }))
                                            .attr("data-original", &["images/", &image_data.path].concat())
                                            .attr("src", &["thumbs/", &image_data.path].concat())
                                            .attr("alt", &[
                                                &image_data.title.clone().unwrap_or_default(),
                                                " โดย ", &image_data.create_username.clone().unwrap_or(String::from("-")),
                                                " เวลา ", &datetime_th_relative(&image_data.create_datetime),
                                            ].concat())
                                            .event(clone!(app, page, image_data => move |_:events::Click| {
                                                if page.select_mode.get() {
                                                    let mut lock = page.selected.lock_mut();
                                                    if let Some(pos) = lock.iter().position(|data| *data == image_data) {
                                                        lock.remove(pos);
                                                    } else {
                                                        lock.push(image_data.clone());
                                                    }
                                                    if page.is_editable && lock.len() == 1 && !matches!(page.mechanic, ImageCpnMechanic::LocalStorage) {
                                                        let single_opt = lock.first().map(|im| im.as_ref().to_owned());
                                                        let title = single_opt.as_ref().and_then(|im| im.title.clone()).unwrap_or_default();
                                                        let title_len = title.len() as u32;
                                                        page.edited.set(single_opt);
                                                        page.old_title.set_neq(title.clone());
                                                        page.edited_title.set_neq(title);
                                                        // set focus
                                                        if let Some(elm) = app.get_id(&page.title_id()) {
                                                            Timeout::new(0, clone!(elm, title_len => move || {
                                                                if let Some(input) = elm.dyn_into::<HtmlInputElement>().ok() {
                                                                    if let Err(e) = input.focus() {
                                                                        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                                        log::error!("Error focus element: {}", message);
                                                                    }
                                                                    if let Err(e) = input.set_selection_start(Some(title_len)) {
                                                                        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                                        log::error!("Error set_selection_start: {}", message);
                                                                    }
                                                                    if let Err(e) = input.set_selection_end(Some(title_len)) {
                                                                        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                                        log::error!("Error set_selection_end: {}", message);
                                                                    }
                                                                }
                                                            })).forget();
                                                        }
                                                    } else {
                                                        page.edited.set_neq(None);
                                                    }
                                                }
                                            }))
                                        }))
                                        .child_signal(page.selected.signal_cloned().map(clone!(image_data => move |selected| {
                                            selected.contains(&image_data).then(|| {
                                                html!("i", {
                                                    .class(class::FA_CHECK_CIRCLE_RED)
                                                    .class(class::ABSOLUTE_RT)
                                                    .style("font-size","48px")
                                                    .style("pointer-events","none")
                                                })
                                            })
                                        })))
                                        .child(html!("div", {
                                            .class(class::ABSOLUTE_FB)
                                            .class(class::TXT_C_WHITE)
                                            .style("font-size","12px")
                                            .style("text-overflow","ellipsis")
                                            .style("background-color","rgba(0,0,0,0.7)")
                                            .style("pointer-events","none")
                                            .style("z-index","1")
                                            .apply(|dom| {
                                                if let Some(title) = &image_data.title {
                                                    dom.text(title).child(html!("br"))
                                                } else {
                                                    dom
                                                }
                                            })
                                            .text(&datetime_th_relative(&image_data.create_datetime))
                                        }))
                                    })
                                })))
                                .children_signal_vec(map_ref!{
                                    let data_len = page.image_paths.signal_vec_cloned().len(),
                                    let viewer_width = page.viewer_width.signal() =>
                                    (*data_len, *viewer_width)
                                }.map(|(data_len, viewer_width)| {
                                    let cols = (viewer_width / 130) as usize;
                                    let mut doms = Vec::new();
                                    // prevent % by zero
                                    if cols > 0 {
                                        let remains = data_len % cols;
                                        for _ in 0..(cols - remains) {
                                            doms.push(blank_image());
                                        }
                                    }
                                    doms
                                }).to_signal_vec())
                            }))
                        }))
                    }))
                }),
                html!("div", {
                    .class(class::CARD_FOOT_BT0)
                    .class_signal("p-0", page.is_empty_signal())
                    .class_signal("p-1", not(page.is_empty_signal()))
                }),
            ])
        })
    }

    fn pdf_btn(per_page: u8, modal_id: &str, report_modal: Mutable<Option<Rc<ReportPreview>>>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let pdf_title = match per_page {
            1 => "Full",
            2 => "Half",
            _ => "2-Cols",
        };
        html!("button" => HtmlButtonElement, {
            .attr("type", "button")
            .class(class::BTN_SM_L_BLUE)
            .attr("data-bs-toggle", "modal")
            .attr("data-bs-target", &["#",&modal_id].concat())
            .child(html!("i", {.class(class::FA_FILE_PDF_L)}))
            .text(pdf_title)
            .event(clone!(report_modal, page => move |_: events::Click| {
                let im_paths = page.selected.get_cloned();
                if let Some(patient) = page.patient.get_cloned() {
                    let vnan = patient.visit_type.vnan();
                    let id = [vnan, "||", &per_page.to_string()].concat();
                    let data_json = serde_json::json!({
                        "id": id,
                        "patient": patient,
                        "im_paths": im_paths,
                    }).to_string();
                    report_modal.set(Some(ReportPreview::new(
                        TypstReport::from_system_with_coercion(SystemReport::DocumentImages, &app.state().report_coercions()),
                        vnan.to_owned(),
                        Some(data_json),
                        false,
                        str_some(page.pdf_title.clone()),
                    )));
                } else if let Some(vnan) = &page.vnan {
                    let id = [vnan, "||", &per_page.to_string()].concat();
                    let data_json = serde_json::json!({
                        "id": id,
                        "im_paths": im_paths,
                    }).to_string();
                    report_modal.set(Some(ReportPreview::new(
                        TypstReport::from_system_with_coercion(SystemReport::DocumentImages, &app.state().report_coercions()),
                        vnan.to_owned(),
                        Some(data_json),
                        false,
                        str_some(page.pdf_title.clone()),
                    )));
                }
            }))
        })
    }

    fn file_action_mixin<T>(page: Rc<Self>, app: Rc<App>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
    where
        T: mixins::TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
    {
        #[inline]
        move |dom| {
            let file_input_mut = Mutable::new(None);
            dom.event(clone!(page, file_input_mut => move |e: events::Change| {
                if let Some(file_input) = e.target().and_then(|input| input.dyn_into::<HtmlInputElement>().ok()) {
                    if let Some(files) = file_input.files() {
                        if files.length() > 0 {
                            for i in 0..files.length() {
                                if let Some(file) = files.item(i) {
                                    page.file_list.lock_mut().push(file);
                                }
                            }
                        }
                    }
                    file_input_mut.set(Some(file_input));
                }
            }))
            .future(
                map_ref! {
                    let busy = app.loader_is_loading(),
                    let files = page.file_list.signal_cloned() =>
                    (*busy, files.clone())
                }
                .for_each(clone!(app, page, file_input_mut => move |(busy, files)| {
                    if !busy && !files.is_empty() {
                        if let Some(file_input) = file_input_mut.lock_ref().as_ref() {
                            file_input.set_value("");
                        }
                        app.loader_load(clone!(app, page => async move {
                            // POST `EndPoint::Image`
                            match ImagePath::call_api_post_files_returning(&files, app.state()).await {
                                Ok(response) => {
                                    let image_path_s = ImagePaths::from_vec(response);
                                    // always save to local storage
                                    {
                                        let mut lock = app.uploaded_images.lock_mut();
                                        lock.reverse();
                                        lock.extend(image_path_s.to_vec_rc());
                                        lock.reverse();
                                    }
                                    app.to_local_storage();
                                    // local storage already done above
                                    match &page.mechanic {
                                        ImageCpnMechanic::WithKey(usage_id, usage_key_id) => {
                                            image_path_s.post_images(usage_id.clone(), *usage_key_id, app).await;
                                            page.loaded.set(false);
                                        }
                                        ImageCpnMechanic::ReturnImages(callback) => {
                                            let mut lock = callback.lock_mut();
                                            lock.extend(image_path_s);
                                            let image_paths = lock.to_vec_rc();
                                            page.set_image_usages(&image_paths);
                                            page.images_redraw.set(true);
                                        }
                                        ImageCpnMechanic::LocalStorage => {
                                            page.set_image_usages(&app.uploaded_images.lock_ref().to_vec());
                                            page.images_redraw.set(true);
                                        }
                                        ImageCpnMechanic::Nothing => {}
                                    }
                                    page.file_list.lock_mut().clear();
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                    page.file_list.lock_mut().clear();
                                }
                            }
                        }));
                    }
                    async {}
                })),
            )
        }
    }

    fn render_capture_modal(page: Rc<Self>, app: Rc<App>) -> Dom {
        let capture_button = dominator::class! {
            .pseudo!(":hover", {
                .style("color","pink")
            })
            .style("position","absolute")
            .style("bottom","25px")
            .style("left","calc(50% - 50px)")
            .style("font-size","80px")
            .style("color","white")
            .style("background","transparent")
            .style("border-style","none")
        };

        let capture_video_id = ["capture-video-", &page.id.to_string()].concat();
        let capture_canvas_id = ["capture-canvas-", &page.id.to_string()].concat();

        html!("div", {
            .class("modal-content")
            .children([
                html!("div", {
                    .class("modal-body")
                    .future(page.devices_loaded.signal().for_each(clone!(app, page => move |loaded| {
                        clone!(app, page => async move {
                            if !loaded {
                                match app.window.with(|w| w.navigator().media_devices()) {
                                    Ok(media_devices) => {
                                        init_media_user(&media_devices).await;
                                        match media_devices.enumerate_devices() {
                                            Ok(promise) => {
                                                match JsFuture::from(promise).await {
                                                    Ok(devices_future) => {
                                                        let devices_js = Array::from(&devices_future);
                                                        let media_infos = devices_js.into_iter().map(|dv| MediaDeviceInfo::from(dv)).filter(|dv| matches!(dv.kind(), MediaDeviceKind::Videoinput)).collect::<Vec<MediaDeviceInfo>>();
                                                        page.selected_info.set(media_infos.first().cloned());
                                                        {
                                                            let mut lock = page.media_devices_info.lock_mut();
                                                            lock.clear();
                                                            lock.extend(media_infos);
                                                        }
                                                    }
                                                    Err(e) => {
                                                        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                        app.alert_error_with_closed("Error await numerate_devices: ", &message).await;
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                app.alert_error_with_closed("Error numerate_devices", &message).await;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                        app.alert_error_with_closed("Error media_devices", &message).await;
                                    }
                                }
                                page.devices_loaded.set(true);
                            }
                        })
                    })))
                    .future(page.selected_info.signal_cloned().for_each(clone!(app, capture_video_id => move |media_info_opt| {
                        clone!(app, capture_video_id => async move {
                            if media_info_opt.is_some() {
                                if let Some(video_elm) = app.get_id(&capture_video_id).and_then(|elm| elm.dyn_into::<HtmlVideoElement>().ok()) {
                                    match app.window.with(|w| w.navigator().media_devices()) {
                                        Ok(media_devices) => {
                                            if let Some(media_info) = media_info_opt {
                                                match get_stream(media_info, media_devices).await {
                                                    Ok(stream) => {
                                                        video_elm.set_src_object(Some(&stream));
                                                        match video_elm.play() {
                                                            Ok(promise) => {
                                                                if let Err(e) = JsFuture::from(promise).await {
                                                                    let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                                    app.alert_error_with_closed("Error await play video", &message).await;
                                                                }
                                                            }
                                                            Err(e) => {
                                                                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                                app.alert_error_with_closed("Error play video", &message).await;
                                                            }
                                                        }
                                                    }
                                                    Err(message) => {
                                                        app.alert_error_with_closed("Error get_stream", &message).await;
                                                    }
                                                }
                                            } else if let Err(e) = stop_video(video_elm) {
                                                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                app.alert_error_with_closed("Error stop_video", &message).await;
                                            }
                                        }
                                        Err(e) => {
                                            let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                            app.alert_error_with_closed("Error media_devices", &message).await;
                                        }
                                    }
                                }
                            }
                        })
                    })))
                    .children([
                        html!("div", {
                            .style("position","relative")
                            .style("width", "100%")
                            .children([
                                html!("div", {
                                    .class("mb-2")
                                    .child(html!("i", {
                                        .class(class::FA_SYNC)
                                        .class("me-1")
                                        .style("cursor","pointer")
                                        .event(clone!(page => move |_:events::Click| {
                                            page.devices_loaded.set(false);
                                        }))
                                    }))
                                    .children_signal_vec(page.media_devices_info.signal_vec_cloned().map(clone!(page => move |device_info| {
                                        html!("button", {
                                            .attr("type","button")
                                            .class(class::BTN_SM_R_BLUEO)
                                            .text(&device_info.label())
                                            .event(clone!(page, device_info => move |_:events::Click| {
                                                page.selected_info.set(Some(device_info.clone()));
                                            }))
                                        })
                                    })))
                                }),
                                html!("video" => HtmlVideoElement, {
                                    .attr("id", &capture_video_id)
                                    .attr("crossorigin","anonymous")
                                    .style("width", "100%")
                                    .text("Video stream not available.")
                                    .with_node!(element => {
                                        .after_inserted(clone!(element => move |_| {
                                            let canplay_cs = Closure::<dyn FnMut(_)>::new(clone!(element => move |_: Event| {
                                                let width = element.parent_element().map(|parent| parent.client_width() as u32).unwrap_or(700);
                                                let video_height = element.video_height();
                                                let video_width = element.video_width();
                                                let height = video_height * width / video_width;
                                                element.set_width(width);
                                                element.set_height(height);
                                            }));

                                            element.set_oncanplay(Some(&canplay_cs.as_ref().unchecked_ref()));
                                            canplay_cs.forget();
                                        }))
                                    })
                                }),
                                html!("canvas", {
                                    .attr("id", &capture_canvas_id)
                                    .visible(false)
                                }),
                                html!("button", {
                                    .attr("type","button")
                                    .attr("data-bs-dismiss", "modal")
                                    .class(capture_button)
                                    .child(html!("i", {.class(class::FA_CIRCLE_NOTCH)}))
                                    .event(clone!(page, app, capture_video_id => move|_: events::Click| {
                                        let video_elm_opt = app.get_id(&capture_video_id).and_then(|elm| elm.dyn_into::<HtmlVideoElement>().ok());
                                        let canvas_elm_opt = app.get_id(&capture_canvas_id).and_then(|elm| elm.dyn_into::<HtmlCanvasElement>().ok());
                                        if let (Some(canvas_elm), Some(video_elm)) = (canvas_elm_opt, video_elm_opt) {
                                            let width = video_elm.video_width();
                                            let height = video_elm.video_height();
                                            if width > 0 && height > 0 {
                                                canvas_elm.set_width(width);
                                                canvas_elm.set_height(height);

                                                if let Some(ctx) = canvas_elm.get_context("2d").ok().flatten().and_then(|obj| obj.dyn_into::<CanvasRenderingContext2d>().ok()) {
                                                    // MacOS Safari in Guest mode will prevent writing video to canvas, result in zero-size image in both
                                                    // elm.toDataURL() and elm.toBlob() method below without any error
                                                    if let Err(e) = ctx.draw_image_with_html_video_element_and_dw_and_dh(&video_elm, 0.0, 0.0, width as f64, height as f64) {
                                                        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                        app.alert_error("ไม่สามารถจับภาพได้", &message);
                                                    }

                                                    // // elm.toDataURL() method
                                                    // log::debug!("Before to_data_url_with_type");
                                                    // match canvas_elm.to_data_url_with_type("image/png") {
                                                    //     Ok(data_url) => {
                                                    //         log::debug!("data_url size: {}", data_url.len());
                                                    //         match DataUrl::process(&data_url) {
                                                    //             Ok(url) => {
                                                    //                 match url.decode_to_vec() {
                                                    //                     Ok((data, _fragment)) => {
                                                    //                         log::debug!("data size: {}", data.len());
                                                    //                         match bytes_to_blob(&data, "image/png") {
                                                    //                             Ok(blob) => {
                                                    //                                 // still 0
                                                    //                                 log::debug!("Blob size: {}", blob.size());
                                                    //                                 let options = FilePropertyBag::new();
                                                    //                                 options.set_type("image/png");
                                                    //                                 let arr = Array::new();
                                                    //                                 arr.push(&blob);
                                                    //                                 match File::new_with_blob_sequence_and_options(&arr, "captured.png", &options) {
                                                    //                                     Ok(file) => {
                                                    //                                         log::debug!("File {} size: {}", file.name(), file.size());
                                                    //                                         page.file_list.lock_mut().push(file);
                                                    //                                     }
                                                    //                                     Err(e) => {
                                                    //                                         let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                    //                                         app.alert_error("Error create file from blob", &message);
                                                    //                                     }
                                                    //                                 }
                                                    //                             }
                                                    //                             Err(e) => {
                                                    //                                 let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                    //                                 app.alert_error("Error bytes_to_blob", &message);
                                                    //                             }
                                                    //                         }
                                                    //                     }
                                                    //                     Err(e) => {
                                                    //                         app.alert_error("Error decode_to_vec", &e.to_string());
                                                    //                     }
                                                    //                 }
                                                    //             }
                                                    //             Err(e) => {
                                                    //                 app.alert_error("Error process DataUrl", &e.to_string());
                                                    //             }
                                                    //         }
                                                    //     }
                                                    //     Err(e) => {
                                                    //         let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                    //         app.alert_error("Error to_data_url_with_type: ", &message);
                                                    //     }
                                                    // }

                                                    // elm.toBlob() method
                                                    let upload_cs = Closure::<dyn FnMut(_)>::new(clone!(app, page => move |blob: Blob| {
                                                        // log::debug!("Blob size: {}", blob.size());
                                                        let options = FilePropertyBag::new();
                                                        options.set_type("image/png");
                                                        let arr = Array::new();
                                                        arr.push(&blob);
                                                        match File::new_with_blob_sequence_and_options(&arr, "captured.png", &options) {
                                                            Ok(file) => {
                                                                // log::debug!("File {} size: {}", file.name(), file.size());
                                                                page.file_list.lock_mut().push(file);
                                                            }
                                                            Err(e) => {
                                                                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                                app.alert_error("Error create file from blob", &message);
                                                            }
                                                        }
                                                    }));
                                                    // log::debug!("Canvas element size: {}x{}", canvas_elm.width(), canvas_elm.height());
                                                    if let Err(e) = canvas_elm.to_blob_with_type(&upload_cs.as_ref().unchecked_ref(), "image/png") {
                                                        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                        app.alert_error("Error canvas to blob", &message);
                                                    }
                                                    upload_cs.forget();
                                                }
                                            }
                                            if let Err(e) = stop_video(video_elm) {
                                                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                                app.alert_error("Error stop_video", &message);
                                            }
                                            page.show_capture_modal.set_neq(false);
                                        }
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class("modal-footer")
                    .child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_GRAY)
                        .attr("data-bs-dismiss", "modal")
                        .text("Close")
                        .event(clone!(page => move |_:events::Click| {
                            if let Some(video_elm) = app.get_id(&capture_video_id).and_then(|elm| elm.dyn_into::<HtmlVideoElement>().ok()) {
                                if let Err(e) = stop_video(video_elm) {
                                    let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                                    app.alert_error("Error stop_video", &message);
                                }
                            }
                            page.show_capture_modal.set_neq(false);
                        }))
                    }))
                }),
            ])
        })
    }
}

#[derive(Clone, Default)]
pub struct ImagePaths {
    inner: Vec<ImagePath>,
}

impl ImagePaths {
    fn from_vec(inner: Vec<ImagePath>) -> Self {
        Self { inner }
    }

    fn to_vec_rc(&self) -> Vec<Rc<ImagePath>> {
        self.inner.clone().into_iter().map(Rc::new).collect()
    }

    fn replace(&mut self, item: ImagePath) {
        if let Some(pos) = self.inner.iter().position(|i| *i == item) {
            self.inner[pos] = item;
        }
    }

    fn extend(&mut self, other: Self) {
        self.inner.extend_from_slice(&other.inner);
    }

    fn remove(&mut self, image_ids: &[u32]) {
        self.inner.retain(|im| !image_ids.contains(&im.image_id));
    }

    // using in ImageCpnMechanic::ReturnImages flow
    /// POST `EndPoint::ImageUsage`
    pub async fn post_images(&self, usage_id: ImageUsage, usage_key_id: u32, app: Rc<App>) {
        let mut images = self.inner.clone();
        if !images.is_empty() {
            for im in images.iter_mut() {
                im.usage_id = Some(usage_id.clone());
                im.usage_key_id = Some(usage_key_id);
            }
            // POST `EndPoint::ImageUsage`
            match ImagePath::call_api_post(&images, app.state()).await {
                Ok(_response) => {
                    // if response.rows_affected > 0 {
                    //     app.alert("บันทึกข้อมูลสำเร็จ");
                    // }
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }
    }
}

#[derive(Clone, Default)]
enum ImageCpnMechanic {
    /// known parent key, complete in image component
    WithKey(ImageUsage, u32),
    /// unknown parent key, image component will send back ImagePaths to parent<br>
    /// parent MUST provides (ImageUsage + key) and call ImagePath::post_images() later
    ReturnImages(Mutable<ImagePaths>),
    /// read/store in app's local storage only, sort DESC
    LocalStorage,
    #[default]
    Nothing,
}

fn find_new_inserting_image(recent: &MutableVec<Rc<ImagePath>>, insert: &MutableVec<Rc<ImagePath>>) -> Vec<Rc<ImagePath>> {
    let recent_images = recent.lock_ref();
    let insert_images = insert.lock_ref();
    let mut results = Vec::with_capacity(insert_images.len());
    for image in insert_images.iter() {
        if !recent_images.contains(image) {
            results.push(image.clone());
        }
    }
    results
}

fn blank_image() -> Dom {
    html!("li", {
        .style("flex-grow","1")
        .child(html!("div", {
            .class("w-100")
            // thumb size 128 + margin 1 both end
            .style("min-width","130px")
        }))
    })
}

// we need to call get_user_media_with_constraints with simple constrain {video:true,audio:false} to get user action
// if we skip this process, we will get media_info with NULL device_id and NULL label
async fn init_media_user(media_devices: &MediaDevices) {
    let constraints = MediaStreamConstraints::new();
    constraints.set_video(&JsValue::from(true));
    constraints.set_audio(&JsValue::from(false));
    match media_devices.get_user_media_with_constraints(&constraints) {
        Ok(promise) => {
            let fut = JsFuture::from(promise);
            match fut.await {
                Ok(stream) => {
                    let stream = MediaStream::from(stream);
                    stream.get_tracks().for_each(&mut |tr, _, _| {
                        if let Some(track) = tr.dyn_into::<MediaStreamTrack>().ok() {
                            track.stop();
                        }
                    });
                }
                Err(e) => {
                    let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                    log::error!("Error resolve init_media_user's promise: {}", message);
                }
            }
        }
        Err(e) => {
            let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
            log::error!("Error init_media_user's get_user_media_with_constraints: {}", message);
        }
    }
}

// MUST call `init_media_user` before calling this function
async fn get_stream(media_info: MediaDeviceInfo, media_devices: MediaDevices) -> Result<MediaStream, String> {
    let device_id_constraint = ConstrainDomStringParameters::new();
    device_id_constraint.set_exact(&JsValue::from_str(&media_info.device_id()));
    let track_constraints = MediaTrackConstraints::new();
    track_constraints.set_device_id(&device_id_constraint);
    let constraints = MediaStreamConstraints::new();
    constraints.set_video(&track_constraints);
    constraints.set_audio(&JsValue::from(false));

    let promise = media_devices.get_user_media_with_constraints(&constraints).map_err(|e| {
        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
        ["Error get_user_media_with_constraints: ", &message].concat()
    })?;
    JsFuture::from(promise).await.map(|stream| MediaStream::from(stream)).map_err(|e| {
        let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
        ["Error await get_user_media_with_constraints: ", &message].concat()
    })
}

fn stop_video(elm: HtmlVideoElement) -> Result<(), JsValue> {
    if let Some(stream) = elm.src_object() {
        elm.pause()?;
        stream.get_tracks().for_each(&mut |tr, _, _| {
            if let Some(track) = tr.dyn_into::<MediaStreamTrack>().ok() {
                track.stop();
            }
        });
        elm.set_src_object(None);
    }
    Ok(())
}
