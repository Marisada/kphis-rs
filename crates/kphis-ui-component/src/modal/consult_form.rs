use dominator::{Dom, clone, events, html, text};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{ops::Deref, rc::Rc};
use time::Duration;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::ImageUsage,
    ipd::consult::{Consult, ConsultParams, ConsultSave, DoctorCodeSave},
    patient_info::PatientInfo,
    route::Route,
    sse::SsePostMessage,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, datetime_8601, datetime_str_th, js_now, time_8601},
    util::{str_some, zero_none},
};

use crate::gadget::image::{ImageCpn, ImagePaths};

/// - GET `EndPoint::IpdConsultId`
/// - POST `EndPoint::IpdConsult` (guarded, remove Save btn)
/// - DELETE `EndPoint::IpdConsult` (guarded, remove Delete btn)
/// - POST `EndPoint::ImageUsage` (guarded, remove request-image gadget)
#[derive(Clone, Default)]
pub struct ConsultForm {
    patient: Mutable<Option<Rc<PatientInfo>>>,

    consult_mode: ConsultFormMode,
    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    // image component callback state
    data_image_callback: Mutable<ImagePaths>,

    an: Mutable<String>,
    is_pre_admit: bool,
    consult_id: Mutable<u32>,
    version: Mutable<i32>,

    consult_type: Mutable<String>,
    consult_ward: Mutable<String>,
    consult_emergency: Mutable<String>,
    consult_doctorcode_mention: Mutable<String>,
    consult_spclty: Mutable<String>,
    consult_date: Mutable<String>,
    consult_time: Mutable<String>,
    consult_data: Mutable<String>,

    consult_datetime_create_reply: Mutable<String>,
    consult_datetime_update_reply: Mutable<String>,
    consult_finding: Mutable<String>,
    consult_diagnosis: Mutable<String>,
    consult_recommendation: Mutable<String>,

    consult_requests: MutableVec<Rc<Doctors>>,
    consult_replies: MutableVec<Rc<Doctors>>,
}

impl ConsultForm {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, consult_id: Option<u32>, consult_mode: ConsultFormMode) -> Rc<Self> {
        let (an, is_pre_admit) = patient.lock_ref().as_ref().and_then(|pt| pt.visit_type.an_and_is_pre_admit_owned()).unwrap_or_default();
        Rc::new(Self {
            patient,
            an: Mutable::new(an),
            is_pre_admit,
            consult_id: Mutable::new(consult_id.unwrap_or_default()),
            consult_mode,
            ..Default::default()
        })
    }

    fn is_not_requested_user(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.consult_requests
            .signal_vec_cloned()
            .filter(move |req| req.doctorcode.lock_ref().as_str() == &app.doctor_code().unwrap_or_default())
            .is_empty()
    }

    fn is_not_replied_user(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.consult_replies
            .signal_vec_cloned()
            .filter(move |req| req.doctorcode.lock_ref().as_str() == &app.doctor_code().unwrap_or_default())
            .is_empty()
    }

    fn is_reply_over_24hr(&self) -> impl Signal<Item = bool> + use<> {
        self.consult_datetime_create_reply
            .signal_cloned()
            .map(|create_reply| datetime_8601(&create_reply).map(|dt| (js_now() - dt) >= Duration::DAY).unwrap_or_default())
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        if modal.consult_id.get() > 0 {
            app.async_load(
                true,
                clone!(app => async move {
                    // fetch by 'id' will return 'string_consult_request_name' and 'string_consult_reply_name' in 'name / name' format
                    // GET `EndPoint::IpdConsultId`
                    match Consult::call_api_get(modal.consult_id.get(), app.state()).await {
                        Ok(response) => {
                            if let Some(consult) = response {
                                let consult_ward = consult.consult_ward.unwrap_or_default();
                                let consult_doctorcode_mention = consult.consult_doctorcode_mention.unwrap_or_default();
                                let consult_spclty = consult.consult_spclty.unwrap_or_default();
                                if matches!(modal.consult_mode, ConsultFormMode::Edit) {
                                    if let Some(elm) = app.get_id("consult_ward") {
                                        NiceSelect::new_default_with_value(&elm, &consult_ward);
                                    }
                                    if let Some(elm) = app.get_id("consult_doctorcode_mention") {
                                        NiceSelect::new_default_with_value(&elm, &consult_doctorcode_mention);
                                    }
                                    if let Some(elm) = app.get_id("consult_spclty") {
                                        NiceSelect::new_default_with_value(&elm, &consult_spclty);
                                    }
                                }

                                modal.an.set_neq(consult.an);
                                modal.consult_id.set_neq(consult.consult_id);
                                modal.consult_type.set_neq(consult.consult_type.map(|i| i.to_string()).unwrap_or_default());
                                modal.consult_ward.set_neq(consult_ward);
                                modal.consult_emergency.set_neq(consult.consult_emergency.unwrap_or_default());
                                modal.consult_doctorcode_mention.set_neq(consult_doctorcode_mention);
                                modal.consult_spclty.set_neq(consult_spclty);
                                modal.consult_date.set_neq(consult.consult_date.map(|d| d.to_string()).unwrap_or_default());
                                modal.consult_time.set_neq(consult.consult_time.map(|t| t.js_string()).unwrap_or_default());
                                modal.consult_data.set_neq(consult.consult_data.unwrap_or_default());

                                modal.consult_datetime_create_reply.set_neq(consult.consult_datetime_create_reply.map(|dt| dt.js_string()).unwrap_or_default());
                                modal.consult_datetime_update_reply.set_neq(consult.consult_datetime_update_reply.map(|dt| dt.js_string()).unwrap_or_default());
                                modal.consult_finding.set_neq(consult.consult_finding.unwrap_or_default());
                                modal.consult_diagnosis.set_neq(consult.consult_diagnosis.unwrap_or_default());
                                modal.consult_recommendation.set_neq(consult.consult_recommendation.unwrap_or_default());
                                modal.version.set_neq(consult.version);

                                let doctorcode = app.doctor_code().unwrap_or_default();
                                let doctorname = app.doctor_name().unwrap_or_default();
                                let mut requests = modal.consult_requests.lock_mut();
                                requests.clear();
                                requests.extend(Doctors::new(&consult.string_consult_request_name.unwrap_or_default()));
                                if matches!(modal.consult_mode, ConsultFormMode::Edit) && (
                                    requests.is_empty() || !requests.iter().any(|req| req.doctorcode.lock_ref().deref() == &doctorcode)
                                ) {
                                    requests.push_cloned(Rc::new(Doctors {
                                        doctorname: Mutable::new(doctorname.clone()),
                                        doctorcode: Mutable::new(doctorcode.clone()),
                                        doctorcode2: Mutable::new(String::new()),
                                    }));
                                }

                                let mut replies = modal.consult_replies.lock_mut();
                                replies.clear();
                                replies.extend(Doctors::new(&consult.string_consult_reply_name.unwrap_or_default()));
                                if matches!(modal.consult_mode, ConsultFormMode::Reply) && (
                                    replies.is_empty() || !replies.iter().any(|rep| rep.doctorcode.lock_ref().deref() == &doctorcode)
                                ) {
                                    replies.push_cloned(Rc::new(Doctors {
                                        doctorname: Mutable::new(doctorname),
                                        doctorcode: Mutable::new(doctorcode),
                                        doctorcode2: Mutable::new(String::new()),
                                    }));
                                }
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        } else {
            let mut requests = modal.consult_requests.lock_mut();
            requests.clear();
            requests.push_cloned(Rc::new(Doctors {
                doctorname: Mutable::new(app.doctor_name().unwrap_or_default()),
                doctorcode: Mutable::new(app.doctor_code().unwrap_or_default()),
                doctorcode2: Mutable::new(String::new()),
            }));
        }
    }

    // ipd-dr-consult-save.php or ipd-dr-consult-update.php
    fn save(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, changed: Mutable<bool>, app: Rc<App>) {
        let mut save = ConsultSave {
            consult_id: zero_none(modal.consult_id.get()),
            consult_mode: modal.consult_mode.str().to_owned(),

            an: modal.an.get_cloned(),
            consult_type: modal.consult_type.lock_ref().parse::<u32>().ok().and_then(zero_none),
            consult_ward: str_some(modal.consult_ward.get_cloned()),
            consult_emergency: str_some(modal.consult_emergency.get_cloned()),
            consult_doctorcode_mention: str_some(modal.consult_doctorcode_mention.get_cloned()),
            consult_spclty: modal.consult_spclty.lock_ref().parse::<u32>().ok(),
            consult_date: date_8601(&modal.consult_date.lock_ref()),
            consult_time: time_8601(&modal.consult_time.lock_ref()),
            consult_data: str_some(modal.consult_data.get_cloned()),

            consult_doctorcode_requests: modal
                .consult_requests
                .lock_ref()
                .iter()
                .map(|request| DoctorCodeSave {
                    person1: str_some(request.doctorcode.get_cloned()),
                    person2: str_some(request.doctorcode2.get_cloned()),
                })
                .collect(),

            consult_finding: str_some(modal.consult_finding.get_cloned()),
            consult_diagnosis: str_some(modal.consult_diagnosis.get_cloned()),
            consult_recommendation: str_some(modal.consult_recommendation.get_cloned()),

            consult_doctorcode_replies: modal
                .consult_replies
                .lock_ref()
                .iter()
                .map(|request| DoctorCodeSave {
                    person1: str_some(request.doctorcode.get_cloned()),
                    person2: str_some(request.doctorcode2.get_cloned()),
                })
                .collect(),

            version: modal.version.get(),
        };

        app.async_load(
            true,
            clone!(app => async move {
                // POST `EndPoint::IpdConsult`
                match save.call_api_post(app.state()).await {
                    Ok((id, responses)) => {
                        app.alert_execute_responses(&responses, clone!(app => async move {
                            // update images
                            // POST `EndPoint::ImageUsage`
                            modal.data_image_callback.lock_ref().post_images(ImageUsage::IpdConsultData, id, app.clone()).await;
                            // SSE message
                            save.consult_id = Some(id);
                            send_sse_by_save(&save, modal.patient.get_cloned(), app);
                            // clearing
                            changed.set_neq(true);
                            display.set(None);
                        })).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    // ipd-nurse-index-note-delete.php
    fn delete(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, changed: Mutable<bool>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยันลบรายการ").await {
                    let consult_id = zero_none(modal.consult_id.get());
                    let version = zero_none(modal.version.get());
                    if consult_id.is_some() && version.is_some() {
                        let params = ConsultParams {
                            consult_id,
                            version,
                        };
                        // DELETE `EndPoint::IpdConsult`
                        match Consult::call_api_delete(&params, app.state()).await {
                            Ok(responses) => {
                                app.alert_execute_responses(&responses, async move {
                                    changed.set_neq(true);
                                    display.set(None);
                                }).await;
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }
            }),
        )
    }

    pub fn render(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, changed: Mutable<bool>, app: Rc<App>) -> Dom {
        let is_new = modal.consult_id.get() == 0;

        let (all_doctor_select_option, doctor_select_option, spclty_kphis_select_option, emergency_select_option, ward_select_option, consult_type_select_option) =
            match app.app_asset.lock_ref().as_ref() {
                Some(assets_arc) => {
                    let asset = assets_arc.as_ref().to_owned();
                    (
                        asset.all_doctor_select_option,
                        asset.doctor_select_option,
                        asset.spclty_kphis_select_option,
                        asset.emergency_select_option,
                        asset.ward_select_option,
                        asset.consult_type_select_option,
                    )
                }
                None => (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            };

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load(modal.clone(), app.clone());
                    modal.loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_LG)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h4", {
                                .class("modal-title")
                                .child(html!("lable", {
                                    .class("fw-bold")
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" Consultation")
                                }))
                            }),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        .child(html!("div", {
                            .children([
                                html!("div", {
                                    .class("row")
                                    .children([
                                        html!("div", {
                                            .class("col-sm-6")
                                            .child(html!("div", {
                                                .class("mb-3")
                                                .children([
                                                    html!("label", {
                                                        .attr("for", "consult_type")
                                                        .text("ชนิดใบ Consult")
                                                    }),
                                                    html!("select" => HtmlSelectElement, {
                                                        .class("form-select")
                                                        .attr("id", "consult_type")
                                                        .child(html!("option", {
                                                            .attr("value", "")
                                                            .text("เลือก")
                                                        }))
                                                        .children(consult_type_select_option.iter().map(|option| {
                                                            doms::select_option(option, "")
                                                        }))
                                                        .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| dom.attr("disabled",""))
                                                        .apply(mixins::string_value_select(modal.consult_type.clone(), modal.changed.clone()))
                                                    }),
                                                ])
                                            }))
                                        }),
                                        html!("div", {
                                            .class("col-sm-6")
                                            .child(html!("div", {
                                                .class("mb-3")
                                                .children([
                                                    html!("label", {
                                                        .attr("for", "consult_ward")
                                                        .text("Ward")
                                                    }),
                                                    html!("select" => HtmlSelectElement, {
                                                        .class("form-control")
                                                        .attr("id", "consult_ward")
                                                        .child(html!("option", {
                                                            .attr("value", "")
                                                            .text("เลือก")
                                                        }))
                                                        .children(ward_select_option.iter().map(|option| {
                                                            doms::select_option(option, "")
                                                        }))
                                                        .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| dom.attr("disabled",""))
                                                        .apply(mixins::string_value_select(modal.consult_ward.clone(), modal.changed.clone()))
                                                    }),
                                                ])
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("row")
                                    .children([
                                        html!("div", {
                                            .class("col-sm-7")
                                            .child(html!("div", {
                                                .class("mb-3")
                                                .children([
                                                    html!("label", {
                                                        .attr("for", "consult_date")
                                                        .text("วันที่ เวลา")
                                                    }),
                                                    html!("div", {
                                                        .class(class::INPUT_GROUP)
                                                        .children([
                                                            doms::date_picker(
                                                                modal.consult_date.clone(),
                                                                modal.changed.clone(), always(!matches!(modal.consult_mode, ConsultFormMode::Edit)), None,
                                                                |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                                |d| d.class("rounded-end-0"),
                                                                |d| d.class("rounded-end-0").attr("id","consult_date"),
                                                                |s| s, always(None),
                                                            ),
                                                            doms::time_picker(
                                                                modal.consult_time.clone(),
                                                                modal.changed.clone(), always(!matches!(modal.consult_mode, ConsultFormMode::Edit)), None,
                                                                |d| d.class(class::FLEX_GROW1).style("min-width","110px"),
                                                                |d| d.class("rounded-0"),
                                                                |d| d.class("rounded-0"),
                                                                |s| s, always(None),
                                                            ),
                                                            html!("button", {
                                                                .attr("type", "button")
                                                                .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                                .class(class::BTN_CYAN)
                                                                //.attr("id", "btn_consult_time_now")
                                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                                .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| dom.attr("disabled",""))
                                                                .event(clone!(modal => move |_: events::Click| {
                                                                    let now = js_now();
                                                                    modal.consult_date.set_neq(now.date().to_string());
                                                                    modal.consult_time.set_neq(now.time().js_string());
                                                                    modal.changed.set_neq(true);
                                                                    // .attr("onclick", "onclickConsultAddDateTimeNow()")
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                ])
                                            }))
                                        }),
                                        html!("div", {
                                            .class("col-sm-5")
                                            .child(html!("div", {
                                                .class("mb-3")
                                                .children([
                                                    html!("label", {
                                                        .attr("for", "consult_emergency")
                                                        .text("Emergency")
                                                    }),
                                                    html!("select" => HtmlSelectElement, {
                                                        .class("form-select")
                                                        .attr("id", "consult_emergency")
                                                        .child(html!("option", {
                                                            .attr("value", "")
                                                            .text("เลือก")
                                                        }))
                                                        .children(emergency_select_option.iter().map(|option| {
                                                            doms::select_option(option, "")
                                                        }))
                                                        .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| dom.attr("disabled",""))
                                                        .apply(mixins::string_value_select(modal.consult_emergency.clone(), modal.changed.clone()))
                                                    }),
                                                ])
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::CARD)
                                    .children([
                                        html!("div", {
                                            .class(class::CARD_HEAD)
                                            .child(html!("i", {.class(class::FA_MEDICINE)}))
                                            .text(" ต้องการ Consult")
                                        }),
                                        html!("div", {
                                            .class("card-body")
                                            .children([
                                                html!("div", {
                                                    .class("row")
                                                    .children([
                                                        html!("div", {
                                                            .class("col-sm-5")
                                                            .child(html!("div", {
                                                                .children([
                                                                    html!("label", {
                                                                        .attr("for", "consult_spclty")
                                                                        .text("แผนกที่ Consult")
                                                                    }),
                                                                    html!("select" => HtmlSelectElement, {
                                                                        .class(class::FORM_CTRL_T)
                                                                        .attr("id", "consult_spclty")
                                                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                                        .child(html!("option", {.attr("value", "0").text("ฝ่ายเภสัชกรรม")}))
                                                                        .children(spclty_kphis_select_option.iter().map(|option| {
                                                                            doms::select_option(option, "")
                                                                        }))
                                                                        .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| dom.attr("disabled",""))
                                                                        .apply(mixins::string_value_select(modal.consult_spclty.clone(), modal.changed.clone()))
                                                                    }),
                                                                ])
                                                            }))
                                                        }),
                                                        html!("div", {
                                                            .class("col-sm-7")
                                                            .child(html!("div", {
                                                                .children([
                                                                    html!("label", {
                                                                        .attr("for", "consult_doctorcode_mention")
                                                                        .text("แพทย์ Staff (Optional)")
                                                                    }),
                                                                    html!("select" => HtmlSelectElement, {
                                                                        .class(class::FORM_CTRL_T)
                                                                        .attr("id", "consult_doctorcode_mention")
                                                                        .child(html!("option", {
                                                                            .attr("value", "")
                                                                            .text("เลือก")
                                                                        }))
                                                                        .children(doctor_select_option.iter().map(|option| {
                                                                            doms::select_option(option, "")
                                                                        }))
                                                                        .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| dom.attr("disabled",""))
                                                                        .apply(mixins::string_value_select(modal.consult_doctorcode_mention.clone(), modal.changed.clone()))
                                                                    }),
                                                                ])
                                                            }))
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("row")
                                                    .child(html!("div", {
                                                        .class("col-sm-12")
                                                        .children([
                                                            html!("label", {
                                                                .attr("for", "consult_data")
                                                                .children([
                                                                    // text("PURPOSE OF CONSULTATION"),html!("br"),
                                                                    text("HISTORY, PHYSICAL EXAMINATION AND LAB FINDING"),
                                                                    html!("br"),
                                                                ])
                                                            }),
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class("form-control")
                                                                .attr("id", "consult_data")
                                                                .attr("rows", "6")
                                                                .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| dom.attr("disabled",""))
                                                                .apply(mixins::textarea_value_auto_expand(modal.consult_data.clone(), modal.changed.clone()))
                                                            }),
                                                        ])
                                                        .apply(clone!(app, modal => move |dom| {
                                                            if modal.consult_id.get() == 0 && matches!(modal.consult_mode, ConsultFormMode::Edit) {
                                                                if app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, modal.is_pre_admit) {
                                                                    dom.child(html!("div", {
                                                                        .class("my-1")
                                                                        // POST `EndPoint::ImageUsage`
                                                                        .child(ImageCpn::render("170px", ImageCpn::new_returning(
                                                                            modal.data_image_callback.clone(),
                                                                            modal.patient.clone(),
                                                                            str_some(modal.an.get_cloned()),
                                                                            "CONSULT-DATA",
                                                                        ), app.clone()))
                                                                    }))
                                                                } else {
                                                                    dom
                                                                }
                                                            } else {
                                                                dom.child(html!("div", {
                                                                    .class("my-1")
                                                                    .child_signal(modal.is_not_requested_user(app.clone()).map(clone!(app, modal => move |not_requester| {
                                                                        Some(ImageCpn::render("170px", ImageCpn::new_with_key(
                                                                            ImageUsage::IpdConsultData,
                                                                            modal.consult_id.get(),
                                                                            !not_requester,
                                                                            modal.patient.clone(),
                                                                            str_some(modal.an.get_cloned()),
                                                                            "", // will use ImageUsage internally, so we add nothing here
                                                                        ), app.clone()))
                                                                    })))
                                                                }))
                                                            }
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("row")
                                                    .child(html!("div", {
                                                        .class("col-md-12")
                                                        .child(html!("div", {
                                                            // .class("mb-3")
                                                            .child(html!("label", {
                                                                //.attr("for", "action-person-dr-consult")
                                                                .text("แพทย์ผู้ Consult")
                                                            }))
                                                            .children_signal_vec(modal.consult_requests.signal_vec_cloned().map(clone!(app, modal, all_doctor_select_option => move |doctor| {
                                                                html!("div", {
                                                                    //.attr("id", "dr-consult-group-input-div")
                                                                    .class("row")
                                                                    .child(html!("div", {
                                                                        .class(class::INPUT_GROUP_T)
                                                                        .children([
                                                                            html!("div", {
                                                                                .class("col-md-6")
                                                                                .child(html!("input", {
                                                                                    .attr("type", "text")
                                                                                    .class("form-control")
                                                                                    .attr("readonly", "")
                                                                                    .prop_signal("value", doctor.doctorname.signal_cloned())
                                                                                }))
                                                                            }),
                                                                            html!("div", {
                                                                                .class("col-md-6")
                                                                                .child(html!("select" => HtmlSelectElement, {
                                                                                    .class("form-select")
                                                                                    .child(html!("option", {
                                                                                        .attr("value", "")
                                                                                        .text("เลือก")
                                                                                    }))
                                                                                    .children(all_doctor_select_option.iter().map(|option| {
                                                                                        doms::select_option(option, "")
                                                                                    }))
                                                                                    .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit)
                                                                                        || doctor.doctorcode.lock_ref().as_str() != &app.doctor_code().unwrap_or_default(),
                                                                                        |dom| dom.attr("disabled",""))
                                                                                    .apply(mixins::string_value_select(doctor.doctorcode2.clone(), modal.changed.clone()))
                                                                                }))
                                                                            }),
                                                                        ])
                                                                    }))
                                                                })
                                                            })))
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class(class::ROW)
                                                    .child(html!("div", {
                                                        .class("col-sm")
                                                        .child(doms::badge_info_center("หากมีแพทย์ตอบ Consult แล้ว จะไม่สามารถแก้ไขข้อมูลได้"))
                                                    }))
                                                }),
                                            ])
                                        }),
                                    ])
                                }),
                            ])
                            .apply_if(!is_new && !matches!(modal.consult_mode, ConsultFormMode::Edit), clone!(app, modal => move |dom| {
                                dom.child(html!("div", {
                                    .apply_if(matches!(modal.consult_mode, ConsultFormMode::Reply), |dom| {
                                        dom.visible_signal(modal.is_not_requested_user(app.clone()))
                                    })
                                    .class("card")
                                    //.attr("id", "card_dr_consult_reply")
                                    .children([
                                        html!("div", {
                                            .class(class::CARD_HEAD)
                                            .child(html!("i", {.class(class::FA_NOTE_MED)}))
                                            .text(" CONSULTATION REPORT")
                                        }),
                                        html!("div", {
                                            .class(class::CARD_BODY_LIGHTS)
                                            .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Edit), |dom| {
                                                dom.child(html!("div", {
                                                    .class(class::ROW)
                                                    .children([
                                                        html!("label", {
                                                            .class(class::FORM_COL_LBL_AUTO)
                                                            .attr("for", "consult_datetime_create_reply")
                                                            .text("วันที่ตอบ")
                                                        }),
                                                        html!("div", {
                                                            .class("col-sm-4")
                                                            .child(html!("input" => HtmlInputElement, {
                                                                .attr("type", "text")
                                                                .class("form-control")
                                                                .attr("id", "consult_datetime_create_reply")
                                                                //.attr("readonly", "")
                                                                .attr("disabled", "")
                                                                .prop_signal("value", modal.consult_datetime_create_reply.signal_cloned().map(|dt| datetime_str_th(&dt)))
                                                                //.apply_if(!matches!(modal.consult_mode, ConsultFormMode::Reply), |dom| dom.attr("disabled",""))
                                                                //.apply(mixins::string_value(modal.consult_datetime_create_reply.clone(), modal.changed.clone()))
                                                            }))
                                                        }),
                                                        html!("label", {
                                                            .class(class::FORM_COL_LBL_AUTO)
                                                            .attr("for", "consult_datetime_update_reply")
                                                            .text("วันที่ตอบล่าสุด")
                                                        }),
                                                        html!("div", {
                                                            .class("col-sm-4")
                                                            .child(html!("input" => HtmlInputElement, {
                                                                .attr("type", "text")
                                                                .class("form-control")
                                                                .attr("id", "consult_datetime_update_reply")
                                                                //.attr("readonly", "")
                                                                .attr("disabled", "")
                                                                .prop_signal("value", modal.consult_datetime_update_reply.signal_cloned().map(|dt| datetime_str_th(&dt)))
                                                                // .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Reply), |dom| dom.attr("disabled",""))
                                                                // .apply(mixins::string_value(modal.consult_datetime_update_reply.clone(), modal.changed.clone()))
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                            .children([
                                                html!("div", {
                                                    .class("row")
                                                    .child(html!("div", {
                                                        .class("col-sm-12")
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .attr("for", "consult_finding")
                                                                    .text("FINDING")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class("form-control")
                                                                    .attr("id", "consult_finding")
                                                                    .attr("rows", "3")
                                                                    .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Reply), |dom| dom.attr("disabled",""))
                                                                    .apply(mixins::textarea_value_auto_expand(modal.consult_finding.clone(), modal.changed.clone()))
                                                                }),
                                                            ])
                                                            .child(html!("div", {
                                                                .class("my-1")
                                                                .child_signal(modal.is_not_replied_user(app.clone()).map(clone!(app, modal => move |not_replier| {
                                                                    let is_editable = matches!(modal.consult_mode, ConsultFormMode::Reply) && !not_replier;
                                                                    Some(ImageCpn::render("170px", ImageCpn::new_with_key(
                                                                        ImageUsage::IpdConsultFinding,
                                                                        modal.consult_id.get(),
                                                                        is_editable,
                                                                        modal.patient.clone(),
                                                                        str_some(modal.an.get_cloned()),
                                                                        "", // will use ImageUsage internally, so we add nothing here
                                                                    ), app.clone()))
                                                                })))
                                                            }))
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("row")
                                                    .child(html!("div", {
                                                        .class("col-sm-12")
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .attr("for", "consult_diagnosis")
                                                                    .text("DIAGNOSIS")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class("form-control")
                                                                    .attr("id", "consult_diagnosis")
                                                                    .attr("rows", "3")
                                                                    .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Reply), |dom| dom.attr("disabled",""))
                                                                    .apply(mixins::textarea_value_auto_expand(modal.consult_diagnosis.clone(), modal.changed.clone()))
                                                                }),
                                                            ])
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("row")
                                                    .child(html!("div", {
                                                        .class("col-sm-12")
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .attr("for", "consult_recommendation")
                                                                    .text("RECOMMENDATION")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class("form-control")
                                                                    .attr("id", "consult_recommendation")
                                                                    .attr("rows", "3")
                                                                    .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Reply), |dom| dom.attr("disabled",""))
                                                                    .apply(mixins::textarea_value_auto_expand(modal.consult_recommendation.clone(), modal.changed.clone()))
                                                                }),
                                                            ])
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("row")
                                                    .child(html!("div", {
                                                        .class("col-md-12")
                                                        .child(html!("div", {
                                                            // .class("mb-3")
                                                            .child(html!("label", {
                                                                //.attr("for", "action-person-dr-consult")
                                                                .text("ลงชื่อแพทย์")
                                                            }))
                                                            .children_signal_vec(modal.consult_replies.signal_vec_cloned().map(clone!(app, modal, all_doctor_select_option => move |doctor| {
                                                                html!("div", {
                                                                    //.attr("id", "dr-consult-reply-group-input-div")
                                                                    .class(class::ROW)
                                                                    .child(html!("div", {
                                                                        .class(class::INPUT_GROUP_T)
                                                                        .children([
                                                                            html!("div", {
                                                                                .class("col-md-6")
                                                                                .child(html!("input", {
                                                                                    .attr("type", "text")
                                                                                    .class("form-control")
                                                                                    .attr("readonly", "")
                                                                                    .prop_signal("value", doctor.doctorname.signal_cloned())
                                                                                }))
                                                                            }),
                                                                            html!("div", {
                                                                                .class("col-md-6")
                                                                                .child(html!("select" => HtmlSelectElement, {
                                                                                    .class("form-select")
                                                                                    .child(html!("option", {
                                                                                        .attr("value", "")
                                                                                        .text("เลือก")
                                                                                    }))
                                                                                    .children(all_doctor_select_option.iter().map(|option| {
                                                                                        doms::select_option(option, "")
                                                                                    }))
                                                                                    .apply_if(!matches!(modal.consult_mode, ConsultFormMode::Reply)
                                                                                        || doctor.doctorcode.lock_ref().as_str() != &app.doctor_code().unwrap_or_default(),
                                                                                        |dom| dom.attr("disabled",""))
                                                                                    .apply(mixins::string_value_select(doctor.doctorcode2.clone(), modal.changed.clone()))
                                                                                }))
                                                                            }),
                                                                        ])
                                                                    }))
                                                                })
                                                            })))
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
                                            ])
                                        }),
                                    ])
                                }))
                            }))
                        }))
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .apply_if(
                            !is_new
                            && matches!(modal.consult_mode, ConsultFormMode::Edit)
                            && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdConsult, modal.is_pre_admit),
                        |dom| {
                            dom.child(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_LX_RED)
                                .attr("data-bs-dismiss", "modal")
                                .child(html!("i", {.class(class::FA_TRASH)}))
                                .text(" Delete")
                                .apply(mixins::click_with_loader_checked(clone!(app, modal, display, changed => move || {
                                    Self::delete(modal.clone(), display.clone(), changed.clone(), app.clone());
                                }), app.state()))
                            }))
                        })
                        .apply_if(!matches!(modal.consult_mode, ConsultFormMode::View)
                            && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdConsult, modal.is_pre_admit),
                        |dom| { dom
                            .child_signal(modal.consult_id.signal_cloned().map(clone!(app, modal => move |or_zero| {
                                (if or_zero == 0 {
                                    app.has_permission(Permission::IpdDoctorConsultAdd)
                                } else {
                                    app.has_permission(Permission::IpdDoctorConsultEdit)
                                }).then(|| {
                                    html!("button" => HtmlButtonElement, {
                                        .apply_if(matches!(modal.consult_mode, ConsultFormMode::Reply), |dom| { dom
                                            .visible_signal(map_ref!{
                                                let not_req = modal.is_not_requested_user(app.clone()),
                                                let over_24hr = modal.is_reply_over_24hr() =>
                                                *not_req && !over_24hr
                                            })
                                        })
                                        .attr("type", "button")
                                        .class(class::BTN_BLUE)
                                        .attr("data-bs-dismiss", "modal")
                                        .child(html!("i", {.class(class::FA_SAVE)}))
                                        .text(" Save")
                                        .apply(mixins::click_with_loader_checked(clone!(app, modal, display, changed => move || {
                                            Self::save(modal.clone(), display.clone(), changed.clone(), app.clone());
                                        }), app.state()))
                                    })
                                })
                            })))}
                        )
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .child(html!("i", {.class(class::FA_X)}))
                            .text(" Cancel")
                        }))
                    }),
                ])
            }))
        })
    }
}

#[derive(Clone, Default)]
pub enum ConsultFormMode {
    Edit,
    Reply,
    #[default]
    View,
}

impl ConsultFormMode {
    fn str(&self) -> &'static str {
        match self {
            Self::Edit => "edit",
            Self::Reply => "reply",
            Self::View => "view",
        }
    }
}

struct Doctors {
    doctorcode: Mutable<String>,
    doctorcode2: Mutable<String>,
    doctorname: Mutable<String>,
}

impl Doctors {
    fn new(cap_pipe: &str) -> impl Iterator<Item = Rc<Self>> {
        cap_pipe.split('|').flat_map(|row| {
            let cols = row.split('^').collect::<Vec<&str>>();
            if cols.len() == 3 {
                Some(Rc::new(Self {
                    doctorcode: Mutable::new(cols[0].to_owned()),
                    doctorname: Mutable::new(cols[1].to_owned()),
                    doctorcode2: Mutable::new(cols[2].to_owned()),
                }))
            } else {
                None
            }
        })
    }
}

fn send_sse_by_save(save: &ConsultSave, patient: Option<Rc<PatientInfo>>, app: Rc<App>) {
    let ward_opt = patient.as_ref().and_then(|pt| pt.ward.clone());
    let an_opt = patient.as_ref().and_then(|pt| pt.an.clone());
    let ward_name = patient.as_ref().and_then(|pt| pt.ward_name.as_ref().map(|ward| [ward, " "].concat())).unwrap_or_default();
    let bed = patient.as_ref().and_then(|pt| pt.bedno.as_ref().map(|bedno| ["เตียง ", bedno, " "].concat())).unwrap_or_default();
    let hn = patient.as_ref().and_then(|pt| pt.hn.as_ref().map(|hn| ["HN ", hn, " "].concat())).unwrap_or_default();

    if let (Some(an), Some(ward)) = (an_opt, ward_opt) {
        let mut messages = Vec::new();
        // edit or reply
        match save.consult_mode.as_str() {
            "edit" => messages.push(SsePostMessage {
                message: [&ward_name, &bed, &hn, "มีคำขอรับการปรึกษา"].concat(),
                person: save.consult_doctorcode_mention.clone(),
                spclty_id: save.consult_spclty,
                route: Some(Route::IpdMain {
                    view_by: String::from("doctor"),
                    an: an.clone(),
                    tab: String::from("consult"),
                    sub: String::from("reply"),
                    id: save.consult_id.unwrap_or_default(),
                }),
                ..Default::default()
            }),
            "reply" => {
                let mut consultants = Vec::with_capacity(save.consult_doctorcode_requests.len() * 2);
                save.consult_doctorcode_requests.iter().for_each(|drs| {
                    if let Some(doctor1) = &drs.person1 {
                        consultants.push(doctor1.clone());
                    }
                    if let Some(doctor2) = &drs.person2 {
                        consultants.push(doctor2.clone());
                    }
                });
                messages.push(SsePostMessage {
                    message: [&ward_name, &bed, &hn, "มีการตอบคำปรึกษา"].concat(),
                    ward: Some(ward),
                    route: Some(Route::IpdMain {
                        view_by: String::from("nurse"),
                        an: an.clone(),
                        tab: String::from("consult"),
                        sub: String::from("view"),
                        id: save.consult_id.unwrap_or_default(),
                    }),
                    ..Default::default()
                });
                consultants.iter().for_each(|dr| {
                    messages.push(SsePostMessage {
                        message: [&ward_name, &bed, &hn, "มีการตอบคำปรึกษา"].concat(),
                        person: Some(dr.clone()),
                        route: Some(Route::IpdMain {
                            view_by: String::from("doctor"),
                            an: an.clone(),
                            tab: String::from("consult"),
                            sub: String::from("view"),
                            id: save.consult_id.unwrap_or_default(),
                        }),
                        ..Default::default()
                    })
                });
            }
            _ => {}
        };
        for message in messages.into_iter() {
            app.send_sse(message);
        }
    }
}
