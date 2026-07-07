// ipd-dr-pre-order.php

use dominator::{Dom, EventOptions, clone, events, html, window_size};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement};

use kphis_model::{
    LEFT_PANEL_MIN_WIDTH, SCREEN_WIDTH_EXTRA,
    endpoint::EndPoint,
    fetch::Method,
    pre_order::master::{PreOrderMaster, PreOrderMasterParams, PreOrderMasterSave},
    report::SystemReport,
    route::Route,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_component::{
    emr::EmrCpn,
    gadget::{aside_resizer::AsideResizerCpn, searchbox::patient::PatientSearchboxCpn},
    ipd_pre_order::IpdPreOrderCpn,
    lab::LabCpn,
    modal::{
        blank_modal,
        pre_order_preview::ToOrderType,
        pre_order_select::{PreOrderSelect, PreOrderType},
    },
};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::date_8601,
    error::CONTACT_ADMIN,
    util::{pre_order_type_display, str_some, zero_none},
};

/// - GET `EndPoint::IpdPreOrderMaster`
/// - GET `EndPoint::SearchBoxPatientText` (PatientSearchboxCpn)
/// - POST `EndPoint::IpdPreOrderMaster` (guarded, remove 'บันทึกการเปลี่ยนแปลงข้อมูลส่วนนี้' btn)
/// - DELETE `EndPoint::IpdPreOrderMasterId` (guarded, remove 'ลบใบ Order' btn)
/// - GET `EndPoint::IpdPreOrderMaster` (PreOrderSelect, guarded, remove 'Template' btn)
/// - GET `EndPoint::IpdPreOrderOrder` (IpdPreOrderCpn/PreOrderSelect, guarded, remove 'Order' tab and 'Template' btn)
/// - GET `EndPoint::IpdPreOrderProgressNote` (IpdPreOrderCpn, guarded, remove 'Order' tab)
/// - GET `EndPoint::LabHead` (LabCpn, guarded, remove 'Lab' tab)
/// - GET `EndPoint::EmrDateHn` (EmrCpn, guarded, remove 'EMR' tab)
/// - GET `EndPoint::EmrVisitVn` (EmrCpn, guarded, remove 'EMR' tab)
#[derive(Clone, Default)]
pub struct IpdPreOrderPage {
    view_by: Mutable<String>,

    loaded_master: Mutable<bool>,
    active_tab: Mutable<Tab>,

    hn: Mutable<String>,
    ptname: Mutable<String>,

    pre_order_master_id: Mutable<u32>,
    pre_order_master_used: Mutable<String>,
    order_doctor: Mutable<String>,
    order_doctor_name: Mutable<String>,
    pre_order_type: Mutable<String>,
    pre_order_type_display: Mutable<String>,
    order_for_date: Mutable<String>,
    template_name: Mutable<String>,
    shared_template: Mutable<String>,

    changed: Mutable<bool>,
    display_patient_searchbox: Mutable<bool>,
    pre_order_select_modal: Mutable<Option<Rc<PreOrderSelect>>>,

    tab_order_loaded: Mutable<Option<Mutable<bool>>>,
}

impl IpdPreOrderPage {
    pub fn new(view_by: &str, pre_order_master_id: u32) -> Rc<Self> {
        Rc::new(Self {
            view_by: Mutable::new(view_by.to_owned()),
            pre_order_master_id: Mutable::new(pre_order_master_id),
            ..Default::default()
        })
    }

    pub fn is_template(&self) -> impl Signal<Item = bool> + use<> {
        self.pre_order_type.signal_cloned().map(|ot| ot.as_str() == "template")
    }

    pub fn is_used(&self) -> impl Signal<Item = bool> + use<> {
        self.pre_order_master_used.signal_cloned().map(|ot| ot.as_str() == "Y")
    }

    fn load_master(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let pre_order_master_id = page.pre_order_master_id.get();
                if pre_order_master_id > 0 {
                    let params = PreOrderMasterParams {
                        pre_order_master_id: Some(pre_order_master_id),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdPreOrderMaster`
                    match PreOrderMaster::call_api_get(&params, app.state()).await {
                        Ok(response) => {
                            if let Some(pre_order) = response.first().cloned() {
                                let hn = pre_order.hn.unwrap_or_default();

                                page.pre_order_type_display.set(pre_order_type_display(&pre_order.pre_order_type).to_owned());
                                page.pre_order_type.set(pre_order.pre_order_type);
                                page.order_doctor.set(pre_order.order_doctor.clone());
                                page.order_doctor_name.set([if pre_order.order_doctor_is_intern.unwrap_or_default() {"(Intern) "} else {""}, &pre_order.order_doctor_name.unwrap_or_default()].concat());
                                page.pre_order_master_used.set(pre_order.used.unwrap_or_default());
                                page.hn.set(hn.clone());
                                page.ptname.set([&hn.clone(), " (", &pre_order.fullname.clone().unwrap_or_default(), ")"].concat());
                                page.order_for_date.set(pre_order.order_for_date.map(|date| date.to_string()).unwrap_or_default());
                                page.template_name.set(pre_order.template_name.unwrap_or_default());
                                page.shared_template.set( pre_order.shared_template.unwrap_or_default());

                                page.changed.set_neq(false);
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    pub fn save_master(page: Rc<Self>, app: Rc<App>) {
        let order_doctor = page.order_doctor.get_cloned();
        let save = PreOrderMasterSave {
            pre_order_master_id: zero_none(page.pre_order_master_id.get()),
            pre_order_type: page.pre_order_type.get_cloned(),
            order_date: None,
            order_time: None,
            order_for_date: date_8601(&page.order_for_date.lock_ref()),
            order_for_time: None,
            order_doctor: if order_doctor.is_empty() { app.doctor_code().unwrap_or_default() } else { order_doctor },
            hn: str_some(page.hn.get_cloned()),
            template_name: str_some(page.template_name.get_cloned()),
            shared_template: str_some(page.shared_template.get_cloned()),
            used: None,
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // POST `EndPoint::IpdPreOrderMaster`
                match save.call_api_post(app.state()).await {
                    Ok((_, response)) => {
                        app.alert_execute_response(&response, async move {
                            page.changed.set_neq(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    pub fn delete_master(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if app.confirm("ยืนยันลบใบ Order").await {
                    let pre_order_master_id = page.pre_order_master_id.get();
                    if pre_order_master_id > 0 {
                        // DELETE `EndPoint::IpdPreOrderMasterId`
                        match PreOrderMaster::call_api_delete(pre_order_master_id, app.state()).await {
                            Ok(responses) => {
                                if let Some(res) = responses.iter().find(|res| res.error.is_some()) {
                                    if let Some(error) = &res.error {
                                        if error == "Used" {
                                            app.alert_error_with_closed("รายการนี้มีการใช้งานแล้ว","มีการอ้างอิงรายการนี้ในใบ Order แล้ว ทำให้ไม่สามารถลบได้").await;
                                            Route::IpdPreOrder { view_by: page.view_by.get_cloned(), pre_order_master_id }.hard_redirect();
                                        } else {
                                            app.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
                                        }
                                    }
                                } else {
                                    Route::IpdPreOrderList { view_by: page.view_by.get_cloned() }.hard_redirect();
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }
            }),
        );
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Pre-Order/Template");

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_master.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_master(page.clone(), app.clone());
                    page.loaded_master.set_neq(true);
                }
                async {}
            })))
            .children([
                html!("ul", {
                    .class(class::NAV_PILLS_M3)
                    .children([
                        html!("li", {
                            .class("nav-item")
                            .child(html!("a", {
                                .class(class::NAV_LINK_ACTIVE)
                                .attr("href", "#")
                                .attr("role", "tab")
                                .child(html!("i", {.class(class::FA_L_ARROW)}))
                                .text(" กลับ")
                                .event_with_options(&EventOptions::preventable(), clone!(app, page => move |event: events::Click| {
                                    event.prevent_default();
                                    if app.go_back_else() {
                                        let route = Route::IpdPreOrderList { view_by: page.view_by.get_cloned() };
                                        if route.has_permission(app.state()) {
                                            route.hard_redirect();
                                        } else {
                                            Route::Info.hard_redirect();
                                        }
                                    }
                                }))
                            }))
                        }),
                        html!("li", {
                            .class("nav-item")
                            .child(html!("h4", {
                                .class("nav-link")
                                .text("บันทึก IPD Order ล่วงหน้า / IPD Order Template")
                            }))
                        }),
                    ])
                }),
                Self::render_main(page.clone(), app.clone()),
                html!("hr"),
            ])
            .child_signal(window_size().map(|ws| ws.width < SCREEN_WIDTH_EXTRA).dedupe().map(move |is_not_wide| {
                Some(if is_not_wide {
                    Self::render_content(page.clone(), app.clone())
                } else {
                    let report_selected = SystemReport::new(&app.report_select.lock_ref());
                    AsideResizerCpn::render(
                        Self::render_content(page.clone(), app.clone()),
                        None,
                        AsideResizerCpn::new(
                            Mutable::new(report_selected), Mutable::new(false),
                            Mutable::new(None), Mutable::new(false),
                            Mutable::new(String::new()), page.hn.clone(), Vec::new(),
                            "ipd-pre-order-main", None, None, app.clone(),
                        ),
                        app.clone(),
                    )
                })
            }))
        })
    }

    fn render_main(page: Rc<Self>, app: Rc<App>) -> Dom {
        let can_edit = app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderMaster, false) && app.has_permission(Permission::DataTypeDoctorUse);

        html!("div", {
            .class("container-fluid")
            .child(doms::form_inline(clone!(app, page => move |form| { form
                .children([
                    doms::form_inline_group_sm(clone!(page => move |group| { group
                        .children([
                            doms::label_group_for("order_doctor_name","ผู้บันทึก"),
                            html!("input", {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("id", "order_doctor_name")
                                .attr("readonly", "readonly")
                                .prop_signal("value",page.order_doctor_name.signal_cloned())
                            }),
                        ])
                    })),
                    doms::form_inline_group_sm(clone!(page => move |group| { group
                        .children([
                            doms::label_group_for("pre_order_type_display","ประเภทใบ Order"),
                            html!("input", {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("id", "pre_order_type_display")
                                .attr("readonly", "readonly")
                                .prop_signal("value",page.pre_order_type_display.signal_cloned())
                            }),
                        ])
                    })),
                ])
                .child_signal(page.is_template().map(clone!(page => move |is_template| {
                    is_template.then(|| {
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("template_name","ชื่อ Template"),
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("id", "template_name")
                                    // !(($IPD_ORDER_ADD || $IPD_ORDER_EDIT) && $DATA_TYPE_DOCTOR_USE)
                                    .apply_if(!can_edit, |d| d.attr("readonly",""))
                                    .apply(mixins::string_value(page.template_name.clone(), page.changed.clone()))
                                }),
                            ])
                        }))
                    })
                })))
                .child_signal(page.is_template().map(clone!(page => move |is_template| {
                    is_template.then(|| {
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("pre_order_master_used_display","ใช้งาน"),
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("id", "pre_order_master_used_display")
                                    .attr("readonly", "readonly")
                                    .prop_signal("value", page.pre_order_master_used.signal_cloned().map(|used| {
                                        if used.as_str() == "Y" {"ใช้ได้"} else {"ยังไม่ได้ใช้"}
                                    }))
                                }),
                            ])
                        }))
                    })
                })))
                .apply_if(app.has_permission(Permission::IpdOrderTemplateShare), |can_share| { can_share
                    .child_signal(page.is_template().map(clone!(page => move |is_template| {
                        is_template.then(|| {
                            doms::form_inline_switch(clone!(page => move |check| { check
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "checkbox")
                                        .class("form-check-input")
                                        .attr("role","switch")
                                        .attr("id", "shared_template")
                                        .attr("value", "Y")
                                        .apply(mixins::checkbox_toggle(page.shared_template.clone(), page.changed.clone(), "Y", "N"))
                                    }),
                                    html!("label", {
                                        .attr("for", "shared_template")
                                        .class("form_check_label")
                                        .text("กำหนดเป็น Shared Template")
                                    }),
                                ])
                            }))
                        })
                    })))
                })
                .child_signal(page.is_template().map(clone!(app, page => move |is_template| {
                    (!is_template).then(|| {
                        doms::form_inline_group_sm(clone!(app, page => move |group| { group
                            .attr("id", "hn-input-group")
                            .children([
                                doms::label_group_for("ptname","HN"),
                                html!("input", {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("id", "ptname")
                                    .attr("readonly", "readonly")
                                    .attr("size", "40")
                                    .prop_signal("value",page.ptname.signal_cloned())
                                }),
                                html!("button", {
                                    .visible_signal(page.is_used().map(clone!(app => move |used| {
                                        !used
                                        && (app.has_permission(Permission::IpdOrderAdd) || app.has_permission(Permission::OpdErOrderAdd))
                                        && app.has_permission(Permission::DataTypeDoctorUse)
                                    })))
                                    .class(class::BTN_SM_GRAY)
                                    .attr("type", "button")
                                    .child(html!("i", {.class(class::FA_SEARCH)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        page.display_patient_searchbox.set_neq(true);
                                    }))
                                })
                            ])
                        }))
                    })
                })))
                .child_signal(page.is_template().map(clone!(page => move |is_template| {
                    (!is_template).then(|| {
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("order_for_date","วันที่นัด/Admit"),
                                doms::date_picker(
                                    page.order_for_date.clone(),
                                    page.changed.clone(), page.is_used(), None,
                                    |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "order_for_date"),
                                    |s| s, always(None),
                                ),
                            ])
                        }))
                    })
                })))
                .child(doms::form_inline_end(clone!(app, page => move |end| { end
                    .apply_if(can_edit, |editable| { editable
                        .children([
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_SM_L_BLUE)
                                .child(html!("i", {.class(class::FA_SAVE)}))
                                .text(" บันทึกการเปลี่ยนแปลงข้อมูลส่วนนี้")
                                .visible_signal(map_ref!{
                                    let changed = page.changed.signal(),
                                    let is_used = page.is_used() =>
                                    !is_used && *changed
                                })
                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                    Self::save_master(page.clone(), app.clone());
                                }), app.state()))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_L_GRAY)
                                .child(html!("i", {.class(class::FA_X)}))
                                .text(" ยกเลิก")
                                .visible_signal(map_ref!{
                                    let changed = page.changed.signal(),
                                    let is_used = page.is_used() =>
                                    !is_used && *changed
                                })
                                .event(clone!(page => move |_: events::Click| {
                                    page.loaded_master.set_neq(false);
                                }))
                            }),
                        ])
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderMaster, true)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderOrder, true),
                    |dom| dom
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_SM_L_GRAY)
                            .attr("data-bs-toggle", "modal")
                            .attr("data-bs-target", "#selectPreOrderModal")
                            .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                            .text(" Template")
                            .event(clone!(page => move |_: events::Click| {
                                page.pre_order_select_modal.set(Some(PreOrderSelect::new(PreOrderType::Template, &page.pre_order_master_id.get().to_string(), ToOrderType::PreOrder)));
                            }))
                        }))
                    )
                    .apply_if(
                        app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdPreOrderMasterId, true)
                        && app.has_permission(Permission::DataTypeDoctorUse),
                    |dom| { dom
                        .child_signal(page.pre_order_master_id.signal_cloned().map(clone!(app, page => move |pre_order_master_id| {
                            (pre_order_master_id > 0).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RED)
                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                    .text(" ลบใบ Order")
                                    .visible_signal(not(page.is_used()))
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::delete_master(page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                    })
                })))
            })))
            .child_signal(page.display_patient_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                if show {
                    app.get_id("hn-input-group").map(clone!(app, page => move |elm| {
                        PatientSearchboxCpn::render(
                            PatientSearchboxCpn::new(),
                            page.display_patient_searchbox.clone(),
                            page.hn.clone(),
                            page.ptname.clone(),
                            elm.get_bounding_client_rect(),
                            page.changed.clone(),
                            app,
                        )
                    }))
                } else {
                    None
                }
            })))
        })
    }

    fn render_content(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("container-fluid")
            .attr("id", "ipd-pre-order-main")
            .style("min-width",LEFT_PANEL_MIN_WIDTH)
            .children([
                html!("ul", {
                    .class(class::NAV_PILLS_T)
                    //.attr("id", "pills-tab")
                    .visible_signal(not(page.is_template()))
                    .attr("role","tablist")
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderOrder, true)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPreOrderProgressNote, true),
                    |dom| dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Order)))
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
                                .text("Order")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Order);
                                }))
                            }))
                        }))
                    )
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false), |dom| dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Lab)))
                                .attr("data-bs-toggle","pill")
                                .attr("href","#pills-lab")
                                .text("Lab")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Lab);
                                }))
                            }))
                        }))
                    )
                    .child_signal(page.hn.signal_cloned().map(clone!(app => move |hn| {
                        app.pacs_hn_url(&hn).map(|hn_url| {
                            doms::nav_item_external_url(&hn_url, "X-Ray ")
                        })
                    })))
                    .child_signal(page.hn.signal_cloned().map(clone!(app => move |hn| {
                        app.ekg_hn_url(&hn).map(|hn_url| {
                            doms::nav_item_external_url(&hn_url, "EKG ")
                        })
                    })))
                    .child_signal(page.hn.signal_cloned().map(clone!(app => move |hn| {
                        app.scan_hn_url(&hn).map(|hn_url| {
                            doms::nav_item_external_url(&hn_url, "Scan ")
                        })
                    })))
                    .apply(|dom| {
                        let route = Route::PrescriptionScreen {hn: page.hn.get_cloned()};
                        if route.has_permission(app.state()) {
                            dom.child(html!("li", {
                                .class(class::NAV_ITEM_PY)
                                .child(html!("a", {
                                    .class("nav-link")
                                    .attr("href", "#")
                                    .attr("data-bs-toggle", "pill")
                                    .text("ประวัติการสั่งยา ")
                                    .child(html!("i", {.class(class::FA_DISPLAY)}))
                                    .event_with_options(&EventOptions::preventable(), move |event: events::Click| {
                                        event.prevent_default();
                                        route.hard_redirect();
                                    })
                                }))
                            }))
                        } else {
                            dom
                        }
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::EmrDateHn, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::EmrVisitVn, false),
                    |dom| dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Emr)))
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
                                .text("EMR")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Emr);
                                }))
                            }))
                        }))
                    )
                }),
                html!("div", {
                    .class("tab-content")
                    //.attr("id", "pills-tabContent")
                    .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                        Some(match tab {
                            Tab::Order => {
                                let pre_order = IpdPreOrderCpn::new(page.pre_order_master_id.clone());
                                page.tab_order_loaded.set(Some(pre_order.loaded_order_all.clone()));
                                IpdPreOrderCpn::render(page.pre_order_master_used.clone(), pre_order, app.clone())
                            }
                            Tab::Lab => {
                                let lab = LabCpn::new(
                                    Mutable::new(None),
                                    page.hn.clone(),
                                    Mutable::new(String::new()),
                                    None,
                                );
                                LabCpn::render("ipd-pre-order-main", lab, app.clone())
                            }
                            // Tab::LabCovid,
                            // Tab::XRay => html!("div", {.text("XRay")}),
                            // Tab::Scan => html!("div", {.text("Scan")}),
                            // Tab::Pharm => html!("div", {.text("Pharm")}),
                            Tab::Emr => {
                                let emr = EmrCpn::new(page.hn.clone());
                                EmrCpn::render("ipd-pre-order-main", emr, app.clone())
                            }
                        })
                    })))
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", "selectPreOrderModal")
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.pre_order_select_modal.signal_cloned().map(clone!(app, page => move |opt| {
                        opt.as_ref().map(clone!(app, page => move |modal| {
                            PreOrderSelect::render(modal.clone(), page.pre_order_select_modal.clone(), page.tab_order_loaded.get_cloned(), None, app)
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }
}

#[derive(Clone, Default, PartialEq)]
enum Tab {
    #[default]
    Order,
    Lab,
    // XRay,
    // Scan,
    // Pharm,
    Emr,
}
