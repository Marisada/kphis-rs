use dominator::{Dom, DomBuilder, clone, events, html, is_window_loaded, link, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTableCellElement};

use kphis_model::{
    route::Route,
    search::ipd_search_patient_pharmacist::{IpdSearchPatientPharmacistRequest, IpdSearchPatientPharmacistResponse},
    tab::Tab,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{datetime_th_opt_relative, datetime_th_relative},
    util::{raw_concat_to_comma_equal, str_some},
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    BedNo,
    An,
    Hn,
    Name,
    Age,
    RegDateTime,
}

/// GET `EndPoint::SearchPharmacist`
#[derive(Clone, Default)]
pub struct IpdSearchPatientPharmacistPage {
    doctor_in_charge: Mutable<String>,
    drug_allergy_check: Mutable<String>,
    patient: Mutable<String>,
    search_result: MutableVec<Rc<IpdSearchPatientPharmacistResponse>>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,

    changed: Mutable<bool>,
}

impl IpdSearchPatientPharmacistPage {
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
                SortBy::RegDateTime => items.sort_by(|a, b| b.regdatetime.cmp(&a.regdatetime)),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::BedNo => items.sort_by(|a, b| a.bedno.cmp(&b.bedno)),
                SortBy::An => items.sort_by(|a, b| a.an.cmp(&b.an)),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::Name => items.sort_by(|a, b| a.fullname.cmp(&b.fullname)),
                SortBy::Age => items.sort_by(|a, b| a.age_y.cmp(&b.age_y).then(a.age_m.cmp(&b.age_m)).then(a.age_d.cmp(&b.age_d))),
                SortBy::RegDateTime => items.sort_by(|a, b| a.regdatetime.cmp(&b.regdatetime)),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
    }

    // ipd-pharmacy-search-patient-table.php
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let request = IpdSearchPatientPharmacistRequest {
            ward: str_some(app.ward_select.get_cloned()),
            doctor_in_charge: str_some(page.doctor_in_charge.get_cloned()),
            drug_allergy_check: str_some(page.drug_allergy_check.get_cloned()),
            patient: str_some(page.patient.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::SearchPharmacist`
                match IpdSearchPatientPharmacistResponse::call_api_get(&request, app.state()).await {
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
                                            // .apply(mixins::string_value_select(app.ward_select.clone(), page.changed.clone()))
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
                            // .style("width","310px")
                            doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                .children([
                                    doms::label_group_for("doctor_in_charge","แพทย์เจ้าของไข้"),
                                    // .style("width","250px")
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
                                    doms::label_group_for("drug_allergy_check","การประเมินข้อมูลแพ้ยาใบแรกรับ"),
                                    // .style("width","250px")
                                    html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .attr("id", "drug_allergy_check")
                                        .children([
                                            html!("option", {
                                                .attr("value", "")
                                                .text("ทั้งหมด")
                                            }),
                                            html!("option", {
                                                .attr("value", "waiting")
                                                .text("รอประเมิน")
                                            }),
                                            html!("option", {
                                                .attr("value", "checked")
                                                .text("ประเมินแล้ว")
                                            }),
                                            html!("option", {
                                                .attr("value", "no_admission_note")
                                                .text("ยังไม่มีบันทึกแรกรับ")
                                            }),
                                        ])
                                        .apply(mixins::string_value_select(page.drug_allergy_check.clone(), page.changed.clone()))
                                        // .attr("onchange", "onchange_select_ward()")
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
                                    })
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
                                                .class("th-sm").attr("scope","col").text("เวลาที่ Admit")
                                                .apply(Self::sortable_mixin(SortBy::RegDateTime, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","ผลการติดตามอาการ ยังไม่ปกติ หรือยังติดตามไม่ครบตามเกณฑ์")
                                                .child(html!("i", {.class(class::FA_ALERT_RED)}))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","Med Reconciliation").text("MR")
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("การประเมินข้อมูล").child(html!("br")).text("แพ้ยาใบแรกรับ")}),
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

fn render_card(row: Rc<IpdSearchPatientPharmacistResponse>, app: Rc<App>) -> Dom {
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

    let monitor_dom = if row.need_monitor {
        html!("span", {
            .class("text-center")
            .child(html!("i", {.class(class::FA_ALERT_RED)}))
            .style("cursor","help")
            .attr("title","ผลการติดตามอาการ ยังไม่ปกติ หรือยังติดตามไม่ครบตามเกณฑ์")
        })
    } else {
        Dom::empty()
    };

    let med_rec_dom = if row.mr_unconfirmed_exists {
        html!("span", {
            .class(class::SMALL_BOLD_C)
            .class("text-danger")
            .style("cursor","help")
            .text("MR")
            .attr("title","มีรายการ MR ที่แพทย์ยังไม่ได้พิจารณา")
        })
    } else if row.mr_confirmed_exists {
        html!("span", {
            .class(class::SMALL_BOLD_C)
            .class("text-info")
            .style("cursor","help")
            .text("MR")
            .attr("title","แพทย์พิจารณาแล้ว")
        })
    } else {
        Dom::empty()
    };

    let main_route = Route::IpdMain {
        view_by: String::from("pharmacist"),
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
                                            .text("Admit ")
                                            .text(&row.regdatetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่ระบุ")))
                                        }),
                                    ])
                                    .apply(|dom| {
                                        if let Some(allergy_drug_history_hosxp) = &row.allergy_drug_history_hosxp {
                                            dom.child(html!("div", {
                                                .class(class::SMALL_WRAP_BOLD_RED)
                                                .text("แพ้ยา-HOSxP : ")
                                                .text(allergy_drug_history_hosxp)
                                            }))
                                        } else {
                                            dom
                                        }
                                    })
                                    .apply(|dom| {
                                        if let Some(allergy_drug_history) = &row.allergy_drug_history {
                                            dom.child(html!("div", {
                                                .class(class::SMALL_WRAP_BOLD_RED)
                                                .text("แพ้ยา-แรกรับ : ")
                                                .text(&raw_concat_to_comma_equal(allergy_drug_history))
                                                .apply(|d| {
                                                    if let Some(drug_allergy_check_status) = &row.drug_allergy_check_status {
                                                        d.text(&[" (", drug_allergy_check_status, ")"].concat())
                                                    } else {
                                                        d
                                                    }
                                                })
                                                .attr("title", &[
                                                    row.drug_allergy_check_status.clone().unwrap_or_default(),
                                                    row.allergy_drug_pharmacy_check_person_name.clone().unwrap_or_default(),
                                                    datetime_th_opt_relative(&row.allergy_drug_pharmacy_check_datetime),
                                                ].join(" "))
                                            }))
                                        } else {
                                            dom
                                        }
                                    })
                                }),
                                html!("div", {
                                    .class(class::FLEX_COL)
                                    .child(monitor_dom).child(med_rec_dom)
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

fn render_table(i: usize, row: Rc<IpdSearchPatientPharmacistResponse>, app: Rc<App>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();
    let kphis_incharge_doctor_name_with_html = html!("div", {
        .class("text-truncate")
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
                        .text(&row.fullname.clone().unwrap_or_default())
                    });
                    let route = Route::IpdMain {
                        view_by: String::from("pharmacist"),
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
                .child(html!("div", {
                    .text(&datetime_th_opt_relative(&row.regdatetime))
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .apply_if(row.need_monitor, |dom| dom
                        .child(html!("i", {
                            .class(class::FA_ALERT_RED)
                            .attr("title","ผลการติดตามอาการ ยังไม่ปกติ หรือยังติดตามไม่ครบตามเกณฑ์")
                        }))
                    )
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .apply(|dom| {
                        if row.mr_unconfirmed_exists {
                            dom.child(html!("i", {
                                .class(class::FA_CIRCLE_CYAN)
                                .attr("title","มีรายการ MR ที่แพทย์ยังไม่ได้พิจารณา")
                            }))
                        } else if row.mr_confirmed_exists {
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
                .child(html!("div", {
                    .text(&row.drug_allergy_check_status.clone().unwrap_or_default())
                    .apply(|dom| {
                        if let Some(drug_allergy_check_status) = &row.drug_allergy_check_status {
                            match drug_allergy_check_status.as_str() {
                                "รอประเมิน" => dom.class(class::BOLD_RED_L),
                                "ประเมินแล้ว" => dom.class(class::BOLD_GREEN),
                                _ => dom,
                            }
                        } else {
                            dom
                        }
                    })
                    .apply(|dom| {
                        if let Some(check_datetime) = row.allergy_drug_pharmacy_check_datetime {
                            dom.child(html!("small", {
                                .text(&[" (", &datetime_th_relative(&check_datetime), ")"].concat())
                            }))
                        } else {
                            dom
                        }
                    })
                }))
            })
        ])
    })
}
