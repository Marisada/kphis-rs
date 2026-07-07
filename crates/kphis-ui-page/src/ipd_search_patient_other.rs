use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, is_window_loaded, link, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTableCellElement};

use kphis_model::{
    route::Route,
    search::ipd_search_patient_other::{IpdSearchPatientOtherRequest, IpdSearchPatientOtherResponse},
    tab::Tab,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{datetime_th_opt_relative, datetime_th_relative},
    util::str_some,
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    BedNo,
    An,
    Hn,
    Name,
    Age,
    MaxVsDateTime,
    MaxFcNoteDateTime,
    MaxOrderDateTime,
}

/// GET `EndPoint::SearchOther`
#[derive(Clone, Default)]
pub struct IpdSearchPatientOtherPage {
    doctor_in_charge: Mutable<String>,
    patient: Mutable<String>,
    passcode: Mutable<String>,
    search_result: MutableVec<Rc<IpdSearchPatientOtherResponse>>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,

    changed: Mutable<bool>,
}

impl IpdSearchPatientOtherPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
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
                SortBy::Age => items.sort_by(|a, b| b.age_y.cmp(&a.age_y).then(b.age_m.cmp(&a.age_m)).then(b.age_d.cmp(&a.age_d))),
                SortBy::MaxVsDateTime => items.sort_by(|a, b| b.max_vs_datetime.cmp(&a.max_vs_datetime)),
                SortBy::MaxFcNoteDateTime => items.sort_by(|a, b| b.max_fcnote_datetime.cmp(&a.max_fcnote_datetime)),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| b.max_order_datetime.cmp(&a.max_order_datetime)),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::BedNo => items.sort_by(|a, b| a.bedno.cmp(&b.bedno)),
                SortBy::An => items.sort_by(|a, b| a.an.cmp(&b.an)),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::Name => items.sort_by(|a, b| a.fullname.cmp(&b.fullname)),
                SortBy::Age => items.sort_by(|a, b| a.age_y.cmp(&b.age_y).then(a.age_m.cmp(&b.age_m)).then(a.age_d.cmp(&b.age_d))),
                SortBy::MaxVsDateTime => items.sort_by(|a, b| a.max_vs_datetime.cmp(&b.max_vs_datetime)),
                SortBy::MaxFcNoteDateTime => items.sort_by(|a, b| a.max_fcnote_datetime.cmp(&b.max_fcnote_datetime)),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| a.max_order_datetime.cmp(&b.max_order_datetime)),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
    }

    // ipd-other-search-patient-table.php
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let request = IpdSearchPatientOtherRequest {
            ward: str_some(app.ward_select.get_cloned()),
            doctor_in_charge: str_some(page.doctor_in_charge.get_cloned()),
            patient: str_some(page.patient.get_cloned()),
            passcode: str_some(page.passcode.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::SearchOther`
                match IpdSearchPatientOtherResponse::call_api_get(&request, app.state()).await {
                    Ok(items) => {
                        let mut lock = page.search_result.lock_mut();
                        lock.clear();
                        lock.extend(items.into_iter().map(Rc::new));
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
        app.set_title("KPHIS - Search Patient");

        let (ward_select_option, doctor_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|asset| (asset.ward_select_option.clone(), asset.doctor_select_option.clone()))
            .unwrap_or_default();

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if let Some(elm) = app.get_id("ward") {
                        NiceSelect::new_default(&elm);
                    }
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
            .class("container-fluid")
            .child(doms::alert_row(clone!(app, page => move |alert| { alert
                .children([
                    doms::form_inline(clone!(app, page => move |form| { form
                        .children([
                            // .style("width","250px")
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
                                            // .apply(mixins::string_value_select(page.ward.clone(), page.changed.clone()))
                                            .prop_signal("value", app.ward_select.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    app.ward_select.set_neq(element.value());
                                                    app.to_local_storage();
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        }))
                                    }),
                                ])
                            })),
                            doms::form_inline_group_sm(clone!(page => move |group| { group
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
                            })),
                            // .style("width","310px")
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
                                                doms::select_option(option, "")
                                            }))
                                            .apply(mixins::string_value_select(page.doctor_in_charge.clone(), page.changed.clone()))
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
                                                    NiceSelect::new_default_with_value(&elm,"");
                                                }
                                                page.changed.set_neq(true);
                                            }
                                        }))
                                        // onclick="setDoctorInchargeAsBlank()
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
            .child_signal(app.is_wide_screen_card_or_table().map(clone!(app, page => move |is_wide_card| {
                Some(match is_wide_card {
                    // NOT wide screen
                    None => {
                        html!("div", {
                            .class(class::ROW_COL_RESP4_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app => move |row| {
                                render_card(row, app.clone())
                            })))
                        })
                    }
                    // wide screen card
                    Some(true) => {
                        html!("div", {
                            .class(class::ROW_COL5_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app => move |row| {
                                render_card(row, app.clone())
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
                                                .class("th-sm").attr("scope","col").text("ชื่อ - สกุล")
                                                .apply(Self::sortable_mixin(SortBy::Name, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("อายุ")
                                                .apply(Self::sortable_mixin(SortBy::Age, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์เจ้าของไข้")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลาล่าสุด").child(html!("br")).text("Vital Sign")
                                                .apply(Self::sortable_mixin(SortBy::MaxVsDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลาล่าสุด").child(html!("br")).text("Nurse Note")
                                                .apply(Self::sortable_mixin(SortBy::MaxFcNoteDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลาล่าสุด").child(html!("br")).text("Order")
                                                .apply(Self::sortable_mixin(SortBy::MaxOrderDateTime, page.clone()))
                                            }),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    .children_signal_vec(page.search_result.signal_vec_cloned().enumerate().map(move |(i,row)| {
                                        render_table(i.get().unwrap_or_default(), row, app.clone())
                                    }))
                                }),
                            ])
                        }))
                    }
                })
            })))
        })
    }
}

fn render_card(row: Rc<IpdSearchPatientOtherResponse>, app: Rc<App>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();

    let kphis_incharge_doctor_name_dom = html!("div", {
        //.style("min-width","40%")
        .children([
            html!("span", {
                .class(class::BADGE_CYAN)
                .style("cursor","default")
                .text("แพทย์เจ้าของไข้")
            }),
            html!("div", {
                .class("small")
                .text(&row.kphis_incharge_doctor_name.clone().unwrap_or_default())
            }),
        ])
    });

    let main_route = Route::IpdMain {
        view_by: String::from("other"),
        an: row.an.clone(),
        tab: Tab::Order.str().to_owned(),
        sub: String::new(),
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
                        html!("span", {
                            .class("fw-bold")
                            .text(&[row.ward_name.clone().unwrap_or_default(), row.bedno.clone().unwrap_or_default()].join(" "))
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
                                            .text("Order ")
                                            .apply_if(row.max_order_datetime.is_none(), |dom| dom.class("text-danger"))
                                            .text(&row.max_order_datetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                            .apply_if(row.nurse_not_accept_exists, |dom| {
                                                dom.child(html!("span", {
                                                    .class(class::BADGE_GOLD_R)
                                                    .style("cursor","default")
                                                    .text("ยังไม่รับ")
                                                }))
                                            })
                                            .apply_if(row.discharge_order_exists, |dom| {
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
                                            .apply_if(row.max_vs_datetime.is_none(), |dom| dom.class("text-danger"))
                                            .text(&row.max_vs_datetime.map(|vs| datetime_th_relative(&vs)).unwrap_or(String::from("ไม่มี")))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("Note ")
                                            .apply_if(row.max_fcnote_datetime.is_none(), |dom| dom.class("text-danger"))
                                            .text(&row.max_fcnote_datetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                        }),
                                    ])
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("d-flex")
                            .apply_if(row.kphis_incharge_doctor_name.is_some(), |dom| {
                                dom.child(kphis_incharge_doctor_name_dom)
                            })
                        }),
                    ])
                }),
            ])
        }))
    })
}

fn render_table(i: usize, row: Rc<IpdSearchPatientOtherResponse>, app: Rc<App>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();
    let kphis_incharge_doctor_name_with_html = html!("div", {
        .class("text-center")
        .children(row.kphis_incharge_doctor_name.clone().map(|docs| {
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
                .child(html!("div", {.class("text-truncate").text(&row.bedno.clone().unwrap_or_default())}))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {.class("text-truncate").text(&row.an)}))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}))
            }),
            html!("td", {
                .attr("title", &row.fullname.clone().unwrap_or_default())
                .apply(|dom| {
                    let child = html!("div", {
                        .class(class::TRUNC_BOLD)
                        .style("max-width","200px")
                        .apply_if(row.wp_status > 0, |d| d.text("* "))
                        .text(&row.fullname.clone().unwrap_or_default())
                    });
                    let route = Route::IpdMain {
                        view_by: String::from("other"),
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
                .attr("title", &row.kphis_incharge_doctor_name.clone().unwrap_or_default())
                .child(kphis_incharge_doctor_name_with_html)
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&datetime_th_opt_relative(&row.max_vs_datetime))
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&datetime_th_opt_relative(&row.max_fcnote_datetime))
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&datetime_th_opt_relative(&row.max_order_datetime))
                }))
                .apply_if(row.nurse_not_accept_exists, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_GOLD_R)
                        .style("cursor","default")
                        .text("ยังไม่รับ")
                    }))
                })
                .apply_if(row.discharge_order_exists, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_CYAN_R)
                        .style("cursor","default")
                        .text("D/C")
                    }))
                })
            }),
        ])
    })
}
