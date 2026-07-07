use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use strum::IntoEnumIterator;
use web_sys::HtmlButtonElement;

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::{DocumentType, ImageUsage},
    ipd::document::DocumentScan,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, mixins};
use kphis_util::util::{str_some, zero_none};

use crate::gadget::image::ImageCpn;

/// - GET `EndPoint::OpdErDocumentScanId`
/// - POST `EndPoint::OpdErDocumentScanId` (guarded, remove เพิ่มเอกสาร)
/// - DELETE `EndPoint::OpdErDocumentScanId` (guarded, remove delete image btn)
#[derive(Clone, Default)]
pub struct OpdErDocumentScanCpn {
    opd_er_order_master_id: Mutable<u32>,
    vn: String,
    is_editable: bool,

    loaded: Mutable<bool>,
    result: MutableVec<DocumentScan>,
    can_add: MutableVec<DocumentType>,
    add_type: Mutable<Option<DocumentType>>,
    delete_type: Mutable<Option<DocumentType>>,
}

impl OpdErDocumentScanCpn {
    pub fn new(opd_er_order_master_id: u32, vn: &str, is_editable: bool) -> Rc<Self> {
        Rc::new(Self {
            opd_er_order_master_id: Mutable::new(opd_er_order_master_id),
            vn: vn.to_owned(),
            is_editable,
            ..Default::default()
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        if let Some(opd_er_order_master_id) = zero_none(page.opd_er_order_master_id.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::OpdErDocumentScanId`
                    match DocumentScan::call_api_get_opd_er(opd_er_order_master_id, app.state()).await {
                        Ok(responses) => {
                            let mut can_add_lock = page.can_add.lock_mut();
                            can_add_lock.clear();
                            can_add_lock.extend(DocumentType::iter().filter(|ty| !responses.iter().any(|scan| scan.document_type_id == *ty)));

                            let mut result_lock = page.result.lock_mut();
                            result_lock.replace_cloned(responses);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let allow_post = app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErDocumentScanId, false);
        let allow_delete = app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErDocumentScanId, false);

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                    page.loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let add_type_opt = page.add_type.signal_cloned() =>
                (*busy, add_type_opt.clone())
            }.for_each(clone!(app, page => move |(busy, add_type_opt)| {
                if !busy {
                    if let Some(add_type) = add_type_opt {
                        page.add_type.set(None);
                        app.loader_load(clone!(app, page => async move {
                            // POST `EndPoint::OpdErDocumentScanId`
                            match add_type.call_api_post_opd_er(page.opd_er_order_master_id.get(), app.state()).await {
                                Ok(response) => {
                                    app.alert_execute_response(&response, async move {
                                        // app.alert("บันทึกข้อมูลสำเร็จ");
                                        page.loaded.set(false);
                                    }).await;
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }));
                    }
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let delete_type_opt = page.delete_type.signal_cloned() =>
                (*busy, delete_type_opt.clone())
            }.for_each(clone!(app, page => move |(busy, delete_type_opt)| {
                if !busy {
                    if let Some(delete_type) = delete_type_opt {
                        page.delete_type.set(None);
                        app.loader_load(clone!(app, page => async move {
                            // DELETE `EndPoint::OpdErDocumentScanId`
                            match delete_type.call_api_delete_opd_er(page.opd_er_order_master_id.get(), app.state()).await {
                                Ok(response) => {
                                    app.alert_execute_response(&response, async move {
                                        // app.alert("บันทึกข้อมูลสำเร็จ");
                                        page.loaded.set(false);
                                    }).await;
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }));
                    }
                }
                async {}
            })))
            //.style("min-height","700px")
            .apply_if(page.is_editable && allow_post, |dom| { dom
                .children([
                    html!("br"),
                    html!("div", {
                        .class("row")
                        .child_signal(page.can_add.signal_vec_cloned().is_empty().map(clone!(page => move |is_empty| {
                            (!is_empty).then(|| {
                                html!("div", {
                                    .class("col-md-auto")
                                    .child(html!("div", {
                                        .class("dropdown")
                                        .children([
                                            html!("button", {
                                                .class(class::BTN_DROP_TGL_CYAN)
                                                .attr("type", "button")
                                                .attr("data-bs-toggle","dropdown")
                                                .attr("aria-expanded","false")
                                                .child(html!("em", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่มเอกสาร")
                                            }),
                                            html!("ul", {
                                                .class("dropdown-menu")
                                                .children_signal_vec(page.can_add.signal_vec_cloned().map(clone!(page => move |document_type| {
                                                    html!("li", {
                                                        .class("dropdown-item")
                                                        .style("cursor","pointer")
                                                        .text(document_type.label())
                                                        .event(clone!(page => move |_:events::Click| {
                                                            page.add_type.set(Some(document_type.clone()));
                                                        }))
                                                    })
                                                })))
                                            })
                                        ])
                                    }))
                                })
                            })
                        })))
                    }),
                    html!("br"),
                ])
            })
            .children([
                html!("div", {
                    .class(class::FLEX_WRAP_G3)
                    .children_signal_vec(page.result.signal_vec_cloned().filter_map(clone!(app => move |doc| {
                        (page.is_editable || doc.has_image).then(|| {
                            let image_box = ImageCpn::new_with_key(
                                ImageUsage::OpdErDocument,
                                doc.document_id,
                                page.is_editable,
                                Mutable::new(None),
                                str_some(page.vn.clone()),
                                doc.document_type_id.label(),
                            );
                            html!("div", {
                                .style("flex-grow", "1")
                                .style("flex-shrink", "1")
                                .style("flex-basis", "300px")
                                .style("max-width", "450px")
                                .children([
                                    html!("div", {
                                        .class("mb-1")
                                        .child(html!("span", {
                                            .style("font-size", "20px")
                                            .style("vertical-align", "bottom")
                                            .text(doc.document_type_id.label())
                                        }))
                                        .apply_if(allow_delete, |dom| {
                                            dom.child_signal(image_box.is_empty_signal().map(clone!(app, page, doc => move |is_empty| {
                                                is_empty.then(|| {
                                                    html!("button" => HtmlButtonElement, {
                                                        .attr("type", "button")
                                                        .class(class::BTN_SM_R_REDO)
                                                        .child(html!("i", {.class(class::FA_TRASH)}))
                                                        .apply(mixins::click_with_loader_checked(clone!(page, doc => move || {
                                                            page.delete_type.set(Some(doc.document_type_id.clone()));
                                                        }), app.state()))
                                                    })
                                                })
                                            })))
                                        })
                                    }),
                                    ImageCpn::render("170px", image_box, app.clone()),
                                ])
                            })
                        })
                    })))
                }),
                html!("br"),
            ])
        })
    }
}
