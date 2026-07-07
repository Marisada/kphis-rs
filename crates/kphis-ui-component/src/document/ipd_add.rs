// ipd-nurse-document_tab_DocumentAdd.php
use dominator::{Dom, clone, html, link};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;

use kphis_model::{ipd::document::IpdDocumentDatetime, route::Route};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::util::str_some;

use super::concat_to_table_row_link;

/// GET `EndPoint::IpdDocumentDatetimeAn`   
#[derive(Clone, Default)]
pub struct IpdDocumentAddCpn {
    an: Mutable<String>,

    loaded: Mutable<bool>,
    result: Mutable<IpdDocumentDatetime>,
}

impl IpdDocumentAddCpn {
    pub fn new(an: &str) -> Rc<Self> {
        Rc::new(Self {
            an: Mutable::new(an.to_owned()),
            ..Default::default()
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        if let Some(an) = str_some(page.an.get_cloned()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdDocumentDatetimeAn`
                    match IpdDocumentDatetime::call_api_get(&an, app.state()).await {
                        Ok(response) => {
                            page.result.set(response);
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
            .children([
                html!("br"),
                html!("div", {
                    .class("row")
                    .child_signal(page.result.signal_cloned().map(clone!(app, page => move |res| {
                        (res.nurse_admission_note.is_none() || res.dr_admission_note.is_none() || res.summary2.is_none()).then(|| {
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
                                            .apply(|dom| {
                                                let route = Route::IpdAdmissionNoteNurse {an: page.an.get_cloned()};
                                                if res.nurse_admission_note.is_none() && route.has_permission(app.state()) {
                                                    dom.child(html!("li", {
                                                        .child(link!(route.string(), {
                                                            .class("dropdown-item")
                                                            .child(html!("em", {.class(class::FA_FILE)}))
                                                            .text(" การประเมินสภาพผู้ป่วยแรกรับและแบบแผนสุขภาพ (ยกเว้นผู้ป่วยเด็กอายุ < 1 ปี)")
                                                        }))
                                                    }))
                                                } else {
                                                    dom
                                                }
                                            })
                                            .apply(|dom| {
                                                let route = Route::IpdAdmissionNoteDr {an: page.an.get_cloned()};
                                                if res.dr_admission_note.is_none() && route.has_permission(app.state()) {
                                                    dom.child(html!("li", {
                                                        .child(link!(route.string(), {
                                                            .class("dropdown-item")
                                                            .child(html!("em", {.class(class::FA_FILE)}))
                                                            .text_signal(app.hospital_name_signal().map(|hospital_name| [" แบบบันทึกการรับใหม่ผู้ป่วยใน ", &hospital_name].concat()))
                                                        }))
                                                    }))
                                                } else {
                                                    dom
                                                }
                                            })
                                            .apply(|dom| {
                                                let route = Route::Summary {view_by: String::from("doctor"), an: page.an.get_cloned()};
                                                if res.summary2.is_none() && route.has_permission(app.state()) {
                                                    dom.child(html!("li", {
                                                        .child(link!(route.string(), {
                                                            .class("dropdown-item")
                                                            .child(html!("em", {.class(class::FA_FILE)}))
                                                            .text(" SUMMARY FORM")
                                                        }))
                                                    }))
                                                } else {
                                                    dom
                                                }
                                            })
                                        })
                                    ])
                                }))
                            })
                        })
                    })))
                }),
                html!("br"),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class("col-md-12")
                        .children([
                            html!("h5", {
                                .child(html!("em", {.class(class::FA_LIST)}))
                                .text(" รายการเอกสาร")
                            }),
                            html!("table", {
                                .class(class::TABLE_SM)
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .class("table-info-subtle")
                                            .children([
                                                html!("th", {.attr("scope", "col").text("ชื่อเอกสาร")}),
                                                html!("th", {.attr("scope", "col").text("วันที่/เวลา (ที่บันทึก)")}),
                                                html!("th", {.attr("scope", "col").text("วันที่/เวลา (ที่แก้ไขล่าสุด)")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        //.attr("id", "Table_DocumentEdit")
                                        .apply(|dom| {
                                            let route = Route::IpdAdmissionNoteNurse {an: page.an.get_cloned()};
                                            if route.has_permission(app.state()) {
                                                dom.child_signal(page.result.signal_cloned().map(move |res| {
                                                    concat_to_table_row_link(&res.nurse_admission_note, link!(route.string(), {
                                                        .text("การประเมินสภาพผู้ป่วยแรกรับและแบบแผนสุขภาพ (ยกเว้นผู้ป่วยเด็กอายุ < 1 ปี)")
                                                    }))
                                                }))
                                            } else {
                                                dom
                                            }
                                        })
                                        .apply(|dom| {
                                            let route = Route::IpdAdmissionNoteDr {an: page.an.get_cloned()};
                                            if route.has_permission(app.state()) {
                                                dom.child_signal(page.result.signal_cloned().map(clone!(app => move |res| {
                                                    concat_to_table_row_link(&res.dr_admission_note, link!(route.string(), {
                                                        .text_signal(app.hospital_name_signal().map(|hospital_name| ["แบบบันทึกการรับใหม่ผู้ป่วยใน ", &hospital_name].concat()))
                                                    }))
                                                })))
                                            } else {
                                                dom
                                            }
                                        })
                                        .apply(|dom| {
                                            let route = Route::Summary {view_by: String::from("doctor"), an: page.an.get_cloned()};
                                            if route.has_permission(app.state()) {
                                                dom.child_signal(page.result.signal_cloned().map(move |res| {
                                                    concat_to_table_row_link(&res.summary2, link!(route.string(), {
                                                        .text("SUMMARY FORM")
                                                    }))
                                                }))
                                            } else {
                                                dom
                                            }
                                        })
                                    }),
                                ])
                            }),
                        ])
                    }))
                }),
                html!("br"),
            ])
        })
    }
}
