use dominator::{Dom, DomBuilder, clone, events, html, is_window_loaded, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTableCellElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    pre_order::master::{PreOrderMaster, PreOrderMasterParams},
    route::Route,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_component::{
    gadget::searchbox::patient::PatientSearchboxCpn,
    modal::{blank_modal, pre_order_new::PreOrderNew},
};
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, date_th_opt, datetime_th_opt_relative},
    util::{pre_order_type_display, str_some},
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    OrderDateTime,
    Hn,
    Name,
    ForDate,
}

/// - GET `EndPoint::IpdPreOrderMaster`
/// - POST `EndPoint::IpdPreOrderMaster` (PreOrderNew, guarded, remove 'เพิ่มใบ Order ใหม่' btn)
/// - GET `EndPoint::SearchBoxPatientText` (PatientSearchboxCpn, guarded, remove search-patient div)
#[derive(Clone, Default)]
pub struct IpdPreOrderListPage {
    view_by: Mutable<String>,

    hn: Mutable<String>,
    pt_name: Mutable<String>,
    start_order_date: Mutable<String>,
    end_order_date: Mutable<String>,
    order_doctor: Mutable<String>,
    pre_order_type: Mutable<String>,
    template_name: Mutable<String>,
    include_shared_template: Mutable<String>, // "Y" or "N"
    used: Mutable<String>,                    // "Y" or "N"

    search_result: MutableVec<Rc<PreOrderMaster>>,
    display_patient_searchbox: Mutable<bool>,
    changed: Mutable<bool>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,

    pre_order_new_modal: Mutable<Option<Rc<PreOrderNew>>>,
}

impl IpdPreOrderListPage {
    pub fn new(view_by: &str, app: Rc<App>) -> Rc<Self> {
        let order_doctor = if (app.has_permission(Permission::IpdOrderAdd) || app.has_permission(Permission::OpdErOrderAdd)) && app.has_permission(Permission::DataTypeDoctorUse) {
            app.doctor_code().unwrap_or_default()
        } else {
            String::new()
        };
        Rc::new(Self {
            view_by: Mutable::new(view_by.to_owned()),
            include_shared_template: Mutable::new(String::from("N")),
            order_doctor: Mutable::new(order_doctor),
            used: Mutable::new(String::from("N")),
            is_desc: Mutable::new(true),
            ..Default::default()
        })
    }

    fn sortable_mixin(sort_by: SortBy, page: Rc<Self>) -> impl FnOnce(DomBuilder<HtmlTableCellElement>) -> DomBuilder<HtmlTableCellElement> {
        #[inline]
        move |dom| {
            with_node!(dom, element => {
                .style("cursor","pointer")
                .child_signal(map_ref! {
                    let is_this = page.sorted_by.signal_ref(clone!(sort_by => move |sb| *sb == sort_by)),
                    let is_desc = page.is_desc.signal() =>
                    is_this.then(|| {
                        html!("i", {
                            .class("ms-1")
                            .class(if *is_desc {
                                class::FA_UP91
                            } else {
                                class::FA_DOWN19
                            })
                        })
                    })
                })
                .event(clone!(sort_by => move |_:events::Click| {
                    if page.sorted_by.get_cloned() != sort_by {
                        page.sorted_by.set(sort_by.clone());
                        page.is_desc.set_neq(false);
                    } else {
                        page.is_desc.set(!page.is_desc.get());
                    }
                    page.sort();
                }))
            })
        }
    }

    fn sort(&self) {
        let mut items = self.search_result.lock_ref().to_vec();
        if self.is_desc.get() {
            match self.sorted_by.get_cloned() {
                SortBy::OrderDateTime => items.sort_by(|a, b| b.order_date_time.cmp(&a.order_date_time)),
                SortBy::Hn => items.sort_by(|a, b| b.hn.cmp(&a.hn)),
                SortBy::Name => items.sort_by(|a, b| b.fullname.cmp(&a.fullname)),
                SortBy::ForDate => items.sort_by(|a, b| b.order_for_date.cmp(&a.order_for_date)),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::OrderDateTime => items.sort_by(|a, b| a.order_date_time.cmp(&b.order_date_time)),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::Name => items.sort_by(|a, b| a.fullname.cmp(&b.fullname)),
                SortBy::ForDate => items.sort_by(|a, b| a.order_for_date.cmp(&b.order_for_date)),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
    }

    // ipd-dr-pre-order-list-data.php
    // send GET method
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let params = PreOrderMasterParams {
            pre_order_master_id: None,
            hn: str_some(page.hn.get_cloned()),
            start_order_date: date_8601(&page.start_order_date.lock_ref()),
            end_order_date: date_8601(&page.end_order_date.lock_ref()),
            order_doctor: str_some(page.order_doctor.get_cloned()),
            include_shared_template: str_some(page.include_shared_template.get_cloned()),
            pre_order_type: str_some(page.pre_order_type.get_cloned()),
            template_name: str_some(page.template_name.get_cloned()),
            used: str_some(page.used.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::IpdPreOrderMaster`
                match PreOrderMaster::call_api_get(&params, app.state()).await {
                    Ok(orders) => {
                        let mut lock = page.search_result.lock_mut();
                        lock.clear();
                        lock.extend(orders.into_iter().map(Rc::new));
                        page.sorted_by.set(SortBy::OrderDateTime);
                        page.is_desc.set_neq(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Pre-Order/Template List");

        let all_doctor_select_option = app.app_asset.lock_ref().as_ref().map(|asset| asset.all_doctor_select_option.clone()).unwrap_or_default();

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if let Some(elm) = app.get_id("order_doctor") {
                        NiceSelect::new_default(&elm);
                    }
                    page.changed.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit(page.clone(), app.clone());
                    page.changed.set_neq(false);
                }
                async {}
            })))
            .class("container-fluid")
            .children([
                doms::alert_row(clone!(app, page => move |alert| { alert
                    .children([
                        doms::form_inline(clone!(app, page => move |form| { form
                            .children([
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("pre_order_type","ประเภทใบ Order"),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM)
                                            .attr("id", "pre_order_type")
                                            .children([
                                                html!("option", {
                                                    .attr("value", "")
                                                    .text("ทั้งหมด")
                                                }),
                                                html!("option", {
                                                    .attr("value", "pre_order")
                                                    // $IPD_ORDER_ADD && $DATA_TYPE_DOCTOR_USE
                                                    .apply_if(
                                                        (app.has_permission(Permission::IpdOrderAdd) || app.has_permission(Permission::OpdErOrderAdd))
                                                        && app.has_permission(Permission::DataTypeDoctorUse),
                                                    |dom| {
                                                        dom.attr("selected","")
                                                    })
                                                    .text("Admit ล่วงหน้า และ Admit ในวัน")
                                                }),
                                                html!("option", {
                                                    .attr("value", "appointment")
                                                    .text("Admit ล่วงหน้า")
                                                }),
                                                html!("option", {
                                                    .attr("value", "opd")
                                                    .text("Admit ในวัน")
                                                }),
                                                html!("option", {
                                                    .attr("value", "template")
                                                    .text("Template")
                                                }),
                                            ])
                                            .apply(mixins::string_value_select(page.pre_order_type.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("order_doctor","ผู้บันทึก"),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "order_doctor")
                                                .child(html!("option", {.attr("value", "").text("ทั้งหมด")}))
                                                .children(all_doctor_select_option.iter().map(|option| {
                                                    doms::select_option(option, &page.order_doctor.lock_ref())
                                                }))
                                                .apply(mixins::string_value_select(page.order_doctor.clone(), page.changed.clone()))
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_GRAY)
                                            .child(html!("i", {.class(class::FA_USER)}))
                                            .event(clone!(app, page => move |_: events::Click| {
                                                let doctor_code = app.doctor_code().unwrap_or_default();
                                                let neq = page.order_doctor.lock_ref().as_str() != doctor_code.as_str();
                                                if neq {
                                                    if let Some(elm) = app.get_id("order_doctor") {
                                                        NiceSelect::new_default_with_value(&elm, &doctor_code);
                                                    }
                                                    page.order_doctor.set_neq(doctor_code);
                                                    page.changed.set_neq(true);
                                                }
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                let no_doctor = page.order_doctor.lock_ref().is_empty();
                                                if !no_doctor {
                                                    page.order_doctor.set_neq(String::new());
                                                    if let Some(elm) = app.get_id("order_doctor") {
                                                        NiceSelect::new_default_with_value(&elm,"");
                                                    }
                                                    page.changed.set_neq(true);
                                                }
                                            }))
                                        }),
                                    ])
                                })),
                            ])
                            .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxPatientText, false), |dom| dom
                                .child(doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .attr("id", "hn_input_group")
                                    .children([
                                        doms::label_group_for("ptname","HN "),
                                        html!("input", {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "ptname")
                                            .attr("readonly", "readonly")
                                            .attr("size", "35")
                                            .prop_signal("value", page.pt_name.signal_cloned())
                                        }),
                                        html!("button", {
                                            .class(class::BTN_SM_GRAY)
                                            //.attr("id", "hn-button")
                                            .attr("type", "button")
                                            .child(html!("i", {.class(class::FA_SEARCH)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                page.display_patient_searchbox.set_neq(true);
                                            }))
                                        }),
                                        html!("button", {
                                            .class(class::BTN_SM_RED)
                                            .attr("type", "button")
                                            .child(html!("i", {
                                                .class(class::FA_X)
                                            }))
                                            .event(clone!(page => move |_: events::Click| {
                                                page.hn.set_neq(String::new());
                                                page.pt_name.set_neq(String::new());
                                                page.changed.set_neq(true);
                                            }))
                                        }),
                                    ])
                                })))
                                .child_signal(page.display_patient_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                                    if show {
                                        app.get_id("hn_input_group").map(clone!(app, page => move |elm| {
                                            PatientSearchboxCpn::render(
                                                PatientSearchboxCpn::new(),
                                                page.display_patient_searchbox.clone(),
                                                page.hn.clone(),
                                                page.pt_name.clone(),
                                                elm.get_bounding_client_rect(),
                                                page.changed.clone(),
                                                app,
                                            )
                                        }))
                                    } else {
                                        None
                                    }
                                })))
                            )
                            .children([
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("start_order_date","วันที่บันทึก"),
                                        doms::date_picker(
                                            page.start_order_date.clone(),
                                            page.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "start_order_date"),
                                            |s| s, always(None),
                                        ),
                                        doms::label_group_for("end_order_date","ถึง"),
                                        doms::date_picker(
                                            page.end_order_date.clone(),
                                            page.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "end_order_date"),
                                            |s| s, always(None),
                                        ),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("template_name_list","ชื่อ Template"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "template_name_list")
                                            .apply(mixins::string_value_end(page.template_name.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("used","ใช้งาน"),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM)
                                            .attr("id", "used")
                                            .children([
                                                html!("option", {
                                                    .attr("value", "")
                                                    .text("ทั้งหมด")
                                                }),
                                                html!("option", {
                                                    .attr("value", "Y")
                                                    .text("ใช้แล้ว")
                                                }),
                                                html!("option", {
                                                    .attr("value", "N")
                                                    .attr("selected", "")
                                                    .text("ยังไม่ได้ใช้")
                                                }),
                                            ])
                                            .apply(mixins::string_value_select(page.used.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_switch(clone!(page => move |check| { check
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("role","switch")
                                            .attr("id", "include_shared_template")
                                            .apply(mixins::checkbox_toggle(page.include_shared_template.clone(), page.changed.clone(), "Y", "N"))
                                        }),
                                        doms::label_check_for("include_shared_template","แสดง Shared Template"),
                                    ])
                                })),
                                doms::form_inline_end(clone!(app, page => move |end| { end
                                    .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderMaster, true), |dom| {
                                        dom.child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_L_BLUE)
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#addPreOrderModal")
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .text(" เพิ่มใบ Order ใหม่")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.pre_order_new_modal.set(Some(PreOrderNew::new(page.view_by.clone())));
                                            }))
                                        }))
                                    })
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_SEARCH)}))
                                        .text(" ค้นหา")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.changed.set_neq(true);
                                        }))
                                    }))
                                })),
                            ])
                        })),
                        html!("div", {
                            .class("col-sm")
                            .child(doms::badge_info_center("หากไม่ได้ระบุช่วงวันที่ โปรแกรมจะแสดงเฉพาะ 200 รายการล่าสุด"))
                            .child_signal(page.search_result.signal_vec_cloned().len().map(|i| {
                                Some(doms::badge_count_with_limit(i, 200))
                            }))
                        }),
                    ])
                })),
                doms::table_responsive(class::TABLE_STRIP, clone!(app, page => move |table| { table
                    .children([
                        html!("thead", {
                            .child(html!("tr", {
                                .class("text-center")
                                .children([
                                    html!("th", {.attr("scope", "col").text("#")}),
                                    // html!("th", {
                                    //     .attr("scope", "col")
                                    //     .text("เลขที่ใบ Order")
                                    // }),
                                    html!("th" => HtmlTableCellElement, {
                                        .attr("scope", "col").text("เวลาที่บันทึก")
                                        .apply(Self::sortable_mixin(SortBy::OrderDateTime, page.clone()))
                                    }),
                                    html!("th", {.attr("scope", "col").text("ประเภทใบ Order")}),
                                    html!("th", {.attr("scope", "col").text("ผู้บันทึก")}),
                                    html!("th" => HtmlTableCellElement, {
                                        .attr("scope", "col").text("HN")
                                        .apply(Self::sortable_mixin(SortBy::Hn, page.clone()))
                                    }),
                                    html!("th" => HtmlTableCellElement, {
                                        .attr("scope", "col").text("ชื่อ-นามสกุล")
                                        .apply(Self::sortable_mixin(SortBy::Name, page.clone()))
                                    }),
                                    html!("th" => HtmlTableCellElement, {
                                        .attr("scope", "col").text("วันที่นัด/Admit")
                                        .apply(Self::sortable_mixin(SortBy::ForDate, page.clone()))
                                    }),
                                    html!("th", {.attr("scope", "col").text("ใช้งาน")}),
                                    html!("th", {.attr("scope", "col").text("ชื่อ Template")}),
                                ])
                            }))
                        }),
                        html!("tbody", {
                            .children_signal_vec(page.search_result.signal_vec_cloned().enumerate().map(move |(i,row)| {
                                Self::render_result(page.clone(), i.get().unwrap_or_default(), row, app.clone())
                            }))
                        }),
                    ])
                })),
                html!("div", {
                    .class("modal")
                    .attr("id", "addPreOrderModal")
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.pre_order_new_modal.signal_cloned().map(clone!(app, page => move |opt| {
                        opt.as_ref().map(clone!(app, page => move |modal| {
                            PreOrderNew::render(modal.clone(), page.pre_order_new_modal.clone(), page.changed.clone(), app)
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }

    fn render_result(page: Rc<Self>, i: usize, row: Rc<PreOrderMaster>, app: Rc<App>) -> Dom {
        html!("tr", {
            .style("cursor","pointer")
            .children([
                html!("td", {.class("text-center").text(&(i + 1).to_string())}),
                // html!("td", {.text(&row.pre_order_master_id.to_owned())}),
                html!("td", {.text(&datetime_th_opt_relative(&row.order_date_time))}),
                html!("td", {.text(pre_order_type_display(&row.pre_order_type))}),
                html!("td", {
                    .apply_if(row.order_doctor_is_intern.unwrap_or_default(), |dom| dom.text("(Intern) "))
                    .text(&row.order_doctor_name.clone().unwrap_or_default())
                }),
                html!("td", {.text(&row.hn.clone().unwrap_or_default())}),
                html!("td", {.text(&row.fullname.clone().unwrap_or_default())}),
                html!("td", {.text(&date_th_opt(&row.order_for_date))}),
                html!("td", {.text({
                    if row.pre_order_type.as_str() == "template" {""} else if row.used == Some(String::from("Y")) {"ใช้แล้ว"} else {"ยังไม่ได้ใช้"}
                })}),
                html!("td", {
                    .apply_if(row.shared_template == Some(String::from("Y")), |dom| {
                        dom.child(html!("i", {.class(class::FA_STAR_L)}))
                    })
                    .child(html!("span", {.text(&row.template_name.clone().unwrap_or_default())}))
                })
            ])
            .apply(|dom| {
                let route = Route::IpdPreOrder {
                    view_by: page.view_by.get_cloned(),
                    pre_order_master_id: row.pre_order_master_id,
                };
                if route.has_permission(app.state()) {
                    dom.event(move |_: events::Click| {
                        route.hard_redirect();
                    })
                } else {
                    dom
                }
            })
        })
    }
}
