use dominator::{Dom, DomBuilder, clone, events, html, link, with_node};
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
    opd_er::order_master::{OpdErOrderMasterList, OpdErOrderMasterParams},
    route::Route,
    score::Scores,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_component::modal::{blank_modal, opd_er_order_new::OpdErOrderNew};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, datetime_8601, datetime_th_opt_relative, datetime_th_relative},
    util::{opt_zero_none, str_some},
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    BedNo,
    VisitDateTime,
    InitDateTime,
    Hn,
    Qn,
    Name,
    Age,
    MaxOrderDateTime,
    MaxVsDateTime,
}

/// - GET `EndPoint::OpdErOrderMaster`
/// - POST `EndPoint::OpdErOrderMaster` (OpdErOrderNew, guarded, remove 'เพิ่มใบ Order ใหม่' btn)
#[derive(Clone, Default)]
pub struct OpdErOrderListPage {
    loaded: Mutable<bool>,
    view_by: Mutable<String>,

    // order_doctor: Mutable<String>,
    order_date: Mutable<String>,
    start_order_date: Mutable<String>,
    end_order_date: Mutable<String>,
    hn: Mutable<String>,
    vn: Mutable<String>,
    vstdate: Mutable<String>,
    qn: Mutable<String>,
    bedno: Mutable<String>,
    er_patient_status: Mutable<String>,
    // er_patient_status_id: Mutable<String>,
    // er_dch_type_id: Mutable<String>,
    search_result: MutableVec<Rc<OpdErOrderMasterList>>,
    changed: Mutable<bool>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,

    timer_handle: Mutable<Option<i32>>,
    timer_second: Mutable<f32>,

    opd_er_order_new_modal: Mutable<Option<Rc<OpdErOrderNew>>>,
}

impl OpdErOrderListPage {
    pub fn new(view_by: String) -> Rc<Self> {
        Rc::new(Self {
            view_by: Mutable::new(view_by),
            // order_doctor: Mutable::new(app.doctor_code().unwrap_or_default()),
            er_patient_status: Mutable::new(String::from("in_er")),
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
                SortBy::BedNo => items.sort_by(|a, b| b.bedno.cmp(&a.bedno)),
                SortBy::VisitDateTime => items.sort_by(|a, b| b.vstdate_time.cmp(&a.vstdate_time)),
                SortBy::InitDateTime => items.sort_by(|a, b| b.order_date_time.cmp(&a.order_date_time)),
                SortBy::Hn => items.sort_by(|a, b| b.hn.cmp(&a.hn)),
                SortBy::Qn => items.sort_by(|a, b| b.oqueue.cmp(&a.oqueue)),
                SortBy::Name => items.sort_by(|a, b| b.ptname.cmp(&a.ptname)),
                SortBy::Age => items.sort_by(|a, b| b.age_y.cmp(&a.age_y).then(b.age_m.cmp(&a.age_m)).then(b.age_d.cmp(&a.age_d))),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| b.max_order_date_time.cmp(&a.max_order_date_time)),
                SortBy::MaxVsDateTime => items.sort_by(|a, b| {
                    b.ews_concat
                        .as_ref()
                        .and_then(|concat| concat.split('|').next())
                        .and_then(|s| datetime_8601(s))
                        .cmp(&a.ews_concat.as_ref().and_then(|concat| concat.split('|').next()).and_then(|s| datetime_8601(s)))
                }),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::BedNo => items.sort_by(|a, b| a.bedno.cmp(&b.bedno)),
                SortBy::VisitDateTime => items.sort_by(|a, b| a.vstdate_time.cmp(&b.vstdate_time)),
                SortBy::InitDateTime => items.sort_by(|a, b| a.order_date_time.cmp(&b.order_date_time)),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::Qn => items.sort_by(|a, b| a.oqueue.cmp(&b.oqueue)),
                SortBy::Name => items.sort_by(|a, b| a.ptname.cmp(&b.ptname)),
                SortBy::Age => items.sort_by(|a, b| a.age_y.cmp(&b.age_y).then(a.age_m.cmp(&b.age_m)).then(a.age_d.cmp(&b.age_d))),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| a.max_order_date_time.cmp(&b.max_order_date_time)),
                SortBy::MaxVsDateTime => items.sort_by(|a, b| {
                    a.ews_concat
                        .as_ref()
                        .and_then(|concat| concat.split('|').next())
                        .and_then(|s| datetime_8601(s))
                        .cmp(&b.ews_concat.as_ref().and_then(|concat| concat.split('|').next()).and_then(|s| datetime_8601(s)))
                }),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
    }

    // opd-er-order-list-data.php
    // send GET method
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let (er_patient_status_id, er_dch_type_id) = match page.er_patient_status.lock_ref().as_str() {
            "in_er" => (Some(0), None), // om.er_patient_status_id <> 7
            "7" => (Some(7), None),
            "7_1" => (Some(7), Some(1)),
            "7_2" => (Some(7), Some(2)),
            "7_3" => (Some(7), Some(3)),
            "7_4" => (Some(7), Some(4)),
            _ => (None, None),
        };

        let params = OpdErOrderMasterParams {
            opd_er_order_master_id: None,
            order_doctor: None,
            order_date: date_8601(&page.order_date.lock_ref()),
            start_order_date: date_8601(&page.start_order_date.lock_ref()),
            end_order_date: date_8601(&page.end_order_date.lock_ref()),
            hn: str_some(page.hn.get_cloned()),
            vn: str_some(page.vn.get_cloned()),
            vstdate: date_8601(&page.vstdate.lock_ref()),
            qn: str_some(page.qn.get_cloned()),
            bedno: str_some(page.bedno.get_cloned()),
            er_patient_status_id,
            er_dch_type_id,
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::OpdErOrderMaster`
                match OpdErOrderMasterList::call_api_get(&params, app.state()).await {
                    Ok(orders) => {
                        let mut lock = page.search_result.lock_mut();
                        lock.clear();
                        lock.extend(orders.into_iter().map(Rc::new));
                        page.sorted_by.set(SortBy::BedNo);
                        page.is_desc.set_neq(false);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - OPD-ER List");

        let all_er_bed_select_option = app.app_asset.lock_ref().as_ref().map(|asset| asset.er_bed_select_options.clone()).unwrap_or_default();

        html!("section", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit(page.clone(), app.clone());
                    page.loaded.set(true);
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
                    page.changed.set(false);
                }
                async {}
            })))
            .future(app.monitor_refresh_interval.signal_cloned().for_each(clone!(app, page => move |interval_str| {
                if let Some(handle_id) = page.timer_handle.get() {
                    app.clear_interval(handle_id);
                }
                page.timer_second.set(0.0);
                if let Some(interval) = opt_zero_none(interval_str.parse::<i32>().ok().map(|i| i * 1000 / 60)) {
                    let handle = app.set_interval(clone!(page => move || {
                        let timer = page.timer_second.get();
                        if timer == 59.0 {
                            page.changed.set_neq(true);
                            page.timer_second.set(0.0);
                        } else {
                            page.timer_second.set(timer + 1.0);
                        }
                    }), interval);
                    page.timer_handle.set_neq(Some(handle));
                }
                async {}
            })))
            .class("container-fluid")
            .child(doms::alert_row(clone!(app, page => move |alert| { alert
                .children([
                    doms::form_inline(clone!(app, page => move |form| { form
                        .children([
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("order_date","วันที่บันทึกรายการ"),
                                    doms::date_picker(
                                        page.order_date.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "order_date"),
                                        |s| s, always(None),
                                    ),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("vstdate","วันที่ส่งตรวจ"),
                                    doms::date_picker(
                                        page.vstdate.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "vstdate"),
                                        |s| s, always(None),
                                    ),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("bedno","เตียง"),
                                    html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        // .style("width","150px")
                                        .attr("id", "bedno")
                                        .child(html!("option", {
                                            .attr("value", "")
                                            .style("color","#777")
                                            .style("background-color","white")
                                            .text("เลือก")
                                        }))
                                        .children(all_er_bed_select_option.iter().map(|option| {
                                            doms::select_option_color(option, &page.bedno.lock_ref())
                                        }))
                                        .apply(mixins::string_value_select(page.bedno.clone(), page.changed.clone()))
                                        // .attr("onchange", "onchangeBedNoSelect(event, this)")
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("er_patient_status","สถานะ"),
                                    html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .attr("id", "er_patient_status")
                                        .child(html!("option", {.attr("value", "in_er").attr("selected", "").text("ผู้ป่วยใน ER")}))
                                        .children([
                                            html!("optgroup", {
                                                .attr("label", "Discharge แล้ว")
                                                .children([
                                                    html!("option", {.attr("value", "7").text("Discharge แล้ว")}),
                                                    html!("option", {.attr("value", "7_1").text("Discharge แล้ว - กลับบ้าน")}),
                                                    html!("option", {.attr("value", "7_2").text("Discharge แล้ว - Admit")}),
                                                    html!("option", {.attr("value", "7_3").text("Discharge แล้ว - เสียชีวิต")}),
                                                    html!("option", {.attr("value", "7_4").text("Discharge แล้ว - ส่ง OPD")}),
                                                ])
                                            }),
                                            html!("option", {
                                                .attr("value", "")
                                                .text("ทุกสถานะ")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.er_patient_status.clone(), page.changed.clone()))
                                        // .attr("onchange", "onchangeParameter(event)")
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("hn","HN"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "hn")
                                        .prop_signal("size", app.hosxp_hn_len_signal().map(|n| n.to_owned()))
                                        .apply(mixins::string_value_end(page.hn.clone(), page.changed.clone()))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("qn","QN"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "qn")
                                        .attr("size", "5")
                                        .apply(mixins::string_value_end(page.qn.clone(), page.changed.clone()))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
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
                            // OPD_ER_ORDER_ADD
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .child_signal(page.view_by.signal_cloned().map(clone!(app, page => move |view_by| {
                                    (app.has_permission(Permission::OpdErOrderAdd)
                                        && app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErOrderMaster, false)
                                        && ["doctor", "nurse"].contains(&view_by.as_str())
                                    ).then(|| {
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_R_BLUE)
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#addOpdErOrderModal")
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .text(" เพิ่มใบ Order ใหม่")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.opd_er_order_new_modal.set(Some(OpdErOrderNew::new()));
                                            }))
                                            // .attr("onclick", "onclickAddOpdErOrderMasterButton(event);")
                                        })
                                    })
                                })))
                            })),
                            doms::form_inline_end(clone!(app, page => move |end| { end
                                .child(doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("refresh_interval","รอบการ Update "),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM)
                                            .attr("id", "refresh_interval")
                                            .children([
                                                html!("option", {.attr("value", "0").text("ไม่ต้องทำ")}),
                                                html!("option", {.attr("value", "5").text("5 วินาที")}),
                                                html!("option", {.attr("value", "15").text("15 วินาที")}),
                                                html!("option", {.attr("value", "30").text("30 วินาที")}),
                                                html!("option", {.attr("value", "60").attr("selected", "").text("1 นาที")}),
                                                html!("option", {.attr("value", "180").text("3 นาที")}),
                                                html!("option", {.attr("value", "300").text("5 นาที")}),
                                                html!("option", {.attr("value", "600").text("10 นาที")}),
                                                html!("option", {.attr("value", "900").text("15 นาที")}),
                                                html!("option", {.attr("value", "1200").text("20 นาที")}),
                                                html!("option", {.attr("value", "1800").text("30 นาที")}),
                                                html!("option", {.attr("value", "2700").text("45 นาที")}),
                                                html!("option", {.attr("value", "3600").text("1 ชั่วโมง")}),
                                            ])
                                            .prop_signal("value", app.monitor_refresh_interval.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    app.monitor_refresh_interval.set_neq(element.value());
                                                    app.to_local_storage();
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }),
                                        html!("label", {
                                            .class("input-group-text")
                                            .child(doms::timer_svg(page.timer_second.clone()))
                                        }),
                                    ])
                                })))
                            })),
                        ])
                    })),
                    html!("div", {
                        .class("col-sm")
                        .child(doms::badge_info_center("โปรแกรมจะแสดงเฉพาะ 100 รายการล่าสุด"))
                        .child_signal(page.search_result.signal_vec_cloned().len().map(|i| {
                            Some(doms::badge_count_with_limit(i, 100))
                        }))
                    }),
                ])
            })))
            .child_signal(app.is_wide_screen_card_or_table().map(clone!(app, page => move |is_wide_card| {
                Some(match is_wide_card {
                    // NOT wide screen
                    None => {
                        html!("div", {
                            .class(class::ROW_COL_RESP4_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app, page => move |row| {
                                render_card(row, page.view_by.clone(), app.clone())
                            })))
                        })
                    }
                    // wide screen card
                    Some(true) => {
                        html!("div", {
                            .class(class::ROW_COL5_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app, page => move |row| {
                                render_card(row, page.view_by.clone(), app.clone())
                            })))
                        })
                    }
                    // wide screen table
                    Some(false) => {
                        doms::table_responsive(class::TABLE_STRIP, clone!(app, page => move |table| { table
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .class("text-center")
                                        .children([
                                            html!("th", {.attr("scope", "col").text("#")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("เตียง")
                                                .apply(Self::sortable_mixin(SortBy::BedNo, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("เวลาส่งตรวจ")
                                                .apply(Self::sortable_mixin(SortBy::VisitDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("เวลาที่มาถึง")
                                                .apply(Self::sortable_mixin(SortBy::InitDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("HN")
                                                .apply(Self::sortable_mixin(SortBy::Hn, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("QN")
                                                .apply(Self::sortable_mixin(SortBy::Qn, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("ชื่อ-นามสกุล")
                                                .apply(Self::sortable_mixin(SortBy::Name, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("อายุ")
                                                .apply(Self::sortable_mixin(SortBy::Age, page.clone()))
                                            }),
                                            html!("th", {.attr("scope", "col").text("แพทย์เจ้าของไข้")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("เวลาล่าสุด").child(html!("br")).text("Order")
                                                .apply(Self::sortable_mixin(SortBy::MaxOrderDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .attr("scope", "col").text("เวลาล่าสุด").child(html!("br")).text("Vital Sign")
                                                .apply(Self::sortable_mixin(SortBy::MaxVsDateTime, page.clone()))
                                            }),
                                            html!("th", {.attr("scope", "col").style("min-width","100px")
                                                .text("EWS/qSOFA/SIRS")
                                                // .text(&app.scores_table_header())
                                            }),
                                            html!("th", {.attr("scope", "col").text("สถานะ")}),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    .children_signal_vec(page.search_result.signal_vec_cloned().enumerate().map(clone!(app, page => move |(i,row)| {
                                        render_table(i.get().unwrap_or_default(), row, page.view_by.clone(), app.clone())
                                    })))
                                }),
                            ])
                        }))
                    }
                })
            })))
            .child(html!("div", {
                .class("modal")
                .attr("id", "addOpdErOrderModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.opd_er_order_new_modal.signal_cloned().map(clone!(app, page => move |opt| {
                    opt.as_ref().map(clone!(app, page => move |modal| {
                        OpdErOrderNew::render(modal.clone(), page.view_by.clone(), page.opd_er_order_new_modal.clone(), page.changed.clone(), app)
                    })).or(Some(blank_modal()))
                })))
            }))
        })
    }
}

fn render_card(row: Rc<OpdErOrderMasterList>, view_by: Mutable<String>, app: Rc<App>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();

    let all_order_doctor_name_dom = html!("div", {
        .style("min-width","40%")
        .children([
            html!("span", {
                .class(class::BADGE_CYAN)
                .style("cursor","default")
                .text("แพทย์เจ้าของไข้")
            }),
            html!("div", {
                .class("small")
                .text(&row.all_order_doctor_name.clone().map(|docs| {
                    docs.replace('\n',", ")
                }).unwrap_or_default())
            }),
        ])
    });

    let (vs_datetime_opt, ews_dom, qsofa_dom, sirs_dom) = doms::badge_scores_and_vs_datetime(&Scores::from_concat(&row.ews_concat, row.birthday, app.state()));

    let main_route = Route::OpdErMain {
        view_by: view_by.get_cloned(),
        opd_er_order_master_id: row.opd_er_order_master_id,
        tab: String::from("order"),
        id: 0,
    };
    let allow_main_route = main_route.has_permission(app.state());

    html!("div", {
        .class("col")
        .child(html!("div", {
            .class(class::CARD_SHADOW)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_P2)
                    .children([
                        html!("div", {
                            .class(class::BADGE_TB_C)
                            .style("cursor","default")
                            .style("font-size","100%")
                            .style("width","88px")
                            .apply(|dom| {
                                if let Some(bed_type_color) = &row.bed_type_color {
                                    dom.style("background-color", bed_type_color)
                                } else {
                                    dom
                                }
                            })
                            .text(&[row.bed_type_name.clone().unwrap_or_default(), row.display_bedno.clone().unwrap_or_default()].join(" "))
                        }),
                        html!("span", {
                            .class("float-end")
                            .children([
                                html!("span", {.class(class::SMALL_R2).text(&["HN: ", &row.hn.clone().unwrap_or_default()].concat())}),
                                html!("span", {.class(class::SMALL_R2).text(&["VN: ", &row.vn.clone().unwrap_or_default()].concat())}),
                            ])
                            .apply(|dom| {
                                if let Some(qn) = row.oqueue {
                                    dom.child(html!("span", {.class(class::SMALL_R2).text(&["QN: ", &qn.to_string()].concat())}))
                                } else {
                                    dom
                                }
                            })
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::CARD_BODY_P2)
                    .apply(|dom| {
                        if row.count_stat_order_nurse_not_accept > 0 {
                            dom.class("bg-danger-subtle")
                        } else if row.count_discharge_order > 0 {
                            dom.class("bg-success-subtle")
                        } else if row.count_nurse_not_accept > 0 {
                            dom.class("bg-warning-subtle")
                        } else {
                            dom
                        }
                    })
                    .children([
                        html!("div", {
                            .class("d-flex")
                            .children([
                                html!("div", {
                                    .apply(|dom| {
                                        let image_dom = doms::patient_image(&row.hn, "90px");
                                        if allow_main_route {
                                            dom.child(link!(main_route.string(), {
                                                .child(image_dom)
                                            }))
                                        } else {
                                            dom.child(image_dom)
                                        }
                                    })
                                }),
                                html!("div", {
                                    .class("ps-2")
                                    .style("width","calc(100% - 128px)")
                                    .apply(|dom| {
                                        if allow_main_route {
                                            dom.child(link!(main_route.string(), {
                                                .class(class::TRUNC_BOLD)
                                                .text(&row.ptname.clone().unwrap_or_default())
                                            }))
                                        } else {
                                            dom.child(html!("span", {
                                                .class(class::TRUNC_BOLD)
                                                .text(&row.ptname.clone().unwrap_or_default())
                                            }))
                                        }
                                    })
                                    .children([
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .child(html!("span", {
                                                .class("me-1")
                                                .text(&row.sex_name.clone().unwrap_or_default())
                                            }))
                                            .apply_if(age_y > 0, |dom| {
                                                dom.text(&[&age_y.to_string(), " ปี"].concat())
                                            })
                                            .apply_if(age_y == 0 && age_m > 0, |dom| {
                                                dom.text(&[&age_m.to_string(), " เดือน ", &age_d.to_string(), " วัน"].concat())
                                            })
                                            .apply_if(age_y == 0 && age_m == 0, |dom| {
                                                dom.text(&[&age_d.to_string(), " วัน"].concat())
                                            })
                                            .child(html!("span", {
                                                .text(" ")
                                                .text(&row.rtname.clone().unwrap_or(String::from("ไม่ระบุ")))
                                            }))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("Visit ")
                                            .text(&row.vstdate_time.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่ระบุ")))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("Order ")
                                            .apply_if(row.max_order_date_time.is_none(), |dom| dom.class("text-danger"))
                                            .text(&row.max_order_date_time.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                            .apply_if(row.count_stat_order_nurse_not_accept > 0, |dom| {
                                                dom.child(html!("span", {
                                                    .class(class::BADGE_RED_R)
                                                    .style("cursor","default")
                                                    .text("STAT")
                                                }))
                                            })
                                            .apply_if(row.count_nurse_not_accept > 0, |dom| {
                                                dom.child(html!("span", {
                                                    .class(class::BADGE_GOLD_R)
                                                    .style("cursor","default")
                                                    .text("ยังไม่รับ")
                                                }))
                                            })
                                            .apply_if(row.count_discharge_order > 0, |dom| {
                                                dom.child(html!("span", {
                                                    .class(class::BADGE_CYAN_R)
                                                    .style("cursor","default")
                                                    .text("D/C")
                                                }))
                                            })
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("VS ")
                                            .apply_if(vs_datetime_opt.is_none(), |dom| dom.class("text-danger"))
                                            .text(&vs_datetime_opt.map(|vs_dt| datetime_th_relative(&vs_dt)).unwrap_or(String::from("ไม่มี")))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("สถานะ ")
                                            .text(&row.er_patient_status_name.clone().unwrap_or(String::from("ไม่ระบุ")))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::FLEX_COL)
                                    .child(ews_dom).child(qsofa_dom).child(sirs_dom)
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("d-flex")
                            .apply_if(row.all_order_doctor_name.is_some(), |dom| {
                                dom.child(all_order_doctor_name_dom)
                            })
                        }),
                    ])
                }),
            ])
        }))
    })
}

// opd-er-order-list.php/getOpdErOrderMasterDataForTable()
fn render_table(i: usize, row: Rc<OpdErOrderMasterList>, view_by: Mutable<String>, app: Rc<App>) -> Dom {
    let (vs_datetime_opt, ews_dom, qsofa_dom, sirs_dom) = doms::badge_scores_and_vs_datetime(&Scores::from_concat(&row.ews_concat, row.birthday, app.state()));
    let row_class = if row.count_stat_order_nurse_not_accept > 0 {
        "table-danger"
    } else if row.count_discharge_order > 0 {
        "table-success"
    } else if row.count_nurse_not_accept > 0 {
        "table-warning"
    } else {
        ""
    };

    let age = if row.vn.is_some() {
        let age_y = row.age_y.unwrap_or_default();
        if age_y > 0 {
            [&age_y.to_string(), " ปี"].concat()
        } else {
            let age_m = row.age_m.unwrap_or_default();
            let day = [&row.age_y.unwrap_or_default().to_string(), " วัน"].concat();
            if age_m > 0 { [&age_m.to_string(), " เดือน", &day].concat() } else { day }
        }
    } else {
        String::new()
    };

    html!("tr", {
        .apply_if(view_by.lock_ref().as_str() == "nurse" && !row_class.is_empty(), |dom| dom.class(row_class))
        .children([
            html!("td", {.text(&(i + 1).to_string())}),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class(class::BADGE_TB_C)
                    .style("cursor","default")
                    .style("font-size","100%")
                    .style("width","88px")
                    .apply(|dom| {
                        if let Some(bed_type_color) = &row.bed_type_color {
                            dom.style("background-color", bed_type_color)
                        } else {
                            dom
                        }
                    })
                    .text(&[row.bed_type_name.clone().unwrap_or_default(), row.display_bedno.clone().unwrap_or_default()].join(" "))
                }))
            }),
            html!("td", {
                .child(html!("span", {
                    .style("white-space","pre-wrap")
                    .apply_if(row.vn.is_some(), |dom| dom.text(&datetime_th_opt_relative(&row.vstdate_time)))
                }))
            }),
            html!("td", {
                .child(html!("span", {
                    .style("white-space","pre-wrap")
                    .text(&datetime_th_opt_relative(&row.order_date_time))
                }))
            }),
            html!("td", {
                .child(html!("span", {
                    .attr("title", &["HN:",&row.hn.clone().unwrap_or_default()," VN:",&row.vn.clone().unwrap_or_default()," AN:",&row.an.clone().unwrap_or_default()].concat())
                    .text(&row.hn.clone().unwrap_or_default())
                }))
            }),
            html!("td", {
                .style("white-space","pre-wrap")
                .apply_if(row.vn.is_some(), |dom| dom.text(&row.oqueue.unwrap_or_default().to_string()))
            }),
            html!("td", {
                .attr("title", &[row.ptname.clone().unwrap_or_default(), row.note.clone().unwrap_or_default()].join("\n"))
                .children([
                    html!("div", {
                        .class("fw-bold")
                        .text(&row.ptname.clone().unwrap_or_default())
                    }),
                    html!("div", {
                        .class("text-truncate")
                        .style("white-space","pre-wrap")
                        .text(&row.note.clone().unwrap_or_default())
                    }),
                ])
            }),
            html!("td", {
                .text(&age)
            }),
            html!("td", {
                .child(html!("span", {
                    .class("text-truncate")
                    .style("white-space","pre-wrap")
                    .attr("title", &row.all_order_doctor_name.clone().unwrap_or_default())
                    .text(&row.all_order_doctor_name.clone().unwrap_or_default())
                }))
            }),
            html!("td", {
                .child(html!("span", {
                    .style("white-space","pre-wrap")
                    .text(&datetime_th_opt_relative(&row.max_order_date_time))
                }))
                .apply_if(row.count_stat_order_nurse_not_accept > 0, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_RED_R)
                        .style("cursor","default")
                        .text("STAT")
                    }))
                })
                .apply_if(row.count_nurse_not_accept > 0, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_GOLD_R)
                        .style("cursor","default")
                        .text("ยังไม่รับ")
                    }))
                })
                .apply_if(row.count_discharge_order > 0, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_CYAN_R)
                        .style("cursor","default")
                        .text("D/C")
                    }))
                })
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&vs_datetime_opt.map(|vs_dt| datetime_th_relative(&vs_dt)).unwrap_or_default())
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(ews_dom).child(qsofa_dom).child(sirs_dom)
            }),
            html!("td", {
                .children([
                    html!("span", {.text(&row.er_patient_status_name.clone().unwrap_or_default())}),
                    html!("span", {.text(&row.er_dch_type_name.as_ref().map(|name| [" (",name,")"].concat()).unwrap_or_default())}),
                    html!("span", {
                        .style("white-space","pre-wrap")
                        .text(&[
                            datetime_th_opt_relative(&row.discharge_date_time),
                            app.nurse_shift(&row.discharge_time).map(|shift| [" (",shift.short(),")"].concat()).unwrap_or_default(),
                        ].concat())
                    })
                ])
            }),
        ])
        .apply(|dom| {
            let route = Route::OpdErMain {
                view_by: view_by.get_cloned(),
                opd_er_order_master_id: row.opd_er_order_master_id,
                tab: String::from("order"),
                id: 0,
            };
            if route.has_permission(app.state()) {
                dom.style("cursor","pointer")
                .event(move |_: events::Click| {
                    route.hard_redirect();
                })
                // onclickOpdErOrderMasterTableRow(v.opd_er_order_master_id);
            } else {
                dom
            }

        })
    })
}
