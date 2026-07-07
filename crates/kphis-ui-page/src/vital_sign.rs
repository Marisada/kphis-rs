use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    app::VisitTypeId,
    avatar::{AvatarEnum, AvatarOpdEr, AvatarParams, AvatarWard},
    patient_info::PatientInfo,
    user::permission::Permission,
    vital_sign::{VitalSign, VsMode},
};
use kphis_ui_app::App;
use kphis_ui_component::{
    show_patient_main::ShowPatientMainCpn,
    vital_sign::{vital_sign_data::VitalSignDataCpn, vital_sign_form::VitalSignFormCpn},
};
use kphis_ui_core::{binding::NiceSelect, class, doms};
use kphis_util::util::{str_some, zero_none};

/// - GET `EndPoint::AvatarIpd`
/// - GET `EndPoint::AvatarOpdEr`
/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainId` (ShowPatientMainCpn)
/// - GET `EndPoint::IpdVitalSign` (VitalSignDataCpn)
/// - GET `EndPoint::OpdErVitalSign` (VitalSignDataCpn)
#[derive(Clone, Default)]
pub struct VitalSignPage {
    is_ipd: bool,
    ward: Mutable<String>,
    search: Mutable<String>,

    search_result: MutableVec<Rc<AvatarEnum>>,
    search_changed: Mutable<bool>,
    search_selected: Mutable<Option<VisitTypeId>>,

    patient: Mutable<Mutable<Option<Rc<PatientInfo>>>>,
    patient_loaded: Mutable<Mutable<bool>>,
    // an: Mutable<String>,
    // patient: Mutable<Rc<IpdShowPatientMainCpn>>,
    pub vs_changed: Mutable<bool>,
    vs_result: Mutable<Vec<Rc<VitalSign>>>,
    vs_data: Mutable<Option<Rc<VitalSignDataCpn>>>,
    start_vs_date: Mutable<String>,
    end_vs_date: Mutable<String>,
    vs_mode: Mutable<VsMode>,

    pub vs_id: Mutable<u32>,
    pub form_rendered: Mutable<bool>,
    form: Mutable<Option<Rc<VitalSignFormCpn>>>,
}

impl VitalSignPage {
    pub fn new(is_ipd: bool, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            is_ipd,
            ward: if is_ipd { app.ward_select.clone() } else { Mutable::new(String::new()) },
            ..Default::default()
        })
    }

    fn is_visit_type_valid(&self) -> bool {
        self.patient.lock_ref().lock_ref().as_ref().map(|pt| !pt.visit_type().is_empty()).unwrap_or_default()
    }

    // ipd-vital-sign-show-data-patient.php?ward=" + ward;
    fn submit_search(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if page.is_ipd {
                    let params = AvatarParams {
                        ward: str_some(page.ward.get_cloned()),
                        search: str_some(page.search.get_cloned()),
                    };
                    if params.is_empty() {
                        page.search_result.lock_mut().clear();
                    } else {
                        // GET `EndPoint::AvatarIpd`
                        match AvatarWard::call_api_get(&params, app.state()).await {
                            Ok(items) => {
                                let mut lock = page.search_result.lock_mut();
                                lock.clear();
                                lock.extend(items.iter().map(|im| Rc::new(AvatarEnum::from(im))));
                                if items.len() == 1 {
                                    page.search_selected.set(items.first().map(|i| i.visit_type(app.hosxp_an_len())));
                                } else {
                                    page.search_selected.set(None);
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                } else {
                    // GET `EndPoint::AvatarOpdEr`
                    match AvatarOpdEr::call_api_get(app.state()).await {
                        Ok(items) => {
                            let mut lock = page.search_result.lock_mut();
                            lock.clear();
                            lock.extend(items.iter().map(|im| Rc::new(AvatarEnum::from(im))));
                            if items.len() == 1 {
                                page.search_selected.set(items.first().map(|i| i.visit_type()));
                            } else {
                                page.search_selected.set(None);
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

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title(if page.is_ipd { "KPHIS - IPD Vital Sign" } else { "KPHIS - Opd-ER Vital Sign" });

        let can_use_form = if page.is_ipd {
            app.has_permission(Permission::VitalSignAdd) || app.has_permission(Permission::VitalSignEdit)
        } else {
            app.has_permission(Permission::OpdErVitalSignAdd) || app.has_permission(Permission::OpdErVitalSignEdit)
        };

        let ward_select_option = if page.is_ipd {
            app.app_asset.lock_ref().as_ref().map(|asset| asset.ward_select_option.clone()).unwrap_or_default()
        } else {
            Vec::new()
        };

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |loaded| {
                if loaded {
                    if page.is_ipd {
                        if let Some(elm) = app.get_id("wards") {
                            NiceSelect::new_default(&elm);
                        }
                    }
                    page.search_changed.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.search_changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit_search(page.clone(), app.clone());
                    page.search_changed.set(false);
                }
                async {}
            })))
            .future(page.patient_loaded.signal_cloned().map(|patient_loaded| patient_loaded.signal()).flatten().for_each(clone!(page => move |loaded| {
                if loaded {
                    page.vs_id.set_neq(0);
                    page.vs_changed.set(true);
                    page.form_rendered.set(false);
                }
                async {}
            })))
            .future(page.vs_changed.signal().for_each(clone!(page => move |changed| {
                if changed {
                    let vs_data_opt = page.is_visit_type_valid().then(|| {
                        VitalSignDataCpn::new(
                            page.patient.get_cloned(),
                            page.vs_result.clone(),
                            page.start_vs_date.clone(),
                            page.end_vs_date.clone(),
                            page.vs_mode.clone(),
                        )
                    });
                    page.vs_data.set(vs_data_opt);
                    page.vs_changed.set(false);
                }
                async {}
            })))
            .future(page.form_rendered.signal().for_each(clone!(app, page => move |done| {
                if !done {
                    let form_opt = page.is_visit_type_valid().then(|| {
                        let vs_opt = zero_none(page.vs_id.get()).and_then(|id| {
                            page.vs_result.lock_ref().iter().find(|vs| vs.vs_id == id).cloned()
                        });
                        VitalSignFormCpn::new(
                            page.patient.get_cloned(),
                            vs_opt,
                            app.clone(),
                        )
                    });
                    page.form.set(form_opt);
                    page.form_rendered.set(true);
                }
                async {}
            })))
            .class(class::CONF_B)
            .child(html!("div", {
                .class("row")
                .children([
                    // left panel
                    html!("div", {
                        .style("width","350px")
                        .apply_if(page.is_ipd, |dom| { dom
                            .children([
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM)
                                    .children([
                                        doms::span_group_text("Ward"),
                                        html!("div", {
                                            .class(class::FLEX_W100)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "wards")
                                                .children(ward_select_option.iter().map(|option| {
                                                    doms::select_option(option, &page.ward.lock_ref())
                                                }))
                                                .prop_signal("value", page.ward.signal_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(app, page, element => move |_: events::Change| {
                                                        page.ward.set_neq(element.value());
                                                        app.to_local_storage();
                                                        page.search_changed.set_neq(true);
                                                    }))
                                                })
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(app, page => move |_:events::Click| {
                                                let no_ward = page.ward.lock_ref().is_empty();
                                                if !no_ward {
                                                    page.ward.set(String::new());
                                                    if let Some(elm) = app.get_id("wards") {
                                                        NiceSelect::new_default(&elm);
                                                    }
                                                    page.search_changed.set_neq(true);
                                                }
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM)
                                    .class("mt-2")
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .focused(true)
                                            .attr("placeholder", "HN/AN/ชื่อ-สกุล")
                                            .prop_signal("value", page.search.signal_cloned())
                                            .with_node!(element => {
                                                .event_with_options(&EventOptions::preventable(), clone!(page, element => move |event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        event.prevent_default();
                                                        page.search.set_neq(element.value());
                                                        page.search_changed.set_neq(true);
                                                    }
                                                }))
                                                .event(clone!(page => move |_: events::Change| {
                                                    page.search.set_neq(element.value());
                                                }))
                                            })
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(page => move |_:events::Click| {
                                                let no_search = page.search.lock_ref().is_empty();
                                                if !no_search {
                                                    page.search.set(String::new());
                                                    page.search_changed.set_neq(true);
                                                }
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_BLUE)
                                            .text("ค้นหา")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.search_changed.set_neq(true);
                                            }))
                                        }),
                                    ])
                                }),
                            ])
                        })
                        .child(html!("div", {
                            //.attr("id", "show-patient")
                            .style("height","calc(100vh - 165px)")
                            .style("width", "100%")
                            .style("box-sizing","border-box")
                            .style("overflow-y","auto")
                            .apply_if(page.is_ipd, |dom| { dom.class("mt-2")})
                            .children([
                                html!("table", {
                                    .class(class::TABLE_STRIP)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .child(html!("th", {
                                                    .attr("scope", "col")
                                                    .class("th-sm")
                                                    .text("รายชื่อผู้ป่วย")
                                                }))
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app, page => move |row| {
                                                doms::render_avatar(row, page.search_selected.clone(), app.state())
                                            })))
                                        }),
                                    ])
                                })
                            ])
                        }))
                    }),
                    // middle + right panel
                    html!("div", {
                        .class(class::COL_PS0)
                        .style("height", "calc(100vh - 90px)")
                        .style("box-sizing", "border-box")
                        .style("overflow-y", "auto")
                        .children([
                            // TOP: PATIENT DATA
                            html!("div", {
                                //.attr("id", "ipd-show-patient-main")
                                .child_signal(page.search_selected.signal_cloned().map(clone!(app, page => move |opt| {
                                    opt.as_ref().and_then(clone!(page, app => move |visit_type| {
                                        let show_patient_main_opt = match visit_type {
                                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (!an.is_empty()).then(|| {
                                                ShowPatientMainCpn::new_with_an(an.to_owned())
                                            }),
                                            VisitTypeId::OpdEr(_vn, opd_er_order_master_id) => (*opd_er_order_master_id > 0).then(|| {
                                                ShowPatientMainCpn::new_with_id(*opd_er_order_master_id)
                                            }),
                                            VisitTypeId::Visit(vn) => (!vn.is_empty()).then(|| {
                                                ShowPatientMainCpn::new_with_vn(vn.to_owned())
                                            }),
                                        };
                                        show_patient_main_opt.map(|show_patient_main| {
                                            let dom = ShowPatientMainCpn::render(true, show_patient_main.clone(), app);
                                            page.patient.set(show_patient_main.patient.clone());
                                            page.patient_loaded.set(show_patient_main.loaded.clone());
                                            dom
                                        })
                                    }))
                                })))
                            }),
                            // BODY: VITAL SIGN DATA + FORM
                            html!("div", {
                                .class(class::FLEX_B2)
                                .child(html!("div", {
                                    .class("me-1")
                                    .apply_if(can_use_form, |dom| dom.style("width","calc(100% - 420px)"))
                                    .style("overflow-x","auto")
                                    .child_signal(page.vs_data.signal_cloned().map(clone!(app, page => move |vs_data_opt| {
                                        vs_data_opt.as_ref().map(clone!(app, page => move |vs_data| VitalSignDataCpn::render(page.vs_id.clone(), page.form_rendered.clone(), vs_data.clone(), app)))
                                    })))
                                }))
                                //SessionManager::checkPermission('VITAL_SIGN','ADD') || SessionManager::checkPermission('VITAL_SIGN','EDIT')
                                .apply_if(can_use_form, |dom| {
                                    dom.child_signal(page.form.signal_cloned().map(clone!(app, page => move |form_opt| {
                                        form_opt.as_ref().map(clone!(app, page => move |form| VitalSignFormCpn::render(page.vs_id.clone(), page.vs_changed.clone(), page.form_rendered.clone(), form.clone(), app)))
                                    })))
                                })
                            }),
                        ])
                    }),
                ])
            }))
        })
    }
}
