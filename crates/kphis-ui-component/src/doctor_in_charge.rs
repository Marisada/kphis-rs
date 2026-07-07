// ipd-nurse-doctor-in-charge.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    ipd::doctor_in_charge::{DoctorInChargeParams, IpdDoctorInCharge},
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms::select_option, mixins};
use kphis_util::util::{str_some, zero_none};

/// - GET `EndPoint::IpdDoctorInCharge`
/// - POST `EndPoint::IpdDoctorInCharge` (guarded, remove 'บันทึกข้อมูล' btn)
/// - DELETE `EndPoint::IpdDoctorInCharge` (guarded, remove 'ลบ' btn)
#[derive(Default)]
pub struct DoctorInChargeCpn {
    loaded_table: Mutable<bool>,
    an: Mutable<String>,
    hn: Mutable<String>,

    table_data: MutableVec<Rc<IpdDoctorInCharge>>,

    // form
    changed: Mutable<bool>,
    doctor_in_charge_id: Mutable<u32>,
    doctor: Mutable<String>,
    spclty: Mutable<String>,
    status: Mutable<String>,
    activated: Mutable<String>,
    version: Mutable<i32>,
}

impl DoctorInChargeCpn {
    pub fn new(an: Mutable<String>, hn: Mutable<String>) -> Rc<Self> {
        Rc::new(Self { an, hn, ..Default::default() })
    }

    fn new_form(&self, app: Rc<App>) {
        if let Some(elm) = app.get_id("doctor") {
            NiceSelect::new_default(&elm);
        }
        if let Some(elm) = app.get_id("spclty") {
            NiceSelect::new_default(&elm);
        }
        self.doctor_in_charge_id.set_neq(0);
        self.doctor.set_neq(String::new());
        self.spclty.set_neq(String::new());
        self.status.set_neq(String::new());
        self.activated.set_neq(String::from("on"));
        self.version.set_neq(0);
        self.changed.set_neq(false);
    }

    fn edit_form(&self, row: &Rc<IpdDoctorInCharge>, app: Rc<App>) {
        let doctor = row.doctor.clone().unwrap_or_default();
        let spclty = row.spclty.clone().unwrap_or_default();
        if let Some(elm) = app.get_id("doctor") {
            NiceSelect::new_default_with_value(&elm, &doctor);
        }
        if let Some(elm) = app.get_id("spclty") {
            NiceSelect::new_default_with_value(&elm, &spclty);
        }
        self.doctor_in_charge_id.set_neq(row.doctor_in_charge_id);
        self.doctor.set_neq(doctor);
        self.spclty.set_neq(spclty);
        self.status.set_neq(row.status.clone().unwrap_or_default());
        self.activated.set_neq(row.activated.clone().unwrap_or_default());
        self.version.set_neq(row.version);
        self.changed.set_neq(false);
    }

    // ipd-nurse-doctor-in-charge-table.php
    fn load_table(page: Rc<Self>, app: Rc<App>) {
        if let Some(an) = str_some(page.an.get_cloned()) {
            let params = DoctorInChargeParams { an: Some(an), ..Default::default() };
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdDoctorInCharge`
                    match IpdDoctorInCharge::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            let mut lock = page.table_data.lock_mut();
                            lock.clear();
                            lock.extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    // ipd-nurse-doctor-in-charge-save.php
    // ipd-nurse-doctor-in-charge-update.php
    fn add_or_edit(page: Rc<Self>, app: Rc<App>) {
        let saver = IpdDoctorInCharge {
            doctor_in_charge_id: page.doctor_in_charge_id.get(),
            an: str_some(page.an.get_cloned()),
            hn: str_some(page.hn.get_cloned()),
            doctor: str_some(page.doctor.get_cloned()),
            spclty: str_some(page.spclty.get_cloned()).map(|s| if s.len() == 1 { ["0", &s].concat() } else { s }),
            status: str_some(page.status.get_cloned()),
            activated: str_some(page.activated.get_cloned()),
            version: page.version.get(),
            doctor_name: None,
            spclty_name: None,
        };

        app.async_load(
            true,
            clone!(app => async move {
                // POST `EndPoint::IpdDoctorInCharge`
                match saver.call_api_post(app.state()).await {
                    Ok((_id, responses)) => {
                        app.alert_execute_responses(&responses, clone!(app => async move {
                            // app.alert("บันทึกข้อมูลสำเร็จ");
                            page.new_form(app);
                            page.loaded_table.set_neq(false);
                        })).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    // ipd-nurse-doctor-in-charge-delete.php
    fn delete(page: Rc<Self>, app: Rc<App>) {
        if let (Some(doctor_in_charge_id), Some(version)) = (zero_none(page.doctor_in_charge_id.get()), zero_none(page.version.get())) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    if app.confirm("ยืนยันลบรายการ").await {
                        let params = DoctorInChargeParams {
                            doctor_in_charge_id: Some(doctor_in_charge_id),
                            version: Some(version),
                            ..Default::default()
                        };
                        // DELETE `EndPoint::IpdDoctorInCharge`
                        match IpdDoctorInCharge::call_api_delete(&params, app.state()).await {
                            Ok(responses) => {
                                app.alert_execute_responses(&responses, clone!(app => async move {
                                    page.new_form(app);
                                    page.loaded_table.set_neq(false);
                                })).await;
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            );
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_pre_admit = app.is_pre_admit(&page.an.lock_ref());
        let (doctor_select_option, spclty_select_option) = match app.app_asset.lock_ref().as_ref() {
            Some(assets_arc) => {
                let asset = assets_arc.as_ref().to_owned();
                (asset.doctor_select_option, asset.spclty_select_option)
            }
            None => (Vec::new(), Vec::new()),
        };

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_table.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_table(page.clone(), app.clone());
                    if let Some(elm) = app.get_id("doctor") {
                        NiceSelect::new_default(&elm);
                    }
                    if let Some(elm) = app.get_id("spclty") {
                        NiceSelect::new_default(&elm);
                    }
                    page.loaded_table.set(true);
                }
                async {}
            })))
            .class("row")
            .children([
                html!("div", {
                    .class(class::COL_MD6_T)
                    .child(html!("div", {
                        //.attr("id", "doctor-in-charge-data-table")
                        .children([
                            html!("div", {
                                .class(class::ALERT_BLUE)
                                .attr("role", "alert")
                                .child(html!("div", {
                                    .child(html!("i", {.class(class::FA_LIST)}))
                                    .text(" รายชื่อแพทย์เจ้าของไข้")
                                }))
                            }),
                            html!("div", {
                                .class("card")
                                .style("border-color","#B1ABB4")
                                //.style("height","42vh")
                                //.style("overflow-y","auto")
                                .child(html!("div", {
                                    .class("card-body")
                                    .child(html!("div", {
                                        //.attr("id", "data_show_table")
                                        .children([
                                            html!("table", {
                                                .class(class::TABLE_SM)
                                                .children([
                                                    html!("thead", {
                                                        .child(html!("tr", {
                                                            .children([
                                                                html!("th", {.text("#").attr("width", "5%").attr("scope", "col").class("text-center")}),
                                                                html!("th", {.text("แพทย์").attr("width", "46%").attr("scope", "col")}),
                                                                html!("th", {.text("แผนก").attr("scope", "col").attr("width", "26%")}),
                                                                html!("th", {.text("On").attr("scope", "col").attr("width", "8%").class("text-center")}),
                                                                html!("th", {.text("หลัก").attr("scope", "col").attr("width", "8%").class("text-center")}),
                                                                html!("th", {.attr("scope", "col").attr("width", "7%")}),
                                                            ])
                                                        }))
                                                    }),
                                                    html!("tbody", {
                                                        .children_signal_vec(page.table_data.signal_vec_cloned().enumerate().map(clone!(app, page => move |(i, row)| {
                                                            html!("tr", {
                                                                .children([
                                                                    html!("td", {.text_signal(i.signal_cloned().map(|opt| opt.map(|rid| (rid + 1).to_string()).unwrap_or_default()))}),
                                                                    html!("td", {.text(&row.doctor_name.clone().unwrap_or_default())}),
                                                                    html!("td", {.text(&row.spclty_name.clone().unwrap_or_default())}),
                                                                    html!("td", {
                                                                        .class("text-center")
                                                                        .apply(|dom| match row.activated.clone().unwrap_or_default().as_str() {
                                                                            "on" => dom.child(html!("i", {.class(class::FA_CHECK)})),
                                                                            "off" => dom.child(html!("i", {.class(class::FA_X)})),
                                                                            _ => dom,
                                                                        })
                                                                    }),
                                                                    html!("td", {
                                                                        .class("text-center")
                                                                        .apply_if(row.status.clone().unwrap_or_default().as_str() == "Y", |dom| {
                                                                            dom.child(html!("i", {.class(class::FA_STAR_GOLD)}))
                                                                        })
                                                                    }),
                                                                    html!("td", {
                                                                        .child(html!("button", {
                                                                            .attr("type", "button")
                                                                            .class(class::BTN_SM_GRAY)
                                                                            .child(html!("i", {.class(class::FA_EDIT)}))
                                                                            .event(clone!(app, page, row => move |_:events::Click| {
                                                                                page.edit_form(&row, app.clone());
                                                                            }))
                                                                        }))
                                                                    })
                                                                ])
                                                            })
                                                        })))
                                                    }),
                                                ])
                                            }),
                                            html!("h5", {
                                                .child(html!("span", {
                                                    .class(class::BADGE_GRAY)
                                                    .style("cursor","default")
                                                    .child(html!("i", {.class(class::FA_STAR_GOLD)}))
                                                    .text(" = แพทย์หลัก")
                                                }))
                                            }),
                                        ])
                                    }))
                                }))
                            }),
                        ])
                    }))
                }),
                html!("div", {
                    .class("col-md-6")
                    .child(html!("div", {
                        //.attr("id", "doctor-in-charge")
                        .children([
                            html!("div", {
                                .class(class::ALERT_BLUE)
                                .attr("role", "alert")
                                .child(html!("div", {
                                    .child(html!("i", {.class(class::FA_FILE)}))
                                    .text(" ฟอร์มกรอกแพทย์เจ้าของไข้")
                                }))
                            }),
                            html!("div", {
                                .class("card")
                                .style("border-color","#B1ABB4")
                                //.style("height","42vh")
                                //.style("overflow-y","auto")
                                .child(html!("div", {
                                    .class("card-body")
                                    .children([
                                        html!("div", {
                                            .class(class::ROW)
                                            .children([
                                                html!("label", {
                                                    .class(class::FORM_COL_LBL_SM3_R)
                                                    .attr("for", "doctor")
                                                    .text("แพทย์")
                                                }),
                                                html!("div", {
                                                    .class("col-sm-6")
                                                    .child(html!("select" => HtmlSelectElement, {
                                                        .class("form-control")
                                                        .attr("id", "doctor")
                                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                        .children(doctor_select_option.iter().map(|option| {
                                                            select_option(option, "")
                                                        }))
                                                        .apply(mixins::string_value_select(page.doctor.clone(), page.changed.clone()))
                                                    }))
                                                }),
                                            ])
                                        }),
                                        // html!("div", {
                                        //     .class("mb-3")
                                        //     .child(html!("div", {
                                        //         .class(class::COL_SM6_MID)
                                        //         .child(html!("div", {
                                        //             .attr("id", "check_value_doctor")
                                        //         }))
                                        //     }))
                                        // }),
                                        html!("div", {
                                            .class(class::ROW)
                                            .children([
                                                html!("label", {
                                                    .class(class::FORM_COL_LBL_SM3_R)
                                                    .attr("for", "spclty")
                                                    .text("แผนก")
                                                }),
                                                html!("div", {
                                                    .class("col-sm-6")
                                                    .child(html!("select" => HtmlSelectElement, {
                                                        .class("form-control")
                                                        .attr("id", "spclty")
                                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                        .children(spclty_select_option.iter().map(|option| {
                                                            select_option(option, "")
                                                        }))
                                                        .apply(mixins::string_value_select(page.spclty.clone(), page.changed.clone()))
                                                    }))
                                                }),
                                            ])
                                        }),
                                        // html!("div", {
                                        //     .class("mb-3")
                                        //     .child(html!("div", {
                                        //         .class(class::COL_SM6_MID)
                                        //         .child(html!("div", {
                                        //             .attr("id", "check_value_spclty")
                                        //         }))
                                        //     }))
                                        // }),
                                        html!("div", {
                                            .class(class::ROW)
                                            .child(html!("div", {
                                                .class(class::COL_SM6_MID)
                                                .child(html!("div", {
                                                    .class(class::FORM_CHK_COL_SM12)
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .attr("type", "checkbox")
                                                            .class("form-check-input")
                                                            .attr("id", "status")
                                                            .apply(mixins::checkbox_toggle(page.status.clone(), page.changed.clone(),"Y",""))
                                                        }),
                                                        html!("label", {
                                                            .class("ms-2")
                                                            .attr("for", "status")
                                                            .text("แพทย์หลัก")
                                                        }),
                                                    ])
                                                }))
                                            }))
                                        }),
                                        html!("div", {
                                            .class(class::ROW)
                                            .child(html!("div", {
                                                .class(class::COL_SM6_MID)
                                                .children([
                                                    html!("div", {
                                                        .class(class::FORM_CHK_COL_SM6)
                                                        .children([
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "radio")
                                                                .class("form-check-input")
                                                                .attr("id", "activated1")
                                                                .attr("checked", "")
                                                                .apply(mixins::radio_match(page.activated.clone(), page.changed.clone(),"on"))
                                                            }),
                                                            html!("label", {
                                                                .class("ms-2")
                                                                .attr("for", "activated1")
                                                                .text("On")
                                                            }),
                                                        ])
                                                    }),
                                                    html!("div", {
                                                        .class(class::FORM_CHK_COL_SM6)
                                                        .children([
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "radio")
                                                                .class("form-check-input")
                                                                .attr("id", "activated2")
                                                                .apply(mixins::radio_match(page.activated.clone(), page.changed.clone(),"off"))
                                                            }),
                                                            html!("label", {
                                                                .class("ms-2")
                                                                .attr("for", "activated2")
                                                                .text("Off")
                                                            }),
                                                        ])
                                                    }),
                                                ])
                                            }))
                                        }),
                                        html!("div", {
                                            .class("row")
                                            .child(html!("div", {
                                                .class("text-end")
                                                .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::IpdDoctorInCharge, is_pre_admit), |dom| { dom
                                                    .children([
                                                        html!("button" => HtmlButtonElement, {
                                                            .attr("type", "button")
                                                            .class(class::BTN_L)
                                                            .class_signal("btn-primary", page.changed.signal())
                                                            .class_signal("btn-secondary", not(page.changed.signal()))
                                                            .text("บันทึกข้อมูล")
                                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                Self::add_or_edit(page.clone(), app.clone());
                                                            }), not(page.changed.signal()), app.state()))
                                                        }),
                                                        html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_L_GRAY)
                                                            .text("ยกเลิก")
                                                            .event(clone!(app, page => move |_:events::Click| {
                                                                page.new_form(app.clone());
                                                            }))
                                                        }),
                                                    ])
                                                })
                                                .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdDoctorInCharge, is_pre_admit), |dom| { dom
                                                    .child_signal(page.doctor_in_charge_id.signal_cloned().map(clone!(app, page => move |id| {
                                                        (id > 0).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_RED)
                                                                .text("ลบ")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete(page.clone(), app.clone());
                                                                }), app.state()))
                                                            })
                                                        })
                                                    })))
                                                })
                                            }))
                                        }),
                                    ])
                                }))
                            }),
                        ])
                    }))
                }),
            ])
        })
    }
}
