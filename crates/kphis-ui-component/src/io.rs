use dominator::{Dom, clone, events, html, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use rust_decimal::Decimal;
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    SCREEN_WIDTH_EXTRA,
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    ipd::io::{IoDate, IoParams, IoShift},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    shift::{NurseShift, Shift},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, date_th, js_now, time_8601, time_hm},
    util::{decimal_rescale, str_some, thousands, zero_none},
};

use crate::gadget::pdf_button::PdfButtons;

/// - GET `EndPoint::IpdIoDateAn`
/// - GET `EndPoint::OpdErIoDateId`
/// - GET `EndPoint::IpdIo`
/// - GET `EndPoint::OpdErIo`
/// - POST `EndPoint::IpdIo` (guarded, remove 'บันทึก' btn)
/// - POST `EndPoint::OpdErIo` (guarded, remove 'บันทึก' btn)
/// - DELETE `EndPoint::IpdIo` (guarded, remove 'ลบ' btn)
/// - DELETE `EndPoint::OpdErIo` (guarded, remove 'ลบ' btn)
#[derive(Clone, Default)]
pub struct IoCpn {
    patient: Mutable<Option<Rc<PatientInfo>>>,

    // print
    print_changed: Mutable<bool>,
    io_date_start: Mutable<String>,
    io_date_end: Mutable<String>,

    form_changed: Mutable<bool>,
    checked: Mutable<bool>,

    io_full_parenteral: Mutable<bool>,
    io_full_oral: Mutable<bool>,
    io_full_output: Mutable<bool>,

    io_id: Mutable<u32>,
    io_date: Mutable<String>,
    io_time: Mutable<String>,
    io_parenteral_type: Mutable<String>,
    io_parenteral_name: Mutable<String>,
    io_parenteral_amount: Mutable<String>,
    io_parenteral_absorb: Mutable<String>,
    io_parenteral_carry_forward: Mutable<String>,
    io_parenteral_remark: Mutable<String>,
    io_oral_name: Mutable<String>,
    io_oral_amount: Mutable<String>,
    io_oral_absorb: Mutable<String>,
    io_oral_carry_forward: Mutable<String>,
    io_oral_remark: Mutable<String>,
    io_output_type: Mutable<String>,
    io_output_amount: Mutable<String>,
    io_output_remark: Mutable<String>,
    version: Mutable<i32>,
    user_name: Mutable<String>,
    entryposition: Mutable<String>,
    // shift: Mutable<NurseShift>,

    // table
    loaded_select_date: Mutable<bool>,
    io_dates: MutableVec<Rc<IoDate>>,
    current_date: Mutable<Option<IoDate>>,

    loaded_shift: Mutable<bool>,
    // all loaded shifts
    io_shifts: MutableVec<Rc<IoShift>>,
    // only shift of current_date
    io_night_shifts: MutableVec<Rc<IoShift>>,
    io_day_shifts: MutableVec<Rc<IoShift>>,
    io_evening_shifts: MutableVec<Rc<IoShift>>,
}

impl IoCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        let now = js_now();
        let date_now = now.date().to_string();
        Rc::new(Self {
            patient,
            io_date_start: Mutable::new(date_now.clone()),
            io_date_end: Mutable::new(date_now.clone()),
            io_date: Mutable::new(date_now),
            io_time: Mutable::new(now.time().js_string()),
            loaded_shift: Mutable::new(true),
            ..Default::default()
        })
    }

    fn renew_form(&self) {
        let now = js_now();
        self.io_id.set_neq(0);
        self.io_date.set_neq(now.date().to_string());
        self.io_time.set_neq(now.time().js_string());
        self.io_parenteral_type.set_neq(String::new());
        self.io_parenteral_name.set_neq(String::new());
        self.io_parenteral_amount.set_neq(String::new());
        self.io_parenteral_absorb.set_neq(String::new());
        self.io_parenteral_carry_forward.set_neq(String::new());
        self.io_parenteral_remark.set_neq(String::new());
        self.io_oral_name.set_neq(String::new());
        self.io_oral_amount.set_neq(String::new());
        self.io_oral_absorb.set_neq(String::new());
        self.io_oral_carry_forward.set_neq(String::new());
        self.io_oral_remark.set_neq(String::new());
        self.io_output_type.set_neq(String::new());
        self.io_output_amount.set_neq(String::new());
        self.io_output_remark.set_neq(String::new());
        self.form_changed.set_neq(false);
        self.version.set_neq(0);
        self.user_name.set_neq(String::new());
        self.entryposition.set_neq(String::new());
        // self.shift.set_neq(String::new());
    }

    fn is_ipd(&self) -> impl Signal<Item = Option<bool>> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.is_ipd()))
    }

    fn is_admited(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.is_admited()).unwrap_or_default())
    }

    fn current_is_io_date(&self, io_date: Rc<IoDate>) -> impl Signal<Item = bool> + use<> {
        self.current_date.signal_cloned().map(move |opt| opt.as_ref().map(|cd| *cd == *io_date).unwrap_or_default())
    }

    // ipd-vital-sign-io-select-date.php
    fn load_select_date(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            app.async_load(
                true,
                clone!(app => async move {
                    let result_opt = match patient.visit_type() {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            // GET `EndPoint::IpdIoDateAn`
                            Some(IoDate::call_api_get_ipd(&an, app.state()).await)
                        }
                        VisitTypeId::OpdEr(_vn, opd_er_order_master_id) => {
                            // GET `EndPoint::OpdErIoDateId`
                            Some(IoDate::call_api_get_opd_er(opd_er_order_master_id, app.state()).await)
                        }
                        VisitTypeId::Visit(_vn) => None,
                    };
                    if let Some(result) = result_opt {
                        match result {
                            Ok(response) => {
                                let mut dates = Vec::new();
                                let now = js_now().date();
                                let today = IoDate { io_date: now, is_today: true };
                                if let Some(dchdate) = patient.lastdate() {
                                    if now <= dchdate {
                                        dates.push(today);
                                    }
                                } else if !response.iter().any(|iod| iod.io_date == now) {
                                    dates.push(today);
                                }
                                dates.extend(response);
                                if !page.current_date.lock_ref().as_ref().map(|current_date| dates.iter().any(|iod| iod.io_date == current_date.io_date)).unwrap_or_default() {
                                    page.current_date.set_neq(dates.first().cloned());
                                }
                                let io_dates = dates.into_iter().map(Rc::new);
                                {
                                    let mut lock = page.io_dates.lock_mut();
                                    if !lock.is_empty() {
                                        lock.clear();
                                    }
                                    lock.extend(io_dates);
                                }
                                // page.order_dates_exact.set(order_dates.collect());
                                page.loaded_shift.set_neq(false);
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

    // ipd-vital-sign-io-table.php
    fn load_io_table(page: Rc<Self>, app: Rc<App>) {
        if let (Some(start), Some(end)) = (date_8601(&page.io_date_start.lock_ref()), date_8601(&page.io_date_end.lock_ref())) {
            if end >= start {
                page.io_shifts.lock_mut().clear();
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
                            Some(VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an)) => {
                                let params = IoParams {
                                    an: str_some(an),
                                    start_date: Some(start),
                                    end_date: Some(end),
                                    ..Default::default()
                                };
                                // GET `EndPoint::IpdIo`
                                match IoShift::call_api_get_ipd(&params, app.state()).await {
                                    Ok(responses) => {
                                        page.checked.set_neq(!responses.is_empty());
                                        Self::renew_shifts(&responses, page.clone());
                                        page.io_shifts.lock_mut().extend(responses.into_iter().filter(|res| res.shift_date.map(|d| d >= start && d <= end).unwrap_or_default()).map(Rc::new));
                                    }
                                    Err(e) => {
                                        app.alert_app_error(&e).await;
                                    }
                                }
                            }
                            Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) => {
                                let params = IoParams {
                                    opd_er_order_master_id: zero_none(opd_er_order_master_id),
                                    start_date: Some(start),
                                    end_date: Some(end),
                                    ..Default::default()
                                };
                                // GET `EndPoint::OpdErIo`
                                match IoShift::call_api_get_opd_er(&params, app.state()).await {
                                    Ok(responses) => {
                                        page.checked.set_neq(!responses.is_empty());
                                        Self::renew_shifts(&responses, page.clone());
                                        page.io_shifts.lock_mut().extend(responses.into_iter().filter(|res| res.shift_date.map(|d| d >= start && d <= end).unwrap_or_default()).map(Rc::new));
                                    }
                                    Err(e) => {
                                        app.alert_app_error(&e).await;
                                    }
                                }
                            }
                            Some(VisitTypeId::Visit(_))
                            | None => {}
                        }
                    }),
                );
            }
        }
    }

    // ipd-vital-sign-io-save.php
    // ipd-vital-sign-io-update.php
    fn add_or_edit(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            let (an, opd_er_order_master_id, valid) = match patient.visit_type() {
                VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (Some(an), None, true),
                VisitTypeId::OpdEr(_, opd_er_order_master_id) => (None, Some(opd_er_order_master_id), true),
                VisitTypeId::Visit(_) => (None, None, false),
            };
            if valid {
                let now = js_now();
                let io_date = date_8601(&page.io_date.lock_ref()).unwrap_or(now.date());
                let io_time = time_8601(&page.io_time.lock_ref()).unwrap_or(now.time());
                let save = IoShift {
                    io_id: page.io_id.get(),
                    io_date,
                    io_time,
                    io_parenteral_type: str_some(page.io_parenteral_type.get_cloned()),
                    io_parenteral_name: str_some(page.io_parenteral_name.get_cloned()),
                    io_parenteral_amount: Decimal::from_str_exact(&page.io_parenteral_amount.lock_ref()).ok(),
                    io_parenteral_absorb: Decimal::from_str_exact(&page.io_parenteral_absorb.lock_ref()).ok(),
                    io_parenteral_carry_forward: Decimal::from_str_exact(&page.io_parenteral_carry_forward.lock_ref()).ok(),
                    io_parenteral_remark: str_some(page.io_parenteral_remark.get_cloned()),
                    io_oral_name: str_some(page.io_oral_name.get_cloned()),
                    io_oral_amount: page.io_oral_amount.lock_ref().parse::<i32>().ok(),
                    io_oral_absorb: page.io_oral_absorb.lock_ref().parse::<i32>().ok(),
                    io_oral_carry_forward: page.io_oral_carry_forward.lock_ref().parse::<i32>().ok(),
                    io_oral_remark: str_some(page.io_oral_remark.get_cloned()),
                    io_output_type: str_some(page.io_output_type.get_cloned()),
                    io_output_amount: page.io_output_amount.lock_ref().parse::<i32>().ok(),
                    io_output_remark: str_some(page.io_output_remark.get_cloned()),
                    version: page.version.get(),
                    user_name: str_some(page.user_name.get_cloned()).or(app.doctor_name()),
                    entryposition: str_some(page.entryposition.get_cloned()).or(app.doctor_entryposition()),
                    shift_date: None,
                    shift: None,
                    an,
                    opd_er_order_master_id,
                };
                app.async_load(
                    true,
                    clone!(app => async move {
                        // POST `EndPoint::IpdIo`
                        // POST `EndPoint::OpdErIo`
                        match save.call_api_post(app.state()).await {
                            Ok((_id, responses)) => {
                                app.alert_execute_responses(&responses, async move {
                                    // app.alert("บันทึกข้อมูลสำเร็จ");
                                    page.current_date.set_neq(Some(IoDate {
                                        io_date: save.io_date,
                                        is_today: js_now().date() == save.io_date,
                                    }));
                                    page.loaded_select_date.set_neq(false);
                                    page.renew_form();
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
    }

    // ipd-vital-sign-io-delete.php
    // DELETE FROM kphis.ipd_io WHERE io_id=? AND version=?;
    /// io_id, version
    fn delete(page: Rc<Self>, app: Rc<App>) {
        if let (Some(io_id), Some(version)) = (zero_none(page.io_id.get()), zero_none(page.version.get())) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    if app.confirm("ยืนยันลบรายการ").await {
                        let params = IoParams {
                            io_id: Some(io_id),
                            version: Some(version),
                            ..Default::default()
                        };
                        let result_opt = match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
                            Some(VisitTypeId::Ipd(_) | VisitTypeId::PreAdmit(_)) => {
                                // DELETE `EndPoint::IpdIo`
                                Some(IoShift::call_api_delete(true, &params, app.state()).await)
                            }
                            Some(VisitTypeId::OpdEr(_, _)) => {
                                // DELETE `EndPoint::OpdErIo`
                                Some(IoShift::call_api_delete(false, &params, app.state()).await)
                            }
                            Some(VisitTypeId::Visit(_))
                            | None => None,
                        };
                        if let Some(result) = result_opt {
                            match result {
                                Ok(responses) => {
                                    app.alert_execute_responses(&responses, async move {
                                        page.loaded_select_date.set_neq(false);
                                        page.renew_form();
                                    }).await;
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                    }
                }),
            );
        }
    }

    fn renew_shifts(shifts: &[IoShift], page: Rc<Self>) {
        let date = page.current_date.lock_ref().as_ref().map(|io| io.io_date);
        {
            let mut night_lock = page.io_night_shifts.lock_mut();
            night_lock.clear();
            night_lock.extend(shifts.iter().filter(|s| s.shift_date == date && matches!(s.shift, Some(NurseShift::Night))).cloned().map(Rc::new));
        }
        {
            let mut day_lock = page.io_day_shifts.lock_mut();
            day_lock.clear();
            day_lock.extend(shifts.iter().filter(|s| s.shift_date == date && matches!(s.shift, Some(NurseShift::Day))).cloned().map(Rc::new));
        }
        {
            let mut evening_lock = page.io_evening_shifts.lock_mut();
            evening_lock.clear();
            evening_lock.extend(shifts.iter().filter(|s| s.shift_date == date && matches!(s.shift, Some(NurseShift::Evening))).cloned().map(Rc::new));
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_select_date.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_select_date(page.clone(), app.clone());
                    page.loaded_select_date.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let print_changed = page.print_changed.signal(),
                let loaded = page.loaded_shift.signal() =>
                !busy && (!loaded || *print_changed)
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_io_table(page.clone(), app.clone());
                    page.loaded_shift.set_neq(true);
                    page.print_changed.set_neq(false);
                }
                async {}
            })))
            .future(map_ref! {
                let without_aside = app.aside_prev_percent.signal().map(|percent| percent == 100.0),
                let extra_width = window_size().map(|ws| ws.width > SCREEN_WIDTH_EXTRA) =>
                *without_aside && *extra_width
            }.dedupe().for_each(clone!(page => move |can_full| {
                page.io_full_parenteral.set_neq(can_full);
                page.io_full_oral.set_neq(can_full);
                page.io_full_output.set_neq(can_full);
                async {}
            })))
            .child(html!("div", {
                .class("row")
                .child(html!("div", {
                    .class(class::COLA_PY_RX)
                    .child(html!("div", {
                        .class(class::ALERT_GRAY)
                        .class("m-0")
                        .attr("role", "alert")
                        .child(html!("div", {
                            .class(class::FLEX_WRAP)
                            .child(html!("div", {
                                .class(class::COLA_P)
                                .child(html!("div", {
                                    .class(class::INPUT_GROUP)
                                    .children([
                                        doms::span_group_text("PDF วันที่"),
                                        doms::date_picker(
                                            page.io_date_start.clone(),
                                            page.print_changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","135px").style("width","135px"),
                                            |d| d.class("rounded-0"),
                                            |d| d.class("rounded-0"),
                                            |s| s, always(None),
                                        ),
                                        doms::span_group_text("ถึง"),
                                        doms::date_picker(
                                            page.io_date_end.clone(),
                                            page.print_changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                            |d| d.class("rounded-start-0"),
                                            |d| d.class("rounded-start-0"),
                                            |s| s, always(None),
                                        ),
                                    ])
                                }))
                            }))
                            .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                                opt.map(|patient| {
                                    match patient.visit_type() {
                                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                            html!("div",{
                                                .class(class::PY_RX)
                                                .children(PdfButtons::buttons(
                                                    PdfButtons::new(
                                                        TypstReport::from_system_with_coercion(SystemReport::IpdIo, &app.state().report_coercions()),
                                                        Mutable::new(an.clone()),
                                                        page.checked.clone(),
                                                        page.form_changed.clone(),
                                                        clone!(page => move || {serde_json::json!({
                                                            "id": an,
                                                            "patient": patient,
                                                            "io": page.io_shifts.lock_ref().to_vec(),
                                                        }).to_string()})
                                                    ), "PDF", Some("PDF (All)"), app.clone()
                                                ))
                                            })
                                        }
                                        VisitTypeId::OpdEr(vn, _opd_er_order_master_id) => {
                                            html!("div",{
                                                .class(class::PY_RX)
                                                .children(PdfButtons::buttons(
                                                    PdfButtons::new(
                                                        TypstReport::from_system_with_coercion(SystemReport::OpdErIo, &app.state().report_coercions()),
                                                        Mutable::new(vn.clone()),
                                                        page.checked.clone(),
                                                        page.form_changed.clone(),
                                                        clone!(page => move || {serde_json::json!({
                                                            "id": vn,
                                                            "patient": patient,
                                                            "io": page.io_shifts.lock_ref().to_vec(),
                                                        }).to_string()})
                                                    ), "PDF", Some("PDF (All)"), app.clone()
                                                ))
                                            })
                                        }
                                        VisitTypeId::Visit(_) => Dom::empty(),
                                    }
                                })
                            })))
                        }))
                    }))
                }))
            }))
            .children([
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COLA_PY_L)
                        .child(html!("div", {
                            //.attr("id", "show_select_date_io")
                            .class(class::INPUT_GROUP)
                            .children([
                                html!("select" => HtmlSelectElement, {
                                    .class(class::FORM_SELECT_SM)
                                    .children_signal_vec(page.io_dates.signal_vec_cloned().map(|date| {
                                        html!("option", {
                                            .attr("value", &date.string())
                                            .text(&[date_th(&date.io_date), if date.is_today {String::from(" (วันนี้)")} else {String::new()}].concat())
                                            // .apply_if(page.current_date.lock_ref().as_ref().map(|iod| iod.io_date == date.io_date).unwrap_or_default(), |dom| {
                                            //     dom.attr("selected","")
                                            // })
                                        })
                                    }))
                                    .prop_signal("value", page.current_date.signal_cloned().map(|opt| opt.as_ref().map(|d| d.string())))
                                    .with_node!(element => {
                                        .event(clone!(page => move |_: events::Change| {
                                            let io_date = IoDate::from_string(&element.value());
                                            let io_date_string = io_date.as_ref().map(|d| d.io_date.to_string()).unwrap_or_default();
                                            page.current_date.set_neq(io_date);
                                            page.io_date_start.set_neq(io_date_string.clone());
                                            page.io_date_end.set_neq(io_date_string);
                                            page.loaded_shift.set_neq(false);
                                        }))
                                    })
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .child(html!("i",{.class(class::FA_L_CARET)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        if let Some(current_date) = page.current_date.get_cloned() {
                                            let lock = page.io_dates.lock_ref();
                                            let len = lock.len();
                                            if let Some(pos) = lock.iter().position(|date| **date == current_date) {
                                                if pos < len - 1 {
                                                    let io_date = Some((*lock[pos + 1]).clone());
                                                    let io_date_string = io_date.as_ref().map(|d| d.io_date.to_string()).unwrap_or_default();
                                                    page.current_date.set_neq(io_date);
                                                    page.io_date_start.set_neq(io_date_string.clone());
                                                    page.io_date_end.set_neq(io_date_string);
                                                    page.loaded_shift.set_neq(false);
                                                }
                                            }
                                        }
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .child(html!("i",{.class(class::FA_R_CARET)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        if let Some(current_date) = page.current_date.get_cloned() {
                                            let lock = page.io_dates.lock_ref();
                                            if let Some(pos) = lock.iter().position(|date| **date == current_date) {
                                                if pos > 0 {
                                                    let io_date = Some((*lock[pos - 1]).clone());
                                                    let io_date_string = io_date.as_ref().map(|d| d.io_date.to_string()).unwrap_or_default();
                                                    page.current_date.set_neq(io_date);
                                                    page.io_date_start.set_neq(io_date_string.clone());
                                                    page.io_date_end.set_neq(io_date_string);
                                                    page.loaded_shift.set_neq(false);
                                                }
                                            }
                                        }
                                    }))
                                }),
                            ])
                        }))
                    }))
                    .children_signal_vec(page.io_dates.signal_vec_cloned().enumerate().filter(|(i,_)| i.get().unwrap_or_default() < 7).map(clone!(page => move |(_,date)| {
                        html!("div", {
                            .class(class::COLA_P)
                            .child(html!("button", {
                                .attr("type", "button")
                                .class("btn")
                                .class_signal("btn-primary", page.current_is_io_date(date.clone()))
                                .class_signal("btn-secondary", not(page.current_is_io_date(date.clone())))
                                .text(&[date_th(&date.io_date), if date.is_today {String::from(" (วันนี้)")} else {String::new()}].concat())
                                .event(clone!(page => move |_: events::Click| {
                                    let io_date = Some(date.as_ref().to_owned());
                                    let io_date_string = io_date.as_ref().map(|d| d.io_date.to_string()).unwrap_or_default();
                                    page.current_date.set_neq(io_date);
                                    page.io_date_start.set_neq(io_date_string.clone());
                                    page.io_date_end.set_neq(io_date_string);
                                    page.loaded_shift.set_neq(false);
                                    page.renew_form();
                                }))
                            }))
                        })
                    })))
                }),
                html!("div", {
                    // .style("height","calc(100vh - 415px)")
                    // .style("overflow-y","auto")
                    .child(doms::table_responsive(class::TABLE, clone!(app, page => move |table| { table
                        .children([
                            html!("thead", {
                                .class("text-center")
                                .children([
                                    html!("tr", {
                                        .children([
                                            html!("th", {.attr("scope", "col").attr("rowspan", "2").text("#").style("vertical-align","middle")}),
                                            html!("th", {.attr("scope", "col").attr("rowspan", "2").text("วันที่").style("vertical-align","middle")}),
                                            html!("th", {.attr("scope", "col").attr("rowspan", "2").text("เวลา").style("vertical-align","middle")}),
                                            html!("th", {
                                                .class("table-info")
                                                .attr("scope", "col")
                                                .attr_signal("colspan", page.io_full_parenteral.signal().map(|is_full| if is_full {"6"} else {"2"}))
                                                .text("Parenteral fluid")
                                                .child(toggle_full_btn(page.io_full_parenteral.clone()))
                                            }),
                                            html!("th", {
                                                .class("table-success")
                                                .attr("scope", "col")
                                                .attr_signal("colspan", page.io_full_oral.signal().map(|is_full| if is_full {"5"} else {"2"}))
                                                .text("Oral fluid")
                                                .child(toggle_full_btn(page.io_full_oral.clone()))
                                            }),
                                            html!("th", {
                                                .class("table-warning")
                                                .attr("scope", "col")
                                                .attr_signal("colspan", page.io_full_output.signal().map(|is_full| if is_full {"3"} else {"2"}))
                                                .text("Output")
                                                .child(toggle_full_btn(page.io_full_output.clone()))
                                            }),
                                            html!("th", {
                                                .attr("scope", "col").attr("rowspan", "2")
                                                .style("vertical-align","middle")
                                                .child_signal(page.io_id.signal().map(clone!(page => move |io_id| {
                                                    (io_id > 0).then(|| {
                                                        html!("button", {
                                                            .attr("type","button")
                                                            .class(class::BTN_BLUEO)
                                                            .child(html!("i", {.class(class::FA_PLUS_L)}))
                                                            .text("เพิ่ม")
                                                            .event(clone!(page => move |_:events::Click| {
                                                                page.renew_form();
                                                            }))
                                                        })
                                                    })
                                                })))
                                            }),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("th", {.attr("scope", "col").text("ประเภท").class("table-info").visible_signal(page.io_full_parenteral.signal())}),
                                            html!("th", {.attr("scope", "col").text("ชื่อสารน้ำ/เลือด").class("table-info")}),
                                            html!("th", {.attr("scope", "col").text("ปริมาณ").class("table-info").visible_signal(page.io_full_parenteral.signal())}),
                                            html!("th", {.attr("scope", "col").text("ได้รับ").class("table-info")}),
                                            html!("th", {.attr("scope", "col").text("ยกไป").class("table-info").visible_signal(page.io_full_parenteral.signal())}),
                                            html!("th", {.attr("scope", "col").text("หมายเหตุ").class("table-info").visible_signal(page.io_full_parenteral.signal())}),
                                            html!("th", {.attr("scope", "col").text("ชื่อของเหลว").class("table-success")}),
                                            html!("th", {.attr("scope", "col").text("ปริมาณ").class("table-success").visible_signal(page.io_full_oral.signal())}),
                                            html!("th", {.attr("scope", "col").text("ได้รับ").class("table-success")}),
                                            html!("th", {.attr("scope", "col").text("ยกไป").class("table-success").visible_signal(page.io_full_oral.signal())}),
                                            html!("th", {.attr("scope", "col").text("หมายเหตุ").class("table-success").visible_signal(page.io_full_oral.signal())}),
                                            html!("th", {.attr("scope", "col").text("ประเภท").class("table-warning")}),
                                            html!("th", {.attr("scope", "col").text("ปริมาณ").class("table-warning")}),
                                            html!("th", {.attr("scope", "col").text("หมายเหตุ").class("table-warning").visible_signal(page.io_full_output.signal())}),
                                        ])
                                    }),
                                ])
                            }),
                            html!("tbody", {
                                //.attr("id", "data_io_table")
                                // night shift details // .sort_by_cloned(sort_io_shift)
                                .children_signal_vec(page.io_night_shifts.signal_vec_cloned().enumerate().map(
                                    clone!(app, page => move |(i, shift)| render_io(i, shift, page.io_id.signal(), page.clone(), app.clone()))
                                ))
                                // night shift summary
                                .children_signal_vec(page.io_night_shifts.signal_vec_cloned().to_signal_cloned().map(
                                    clone!(app, page => move |shifts| Self::render_shift(&shifts, app.shift(NurseShift::Night), page.clone()))
                                ).to_signal_vec())
                                // day shift details // .sort_by_cloned(sort_io_shift)
                                .children_signal_vec(page.io_day_shifts.signal_vec_cloned().enumerate().map(
                                    clone!(app, page => move |(i, shift)| render_io(i, shift, page.io_id.signal(), page.clone(), app.clone()))
                                ))
                                // day shift summary
                                .children_signal_vec(page.io_day_shifts.signal_vec_cloned().to_signal_cloned().map(
                                    clone!(app, page => move |shifts| Self::render_shift(&shifts, app.shift(NurseShift::Day), page.clone()))
                                ).to_signal_vec())
                                // evening shift details // .sort_by_cloned(sort_io_shift)
                                .children_signal_vec(page.io_evening_shifts.signal_vec_cloned().enumerate().map(
                                    clone!(app, page => move |(i, shift)| render_io(i, shift, page.io_id.signal(), page.clone(), app.clone()))
                                ))
                                // evening shift summary
                                .children_signal_vec(page.io_evening_shifts.signal_vec_cloned().to_signal_cloned().map(
                                    clone!(app, page => move |shifts| Self::render_shift(&shifts, app.shift(NurseShift::Evening), page.clone()))
                                ).to_signal_vec())
                                // new IO
                                .child_signal(map_ref!{
                                    let allowed = page.is_ipd().map(clone!(app => move |is_ipd_opt| is_ipd_opt.map(|is_ipd| {
                                        if is_ipd {
                                            app.has_permission(Permission::IoAdd)
                                        } else {
                                            app.has_permission(Permission::OpdErIoAdd)
                                        }
                                    }).unwrap_or_default())),
                                    let io_id = page.io_id.signal() =>
                                    *allowed && *io_id == 0
                                }.map(clone!(app, page => move |ok| ok.then(||html!("tr", {
                                    .class("table-danger")
                                    .children(Self::render_form(page.clone(), app.clone()))
                                })))))
                                // total shift summary
                                .children_signal_vec(map_ref!{
                                    let night = page.io_night_shifts.signal_vec_cloned().to_signal_cloned(),
                                    let day = page.io_day_shifts.signal_vec_cloned().to_signal_cloned(),
                                    let evening = page.io_evening_shifts.signal_vec_cloned().to_signal_cloned() => {
                                        let night_parenteral = night.iter().map(|io| io.io_parenteral_absorb.unwrap_or_default()).sum::<Decimal>();
                                        let day_parenteral = day.iter().map(|io| io.io_parenteral_absorb.unwrap_or_default()).sum::<Decimal>();
                                        let evening_parenteral = evening.iter().map(|io| io.io_parenteral_absorb.unwrap_or_default()).sum::<Decimal>();
                                        let night_oral = night.iter().map(|io| io.io_oral_absorb.unwrap_or_default()).sum::<i32>();
                                        let day_oral = day.iter().map(|io| io.io_oral_absorb.unwrap_or_default()).sum::<i32>();
                                        let evening_oral = evening.iter().map(|io| io.io_oral_absorb.unwrap_or_default()).sum::<i32>();
                                        let night_output = night.iter().map(|io| io.io_output_amount.unwrap_or_default()).sum::<i32>();
                                        let day_output = day.iter().map(|io| io.io_output_amount.unwrap_or_default()).sum::<i32>();
                                        let evening_output = evening.iter().map(|io| io.io_output_amount.unwrap_or_default()).sum::<i32>();
                                        (
                                            night_parenteral + day_parenteral + evening_parenteral,
                                            night_oral + day_oral + evening_oral,
                                            night_output + day_output + evening_output,
                                        )
                                    }
                                }.map(move |(parenteral, oral, output)| {
                                    if parenteral > Decimal::ZERO || oral > 0 || output > 0 {
                                        vec![
                                            html!("tr", {
                                                .class("table-secondary")
                                                .children([
                                                    html!("td", {
                                                        .attr("colspan","3").attr("rowspan","2").class("fw-bold").style("vertical-align","middle")
                                                        .text("รวม 24 ชั่วโมง")
                                                    }),
                                                    html!("td", {
                                                        .attr_signal("colspan", page.io_full_parenteral.signal().map(|is_full| if is_full {"6"} else {"2"}))
                                                        .style("text-align","center").class("fw-bold")
                                                        .text(&thousands(&decimal_rescale(parenteral,2).to_string())).text(" c.c.")
                                                    }),
                                                    html!("td", {
                                                        .attr_signal("colspan", page.io_full_oral.signal().map(|is_full| if is_full {"5"} else {"2"}))
                                                        .style("text-align","center").class("fw-bold")
                                                        .text(&thousands(&oral.to_string())).text(" c.c.")
                                                    }),
                                                    html!("td", {
                                                        .attr_signal("colspan", page.io_full_output.signal().map(|is_full| if is_full {"3"} else {"2"}))
                                                        .style("text-align","center").class("fw-bold")
                                                        .text(&thousands(&output.to_string())).text(" c.c.")
                                                    }),
                                                    html!("td"),
                                                ])
                                            }),
                                            html!("tr", {
                                                .class("table-secondary")
                                                .children([
                                                    html!("td", {
                                                        .attr_signal("colspan", map_ref!{
                                                            let io_full_parenteral = page.io_full_parenteral.signal(),
                                                            let io_full_oral = page.io_full_oral.signal() =>
                                                            (*io_full_parenteral, *io_full_oral)
                                                        }.map(|tuple| {
                                                            match tuple {
                                                                (true, true) => "11",
                                                                (true, false) => "8",
                                                                (false, true) => "7",
                                                                (false, false) => "4",
                                                            }
                                                        }))
                                                        .style("text-align","center").class("fw-bold")
                                                        .text(&thousands(&decimal_rescale(parenteral + Decimal::from(oral),2).to_string())).text(" c.c.")
                                                    }),
                                                    html!("td", {
                                                        .attr_signal("colspan", page.io_full_output.signal().map(|is_full| if is_full {"3"} else {"2"}))
                                                        .style("text-align","center").class("fw-bold")
                                                        .text(&thousands(&output.to_string())).text(" c.c.")
                                                    }),
                                                    html!("td"),
                                                ])
                                            }),
                                        ]
                                    } else {
                                        Vec::new()
                                    }
                                }).to_signal_vec())
                            }),
                        ])
                    })))
                }),
            ])
        })
    }

    fn render_form(page: Rc<Self>, app: Rc<App>) -> Vec<Dom> {
        vec![
            html!("td", {.class("p-0")}),
            html!("td", {
                .class("p-0")
                .child(doms::date_picker(
                    page.io_date.clone(),
                    page.form_changed.clone(), always(false), None,
                    |d| d.style("min-width","120px"),
                    |d| d.class(class::FORM_CTRL_ONLY_SM_B0),
                    |d| d.class(class::FORM_CTRL_ONLY_SM_B0),
                    |s| s, always(None),
                ))
            }),
            html!("td", {
                .class("p-0")
                .child(doms::time_picker(
                    page.io_time.clone(),
                    page.form_changed.clone(), always(false), None,
                    |d| d.style("min-width","95px"),
                    |d| d.class(class::FORM_CTRL_ONLY_SM_B0),
                    |d| d.class(class::FORM_CTRL_ONLY_SM_B0),
                    |s| s, always(None),
                ))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_parenteral.signal())
                .child(html!("select" => HtmlSelectElement, {
                    .class(class::FORM_SELECT_SM_R0)
                    .style("min-width","150px")
                    .style("border-color","transparent")
                    .children([
                        html!("option", {.attr("value", "").text("เลือก")}),
                        html!("option", {.attr("value", "iv").text("IV")}),
                        html!("option", {.attr("value", "medication").text("Medication")}),
                        html!("option", {.attr("value", "blood_component").text("Blood component")}),
                        html!("option", {.attr("value", "other").text("Other")}),
                    ])
                    .apply(mixins::string_value_select(page.io_parenteral_type.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "text")
                    .class(class::FORM_CTRL_SM_B0)
                    .attr("maxlength", "200")
                    .style("min-width","150px")
                    .apply(mixins::string_value(page.io_parenteral_name.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_parenteral.signal())
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "number")
                    .class(class::FORM_CTRL_SM_B0)
                    .attr("step", "0.1")
                    .style("min-width","70px")
                    .apply(mixins::string_value(page.io_parenteral_amount.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "number")
                    .class(class::FORM_CTRL_SM_B0)
                    .attr("step", "0.1")
                    .style("min-width","70px")
                    .apply(mixins::string_value(page.io_parenteral_absorb.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_parenteral.signal())
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "number")
                    .class(class::FORM_CTRL_SM_B0)
                    .attr("step", "0.1")
                    .style("min-width","70px")
                    .apply(mixins::string_value(page.io_parenteral_carry_forward.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_parenteral.signal())
                .child(html!("textarea" => HtmlTextAreaElement, {
                    .class(class::FORM_CTRL_SM_B0)
                    .attr("rows", "1")
                    .style("min-width","80px")
                    .apply(mixins::string_value(page.io_parenteral_remark.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "text")
                    .class(class::FORM_CTRL_SM_B0)
                    .attr("maxlength", "200")
                    .style("min-width","150px")
                    .apply(mixins::string_value(page.io_oral_name.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_oral.signal())
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "number")
                    .class(class::FORM_CTRL_SM_B0)
                    .style("min-width","70px")
                    .apply(mixins::string_value(page.io_oral_amount.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "number")
                    .class(class::FORM_CTRL_SM_B0)
                    .style("min-width","70px")
                    .apply(mixins::string_value(page.io_oral_absorb.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_oral.signal())
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "number")
                    .class(class::FORM_CTRL_SM_B0)
                    .style("min-width","70px")
                    .apply(mixins::string_value(page.io_oral_carry_forward.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_oral.signal())
                .child(html!("textarea" => HtmlTextAreaElement, {
                    .class(class::FORM_CTRL_SM_B0)
                    .attr("rows", "1")
                    .style("min-width","80px")
                    .apply(mixins::string_value(page.io_oral_remark.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .child(html!("select" => HtmlSelectElement, {
                    .class(class::FORM_SELECT_SM_R0)
                    .style("min-width","130px")
                    .style("border-color","transparent")
                    .children([
                        html!("option", {.attr("value", "").text("เลือก")}),
                        html!("option", {.attr("value", "vomit").text("Vomit")}),
                        html!("option", {.attr("value", "gastric_content").text("Gastric content")}),
                        html!("option", {.attr("value", "drain_tube").text("Drain tube")}),
                        html!("option", {.attr("value", "urine").text("Urine")}),
                        html!("option", {.attr("value", "dyalysis").text("Dialysis")}),
                        html!("option", {.attr("value", "other").text("Other")}),
                    ])
                    .apply(mixins::string_value_select(page.io_output_type.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "number")
                    .class(class::FORM_CTRL_SM_B0)
                    .style("min-width","70px")
                    .apply(mixins::string_value(page.io_output_amount.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .visible_signal(page.io_full_output.signal())
                .child(html!("textarea" => HtmlTextAreaElement, {
                    .class(class::FORM_CTRL_SM_B0)
                    .style("min-width","80px")
                    .attr("rows", "1")
                    .apply(mixins::string_value(page.io_output_remark.clone(), page.form_changed.clone()))
                }))
            }),
            html!("td", {
                .class("p-0")
                .child(html!("div", {
                    .style("min-width","145px")
                    .class("text-center")
                    .child_signal(map_ref!{
                        let is_ipd = page.is_ipd(),
                        let is_admited = page.is_admited(),
                        let changed = page.form_changed.signal() =>
                        (*changed, *is_ipd, *is_admited)
                    }.map(clone!(app, page => move |(changed, is_ipd_opt, is_admited)| {
                        (changed && is_ipd_opt.map(|is_ipd| {
                            if is_ipd {
                                app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIo, !is_admited)
                            } else {
                                app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIo, false)
                            }
                        }).unwrap_or_default()).then(|| {
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_SM_BLUE)
                                .text("บันทึก")
                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                    Self::add_or_edit(page.clone(), app.clone());
                                }), app.state()))
                            })
                        })
                    })))
                    .child_signal(page.form_changed.signal_cloned().map(clone!(page => move |changed| {
                        changed.then(|| {
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_R_GRAY)
                                .child(html!("i", {.class(class::FA_UNDO)}))
                                .event(clone!(page => move |_:events::Click| {
                                    page.renew_form();
                                }))
                            })
                        })
                    })))
                    .child_signal(map_ref!{
                        let is_ipd = page.is_ipd(),
                        let is_admited = page.is_admited(),
                        let io_id = page.io_id.signal() =>
                        (*io_id, *is_ipd, *is_admited)
                    }.map(clone!(app, page => move |(io_id, is_ipd_opt, is_admited)| {
                        (io_id > 0 && is_ipd_opt.map(|is_ipd| {
                            if is_ipd {
                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdIo, !is_admited)
                            } else {
                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErIo, false)
                            }
                        }).unwrap_or_default()).then(|| {
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_SM_R_RED)
                                .text("ลบ")
                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                    Self::delete(page.clone(), app.clone());
                                }), app.state()))
                            })
                        })
                    })))
                }))
            }),
        ]
    }

    fn render_shift(io_shifts: &[Rc<IoShift>], shift: Option<Shift>, page: Rc<Self>) -> Vec<Dom> {
        let (shift_name, shift_duration, shift_detail) = shift.map(|s| (s.shift.long(), s.duration, s.detail)).unwrap_or(("", String::new(), String::new()));

        if io_shifts.is_empty() {
            Vec::new()
        } else {
            let parenteral = io_shifts.iter().map(|io| io.io_parenteral_absorb.unwrap_or_default()).sum::<Decimal>();
            let oral = io_shifts.iter().map(|io| io.io_oral_absorb.unwrap_or_default()).sum::<i32>();
            let output = io_shifts.iter().map(|io| io.io_output_amount.unwrap_or_default()).sum::<i32>();

            vec![
                html!("tr", {
                    .class("table-secondary")
                    .children([
                        html!("td", {
                            .attr("colspan","3").attr("rowspan","2")
                            .class("fw-bold")
                            .style("vertical-align","middle")
                            .text(&["เวร",shift_name," รวม ", &shift_duration].concat())
                            .attr("title", &shift_detail)
                        }),
                        html!("td", {
                            .attr_signal("colspan", page.io_full_parenteral.signal().map(|is_full| if is_full {"6"} else {"2"}))
                            .class("fw-bold").style("text-align","center")
                            .text(&thousands(&decimal_rescale(parenteral,2).to_string()))
                            .text(" c.c.")
                        }),
                        html!("td", {
                            .attr_signal("colspan", page.io_full_oral.signal().map(|is_full| if is_full {"5"} else {"2"}))
                            .class("fw-bold").style("text-align","center")
                            .text(&thousands(&oral.to_string()))
                            .text(" c.c.")
                        }),
                        html!("td", {
                            .attr_signal("colspan", page.io_full_output.signal().map(|is_full| if is_full {"3"} else {"2"}))
                            .class("fw-bold").style("text-align","center")
                            .text(&thousands(&output.to_string()))
                            .text(" c.c.")
                        }),
                        html!("td"),
                    ])
                }),
                html!("tr", {
                    .class("table-secondary")
                    .children([
                        html!("td", {
                            .attr_signal("colspan", map_ref!{
                                let io_full_parenteral = page.io_full_parenteral.signal(),
                                let io_full_oral = page.io_full_oral.signal() =>
                                (*io_full_parenteral, *io_full_oral)
                            }.map(|tuple| {
                                match tuple {
                                    (true, true) => "11",
                                    (true, false) => "8",
                                    (false, true) => "7",
                                    (false, false) => "4",
                                }
                            }))
                            .class("fw-bold").style("text-align","center")
                            .text(&thousands(&decimal_rescale(parenteral + Decimal::from(oral),2).to_string())).text(" c.c.")
                        }),
                        html!("td", {
                            .attr_signal("colspan", page.io_full_output.signal().map(|is_full| if is_full {"3"} else {"2"}))
                            .class("fw-bold").style("text-align","center")
                            .text(&thousands(&output.to_string())).text(" c.c.")
                        }),
                        html!("td"),
                    ])
                }),
            ]
        }
    }
}

fn render_io<S>(i: ReadOnlyMutable<Option<usize>>, io: Rc<IoShift>, id: S, page: Rc<IoCpn>, app: Rc<App>) -> Dom
where
    S: Signal<Item = u32> + 'static,
{
    let bc = id.broadcast();
    html!("tr", {
        .class_signal("table-danger", bc.signal().map(clone!(io => move |io_id| io_id == io.io_id)))
        .children_signal_vec(bc.signal().map(clone!(app, page, io, i => move |io_id| {
            if io_id == io.io_id {
                clone!(app, page => IoCpn::render_form(page, app))
            } else {
                vec![
                    html!("td", {.text_signal(i.signal_cloned().map(|opt| opt.map(|n| (n+1).to_string()).unwrap_or_default()))}),
                    html!("td", {.text(&date_th(&io.io_date))}),
                    html!("td", {.text(&time_hm(&io.io_time))}),
                    html!("td", {
                        .text(parenteral_type(&io.io_parenteral_type))
                        .class(class::BG_CYAN_10)
                        .visible_signal(page.io_full_parenteral.signal())
                    }),
                    html!("td", {
                        .children(doms::square_bracket_to_span(&io.io_parenteral_name.clone().unwrap_or_default()))
                        .class(class::BG_CYAN_10)
                    }),
                    html!("td", {
                        .text(&io.io_parenteral_amount.map(|d| thousands(&decimal_rescale(d,2).to_string())).unwrap_or_default())
                        .class(class::BG_CYAN_10)
                        .class("text-center")
                        .visible_signal(page.io_full_parenteral.signal())
                    }),
                    html!("td", {
                        .text(&io.io_parenteral_absorb.map(|d| thousands(&decimal_rescale(d,2).to_string())).unwrap_or_default())
                        .class(class::BG_CYAN_10)
                        .class("text-center")
                    }),
                    html!("td", {
                        .text(&io.io_parenteral_carry_forward.map(|d| thousands(&decimal_rescale(d,2).to_string())).unwrap_or_default())
                        .class(class::BG_CYAN_10)
                        .class("text-center")
                        .visible_signal(page.io_full_parenteral.signal())
                    }),
                    html!("td", {
                        .text(&io.io_parenteral_remark.clone().unwrap_or_default())
                        .class(class::BG_CYAN_10)
                        .visible_signal(page.io_full_parenteral.signal())
                    }),
                    html!("td", {
                        .text(&io.io_oral_name.clone().unwrap_or_default())
                        .class(class::BG_GREEN_10)
                    }),
                    html!("td", {
                        .text(&io.io_oral_amount.map(|i| thousands(&i.to_string())).unwrap_or_default())
                        .class(class::BG_GREEN_10)
                        .class("text-center")
                        .visible_signal(page.io_full_oral.signal())
                    }),
                    html!("td", {
                        .text(&io.io_oral_absorb.map(|i| thousands(&i.to_string())).unwrap_or_default())
                        .class(class::BG_GREEN_10)
                        .class("text-center")
                    }),
                    html!("td", {
                        .text(&io.io_oral_carry_forward.map(|i| thousands(&i.to_string())).unwrap_or_default())
                        .class(class::BG_GREEN_10)
                        .class("text-center")
                        .visible_signal(page.io_full_oral.signal())
                    }),
                    html!("td", {
                        .text(&io.io_oral_remark.clone().unwrap_or_default())
                        .class(class::BG_GREEN_10)
                        .visible_signal(page.io_full_oral.signal())
                    }),
                    html!("td", {
                        .text(output_type(&io.io_output_type))
                        .class(class::BG_GOLD_10)
                    }),
                    html!("td", {
                        .text(&io.io_output_amount.map(|i| thousands(&i.to_string())).unwrap_or_default())
                        .class(class::BG_GOLD_10)
                        .class("text-center")
                    }),
                    html!("td", {
                        .text(&io.io_output_remark.clone().unwrap_or_default())
                        .class(class::BG_GOLD_10)
                        .visible_signal(page.io_full_output.signal())
                    }),
                    html!("td", {
                        .class(class::TXT_C_P1)
                        .child_signal(page.is_ipd().map(clone!(app, page, io => move |is_ipd_opt| (if let Some(is_ipd) = is_ipd_opt {
                            if is_ipd {
                                app.has_permission(Permission::IoEdit)
                            } else {
                                app.has_permission(Permission::OpdErIoEdit)
                            }
                        } else {
                            false
                        }).then(|| {
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_GRAY)
                                .text("แก้ไข")
                                .event(clone!(page, io => move |_:events::Click| {
                                    page.io_id.set_neq(io.io_id);
                                    page.io_date.set_neq(io.io_date.to_string());
                                    page.io_time.set_neq(io.io_time.js_string());
                                    page.io_parenteral_type.set_neq(io.io_parenteral_type.clone().unwrap_or_default());
                                    page.io_parenteral_name.set_neq(io.io_parenteral_name.clone().unwrap_or_default());
                                    page.io_parenteral_amount.set_neq(io.io_parenteral_amount.map(|d| decimal_rescale(d,2).to_string()).unwrap_or_default());
                                    page.io_parenteral_absorb.set_neq(io.io_parenteral_absorb.map(|d| decimal_rescale(d,2).to_string()).unwrap_or_default());
                                    page.io_parenteral_carry_forward.set_neq(io.io_parenteral_carry_forward.map(|d| decimal_rescale(d,2).to_string()).unwrap_or_default());
                                    page.io_parenteral_remark.set_neq(io.io_parenteral_remark.clone().unwrap_or_default());
                                    page.io_oral_name.set_neq(io.io_oral_name.clone().unwrap_or_default());
                                    page.io_oral_amount.set_neq(io.io_oral_amount.map(|i| i.to_string()).unwrap_or_default());
                                    page.io_oral_absorb.set_neq(io.io_oral_absorb.map(|i| i.to_string()).unwrap_or_default());
                                    page.io_oral_carry_forward.set_neq(io.io_oral_carry_forward.map(|i| i.to_string()).unwrap_or_default());
                                    page.io_oral_remark.set_neq(io.io_oral_remark.clone().unwrap_or_default());
                                    page.io_output_type.set_neq(io.io_output_type.clone().unwrap_or_default());
                                    page.io_output_amount.set_neq(io.io_output_amount.map(|i| i.to_string()).unwrap_or_default());
                                    page.io_output_remark.set_neq(io.io_output_remark.clone().unwrap_or_default());
                                    page.version.set_neq(io.version);
                                    page.user_name.set_neq(io.user_name.clone().unwrap_or_default());
                                    page.entryposition.set_neq(io.entryposition.clone().unwrap_or_default());
                                    // page.shift.set_neq(io.shift.clone());
                                    page.form_changed.set_neq(false);
                                }))
                            })
                        }))))
                        .child(html!("i", {
                            .class(class::FA_USER_R)
                            .style("cursor","pointer")
                            .attr("title", &[io.user_name.clone().unwrap_or_default(), io.entryposition.as_ref().map(|entry| [" (", entry,")"].concat()).unwrap_or_default()].concat())
                        }))
                    }),
                ]
            }
        })).to_signal_vec())
    })
}

fn toggle_full_btn(is_full_mutable: Mutable<bool>) -> Dom {
    html!("button", {
        .attr("type","button")
        .class(class::BTN_SM_FL_BLUEO)
        .child(html!("i", {
            .class("fa-solid")
            .class_signal("fa-compress", is_full_mutable.signal())
            .class_signal("fa-expand", not(is_full_mutable.signal()))
        }))
        .event(clone!(is_full_mutable => move |_: events::Click| {
            is_full_mutable.set(!is_full_mutable.get());
        }))
    })
}

fn parenteral_type(opt: &Option<String>) -> &'static str {
    opt.as_ref()
        .map(|ty| match ty.as_str() {
            "iv" => "IV",
            "medication" => "Medication",
            "blood_component" => "Blood component",
            "other" => "Other",
            _ => "",
        })
        .unwrap_or_default()
}

fn output_type(opt: &Option<String>) -> &'static str {
    opt.as_ref()
        .map(|ty| match ty.as_str() {
            "urine" => "Urine",
            "vomit" => "Vomit",
            "gastric_content" => "Gastric content",
            "drain_tube" => "Drain tube",
            "dyalysis" => "Dialysis",
            "other" => "Other",
            _ => "",
        })
        .unwrap_or_default()
}
