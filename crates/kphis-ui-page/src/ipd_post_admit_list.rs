use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, is_window_loaded, link, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{rc::Rc, str::FromStr};
use strum::IntoEnumIterator;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTableCellElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    ipd::summary::AuditStatus,
    post_admit::{PostAdmitList, PostAdmitParams},
    route::Route,
    tab::Tab,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_component::{gadget::pdf_button::static_pdf_btn_with_modal, modal::ipd_passcode::IpdPasscodeForm};
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, date_th_opt_relative, datetime_from_opt, datetime_th_opt_relative, datetime_th_relative, js_now, time_hm_opt},
    util::str_some,
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    DchDateTime,
    Hn,
    An,
    Name,
    Age,
    MaxOrderDateTime,
    MaxProgressDateTime,
}

/// - GET `EndPoint::IpdPostAdmit`
/// - GET `EndPoint::IpdPasscode` (IpdPasscodeForm, guarded, remove cog btn)
/// - POST `EndPoint::IpdPasscode` (IpdPasscodeForm, guarded, remove cog btn)
#[derive(Clone, Default)]
pub struct IpdPostAdmitListPage {
    view_by: Mutable<String>,

    adm_doctor: Mutable<String>,
    dch_doctor: Mutable<String>,
    patient: Mutable<String>,
    passcode: Mutable<String>,
    search_result: MutableVec<Rc<PostAdmitList>>,

    changed: Mutable<bool>,
    use_date_limit: Mutable<bool>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,
}

impl IpdPostAdmitListPage {
    pub fn new(view_by: &str, app: Rc<App>) -> Rc<Self> {
        let (adm_doctor, dch_doctor) = if view_by == "doctor" {
            if let Some(doctorcode) = app.doctor_code() {
                app.dch_doctor_select.set_neq(doctorcode);
            }
            if app.summary_status_select.get_cloned().is_empty() {
                app.summary_status_select.set(AuditStatus::Null.as_ref().to_string());
            }
            (app.adm_doctor_select.clone(), app.dch_doctor_select.clone())
        } else {
            (Mutable::new(String::new()), Mutable::new(String::new()))
        };
        let use_date_limit = AuditStatus::from_str(&app.summary_status_select.get_cloned())
            .map(|status| status.is_list_with_date_limit())
            .unwrap_or(true);
        Rc::new(Self {
            view_by: Mutable::new(view_by.to_owned()),
            adm_doctor,
            dch_doctor,
            use_date_limit: Mutable::new(use_date_limit),
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
                SortBy::DchDateTime => items.sort_by(|a, b| datetime_from_opt(b.dchdate, b.dchtime).cmp(&datetime_from_opt(a.dchdate, a.dchtime))),
                SortBy::Hn => items.sort_by(|a, b| b.hn.cmp(&a.hn)),
                SortBy::An => items.sort_by(|a, b| b.an.cmp(&a.an)),
                SortBy::Name => items.sort_by(|a, b| b.fullname.cmp(&a.fullname)),
                SortBy::Age => items.sort_by(|a, b| b.age_y.cmp(&a.age_y).then(b.age_m.cmp(&a.age_m)).then(b.age_d.cmp(&a.age_d))),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| b.max_order_datetime.cmp(&a.max_order_datetime)),
                SortBy::MaxProgressDateTime => items.sort_by(|a, b| b.max_progress_note_datetime.cmp(&a.max_progress_note_datetime)),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::DchDateTime => items.sort_by(|a, b| datetime_from_opt(a.dchdate, a.dchtime).cmp(&datetime_from_opt(b.dchdate, b.dchtime))),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::An => items.sort_by(|a, b| a.an.cmp(&b.an)),
                SortBy::Name => items.sort_by(|a, b| a.fullname.cmp(&b.fullname)),
                SortBy::Age => items.sort_by(|a, b| a.age_y.cmp(&b.age_y).then(a.age_m.cmp(&b.age_m)).then(a.age_d.cmp(&b.age_d))),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| a.max_order_datetime.cmp(&b.max_order_datetime)),
                SortBy::MaxProgressDateTime => items.sort_by(|a, b| a.max_progress_note_datetime.cmp(&b.max_progress_note_datetime)),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
    }

    // send GET method
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let (start_dchdate, end_dchdate) = if page.use_date_limit.get() {
            (date_8601(&app.start_dchdate.lock_ref()), date_8601(&app.end_dchdate.lock_ref()))
        } else {
            (None, None)
        };
        let params = PostAdmitParams {
            ward: str_some(app.ward_select.get_cloned()),
            inscl: str_some(app.inscl_select.get_cloned()),
            adm_doctor: str_some(page.adm_doctor.get_cloned()),
            dch_doctor: str_some(page.dch_doctor.get_cloned()),
            patient: str_some(page.patient.get_cloned()),
            passcode: str_some(page.passcode.get_cloned()),
            start_dchdate,
            end_dchdate,
            summary_status: str_some(app.summary_status_select.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::IpdPostAdmit`
                match PostAdmitList::call_api_get(&params, app.state()).await {
                    Ok(items) => {
                        let mut lock = page.search_result.lock_mut();
                        lock.clear();
                        lock.extend(items.into_iter().map(Rc::new));
                        page.sorted_by.set(SortBy::DchDateTime);
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
        app.set_title("KPHIS - Post-Admit");

        let (ward_select_option, inscl_select_option, doctor_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|asset| (asset.ward_select_option.clone(), asset.inscl_select_option.clone(), asset.doctor_select_option.clone()))
            .unwrap_or_default();

        let allow_passcode =
            app.can_change_ward_passcode() && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPasscode, false) && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPasscode, false);

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if let Some(elm) = app.get_id("adm_doctor") {
                        NiceSelect::new_default(&elm);
                    }
                    if let Some(elm) = app.get_id("dch_doctor") {
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
            .child(doms::alert_row(clone!(app, page => move |alert| {
                alert.children([
                    doms::form_inline(clone!(app, page => move |form| { form
                        .children([
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("summary_status","สถานะ"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "summary_status")
                                            .child(html!("option", {
                                                .attr("value","")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(AuditStatus::iter().map(|status| {
                                                html!("option", {.attr("value", status.as_ref()).text(status.status_text())})
                                            }))
                                            .prop_signal("value", app.summary_status_select.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    let value = element.value();
                                                    page.use_date_limit.set_neq(AuditStatus::from_str(&value).map(|status| status.is_list_with_date_limit()).unwrap_or(true));
                                                    app.summary_status_select.set_neq(value);
                                                    app.to_local_storage();
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("ward","สิทธิ์"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "inscl")
                                            .child(html!("option", {
                                                .attr("value","")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(inscl_select_option.iter().map(|option| {
                                                doms::select_option(option, &app.inscl_select.lock_ref())
                                            }))
                                            .prop_signal("value", app.inscl_select.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    app.inscl_select.set_neq(element.value());
                                                    app.to_local_storage();
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("ward","แผนก"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "ward")
                                            .child(html!("option", {
                                                .attr("value","")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(ward_select_option.iter().map(|option| {
                                                doms::select_option(option, &app.ward_select.lock_ref())
                                            }))
                                            .prop_signal("value", app.ward_select.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    app.ward_select.set_neq(element.value());
                                                    if page.view_by.lock_ref().as_str() == "nurse" {
                                                        app.to_local_storage();
                                                    }
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }))
                                    }),
                                ])
                            })),
                        ])
                        .child_signal(page.view_by.signal_cloned().map(clone!(app, page => move |view_by| {
                            (view_by.as_str() != "pharmacist").then(|| {
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("passcode","Passcode"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "passcode")
                                            .attr("autocomplete","off")
                                            .attr("length","4")
                                            .attr("size","4")
                                            .prop_signal("value", page.passcode.signal_cloned())
                                            .with_node!(element => {
                                                .event_with_options(&EventOptions::preventable(), clone!(page, element => move |event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        event.prevent_default();
                                                        page.passcode.set_neq(element.value());
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                                .event(clone!(page => move |_: events::Change| {
                                                    page.passcode.set_neq(element.value());
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }),
                                    ])
                                    .apply_if(allow_passcode, |dom| dom
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_BLUE)
                                            .attr("data-bs-toggle","modal")
                                            .attr("data-bs-target","#passcodeModal")
                                            .child(html!("i", {
                                                .class(class::FA_COG)
                                            }))
                                        }))
                                    )
                                }))
                            })
                        })))
                        .children([
                            // .style("width","350px")
                            doms::form_inline_group_sm(clone!(app, page, doctor_select_option => move |group| { group
                                .children([
                                    doms::label_group_for("adm_doctor","แพทย์ผู้ Admit"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "adm_doctor")
                                            .child(html!("option", {
                                                .attr("value","")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(doctor_select_option.iter().map(|option| {
                                                doms::select_option(option, &page.adm_doctor.lock_ref())
                                            }))
                                            //.apply(mixins::string_value_select(page.adm_doctor.clone(), page.changed.clone()))
                                            .prop_signal("value", page.adm_doctor.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    page.adm_doctor.set_neq(element.value());
                                                    if page.view_by.lock_ref().as_str() == "doctor" {
                                                        app.to_local_storage();
                                                    }
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_USER)}))
                                        .event(clone!(app, page => move |_: events::Click| {
                                            let doctor_code = app.doctor_code().unwrap_or_default();
                                            let neq = page.adm_doctor.lock_ref().as_str() != doctor_code.as_str();
                                            if neq {
                                                if let Some(elm) = app.get_id("adm_doctor") {
                                                    NiceSelect::new_default_with_value(&elm, &doctor_code);
                                                }
                                                page.adm_doctor.set_neq(doctor_code);
                                                if page.view_by.lock_ref().as_str() == "doctor" {
                                                    app.to_local_storage();
                                                }
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RED)
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(app, page => move |_: events::Click| {
                                            let no_doctor = page.adm_doctor.lock_ref().is_empty();
                                            if !no_doctor {
                                                if let Some(elm) = app.get_id("adm_doctor") {
                                                    NiceSelect::new_default_with_value(&elm,"");
                                                }
                                                page.adm_doctor.set_neq(String::new());
                                                if page.view_by.lock_ref().as_str() == "doctor" {
                                                    app.to_local_storage();
                                                }
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                    }),
                                ])
                            })),
                            // .style("width","350px")
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("dch_doctor","แพทย์ผู้ Discharge"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "dch_doctor")
                                            .child(html!("option", {
                                                .attr("value","")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(doctor_select_option.iter().map(|option| {
                                                doms::select_option(option, &page.dch_doctor.lock_ref())
                                            }))
                                            //.apply(mixins::string_value_select(page.dch_doctor.clone(), page.changed.clone()))
                                            .prop_signal("value", page.dch_doctor.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    page.dch_doctor.set_neq(element.value());
                                                    if page.view_by.lock_ref().as_str() == "doctor" {
                                                        app.to_local_storage();
                                                    }
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_USER)}))
                                        .event(clone!(app, page => move |_: events::Click| {
                                            let doctor_code = app.doctor_code().unwrap_or_default();
                                            let neq = page.dch_doctor.lock_ref().as_str() != doctor_code.as_str();
                                            if neq {
                                                if let Some(elm) = app.get_id("dch_doctor") {
                                                    NiceSelect::new_default_with_value(&elm, &doctor_code);
                                                }
                                                page.dch_doctor.set_neq(doctor_code);
                                                if page.view_by.lock_ref().as_str() == "doctor" {
                                                    app.to_local_storage();
                                                }
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RED)
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(app, page => move |_: events::Click| {
                                            let no_doctor = page.dch_doctor.lock_ref().is_empty();
                                            if !no_doctor {
                                                if let Some(elm) = app.get_id("dch_doctor") {
                                                    NiceSelect::new_default_with_value(&elm,"");
                                                }
                                                page.dch_doctor.set_neq(String::new());
                                                if page.view_by.lock_ref().as_str() == "doctor" {
                                                    app.to_local_storage();
                                                }
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    html!("div", {
                                        .class("input-group-text")
                                        .attr("title","แสดงรายการจำหน่ายทุกวัน")
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .attr("id", "show-all-date-checkbox")
                                                .with_node!(element => {
                                                    .future(page.use_date_limit.signal().for_each(clone!(element => move |v| {
                                                        element.set_checked(!v);
                                                        async {}
                                                    })))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.use_date_limit.set(!page.use_date_limit.get());
                                                        page.changed.set_neq(true);
                                                    }))
                                                })
                                            }),
                                            html!("label", {
                                                .class(class::FORM_CHK_LBL_R)
                                                .attr("for", "show-all-order-date-checkbox")
                                                .style("user-select","none")
                                                .text("ทุกวัน")
                                            })
                                        ])
                                    }),
                                    doms::label_group_for("start_dchdate","จำหน่ายวันที่"),
                                    doms::date_picker(
                                        app.start_dchdate.clone(),
                                        page.changed.clone(), not(page.use_date_limit.signal()), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "start_dchdate"),
                                        |s| s, always(None),
                                    ),
                                    doms::label_group_for("end_dchdate","ถึงวันที่"),
                                    doms::date_picker(
                                        app.end_dchdate.clone(),
                                        page.changed.clone(), not(page.use_date_limit.signal()), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "end_dchdate"),
                                        |s| s, always(None),
                                    ),
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
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_SEARCH)}))
                                        .text(" ค้นหา")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.changed.set_neq(true);
                                        }))
                                    }),
                                ])
                            })),
                            static_pdf_btn_with_modal(
                                "Print",
                                "รายงานจำหน่ายผู้ป่วยใน",
                                include_str!("../../../volume/pwa/templates/statics/ipd-post-admit-from-screen.typ"),
                                serde_json::json!({
                                    "rows": page.search_result.lock_ref().to_vec(),
                                }).to_string(),
                                app.clone(),
                            ),
                        ])
                    })),
                    html!("div", {
                        .class("col-sm")
                        .child(doms::badge_info_center("โปรแกรมจะแสดงเฉพาะ 500 รายการแรกเท่านั้น"))
                        .child_signal(page.search_result.signal_vec_cloned().len().map(|i| {
                            Some(doms::badge_count_with_limit(i, 500))
                        }))
                    }),
                ])
            })))
            // /kphis-config-ipd-ward-passcode.php
            .apply_if(allow_passcode, |dom| dom
                .child(html!("div", {
                    .class("modal")
                    .attr("id", "passcodeModal")
                    .attr("role","dialog")
                    .attr("tabindex", "-1")
                    .child(IpdPasscodeForm::render(IpdPasscodeForm::new(), app.clone()))
                }))
            )
            .child_signal(app.is_wide_screen_card_or_table().map(clone!(app, page => move |is_wide_card| {
                Some(match is_wide_card {
                    // NOT wide screen
                    None => {
                        html!("div", {
                            .class(class::ROW_COL_RESP4_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app, page => move |row| {
                                Self::render_card(row, page.clone(), app.clone())
                            })))
                        })
                    }
                    // wide screen card
                    Some(true) => {
                        html!("div", {
                            .class(class::ROW_COL5_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app, page => move |row| {
                                Self::render_card(row, page.clone(), app.clone())
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
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("HN")
                                                .apply(Self::sortable_mixin(SortBy::Hn, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("AN")
                                                .apply(Self::sortable_mixin(SortBy::An, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("Ward")}),
                                            // html!("th", {.class("th-sm").attr("scope","col").text("เวลา Admit")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("ชื่อ - สกุล")
                                                .apply(Self::sortable_mixin(SortBy::Name, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("อายุ")
                                                .apply(Self::sortable_mixin(SortBy::Age, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("สถานะ D/C")}),
                                        ])
                                        .apply(|dom| {
                                            if page.view_by.lock_ref().as_str() == "other" { dom
                                                .children([
                                                    html!("th", {.class("th-sm").attr("scope","col").text("สิทธิ์การรักษา")}),
                                                    html!("th", {.class("th-sm").attr("scope","col").text("จำนวนวันหลัง D/C")}),
                                                ])
                                            } else { dom
                                                .children([
                                                    html!("th" => HtmlTableCellElement, {
                                                        .class("th-sm").attr("scope","col").text("เวลา D/C")
                                                        .apply(Self::sortable_mixin(SortBy::DchDateTime, page.clone()))
                                                    }),
                                                    html!("th" => HtmlTableCellElement, {
                                                        .class("th-sm").attr("scope","col").text("เวลา Order ล่าสุด")
                                                        .apply(Self::sortable_mixin(SortBy::MaxOrderDateTime, page.clone()))
                                                    }),
                                                    html!("th" => HtmlTableCellElement, {
                                                        .class("th-sm").attr("scope","col").text("เวลา Progress ล่าสุด")
                                                        .apply(Self::sortable_mixin(SortBy::MaxProgressDateTime, page.clone()))
                                                    }),
                                                ])
                                            }
                                        })
                                        .children([
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์ผู้ Admit")}),
                                            html!("th", {
                                                .class("th-sm")
                                                .attr("scope","col")
                                                .attr("title","แบบบันทึกการรับใหม่ผู้ป่วยใน")
                                                .text("Hx")
                                                //.child(html!("i", {.class(class::FA_NOTE_MED)}))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์ผู้ D/C")}),
                                            html!("th", {
                                                .class("th-sm")
                                                .attr("scope","col")
                                                .attr("title","IPD SUMMARY FORM")
                                                .text("SUM")
                                                //.child(html!("i", {.class(class::FA_FILE_MONEY)}))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("สถานะ")}),
                                        ])
                                        .apply_if(app.has_permission(Permission::DataTypeAuditorUse), |dom| { dom
                                            .children([
                                                html!("th", {
                                                    .class("th-sm")
                                                    .attr("scope","col")
                                                    .attr("title","Summary / Coding Audit ผู้ป่วยใน")
                                                    .child(html!("i", {.class(class::FA_SPELL_CHECK)}))
                                                }),
                                                html!("th", {
                                                    .class("th-sm")
                                                    .attr("scope","col")
                                                    .attr("title","ทบทวนเวชระเบียนผู้ป่วยใน")
                                                    .child(html!("i", {.class(class::FA_LIST_CHECK)}))
                                                }),
                                            ])
                                        })
                                    }))
                                }),
                                html!("tbody", {
                                    .children_signal_vec(page.search_result.signal_vec_cloned().enumerate().map(move |(i,row)| {
                                        Self::render_table(i.get().unwrap_or_default(), row, page.clone(), app.clone())
                                    }))
                                }),
                            ])
                        }))
                    }
                })
            })))
        })
    }

    fn render_card(row: Rc<PostAdmitList>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let age_y = row.age_y.unwrap_or_default();
        let age_m = row.age_m.unwrap_or_default();
        let age_d = row.age_d.unwrap_or_default();

        let main_route = Route::IpdMain {
            view_by: page.view_by.get_cloned(),
            an: row.an.clone(),
            tab: Tab::Order.str().to_owned(),
            sub: String::new(),
            id: 0,
        };
        let allow_main_route = main_route.has_permission(app.state());

        let status = if let Some(s) = row.summary_status.as_ref() {
            AuditStatus::from_str(s).unwrap_or_default()
        } else {
            match (row.attending_doctor_exists, row.approve_doctor_exists) {
                (true, true) => AuditStatus::Code,
                (true, false) => AuditStatus::Approve,
                (false, _) => AuditStatus::Null,
            }
        };

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
                                .text(&row.ward_name.clone().unwrap_or_default())
                            }),
                            html!("span", {
                                .class("float-end")
                                .children([
                                    html!("span", {.class(class::SMALL_R2).text(&["HN: ", &row.hn.clone().unwrap_or_default()].concat())}),
                                    html!("span", {.class(class::SMALL_R2).text(&["AN: ", &row.an].concat())}),
                                ])
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
                                        .class(class::FLEX_COL)
                                        .style("width","90px")
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
                                        .apply(|dom| {
                                            if let Some(dchstts) = &row.dchstts {
                                                dom.child(doms::badge_dchstts(dchstts))
                                            } else {
                                                dom
                                            }
                                        })
                                        .apply(|dom| {
                                            if let Some(dchtype) = &row.dchtype {
                                                dom.child(doms::badge_dchtype(dchtype))
                                            } else {
                                                dom
                                            }
                                        })
                                        .child(html!("span", {
                                            .class(class::BADGE_TRUNC)
                                            .class(status.color_class())
                                            .style("margin-top","1px")
                                            .style("cursor","default")
                                            //.style("padding-top","5px")
                                            .text(status.status_text())
                                        }))
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
                                                .text("Order ")
                                                .apply_if(row.max_order_datetime.is_none(), |dom| dom.class("text-danger"))
                                                .text(&row.max_order_datetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                            }),
                                            html!("div", {
                                                .class(class::SMALL_TRUNC)
                                                .text("Progress Note ")
                                                .apply_if(row.max_progress_note_datetime.is_none(), |dom| dom.class("text-danger"))
                                                .text(&row.max_progress_note_datetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                            }),
                                            html!("div", {
                                                .class(class::SMALL_TRUNC)
                                                .text("Admit ")
                                                .text(&[date_th_opt_relative(&row.regdate), time_hm_opt(&row.regtime), row.admdoctor_name.clone().unwrap_or_default()].join(" "))
                                            }),
                                            html!("div", {
                                                .class(class::SMALL_TRUNC)
                                                .text("D/C ")
                                                .text(&[date_th_opt_relative(&row.dchdate), time_hm_opt(&row.dchtime), row.dchdoctor_name.clone().unwrap_or_default()].join(" "))
                                            }),
                                            html!("div", {
                                                .class("d-flex")
                                                .apply(|dom| {
                                                    let route = Route::IpdAdmissionNoteDr {an: row.an.clone()};
                                                    let is_allow = route.has_permission(app.state());
                                                    let child = html!("span", {
                                                        .class(class::BTN_SM_L)
                                                        .apply_if(!is_allow, |d| d.class("disabled"))
                                                        .apply(|d| if row.dr_admission_note_exists {
                                                            d.class("btn-outline-primary")
                                                        } else {
                                                            d.class("btn-outline-danger")
                                                        })
                                                        .text("Hx/PE")
                                                    });

                                                    if is_allow {
                                                        dom.child(link!(route.string(), {
                                                            .child(child)
                                                        }))
                                                    } else {
                                                        dom.child(child)
                                                    }
                                                })
                                                .apply(|dom| {
                                                    let route = Route::Summary {view_by: page.view_by.get_cloned(), an: row.an.clone()};
                                                    let is_allow = route.has_permission(app.state());
                                                    let child = html!("span", {
                                                        .class(class::BTN_SM_L)
                                                        .apply_if(!is_allow, |d| d.class("disabled"))
                                                        .apply(|d| if row.attending_doctor_exists {
                                                            d.class("btn-outline-primary")
                                                        } else {
                                                            d.class("btn-outline-danger")
                                                        })
                                                        .text("SUM")
                                                    });
                                                    if is_allow {
                                                        dom.child(link!(route.string(), {
                                                            .child(child)
                                                        }))
                                                    } else {
                                                        dom.child(child)
                                                    }
                                                })
                                                .apply(|dom| {
                                                    let route = Route::IpdSummaryAudit {an: row.an.clone()};
                                                    let is_allow = route.has_permission(app.state());
                                                    let child = html!("span", {
                                                        .class(class::BTN_SM_L_BLUEO)
                                                        .class("position-relative")
                                                        .apply_if(!is_allow, |d| d.class("disabled"))
                                                        .apply(|dom| if row.summary_audit_count > 0 {
                                                            dom.class("btn-outline-primary")
                                                        } else {
                                                            dom.class("btn-outline-danger")
                                                        })
                                                        .text("SA/CA")
                                                        .apply(|dom| {
                                                            if let Some(badge) = doms::badge_count_blue(row.summary_audit_count as usize) {
                                                                dom.child(badge)
                                                            } else {
                                                                dom
                                                            }
                                                        })
                                                    });
                                                    if is_allow {
                                                        dom.child(link!(route.string(), {
                                                            .child(child)
                                                        }))
                                                    } else {
                                                        dom.child(child)
                                                    }
                                                })
                                                .apply(|dom| {
                                                    let route = Route::IpdMra {an: row.an.clone()};
                                                    let is_allow = route.has_permission(app.state());
                                                    let child = html!("span", {
                                                        .class(class::BTN_SM_L_BLUEO)
                                                        .class("position-relative")
                                                        .apply_if(!is_allow, |d| d.class("disabled"))
                                                        .apply(|dom| if row.mra_count > 0 {
                                                            dom.class("btn-outline-primary")
                                                        } else {
                                                            dom.class("btn-outline-danger")
                                                        })
                                                        .text("MRA")
                                                        .apply(|dom| {
                                                            if let Some(badge) = doms::badge_count_blue(row.mra_count as usize) {
                                                                dom.child(badge)
                                                            } else {
                                                                dom
                                                            }
                                                        })
                                                    });
                                                    if is_allow {
                                                        dom.child(link!(route.string(), {
                                                            .child(child)
                                                        }))
                                                    } else {
                                                        dom.child(child)
                                                    }
                                                })
                                            }),
                                        ])
                                    }),
                                ])
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    fn render_table(i: usize, row: Rc<PostAdmitList>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let age_y = row.age_y.unwrap_or_default();
        let age_m = row.age_m.unwrap_or_default();
        let age_d = row.age_d.unwrap_or_default();

        let status = if let Some(s) = row.summary_status.as_ref() {
            AuditStatus::from_str(s).unwrap_or_default()
        } else {
            match (row.attending_doctor_exists, row.approve_doctor_exists) {
                (true, true) => AuditStatus::Code,
                (true, false) => AuditStatus::Approve,
                (false, _) => AuditStatus::Null,
            }
        };

        html!("tr", {
            .children([
                html!("td", {.class("text-center").text(&(i + 1).to_string())}),
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}))
                }),
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&row.an.clone())}))
                }),
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&row.ward_name.clone().unwrap_or_default())}))
                }),
                // html!("td", {
                //     .child(html!("div", {.class("text-truncate").text(&[date_th_opt(&row.regdate),time_hm_opt(&row.regtime)].join(" "))}))
                // }),
                html!("td", {
                    .attr("title", &row.fullname.clone().unwrap_or_default())
                    .apply(|dom| {
                        let child = html!("div", {
                            .class(class::TRUNC_BOLD)
                            .style("max-width","200px")
                            .text(&row.fullname.clone().unwrap_or_default())
                        });
                        let route = Route::IpdMain {
                            view_by: page.view_by.get_cloned(),
                            an: row.an.clone(),
                            tab: Tab::Order.str().to_owned(),
                            sub: String::new(),
                            id: 0,
                        };
                        if route.has_permission(app.state()) {
                            dom.child(link!(route.string(), {
                                .child(child)
                            }))
                        } else {
                            dom.child(child)
                        }
                    })
                }),
                html!("td", {
                    .child(html!("div", {
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
                    }))
                }),
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&row.dchtype_name.clone().unwrap_or_default())}))
                }),
            ])
            .apply(|dom| {
                if page.view_by.lock_ref().as_str() == "other" { dom
                    .children([
                        html!("td", {
                            .child(html!("div", {.class("text-truncate").text(&row.rtname.clone().unwrap_or_default())}))
                        }),
                        html!("td", {
                            .child(html!("div", {.class("text-truncate").text(&[&row.dchdate.map(|d| (js_now().date() - d).whole_days().to_string()).unwrap_or_default(), " วัน"].concat())}))
                        }),
                    ])
                } else { dom
                    .children([
                        html!("td", {
                            .child(html!("div", {.class("text-truncate").text(&[date_th_opt_relative(&row.dchdate),time_hm_opt(&row.dchtime)].join(" "))}))
                        }),
                        html!("td", {
                            .child(html!("div", {
                                .text(&datetime_th_opt_relative(&row.max_order_datetime))
                            }))
                        }),
                        html!("td", {
                            .child(html!("div", {
                                .text(&datetime_th_opt_relative(&row.max_progress_note_datetime))
                            }))
                        }),
                    ])
                }
            })
            .children([
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&row.admdoctor_name.clone().unwrap_or_default())}))
                }),
                html!("td", {
                    .class("text-center")
                    .attr("title","แบบบันทึกการรับใหม่ผู้ป่วยใน")
                    .child(html!("div", {
                        .apply(|dom| {
                            let child = html!("i", {
                                .apply(|d| if row.dr_admission_note_exists {
                                    d.class(class::FA_CHECK_GREEN)
                                } else {
                                    d.class(class::FA_X_RED)
                                })
                            });
                            let route = Route::IpdAdmissionNoteDr {an: row.an.clone()};
                            if route.has_permission(app.state()) {
                                dom.child(link!(route.string(), {
                                    .child(child)
                                }))
                            } else {
                                dom.child(child)
                            }
                        })
                    }))
                }),
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&row.dchdoctor_name.clone().unwrap_or_default())}))
                }),
                html!("td", {
                    .class("text-center")
                    .attr("title","IPD SUMMARY FORM")
                    .child(html!("div", {
                        .apply(|dom| {
                            let child = html!("i", {
                                .apply(|d| if row.attending_doctor_exists {
                                    d.class(class::FA_CHECK_GREEN)
                                } else {
                                    d.class(class::FA_X_RED)
                                })
                            });
                            let route = Route::Summary {view_by: page.view_by.get_cloned(), an: row.an.clone()};
                            if route.has_permission(app.state()) {
                                dom.child(link!(route.string(), {
                                    .child(child)
                                }))
                            } else {
                                dom.child(child)
                            }
                        })
                    }))
                }),
                html!("td", {
                    .class(status.color_class())
                    .class("text-center")
                    .apply(|dom| {
                        let route = Route::Summary {view_by: page.view_by.get_cloned(), an: row.an.clone()};
                        if route.has_permission(app.state()) {
                            dom.child(link!(route.string(), {
                                .style("color","inherit")
                                .text(status.status_text())
                            }))
                        } else {
                            dom.text(status.status_text())
                        }
                    })
                }),
            ])
            .apply_if(app.has_permission(Permission::DataTypeAuditorUse), |dom| { dom
                .child(html!("td", {
                    .class(class::TXT_C_P1)
                    .attr("title","IPD SUMMARY/CODING AUDIT")
                    .apply(|dom| {
                        let route = Route::IpdSummaryAudit {an: row.an.clone()};
                        let is_allow = route.has_permission(app.state());
                        let child = html!("span", {
                            .class(class::BTN_SM_L_BLUEO)
                            .class("position-relative")
                            .apply_if(!is_allow, |d| d.class("disabled"))
                            .apply(|d| if row.summary_audit_count > 0 {
                                d.class("btn-outline-primary")
                            } else {
                                d.class("btn-outline-danger")
                            })
                            .text("SA/CA")
                            .apply(|d| {
                                if let Some(badge) = doms::badge_count_blue(row.summary_audit_count as usize) {
                                    d.child(badge)
                                } else {
                                    d
                                }
                            })
                        });
                        if is_allow {
                            dom.child(link!(route.string(), {
                                .child(child)
                            }))
                        } else {
                            dom.child(child)
                        }
                    })
                }))
                .child(html!("td", {
                    .class(class::TXT_C_P1)
                    .attr("title","IPD MEDICAL RECORD AUDIT")
                    .apply(|dom| {
                        let route = Route::IpdMra {an: row.an.clone()};
                        let is_allow = route.has_permission(app.state());
                        let child = html!("span", {
                            .class(class::BTN_SM_L_BLUEO)
                            .class("position-relative")
                            .apply_if(!is_allow, |d| d.class("disabled"))
                            .apply(|d| if row.mra_count > 0 {
                                d.class("btn-outline-primary")
                            } else {
                                d.class("btn-outline-danger")
                            })
                            .text("MRA")
                            .apply(|d| {
                                if let Some(badge) = doms::badge_count_blue(row.mra_count as usize) {
                                    d.child(badge)
                                } else {
                                    d
                                }
                            })
                        });
                        if is_allow {
                            dom.child(link!(route.string(), {
                                .child(child)
                            }))
                        } else {
                            dom.child(child)
                        }
                    })
                }))
            })
        })
    }
}
