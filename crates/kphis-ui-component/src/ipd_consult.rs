// ipd-dr-consult.php

use dominator::{Dom, clone, events, html, text};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use time::Duration;

use kphis_model::{
    app::VisitTypeId,
    ipd::consult::ConsultWithName,
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::Modal, class, doms};
use kphis_util::{
    datetime::{date_th_opt, datetime_th_opt, js_now, time_hm_opt},
    util::zero_none,
};

use crate::{
    gadget::pdf_button::PdfButtons,
    modal::{
        blank_modal,
        consult_form::{ConsultForm, ConsultFormMode},
    },
};

/// - GET `EndPoint::IpdConsultAn`
/// - GET `EndPoint::IpdConsultId` (ConsultForm)
#[derive(Default)]
pub struct IpdConsultCpn {
    loaded: Mutable<bool>,
    changed: Mutable<bool>,
    checker: Mutable<bool>,

    focused: Mutable<bool>,
    focus: Mutable<bool>,
    sub: Mutable<String>,
    focused_id: Mutable<u32>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    consults: MutableVec<Rc<ConsultWithName>>,
    consult_form_modal: Mutable<Option<Rc<ConsultForm>>>,
}

impl IpdConsultCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, sub: Mutable<String>, focused_id: Mutable<u32>) -> Rc<Self> {
        Rc::new(Self {
            sub,
            focused_id,
            patient,
            ..Default::default()
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        match visit_type {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                app.async_load(
                    true,
                    clone!(app => async move {
                        // fetch by 'an' will return 'string_consult_request_name' and 'string_consult_reply_name' in 'code^name^code2|code^name^code2' format
                        // GET `EndPoint::IpdConsultAn`
                        match ConsultWithName::call_api_get(&an, app.state()).await {
                            Ok(response) => {
                                page.checker.set_neq(!response.is_empty());
                                let mut lock = page.consults.lock_mut();
                                lock.clear();
                                lock.extend(response.into_iter().map(Rc::new));
                                page.focus.set(true);
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }),
                );
            }
            Some(VisitTypeId::OpdEr(_, _)) | Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                    page.loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                    page.changed.set_neq(false);
                }
                async {}
            })))
            .future(map_ref!{
                let focused = page.focused.signal(),
                let focus = page.focus.signal() =>
                *focus && !focused
            }.for_each(clone!(page => move |ready| {
                if ready {
                    if let Some(id) = zero_none(page.focused_id.get()) {
                        match page.sub.lock_ref().as_str() {
                            "reply" => {
                                page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), Some(id), ConsultFormMode::Reply)));
                            }
                            _ => {
                                page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), Some(id), ConsultFormMode::View)));
                            }
                        }
                        Modal::new("#consultFormModal").show();
                    }
                    // focus only once
                    // page.focus.set(false);
                    page.focused.set(true);
                }
                async {}
            })))
            .children([
                html!("div", {
                    .class(class::ROW)
                    .child(html!("div", {
                        .class("col")
                        //.attr("id", "openConsultActionButtonCol")
                        .child(html!("div", {
                            .class("float-end")
                            .apply_if(app.has_permission(Permission::IpdDoctorConsultAdd) || is_pre_admit, |dom| {
                                dom.child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_L_GRAY)
                                    //.attr("id", "openConsultActionButton")
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", "#consultFormModal")
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" เพิ่มใบ consult")
                                    .event(clone!(page => move |_: events::Click| {
                                        page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), None, ConsultFormMode::Edit)));
                                    }))
                                }))
                            })
                            .apply_if(app.has_permission(Permission::IpdDoctorConsultPrint) || is_pre_admit, clone!(page, app => move |dom| {
                                dom.children_signal_vec(clone!(page, app => page.patient.signal_cloned().map(move |opt| {
                                    if let Some(patient) = opt {
                                        match patient.visit_type() {
                                            VisitTypeId::Ipd(an)
                                            | VisitTypeId::PreAdmit(an) => {
                                                PdfButtons::buttons(
                                                    PdfButtons::new(
                                                        TypstReport::from_system_with_coercion(SystemReport::IpdConsult, &app.state().report_coercions()),
                                                        Mutable::new(an.clone()),
                                                        page.checker.clone(),
                                                        page.changed.clone(),
                                                        clone!(page => move || {serde_json::json!({
                                                            "id": an,
                                                            "patient": patient,
                                                            "consults": page.consults.lock_ref().to_vec(),
                                                        }).to_string()})
                                                    ), "Print PDF", None, app.clone()
                                                )
                                            }
                                            VisitTypeId::OpdEr(_, _)
                                            | VisitTypeId::Visit(_) => Vec::new(),
                                        }
                                    } else {
                                        Vec::new()
                                    }
                                }).to_signal_vec()))
                            }))
                        }))
                    }))
                }),
                html!("div", {
                    .class(class::ROW)
                    .child(html!("div", {
                        .class("col-sm")
                        .child(doms::badge_info_center("หลังการตอบ Consult ครั้งแรก เกิน 24 ชม. จะไม่สามารถตอบ Consult เพิ่มเติมได้"))
                    }))
                }),
                doms::table_responsive(class::TABLE_STRIP, clone!(app, page => move |table| { table
                    //.attr("id", "consult-table")
                    .children([
                        html!("thead", {
                            .class("text-center")
                            .children([
                                html!("tr", {
                                    .children([
                                        html!("th", {
                                            .attr("rowspan", "2")
                                            .attr("width", "20%")
                                            .text("ชื่อใบ consult")
                                        }),
                                        html!("th", {
                                            .attr("rowspan", "2")
                                            .attr("width", "7%")
                                            .text("EMERGENCY")
                                        }),
                                        html!("th", {
                                            .attr("colspan", "2")
                                            .text("ข้อมูลผู้ Consult")
                                        }),
                                        html!("th", {
                                            .attr("rowspan", "2")
                                            .attr("width", "3%")
                                            .text("แก้ไข")
                                        }),
                                        html!("th", {
                                            .attr("colspan", "3")
                                            .class("bg-success-subtle")
                                            .text("ข้อมูลผู้รับ Consult")
                                        }),
                                        html!("th", {
                                            .attr("colspan", "2")
                                            .attr("width", "10%")
                                            .text("สถานะ")
                                        }),
                                    ])
                                }),
                                html!("tr", {
                                    .children([
                                        html!("th", {
                                            .text("แพทย์")
                                        }),
                                        html!("th", {
                                            .text("วันที่ Consult")
                                        }),
                                        html!("th", {
                                            .class("bg-success-subtle")
                                            .text("แพทย์")
                                        }),
                                        html!("th", {
                                            .class("bg-success-subtle")
                                            .text("วันที่ตอบ Consult")
                                        }),
                                        html!("th", {
                                            .class("bg-success-subtle")
                                            .text("วันที่อัพเดทล่าสุด")
                                        }),
                                        html!("th", {
                                            .text("สถานะ")
                                        }),
                                        html!("th", {
                                            .text("ตอบกลับ")
                                        }),
                                    ])
                                }),
                            ])
                        }),
                        // ipd-dr-consult-data.php
                        html!("tbody", {
                            //.attr("id", "consult-table-row")
                            .children_signal_vec(page.consults.signal_vec_cloned().map(clone!(app, page => move |consult| {
                                render_consult(consult, page.clone(), app.clone())
                            })))
                        }),
                    ])
                })),
            ])
            // ipd-dr-consult-form.php
            .child(html!("div", {
                .class("modal")
                .attr("id", "consultFormModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.consult_form_modal.signal_cloned().map(clone!(app, page => move |opt| {
                    opt.as_ref().map(clone!(app, page => move |modal| {
                        ConsultForm::render(modal.clone(), page.consult_form_modal.clone(), page.changed.clone(), app)
                    })).or(Some(blank_modal()))
                })))
            }))
        })
    }
}

fn render_consult(row: Rc<ConsultWithName>, page: Rc<IpdConsultCpn>, app: Rc<App>) -> Dom {
    let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
    let is_request_user = row
        .string_consult_request_name
        .clone()
        .map(|reqs| {
            reqs.split(',')
                .flat_map(|row| row.split('/').map(|s| s.trim()).collect::<Vec<&str>>())
                .any(|name| name == app.doctor_name().unwrap_or_default().trim())
        })
        .unwrap_or_default();

    let html_consult_request_name = row
        .string_consult_request_name
        .clone()
        .map(|reqs| {
            reqs.split(',')
                .map(|req| {
                    html!("div", {
                        .class(class::TRUNC_SM)
                        .style("max-width","240px")
                        .text(req)
                    })
                })
                .collect::<Vec<Dom>>()
        })
        .unwrap_or_default();

    let html_consult_reply_name = row
        .string_consult_reply_name
        .clone()
        .map(|reps| {
            // reps.split('|').map(|rep| {
            //     let concat = rep.split('^').collect::<Vec<&str>>();
            //     // names, crete_datetime, update_datetime
            //     if concat.len() == 3 {
            //         html!("div", {
            //             .class(class::TRUNC_SM)
            //             .style("max-width","240px")
            //             .text(concat[0])
            //         })
            //     } else {
            //         html!("div")
            //     }
            // })
            // .collect::<Vec<Dom>>()
            reps.split(',')
                .map(|rep| {
                    html!("div", {
                        .class(class::TRUNC_SM)
                        .style("max-width","240px")
                        .text(rep)
                    })
                })
                .collect::<Vec<Dom>>()
        })
        .unwrap_or_default();

    let reply_over_24hr = row.consult_datetime_create_reply.map(|create_reply| (js_now() - create_reply) >= Duration::DAY).unwrap_or_default();

    let consult_status = match &row.consult_status {
        Some(v) => {
            if v == "Y" {
                html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)})
            } else {
                html!("i", {.class(class::FA_HOURGLASS_GOLD)})
            }
        }
        None => html!("i", {.class(class::FA_HOURGLASS_GOLD)}),
    };

    html!("tr", {
        .attr("id", &["consult_id_", &row.consult_id.to_string(), "_div"].concat())
        .apply_if(page.focused_id.get() == row.consult_id, |dom| dom.class(class::BORDER3_RED))
        .children([
            html!("td", {
                .style("cursor","pointer")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#consultFormModal")
                .text(&[row.consult_type_name.clone().unwrap_or_default(), row.spcltyname.as_ref().map(|spcltyname| [" (ส่งแผนก ", spcltyname, ")"].concat()).unwrap_or_default()].concat())
                .event(clone!(row, page => move |_: events::Click| {
                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::View)));
                }))
            }),
            html!("td", {
                .style("cursor","pointer")
                .class("text-center")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#consultFormModal")
                .text(&row.consult_emergency_name.clone().unwrap_or_default())
                .event(clone!(row, page => move |_: events::Click| {
                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::View)));
                }))
            }),
            html!("td", {
                .style("cursor","pointer")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#consultFormModal")
                .attr("title", &row.string_consult_request_name.clone().unwrap_or_default())
                .children(html_consult_request_name)
                .event(clone!(row, page => move |_: events::Click| {
                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::View)));
                }))
            }),
            html!("td", {
                .style("cursor","pointer")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#consultFormModal")
                .class("text-center")
                .children([
                    text(&date_th_opt(&row.consult_date)),
                    html!("br"),
                    text(&time_hm_opt(&row.consult_time)),
                ])
                .event(clone!(row, page => move |_: events::Click| {
                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::View)));
                }))
            }),
            html!("td", {
                .class("text-center")
                .apply_if(app.has_permission(Permission::IpdDoctorConsultEdit) || is_pre_admit, |dom| { // && is_request_user,
                    dom.child(html!("button", {
                        .attr("type", "button")
                        .attr("data-bs-toggle", "modal")
                        .attr("data-bs-target", "#consultFormModal")
                        .class(class::BTN_SM_GRAY)
                        .apply(|dom| {
                            if !html_consult_reply_name.is_empty() {
                                dom.attr("disabled", "")
                            } else {
                                dom.event(clone!(row, page => move |_: events::Click| {
                                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::Edit)));
                                }))
                            }
                        })
                        .child(html!("i", {.class(class::FA_EDIT)}))
                    }))
                })
            }),
            html!("td", {
                .class("bg-info-subtle")
                .style("cursor","pointer")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#consultFormModal")
                .attr("title", &row.string_consult_reply_name.clone().unwrap_or_default())
                .children(html_consult_reply_name)
                .event(clone!(row, page => move |_: events::Click| {
                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::View)));
                }))
            }),
            html!("td", {
                .class("bg-info-subtle")
                .style("cursor","pointer")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#consultFormModal")
                .class("text-center")
                .text(&datetime_th_opt(&row.consult_datetime_create_reply))
                .event(clone!(row, page => move |_: events::Click| {
                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::View)));
                }))
            }),
            html!("td", {
                .class("bg-info-subtle")
                .style("cursor","pointer")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#consultFormModal")
                .class("text-center")
                .text(&datetime_th_opt(&row.consult_datetime_update_reply))
                .event(clone!(row, page => move |_: events::Click| {
                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::View)));
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(consult_status)
            }),
            html!("td", {
                .class("text-center")
                .apply_if(app.has_permission(Permission::IpdDoctorConsultEdit) || is_pre_admit, |dom| {
                    dom.child(html!("button", {
                        .attr("type", "button")
                        .attr("data-bs-toggle", "modal")
                        .attr("data-bs-target", "#consultFormModal")
                        .class(class::BTN_SM_GRAY)
                        .apply(|dom| {
                            if reply_over_24hr {
                                dom.attr("disabled", "")
                            } else if is_request_user {
                                dom.attr("title", "ผู้ขอปรึกษา ไม่สามารถตอบข้อมูลได้").attr("disabled", "")
                            } else {
                                dom.event(clone!(row, page => move |_: events::Click| {
                                    page.consult_form_modal.set(Some(ConsultForm::new(page.patient.clone(), zero_none(row.consult_id), ConsultFormMode::Reply)));
                                }))
                            }
                        })
                        .child(html!("i", {.class(class::FA_SHARE)}))
                    }))
                })
            }),
        ])
    })
}
