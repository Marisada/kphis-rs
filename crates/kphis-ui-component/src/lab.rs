// ipd-nurse-lab.php

use dominator::{Dom, clone, events, html, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
    signal_vec::{SignalVec, SignalVecExt},
};
use std::{cmp::Ordering, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    lab::{LabHead, LabHeadParams, LabItemsGroup},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_th, date_th_opt, datetime_th_opt_relative, datetime_th_relative, js_now, time_hm, time_hm_opt},
    util::{opt_zero_none, str_some, zero_none},
};

use crate::{
    gadget::pdf_button::PdfButtons,
    modal::{blank_modal, lab_history::LabHistory},
};

#[derive(Clone, Default, PartialEq)]
enum LabStatus {
    Confirm,
    Wait,
    #[default]
    All,
}

/// - GET `EndPoint::LabHead`
/// - GET `EndPoint::LabItem` (LabHistory, guarded, remove 'ประวัติ' btn)
/// - POST `EndPoint::LabReadId` (guarded, remove read toggle)
/// - DELETE `EndPoint::LabReadId` (guarded, remove read toggle)
#[derive(Clone, Default)]
pub struct LabCpn {
    patient: Mutable<Option<Rc<PatientInfo>>>,
    hn: Mutable<String>,
    vnan: Mutable<String>,
    loaded_lab_unread_exists_spinner: Option<Mutable<bool>>,

    prev: Mutable<String>,
    prev_changed: Mutable<bool>,
    previous_headers: Mutable<Vec<(String, String)>>,
    previous_detail: Mutable<Vec<Rc<LabHead>>>, // head with details

    loaded_head: Mutable<bool>,
    head_status: Mutable<LabStatus>,
    is_orderby_report: Mutable<bool>,
    lab_heads: Mutable<Vec<Rc<LabHead>>>, // head without details

    lab_order_number: Mutable<i32>,
    loaded_detail: Mutable<bool>,
    lab_detail: Mutable<Option<Rc<LabHead>>>, // head with details
    lab_detail_read: Mutable<String>,         // "Y" or "N"
    lab_detail_read_user: Mutable<String>,
    lab_detail_read_datetime: Mutable<String>,

    lab_history_modal: Mutable<Option<Rc<LabHistory>>>,
}

impl LabCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, hn: Mutable<String>, vnan: Mutable<String>, loaded_lab_unread_exists_spinner: Option<Mutable<bool>>) -> Rc<Self> {
        Rc::new(Self {
            patient,
            hn,
            vnan,
            loaded_lab_unread_exists_spinner,
            prev: Mutable::new(String::from("1")),
            loaded_detail: Mutable::new(true),
            ..Default::default()
        })
    }

    fn sorted_heads(&self) -> impl SignalVec<Item = Rc<LabHead>> + use<> {
        map_ref! {
            let heads = self.lab_heads.signal_cloned(),
            let status = self.head_status.signal_cloned(),
            let by_report = self.is_orderby_report.signal() => {
                let mut filtered: Vec<Rc<LabHead>> = match status {
                    LabStatus::Confirm => heads.iter().filter(|head| head.confirm_report == Some(String::from("Y"))).cloned().collect(),
                    LabStatus::Wait => heads.iter().filter(|head| head.confirm_report == Some(String::from("N"))).cloned().collect(),
                    LabStatus::All => heads.clone(),
                };
                if *by_report {
                    filtered.sort_by(sort_by_report_datetime);
                    filtered.reverse();
                } else {
                    filtered.sort_by(sort_by_order_datetime);
                    filtered.reverse();
                }
                filtered
            }
        }
        .to_signal_vec()
    }

    fn load_head(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = LabHeadParams {
                    hn: Some(page.hn.get_cloned()),
                    only_head: Some(true),
                    ..Default::default()
                };
                // GET `EndPoint::LabHead`
                match LabHead::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.lab_heads.lock_mut();
                        if !lock.is_empty() {
                            lock.clear();
                        }
                        lock.extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    // ipd-nurse-lab-detail.php
    fn load_detail(page: Rc<Self>, app: Rc<App>) {
        if let Some(lab_order_number) = zero_none(page.lab_order_number.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    let prev = page.prev.lock_ref().parse::<i32>().unwrap_or(1);
                    let params = LabHeadParams {
                        hn: str_some(page.hn.get_cloned()),
                        id: zero_none(lab_order_number),
                        prev: Some(prev),
                        with_scan: Some(prev < 2),
                        ..Default::default()
                    };
                    // GET `EndPoint::LabHead`
                    match LabHead::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            let (main_vec, mut previous): (Vec<LabHead>, Vec<LabHead>) = responses.into_iter().partition(|h| h.lab_order_number == lab_order_number);
                            let main_opt = main_vec.first().cloned();
                            page.lab_detail_read.set_neq(main_opt.as_ref().map(|m| m.lab_read_status.clone()).unwrap_or_default());
                            page.lab_detail_read_user.set_neq(main_opt.as_ref().and_then(|m| m.lab_read_user.clone()).unwrap_or_default());
                            page.lab_detail_read_datetime.set_neq(main_opt.as_ref().map(|m| datetime_th_opt_relative(&m.lab_read_datetime)).unwrap_or_default());

                            if let Some(mut main) = main_opt {
                                // 1. sorted head by lab_order_number
                                previous.sort_by(|a,b| b.lab_order_number.cmp(&a.lab_order_number));
                                // 2. create vec of report_date + report_time
                                let headers = previous.iter().map(|p| {
                                    if let (Some(recv_date), Some(recv_time)) = (p.receive_date, p.receive_time) {
                                        (date_th(&recv_date), time_hm(&recv_time))
                                    } else if let (Some(ord_date), Some(ord_time)) = (p.order_date, p.order_time) {
                                        (date_th(&ord_date), time_hm(&ord_time))
                                    } else {
                                        (String::from("No."), p.lab_order_number.to_string())
                                    }
                                }).collect::<Vec<(String, String)>>();

                                page.previous_headers.set(headers);

                                // 3. loop over main.lab_items_group
                                for main_group in main.lab_items_group.iter_mut() {
                                    // prepared matched previous group
                                    let prev_groups = previous.iter().map(|h| {
                                        h.lab_items_group.iter().find(|g| g.lab_items_group.unwrap_or_default() == main_group.lab_items_group.unwrap_or_default())
                                    }).collect::<Vec<Option<&LabItemsGroup>>>();
                                    // loop over main items and edit prev_lab_order_results
                                    for main_item in main_group.lab_items.iter_mut() {
                                        main_item.prev_lab_order_results = prev_groups.iter().map(|g_opt| {
                                            g_opt.and_then(|g| {
                                                g.lab_items.iter().find(|i| i.lab_items_code.unwrap_or_default() == main_item.lab_items_code.unwrap_or_default())
                                                .and_then(|i| i.lab_order_result.clone())
                                            })
                                        }).collect::<Vec<Option<String>>>();
                                    }
                                }

                                page.lab_detail.set(Some(Rc::new(main)));
                                page.previous_detail.set(previous.into_iter().map(Rc::new).collect())
                            } else {
                                page.lab_detail.set(None);
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    // ipd-lab-read-save.php
    fn set_read(page: Rc<Self>, app: Rc<App>) {
        if let Some(lab_order_number) = zero_none(page.lab_order_number.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::LabReadId`
                    match LabHead::call_api_post_readed(lab_order_number, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                page.lab_detail_read.set(String::from("Y"));
                                page.lab_detail_read_user.set(app.user_name().unwrap_or_default());
                                page.lab_detail_read_datetime.set(datetime_th_relative(&js_now()));
                                if let Some(loaded_lab_unread_exists_spinner) = &page.loaded_lab_unread_exists_spinner {
                                    loaded_lab_unread_exists_spinner.set_neq(false);
                                }
                                page.loaded_head.set(false);
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    // ipd-lab-read-delete.php
    fn remove_read(page: Rc<Self>, app: Rc<App>) {
        if let Some(lab_order_number) = zero_none(page.lab_order_number.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // DELETE `EndPoint::LabReadId`
                    match LabHead::call_api_delete_readed(lab_order_number, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.lab_detail_read.set(String::from("N"));
                                page.lab_detail_read_user.set(String::new());
                                page.lab_detail_read_datetime.set(String::new());
                                if let Some(loaded_lab_unread_exists_spinner) = &page.loaded_lab_unread_exists_spinner {
                                    loaded_lab_unread_exists_spinner.set_neq(false);
                                }
                                page.loaded_head.set(false);
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    pub fn render(cpn_id: &'static str, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_head.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_head(page.clone(), app.clone());
                    page.loaded_head.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_detail.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_detail(page.clone(), app.clone());
                    page.loaded_detail.set_neq(true);
                }
                async {}
            })))
            .future(page.lab_order_number.signal().for_each(clone!(page => move |lab_order_number| {
                if lab_order_number > 0 {
                    page.loaded_detail.set(false);
                }
                async {}
            })))
            .future(map_ref! {
                let sorted = page.sorted_heads().to_signal_cloned(),
                let lab_order_number = page.lab_order_number.signal() =>
                (sorted.to_vec(), *lab_order_number)
            }.for_each(clone!(page => move |(sorted, lab_order_number)| {
                if !sorted.iter().any(|lh| lh.lab_order_number == lab_order_number) {
                    let new = sorted.first().map(|lh| lh.lab_order_number).unwrap_or_default();
                    page.lab_order_number.set_neq(new);
                }
                async {}
            })))
            .class("row")
            .children([
                html!("div", {
                    .class("pe-0")
                    .style("max-width","270px")
                    .children([
                        html!("div", {
                            .class("mb-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class("btn")
                                    .class_signal("btn-primary", not(page.is_orderby_report.signal()))
                                    .child(html!("i", {.class(class::FA_UP91)}))
                                    .text(" เวลาสั่ง")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.is_orderby_report.set_neq(false);
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class("btn")
                                    .class_signal("btn-primary", page.is_orderby_report.signal())
                                    .child(html!("i", {.class(class::FA_UP91)}))
                                    .text(" เวลารายงาน")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.is_orderby_report.set_neq(true);
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("mb-3")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class("btn")
                                    .class_signal("btn-primary", page.head_status.signal_cloned().map(|status| matches!(status, LabStatus::All)))
                                    .text("ทั้งหมด")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.head_status.set_neq(LabStatus::All);
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class("btn")
                                    .class_signal("btn-primary", page.head_status.signal_cloned().map(|status| matches!(status, LabStatus::Confirm)))
                                    .text("ยืนยัน")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.head_status.set_neq(LabStatus::Confirm);
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class("btn")
                                    .class_signal("btn-primary", page.head_status.signal_cloned().map(|status| matches!(status, LabStatus::Wait)))
                                    .text("รอผล")
                                    .event(clone!(page => move |_:events::Click| {
                                        page.head_status.set_neq(LabStatus::Wait);
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class("btn")
                                    .child(html!("i", {.class(class::FA_SYNC)}))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.loaded_head.set(false);
                                    }))
                                }),
                            ])
                        }),
                        html!("hr"),
                        html!("div", {
                            .style("height","100vh") // 100vh is ok for 16 items lab
                            .style("overflow-y","auto")
                            .child(html!("table", {
                                .class(class::TABLE_SM)
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .child(html!("th", {.attr("scope", "col").text("LAB")}))
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children_signal_vec(page.sorted_heads().map(clone!(app, page => move |head| {
                                            html!("tr", {
                                                .class("small")
                                                .style("cursor","pointer")
                                                .class_signal("table-info", page.lab_order_number.signal().map(clone!(head => move |n| n == head.lab_order_number)))
                                                .style_signal("border-right-color", page.lab_order_number.signal().map(clone!(head => move |n| {
                                                    if n == head.lab_order_number {
                                                        "red"
                                                    } else {
                                                        "inherit"
                                                    }
                                                })))
                                                .style_signal("border-right-width", page.lab_order_number.signal().map(clone!(head => move |n| {
                                                    if n == head.lab_order_number {
                                                        "5px"
                                                    } else {
                                                        "inherit"
                                                    }
                                                })))
                                                .child(html!("td", {
                                                    .children([
                                                        html!("span", {
                                                            .class("fw-bold")
                                                            .apply(|dom| {
                                                                if head.receive_date.is_some() {
                                                                    dom.child(html!("span", {.text(&[&date_th_opt(&head.receive_date)," ",&time_hm_opt(&head.receive_time)," (เวลาที่รับ)"].concat())}))
                                                                } else if head.order_date.is_some() {
                                                                    dom.child(html!("span", {.text(&[&date_th_opt(&head.order_date)," ",&time_hm_opt(&head.order_time)," (เวลาที่สั่ง)"].concat())}))
                                                                } else {
                                                                    dom
                                                                }
                                                            })
                                                            .apply(|dom| match head.lab_confirm_state.as_str() {
                                                                "1" => dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).class(class::FLOAT_RL)})),
                                                                "2" => dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GOLD).class(class::FLOAT_RL)})),
                                                                "3" => dom.child(html!("i", {.class(class::FA_HOURGLASS_GOLD).class(class::FLOAT_RL)})),
                                                                _ => dom,
                                                            })
                                                            .apply_if(app.has_permission(Permission::IpdLabReadView)
                                                                && head.vn.as_ref().map(|vn| vn == page.vnan.lock_ref().as_str()).unwrap_or_default()
                                                                && head.report_date.is_some()
                                                                && head.confirm_report == Some(String::from("Y"))
                                                                && head.lab_read_status.as_str() != "Y",
                                                            |dom| {
                                                                dom.child(html!("i", {.class(class::FA_ENV).class(class::FLOAT_RL)}))
                                                            })
                                                        }),
                                                        html!("br"),
                                                        text(&[&head.form_name.clone().unwrap_or_default(), " (",&head.lab_order_number.to_string(),")"].concat()),
                                                        html!("br"),
                                                        html!("span", {.style("font-style","italic").text(&head.lab_name_cc.clone().unwrap_or_default())}),
                                                    ])
                                                }))
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.lab_order_number.set_neq(head.lab_order_number);
                                                }))
                                            })
                                        })))
                                    }),
                                ])
                            }))
                        }),
                        html!("hr"),
                    ])
                }),
                html!("div", {
                    .class("col")
                    .style("width","calc(100% - 270px)")
                    .child_signal(page.lab_detail.signal_cloned().map(clone!(app, page => move |opt| {
                        opt.as_ref().map(|detail| Self::render_detail(cpn_id, detail.clone(), page.clone(), app.clone()))
                    })))
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", &["lab-history-modal-", cpn_id].concat())
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.lab_history_modal.signal_cloned().map(clone!(app, page => move |opt| {
                        opt.as_ref().map(clone!(app, page => move |modal| {
                            LabHistory::render(modal.clone(), app, Some(page.lab_order_number.clone()))
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }

    // ipd-nurse-lab-detail.php
    fn render_detail(cpn_id: &'static str, detail: Rc<LabHead>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BCYAN_T)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_CYANS)
                    .children([
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("HN : ")}))
                            .text(&detail.hn.clone().unwrap_or_default())
                        }),
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text(&[
                                if detail.department.as_ref().map(|d| d == "OPD").unwrap_or_default() {"VN"} else {"AN"}, " : "
                            ].concat())}))
                            .text(&detail.vn.clone().unwrap_or_default())
                        }),
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("Lab Order No. : ")}))
                            .text(&page.lab_order_number.get().to_string())
                        }),
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("ใบ Lab : ")}))
                            .text(&detail.form_name.clone().unwrap_or_default())
                        }),
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("Detail : ")}))
                            .text(&detail.lab_name_cc.clone().unwrap_or_default())
                        }),
                    ])
                    .apply(|dom| {
                        if let Some(specimen_name_cc) = &detail.specimen_name_cc {
                            dom.child(html!("label", {
                                .class("me-2")
                                .child(html!("b", {.text("Specimen : ")}))
                                .text(specimen_name_cc)
                            }))
                        } else {
                            dom
                        }
                    })
                    .apply_if(detail.order_date.is_some(), |dom| {
                        dom.child(html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("เวลาที่สั่ง : ")}))
                            .text(&[date_th_opt(&detail.order_date), time_hm_opt(&detail.order_time)].join(" "))
                        }))
                    })
                    .apply_if(detail.receive_date.is_some(), |dom| {
                        dom.child(html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("เวลาที่รับ : ")}))
                            .text(&[date_th_opt(&detail.receive_date), time_hm_opt(&detail.receive_time)].join(" "))
                        }))
                    })
                    .apply_if(detail.report_date.is_some(), |dom| {
                        dom.child(html!("label", {
                            .child(html!("b", {.text("เวลาที่รายงาน : ")}))
                            .text(&[date_th_opt(&detail.report_date), time_hm_opt(&detail.report_time)].join(" "))
                        }))
                    })
                }),
                html!("div", {
                    .class(class::CARD_BODY_P1)
                    .child(html!("div", {
                        .child(html!("div", {
                            .class(class::FLEX_WRAP)
                            .style("min-height","30px")
                            .apply_if(!detail.scan_images.is_empty(), |dom| dom
                                .child(html!("div",{
                                    .class(class::COLA_PY_L)
                                    .children(PdfButtons::buttons(
                                        PdfButtons::new(
                                            TypstReport::from_system_with_coercion(SystemReport::ScanImages, &app.state().report_coercions()),
                                            page.vnan.clone(),
                                            Mutable::new(true),
                                            Mutable::new(false),
                                            clone!(page, detail => move || {
                                                let id = [&page.vnan.lock_ref(), "|lab|1"].concat();
                                                serde_json::json!({
                                                    "id": id,
                                                    "patient": page.patient.lock_ref().as_ref(),
                                                    "images": &detail.scan_images,
                                                }).to_string()
                                            })
                                        ), "SCANNED IMAGE", None, app.clone()
                                    ))
                                }))
                            )
                            .apply_if(detail.vn.as_ref().map(|vn| vn == page.vnan.lock_ref().as_str()).unwrap_or_default()
                                && app.has_permission(Permission::IpdLabReadView)
                                && detail.report_date.is_some()
                                && detail.confirm_report == Some(String::from("Y")),
                            |dom| {
                                dom.child(html!("div", {
                                    .class(class::COLA_PY_L)
                                    .child(html!("div", {
                                        .class(class::FORM_CHK_PT)
                                        .apply_if(
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::LabReadId, false)
                                            && app.endpoint_is_allow(&Method::DELETE, &EndPoint::LabReadId, false),
                                        |dom| dom
                                            .child(html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("id", &["lab-read-status-check-", cpn_id].concat())
                                                .with_node!(element => {
                                                    .future(page.lab_detail_read.signal_cloned().for_each(clone!(element => move |v| {
                                                        if v == "Y" {
                                                            element.set_checked(true);
                                                        } else {
                                                            element.set_checked(false);
                                                        }
                                                        async {}
                                                    })))
                                                    .apply(mixins::click_with_loader_checked(clone!(app, page, element => move || {
                                                        // after click status
                                                        if element.checked() {
                                                            Self::set_read(page.clone(), app.clone());
                                                        } else {
                                                            Self::remove_read(page.clone(), app.clone());
                                                        }
                                                    }), app.state()))
                                                })
                                            }))
                                        )
                                        .child(html!("label", {
                                            .class("form-check-label")
                                            .attr("for", &["lab-read-status-check-", cpn_id].concat())
                                            .style("user-select","none")
                                            .text("อ่านแล้ว ")
                                            .text_signal(page.lab_detail_read_datetime.signal_cloned())
                                            .text_signal(page.lab_detail_read_user.signal_cloned().map(|user| {
                                                if user.is_empty() {String::new()} else {[" โดย ", &user].concat()}
                                            }))
                                        }))
                                    }))
                                }))
                            })
                            .child(html!("div", {
                                .class(class::COLA_PY_RX)
                                .child(html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div", {
                                            .future(page.prev_changed.signal().for_each(clone!(page => move |changed| {
                                                if changed {
                                                    page.loaded_detail.set(false);
                                                    page.prev_changed.set(false);
                                                }
                                                async {}
                                            })))
                                            .class(class::INPUT_GROUP_SM)
                                            .style("width","190px")
                                            .children([
                                                doms::label_group_for(&["prev-", cpn_id].concat(),"จำนวนผลย้อนหลัง"),
                                                html!("select" => HtmlSelectElement, {
                                                    .class(class::FORM_SELECT_SM)
                                                    .attr("id", &["prev-", cpn_id].concat())
                                                    .children([
                                                        html!("option", {
                                                            .attr("value", "1")
                                                            .text("0")
                                                        }),
                                                        html!("option", {
                                                            .attr("value", "2")
                                                            .text("1")
                                                        }),
                                                        html!("option", {
                                                            .attr("value", "4")
                                                            .text("3")
                                                        }),
                                                        html!("option", {
                                                            .attr("value", "6")
                                                            .text("5")
                                                        }),
                                                        html!("option", {
                                                            .attr("value", "8")
                                                            .text("7")
                                                        }),
                                                        html!("option", {
                                                            .attr("value", "10")
                                                            .text("9")
                                                        }),
                                                    ])
                                                    .apply(mixins::string_value_select(page.prev.clone(), page.prev_changed.clone()))
                                                }),
                                            ])
                                        }),
                                        html!("div",{
                                            .children(PdfButtons::buttons(
                                                PdfButtons::new(
                                                    TypstReport::from_system_with_coercion(SystemReport::Lab, &app.state().report_coercions()),
                                                    page.vnan.clone(),
                                                    Mutable::new(true),
                                                    Mutable::new(false),
                                                    clone!(page => move || {serde_json::json!({
                                                        "id": page.vnan.get_cloned(),
                                                        "lab": vec![page.lab_detail.get_cloned()],
                                                    }).to_string()})
                                                ), "PDF", None, app.clone()
                                            ))
                                        }),
                                        html!("div",{
                                            .children(PdfButtons::buttons(
                                                PdfButtons::new(
                                                    TypstReport::from_system_with_coercion(SystemReport::LabSummary, &app.state().report_coercions()),
                                                    page.vnan.clone(),
                                                    Mutable::new(true),
                                                    Mutable::new(false),
                                                    clone!(page => move || {
                                                        let mut labs = match page.lab_detail.get_cloned() {
                                                            Some(lab) => vec![lab],
                                                            None => Vec::new(),
                                                        };
                                                        labs.extend(page.previous_detail.get_cloned());
                                                        serde_json::json!({
                                                            "id": page.vnan.get_cloned(),
                                                            "lab": labs,
                                                        }).to_string()
                                                    })
                                                ), "PDF SUMMARY", None, app.clone()
                                            ))
                                        }),
                                        html!("div",{
                                            .attr("type","button")
                                            .class(class::BTN_R_BLUE)
                                            .child(html!("i", {.class(class::FA_CLONE)}))
                                            .attr("title","Copy to Clipboard")
                                            .event(clone!(app, detail => move |_:events::Click| {
                                                spawn_local(clone!(app, detail => async move {
                                                    app.set_clipboard(&full_text(&detail)).await;
                                                }));
                                            }))
                                        }),
                                    ])
                                }))
                            }))
                        }))
                        .children(Self::render_detail_groups(cpn_id, detail.lab_items_group.clone(), page.clone(), app.clone()))
                    }))
                }),
            ])
        })
    }

    fn render_detail_groups(cpn_id: &'static str, groups: Vec<LabItemsGroup>, page: Rc<Self>, app: Rc<App>) -> impl Iterator<Item = Dom> {
        groups.into_iter().map(move |group| {
            html!("div", {
                .class(class::CARD)
                .children([
                    html!("div", {
                        .class(class::CARD_HEAD_CYANS)
                        .text(&group.lab_items_group_name.clone().unwrap_or_default())
                    }),
                    html!("div", {
                        .class(class::CARD_BODY_P1)
                        .child(doms::table_responsive(class::TABLE_SM_STRIP, clone!(app, page => move |table| { table
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .children([
                                            html!("th", {.attr("scope", "col").style("width","20px").text("#")}),
                                            html!("th", {.attr("scope", "col").style("min-width","100px").text("Lab Item")}),
                                            html!("th", {.attr("scope", "col").style("min-width","120px").text("ผล")}),
                                            html!("th", {.attr("scope", "col").style("min-width","120px").text("ค่าปกติ")}),
                                        ])
                                        .children(page.previous_headers.lock_ref().iter().map(|(header1, header2)| {
                                            html!("th", {
                                                .attr("scope", "col")
                                                .style("min-width","75px")
                                                .class("small")
                                                .children([
                                                    html!("span", {.text(header1)}),
                                                    html!("br"),
                                                    html!("span", {.text(header2)}),
                                                ])
                                            })
                                        }).collect::<Vec<Dom>>())
                                    }))
                                }),
                                html!("tbody", {
                                    .children(group.lab_items.into_iter().enumerate().map(|(i,item)| {
                                        let result_cmp = if let (Some(result), Some(normal)) = (item.lab_order_result.as_ref(), item.lab_items_normal_value_ref.as_ref()) {
                                            LabCmp::new(result, normal)
                                        } else {
                                            LabCmp::Normal
                                        };
                                        html!("tr", {
                                            .children([
                                                html!("td", {.text(&(i + 1).to_string())}),
                                                html!("td", {.text(&item.lab_items_name_ref.clone().unwrap_or_default())}),
                                                html!("td", {
                                                    .child(html!("span", {
                                                        .class("fw-bold")
                                                        .apply_if(!matches!(result_cmp, LabCmp::Normal), |dom| dom.class("text-danger"))
                                                        .text(&item.lab_order_result.clone().unwrap_or_default())
                                                    }))
                                                    .apply_if(item.lab_order_remark.as_ref().map(|s| !s.is_empty()).unwrap_or_default(), |dom| { dom
                                                        .apply_if(item.lab_order_result.as_ref().map(|s| !s.is_empty()).unwrap_or_default(), |d| d.child(html!("br")))
                                                        .child(html!("strong", {.text("หมายเหตุ: ")}))
                                                        .text(&item.lab_order_remark.clone().unwrap_or_default())
                                                    })
                                                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabItem, false), |dom| dom
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_SM_FR_GRAY)
                                                            .attr("data-bs-toggle", "modal")
                                                            .attr("data-bs-target", &["#lab-history-modal-", cpn_id].concat())
                                                            .text("ประวัติ")
                                                            .event(clone!(page, item => move |_:events::Click| {
                                                                if let Some(lab_items_code) = opt_zero_none(item.lab_items_code) {
                                                                    let lab_history_modal = LabHistory::new(
                                                                        page.hn.clone(),
                                                                        lab_items_code,
                                                                        &item.lab_items_name_ref,
                                                                        &item.lab_items_unit,
                                                                        &item.lab_order_number,
                                                                    );
                                                                    page.lab_history_modal.set(Some(lab_history_modal));
                                                                }
                                                            }))
                                                        }))
                                                    )
                                                    .apply_if(matches!(result_cmp, LabCmp::High), |dom| { dom
                                                        .child(html!("span", {
                                                            .class(class::BOLD_RED_R)
                                                            .text("H")
                                                        }))
                                                    })
                                                    .apply_if(matches!(result_cmp, LabCmp::Low), |dom| { dom
                                                        .child(html!("span", {
                                                            .class(class::BOLD_RED_R)
                                                            .text("L")
                                                        }))
                                                    })
                                                }),
                                                html!("td", {
                                                    .child(html!("span", {
                                                        .class("small")
                                                        .text(&item.lab_items_normal_value_ref.clone().unwrap_or_default())
                                                    }))
                                                    .apply_if(item.lab_items_unit.is_some(), |d| d.child(html!("span", {
                                                        .class(class::SMALL_FLOAT_R)
                                                        .text(&item.lab_items_unit.clone().unwrap_or_default())
                                                    })))
                                                }),
                                            ])
                                            .children(item.prev_lab_order_results.iter().map(|prev_result| {
                                                let prev_cmp = if let (Some(prev), Some(normal)) = (prev_result, item.lab_items_normal_value_ref.as_ref()) {
                                                    LabCmp::new(prev, normal)
                                                } else {
                                                    LabCmp::Normal
                                                };
                                                html!("td", {
                                                    .child(html!("span", {
                                                        .class("fw-bold")
                                                        .apply_if(!matches!(prev_cmp, LabCmp::Normal), |dom| dom.class("text-danger"))
                                                        .text(&prev_result.clone().unwrap_or_default())
                                                    }))
                                                    .apply_if(matches!(prev_cmp, LabCmp::High), |dom| { dom
                                                        .child(html!("span", {
                                                            .class(class::BOLD_RED_R)
                                                            .text("H")
                                                        }))
                                                    })
                                                    .apply_if(matches!(prev_cmp, LabCmp::Low), |dom| { dom
                                                        .child(html!("span", {
                                                            .class(class::BOLD_RED_R)
                                                            .text("L")
                                                        }))
                                                    })
                                                })
                                            }))
                                        })
                                    }))
                                }),
                            ])
                        })))
                    })
                ])
            })
        })
    }
}

fn sort_by_order_datetime(a: &Rc<LabHead>, b: &Rc<LabHead>) -> Ordering {
    let date_ord = match (a.order_date, b.order_date) {
        (Some(date_a), Some(date_b)) => date_a.cmp(&date_b),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    };
    let ord = if date_ord == Ordering::Equal {
        match (a.order_time, b.order_time) {
            (Some(time_a), Some(time_b)) => time_a.cmp(&time_b),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    } else {
        date_ord
    };
    if ord == Ordering::Equal { a.lab_order_number.cmp(&b.lab_order_number) } else { ord }
}

// fn sort_by_receive_datetime(a: &Rc<LabHead>, b: &Rc<LabHead>) -> Ordering {
//     let date_ord = match (a.receive_date, b.receive_date) {
//         (Some(date_a), Some(date_b)) => date_a.cmp(&date_b),
//         (Some(_), None) => Ordering::Greater,
//         (None, Some(_)) => Ordering::Less,
//         (None, None) => Ordering::Equal,
//     };
//     let ord = if date_ord == Ordering::Equal {
//         match (a.receive_time, b.receive_time) {
//             (Some(time_a), Some(time_b)) => time_a.cmp(&time_b),
//             (Some(_), None) => Ordering::Greater,
//             (None, Some(_)) => Ordering::Less,
//             (None, None) => Ordering::Equal,
//         }
//     } else {
//         date_ord
//     };
//     if ord == Ordering::Equal { sort_by_order_datetime(a, b) } else { ord }
// }

fn sort_by_report_datetime(a: &Rc<LabHead>, b: &Rc<LabHead>) -> Ordering {
    let date_ord = match (a.report_date, b.report_date) {
        (Some(date_a), Some(date_b)) => date_a.cmp(&date_b),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    };
    let ord = if date_ord == Ordering::Equal {
        match (a.order_time, b.order_time) {
            (Some(time_a), Some(time_b)) => time_a.cmp(&time_b),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    } else {
        date_ord
    };
    if ord == Ordering::Equal { sort_by_order_datetime(a, b) } else { ord }
}

pub enum LabCmp {
    Abnormal,
    High,
    Low,
    Normal,
}

impl LabCmp {
    /// support (x,y is f32 parsable) 'x-y', '<x', '>x', '<=x', '>=x' and match normal text<br>
    pub fn new(result: &str, normal: &str) -> Self {
        let result = result.trim();
        let normal = normal.trim();
        let result_has_lt = result.contains('<');
        let result_has_gt = result.contains('>');
        let result_has_eq = result.contains('=');

        if result.contains('-') {
            // for null '-' result
            // not support ranged result ex.'0-1'
            Self::Normal
        } else if let Ok(res) = result.chars().filter(|c| c.is_ascii_digit() || c == &'.').collect::<String>().parse::<f32>() {
            // result is number
            let res = if result_has_lt && !result_has_eq {
                res - 1.0
            } else if result_has_gt && !result_has_eq {
                res + 1.0
            } else {
                res
            };
            if normal.contains('-') {
                let split = normal.split('-').collect::<Vec<&str>>();
                if split.len() == 2 {
                    if let (Ok(min), Ok(max)) = (split[0].trim().replace(',', "").parse::<f32>(), split[1].trim().replace(',', "").parse::<f32>()) {
                        if res < min {
                            Self::Low
                        } else if res > max {
                            Self::High
                        } else {
                            Self::Normal
                        }
                    } else {
                        Self::Normal
                    }
                } else {
                    Self::Normal
                }
            } else if normal.contains('<') {
                let split = normal.split('<').collect::<Vec<&str>>();
                if split.len() == 2 {
                    let right = split[1].trim();
                    // check first char is '='
                    let has_eq = right.chars().next().map(|c| c == '=').unwrap_or_default();
                    if let Ok(right) = right.replace(['=', ','], "").parse::<f32>() {
                        if has_eq {
                            // ex. normal is <=100, abnormal is >100
                            if res > right { Self::Abnormal } else { Self::Normal }
                        } else {
                            // ex. normal is <100, abnormal is >=100
                            if res >= right { Self::Abnormal } else { Self::Normal }
                        }
                    } else {
                        Self::Normal
                    }
                } else {
                    Self::Normal
                }
            } else if normal.contains('>') {
                let split = normal.split('>').collect::<Vec<&str>>();
                if split.len() == 2 {
                    let right = split[1].trim();
                    // check first char is '='
                    let has_eq = right.chars().next().map(|c| c == '=').unwrap_or_default();
                    if let Ok(right) = right.replace(['=', ','], "").parse::<f32>() {
                        if has_eq {
                            // ex. normal is >=100, abnormal is <100
                            if res < right { Self::Abnormal } else { Self::Normal }
                        } else {
                            // ex. normal is >100, abnormal is <=100
                            if res <= right { Self::Abnormal } else { Self::Normal }
                        }
                    } else {
                        Self::Normal
                    }
                } else {
                    Self::Normal
                }
            } else {
                Self::Normal
            }
        } else if !result.is_empty() && !normal.is_empty() {
            if result != normal { Self::Abnormal } else { Self::Normal }
        } else {
            Self::Normal
        }
    }
}

pub fn full_text(row: &LabHead) -> String {
    row.lab_items_group
        .iter()
        .flat_map(|group| &group.lab_items)
        .filter_map(|item| {
            let result = item.lab_order_result.clone().and_then(|res| {
                let exact = res.trim_matches([' ', '-']);
                if exact.is_empty() { None } else { Some(exact.to_owned()) }
            });
            if let (Some(lab_items_name_ref), Some(lab_order_result)) = (&item.lab_items_name_ref, &result) {
                Some(
                    [
                        lab_items_name_ref,
                        " ",
                        lab_order_result,
                        &item.lab_items_unit.clone().map(|unit| [" ", &unit].concat()).unwrap_or_default(),
                    ]
                    .concat(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}
