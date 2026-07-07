pub mod ipd_add;
pub mod ipd_list;
pub mod ipd_scan;
pub mod opd_er_list;
pub mod opd_er_scan;

// ipd-nurse-document.php
use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use std::rc::Rc;

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    {patient_info::PatientInfo, report::TypstReport},
};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::datetime::datetime_str_th;

#[derive(Clone, Default, PartialEq)]
pub enum Tab {
    Add,
    Scan,
    #[default]
    List,
}

#[derive(Clone, Default)]
pub struct DocumentCpn {
    active_tab: Mutable<Tab>,

    vn: Mutable<Option<String>>,
    patient: Mutable<Option<Rc<PatientInfo>>>,
}

impl DocumentCpn {
    pub fn new(vn: Mutable<Option<String>>, patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        Rc::new(Self { vn, patient, ..Default::default() })
    }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient.signal_ref(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("row")
            .child(html!("div", {
                .class("col-sm-12")
                .child(html!("div", {
                    .class(class::NAV_TABS_T)
                    .attr("role", "tablist")
                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                        (is_ipd && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDocumentDatetimeAn, is_pre_admit)).then(|| {
                            html!("a", {
                                .class(class::NAV_ITEM_LINK)
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Add)))
                                .attr("data-bs-toggle", "tab")
                                .attr("href", "#")
                                .attr("role", "tab")
                                .text("เพิ่มเอกสาร")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Add);
                                }))
                            })
                        })
                    })))
                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                        (if is_ipd {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDocumentScanAn, is_pre_admit)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErDocumentScanId, false)
                        }).then(|| {
                            html!("a", {
                                .class(class::NAV_ITEM_LINK)
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Scan)))
                                .attr("data-bs-toggle", "tab")
                                .attr("href", "#")
                                .attr("role", "tab")
                                .text("เอกสาร")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Scan);
                                }))
                            })
                        })
                    })))
                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                        (if is_ipd {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDocumentListVnAn, is_pre_admit)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErDocumentListVnId, false)
                        }).then(|| {
                            html!("a", {
                                .class(class::NAV_ITEM_LINK)
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::List)))
                                .attr("data-bs-toggle", "tab")
                                .attr("href", "#")
                                .attr("role", "tab")
                                .text("รวมเอกสาร")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::List);
                                }))
                            })
                        })
                    })))
                }))
                .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                    opt.map(|pt| {
                        match pt.visit_type.clone() {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                html!("div", {
                                    .class("tab-content")
                                    .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                                        Some(match tab {
                                            Tab::Add => {
                                                let ipd_add = ipd_add::IpdDocumentAddCpn::new(&an);
                                                ipd_add::IpdDocumentAddCpn::render(ipd_add, app.clone())
                                            }
                                            Tab::Scan => {
                                                let ipd_scan = ipd_scan::IpdDocumentScanCpn::new(&an, true);
                                                ipd_scan::IpdDocumentScanCpn::render(ipd_scan, app.clone())
                                            }
                                            Tab::List => {
                                                let ipd_list = ipd_list::IpdDocumentListCpn::new(page.vn.clone(), &an, true);
                                                ipd_list::IpdDocumentListCpn::render(ipd_list, app.clone())
                                            }
                                        })
                                    })))
                                })
                            }
                            VisitTypeId::OpdEr(vn, opd_er_order_master_id) => {
                                html!("div", {
                                    .class("tab-content")
                                    .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                                        Some(match tab {
                                            Tab::Add => {
                                                Dom::empty()
                                            }
                                            Tab::Scan => {
                                                let opd_er_scan = opd_er_scan::OpdErDocumentScanCpn::new(opd_er_order_master_id, &vn, true);
                                                opd_er_scan::OpdErDocumentScanCpn::render(opd_er_scan, app.clone())
                                            }
                                            Tab::List => {
                                                let opd_er_list = opd_er_list::OpdErDocumentListCpn::new(page.vn.clone(), opd_er_order_master_id, true);
                                                opd_er_list::OpdErDocumentListCpn::render(opd_er_list, app.clone())
                                            }
                                        })
                                    })))
                                })
                            }
                            VisitTypeId::Visit(_) => Dom::empty(),
                        }
                    })
                })))
                .child(html!("br"))
            }))
        })
    }
}

pub fn concat_to_table_row_link(concat: &Option<String>, link: Dom) -> Option<Dom> {
    concat.as_ref().and_then(|concat| {
        let split = concat.split('|').collect::<Vec<&str>>();
        (split.len() == 2).then(|| {
            html!("tr", {
                .children([
                    html!("td", {.child(link)}),
                    html!("td", {.text(&datetime_str_th(split[0]))}),
                    html!("td", {.text(&datetime_str_th(split[1]))}),
                ])
            })
        })
    })
}

pub fn check_unused(level: u8, label: &str) -> Dom {
    let (icon, tab_class, label_class) = match level {
        0 => ("fa-square", "col-md-1", "col-md-11"),
        _ => ("fa-circle", "col-md-2", "col-md-10"),
    };
    html!("li", {
        .class("list-group-item")
        .child(html!("div", {
            .class("row")
            .children([
                html!("div", {
                    .class([tab_class,"text-end"])
                    .child(html!("h5", {
                        .child(html!("div", {
                            .class(["text-secondary","fas",icon]) // grey filled icon
                        }))
                    }))
                }),
                html!("div", {
                    .class(label_class)
                    .child(html!("label", {.text(label)}))
                }),
            ])
        }))
    })
}

pub fn check_used_group(level: u8, label: &str, is_used: bool) -> Dom {
    let (icon_checked, icon_unchecked, tab_class, label_class) = match level {
        0 => ("fa-check-square", "fa-square", "col-md-1", "col-md-11"),
        _ => ("fa-check-circle", "fa-circle", "col-md-2", "col-md-10"),
    };
    html!("li", {
        .class("list-group-item")
        .child(html!("div", {
            .class("row")
            .children([
                html!("div", {
                    .class([tab_class,"text-end"])
                    .child(html!("h5", {
                        .child(html!("div", {
                            .apply(|dom| if is_used {
                                dom.class(["text-success","fas",icon_checked]) // white check on green icon
                            } else {
                                dom.class(["text-secondary","far",icon_unchecked]) // grey bordered icon
                            })
                        }))
                    }))
                }),
                html!("div", {
                    .class(label_class)
                    .child(html!("label", {.text(label)}))
                }),
            ])
        }))
    })
}

#[derive(Clone)]
pub struct PdfInner {
    pub report: TypstReport,
    pub ids: String,
    pub title: Option<String>,
}

impl PdfInner {
    pub fn new(report: TypstReport, ids: String, title: Option<String>) -> Self {
        Self { report, ids, title }
    }
}

#[derive(Clone)]
/// first argument is 'is_used'
pub enum PdfSource {
    HosXp(bool, Option<PdfInner>, &'static str),
    Kphis(bool, Option<PdfInner>, &'static str),
    Scan(bool, Option<PdfInner>, &'static str),
}
impl PdfSource {
    fn label(&self) -> &'static str {
        match self {
            Self::HosXp(_, _, _) => "HOSxP",
            Self::Kphis(_, _, _) => "KPHIS",
            Self::Scan(_, _, _) => "SCAN",
        }
    }
    fn bg_class(&self) -> &'static str {
        match self {
            Self::HosXp(_, _, _) => "text-bg-dark",
            Self::Kphis(_, _, _) => "text-bg-primary",
            Self::Scan(_, _, _) => "text-bg-info",
        }
    }
    fn is_used(&self) -> bool {
        match self {
            Self::HosXp(is_used, _, _) => *is_used,
            Self::Kphis(is_used, _, _) => *is_used,
            Self::Scan(is_used, _, _) => *is_used,
        }
    }
    fn inner(&self) -> Option<PdfInner> {
        match self {
            Self::HosXp(_, inner, _) => inner.clone(),
            Self::Kphis(_, inner, _) => inner.clone(),
            Self::Scan(_, inner, _) => inner.clone(),
        }
    }
    fn tag(&self) -> &'static str {
        match self {
            Self::HosXp(_, _, tag) => tag,
            Self::Kphis(_, _, tag) => tag,
            Self::Scan(_, _, tag) => tag,
        }
    }
}

pub fn check_used(level: u8, modal_hash: &str, label: &str, sources: Vec<PdfSource>, pdf_path_mutable: Mutable<Option<PdfInner>>, render_pdf_mutable: Mutable<bool>, app: Rc<App>) -> Dom {
    let (icon_checked, icon_unchecked, tab_class, label_class) = match level {
        0 => ("fa-check-square", "fa-square", "col-md-1", "col-md-11"),
        _ => ("fa-check-circle", "fa-circle", "col-md-2", "col-md-10"),
    };
    let badges = if app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false) {
        sources
            .iter()
            .map(clone!(pdf_path_mutable, render_pdf_mutable => move |s| {
                html!("span", {
                    .apply_if(s.is_used(), clone!(pdf_path_mutable, render_pdf_mutable => move |dom| {
                        dom.children([
                            html!("span", {
                                .class(["text-light","fw-bold","badge",s.bg_class(),"ms-1"]).text(s.label())
                            }),
                            s.inner().map(|inner| {
                                html!("a", {
                                    .class(class::BADGE_WRAP_RT_GRAY)
                                    .style("cursor","pointer")
                                    .attr("href","#")
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", modal_hash)
                                    .child(html!("i", {.class(class::FA_PRINT)}))
                                    .text(" PDF ")
                                    .text(s.tag())
                                    .event_with_options(&EventOptions::preventable(), clone!(inner => move |event: events::Click| {
                                        event.prevent_default();
                                        pdf_path_mutable.set(Some(inner.clone()));
                                        render_pdf_mutable.set(true);
                                    }))
                                })
                            }).unwrap_or(Dom::empty()),
                        ])
                    }))
                })
            }))
            .collect::<Vec<Dom>>()
    } else {
        Vec::new()
    };

    html!("li", {
        .class("list-group-item")
        .child(html!("div", {
            .class("row")
            .children([
                html!("div", {
                    .class([tab_class,"text-end"])
                    .child(html!("h5", {
                        .child(html!("div", {
                            .apply(|dom| if sources.iter().any(|s| s.is_used()) {
                                dom.class(["text-success","fas",icon_checked]) // white check on green icon
                            } else {
                                dom.class(["text-secondary","far",icon_unchecked]) // grey bordered icon
                            })
                        }))
                    }))
                }),
                html!("div", {
                    .class(label_class)
                    .child(html!("label", {.text(label)}))
                    .children(badges)
                    .child(html!("i", {
                        .class(class::FA_SPIN_R)
                        .visible_signal(map_ref!{
                            let loading = app.loader_is_loading(),
                            let path = pdf_path_mutable.signal_cloned() =>
                            *loading && sources.iter().any(|s| {
                                if let (Some(a), Some(b)) = (s.inner().map(|t| t.report), path.clone().map(|t| t.report)) {
                                    a == b
                                } else {
                                    false
                                }
                            })
                        })
                    }))
                }),
            ])
        }))
    })
}
