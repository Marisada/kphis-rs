use dominator::{Dom, DomBuilder, clone, events, html, is_window_loaded, link, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTableCellElement};

use kphis_model::{
    ipd::consult::{IpdConsultList, IpdConsultListParams},
    route::Route,
    tab::Tab,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{date_th_opt_relative, datetime_from_opt, datetime_th_opt_relative, datetime_th_relative, time_hm_opt},
    util::{opt_empty_none, str_some},
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    ConsultDateTime,
    BedNo,
    An,
    Hn,
    Name,
    ReplyDateTime,
}

/// - GET `EndPoint::IpdConsult`
#[derive(Clone, Default)]
pub struct IpdConsultListPage {
    view_by: Mutable<String>,

    search_consult_status: Mutable<String>,
    consult_dr_search: Mutable<String>,
    consult_dr_reply_search: Mutable<String>,
    search_consult_emergency: Mutable<String>,
    patient: Mutable<String>,
    search_result: MutableVec<Rc<IpdConsultList>>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,

    changed: Mutable<bool>,
}

impl IpdConsultListPage {
    pub fn new(view_by: String) -> Rc<Self> {
        Rc::new(Self {
            view_by: Mutable::new(view_by),
            search_consult_status: Mutable::new(String::from("N")),
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
                SortBy::An => items.sort_by(|a, b| b.an.cmp(&a.an)),
                SortBy::Hn => items.sort_by(|a, b| b.hn.cmp(&a.hn)),
                SortBy::Name => items.sort_by(|a, b| b.fullname.cmp(&a.fullname)),
                SortBy::ConsultDateTime => items.sort_by(|a, b| datetime_from_opt(b.consult_date, b.consult_time).cmp(&datetime_from_opt(a.consult_date, a.consult_time))),
                SortBy::ReplyDateTime => items.sort_by(|a, b| {
                    b.consult_datetime_update_reply
                        .or(b.consult_datetime_create_reply)
                        .cmp(&a.consult_datetime_update_reply.or(a.consult_datetime_create_reply))
                }),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::BedNo => items.sort_by(|a, b| a.bedno.cmp(&b.bedno)),
                SortBy::An => items.sort_by(|a, b| a.an.cmp(&b.an)),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::Name => items.sort_by(|a, b| a.fullname.cmp(&b.fullname)),
                SortBy::ConsultDateTime => items.sort_by(|a, b| datetime_from_opt(a.consult_date, a.consult_time).cmp(&datetime_from_opt(b.consult_date, b.consult_time))),
                SortBy::ReplyDateTime => items.sort_by(|a, b| {
                    a.consult_datetime_update_reply
                        .or(a.consult_datetime_create_reply)
                        .cmp(&b.consult_datetime_update_reply.or(b.consult_datetime_create_reply))
                }),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
    }

    // ipd-consult-list-table.php
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let request = IpdConsultListParams {
            spclty: str_some(app.spclty_select.get_cloned()),
            search_consult_status: str_some(page.search_consult_status.get_cloned()),
            consult_dr_search: str_some(page.consult_dr_search.get_cloned()),
            consult_dr_reply_search: str_some(page.consult_dr_reply_search.get_cloned()),
            search_consult_emergency: str_some(page.search_consult_emergency.get_cloned()),
            patient: str_some(page.patient.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::IpdConsult`
                match IpdConsultList::call_api_get(&request, app.state()).await {
                    Ok(items) => {
                        let mut lock = page.search_result.lock_mut();
                        lock.clear();
                        lock.extend(items.into_iter().map(Rc::new));
                        page.sorted_by.set(SortBy::ConsultDateTime);
                        page.is_desc.set_neq(false);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    // SessionManager::checkPermission('IPD_DOCTOR_CONSULT','VIEW') && ($view_by == 'nurse' || $view_by == 'doctor')
    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Consult");

        let (all_doctor_select_option, spclty_kphis_select_option, emergency_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|asset| (asset.all_doctor_select_option.clone(), asset.spclty_kphis_select_option.clone(), asset.emergency_select_option.clone()))
            .unwrap_or_default();

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if let Some(elm) = app.get_id("spclty") {
                        NiceSelect::new_default(&elm);
                    }
                    if let Some(elm) = app.get_id("consult_dr_search") {
                        NiceSelect::new_default(&elm);
                    }
                    if let Some(elm) = app.get_id("consult_dr_reply_search") {
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
                    page.changed.set(false);
                }
                async {}
            })))
            .class("container-fluid")
            .child(html!("div", {
                .class(class::ROW_COL_MD12)
                .child(html!("label", {
                    .class(class::FORM_COL_LBL_AUTO)
                    .child(html!("h3", {
                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                        .text(" Consult")
                    }))
                }))
            }))
            .child(doms::alert_row(clone!(app, page => move |alert| { alert
                .children([
                    doms::form_inline(clone!(app, page => move |form| { form
                        .children([
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("spclty","แผนกที่รับ Consult"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "spclty")
                                            .children([
                                                html!("option", {
                                                    .attr("value", "000")
                                                    .text("เลือก")
                                                }),
                                                html!("option", {
                                                    .attr("value", "")
                                                    .text("ทั้งหมด")
                                                }),
                                            ])
                                            .children(spclty_kphis_select_option.iter().map(|option| {
                                                doms::select_option(option, "")
                                            }))
                                            .prop_signal("value", app.spclty_select.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    app.spclty_select.set_neq(element.value());
                                                    app.to_local_storage();
                                                    page.changed.set_neq(true);
                                                }))
                                                //.attr("onchange", "onchange_select_spclty()")
                                            })
                                        }))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(app, page, all_doctor_select_option => move |group| { group
                                .children([
                                    doms::label_group_for("consult_dr_search","แพทย์ผู้รับ Consult"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "consult_dr_search")
                                            .child(html!("option", {
                                                .attr("value", "")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(all_doctor_select_option.iter().map(|option| {
                                                doms::select_option(option, "")
                                            }))
                                            .apply(mixins::string_value_select(page.consult_dr_search.clone(), page.changed.clone()))
                                            // .attr("onchange", "onchange_select_consult_dr_search()")
                                        }))
                                    }),
                                ])
                                .child_signal(page.view_by.signal_cloned().map(|view_by| view_by == "doctor").map(clone!(app, page => move |is_doctor| {
                                    is_doctor.then(|| {
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_GRAY)
                                            .child(html!("i", {.class(class::FA_USER)}))
                                            .event(clone!(app, page => move |_: events::Click| {
                                                let doctor_code = app.doctor_code().unwrap_or_default();
                                                let neq = page.consult_dr_search.lock_ref().as_str() != doctor_code.as_str();
                                                if neq {
                                                    if let Some(elm) = app.get_id("consult_dr_search") {
                                                        NiceSelect::new_default_with_value(&elm, &doctor_code);
                                                    }
                                                    page.consult_dr_search.set_neq(doctor_code);
                                                    page.changed.set_neq(true);
                                                }
                                            }))
                                        })
                                    })
                                })))
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_RED)
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(app, page => move |_: events::Click| {
                                            if !page.consult_dr_search.get_cloned().is_empty() {
                                                page.consult_dr_search.set_neq(String::new());
                                                if let Some(elm) = app.get_id("consult_dr_search") {
                                                    NiceSelect::new_default_with_value(&elm,"");
                                                }
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("consult_dr_reply_search","แพทย์ผู้ตอบ Consult"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "consult_dr_reply_search")
                                            .child(html!("option", {
                                                .attr("value", "")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(all_doctor_select_option.iter().map(|option| {
                                                doms::select_option(option, "")
                                            }))
                                            .apply(mixins::string_value_select(page.consult_dr_reply_search.clone(), page.changed.clone()))
                                            // .attr("onchange", "onchange_select_consult_dr_reply_search()")
                                        }))
                                    }),
                                ])
                                .child_signal(page.view_by.signal_cloned().map(|view_by| view_by == "doctor").map(clone!(app, page => move |is_doctor| {
                                    is_doctor.then(|| {
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_GRAY)
                                            .child(html!("i", {.class(class::FA_USER)}))
                                            .event(clone!(app, page => move |_: events::Click| {
                                                let doctor_code = app.doctor_code().unwrap_or_default();
                                                let neq = page.consult_dr_reply_search.lock_ref().as_str() != doctor_code.as_str();
                                                if neq {
                                                    if let Some(elm) = app.get_id("consult_dr_reply_search") {
                                                        NiceSelect::new_default_with_value(&elm, &doctor_code);
                                                    }
                                                    page.consult_dr_reply_search.set_neq(doctor_code);
                                                    page.changed.set_neq(true);
                                                }
                                            }))
                                        })
                                    })
                                })))
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_RED)
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .event(clone!(app, page => move |_: events::Click| {
                                        let empty_search = page.consult_dr_reply_search.lock_ref().is_empty();
                                        if !empty_search {
                                            page.consult_dr_reply_search.set_neq(String::new());
                                            if let Some(elm) = app.get_id("consult_dr_reply_search") {
                                                NiceSelect::new_default_with_value(&elm,"");
                                            }
                                            page.changed.set_neq(true);
                                        }
                                    }))
                                }))
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("search_consult_status","สถานะ"),
                                    html!("select" => HtmlSelectElement, {
                                        .class("form-select")
                                        .attr("id", "search_consult_status")
                                        .children([
                                            html!("option", {
                                                .attr("value", "")
                                                .text("ทั้งหมด")
                                            }),
                                            html!("option", {
                                                .attr("value", "N")
                                                .attr("selected", "")
                                                .text("ยังไม่ตอบ")
                                            }),
                                            html!("option", {
                                                .attr("value", "Y")
                                                .text("ตอบแล้ว")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.search_consult_status.clone(), page.changed.clone()))
                                        // .attr("onchange", "onchange_select_search_consult_status()")
                                    }),
                                    html!("label", {
                                        .class("input-group-text")
                                        .children([
                                            html!("i", {.class(class::FA_HOURGLASS_GOLD).class("me-2")}),
                                            text("ยังไม่ตอบ"),
                                            html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).class("mx-2")}),
                                            text("ตอบแล้ว")
                                        ])
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("search_consult_emergency","เร่งด่วน"),
                                    html!("select" => HtmlSelectElement, {
                                        .class("form-select")
                                        .attr("id", "search_consult_emergency")
                                        .child(html!("option", {
                                            .attr("value", "")
                                            .text("ทั้งหมด")
                                        }))
                                        .children(emergency_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(page.search_consult_emergency.clone(), page.changed.clone()))
                                        // .attr("onchange", "onchange_select_consult_emergency_search()")
                                    }),
                                    html!("label", {
                                        .class("input-group-text")
                                        .children([
                                            html!("i", {.class(class::FA_CIRCLE_GRAY).class("me-2")}),
                                            text("ไม่ด่วน"),
                                            html!("i", {.class(class::FA_CIRCLE_RED).class("mx-2")}),
                                            text("ด่วน"),
                                        ])
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("patient","HN, AN, CID, ชื่อ-สกุล"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "patient")
                                        .attr("autocomplete","off")
                                        .apply(mixins::string_value_end(page.patient.clone(), page.changed.clone()))
                                    }),
                                ])
                            })),
                            doms::form_inline_end(clone!(app, page => move |end| { end
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_L_GRAY)
                                        .child(html!("i", {.class(class::FA_SEARCH)}))
                                        .text(" ค้นหา")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.changed.set_neq(true);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_UNDO)}))
                                        .text(" กลับสู่การค้นหาเริ่มต้น")
                                        .event(clone!(app, page => move |_: events::Click| {
                                            app.spclty_select.set_neq(String::from("000"));
                                            app.to_local_storage();
                                            page.search_consult_status.set_neq(String::from("N"));
                                            page.search_consult_emergency.set_neq(String::new());
                                            page.consult_dr_search.set_neq(String::new());
                                            page.consult_dr_reply_search.set_neq(String::new());
                                            page.patient.set_neq(String::new());
                                            if let Some(elm) = app.get_id("spclty") {
                                                NiceSelect::new_default_with_value(&elm, "000");
                                            }
                                            if let Some(elm) = app.get_id("consult_dr_search") {
                                                NiceSelect::new_default(&elm);
                                            }
                                            if let Some(elm) = app.get_id("consult_dr_reply_search") {
                                                NiceSelect::new_default(&elm);
                                            }
                                            page.changed.set_neq(true);
                                        }))
                                    }),
                                    // .attr("onclick", "refrech_consult_search();")
                                ])
                            })),
                        ])
                    })),
                    html!("div", {
                        .class("col-sm")
                        .child(doms::badge_info_center("หากค้นหาด้วย HN, AN, CID, ชื่อ-สกุล โปรแกรมจะแสดงเฉพาะ 200 รายการล่าสุด"))
                        .child_signal(page.search_result.signal_vec_cloned().len().map(|i| {
                            Some(doms::badge_count_with_limit(i, 200))
                        }))
                    }),
                ])
            })))
            .child_signal(app.is_wide_screen_card_or_table().map(clone!(page => move |is_wide_card| {
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
                                            html!("th", {.class("th-sm").attr("scope","col").text("#")}),
                                            html!("th", {.class("th-sm").attr("scope","col").text("ตึกผู้ป่วย")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เตียง")
                                                .apply(Self::sortable_mixin(SortBy::BedNo, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("AN")
                                                .apply(Self::sortable_mixin(SortBy::An, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("HN")
                                                .apply(Self::sortable_mixin(SortBy::Hn, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("ชื่อ - สกุล (อายุ)")
                                                .apply(Self::sortable_mixin(SortBy::Name, page.clone()))
                                            }),
                                            // html!("th", {.class("th-sm").attr("scope","col").text("อายุ")}),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์เจ้าของไข้")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("วันที่ Consult")
                                                .apply(Self::sortable_mixin(SortBy::ConsultDateTime, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("ตอบ")}),
                                            html!("th", {.class("th-sm").attr("scope","col").text("ด่วน")}),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แผนกที่รับ Consult")}),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์ผู้รับ")}),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์ผู้ตอบ")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("วันที่ตอบ")
                                                .apply(Self::sortable_mixin(SortBy::ReplyDateTime, page.clone()))
                                            }),
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
        })
    }
}

fn render_card(row: Rc<IpdConsultList>, view_by: Mutable<String>, app: Rc<App>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();

    let kphis_incharge_doctor_name_dom = html!("div", {
        .style("min-width","40%")
        .children([
            html!("span", {
                .class(class::BADGE_CYAN)
                .style("cursor","default")
                .text("แพทย์เจ้าของไข้")
            }),
            html!("div", {
                .class("small")
                .children(row.kphis_incharge_doctor_name.clone().map(|docs| {
                    docs.split(',').map(|doc| {
                        html!("div", {
                            .class(class::TRUNC_SM)
                            .text(doc)
                        })
                    }).collect::<Vec<Dom>>()
                }).unwrap_or_default())
            }),
        ])
    });

    let consult_reply_name_dom = html!("div", {
        .class("ps-2")
        .style("max-width","60%")
        .children([
            html!("span", {
                .class(class::BADGE_GOLD_L)
                .style("cursor","default")
                .text("แพทย์ผู้ตอบ Consult")
            }),
            html!("div", {
                .class("small")
                .children(row.string_consult_reply_name.clone().map(|reps| {
                    reps.split('|').map(|rep| {
                        html!("div", {
                            .class(class::TRUNC_SM)
                            .text(&rep.split(',').collect::<Vec<&str>>().join(" / "))
                        })
                    }).collect::<Vec<Dom>>()
                }).unwrap_or_default())
            }),
        ])
    });

    let consult_status_dom = row.consult_status.as_ref().and_then(|status| match status.as_str() {
        "Y" => Some(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)})),
        "N" => Some(html!("i", {.class(class::FA_HOURGLASS_GOLD)})),
        _ => None,
    });

    let consult_emergency_dom = row.consult_emergency.as_ref().and_then(|emergency| match emergency.as_str() {
        "1" => Some(html!("span", {.class(class::BOX_ROUND_DARKS_BOLD_R_P1).class("bg-danger").text("เร่งด่วน")})),
        "2" => Some(html!("span", {.class(class::BOX_ROUND_DARKS_BOLD_R_P1).class("bg-secondary").text("ปกติ")})),
        _ => None,
    });

    let main_route = Route::IpdMain {
        view_by: view_by.get_cloned(),
        an: row.an.clone().unwrap_or_default(),
        tab: Tab::Consult.str().to_owned(),
        sub: String::new(),
        id: row.consult_id,
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
                        html!("span", {
                            .class("fw-bold")
                            .text(&[row.ward_name.clone().unwrap_or_default(), row.bedno.clone().unwrap_or_default()].join(" "))
                        }),
                        html!("span", {
                            .class("float-end")
                            .children([
                                html!("span", {.class(class::SMALL_R2).text(&["HN: ", &row.hn.clone().unwrap_or_default()].concat())}),
                                html!("span", {.class(class::SMALL_R2).text(&["AN: ", &row.an.clone().unwrap_or_default()].concat())}),
                            ])
                            .apply(|dom| {
                                if let Some(consult_status) = consult_status_dom {
                                    dom.child(html!("span",{.class(class::SMALL_R2).child(consult_status)}))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(consult_emergency) = consult_emergency_dom {
                                    dom.child(html!("span",{.class(class::SMALL_R2).child(consult_emergency)}))
                                } else {
                                    dom
                                }
                            })
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::CARD_BODY_P2)
                    .children([
                        html!("div", {
                            .class("d-flex")
                            .children([
                                html!("div", {
                                    .apply(|dom| {
                                        let image_dom = doms::patient_image(&opt_empty_none(row.hn.clone()), "90px");
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
                                    .style("width","calc(100% - 90px)")
                                    .apply(|dom| {
                                        if allow_main_route {
                                            dom.child(link!(main_route.string(), {
                                                .class(class::TRUNC_BOLD)
                                                .text(&row.fullname.clone().unwrap_or_default())
                                            }))
                                        } else {
                                            dom.child(html!("span", {
                                                .class(class::TRUNC_BOLD)
                                                .text(&row.fullname.clone().unwrap_or_default())
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
                                            .text("Admit ")
                                            .text(&row.regdatetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่ระบุ")))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("Consult ")
                                            .text(&[row.spclty_name.clone().unwrap_or_default(), row.consult_doctorcode_mention_name.clone().unwrap_or_default()].join(" "))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text(&[date_th_opt_relative(&row.consult_date), time_hm_opt(&row.consult_time)].join(" "))
                                        }),
                                    ])
                                    .apply(|dom| {
                                        if let Some(consult_datetime_update_reply) = &row.consult_datetime_update_reply {
                                            dom.child(html!("div", {
                                                .class(class::SMALL_TRUNC)
                                                .text("เพิ่มเติม ")
                                                .text(&&datetime_th_relative(consult_datetime_update_reply))
                                            }))
                                        } else if let Some(consult_datetime_create_reply) = &row.consult_datetime_create_reply {
                                            dom.child(html!("div", {
                                                .class(class::SMALL_TRUNC)
                                                .text("ตอบ ")
                                                .text(&datetime_th_relative(consult_datetime_create_reply))
                                            }))
                                        } else {
                                            dom
                                        }
                                    })
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("d-flex")
                            .apply_if(row.kphis_incharge_doctor_name.is_some(), |dom| {
                                dom.child(kphis_incharge_doctor_name_dom)
                            })
                            .apply_if(row.string_consult_reply_name.is_some(), |dom| {
                                dom.child(consult_reply_name_dom)
                            })
                        }),
                    ])
                }),
            ])
        }))
    })
}

fn render_table(i: usize, row: Rc<IpdConsultList>, view_by: Mutable<String>, app: Rc<App>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();
    let kphis_incharge_doctor_name_with_html = html!("div", {
        .children(row.kphis_incharge_doctor_name.clone().map(|docs| {
            docs.split(',').map(|doc| {
                html!("div", {
                    .class(class::TRUNC_SM)
                    .style("max-width","130px")
                    .text(doc)
                })
            }).collect::<Vec<Dom>>()
        }).unwrap_or_default())
    });
    let mut html_consult_reply_name = row
        .string_consult_reply_name
        .clone()
        .map(|reps| {
            reps.split(',')
                .map(|rep| {
                    html!("div", {
                        .class(class::TRUNC_SM)
                        .style("max-width","172px")
                        .text(rep)
                    })
                })
                .collect::<Vec<Dom>>()
        })
        .unwrap_or_default();
    let consult_status = row.consult_status.as_ref().and_then(|status| match status.as_str() {
        "Y" => Some(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)})),
        "N" => Some(html!("i", {.class(class::FA_HOURGLASS_GOLD)})),
        _ => None,
    });
    let consult_emergency = row.consult_emergency.as_ref().and_then(|emergency| match emergency.as_str() {
        "1" => Some(html!("i", {.class(class::FA_CIRCLE_RED)})),
        "2" => Some(html!("i", {.class(class::FA_CIRCLE_GRAY)})),
        _ => None,
    });
    let main_route = Route::IpdMain {
        view_by: view_by.get_cloned(),
        an: row.an.clone().unwrap_or_default(),
        tab: Tab::Consult.str().to_owned(),
        sub: String::new(),
        id: row.consult_id,
    };

    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&(i + 1).to_string())}),
            html!("td", {
                .child(html!("div", {
                    .style("word-wrap","break-word")
                    .style("max-width","90px")
                    .text(&row.ward_name.clone().unwrap_or_default())
                }))
            }),
            html!("td", {
                .child(html!("div", {.class("text-truncate").text(&row.bedno.clone().unwrap_or_default())}))
            }),
            html!("td", {
                .child(html!("div", {.class("text-truncate").text(&row.an.clone().unwrap_or_default())}))
            }),
            html!("td", {
                .child(html!("div", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}))
            }),
            html!("td", {
                .attr("title", &row.fullname.clone().unwrap_or_default())
                .apply(|dom| {
                    if main_route.has_permission(app.state()) {
                        dom.child(link!(main_route.string(), {
                            .child(html!("div", {
                                .class(class::TRUNC_BOLD)
                                // .style("max-width","150px")
                                .text(&row.fullname.clone().unwrap_or_default())
                            }))
                        }))
                    } else {
                        dom.child(html!("div", {
                            .class(class::TRUNC_BOLD)
                            // .style("max-width","150px")
                            .text(&row.fullname.clone().unwrap_or_default())
                        }))
                    }
                })
                .children([
                    html!("div", {
                        .class("text-truncate")
                        .apply_if(age_y > 0, |dom| {
                            dom.text(&[&age_y.to_string(), " ปี"].concat())
                        })
                        .apply_if(age_y == 0 && age_m > 0, |dom| {
                            dom.text(&[&age_m.to_string(), " เดือน ", &age_d.to_string(), " วัน"].concat())
                        })
                        .apply_if(age_y == 0 && age_m == 0, |dom| {
                            dom.text(&[&age_d.to_string(), " วัน"].concat())
                        })
                    }),
                ])
            }),
            html!("td", {
                .attr("title", &row.kphis_incharge_doctor_name.clone().unwrap_or_default())
                .child(kphis_incharge_doctor_name_with_html)
            }),
            html!("td", {
                .class("text-center")
                .text(&date_th_opt_relative(&row.consult_date))
                .child(html!("br"))
                .text(&time_hm_opt(&row.consult_time))
            }),
            html!("td", {
                .child(html!("div", {
                    .class("text-center")
                    .apply(|dom| if let Some(child) = consult_status {
                        dom.child(child)
                    } else {
                        dom
                    })
                }))
            }),
            html!("td", {
                .child(html!("div", {
                    .class("text-center")
                    .apply(|dom| if let Some(child) = consult_emergency {
                        dom.child(child)
                    } else {
                        dom
                    })
                }))
            }),
            html!("td", {
                .child(html!("div", {
                    .style("word-wrap","break-word")
                    .style("max-width","95px")
                    .text(&row.spclty_name.clone().unwrap_or_default())
                }))
            }),
            html!("td", {
                .attr("title", &row.consult_doctorcode_mention_name.clone().unwrap_or_default())
                .child(html!("div", {
                    .class(class::TRUNC_SM)
                    .style("max-width","130px")
                    .text(&row.consult_doctorcode_mention_name.clone().unwrap_or_default())
                }))
            }),
            html!("td", {
                .attr("title", &row.string_consult_reply_name.clone().unwrap_or_default())
                .child(html!("div", {
                    .class(class::TRUNC_SM)
                    .style("max-width","130px")
                    .children(html_consult_reply_name.iter_mut())
                }))
            }),
            html!("td", {
                .class("text-center")
                .text(&datetime_th_opt_relative(&row.consult_datetime_update_reply.or(row.consult_datetime_create_reply)))
            }),
        ])
    })
}
