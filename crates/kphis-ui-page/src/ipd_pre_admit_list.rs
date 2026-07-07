use dominator::{Dom, DomBuilder, clone, events, html, is_window_loaded, link, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTableCellElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::{Method, call_api_get_exists_key_id},
    pre_admit::{PreAdmitList, PreAdmitParams, PreAdmitPatch},
    route::Route,
    tab::Tab,
    user::permission::Permission,
};

use kphis_ui_app::App;
use kphis_ui_component::{
    gadget::searchbox::opd_visit::OpdVisitSearchboxCpn,
    modal::{blank_modal, pre_admit_new::PreAdmitNew},
};
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{date_th_opt_relative, datetime_from_opt, datetime_th_opt_relative, datetime_th_relative, time_hm_opt},
    util::str_some,
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    RegDateTime,
    An,
    Hn,
    Name,
    Age,
    MaxOrderDateTime,
}

/// - GET `EndPoint::IpdPreAdmit`
/// - GET `EndPoint::ExistsKeyId` (guarded, remove check-an div)
/// - PATCH `EndPoint::IpdPreAdmit` (guarded, remove action btns and check-an div)
/// - POST `EndPoint::IpdPreAdmit` (PreAdmitNew, guarded, remove 'เพิ่มใบ Order ใหม่' btn)
/// - GET `EndPoint::SearchBoxOpdVisitModeText` (OpdVisitSearchboxCpn, guarded, remove check-an div)
#[derive(Clone, Default)]
pub struct IpdPreAdmitListPage {
    view_by: Mutable<String>,

    status: Mutable<String>,
    all: Mutable<String>,
    doctor_in_charge: Mutable<String>,
    patient: Mutable<String>,
    search_result: MutableVec<Rc<PreAdmitList>>,

    changed: Mutable<bool>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,

    check_an: Mutable<String>,
    check_an_changed: Mutable<bool>,
    revoke_to_vn: Mutable<String>,
    revoke_to_vn_detail: Mutable<String>,
    an_exists: Mutable<i8>, // -1 = not exists, 0 = not check, 1 = exixts
    display_patient_searchbox: Mutable<bool>,

    pre_admit_new_modal: Mutable<Option<Rc<PreAdmitNew>>>,
}

impl IpdPreAdmitListPage {
    pub fn new(view_by: &str) -> Rc<Self> {
        Rc::new(Self {
            view_by: Mutable::new(view_by.to_owned()),
            status: Mutable::new(String::from("pre")),
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
                SortBy::RegDateTime => items.sort_by(|a, b| datetime_from_opt(b.vstdate, b.vsttime).cmp(&datetime_from_opt(a.vstdate, a.vsttime))),
                SortBy::Hn => items.sort_by(|a, b| b.hn.cmp(&a.hn)),
                SortBy::An => items.sort_by(|a, b| b.an.cmp(&a.an)),
                SortBy::Name => items.sort_by(|a, b| b.fullname.cmp(&a.fullname)),
                SortBy::Age => items.sort_by(|a, b| b.age_y.cmp(&a.age_y).then(b.age_m.cmp(&a.age_m)).then(b.age_d.cmp(&a.age_d))),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| b.max_order_datetime.cmp(&a.max_order_datetime)),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::RegDateTime => items.sort_by(|a, b| datetime_from_opt(a.vstdate, a.vsttime).cmp(&datetime_from_opt(b.vstdate, b.vsttime))),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::An => items.sort_by(|a, b| a.an.cmp(&b.an)),
                SortBy::Name => items.sort_by(|a, b| a.fullname.cmp(&b.fullname)),
                SortBy::Age => items.sort_by(|a, b| a.age_y.cmp(&b.age_y).then(a.age_m.cmp(&b.age_m)).then(a.age_d.cmp(&b.age_d))),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| a.max_order_datetime.cmp(&b.max_order_datetime)),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
    }

    // send GET method
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let params = PreAdmitParams {
            status: str_some(page.status.get_cloned()),
            doctor_in_charge: str_some(page.doctor_in_charge.get_cloned()),
            patient: str_some(page.patient.get_cloned()),
            all: str_some(page.all.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::IpdPreAdmit`
                match PreAdmitList::call_api_get(&params, app.state()).await {
                    Ok(items) => {
                        let mut lock = page.search_result.lock_mut();
                        lock.clear();
                        lock.extend(items.into_iter().map(Rc::new));
                        page.sorted_by.set(SortBy::RegDateTime);
                        page.is_desc.set_neq(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    fn patch(patcher: PreAdmitPatch, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                let is_confirmed = if matches!(patcher, PreAdmitPatch::RevokeAn(_) | PreAdmitPatch::RevokeVnAn(_,_)) {
                    app.confirm("ท่านต้องการยกเลิก Admit เฉพาะใน KPHIS หรือไม่ ?").await
                } else {
                    true
                };
                if is_confirmed {
                    // PATCH `EndPoint::IpdPreAdmit`
                    match patcher.call_api_patch(app.state()).await {
                        Ok(responses) => {
                            if !responses.is_empty() {
                                page.an_exists.set_neq(0);
                                page.check_an.set_neq(String::new());
                                page.revoke_to_vn.set_neq(String::new());
                                page.revoke_to_vn_detail.set_neq(String::new());
                                page.changed.set(true);
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    fn any_an_exists(page: Rc<Self>, app: Rc<App>) {
        if let Some(an) = str_some(page.check_an.get_cloned()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("an/", &an, app.state()).await {
                        Ok(response) => {
                            page.an_exists.set_neq(if response {1} else {-1});
                        }
                        Err(e) => {
                            page.an_exists.set_neq(0);
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        } else {
            page.an_exists.set_neq(0);
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Pre-Admit");

        let doctor_select_option = app.app_asset.lock_ref().as_ref().map(|asset| asset.doctor_select_option.clone()).unwrap_or_default();
        let allow_patch = app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdPreAdmit, true);

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if let Some(elm) = app.get_id("doctor_in_charge") {
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
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.check_an_changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::any_an_exists(page.clone(), app.clone());
                    page.check_an_changed.set(false);
                }
                async {}
            })))
            .class("container-fluid")
            .child(doms::alert_row(clone!(app, page => move |alert| {
                alert.children([
                    doms::form_inline(clone!(app, page => move |form| {
                        form.child(doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("status","ประเภท"),
                                html!("div", {
                                    .class(class::FLEX_GROW1)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "status")
                                        .children([
                                            html!("option", {
                                                .attr("value", "pre")
                                                .text("รอ Admit")
                                            }),
                                            html!("option", {
                                                .attr("value", "admited")
                                                .text("Admit แล้ว")
                                            }),
                                            html!("option", {
                                                .attr("value", "revoked")
                                                .text("ยกเลิก Admit")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.status.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        })))
                        .child_signal(page.status.signal_cloned().map(clone!(page => move |stts| {
                            let title_opt = match stts.as_str() {
                                "pre" => Some("รวมมากกว่า 3 วัน"),
                                "admited" => Some("รวมจำหน่ายแล้ว"),
                                _ => None,
                            };
                            title_opt.map(|title| {
                                html!("div", {
                                    .class(class::FORM_CHK_SW)
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .attr("id", "all-toggle")
                                            .attr("role","switch")
                                            .class("form-check-input")
                                            .attr("value", "Y")
                                            .apply(mixins::checkbox_toggle(page.all.clone(), page.changed.clone(), "Y", ""))
                                        }),
                                        doms::label_check_for("all-toggle", title),
                                    ])
                                })
                            })
                        })))
                        .children([
                            // .style("width","350px")
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("doctor_in_charge","แพทย์เจ้าของไข้"),
                                    html!("div", {
                                        .class(class::FLEX_GROW1)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "doctor_in_charge")
                                            .child(html!("option", {
                                                .attr("value","")
                                                .text("ทั้งหมด")
                                            }))
                                            .children(doctor_select_option.iter().map(|option| {
                                                doms::select_option(option, &page.doctor_in_charge.lock_ref())
                                            }))
                                            .apply(mixins::string_value_select(page.doctor_in_charge.clone(), page.changed.clone()))
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_USER)}))
                                        .event(clone!(app, page => move |_: events::Click| {
                                            let doctor_code = app.doctor_code().unwrap_or_default();
                                            let neq = page.doctor_in_charge.lock_ref().as_str() != doctor_code.as_str();
                                            if neq {
                                                if let Some(elm) = app.get_id("doctor_in_charge") {
                                                    NiceSelect::new_default_with_value(&elm, &doctor_code);
                                                }
                                                page.doctor_in_charge.set_neq(doctor_code);
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RED)
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(app, page => move |_: events::Click| {
                                            let no_doctor = page.doctor_in_charge.lock_ref().is_empty();
                                            if !no_doctor {
                                                page.doctor_in_charge.set_neq(String::new());
                                                if let Some(elm) = app.get_id("doctor_in_charge") {
                                                    NiceSelect::new_default_with_value(&elm, "");
                                                }
                                                page.changed.set_neq(true);
                                            }
                                        }))
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
                        ])
                        .apply_if(
                            app.endpoint_is_allow(&Method::GET, &EndPoint::ExistsKeyId, false)
                            && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxOpdVisitModeText, false)
                            && allow_patch,
                        |dom| dom
                            .child(doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .attr("id", "check_an_exists_input_group")
                                .children([
                                    doms::label_group_for("check_an","ยกเลิก AN ใน KPHIS"),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "check_an")
                                        .attr("autocomplete","off")
                                        .style("max-width","100px")
                                        .apply(mixins::string_value_end(page.check_an.clone(), page.check_an_changed.clone()))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .visible_signal(page.an_exists.signal_cloned().map(|an_exists| an_exists == 0))
                                        .child(html!("i", {.class(class::FA_SEARCH)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            page.check_an_changed.set_neq(true);
                                        }))
                                    }),
                                ])
                                .child_signal(page.an_exists.signal_cloned().map(clone!(app, page => move |an_exists| {
                                    match an_exists {
                                        1 => {
                                            Some(html!("button" => HtmlButtonElement, {
                                                .attr("type","button")
                                                .class(class::BTN_SM)
                                                .class_signal("btn-warning", page.revoke_to_vn.signal_cloned().map(|vn| !vn.is_empty()))
                                                .class_signal("btn-secondary", page.revoke_to_vn.signal_cloned().map(|vn| vn.is_empty()))
                                                .text_signal(page.revoke_to_vn_detail.signal_cloned().map(|detail| {
                                                    if detail.is_empty() {
                                                        String::from("ค้นหา Visit")
                                                    } else {
                                                        ["บันทึกเป็น ", &detail].concat()
                                                    }
                                                }))
                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                    if let (Some(an), Some(vn)) = (str_some(page.check_an.get_cloned()), str_some(page.revoke_to_vn.get_cloned())) {
                                                        Self::patch(PreAdmitPatch::RevokeVnAn(vn, an), page.clone(), app.clone());
                                                    } else {
                                                        page.display_patient_searchbox.set_neq(true);
                                                    }
                                                }), app.state()))
                                            }))
                                        }
                                        -1 => {
                                            Some(doms::span_group_text("ไม่พบข้อมูล"))
                                        }
                                        _ => None
                                    }
                                })))
                                .child_signal(page.display_patient_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                                    if show {
                                        app.get_id("check_an_exists_input_group").map(|elm| {
                                            OpdVisitSearchboxCpn::render(
                                                OpdVisitSearchboxCpn::new(),
                                                page.display_patient_searchbox.clone(),
                                                page.revoke_to_vn.clone(),
                                                page.revoke_to_vn_detail.clone(),
                                                elm.get_bounding_client_rect(),
                                                Mutable::new(false),
                                                app.clone(),
                                            )
                                        })
                                    } else {
                                        None
                                    }
                                })))
                            })))
                        )
                        .child_signal(page.view_by.signal_cloned().map(clone!(app, page => move |view_by| {
                            (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreAdmit, true)
                                && ["doctor", "nurse"].contains(&view_by.as_str())
                            ).then(|| {
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_R_BLUE)
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", "#addPreAdmitModal")
                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                    .text(" เพิ่มใบ Order ใหม่")
                                    .event(clone!(page => move |_: events::Click| {
                                        page.pre_admit_new_modal.set(Some(PreAdmitNew::new()));
                                    }))
                                })
                            })
                        })))
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
                                                .class("th-sm").attr("scope","col").text("เวลาที่มาถึง")
                                                .apply(Self::sortable_mixin(SortBy::RegDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("HN")
                                                .apply(Self::sortable_mixin(SortBy::Hn, page.clone()))
                                            }),
                                        ])
                                        .children_signal_vec(page.status.signal_cloned().map(clone!(page => move |stts| {
                                            if stts.as_str() == "admited" {
                                                vec![
                                                    html!("th" => HtmlTableCellElement, {
                                                        .class("th-sm").attr("scope","col").text("AN")
                                                        .apply(Self::sortable_mixin(SortBy::An, page.clone()))
                                                    }),
                                                    html!("th", {.class("th-sm").attr("scope","col").text("แผนก")}),
                                                ]
                                            } else {
                                                Vec::new()
                                            }
                                        })).to_signal_vec())
                                        .children([
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("ชื่อ - สกุล")
                                                .apply(Self::sortable_mixin(SortBy::Name, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("อายุ")
                                                .apply(Self::sortable_mixin(SortBy::Age, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์เจ้าของไข้")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลา Order ล่าสุด")
                                                .apply(Self::sortable_mixin(SortBy::MaxOrderDateTime, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","Med Reconciliation").text("MR")
                                            }),
                                            html!("th", {
                                                .class("th-sm")
                                                .attr("scope","col")
                                                .attr("title","แบบบันทึกการรับใหม่ผู้ป่วยใน")
                                                .text("Hx")
                                                //.child(html!("i", {.class(class::FA_NOTE_MED)}))
                                            }),
                                        ])
                                        .apply_if(
                                            app.has_permission(Permission::IpdNurseMainProgramAccess)
                                            || app.has_permission(Permission::OpdErNurseProgramAccess),
                                        |dom| { dom
                                            .child(html!("th", {
                                                .class("th-sm")
                                                .attr("scope","col")
                                                .attr("title","ตรวจสอบใน HOSxP")
                                                .text("Tools")
                                            }))
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
            .child(html!("div", {
                .class("modal")
                .attr("id", "addPreAdmitModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.pre_admit_new_modal.signal_cloned().map(clone!(app, page => move |opt| {
                    opt.as_ref().map(clone!(app, page => move |modal| {
                        PreAdmitNew::render(modal.clone(), page.view_by.clone(), page.pre_admit_new_modal.clone(), page.changed.clone(), app)
                    })).or(Some(blank_modal()))
                })))
            }))
        })
    }

    fn render_card(row: Rc<PreAdmitList>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_admited = page.status.lock_ref().as_str() == "admited";

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
                    .text(&row.all_order_doctor_name.clone().unwrap_or_default())
                }),
            ])
        });

        let med_rec_dom = if row.mr_unconfirmed_count.unwrap_or_default() > 0 {
            html!("span", {
                .class(class::SMALL_BOLD_C)
                .class("text-danger")
                .style("cursor","help")
                .text("MR")
                .attr("title","มีรายการ MR ที่แพทย์ยังไม่ได้พิจารณา")
            })
        } else if row.mr_confirmed_count.unwrap_or_default() > 0 {
            html!("span", {
                .class(class::SMALL_BOLD_C)
                .class("text-success")
                .style("cursor","help")
                .text("MR")
                .attr("title","แพทย์พิจารณาแล้ว")
            })
        } else {
            Dom::empty()
        };

        let main_route = Route::IpdMain {
            view_by: page.view_by.get_cloned(),
            an: row.an.clone().unwrap_or(row.vn.clone()),
            tab: Tab::Order.str().to_owned(),
            sub: String::new(),
            id: 0,
        };
        let allow_main_route = main_route.has_permission(app.state());
        let allow_patch = app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdPreAdmit, true);

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
                                    html!("span", {.class(class::SMALL_R2).text(&["VN: ", &row.vn].concat())}),
                                ])
                                .apply(|dom| {
                                    if let Some(an) = row.an.as_ref() {
                                        dom.child(html!("span", {.class(class::SMALL_R2).text(&["AN: ", an].concat())}))
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
                                        .style("width","calc(100% - 110px)")
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
                                                .text("Visit ")
                                                .text(&[date_th_opt_relative(&row.vstdate), time_hm_opt(&row.vsttime)].join(" "))
                                            }),
                                            html!("div", {
                                                .class(class::SMALL_TRUNC)
                                                .text("Order ")
                                                .apply_if(row.max_order_datetime.is_none(), |dom| dom.class("text-danger"))
                                                .text(&row.max_order_datetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                            }),
                                            html!("div", {
                                                .apply(|dom| {
                                                    let route = Route::IpdAdmissionNoteDr  {an: row.an.clone().unwrap_or(row.vn.clone())};
                                                    let is_allow = route.has_permission(app.state());
                                                    let child = html!("span", {
                                                        .class(class::BTN_SM_L)
                                                        .apply_if(!is_allow, |d| d.class("disabled"))
                                                        .apply(|d| if row.dr_admission_note_exists {
                                                            d.class("btn-outline-primary")
                                                        } else {
                                                            d.class("btn-outline-danger")
                                                        })
                                                        .text("Hx/PE ")
                                                    });

                                                    if is_allow {
                                                        dom.child(link!(route.string(), {
                                                            .child(child)
                                                        }))
                                                    } else {
                                                        dom.child(child)
                                                    }
                                                })
                                                .apply_if(
                                                    allow_patch
                                                    && (app.has_permission(Permission::IpdNurseMainProgramAccess)
                                                        || app.has_permission(Permission::OpdErNurseProgramAccess)),
                                                |dom| { dom
                                                    .child(html!("button" => HtmlButtonElement, {
                                                        .attr("type","button")
                                                        .attr("title","ตรวจสอบใน HOSxP")
                                                        .class(class::BTN_SM_BLUEO)
                                                        .child(html!("i", {.class(class::FA_WRENCH)}))
                                                        .apply(mixins::click_with_loader_checked(clone!(app, page, row => move || {
                                                            if is_admited {
                                                                if let Some(an) = row.an.clone() {
                                                                    Self::patch(PreAdmitPatch::SyncAn(an), page.clone(), app.clone());
                                                                }
                                                            } else {
                                                                Self::patch(PreAdmitPatch::SyncVn(row.vn.clone()), page.clone(), app.clone());
                                                            }
                                                        }), app.state()))
                                                    }))
                                                    .apply_if(is_admited, |dom| { dom
                                                        .child(html!("button" => HtmlButtonElement, {
                                                            .attr("type","button")
                                                            .attr("title","ยกเลิก Admit")
                                                            .class(class::BTN_SM_R_RED)
                                                            .child(html!("i", {.class(class::FA_X)}))
                                                            .apply(mixins::click_with_loader_checked(clone!(app, page, row => move || {
                                                                if let Some(an) = row.an.clone() {
                                                                    Self::patch(PreAdmitPatch::RevokeAn(an), page.clone(), app.clone());
                                                                }
                                                            }), app.state()))
                                                        }))
                                                    })
                                                })
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FLEX_COL)
                                        // .child(ews_dom).child(qsofa_dom).child(sirs_dom)
                                        .child(med_rec_dom)
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

    fn render_table(i: usize, row: Rc<PreAdmitList>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_admited = page.status.lock_ref().as_str() == "admited";
        let allow_patch = app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdPreAdmit, true);
        let age_y = row.age_y.unwrap_or_default();
        let age_m = row.age_m.unwrap_or_default();
        let age_d = row.age_d.unwrap_or_default();
        let all_order_doctor_name_dom = html!("div", {
            .children(row.all_order_doctor_name.clone().map(|docs| {
                docs.split(',').map(|doc| {
                    html!("div", {
                        .class(class::TRUNC_SM)
                        .style("max-width","162px")
                        .text(doc)
                    })
                }).collect::<Vec<Dom>>()
            }).unwrap_or_default())
        });

        html!("tr", {
            .children([
                html!("td", {.class("text-center").text(&(i + 1).to_string())}),
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&[date_th_opt_relative(&row.vstdate),time_hm_opt(&row.vsttime)].join(" "))}))
                }),
                html!("td", {
                    .child(html!("div", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}))
                }),
            ])
            .apply_if(is_admited, |dom| { dom
                .children([
                    html!("td", {
                        .child(html!("div", {.class("text-truncate").text(&row.an.clone().unwrap_or_default())}))
                    }),
                    html!("td", {
                        .child(html!("div", {.class("text-truncate").text(&row.ward_name.clone().unwrap_or_default())}))
                    }),
                ])
            })
            .children([
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
                            an: row.an.clone().unwrap_or(row.vn.clone()),
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
                    .attr("title", &row.all_order_doctor_name.clone().unwrap_or_default())
                    .child(all_order_doctor_name_dom)
                }),
                html!("td", {
                    .child(html!("div", {
                        .text(&datetime_th_opt_relative(&row.max_order_datetime))
                    }))
                }),
                html!("td", {
                    .class("text-center")
                    .child(html!("div", {
                        .apply(|dom| {
                            if row.mr_unconfirmed_count.unwrap_or_default() > 0 {
                                dom.child(html!("i", {
                                    .class(class::FA_CIRCLE_RED)
                                    .attr("title","มีรายการ MR ที่แพทย์ยังไม่ได้พิจารณา")
                                }))
                            } else if row.mr_confirmed_count.unwrap_or_default() > 0 {
                                dom.child(html!("i", {
                                    .class(class::FA_CIRCLE_GREEN)
                                    .attr("title","แพทย์พิจารณาแล้ว")
                                }))
                            } else {
                                dom
                            }
                        })
                    }))
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
                            let route = Route::IpdAdmissionNoteDr {an: row.an.clone().unwrap_or(row.vn.clone())};
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
            ])
            .apply_if(
                allow_patch
                && (app.has_permission(Permission::IpdNurseMainProgramAccess)
                    || app.has_permission(Permission::OpdErNurseProgramAccess)),
            |dom| { dom
                .child(html!("td", {
                    .class(class::TXT_C_P1)
                    .child(html!("button" => HtmlButtonElement, {
                        .attr("type","button")
                        .attr("title","ตรวจสอบใน HOSxP")
                        .class(class::BTN_SM_BLUEO)
                        .child(html!("i", {.class(class::FA_WRENCH)}))
                        .apply(mixins::click_with_loader_checked(clone!(app, page, row => move || {
                            if is_admited {
                                if let Some(an) = row.an.clone() {
                                    Self::patch(PreAdmitPatch::SyncAn(an), page.clone(), app.clone());
                                }
                            } else {
                                Self::patch(PreAdmitPatch::SyncVn(row.vn.clone()), page.clone(), app.clone());
                            }
                        }), app.state()))
                    }))
                    .apply_if(is_admited, |dom| { dom
                        .child(html!("button" => HtmlButtonElement, {
                            .attr("type","button")
                            .attr("title","ยกเลิก Admit")
                            .class(class::BTN_SM_R_RED)
                            .child(html!("i", {.class(class::FA_X)}))
                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                if let Some(an) = row.an.clone() {
                                    Self::patch(PreAdmitPatch::RevokeAn(an), page.clone(), app.clone());
                                }
                            }), app.state()))
                        }))
                    })
                }))
            })
        })
    }
}
