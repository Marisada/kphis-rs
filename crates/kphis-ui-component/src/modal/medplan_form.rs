use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    ipd::his::HisMedPlanIpd,
    order::{MedPlanItem, Order, OrderPatchAction},
    search::searchbox::IvfluidSearchbox,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_th_opt_relative, time_8601},
    util::{get_day_dose_from_detail, sanity_dot_space, str_some},
};

use crate::{
    gadget::searchbox::{dec_to_color, med::search_drugusage},
    order::{OffOrderItem, OrderCpn},
};

/// - GET `EndPoint::HisMedPlanIpdAn`
/// - GET `EndPoint::SearchBoxIvfluidText`
/// - PATCH `EndPoint::IpdOrderOrder`
/// - PATCH `EndPoint::OpdErOrderOrder`
pub struct MedPlanForm {
    is_oneday: bool,
    medplans_loaded: Mutable<bool>,
    changed: Mutable<bool>,
    parent: Rc<OrderCpn>,
    order: Rc<Order>,
    order_time_mutable: Mutable<String>,

    // ivfluids
    icode: Mutable<String>,
    ivfluid_name: Mutable<String>,
    order_item_detail: Mutable<String>,
    ivfluid_changed: Mutable<bool>,

    display_ivfluid_searchbox: Mutable<bool>,
    text: Mutable<String>,
    results: MutableVec<Rc<IvfluidSearchbox>>,
    status_text: Mutable<Option<String>>,
}

impl MedPlanForm {
    pub fn new(order: Rc<Order>, parent: Rc<OrderCpn>, order_time_mutable: Mutable<String>, is_oneday: bool) -> Rc<Self> {
        Rc::new(Self {
            is_oneday,
            medplans_loaded: Mutable::new(false),
            parent,
            order,
            order_time_mutable,
            changed: Mutable::new(false),

            icode: Mutable::new(String::new()),
            ivfluid_name: Mutable::new(String::new()),
            order_item_detail: Mutable::new(String::new()),
            ivfluid_changed: Mutable::new(false),

            display_ivfluid_searchbox: Mutable::new(false),
            text: Mutable::new(String::new()),
            results: MutableVec::new(),
            status_text: Mutable::new(Some(String::from("กรอกข้อความเพื่อค้นหา"))),
        })
    }

    fn load_medplan(modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::HisMedPlanIpdAn`
                match HisMedPlanIpd::call_api_get(
                    &modal.order.visit_type.vnan(),
                    app.state(),
                ).await {
                    Ok(responses) => {
                        let mut off_medplans = modal.parent.off_medplans.lock_mut();
                        let mut retain_medplans = modal.parent.retain_medplans.lock_mut();
                        let off_icodes = modal.parent.off_icodes.lock_ref();
                        off_medplans.clear();
                        retain_medplans.clear();
                        for res in responses {
                            if let Some(off_item) = OffOrderItem::from_medplan_ipd(&res) {
                                let is_off = off_icodes.contains(&off_item);
                                if is_off {
                                    off_medplans.push_cloned(OffMedPlanMutable::new_with_off(res, true));
                                } else {
                                    retain_medplans.push_cloned(OffMedPlanMutable::new_with_off(res, false));
                                }
                            } else {
                                retain_medplans.push_cloned(OffMedPlanMutable::new_with_off(res, false));
                            }
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_ivfluids(modal: Rc<Self>, app: Rc<App>) {
        modal.status_text.set_neq(Some(String::from("กำลังค้นหา...")));
        app.async_load(
            true,
            clone!(app, modal => async move {
                let search_text = modal.text.get_cloned();
                if !search_text.is_empty() {
                    // GET `EndPoint::SearchBoxIvfluidText`
                    match IvfluidSearchbox::call_api_get(&search_text, app.state()).await {
                        Ok(results) => {
                            let mut lock = modal.results.lock_mut();
                            if !lock.is_empty() {
                                lock.clear();
                            }
                            if results.is_empty() {
                                modal.status_text.set(Some(String::from("ไม่พบรายการที่ค้นหา")));
                            } else {
                                lock.extend(results.into_iter().map(Rc::new));
                                modal.status_text.set(None);
                            }
                        }
                        Err(e) => {
                            modal.status_text.set(Some(String::from("การเชื่อมต่อขัดข้อง")));
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    fn add_ivfluids(&self) {
        // MedPlanForm is called after checking `is_ipd` and `!visit_type.is_pre_admit()`
        // so `visit_type.vnan()` is AN
        let an = self.parent.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned()).unwrap_or_default();
        let med_plan = MedPlanMutable::new(MedPlanItem {
            order_item_id: 0,
            an,
            order_date: Some(self.order.order_date),
            order_time: Some(self.order.order_time),
            order_type: Some(self.order.order_type.clone()),
            order_item_type: Some(String::from("ivfluid")),
            med_name: str_some(self.ivfluid_name.get_cloned()),
            order_item_detail: str_some(sanity_dot_space(&self.order_item_detail.get_cloned())),
            icode: str_some(self.icode.get_cloned()),
            order_doctor: self.order.order_doctor.clone(),
            stat: None,
            med_reconciliation_item_id: None,
            first_qty: Some(1),
            qty: Some(1),
        });
        {
            let mut lock = self.parent.medplans.lock_mut();
            lock.push_cloned(med_plan);
        }
        self.clear_ivfluid_searchbox();
    }

    fn ivfluid_ready(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let ivfluid_name = self.ivfluid_name.signal_cloned(),
            let order_item_detail = self.order_item_detail.signal_cloned() =>
            !ivfluid_name.is_empty() && !order_item_detail.is_empty()
        }
    }

    fn clear_ivfluid_searchbox(&self) {
        self.ivfluid_name.set_neq(String::new());
        self.order_item_detail.set_neq(String::new());
        self.icode.set_neq(String::new());
        self.text.set_neq(String::new());
        self.results.lock_mut().clear();
        self.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
        self.display_ivfluid_searchbox.set(false);
    }

    pub fn render(modal: Rc<Self>, app: Rc<App>) -> Dom {
        let injections = app.app_status.lock_ref().as_ref().map(|status| status.hosxp_injection_dosageforms.clone()).unwrap_or_default();
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.medplans_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load_medplan(modal.clone(), app.clone());
                    modal.medplans_loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_XL_FULL)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {.class("modal-title").text("ปรับปรุงแผนการจ่ายยาใน HOSxP")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        .style("height","75vh")
                        .style("width", "100%")
                        .style("overflow-y","auto")
                        .children([
                            html!("div", {
                                .visible_signal(not(modal.parent.medplans.signal_vec_cloned().is_empty()))
                                .children([
                                    html!("div", {.class(class::BOLD_BLUE).text("รายการที่ต้องการเพิ่มใน HOSxP")}),
                                    doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                        .children([
                                            html!("thead", {
                                                .child(html!("tr", {
                                                    .class("table-info")
                                                    .children([
                                                        // html!("th", {.attr("scope", "col").visible(false).text("#")}),
                                                        html!("th", {.attr("scope", "col").text("ประเภท")}),
                                                        html!("th", {.attr("scope", "col").text("รายการ")}),
                                                        html!("th", {.attr("scope", "col").text("วิธีใช้").style("min-width","200px")}),
                                                        html!("th", {.attr("scope", "col").text("วันแรก").style("width","70px")}),
                                                        html!("th", {.attr("scope", "col").text("วันถัดไป").style("width","80px")}),
                                                        html!("th", {.attr("scope", "col").style("min-width","75px")}),
                                                    ])
                                                }))
                                            }),
                                            html!("tbody", {
                                                .children_signal_vec(modal.parent.medplans.signal_vec_cloned().map(clone!(modal => move |mpm| {
                                                    html!("tr", {
                                                        .style("cursor","pointer")
                                                        .class_signal("table-warning", not(mpm.add.signal()))
                                                        .class_signal("table-danger", mpm.add.signal().map(clone!(mpm => move |is_add| {
                                                            if is_add {
                                                                mpm.medplan.order_item_type == Some(String::from("injection"))
                                                            } else {
                                                                false
                                                            }
                                                        })))
                                                        .children([
                                                            html!("td", {.class("text-center").text(&mpm.medplan.order_type.clone().unwrap_or_default())}),
                                                            html!("td", {
                                                                .class("fw-bold")
                                                                .text(&mpm.medplan.med_name.clone().unwrap_or_default())
                                                                .apply_if(mpm.medplan.med_reconciliation_item_id.is_some(), |d| d.child(html!("span", {
                                                                    .class(class::BADGE_WRAP_R_GRAY)
                                                                    .style("cursor","default")
                                                                    .text("MR")
                                                                })))
                                                            }),
                                                            html!("td", {
                                                                .text(&mpm.medplan.order_item_detail.clone().unwrap_or_default())
                                                                .apply_if(mpm.medplan.stat == Some(String::from("Y")), |dom| dom.child(html!("span", {
                                                                    .class(class::BADGE_WRAP_R_RED)
                                                                    .style("cursor","default")
                                                                    .text("STAT")
                                                                })))
                                                            }),
                                                            html!("td", {
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type", "number")
                                                                    .class(class::FORM_CTRL_SM)
                                                                    .apply(mixins::string_value(mpm.first_qty.clone(), modal.changed.clone()))
                                                                }))
                                                            }),
                                                            html!("td", {
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type", "number")
                                                                    .class(class::FORM_CTRL_SM)
                                                                    .apply(mixins::string_value(mpm.qty.clone(), modal.changed.clone()))
                                                                }))
                                                            }),
                                                            html!("td", {
                                                                .child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .class_signal("btn-primary", not(mpm.add.signal()))
                                                                    .class_signal("btn-warning", mpm.add.signal())
                                                                    .text_signal(mpm.add.signal_cloned().map(|is_add| if is_add {"Ignore"} else {"Add"}))
                                                                    .event(clone!(mpm => move |_:events::Click| {
                                                                        mpm.add.set(!mpm.add.get());
                                                                    }))
                                                                }))
                                                            }),
                                                        ])
                                                    })
                                                })))
                                            }),
                                        ])
                                    })),
                                ])
                            }),
                            html!("div", {
                                .child(doms::form_inline(clone!(app, modal => move |form| { form
                                    .class("flex-nowrap")
                                    .child(doms::form_inline_group_sm(clone!(app, modal => move |group| { group
                                        .attr("id", "ivfluid_input_group")
                                        .children([
                                            doms::label_group_for("raw_ivfluid_name","สารน้ำผสมยา"),
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "text")
                                                .class("form-control")
                                                .attr("disabled", "")
                                                .attr("id", "raw_ivfluid_name")
                                                .style("width","338px")
                                                .prop_signal("value", modal.ivfluid_name.signal_cloned())
                                            }),
                                        ])
                                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxIvfluidText, false), |dom| dom
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_GRAY)
                                                .children([
                                                    html!("i", {.class(class::FA_PLUS_L)}),
                                                    html!("i", {.class(class::FA_SEARCH)}),
                                                ])
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.ivfluid_name.set(String::new());
                                                    modal.display_ivfluid_searchbox.set_neq(true);
                                                }))
                                            }))
                                        )
                                    })))
                                    .child_signal(modal.display_ivfluid_searchbox.signal_cloned().map(clone!(app, modal => move |show| {
                                        if show {
                                            app.get_id("ivfluid_input_group").map(|elm| {
                                                doms::under_box(
                                                    elm.get_bounding_client_rect(),
                                                    600.0, 300.0, app.window_scroll_y(),
                                                    clone!(app, modal => move |bx| { bx
                                                        .child(Self::render_ivfluid_searchbox(modal, app))
                                                    })
                                                )
                                            })
                                        } else {
                                            None
                                        }
                                    })))
                                    .child(doms::form_inline_group_sm(clone!(modal => move |group| { group
                                        .attr("id", "order_item_detail_input_group")
                                        .children([
                                            doms::label_group_for("order_item_detail","วิธีใช้"),
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "text")
                                                .class("form-control")
                                                .style("width","410px")
                                                .attr("id", "order_item_detail")
                                                .attr("placeholder", "เช่น *iv หรือ ระบุวิธีใช้ยา")
                                                .attr("required", "")
                                                .apply(mixins::string_value(modal.order_item_detail.clone(), modal.ivfluid_changed.clone()))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_SM_GRAY)
                                                .attr("type", "button")
                                                .child(html!("i", {.class(class::FA_X)}))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.order_item_detail.set_neq(String::new());
                                                    modal.ivfluid_changed.set_neq(false);
                                                }))
                                            }),
                                        ])
                                    })))
                                    .child_signal(modal.order_item_detail.signal_cloned().map(clone!(app, modal => move |order_item_detail| {
                                        if order_item_detail.is_empty() || order_item_detail.chars().any(|c| !c.is_ascii()) {
                                            None
                                        } else {
                                            app.get_id("order_item_detail_input_group").map(|elm| {
                                                doms::under_box(
                                                    elm.get_bounding_client_rect(),
                                                    600.0, 300.0, app.window_scroll_y(),
                                                    clone!(app, modal => move |bx| { bx
                                                        .child(html!("div", {
                                                            .class(class::CARD_CYANS)
                                                            .style("height", "294px")
                                                            .child(search_drugusage("294px", modal.order_item_detail.clone(), app.clone()))
                                                        }))
                                                    })
                                                )
                                            })
                                        }
                                    })))
                                    .child(html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_L)
                                        .class_signal("btn-primary", modal.ivfluid_ready())
                                        .class_signal("btn-secondary", not(modal.ivfluid_ready()))
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .text(" เพิ่มสารน้ำ")
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                            modal.add_ivfluids();
                                        }), not(modal.ivfluid_ready()), app.state()))
                                    }))
                                })))
                            }),
                            html!("hr"),
                            html!("div", {
                                .visible_signal(not(modal.parent.off_medplans.signal_vec_cloned().is_empty()))
                                .children([
                                    html!("div", {.class(class::BOLD_RED).text("รายการที่ต้องการ OFF ใน HOSxP")}),
                                    doms::table_responsive(class::TABLE_STRIP, clone!(modal, injections => move |table| { table
                                        .children([
                                            html!("thead", {
                                                .child(html!("tr", {
                                                    .class("table-warning")
                                                    .children([
                                                        html!("th", {.attr("scope", "col").text("วันที่สั่ง").style("width","110px")}),
                                                        html!("th", {.attr("scope", "col").text("ประเภท")}),
                                                        html!("th", {.attr("scope", "col").text("รายการ")}),
                                                        html!("th", {.attr("scope", "col").text("วิธีใช้").style("min-width","200px")}),
                                                        html!("th", {.attr("scope", "col").style("min-width","75px")}),
                                                    ])
                                                }))
                                            }),
                                            html!("tbody", {
                                                .children_signal_vec(modal.parent.off_medplans.signal_vec_cloned().map(clone!(injections => move |ompm| {
                                                    html!("tr", {
                                                        .style("cursor","pointer")
                                                        .class_signal("table-warning", ompm.off.signal_cloned().map(clone!(ompm => move |off_status| off_status != ompm.is_off)))
                                                        .class_signal("table-danger", ompm.off.signal_cloned().map(clone!(injections, ompm => move |off_status| {
                                                            if off_status == ompm.is_off {
                                                                ompm.medplan.dosageform.as_ref().map(|dosageform| injections.contains(&dosageform)).unwrap_or_default()
                                                            } else {
                                                                false
                                                            }
                                                        })))
                                                        .children([
                                                            html!("td", {.class("text-center").text(&date_th_opt_relative(&ompm.medplan.orderdate.map(|dt| dt.date())))}),
                                                            html!("td", {.class("text-center").text(&ompm.medplan.orderstatus.as_ref().map(|st| if st == "C" {"continuous"} else {"oneday"}).unwrap_or_default())}),
                                                            html!("td", {.class("fw-bold").text(&ompm.medplan.med_name.clone().unwrap_or_default())}),
                                                            html!("td", {.text(&ompm.medplan.drug_usage.clone().unwrap_or_default())}),
                                                            html!("td", {
                                                                .child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .class_signal("btn-primary", not(ompm.off.signal()))
                                                                    .class_signal("btn-warning", ompm.off.signal())
                                                                    .text_signal(ompm.off.signal_cloned().map(|status_off| if status_off {"Ignore"} else {"OFF"}))
                                                                    .event(clone!(ompm => move |_:events::Click| {
                                                                        ompm.off.set(!ompm.off.get());
                                                                    }))
                                                                }))
                                                            }),
                                                        ])
                                                    })
                                                })))
                                            }),
                                        ])
                                    })),
                                ])
                            }),
                            html!("div", {
                                .visible_signal(not(modal.parent.retain_medplans.signal_vec_cloned().is_empty()))
                                .children([
                                    html!("hr"),
                                    html!("div", {.class("fw-bold").text("รายการปัจจุบันใน HOSxP")}),
                                    doms::table_responsive(class::TABLE_STRIP, clone!(modal, injections => move |table| { table
                                        .children([
                                            html!("thead", {
                                                .child(html!("tr", {
                                                    .children([
                                                        html!("th", {.attr("scope", "col").text("วันที่สั่ง").style("width","110px")}),
                                                        html!("th", {.attr("scope", "col").text("ประเภท")}),
                                                        html!("th", {.attr("scope", "col").text("รายการ")}),
                                                        html!("th", {.attr("scope", "col").text("วิธีใช้").style("min-width","200px")}),
                                                        html!("th", {.attr("scope", "col").style("min-width","75px")}),
                                                    ])
                                                }))
                                            }),
                                            html!("tbody", {
                                                .children_signal_vec(modal.parent.retain_medplans.signal_vec_cloned().map(clone!(injections => move |rmpm| {
                                                    html!("tr", {
                                                        .style("cursor","pointer")
                                                        .class_signal("table-warning", rmpm.off.signal_cloned().map(clone!(rmpm => move |off_status| off_status != rmpm.is_off)))
                                                        .class_signal("table-danger", rmpm.off.signal_cloned().map(clone!(injections, rmpm => move |off_status| {
                                                            if off_status == rmpm.is_off {
                                                                rmpm.medplan.dosageform.as_ref().map(|dosageform| injections.contains(&dosageform)).unwrap_or_default()
                                                            } else {
                                                                false
                                                            }
                                                        })))
                                                        .children([
                                                            html!("td", {.class("text-center").text(&date_th_opt_relative(&rmpm.medplan.orderdate.map(|dt| dt.date())))}),
                                                            html!("td", {.class("text-center").text(&rmpm.medplan.orderstatus.as_ref().map(|st| if st == "C" {"continuous"} else {"oneday"}).unwrap_or_default())}),
                                                            html!("td", {.class("fw-bold").text(&rmpm.medplan.med_name.clone().unwrap_or_default())}),
                                                            html!("td", {.text(&rmpm.medplan.drug_usage.clone().unwrap_or_default())}),
                                                            html!("td", {
                                                                .child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .class_signal("btn-primary", not(rmpm.off.signal()))
                                                                    .class_signal("btn-warning", rmpm.off.signal())
                                                                    .text_signal(rmpm.off.signal_cloned().map(|status_off| if status_off {"ยกเลิก"} else {"OFF"}))
                                                                    .event(clone!(rmpm => move |_:events::Click| {
                                                                        rmpm.off.set(!rmpm.off.get());
                                                                    }))
                                                                }))
                                                            }),
                                                        ])
                                                    })
                                                })))
                                            }),
                                        ])
                                    })),
                                ])
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .children([
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_L_BLUE)
                                .attr("data-bs-dismiss", "modal")
                                .text("ดำเนินการต่อ")
                                .apply(mixins::click_with_loader_checked(clone!(app, modal => move || {
                                    OrderCpn::patch_order(OrderPatchAction::PharmacistAccept, modal.order.clone(), time_8601(&modal.order_time_mutable.lock_ref()), None, modal.is_oneday, modal.parent.clone(), app.clone());
                                }), app.state()))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GRAY)
                                .attr("data-bs-dismiss", "modal")
                                .text("ยกเลิก")
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    fn render_ivfluid_searchbox(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .child(html!("div", {
                .class(class::CARD_TW_T_CYANS)
                .children([
                    html!("div", {
                        .class(class::INPUT_GROUP_T)
                        .children([
                            html!("span", {
                                .class("input-group-text")
                                .child(html!("i", {.class(class::FA_SEARCH)}))
                            }),
                            html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("placeholder", "กรอกข้อความเพื่อค้นหา")
                                .attr("autocomplete", "off")
                                .focused(true)
                                .with_node!(element => {
                                    .apply(|dom| {
                                        let load_mut = Mutable::new(false);
                                        dom.event(clone!(modal, load_mut => move |_: events::Input| {
                                            let search_text = element.value();
                                            if search_text.is_empty() {
                                                modal.results.lock_mut().clear();
                                                modal.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
                                            } else {
                                                modal.text.set_neq(search_text);
                                                load_mut.set(true);
                                            }
                                        }))
                                        .future(map_ref!{
                                            let busy = app.loader_is_loading(),
                                            let load = load_mut.signal() =>
                                            !busy && *load
                                        }.for_each(clone!(app, modal, load_mut => move |ready| {
                                            if ready {
                                                load_mut.set(false);
                                                Self::load_ivfluids(modal.clone(), app.clone());
                                            }
                                            async {}
                                        })))
                                    })
                                })
                            }),
                            html!("button", {
                                .class(class::BTN_SM_R_RED)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_X)}))
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.display_ivfluid_searchbox.set_neq(false);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .style("height","250px")
                        .style("border-width","thin")
                        .style("overflow-y","auto")
                        .child(html!("table", {
                            .class(class::TABLE_STRIP)
                            .children([
                                html!("thead", {
                                    .visible(false)
                                    .child(html!("tr", {
                                        .child(html!("th", {
                                            .attr("scope", "col")
                                            .text("Name")
                                        }))
                                    }))
                                }),
                                html!("tbody", {
                                    .child_signal(modal.status_text.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|text| {
                                            html!("tr", {
                                                .child(html!("td", {
                                                    .attr("colspan", "2")
                                                    .text(text)
                                                }))
                                            })
                                        })
                                    }))
                                    .children_signal_vec(modal.results.signal_vec_cloned().map(move |result| {
                                        html!("tr", {
                                            .child(html!("td", {
                                                .class("fw-bold")
                                                .style("cursor","pointer")
                                                .child(doms::color_prefix_span(&dec_to_color(result.displaycolor.unwrap_or_default())))
                                                .text(&result.ivfluid_name.clone().unwrap_or_default())
                                                .event(clone!(modal, result => move |_: events::Click| {
                                                    modal.ivfluid_name.set(result.ivfluid_name.clone().unwrap_or_default());
                                                    modal.icode.set(result.icode.clone());
                                                    modal.display_ivfluid_searchbox.set(false);
                                                }))
                                            }))
                                        })
                                    }))
                                }),
                            ])
                        }))
                    }),
                ])
            }))
        })
    }
}

pub struct MedPlanMutable {
    medplan: MedPlanItem,
    first_qty: Mutable<String>,
    qty: Mutable<String>,
    add: Mutable<bool>,
}

impl MedPlanMutable {
    pub fn new(medplan: MedPlanItem) -> Rc<Self> {
        let is_oneday = medplan.order_type == Some(String::from("oneday"));
        let day_dose = get_day_dose_from_detail(&medplan.order_item_detail.clone().unwrap_or_default()).to_string();
        let first_qty = medplan.first_qty.map(|i| i.to_string()).unwrap_or(day_dose.clone());
        Rc::new(Self {
            medplan,
            first_qty: Mutable::new(first_qty),
            qty: Mutable::new(if is_oneday { String::from("0") } else { day_dose }),
            add: Mutable::new(true),
        })
    }
    pub fn export(&self) -> Option<MedPlanItem> {
        self.add.get().then(|| MedPlanItem {
            order_item_id: self.medplan.order_item_id,
            an: self.medplan.an.clone(),
            order_date: self.medplan.order_date,
            order_time: self.medplan.order_time,
            order_type: self.medplan.order_type.clone(),
            order_item_type: self.medplan.order_item_type.clone(),
            med_name: self.medplan.med_name.clone(),
            order_item_detail: self.medplan.order_item_detail.clone(),
            icode: self.medplan.icode.clone(),
            order_doctor: self.medplan.order_doctor.clone(),
            stat: self.medplan.stat.clone(),
            first_qty: self.first_qty.lock_ref().parse::<i32>().ok(),
            qty: self.qty.lock_ref().parse::<i32>().ok(),
            med_reconciliation_item_id: self.medplan.med_reconciliation_item_id,
        })
    }
}

pub struct OffMedPlanMutable {
    medplan: HisMedPlanIpd,
    is_off: bool,
    off: Mutable<bool>,
}

impl OffMedPlanMutable {
    pub fn new_with_off(medplan: HisMedPlanIpd, is_off: bool) -> Rc<Self> {
        Rc::new(Self {
            medplan,
            is_off,
            off: Mutable::new(is_off),
        })
    }
    pub fn off_med_plan_number(&self) -> Option<i32> {
        self.off.get().then(|| self.medplan.med_plan_number)
    }
}
