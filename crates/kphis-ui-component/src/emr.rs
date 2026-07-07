// ipd-emr.php

use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;

use kphis_model::{
    emr::{EmrDate, EmrVisit},
    endpoint::EndPoint,
    fetch::Method,
    image::scan_his::{ScanImage, ScanImageParams},
    report::{SystemReport, TypstReport},
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::Viewer, class};
use kphis_util::{
    datetime::{date_th_opt, time_hm_opt},
    util::{f64_rescale, sanity_dot_space, str_some, zero_none},
};

use crate::{
    document::{ipd_list::IpdDocumentListCpn, ipd_scan::IpdDocumentScanCpn, opd_er_list::OpdErDocumentListCpn, opd_er_scan::OpdErDocumentScanCpn},
    gadget::pdf_button::PdfButtons,
};

#[derive(Clone, Default, PartialEq)]
enum Tab {
    #[default]
    Null,
    Opd,
    Er,
    Pe,
    Lab,
}

/// - GET `EndPoint::EmrDateHn`
/// - GET `EndPoint::EmrVisitVn`
/// - GET `EndPoint::ScanHisImage` (guarded, remove his-image)
/// - GET `EndPoint::IpdDocumentListVnAn` (IpdDocumentListCpn, guarded, remove 'KPHIS IPD' section)
/// - GET `EndPoint::OpdErDocumentListVnId` (OpdErDocumentListCpn, guarded, remove 'KPHIS OPD-ER' section)
/// - GET `EndPoint::IpdDocumentScanAn` (IpdDocumentScanCpn, guarded, remove 'KPHIS IPD SCAN' section)
/// - GET `EndPoint::OpdErDocumentScanId` (OpdErDocumentScanCpn, guarded, remove 'KPHIS OPD-ER SCAN' section)
#[derive(Clone, Default)]
pub struct EmrCpn {
    hn: Mutable<String>,

    loaded_date: Mutable<bool>,
    redraw_date: Mutable<bool>,
    emr_dates: MutableVec<Rc<EmrDate>>,

    vn: Mutable<String>,
    an: Mutable<Option<String>>,
    opd_er_order_master_id: Mutable<Option<u32>>,

    loaded_visit: Mutable<bool>,
    emr_visit: Mutable<Option<Rc<EmrVisit>>>,

    load_images: Mutable<bool>,
    viewer: Mutable<Option<Rc<Viewer>>>,
    images_redraw: Mutable<bool>,

    active_tab: Mutable<Tab>,
    images: MutableVec<Rc<ScanImage>>,
}

impl EmrCpn {
    pub fn new(hn: Mutable<String>) -> Rc<Self> {
        Rc::new(Self {
            hn,
            loaded_visit: Mutable::new(true),
            ..Default::default()
        })
    }

    fn viewer_render(cpn_id: &'static str, page: Rc<Self>, app: Rc<App>) {
        match page.viewer.get_cloned() {
            Some(viewer) => {
                viewer.update();
            }
            None => {
                if let Some(elm) = app.get_id(&["image-content-", cpn_id].concat()) {
                    let viewer = Viewer::new(&elm);
                    page.viewer.set(Some(Rc::new(viewer)));
                }
            }
        }
    }

    fn viewer_destroy(&self) {
        if let Some(viewer) = self.viewer.get_cloned() {
            viewer.destroy();
            self.viewer.set(None);
        }
    }

    fn load_date(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::EmrDateHn`
                match EmrDate::call_api_get(&page.hn.get_cloned(), app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.emr_dates.lock_mut();
                        if !lock.is_empty() {
                            lock.clear();
                        }
                        lock.extend(responses.into_iter().map(Rc::new));
                        page.redraw_date.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    // ipd-emr-detail.php
    fn load_visit(page: Rc<Self>, app: Rc<App>) {
        if let Some(vn) = str_some(page.vn.get_cloned()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::EmrVisitVn`
                    match EmrVisit::call_api_get(&vn, app.state()).await {
                        Ok(response) => {
                            page.emr_visit.set(response.map(Rc::new));
                            page.images.lock_mut().clear();
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn load_images(page: Rc<Self>, app: Rc<App>) {
        page.images.lock_mut().clear();
        if let Some(vn) = str_some(page.vn.get_cloned()) {
            app.async_load(
                true,
                clone!(app => async move {
                    let key = match page.active_tab.get_cloned() {
                        Tab::Opd => "opd",
                        Tab::Er => "er",
                        Tab::Pe => "pe",
                        Tab::Lab => "lab",
                        Tab::Null => "null",
                    };
                    let params = ScanImageParams {
                        key: Some(key.to_owned()),
                        vn: Some(vn),
                        an: page.an.get_cloned().and_then(str_some),
                    };
                    // GET `EndPoint::ScanHisImage`
                    match ScanImage::call_api_get(&params, app.state()).await {
                        Ok(response) => {
                            page.images.lock_mut().extend(response.into_iter().map(Rc::new));
                            page.viewer_destroy();
                            page.images_redraw.set_neq(true);
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
                let loaded = page.loaded_date.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_date(page.clone(), app.clone());
                    page.loaded_date.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_visit.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_visit(page.clone(), app.clone());
                    page.loaded_visit.set_neq(true);
                }
                async {}
            })))
            .future(page.images_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    page.images_redraw.set(false);
                    Self::viewer_render(cpn_id, page.clone(), app.clone());
                }
                async {}
            })))
            .class("d-flex")
            .children([
                html!("div", {
                    .class(class::ROUND_BOLD)
                    .style("width","198px")
                    .children([
                        html!("div", {
                            .class(class::BOLD_BB_C_P1)
                            .text("วันที่-เวลา")
                        }),
                        html!("div", {
                            .class("small")
                            .style("height","100vh")
                            .style("overflow-y","auto")
                            .child(html!("ul", {
                                .class("list-group")
                                .children_signal_vec(page.emr_dates.signal_vec_cloned().enumerate().map(clone!(page => move |(i,date)| {
                                    if i.get().unwrap_or_default() == 0 && page.vn.lock_ref().as_str() != date.vn.as_str() {
                                        page.vn.set(date.vn.clone());
                                        page.an.set(date.an.clone());
                                        page.opd_er_order_master_id.set(date.opd_er_order_master_id);
                                        page.loaded_visit.set(false);
                                    }
                                    html!("li", {
                                        .class(class::LIST_GROUP_ITEM_BOLD_C)
                                        .class_signal("bg-info", page.vn.signal_ref(clone!(date => move |vn| *vn == date.vn)))
                                        .style("cursor","pointer")
                                        .child(html!("span", {
                                            .apply_if(date.an.is_some(), |dom| dom.class("text-danger"))
                                            .text(&[date_th_opt(&date.vstdate), time_hm_opt(&date.vsttime)].join(" "))
                                        }))
                                        .event(clone!(page => move |_:events::Click| {
                                            page.vn.set_neq(date.vn.clone());
                                            page.an.set(date.an.clone());
                                            page.opd_er_order_master_id.set(date.opd_er_order_master_id);
                                            page.loaded_visit.set(false);
                                        }))
                                    })
                                })))
                            }))
                        })
                    ])
                }),
                html!("div", {
                    .class("ms-2")
                    .style("width","calc(100% - 206px)")
                    .child_signal(page.emr_visit.signal_cloned().map(clone!(page, app => move |opt| {
                        opt.as_ref().map(clone!(page, app => move |visit| Self::render_visit(cpn_id, visit, page, app)))
                    })))
                }),
            ])
        })
    }

    fn render_visit(cpn_id: &'static str, visit: &Rc<EmrVisit>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let allow_load_image = app.endpoint_is_allow(&Method::GET, &EndPoint::ScanHisImage, false);

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let load = page.load_images.signal() =>
                !busy && *load
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_images(page.clone(), app.clone());
                    page.load_images.set_neq(false);
                }
                async {}
            })))
            .children([
                html!("div", {
                    .class(class::BOX_ROUND_T)
                    .child(html!("span", {
                        .class("text-end")
                        .apply_if(visit.hn.is_some(), |dom| {
                            dom.children([
                                html!("span", {.class(class::BOLD_X).text("HN :")}),
                                html!("span", {.text(&visit.hn.clone().unwrap_or_default())}),
                            ])
                        })
                        .apply_if(visit.an.is_some(), |dom| {
                            dom.children([
                                html!("span", {.class(class::BOLD_X).text("AN :")}),
                                html!("span", {.text(&visit.an.clone().unwrap_or_default())}),
                            ])
                        })
                        .children([
                            html!("span", {.class(class::BOLD_X).text("VN :")}),
                            html!("span", {.text(&visit.vn.clone())}),
                            html!("span", {.class(class::BOLD_X).text("วันที่่ให้บริการ :")}),
                            html!("span", {.text(&date_th_opt(&visit.vstdate))}),
                            html!("span", {.class(class::BOLD_X).text("เวลา :")}),
                            html!("span", {.text(&time_hm_opt(&visit.vsttime))}),
                        ])
                    }))
                }),
                html!("div", {
                    .class(class::BOX_ROUND_T)
                    .children([
                        html!("h4", {
                            .class(class::TXT_R_PE)
                            .text("HOSxP")
                        }),
                        html!("hr", {.class("mt-0")}),
                        html!("ul", {
                            .children([
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("อายุ :")}),
                                        html!("span", {.text(&visit.age_th.clone().unwrap_or_default())}),
                                    ])
                                }),
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("แพทย์ผู้ตรวจ :")}),
                                        html!("span", {.text(&visit.doctor_name.clone().unwrap_or_default())}),
                                    ])
                                }),
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("ชนิดของการมา :")}),
                                        html!("span", {.text(&visit.ovstist_name.clone().unwrap_or_default())}),
                                    ])
                                }),
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("สิทธิการรักษา :")}),
                                        html!("span", {.text(&visit.pttype_name.clone().unwrap_or_default())}),
                                    ])
                                }),
                            ])
                        }),
                        html!("ul", {
                            .children([
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD).text("Vital Sign :")}),
                                        html!("span", {.class(class::BOLD_X).text("BT")}),
                                        html!("span", {.text(&visit.temperature.and_then(zero_none).map(|v|v.to_string()).unwrap_or(String::from("-"))).text(" °C")}),
                                        html!("span", {.class(class::BOLD_X).text("PR")}),
                                        html!("span", {.text(&visit.pulse.and_then(zero_none).map(|v|v.to_string()).unwrap_or(String::from("-"))).text(" /min")}),
                                        html!("span", {.class(class::BOLD_X).text("RR")}),
                                        html!("span", {.text(&visit.rr.and_then(zero_none).map(|v|v.to_string()).unwrap_or(String::from("-"))).text(" /min")}),
                                        html!("span", {.class(class::BOLD_X).text("BP")}),
                                        html!("span", {.text(&visit.bps.and_then(zero_none).map(|v|v.to_string()).unwrap_or(String::from("-")))
                                            .text("/")
                                            .text(&visit.bpd.and_then(zero_none).map(|v|v.to_string()).unwrap_or(String::from("-")))
                                            .text(" mmHg")
                                        }),
                                        html!("span", {.class(class::BOLD_X).text("BW")}),
                                        html!("span", {.text(&visit.bw.and_then(zero_none).map(|v|v.to_string()).unwrap_or(String::from("-"))).text(" kg.")}),
                                        html!("span", {.class(class::BOLD_X).text("Height")}),
                                        html!("span", {.text(&visit.height.and_then(zero_none).map(|v|v.to_string()).unwrap_or(String::from("-"))).text(" cm.")}),
                                        html!("span", {.class(class::BOLD_X).text("BMI")}),
                                        html!("span", {.text(&visit.bmi.and_then(zero_none).map(|v| f64_rescale(v,2).to_string()).unwrap_or(String::from("-")))}),
                                    ])
                                }),
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("CC :")}),
                                        html!("span", {.style("white-space","pre-wrap").text(&visit.cc.clone().unwrap_or_default())}),
                                    ])
                                }),
                            ])
                            .apply_if(visit.hpi.is_some(), |dom| {
                                dom.child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("PI :")}),
                                        html!("span", {.style("white-space","pre-wrap").text(&visit.hpi.clone().unwrap_or_default())}),
                                    ])
                                }))
                            })
                            .apply_if(visit.pmh.is_some(), |dom| {
                                dom.child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("PMH :")}),
                                        html!("span", {.style("white-space","pre-wrap").text(&visit.pmh.clone().unwrap_or_default())}),
                                    ])
                                }))
                            })
                            .apply_if(visit.fh.is_some(), |dom| {
                                dom.child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("FH :")}),
                                        html!("span", {.style("white-space","pre-wrap").text(&visit.fh.clone().unwrap_or_default())}),
                                    ])
                                }))
                            })
                            .apply_if(visit.sh.is_some(), |dom| {
                                dom.child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("SH :")}),
                                        html!("span", {.style("white-space","pre-wrap").text(&visit.sh.clone().unwrap_or_default())}),
                                    ])
                                }))
                            })
                            .children([
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("PE :")}),
                                        html!("span", {.style("white-space","pre-wrap").text(&visit.pe.clone().unwrap_or_default())}),
                                    ])
                                }),
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("Diagnosis :")}),
                                        html!("ul", {.children(visit.diagnoses.iter().map(|dx| {
                                            html!("li", {.text(dx)})
                                        }))}),
                                    ])
                                }),
                                html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("OPD Med :")}),
                                        html!("ul", {.children(visit.drugs.iter().map(|s| {
                                            html!("li", {.text(&sanity_dot_space(s))})
                                        }))}),
                                    ])
                                }),
                            ])
                            .apply_if(visit.an.is_some(), |dom| {dom
                                .child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("Home Med :")}),
                                        html!("ul", {.children(visit.home_drugs.iter().map(|s| {
                                            html!("li", {.text(&sanity_dot_space(s))})
                                        }))}),
                                    ])
                                }))
                            })
                            .child(html!("li", {
                                .children([
                                    html!("span", {.class(class::BOLD_L2).text("OPD Non-Drug :")}),
                                    html!("ul", {.children(visit.nondrugs.iter().map(|dx| {
                                        html!("li", {.text(dx)})
                                    }))}),
                                ])
                            }))
                            .apply_if(visit.an.is_some(), |dom| {dom
                                .child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("Home Non-Drug :")}),
                                        html!("ul", {.children(visit.home_nondrugs.iter().map(|dx| {
                                            html!("li", {.text(dx)})
                                        }))}),
                                    ])
                                }))
                            })
                            .apply_if(visit.has_data_refer_out, |dom| dom
                                .child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("Refer :")}),
                                        html!("span", {
                                            .children(PdfButtons::buttons(
                                                PdfButtons::new(
                                                    TypstReport::from_system_with_coercion(SystemReport::ReferOut, &app.state().report_coercions()),
                                                    Mutable::new(visit.vn.clone()),
                                                    Mutable::new(true),
                                                    Mutable::new(false),
                                                    clone!(visit => move || {serde_json::json!({
                                                        "id": &visit.vn
                                                    }).to_string()}),
                                                ), "PDF", None, app.clone()
                                            ))
                                        })
                                    ])
                                }))
                            )
                            .apply_if(!visit.next_app.is_empty(), |dom| dom
                                .child(html!("li", {
                                    .children([
                                        html!("span", {.class(class::BOLD_L2).text("นัดหมาย :")}),
                                        html!("ul", {
                                            .children(visit.next_app.iter().map(clone!(page => move |next_app| {
                                                let appointment_date_opt = next_app.nextdate.map(|nextdate| page.emr_dates.lock_ref().iter().find(|date| date.vstdate.map(|d| d == nextdate).unwrap_or_default()).cloned()).flatten();
                                                html!("li", {
                                                    .text(&next_app.string())
                                                    .apply(|dom| {
                                                        if let Some(appointment_date) = appointment_date_opt { dom
                                                            .style("cursor","pointer")
                                                            .child(html!("i", {.class(class::FA_REPLY).class("ms-1")}))
                                                            .event(clone!(page => move |_:events::Click| {
                                                                page.vn.set_neq(appointment_date.vn.clone());
                                                                page.an.set(appointment_date.an.clone());
                                                                page.opd_er_order_master_id.set(appointment_date.opd_er_order_master_id);
                                                                page.loaded_visit.set(false);
                                                            }))
                                                        } else {
                                                            dom
                                                        }

                                                    })
                                                })
                                            })))
                                        }),
                                    ])
                                }))
                            )
                        })
                    ])
                }),
            ])
            .apply_if(allow_load_image && visit.image_exists.is_not_empty(), |dom| { dom
                .child(html!("div", {
                    .class(class::BOX_ROUND_T)
                    .apply(|d| {
                        if let Some(an) = visit.an.as_ref() {
                            if let Some(url) = app.scan_an_url(an) {
                                d.child(html!("a", {
                                    .class(class::BTN_FL_GRAY)
                                    .attr("href", &url)
                                    .attr("rel","noopener noreferrer")
                                    .attr("target","_blank")
                                    .text("IPD Scan ")
                                    .child(html!("i", { .class(class::FA_EXT_LINK)}))
                                }))
                            } else {
                                d
                            }
                        } else {
                            d
                        }
                    })
                    .children([
                        html!("h4", {
                            .class(class::TXT_R_PE)
                            .text("SCAN")
                        }),
                        html!("hr"),
                        html!("div", {
                            .child(html!("ul", {
                                .class(class::NAV_TABS_T)
                                .attr("role","tablist")
                                .children([
                                    html!("li", {
                                        .class("nav-item")
                                        .apply_if(visit.image_exists.has_scan, |dom| {
                                            dom.child(html!("a", {
                                                .class("nav-link")
                                                .attr("data-bs-toggle","pill")
                                                .attr("href","#")
                                                .text("OPD Scan")
                                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                    event.prevent_default();
                                                    page.active_tab.set_neq(Tab::Opd);
                                                    page.load_images.set(true);
                                                }))
                                            }))
                                        })
                                    }),
                                    html!("li", {
                                        .class("nav-item")
                                        .apply_if(visit.image_exists.has_er, |dom| {
                                            dom.child(html!("a", {
                                                .class("nav-link")
                                                .attr("data-bs-toggle","pill")
                                                .attr("href","#")
                                                .text("ER Image")
                                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                    event.prevent_default();
                                                    page.active_tab.set_neq(Tab::Er);
                                                    page.load_images.set(true);
                                                }))
                                            }))
                                        })
                                    }),
                                    html!("li", {
                                        .class("nav-item")
                                        .apply_if(visit.image_exists.has_pe, |dom| {
                                            dom.child(html!("a", {
                                                .class("nav-link")
                                                .attr("data-bs-toggle","pill")
                                                .attr("href","#")
                                                .text("PE Image")
                                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                    event.prevent_default();
                                                    page.active_tab.set_neq(Tab::Pe);
                                                    page.load_images.set(true);
                                                }))
                                            }))
                                        })
                                    }),
                                    html!("li", {
                                        .class("nav-item")
                                        .apply_if(visit.image_exists.has_lab, |dom| {
                                            dom.child(html!("a", {
                                                .class("nav-link")
                                                .attr("data-bs-toggle","pill")
                                                .attr("href","#")
                                                .text("Lab Image")
                                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                                    event.prevent_default();
                                                    page.active_tab.set_neq(Tab::Lab);
                                                    page.load_images.set(true);
                                                }))
                                            }))
                                        })
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class(class::ROW_TC)
                            .child(html!("ul", {
                                .style("list-style","none")
                                .attr("id", &["image-content-", cpn_id].concat())
                                .children_signal_vec(page.images.signal_vec_cloned().map(|image| {
                                    html!("li", {
                                        .style("display","inline-block")
                                        .style("padding","10px")
                                        .child(html!("img", {
                                            .style("height","300px")
                                            .attr("src", &image.image.image)
                                            .attr("alt","image")
                                            .apply_if(image.note.is_some(), |dom| { dom
                                                .attr("title", &image.note.clone().unwrap_or_default())
                                            })
                                        }))
                                    })
                                }))
                            }))
                        }),
                    ])
                }))
            })
            .apply(|dom| {
                if let Some(opd_er_order_master_id) = page.opd_er_order_master_id.get() {
                    let mut children = Vec::with_capacity(2);
                    if app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErDocumentListVnId, false) {
                        let opd_er_list = OpdErDocumentListCpn::new(Mutable::new(Some(visit.vn.clone())), opd_er_order_master_id, false);
                        children.push(html!("div", {
                            .class(class::BOX_ROUND_T)
                            .children([
                                html!("h4", {
                                    .class(class::TXT_R_PE)
                                    .text("KPHIS OPD-ER")
                                }),
                                OpdErDocumentListCpn::render(opd_er_list, app.clone()),
                            ])
                        }),);
                    }
                    if app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErDocumentScanId, false) {
                        let opd_er_scan = OpdErDocumentScanCpn::new(opd_er_order_master_id, &visit.vn, false);
                        children.push(html!("div", {
                            .class(class::BOX_ROUND_T)
                            .children([
                                html!("h4", {
                                    .class(class::TXT_R_PE)
                                    .text("KPHIS OPD-ER SCAN")
                                }),
                                OpdErDocumentScanCpn::render(opd_er_scan, app.clone()),
                            ])
                        }));
                    }

                    dom.children(children)
                } else {
                    dom
                }
            })
            .apply(|dom| {
                if let Some(an) = visit.an.as_ref() {
                    let is_pre_admit = app.is_pre_admit(an);
                    let mut children = Vec::with_capacity(2);
                    if app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDocumentListVnAn, is_pre_admit) {
                        let ipd_list = IpdDocumentListCpn::new(Mutable::new(Some(visit.vn.clone())), an, false);
                        children.push(html!("div", {
                            .class(class::BOX_ROUND_T)
                            .children([
                                html!("h4", {
                                    .class(class::TXT_R_PE)
                                    .text("KPHIS IPD")
                                }),
                                IpdDocumentListCpn::render(ipd_list, app.clone()),
                            ])
                        }));
                    }
                    if app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDocumentScanAn, is_pre_admit) {
                        let ipd_scan = IpdDocumentScanCpn::new(an, false);
                        children.push(html!("div", {
                            .class(class::BOX_ROUND_T)
                            .children([
                                html!("h4", {
                                    .class(class::TXT_R_PE)
                                    .text("KPHIS IPD SCAN")
                                }),
                                IpdDocumentScanCpn::render(ipd_scan, app.clone()),
                            ])
                        }));
                    }

                    dom.children(children)
                } else {
                    dom
                }
            })
        })
    }
}
