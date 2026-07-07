use dominator::{Dom, clone, events, html, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use kphis_model::{
    PATH_PREFIX_API_XRAY_THUMBNAIL,
    endpoint::{EndPoint, QueryString},
    fetch::Method,
    pacs::{PacsImageData, PacsParams, PacsXnData},
    xray::XrayReport,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, mixins};
use kphis_util::{
    datetime::{date_and_time_th_opt_relative, date_th, date_th_opt, datetime_th_relative, js_now, time_hm_opt},
    util::{opt_empty_none, str_some, zero_none},
};

use crate::gadget::xray_viewer::XrayViewer;

/// - GET `EndPoint::XrayReportHn`
/// - GET `EndPoint::XrayPacsXn`
/// - POST `EndPoint::XrayReadId` (guarded, remove read toggle)
/// - DELETE `EndPoint::XrayReadId` (guarded, remove read toggle)
#[derive(Clone, Default)]
pub struct XrayCpn {
    is_ipd: bool,
    hn: Mutable<String>,
    an: Mutable<String>,
    vn: Mutable<String>,
    loaded_xray_unread_exists_spinner: Option<Mutable<bool>>,

    loaded_report: Mutable<bool>,
    xray_reports: MutableVec<Rc<XrayReport>>,
    selected_xray_report: Mutable<Option<Rc<XrayReport>>>,

    // "Y" or "N"
    xray_report_read: Mutable<String>,
    xray_report_read_user: Mutable<String>,
    xray_report_read_datetime: Mutable<String>,

    xn: Mutable<i32>,
    loaded_xn_data: Mutable<bool>,
    xn_data: Mutable<Option<PacsXnData>>,

    selected_image_data: Mutable<Option<PacsImageData>>,
}

impl XrayCpn {
    pub fn new_ipd(hn: Mutable<String>, an: Mutable<String>, loaded_xray_unread_exists_spinner: Option<Mutable<bool>>) -> Rc<Self> {
        Rc::new(Self {
            is_ipd: true,
            hn,
            an,
            loaded_xray_unread_exists_spinner,
            loaded_xn_data: Mutable::new(true),
            ..Default::default()
        })
    }

    pub fn new_opd_er(hn: Mutable<String>, vn: Mutable<String>, loaded_xray_unread_exists_spinner: Option<Mutable<bool>>) -> Rc<Self> {
        Rc::new(Self {
            is_ipd: false,
            hn,
            vn,
            loaded_xray_unread_exists_spinner,
            loaded_xn_data: Mutable::new(true),
            ..Default::default()
        })
    }

    fn load_report(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::XrayReportHn`
                match XrayReport::call_api_get(&page.hn.lock_ref(), app.state()).await {
                    Ok(responses) => {
                        let all_rc = responses.into_iter().map(Rc::new).collect::<Vec<Rc<XrayReport>>>();
                        let current_xn = page.xn.get();
                        if !all_rc.iter().any(|xr| xr.xn == current_xn) {
                            if let Some(new) = all_rc.first() {
                                page.xn.set_neq(new.xn);
                                page.selected_xray_report.set(Some(new.to_owned()));
                            } else {
                                page.xn.set_neq(0);
                                page.selected_xray_report.set(None);
                            }
                        }
                        page.xray_reports.lock_mut().replace_cloned(all_rc);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_xn_data(page: Rc<Self>, app: Rc<App>) {
        if let Some(xn) = zero_none(page.xn.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::XrayPacsXn`
                    match PacsXnData::call_api_get(xn, app.state()).await {
                        Ok(response) => {
                            page.xn_data.set(Some(response));
                        }
                        Err(e) => {
                            page.xn_data.set(None);
                            if e.status == 404 || e.message.contains("404") {
                                app.alert("ไม่พบข้อมูล", &["ไม่พบข้อมูล XN: ", &xn.to_string(), " ในระบบ PACs"].concat());
                            } else {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            )
        }
    }

    fn set_read(page: Rc<Self>, app: Rc<App>) {
        if let Some(xn) = zero_none(page.xn.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::XrayReadId`
                    match PacsXnData::call_api_post_readed(xn, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                page.xray_report_read.set(String::from("Y"));
                                page.xray_report_read_user.set(app.user_name().unwrap_or_default());
                                page.xray_report_read_datetime.set(datetime_th_relative(&js_now()));
                                if let Some(loaded_xray_unread_exists_spinner) = &page.loaded_xray_unread_exists_spinner {
                                    loaded_xray_unread_exists_spinner.set_neq(false);
                                }
                                page.loaded_report.set(false);
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

    fn remove_read(page: Rc<Self>, app: Rc<App>) {
        if let Some(xn) = zero_none(page.xn.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // DELETE `EndPoint::XrayReadId`
                    match PacsXnData::call_api_delete_readed(xn, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.xray_report_read.set(String::from("N"));
                                page.xray_report_read_user.set(String::new());
                                page.xray_report_read_datetime.set(String::new());
                                if let Some(loaded_xray_unread_exists_spinner) = &page.loaded_xray_unread_exists_spinner {
                                    loaded_xray_unread_exists_spinner.set_neq(false);
                                }
                                page.loaded_report.set(false);
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
                let loaded = page.loaded_report.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_report(page.clone(), app.clone());
                    page.loaded_report.set_neq(true);
                }
                async {}
            })))
            .future(page.xn.signal().for_each(clone!(page => move |xn| {
                if xn > 0 {
                    page.loaded_xn_data.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_xn_data.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_xn_data(page.clone(), app.clone());
                    page.loaded_xn_data.set_neq(true);
                }
                async {}
            })))
            .class("row")
            .children([
                html!("div", {
                    .class("pe-0")
                    .style("max-width","270px")
                    .child(html!("div", {
                        .style("height","100vh")
                        .style("overflow-y","auto")
                        .child(html!("table", {
                            .class(class::TABLE_SM)
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .child(html!("th", {
                                            .attr("scope", "col")
                                            .text("X-RAY")
                                            .child_signal(page.hn.signal_cloned().map(clone!(app => move |hn| {
                                                app.pacs_hn_url(&hn).map(|pacs_hn_url| {
                                                    html!("a", {
                                                        .class("float-end")
                                                        .attr("href", &pacs_hn_url)
                                                        .attr("rel","noopener noreferrer")
                                                        .attr("target","_blank")
                                                        .child(html!("i", { .class(class::FA_EXT_LINK)}))
                                                    })
                                                })
                                            })))
                                        }))
                                    }))
                                }),
                                html!("tbody", {
                                    .children_signal_vec(page.xray_reports.signal_vec_cloned().map(clone!(page => move |report| {
                                        html!("tr", {
                                            .class("small")
                                            .style("cursor","pointer")
                                            .class_signal("table-info", page.xn.signal().map(clone!(report => move |n| n == report.xn)))
                                            .style_signal("border-right-color", page.xn.signal().map(clone!(report => move |n| {
                                                if n == report.xn {
                                                    "red"
                                                } else {
                                                    "inherit"
                                                }
                                            })))
                                            .style_signal("border-right-width", page.xn.signal().map(clone!(report => move |n| {
                                                if n == report.xn {
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
                                                            if report.examined_date.is_some() {
                                                                dom.child(html!("span", {.text(&[&date_th_opt(&report.examined_date)," ",&time_hm_opt(&report.examined_time)," (เวลาที่ทำ)"].concat())}))
                                                            } else if report.request_date.is_some() {
                                                                dom.child(html!("span", {.text(&[&date_th_opt(&report.request_date)," ",&time_hm_opt(&report.request_time)," (เวลาที่สั่ง)"].concat())}))
                                                            } else {
                                                                dom
                                                            }
                                                        })
                                                        .apply(|dom| match report.confirm.clone().unwrap_or_default().as_str() {
                                                            "Y" => dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).class(class::FLOAT_RL)})),
                                                            "N" => dom.child(html!("i", {.class(class::FA_HOURGLASS_GOLD).class(class::FLOAT_RL)})),
                                                            _ => dom,
                                                        })
                                                        .apply_if(if page.is_ipd {
                                                            report.an.as_ref().map(|an| an == page.an.lock_ref().as_str()).unwrap_or_default()
                                                        } else {
                                                            report.vn.as_ref().map(|vn| vn == page.vn.lock_ref().as_str()).unwrap_or_default()
                                                        } && report.xray_read_status.as_str() != "Y",
                                                        |dom| {
                                                            dom.child(html!("i", {.class(class::FA_ENV).class(class::FLOAT_RL)}))
                                                        })
                                                    }),
                                                    html!("br"),
                                                    text(&[&report.xray_items_name.clone().unwrap_or_default(), " (",&report.xn.to_string(),")"].concat()),
                                                ])
                                            }))
                                            .attr("title", &[
                                                "Requested by : ", &report.request_doctor_name.clone().unwrap_or_default(), " ",
                                                &date_and_time_th_opt_relative(&report.request_date, &report.request_time), "\n",
                                                "Examined by : ", &report.technician_name.clone().unwrap_or_default(), " ",
                                                &date_and_time_th_opt_relative(&report.examined_date, &report.examined_time),
                                            ].concat())
                                            .event(clone!(page => move |_:events::Click| {
                                                page.xn.set_neq(report.xn);
                                                page.selected_xray_report.set(Some(report.clone()));
                                            }))
                                        })
                                    })))
                                }),
                            ])
                        }))
                    }))
                }),
                html!("div", {
                    .class("col")
                    .style("width","calc(100% - 270px)")
                    .child(html!("div", {
                        .class("d-flex")
                        .style("flex-wrap", "wrap")
                        .style("gap", "10px")
                        .children([
                            // thumbnails
                            html!("div", {
                                .class("col")
                                .style("min-width","500px")
                                .child_signal(page.selected_xray_report.signal_cloned().map(clone!(app, page => move |opt| {
                                    opt.map(|selected_xray_report| {
                                        Self::render_report(cpn_id, selected_xray_report, page.clone(), app.clone())
                                    })
                                })))
                            }),
                            // full image
                            html!("div", {
                                .class("col")
                                .style("min-width","500px")
                                .child(XrayViewer::render(cpn_id, XrayViewer::new(page.selected_image_data.clone())))
                            }),
                        ])
                    }))
                }),
            ])
        })
    }

    fn render_report(cpn_id: &'static str, report: Rc<XrayReport>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BCYAN)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_CYANS)
                    .children([
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("HN : ")}))
                            .text(&report.hn.clone().unwrap_or_default())
                        }),
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("VN : ")}))
                            .text(&report.vn.clone().unwrap_or_default())
                        }),
                    ])
                    .apply(|dom| {
                        if let Some(an) = opt_empty_none(report.an.clone()) {
                            dom.child(html!("label", {
                                .class("me-2")
                                .child(html!("b", {.text("AN : ")}))
                                .text(&an)
                            }))
                        } else {
                            dom
                        }
                    })
                    .children([
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("XN. : ")}))
                            .text(&report.xn.to_string())
                        }),
                        html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("X-Ray : ")}))
                            .text(&report.xray_items_name.clone().unwrap_or_default())
                        }),
                    ])
                    .apply_if(report.request_date.is_some(), |dom| {
                        dom.child(html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("เวลาที่สั่ง : ")}))
                            .text(&[date_th_opt(&report.request_date), time_hm_opt(&report.request_time)].join(" "))
                        }))
                    })
                    .apply_if(report.examined_date.is_some(), |dom| {
                        dom.child(html!("label", {
                            .class("me-2")
                            .child(html!("b", {.text("เวลาที่ X-Ray : ")}))
                            .text(&[date_th_opt(&report.examined_date), time_hm_opt(&report.examined_time)].join(" "))
                        }))
                    })
                }),
                html!("div", {
                    .class(class::CARD_BODY_P1)
                    .child(html!("div", {
                        .class(class::FLEX_WRAP)
                        .style("min-height","30px")
                        .apply_if(if page.is_ipd {
                            report.an.as_ref().map(|an| an == page.an.lock_ref().as_str()).unwrap_or_default()
                        } else {
                            report.vn.as_ref().map(|vn| vn == page.vn.lock_ref().as_str()).unwrap_or_default()
                        }, |dom| {
                            dom.child(html!("div", {
                                .class(class::COLA_PY_L)
                                .child(html!("div", {
                                    .class(class::FORM_CHK_PT)
                                    .apply_if(
                                        app.endpoint_is_allow(&Method::POST, &EndPoint::XrayReadId, false)
                                        && app.endpoint_is_allow(&Method::DELETE, &EndPoint::LabReadId, false),
                                    |dom| dom
                                        .child(html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .class("form-check-input")
                                            .attr("id", &["xray-read-status-check-", cpn_id].concat())
                                            .with_node!(element => {
                                                .future(page.xray_report_read.signal_cloned().for_each(clone!(element => move |v| {
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
                                        .attr("for", &["xray-read-status-check-", cpn_id].concat())
                                        .style("user-select","none")
                                        .text("อ่านแล้ว ")
                                        .text_signal(page.xray_report_read_datetime.signal_cloned())
                                        .text_signal(page.xray_report_read_user.signal_cloned().map(|user| {
                                            if user.is_empty() {String::new()} else {[" โดย ", &user].concat()}
                                        }))
                                    }))
                                }))
                            }))
                        })
                        .child(html!("div", {
                            .class(class::COLA_P)
                            .class("w-100")
                            .child_signal(page.xn_data.signal_cloned().map(clone!(page => move |opt| {
                                opt.map(|xn_data| {
                                    // prepare data
                                    let pt_name = [xn_data.sname, xn_data.fname, xn_data.mname, xn_data.lname].join(" ");
                                    let gender_opt = match xn_data.gender.as_str() {
                                        "M" => Some("ชาย"),
                                        "F" => Some("หญิง"),
                                        _ => None,
                                    };
                                    let age_opt = if let Some(birth_datetime) = xn_data.birth {
                                        let birthdate_str = date_th(&birth_datetime.date());
                                        let age_days = (js_now() - birth_datetime).whole_days();
                                        let age_y = age_days / 365;
                                        if age_y > 0 {
                                            Some([&age_y.to_string(), " ปี (เกิด ", &birthdate_str, ")"].concat())
                                        } else if age_days > 30 {
                                            Some([&(age_days / 30).to_string(), " เดือน (เกิด ", &birthdate_str, ")"].concat())
                                        } else {
                                            Some([&age_days.to_string(), " วัน (เกิด ", &birthdate_str, ")"].concat())
                                        }
                                    } else {
                                        None
                                    };
                                    let ext_id = xn_data.ext_id.clone();

                                    // auto init first image
                                    page.selected_image_data.set(xn_data.images.first().cloned());

                                    html!("div", {
                                        .class(class::FLEX_COL)
                                        .children(xn_data.images.into_iter().map(clone!(page, pt_name, gender_opt, age_opt, ext_id => move |image| {
                                            let show_spinner = Mutable::new(true);
                                            let no_image = Mutable::new(false);
                                            html!("div", {
                                                .class(class::BOX_ROUND)
                                                .class(class::FLEX_W100)
                                                .class("my-2")
                                                .children([
                                                    html!("div", {
                                                        .class(class::W100_L)
                                                        .children([
                                                            html!("div", {
                                                                .children([
                                                                    html!("span", {.class("fw-bold").text("ชื่อ-สกุล: ")}),
                                                                    html!("span", {.text(&pt_name)}),
                                                                ])
                                                            }),
                                                            html!("div", {
                                                                .apply(|dom| {
                                                                    if let Some(gender) = gender_opt {
                                                                        dom.children([
                                                                            html!("span", {.class("fw-bold").text("เพศ: ")}),
                                                                            html!("span", {.text(gender)}),
                                                                        ])
                                                                    } else {
                                                                        dom
                                                                    }
                                                                })
                                                                .apply_if(gender_opt.is_some() && age_opt.is_some(), |dom| dom.text("\u{2003}"))
                                                                .apply(|dom| {
                                                                    if let Some(age) = &age_opt {
                                                                        dom.children([
                                                                            html!("span", {.class("fw-bold").text("อายุ: ")}),
                                                                            html!("span", {.text(&age)}),
                                                                        ])
                                                                    } else {
                                                                        dom
                                                                    }
                                                                })
                                                            }),
                                                            html!("div", {
                                                                .children([
                                                                    html!("span", {.class("fw-bold").text("XN: ")}),
                                                                    html!("span", {.text(&ext_id)}),
                                                                ])
                                                                .apply(|dom| {
                                                                    if let Some(label) = &image.label {
                                                                        dom.children([
                                                                            html!("span", {.class("fw-bold").text(" Note: ")}),
                                                                            html!("span", {.text(label)}),
                                                                        ])
                                                                    } else {
                                                                        dom
                                                                    }
                                                                })
                                                            }),
                                                        ])
                                                        .apply(|dom| {
                                                            if let Some(series_datetime) = image.series_datetime {
                                                                dom.child(html!("div", {
                                                                    .children([
                                                                        html!("span", {.class("fw-bold").text("วันที่-เวลา: ")}),
                                                                        html!("span", {.text(&datetime_th_relative(&series_datetime))}),
                                                                    ])
                                                                }))
                                                            } else {
                                                                dom
                                                            }
                                                        })
                                                    }),
                                                    html!("img", {
                                                        .attr("src", &[PATH_PREFIX_API_XRAY_THUMBNAIL, &PacsParams {
                                                            study_uid: str_some(image.study_uid.to_owned()),
                                                            series_uid: str_some(image.series_uid.to_owned()),
                                                            object_uid: str_some(image.object_uid.to_owned()),
                                                            file_path: str_some(image.file_path.to_owned()),
                                                            ..Default::default()
                                                        }.query_string()].concat())
                                                        .attr("alt", "thumbnail")
                                                        .class(class::M_YC)
                                                        .style("max-width","88px")
                                                        .style("height","100%")
                                                        .event(clone!(show_spinner => move |_:events::Load| {
                                                            show_spinner.set(false);
                                                        }))
                                                        .with_node!(element => {
                                                            .event(clone!(show_spinner, no_image => move |_:events::Error| {
                                                                element.set_hidden(true);
                                                                show_spinner.set(false);
                                                                no_image.set(true);
                                                            }))
                                                        })
                                                    }),
                                                ])
                                                .child_signal(show_spinner.signal().map(|show| {
                                                    show.then(|| {
                                                        html!("div", {
                                                            .style("padding", "32px 24px")
                                                            .style("z-index", "-1")
                                                            .child(html!("i",{
                                                                .class(class::FA_SPIN)
                                                                .style("font-size","32px")
                                                            }))
                                                        })
                                                    })
                                                }))
                                                .child_signal(no_image.signal().map(|show| {
                                                    show.then(|| {
                                                        html!("div", {
                                                            .style("text-align", "center")
                                                            .style("width", "100px")
                                                            .style("height", "100px")
                                                            .style("padding-top", "20px")
                                                            .children([
                                                                html!("i",{
                                                                    .class(class::FA_ALERT_GOLD)
                                                                    .style("font-size","32px")
                                                                }),
                                                                html!("br"),
                                                                text("No thumbnail"),
                                                            ])
                                                        })
                                                    })
                                                }))
                                                .style("cursor", "pointer")
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.selected_image_data.set(Some(image.clone()));
                                                }))
                                            })
                                        })))
                                    })
                                })
                            })))
                        }))
                    }))
                }),
            ])
        })
    }
}
