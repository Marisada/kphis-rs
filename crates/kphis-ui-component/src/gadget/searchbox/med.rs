// common-searchbox-med.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use rust_decimal::Decimal;
use std::{collections::HashSet, rc::Rc};
use web_sys::HtmlInputElement;

use kphis_model::search::searchbox::{DrugCheckParams, DrugDuplicateCheck, DrugInteractionCheck, MedSearchbox};
use kphis_ui_app::App;
use kphis_ui_core::{binding::Modal, class, doms};
use kphis_util::{
    datetime::js_now,
    util::{opt_zero_none, str_some},
};

use super::dec_to_color;
use crate::{
    modal::{blank_modal, drug_duplication::DrugDuplication, drug_interaction::DrugInteraction, drug_notify::DrugNotify},
    order::{MedSearchable, OrderItemMutable},
};

/// - GET `EndPoint::SearchBoxMedHnText`
/// - GET `EndPoint::SearchBoxMedDuplicate`
/// - GET `EndPoint::SearchBoxMedInteraction`
#[derive(Default)]
pub struct MedSearchboxCpn {
    pub hn: Mutable<Option<String>>,
    pub is_homemed: Mutable<bool>,

    pub text: Mutable<String>,
    pub results: MutableVec<Rc<MedSearchbox>>,
    pub status_text: Mutable<Option<String>>,

    pub selected_result: Mutable<Option<Rc<MedSearchbox>>>,
    pub drug_notify_modal: Mutable<Option<DrugNotify>>,
    drug_duplication_checked: Mutable<bool>,
    pub drug_duplication_modal: Mutable<Option<DrugDuplication>>,
    drug_interaction_checked: Mutable<bool>,
    pub drug_interaction_modal: Mutable<Option<DrugInteraction>>,
    pub allowed: Mutable<bool>,
    rendered: Mutable<bool>,
}

impl MedSearchboxCpn {
    pub fn new(hn: Option<String>, is_homemed: bool) -> Rc<Self> {
        Rc::new(Self {
            hn: Mutable::new(hn),
            is_homemed: Mutable::new(is_homemed),
            status_text: Mutable::new(Some(String::from("กรอกข้อความเพื่อค้นหา"))),
            ..Default::default()
        })
    }

    pub fn renew(&self) {
        self.text.set_neq(String::new());
        self.results.lock_mut().clear();
        self.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));

        self.selected_result.set(None);
        self.drug_notify_modal.set(None);
        self.drug_duplication_checked.set_neq(false);
        self.drug_duplication_modal.set(None);
        self.drug_interaction_checked.set_neq(false);
        self.drug_interaction_modal.set(None);
        self.allowed.set_neq(false);
    }

    fn load_data(page: Rc<Self>, app: Rc<App>) {
        page.status_text.set_neq(Some(String::from("กำลังค้นหา...")));
        app.async_load(
            true,
            clone!(app, page => async move {
                let search_text = page.text.get_cloned();
                if !search_text.is_empty() {
                    let hn = page.hn.get_cloned().unwrap_or(String::from("-"));
                    // GET `EndPoint::SearchBoxMedHnText`
                    match MedSearchbox::call_api_get(&hn, &search_text, app.state()).await {
                        Ok(results) => {
                            let mut lock = page.results.lock_mut();
                            if !lock.is_empty() {
                                lock.clear();
                            }
                            if results.is_empty() {
                                page.status_text.set(Some(String::from("ไม่พบรายการที่ค้นหา")));
                            } else {
                                lock.extend(results.into_iter().map(Rc::new));
                                page.status_text.set(None);
                            }
                        }
                        Err(e) => {
                            page.results.lock_mut().clear();
                            page.status_text.set(Some(String::from("การเชื่อมต่อขัดข้อง")));
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    /// MUST USE WITH 'fn render_modals' OUTSIDE this parent with 'fixed' or 'absolute' css position
    pub fn render<T: MedSearchable + 'static>(pre_order_master_id: Option<u32>, page: Rc<Self>, order_form: Rc<T>, app: Rc<App>) -> Dom {
        // prevent double 'renew()' at first call
        if page.rendered.get() {
            page.renew();
        } else {
            page.rendered.set(true);
        }

        html!("div", {
            .future(map_ref!{
                let is_selected = page.selected_result.signal_ref(|opt| opt.is_some()),
                let allowed = page.allowed.signal(),
                let drug_notify = page.drug_notify_modal.signal_cloned(),
                let drug_duplication_checked = page.drug_duplication_checked.signal(),
                let drug_duplication = page.drug_duplication_modal.signal_cloned(),
                let drug_interaction_checked = page.drug_interaction_checked.signal(),
                let drug_interaction = page.drug_interaction_modal.signal_cloned() =>
                *is_selected && *allowed && drug_notify.is_none() &&
                *drug_duplication_checked && *drug_interaction_checked && drug_duplication.is_none() && drug_interaction.is_none()
            }.for_each(clone!(order_form, page => move |allow| {
                if allow {
                    if let Some(result) = page.selected_result.get_cloned() {
                        let is_homemed = page.is_homemed.get();
                        let (meds, item_type) = if is_homemed {
                            (order_form.homemeds(), "home-medication")
                        } else {
                            (order_form.meds(), "med")
                        };
                        let mut lock = meds.lock_mut();
                        let order_item = OrderItemMutable::new(item_type, pre_order_master_id);
                        order_form.focused().set(Some(order_item.id));
                        order_form.changed().set_neq(true);
                        // trim for correcting "  " from ["","",""].join(" ")
                        order_item.order_item_detail.set(result.usage.as_ref().map(|usage| usage.trim()).unwrap_or_default().to_owned());
                        order_item.icode.set(Some(result.icode.clone()));
                        order_item.med_name.set(result.med_name.clone());
                        order_item.generic_name.set(result.generic_name.clone());
                        order_item.dosageform.set(result.dosageform.clone());
                        order_item.due_usage.set(result.due_usage.clone());
                        order_item.due_status.set(result.due_status.clone());
                        order_item.info.set(result.info.clone());
                        order_item.info_status.set(result.info_status.clone());
                        lock.push_cloned(order_item);
                        if is_homemed {
                            order_form.display_homemed_searchbox().set(false);
                        } else {
                            order_form.display_med_searchbox().set(false);
                        }
                        page.selected_result.set(None);
                    }
                }
                async {}
            })))
            .class(class::CARD_TW_T_CYANS)
            .style("height", [&super::BOX_HEIGHT.to_string(),"px"].concat())
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
                                    dom.event(clone!(page, load_mut => move |_: events::Input| {
                                        let search_text = element.value();
                                        if search_text.is_empty() {
                                            page.results.lock_mut().clear();
                                            page.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
                                        } else {
                                            page.text.set_neq(search_text);
                                            load_mut.set(true);
                                        }
                                    }))
                                    .future(map_ref!{
                                        let busy = app.loader_is_loading(),
                                        let load = load_mut.signal() =>
                                        !busy && *load
                                    }.for_each(clone!(app, page, load_mut => move |ready| {
                                        if ready {
                                            load_mut.set(false);
                                            Self::load_data(page.clone(), app.clone());
                                        }
                                        async {}
                                    })))
                                    // let timer_handle = Mutable::new(None);
                                    // dom.event(clone!(page, element, timer_handle, load_mut => move |_: events::KeyUp| {
                                    //     let wait = Timeout::new(1000, clone!(page, element, load_mut => move || {
                                    //         let search_text = element.value();
                                    //         if search_text.is_empty() {
                                    //             page.results.lock_mut().clear();
                                    //             page.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
                                    //         } else {
                                    //             page.text.set_neq(search_text);
                                    //             load_mut.set(true);
                                    //         }
                                    //     }));
                                    //     // prevent multiple keyup
                                    //     if let Some(handle) = timer_handle.get() {
                                    //         Timeout::manual_drop(handle);
                                    //     }
                                    //     timer_handle.set(Some(wait.handle()));
                                    //     wait.forget();
                                    // }))
                                    // .event_with_options(&EventOptions::preventable(), clone!(timer_handle => move |event: events::KeyDown| {
                                    //     if let Some(handle) = timer_handle.get() {
                                    //         Timeout::manual_drop(handle);
                                    //     }
                                    //     if event.key() == "Enter" {
                                    //         event.prevent_default();
                                    //     }
                                    // }))
                                })
                            })
                        }),
                        html!("button", {
                            .class(class::BTN_SM_RED)
                            .attr("type", "button")
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(order_form, page => move |_: events::Click| {
                                if page.is_homemed.get() {
                                    order_form.display_homemed_searchbox().set_neq(false);
                                } else {
                                    order_form.display_med_searchbox().set_neq(false);
                                }
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .style("border-width","thin")
                    .style("overflow-y","auto")
                    .child(html!("table", {
                        //.attr("id", "common-searchbox-med-table")
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
                                //.attr("id", "med-searchbox-tbody")
                                .child_signal(page.status_text.signal_cloned().map(|opt| {
                                    opt.as_ref().map(|text| {
                                        html!("tr", {
                                            .child(html!("td", {
                                                .attr("colspan", "2")
                                                .text(text)
                                            }))
                                        })
                                    })
                                }))
                                .children_signal_vec(page.results.signal_vec_cloned().map(clone!(order_form, page => move |result| {
                                    html!("tr", {
                                        .child(html!("td", {
                                            .class("fw-bold")
                                            .style("cursor","pointer")
                                            .child(doms::color_prefix_span(&dec_to_color(result.displaycolor.unwrap_or_default())))
                                            .text(&result.med_name.clone().unwrap_or_default())
                                            .apply_if(result.allergy_agent.is_some(), |dom| {
                                                dom.child(html!("div", {
                                                    .class(class::BOLD_RED_L)
                                                    .text(&["แพ้ยา: ", &result.allergy_agent_symptom.clone().unwrap_or_default()].concat())
                                                }))
                                            })
                                            .apply_if(result.allergy_count_force_no_order > Decimal::ZERO, |dom| {
                                                dom.child(html!("div", {
                                                    .class(class::BOLD_RED_L)
                                                    .text("[มีการห้ามสั่งใช้]")
                                                }),)
                                            })
                                            // ipd-dr-order.php:664, check_drug_duplication_and_interaction()
                                            .event(clone!(app, page, order_form, result => move |_: events::Click| {
                                                page.selected_result.set(Some(result.clone()));
                                                // display result's show_notify_text modal
                                                if let Some(status) = app.app_status.lock_ref().as_ref() {
                                                    if let Some(show_notify_text) = &result.show_notify_text {
                                                        if status.drug_notify_use == "Y" && result.show_notify.clone().unwrap_or_default() == "Y" && !show_notify_text.is_empty() && show_notify_text.contains(&status.drug_notify_start_marker) {
                                                            let show_text = if status.drug_notify_start_end_marker_use == "Y" {
                                                                let left = show_notify_text.find(&status.drug_notify_start_marker).map(|pos| pos + status.drug_notify_start_marker.len()).unwrap_or_default();
                                                                let right = show_notify_text.rfind(&status.drug_notify_end_marker).unwrap_or_default();
                                                                if right > left {
                                                                    &show_notify_text[left..right]
                                                                } else {
                                                                    ""
                                                                }
                                                            } else {
                                                                show_notify_text
                                                            };
                                                            page.drug_notify_modal.set(Some(DrugNotify::new(&result.med_name.clone().unwrap_or_default(), show_text)));
                                                            // fix with .future
                                                            // Modal::new("#drugNotifyModal").show();
                                                        }
                                                    }
                                                }
                                                let allergy_passed = if result.allergy_agent.is_some() {
                                                    if result.allergy_count_force_no_order > Decimal::ZERO {
                                                        app.alert_error("พบประวัติการแพ้ยา", &["แพ้ยา: ", &result.allergy_agent_symptom.clone().unwrap_or_default(), " (มีการห้ามสั่งใช้)"].concat());
                                                        false
                                                    } else {
                                                        app.alert_error("พบประวัติการแพ้ยา", &["แพ้ยา: ", &result.allergy_agent_symptom.clone().unwrap_or_default()].concat());
                                                        true
                                                    }
                                                } else {
                                                    true
                                                };
                                                if allergy_passed {
                                                    clone!(order_form, page, app => {
                                                        if app.app_status.lock_ref().as_ref().map(|aps| aps.hosxp_med_reconcilation_icode == result.icode).unwrap_or_default() {
                                                            app.alert("คำแนะนำ", "กรุณาสั่งยาเดิมผู้ป่วย ที่หัวข้อ Med Reconciliation");
                                                        } else {
                                                            Self::check_drug_duplication_and_interaction(
                                                                result.generic_name.as_ref().to_owned().cloned(),
                                                                result.med_name.as_ref().to_owned().cloned(),
                                                                order_form, page, app,
                                                            )
                                                        }
                                                    })
                                                }
                                            }))
                                        }))
                                    })
                                })))
                            }),
                        ])
                    }))
                }),
            ])
        })
    }

    pub fn render_modals(page: Rc<Self>) -> Vec<Dom> {
        vec![
            html!("div", {
                .future(page.drug_notify_modal.signal_cloned().map(|modal| modal.is_some()).for_each(|show| {
                    if show {
                        Modal::new("#drugNotifyModal").show();
                    }
                    async {}
                }))
                .class("modal")
                .attr("id", "drugNotifyModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.drug_notify_modal.signal_cloned().map(clone!(page => move |opt| {
                    opt.as_ref().map(|modal| {
                        DrugNotify::render(modal, page.drug_notify_modal.clone())
                    }).or(Some(blank_modal()))
                })))
            }),
            html!("div", {
                .future(page.drug_duplication_modal.signal_cloned().map(|modal| modal.is_some()).for_each(|show| {
                    if show {
                        Modal::new("#drugDuplicationModal").show();
                    }
                    async {}
                }))
                .class("modal")
                .attr("id", "drugDuplicationModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.drug_duplication_modal.signal_cloned().map(clone!(page => move |opt| {
                    opt.as_ref().map(|modal| {
                        DrugDuplication::render(modal, page.drug_duplication_modal.clone(), page.allowed.clone())
                    }).or(Some(blank_modal()))
                })))
            }),
            html!("div", {
                .future(page.drug_interaction_modal.signal_cloned().map(|modal| modal.is_some()).for_each(|show| {
                    if show {
                        Modal::new("#drugInteractionModal").show();
                    }
                    async {}
                }))
                .class("modal")
                .attr("id", "drugInteractionModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.drug_interaction_modal.signal_cloned().map(clone!(page => move |opt| {
                    opt.as_ref().map(|modal| {
                        DrugInteraction::render(modal, page.drug_interaction_modal.clone(), page.allowed.clone())
                    }).or(Some(blank_modal()))
                })))
            }),
        ]
    }

    fn check_drug_duplication_and_interaction<T: MedSearchable + 'static>(generic_name: Option<String>, med_name_opt: Option<String>, order_form: Rc<T>, page: Rc<Self>, app: Rc<App>) {
        if let Some(med_name) = med_name_opt {
            app.async_load(
                true,
                clone!(page, app => async move {
                    let offs = order_form.offs();
                    let offs_lock = offs.lock_ref();
                    let off_order_item_ids = if !offs_lock.is_empty() {
                        Some(offs_lock.iter().map(|off| off.id.to_string()).collect())
                    } else {
                        None
                    };
                    let mut additional_icodes = HashSet::new();
                    {
                        let meds = order_form.meds();
                        let meds_lock = meds.lock_ref();
                        if !meds_lock.is_empty() {
                            additional_icodes.extend(meds_lock.iter().filter_map(|med| med.icode.get_cloned()));
                        };
                    }

                    let params = DrugCheckParams {
                        an: order_form.an(),
                        generic_name: generic_name.clone(),
                        exclude_order_id: opt_zero_none(order_form.order_id()),
                        off_order_item_ids,
                        additional_icodes: str_some(additional_icodes.into_iter().collect::<Vec<String>>().join(",")),
                    };

                    // Check Drug Duplication
                    if !page.is_homemed.get() {
                        // GET `EndPoint::SearchBoxMedDuplicate`
                        match DrugDuplicateCheck::call_api_get(&params, app.state()).await {
                            Ok(mut results) => {
                                {
                                    let meds = order_form.meds();
                                    let meds_lock = meds.lock_ref();
                                    meds_lock.iter()
                                        .filter(|med| med.generic_name.lock_ref().as_ref() == generic_name.as_ref())
                                        .for_each(|med| {
                                            if let Some(icode) = med.icode.get_cloned() && !results.iter().any(|res| res.icode.as_ref().map(|res_icode| res_icode == &icode).unwrap_or_default()) {
                                                let now = js_now();
                                                results.push(DrugDuplicateCheck {
                                                    icode: Some(icode),
                                                    med_name: med.med_name.get_cloned(),
                                                    order_item_detail: str_some(med.order_item_detail.get_cloned()),
                                                    order_date: Some(now.date()),
                                                    order_time: Some(now.time()),
                                                });
                                            }
                                        });
                                }

                                if !results.is_empty() {
                                    page.drug_duplication_modal.set(Some(DrugDuplication::new(&med_name, &results)));
                                } else {
                                    page.drug_duplication_modal.set(None);
                                }
                                page.drug_duplication_checked.set_neq(true);
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    } else {
                        page.drug_duplication_checked.set_neq(true);
                    }

                    // Check Drug Interaction
                    // GET `EndPoint::SearchBoxMedInteraction`
                    match DrugInteractionCheck::call_api_get(&params, app.state()).await {
                        Ok(results) => {
                            // if results.iter().any(|result| result.not_allow == Some(String::from("Y"))) {
                            if !results.is_empty() {
                                page.drug_interaction_modal.set(Some(DrugInteraction::new(&results)));
                            } else {
                                page.drug_interaction_modal.set(None);
                            }
                            page.drug_interaction_checked.set_neq(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }

                    // 'allowed' will mark in drug_duplication_modal and drug_interaction_modal
                    if page.drug_duplication_modal.lock_ref().is_none() && page.drug_interaction_modal.lock_ref().is_none() {
                        // we mark 'allowed' when no modal active
                        page.allowed.set_neq(true);
                    }
                }),
            )
        }
    }
}

pub fn search_drugusage(height: &'static str, order_item_detail: Mutable<String>, app: Rc<App>) -> Dom {
    let drugusages = MutableVec::new();
    html!("div", {
        .future(order_item_detail.signal_cloned().for_each(clone!(app, drugusages => move |detail| {
            if !detail.is_empty() {
                if let Some(app_drugusages) = app.state().app_asset.lock_ref().as_ref().map(|asset| &asset.drugusages) {
                    let mut lock = drugusages.lock_mut();
                    lock.clear();
                    lock.extend(app_drugusages.iter().filter(|du| du.code.as_ref().map(|code| code.to_ascii_lowercase().starts_with(&detail.to_ascii_lowercase())).unwrap_or_default()).cloned());
                }
            }
            async {}
        })))
        .style_signal("height", drugusages.signal_vec_cloned().is_empty().map(move |is_empty| if is_empty {"0px"} else {height}))
        .style("overflow-y", "auto")
        .child(html!("div", {
            .children_signal_vec(drugusages.signal_vec_cloned().map(clone!(order_item_detail => move |drugusage| {
                let usage = [
                    drugusage.name1.clone().unwrap_or_default(),
                    drugusage.name2.clone().unwrap_or_default(),
                    drugusage.name3.clone().unwrap_or_default(),
                ].join(" ").trim().to_owned();
                html!("div", {
                    .class(class::BORDER_ROUND_SMALL_BG_GOLD)
                    .style("cursor", "pointer")
                    .child(html!("b", {.text(&drugusage.code.clone().unwrap_or_default())}))
                    .text(" : ")
                    .text(&usage)
                    .event(clone!(order_item_detail => move |_:events::Click| {
                        order_item_detail.set(usage.clone());
                    }))
                })
            })))
        }))
    })
}
