use dominator::{Dom, clone, events, html, text, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{
    ops::{Div, Mul},
    rc::Rc,
};
use time::Date;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_model::{
    SCREEN_WIDTH_EXTRA,
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::DocumentType,
    ipd::mra::{IpdMra, MraParams},
    report::{SystemReport, TypstReport},
    route::Route,
};
use kphis_ui_app::App;
use kphis_ui_component::{
    gadget::{aside_resizer::AsideResizerCpn, pdf_button::PdfButtons},
    mra::{
        control::{Mra, MraCol, MraRow, Report},
        doc::MraDoc,
        ipd_mra, ipd_mra_psy,
    },
    show_patient_main::ShowPatientMainCpn,
};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, date_str_th, date_th_opt, js_now},
    util::{f64_rescale, str_some, zero_none},
};

/// - GET `EndPoint::IpdMra`
/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - POST/PUT `EndPoint::IpdMra` (guarded, remove 'แก้ไข','บันทึก' btn)
/// - DELETE `EndPoint::IpdMra` (guarded, remove 'ลบ' btn)
#[derive(Clone, Default)]
pub struct IpdMraPage {
    list_loaded: Mutable<bool>,
    ipd_mra_list: MutableVec<Rc<IpdMra>>,
    patient: Mutable<Rc<ShowPatientMainCpn>>,

    ipd_mra_selected: Mutable<Option<Rc<IpdMra>>>,
    ipd_mra_mutable: Mutable<Option<Rc<IpdMraMutable>>>,

    an: Mutable<String>,
    selected_template: Mutable<Option<SystemReport>>,
    load_and_render_report_svg: Mutable<bool>,
    selected_document: Mutable<Option<DocumentType>>,
    load_and_render_document_svg: Mutable<bool>,
}

impl IpdMraPage {
    pub fn new(an: String) -> Rc<Self> {
        Rc::new(Self {
            an: Mutable::new(an),
            ..Default::default()
        })
    }

    fn load_list(page: Rc<Self>, app: Rc<App>) {
        if let Some(an) = str_some(page.an.get_cloned()) {
            page.ipd_mra_list.lock_mut().clear();
            let params = MraParams { an: Some(an), ..Default::default() };
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdMra`
                    match IpdMra::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.ipd_mra_list.lock_mut().extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn init_inner(page: Rc<Self>, app: Rc<App>) {
        let no_mra = page.ipd_mra_mutable.lock_ref().is_none();
        if no_mra {
            if let Some((hn, adm_date, dch_date)) = page.patient.lock_ref().patient.lock_ref().as_ref().map(|pt| (pt.hn.clone(), pt.regdate, pt.dchdate)) {
                page.ipd_mra_mutable.set(Some(IpdMraMutable::new(&hn, adm_date, dch_date, &app.doctor_name(), page.clone())));
            }
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - IPD MRA");

        let show_patient_main = ShowPatientMainCpn::new_with_an(page.an.get_cloned());
        let hn = show_patient_main.hn.clone();
        let patient_main = ShowPatientMainCpn::render(false, show_patient_main.clone(), app.clone());
        page.patient.set(show_patient_main);

        html!("div", {
            .child(patient_main)
            .child_signal(window_size().map(|ws| ws.width < SCREEN_WIDTH_EXTRA).dedupe().map(move |is_not_wide| {
                Some(if is_not_wide {
                    Self::render_body(page.clone(), app.clone())
                } else {
                    // aside_resizer
                    AsideResizerCpn::render(
                        Self::render_body(page.clone(), app.clone()),
                        Some((true, page.patient.lock_ref().patient.clone())),
                        AsideResizerCpn::new(
                            page.selected_template.clone(), page.load_and_render_report_svg.clone(),
                            page.selected_document.clone(), page.load_and_render_document_svg.clone(),
                            page.an.clone(), hn.clone(), SystemReport::ipd_set(),
                            "ipd-mra-page-main", None, None, app.clone(),
                        ),
                        app.clone(),
                    )
                })
            }))
        })
    }

    pub fn render_body(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.list_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_list(page.clone(), app.clone());
                    page.list_loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::CONF_P3)
            .attr("id", "ipd-mra-page-main")
            .style("min-width","920px")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {
                            .class("col-auto")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_LT_BLUE)
                                .child(html!("i", {.class(class::FA_L_ARROW)}))
                                .text(" กลับ")
                                .event(clone!(app => move |_: events::Click| {
                                    if app.go_back_else() {
                                        for route in [
                                            Route::IpdPostAdmitList { view_by: String::from("doctor") },
                                            Route::IpdPostAdmitList { view_by: String::from("nurse") },
                                            Route::IpdPostAdmitList { view_by: String::from("pharmacist") },
                                            Route::IpdPostAdmitList { view_by: String::from("other") },
                                            Route::Info,
                                        ] {
                                            if route.has_permission(app.state()) {
                                                route.hard_redirect();
                                                break;
                                            }
                                        }
                                    }
                                }))
                            }))
                        }),
                        html!("div", {
                            .class(class::COLA_BOLD_P2)
                            .text("IPD MEDICAL RECORD AUDIT (MRA)")
                        }),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class(class::FLEX_WRAP_T)
                    .children_signal_vec(page.ipd_mra_list.signal_vec_cloned().map(clone!(page, app => move |mra| {
                        let mra_id = mra.mra_id;
                        html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_LT)
                            .class_signal("btn-primary", page.ipd_mra_selected.signal_cloned().map(move |opt| opt.as_ref().map(|selected_mra| selected_mra.mra_id == mra_id).unwrap_or_default()))
                            .class_signal("btn-secondary", not(page.ipd_mra_selected.signal_cloned().map(move |opt| opt.as_ref().map(|selected_mra| selected_mra.mra_id == mra_id).unwrap_or_default())))
                            .text(if mra.audit_type.as_str() == "E" {"External"} else {"Internal"})
                            .text(if mra.is_psychiatry {" จิตเวช"} else {""})
                            .text(&[" (",&mra.score().to_string(),"/",&mra.full().to_string(),") โดย "].concat())
                            .text(&mra.auditor.clone().unwrap_or(String::from("ไม่ระบุผู้ประเมิน")))
                            .event(clone!(page, app => move |_:events::Click| {
                                Self::init_inner(page.clone(), app.clone());
                                page.ipd_mra_selected.set(Some(mra.clone()));
                            }))
                        })
                    })))
                    .child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_LT)
                        .class_signal("btn-primary", page.ipd_mra_selected.signal_cloned().map(move |opt| opt.is_none()))
                        .class_signal("btn-secondary", page.ipd_mra_selected.signal_cloned().map(move |opt| opt.is_some()))
                        .child(html!("i", {.class(class::FA_PLUS)}))
                        .text(" เพิ่ม")
                        .event(clone!(app, page => move |_:events::Click| {
                            Self::init_inner(page.clone(), app.clone());
                            page.ipd_mra_selected.set(None);
                        }))
                    }))
                }),
            ])
            .child_signal(page.ipd_mra_mutable.signal_cloned().map(move |opt| {
                opt.map(|mra_mutable| {
                    IpdMraMutable::render(mra_mutable, page.list_loaded.clone(), app.clone())
                })
            }))
        })
    }
}

#[derive(Clone, Default)]
struct IpdMraMutable {
    new_mra: Mutable<Option<Rc<IpdMra>>>,
    parent_auditor: Mutable<Option<String>>,

    selected_template: Mutable<Option<SystemReport>>,
    load_and_render_report_svg: Mutable<bool>,

    selected_document: Mutable<Option<DocumentType>>,
    load_and_render_document_svg: Mutable<bool>,

    ipd_mra_selected: Mutable<Option<Rc<IpdMra>>>,

    // checker: Mutable<bool>,
    changed: Mutable<bool>,
    selected_row: Mutable<Option<MraRow>>,

    mra_id: Mutable<u32>, // 0 is new
    hn: Mutable<Option<String>>,
    an: Mutable<String>,
    adm_date: Mutable<Option<Date>>,
    dch_date: Mutable<Option<Date>>,
    /// - I : Internal
    /// - E : Extermal
    audit_type: Mutable<String>,
    is_psychiatry: Mutable<bool>,
    is_not_sorted: Mutable<bool>,
    is_unknown: Mutable<bool>,
    overall: Mutable<Option<String>>,
    overall_text: Mutable<Option<String>>,
    auditor: Mutable<Option<String>>,
    audit_date: Mutable<String>,

    document: Mutable<Option<MraDoc>>,
    full: Mutable<usize>,
    score: Mutable<usize>,

    recal_all: Mutable<bool>,
    recal: Mutable<bool>,

    sd_recal: Mutable<bool>,
    so_recal: Mutable<bool>,
    ic_recal: Mutable<bool>,
    hx_recal: Mutable<bool>,
    pe_recal: Mutable<bool>,
    pn_recal: Mutable<bool>,
    cr_recal: Mutable<bool>,
    ar_recal: Mutable<bool>,
    on_recal: Mutable<bool>,
    lr_recal: Mutable<bool>,
    rr_recal: Mutable<bool>,
    nn_recal: Mutable<bool>,

    sd_full: Mutable<usize>,
    so_full: Mutable<usize>,
    ic_full: Mutable<usize>,
    hx_full: Mutable<usize>,
    pe_full: Mutable<usize>,
    pn_full: Mutable<usize>,
    cr_full: Mutable<usize>,
    ar_full: Mutable<usize>,
    on_full: Mutable<usize>,
    lr_full: Mutable<usize>,
    rr_full: Mutable<usize>,
    nn_full: Mutable<usize>,

    sd_score: Mutable<usize>,
    so_score: Mutable<usize>,
    ic_score: Mutable<usize>,
    hx_score: Mutable<usize>,
    pe_score: Mutable<usize>,
    pn_score: Mutable<usize>,
    cr_score: Mutable<usize>,
    ar_score: Mutable<usize>,
    on_score: Mutable<usize>,
    lr_score: Mutable<usize>,
    rr_score: Mutable<usize>,
    nn_score: Mutable<usize>,

    sd_m: Mutable<bool>,
    sd_n: Mutable<bool>,
    sd_1: Mutable<Option<bool>>,
    sd_2: Mutable<Option<bool>>,
    sd_3: Mutable<Option<bool>>,
    sd_4: Mutable<Option<bool>>,
    sd_5: Mutable<Option<bool>>,
    sd_6: Mutable<Option<bool>>,
    sd_7: Mutable<Option<bool>>,
    sd_8: Mutable<Option<bool>>,
    sd_9: Mutable<Option<bool>>,
    sd_text: Mutable<Option<String>>,

    so_m: Mutable<bool>,
    so_n: Mutable<bool>,
    so_1: Mutable<Option<bool>>,
    so_2: Mutable<Option<bool>>,
    so_3: Mutable<Option<bool>>,
    so_4: Mutable<Option<bool>>,
    so_5: Mutable<Option<bool>>,
    so_6: Mutable<Option<bool>>,
    so_7: Mutable<Option<bool>>,
    so_text: Mutable<Option<String>>,

    ic_m: Mutable<bool>,
    ic_n: Mutable<bool>,
    ic_1: Mutable<Option<bool>>,
    ic_2: Mutable<Option<bool>>,
    ic_3: Mutable<Option<bool>>,
    ic_4: Mutable<Option<bool>>,
    ic_5: Mutable<Option<bool>>,
    ic_6: Mutable<Option<bool>>,
    ic_7: Mutable<Option<bool>>,
    ic_8: Mutable<Option<bool>>,
    ic_9: Mutable<Option<bool>>,
    ic_text: Mutable<Option<String>>,

    hx_m: Mutable<bool>,
    hx_n: Mutable<bool>,
    hx_1: Mutable<Option<bool>>,
    hx_2: Mutable<Option<bool>>,
    hx_3: Mutable<Option<bool>>,
    hx_4: Mutable<Option<bool>>,
    hx_5: Mutable<Option<bool>>,
    hx_6: Mutable<Option<bool>>,
    hx_7: Mutable<Option<bool>>,
    hx_8: Mutable<Option<bool>>,
    hx_9: Mutable<Option<bool>>,
    hx_text: Mutable<Option<String>>,

    pe_m: Mutable<bool>,
    pe_n: Mutable<bool>,
    pe_1: Mutable<Option<bool>>,
    pe_2: Mutable<Option<bool>>,
    pe_3: Mutable<Option<bool>>,
    pe_4: Mutable<Option<bool>>,
    pe_5: Mutable<Option<bool>>,
    pe_6: Mutable<Option<bool>>,
    pe_7: Mutable<Option<bool>>,
    pe_8: Mutable<Option<bool>>,
    pe_9: Mutable<Option<bool>>,
    pe_text: Mutable<Option<String>>,

    pn_m: Mutable<bool>,
    pn_n: Mutable<bool>,
    pn_1: Mutable<Option<bool>>,
    pn_2: Mutable<Option<bool>>,
    pn_3: Mutable<Option<bool>>,
    pn_4: Mutable<Option<bool>>,
    pn_5: Mutable<Option<bool>>,
    pn_6: Mutable<Option<bool>>,
    pn_7: Mutable<Option<bool>>,
    pn_8: Mutable<Option<bool>>,
    pn_9: Mutable<Option<bool>>,
    pn_text: Mutable<Option<String>>,

    cr_na: Mutable<bool>,
    cr_m: Mutable<bool>,
    cr_n: Mutable<bool>,
    cr_1: Mutable<Option<bool>>,
    cr_2: Mutable<Option<bool>>,
    cr_3: Mutable<Option<bool>>,
    cr_4: Mutable<Option<bool>>,
    cr_5: Mutable<Option<bool>>,
    cr_6: Mutable<Option<bool>>,
    cr_7: Mutable<Option<bool>>,
    cr_8: Mutable<Option<bool>>,
    cr_9: Mutable<Option<bool>>,
    cr_text: Mutable<Option<String>>,

    ar_na: Mutable<bool>,
    ar_m: Mutable<bool>,
    ar_n: Mutable<bool>,
    ar_1: Mutable<Option<bool>>,
    ar_2: Mutable<Option<bool>>,
    ar_3: Mutable<Option<bool>>,
    ar_4: Mutable<Option<bool>>,
    ar_5: Mutable<Option<bool>>,
    ar_6: Mutable<Option<bool>>,
    ar_7: Mutable<Option<bool>>,
    ar_8: Mutable<Option<bool>>,
    ar_9: Mutable<Option<bool>>,
    ar_text: Mutable<Option<String>>,

    on_na: Mutable<bool>,
    on_m: Mutable<bool>,
    on_n: Mutable<bool>,
    on_1: Mutable<Option<bool>>,
    on_2: Mutable<Option<bool>>,
    on_3: Mutable<Option<bool>>,
    on_4: Mutable<Option<bool>>,
    on_5: Mutable<Option<bool>>,
    on_6: Mutable<Option<bool>>,
    on_7: Mutable<Option<bool>>,
    on_8: Mutable<Option<bool>>,
    on_9: Mutable<Option<bool>>,
    on_text: Mutable<Option<String>>,

    lr_na: Mutable<bool>,
    lr_m: Mutable<bool>,
    lr_n: Mutable<bool>,
    lr_1: Mutable<Option<bool>>,
    lr_2: Mutable<Option<bool>>,
    lr_3: Mutable<Option<bool>>,
    lr_4: Mutable<Option<bool>>,
    lr_5: Mutable<Option<bool>>,
    lr_6: Mutable<Option<bool>>,
    lr_7: Mutable<Option<bool>>,
    lr_8: Mutable<Option<bool>>,
    lr_9: Mutable<Option<bool>>,
    lr_text: Mutable<Option<String>>,

    rr_na: Mutable<bool>,
    rr_m: Mutable<bool>,
    rr_n: Mutable<bool>,
    rr_1: Mutable<Option<bool>>,
    rr_2: Mutable<Option<bool>>,
    rr_3: Mutable<Option<bool>>,
    rr_4: Mutable<Option<bool>>,
    rr_5: Mutable<Option<bool>>,
    rr_6: Mutable<Option<bool>>,
    rr_7: Mutable<Option<bool>>,
    rr_8: Mutable<Option<bool>>,
    rr_9: Mutable<Option<bool>>,
    rr_text: Mutable<Option<String>>,

    nn_m: Mutable<bool>,
    nn_n: Mutable<bool>,
    nn_1: Mutable<Option<bool>>,
    nn_2: Mutable<Option<bool>>,
    nn_3: Mutable<Option<bool>>,
    nn_4: Mutable<Option<bool>>,
    nn_5: Mutable<Option<bool>>,
    nn_6: Mutable<Option<bool>>,
    nn_7: Mutable<Option<bool>>,
    nn_8: Mutable<Option<bool>>,
    nn_9: Mutable<Option<bool>>,
    nn_sub: Mutable<Option<bool>>,
    nn_text: Mutable<Option<String>>,
}

impl IpdMraMutable {
    fn new(hn: &Option<String>, adm_date: Option<Date>, dch_date: Option<Date>, auditor: &Option<String>, parent_page: Rc<IpdMraPage>) -> Rc<Self> {
        Rc::new(Self {
            new_mra: Mutable::new(Some(Rc::new(IpdMra::new(hn, &parent_page.an.lock_ref(), adm_date, dch_date, auditor)))),
            parent_auditor: Mutable::new(auditor.to_owned()),
            selected_template: parent_page.selected_template.clone(),
            load_and_render_report_svg: parent_page.load_and_render_report_svg.clone(),
            selected_document: parent_page.selected_document.clone(),
            load_and_render_document_svg: parent_page.load_and_render_document_svg.clone(),
            ipd_mra_selected: parent_page.ipd_mra_selected.clone(),
            recal_all: Mutable::new(true),
            document: Mutable::new(Some(ipd_mra::IPD_MRA_INTRO.clone())),
            ..Default::default()
        })
    }

    fn set_mra(&self, mra: Rc<IpdMra>) {
        self.changed.set(false);
        self.mra_id.set_neq(mra.mra_id);
        self.hn.set_neq(mra.hn.clone());
        self.an.set_neq(mra.an.clone());
        self.auditor.set_neq(mra.auditor.clone());
        self.adm_date.set_neq(mra.adm_date);
        self.dch_date.set_neq(mra.dch_date);
        self.audit_date.set_neq(mra.audit_date.map(|d| d.to_string()).unwrap_or_default());
        self.audit_type.set_neq(mra.audit_type.clone());
        self.is_psychiatry.set_neq(mra.is_psychiatry);
        self.is_not_sorted.set_neq(mra.is_not_sorted);
        self.is_unknown.set_neq(mra.is_unknown);
        self.overall.set_neq(mra.overall.clone());
        self.overall_text.set_neq(mra.overall_text.clone());
        self.sd_m.set_neq(mra.sd_m);
        self.sd_n.set_neq(mra.sd_n);
        self.sd_1.set_neq(mra.sd_1);
        self.sd_2.set_neq(mra.sd_2);
        self.sd_3.set_neq(mra.sd_3);
        self.sd_4.set_neq(mra.sd_4);
        self.sd_5.set_neq(mra.sd_5);
        self.sd_6.set_neq(mra.sd_6);
        self.sd_7.set_neq(mra.sd_7);
        self.sd_8.set_neq(mra.sd_8);
        self.sd_9.set_neq(mra.sd_9);
        self.sd_text.set_neq(mra.sd_text.clone());
        self.so_m.set_neq(mra.so_m);
        self.so_n.set_neq(mra.so_n);
        self.so_1.set_neq(mra.so_1);
        self.so_2.set_neq(mra.so_2);
        self.so_3.set_neq(mra.so_3);
        self.so_4.set_neq(mra.so_4);
        self.so_5.set_neq(mra.so_5);
        self.so_6.set_neq(mra.so_6);
        self.so_7.set_neq(mra.so_7);
        self.so_text.set_neq(mra.so_text.clone());
        self.ic_m.set_neq(mra.ic_m);
        self.ic_n.set_neq(mra.ic_n);
        self.ic_1.set_neq(mra.ic_1);
        self.ic_2.set_neq(mra.ic_2);
        self.ic_3.set_neq(mra.ic_3);
        self.ic_4.set_neq(mra.ic_4);
        self.ic_5.set_neq(mra.ic_5);
        self.ic_6.set_neq(mra.ic_6);
        self.ic_7.set_neq(mra.ic_7);
        self.ic_8.set_neq(mra.ic_8);
        self.ic_9.set_neq(mra.ic_9);
        self.ic_text.set_neq(mra.ic_text.clone());
        self.hx_m.set_neq(mra.hx_m);
        self.hx_n.set_neq(mra.hx_n);
        self.hx_1.set_neq(mra.hx_1);
        self.hx_2.set_neq(mra.hx_2);
        self.hx_3.set_neq(mra.hx_3);
        self.hx_4.set_neq(mra.hx_4);
        self.hx_5.set_neq(mra.hx_5);
        self.hx_6.set_neq(mra.hx_6);
        self.hx_7.set_neq(mra.hx_7);
        self.hx_8.set_neq(mra.hx_8);
        self.hx_9.set_neq(mra.hx_9);
        self.hx_text.set_neq(mra.hx_text.clone());
        self.pe_m.set_neq(mra.pe_m);
        self.pe_n.set_neq(mra.pe_n);
        self.pe_1.set_neq(mra.pe_1);
        self.pe_2.set_neq(mra.pe_2);
        self.pe_3.set_neq(mra.pe_3);
        self.pe_4.set_neq(mra.pe_4);
        self.pe_5.set_neq(mra.pe_5);
        self.pe_6.set_neq(mra.pe_6);
        self.pe_7.set_neq(mra.pe_7);
        self.pe_8.set_neq(mra.pe_8);
        self.pe_9.set_neq(mra.pe_9);
        self.pe_text.set_neq(mra.pe_text.clone());
        self.pn_m.set_neq(mra.pn_m);
        self.pn_n.set_neq(mra.pn_n);
        self.pn_1.set_neq(mra.pn_1);
        self.pn_2.set_neq(mra.pn_2);
        self.pn_3.set_neq(mra.pn_3);
        self.pn_4.set_neq(mra.pn_4);
        self.pn_5.set_neq(mra.pn_5);
        self.pn_6.set_neq(mra.pn_6);
        self.pn_7.set_neq(mra.pn_7);
        self.pn_8.set_neq(mra.pn_8);
        self.pn_9.set_neq(mra.pn_9);
        self.pn_text.set_neq(mra.pn_text.clone());
        self.cr_na.set_neq(mra.cr_na);
        self.cr_m.set_neq(mra.cr_m);
        self.cr_n.set_neq(mra.cr_n);
        self.cr_1.set_neq(mra.cr_1);
        self.cr_2.set_neq(mra.cr_2);
        self.cr_3.set_neq(mra.cr_3);
        self.cr_4.set_neq(mra.cr_4);
        self.cr_5.set_neq(mra.cr_5);
        self.cr_6.set_neq(mra.cr_6);
        self.cr_7.set_neq(mra.cr_7);
        self.cr_8.set_neq(mra.cr_8);
        self.cr_9.set_neq(mra.cr_9);
        self.cr_text.set_neq(mra.cr_text.clone());
        self.ar_na.set_neq(mra.ar_na);
        self.ar_m.set_neq(mra.ar_m);
        self.ar_n.set_neq(mra.ar_n);
        self.ar_1.set_neq(mra.ar_1);
        self.ar_2.set_neq(mra.ar_2);
        self.ar_3.set_neq(mra.ar_3);
        self.ar_4.set_neq(mra.ar_4);
        self.ar_5.set_neq(mra.ar_5);
        self.ar_6.set_neq(mra.ar_6);
        self.ar_7.set_neq(mra.ar_7);
        self.ar_8.set_neq(mra.ar_8);
        self.ar_9.set_neq(mra.ar_9);
        self.ar_text.set_neq(mra.ar_text.clone());
        self.on_na.set_neq(mra.on_na);
        self.on_m.set_neq(mra.on_m);
        self.on_n.set_neq(mra.on_n);
        self.on_1.set_neq(mra.on_1);
        self.on_2.set_neq(mra.on_2);
        self.on_3.set_neq(mra.on_3);
        self.on_4.set_neq(mra.on_4);
        self.on_5.set_neq(mra.on_5);
        self.on_6.set_neq(mra.on_6);
        self.on_7.set_neq(mra.on_7);
        self.on_8.set_neq(mra.on_8);
        self.on_9.set_neq(mra.on_9);
        self.on_text.set_neq(mra.on_text.clone());
        self.lr_na.set_neq(mra.lr_na);
        self.lr_m.set_neq(mra.lr_m);
        self.lr_n.set_neq(mra.lr_n);
        self.lr_1.set_neq(mra.lr_1);
        self.lr_2.set_neq(mra.lr_2);
        self.lr_3.set_neq(mra.lr_3);
        self.lr_4.set_neq(mra.lr_4);
        self.lr_5.set_neq(mra.lr_5);
        self.lr_6.set_neq(mra.lr_6);
        self.lr_7.set_neq(mra.lr_7);
        self.lr_8.set_neq(mra.lr_8);
        self.lr_9.set_neq(mra.lr_9);
        self.lr_text.set_neq(mra.lr_text.clone());
        self.rr_na.set_neq(mra.rr_na);
        self.rr_m.set_neq(mra.rr_m);
        self.rr_n.set_neq(mra.rr_n);
        self.rr_1.set_neq(mra.rr_1);
        self.rr_2.set_neq(mra.rr_2);
        self.rr_3.set_neq(mra.rr_3);
        self.rr_4.set_neq(mra.rr_4);
        self.rr_5.set_neq(mra.rr_5);
        self.rr_6.set_neq(mra.rr_6);
        self.rr_7.set_neq(mra.rr_7);
        self.rr_8.set_neq(mra.rr_8);
        self.rr_9.set_neq(mra.rr_9);
        self.rr_text.set_neq(mra.rr_text.clone());
        self.nn_m.set_neq(mra.nn_m);
        self.nn_n.set_neq(mra.nn_n);
        self.nn_1.set_neq(mra.nn_1);
        self.nn_2.set_neq(mra.nn_2);
        self.nn_3.set_neq(mra.nn_3);
        self.nn_4.set_neq(mra.nn_4);
        self.nn_5.set_neq(mra.nn_5);
        self.nn_6.set_neq(mra.nn_6);
        self.nn_7.set_neq(mra.nn_7);
        self.nn_8.set_neq(mra.nn_8);
        self.nn_9.set_neq(mra.nn_9);
        self.nn_sub.set_neq(mra.nn_sub);
        self.nn_text.set_neq(mra.nn_text.clone());
        self.recal_all.set(true);
    }

    fn to_mra(&self) -> IpdMra {
        IpdMra {
            mra_id: self.mra_id.get(),
            hn: self.hn.get_cloned(),
            an: self.an.get_cloned(),
            auditor: self.auditor.get_cloned(),
            adm_date: self.adm_date.get(),
            dch_date: self.dch_date.get(),
            audit_date: date_8601(&self.audit_date.lock_ref()),
            audit_type: self.audit_type.get_cloned(),
            is_psychiatry: self.is_psychiatry.get(),
            is_not_sorted: self.is_not_sorted.get(),
            is_unknown: self.is_unknown.get(),
            overall: self.overall.get_cloned(),
            overall_text: self.overall_text.get_cloned(),
            sd_m: self.sd_m.get(),
            sd_n: self.sd_n.get(),
            sd_1: self.sd_1.get(),
            sd_2: self.sd_2.get(),
            sd_3: self.sd_3.get(),
            sd_4: self.sd_4.get(),
            sd_5: self.sd_5.get(),
            sd_6: self.sd_6.get(),
            sd_7: self.sd_7.get(),
            sd_8: self.sd_8.get(),
            sd_9: self.sd_9.get(),
            sd_text: self.sd_text.get_cloned(),
            so_m: self.so_m.get(),
            so_n: self.so_n.get(),
            so_1: self.so_1.get(),
            so_2: self.so_2.get(),
            so_3: self.so_3.get(),
            so_4: self.so_4.get(),
            so_5: self.so_5.get(),
            so_6: self.so_6.get(),
            so_7: self.so_7.get(),
            so_text: self.so_text.get_cloned(),
            ic_m: self.ic_m.get(),
            ic_n: self.ic_n.get(),
            ic_1: self.ic_1.get(),
            ic_2: self.ic_2.get(),
            ic_3: self.ic_3.get(),
            ic_4: self.ic_4.get(),
            ic_5: self.ic_5.get(),
            ic_6: self.ic_6.get(),
            ic_7: self.ic_7.get(),
            ic_8: self.ic_8.get(),
            ic_9: self.ic_9.get(),
            ic_text: self.ic_text.get_cloned(),
            hx_m: self.hx_m.get(),
            hx_n: self.hx_n.get(),
            hx_1: self.hx_1.get(),
            hx_2: self.hx_2.get(),
            hx_3: self.hx_3.get(),
            hx_4: self.hx_4.get(),
            hx_5: self.hx_5.get(),
            hx_6: self.hx_6.get(),
            hx_7: self.hx_7.get(),
            hx_8: self.hx_8.get(),
            hx_9: self.hx_9.get(),
            hx_text: self.hx_text.get_cloned(),
            pe_m: self.pe_m.get(),
            pe_n: self.pe_n.get(),
            pe_1: self.pe_1.get(),
            pe_2: self.pe_2.get(),
            pe_3: self.pe_3.get(),
            pe_4: self.pe_4.get(),
            pe_5: self.pe_5.get(),
            pe_6: self.pe_6.get(),
            pe_7: self.pe_7.get(),
            pe_8: self.pe_8.get(),
            pe_9: self.pe_9.get(),
            pe_text: self.pe_text.get_cloned(),
            pn_m: self.pn_m.get(),
            pn_n: self.pn_n.get(),
            pn_1: self.pn_1.get(),
            pn_2: self.pn_2.get(),
            pn_3: self.pn_3.get(),
            pn_4: self.pn_4.get(),
            pn_5: self.pn_5.get(),
            pn_6: self.pn_6.get(),
            pn_7: self.pn_7.get(),
            pn_8: self.pn_8.get(),
            pn_9: self.pn_9.get(),
            pn_text: self.pn_text.get_cloned(),
            cr_na: self.cr_na.get(),
            cr_m: self.cr_m.get(),
            cr_n: self.cr_n.get(),
            cr_1: self.cr_1.get(),
            cr_2: self.cr_2.get(),
            cr_3: self.cr_3.get(),
            cr_4: self.cr_4.get(),
            cr_5: self.cr_5.get(),
            cr_6: self.cr_6.get(),
            cr_7: self.cr_7.get(),
            cr_8: self.cr_8.get(),
            cr_9: self.cr_9.get(),
            cr_text: self.cr_text.get_cloned(),
            ar_na: self.ar_na.get(),
            ar_m: self.ar_m.get(),
            ar_n: self.ar_n.get(),
            ar_1: self.ar_1.get(),
            ar_2: self.ar_2.get(),
            ar_3: self.ar_3.get(),
            ar_4: self.ar_4.get(),
            ar_5: self.ar_5.get(),
            ar_6: self.ar_6.get(),
            ar_7: self.ar_7.get(),
            ar_8: self.ar_8.get(),
            ar_9: self.ar_9.get(),
            ar_text: self.ar_text.get_cloned(),
            on_na: self.on_na.get(),
            on_m: self.on_m.get(),
            on_n: self.on_n.get(),
            on_1: self.on_1.get(),
            on_2: self.on_2.get(),
            on_3: self.on_3.get(),
            on_4: self.on_4.get(),
            on_5: self.on_5.get(),
            on_6: self.on_6.get(),
            on_7: self.on_7.get(),
            on_8: self.on_8.get(),
            on_9: self.on_9.get(),
            on_text: self.on_text.get_cloned(),
            lr_na: self.lr_na.get(),
            lr_m: self.lr_m.get(),
            lr_n: self.lr_n.get(),
            lr_1: self.lr_1.get(),
            lr_2: self.lr_2.get(),
            lr_3: self.lr_3.get(),
            lr_4: self.lr_4.get(),
            lr_5: self.lr_5.get(),
            lr_6: self.lr_6.get(),
            lr_7: self.lr_7.get(),
            lr_8: self.lr_8.get(),
            lr_9: self.lr_9.get(),
            lr_text: self.lr_text.get_cloned(),
            rr_na: self.rr_na.get(),
            rr_m: self.rr_m.get(),
            rr_n: self.rr_n.get(),
            rr_1: self.rr_1.get(),
            rr_2: self.rr_2.get(),
            rr_3: self.rr_3.get(),
            rr_4: self.rr_4.get(),
            rr_5: self.rr_5.get(),
            rr_6: self.rr_6.get(),
            rr_7: self.rr_7.get(),
            rr_8: self.rr_8.get(),
            rr_9: self.rr_9.get(),
            rr_text: self.rr_text.get_cloned(),
            nn_m: self.nn_m.get(),
            nn_n: self.nn_n.get(),
            nn_1: self.nn_1.get(),
            nn_2: self.nn_2.get(),
            nn_3: self.nn_3.get(),
            nn_4: self.nn_4.get(),
            nn_5: self.nn_5.get(),
            nn_6: self.nn_6.get(),
            nn_7: self.nn_7.get(),
            nn_8: self.nn_8.get(),
            nn_9: self.nn_9.get(),
            nn_sub: self.nn_sub.get(),
            nn_text: self.nn_text.get_cloned(),
        }
    }

    fn save(mra: Rc<Self>, list_loaded: Mutable<bool>, app: Rc<App>) {
        let saver = mra.to_mra();

        app.async_load(
            true,
            clone!(app => async move {
                // POST `EndPoint::IpdMra`
                // PUT `EndPoint::IpdMra`
                match saver.call_api_save(app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            mra.ipd_mra_selected.set(None);
                            mra.changed.set(false);
                            list_loaded.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    fn delete(mra: Rc<Self>, list_loaded: Mutable<bool>, app: Rc<App>) {
        if let Some(mra_id) = zero_none(mra.mra_id.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    if app.confirm("ยืนยันลบรายการ").await {
                        let params = MraParams { mra_id: Some(mra_id), ..Default::default()};
                        // DELETE `EndPoint::IpdMra`
                        match IpdMra::call_api_delete(&params, app.state()).await {
                            Ok(response) => {
                                app.alert_execute_response(&response, async move {
                                    mra.ipd_mra_selected.set(None);
                                    mra.changed.set(false);
                                    list_loaded.set(false);
                                }).await;
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

    // Some(true) is 1, Some(false) is 0, None is NA
    fn set_all(&self, row: &MraRow, v: Option<bool>) {
        match row {
            MraRow::Sd(_) => {
                self.sd_1.set(v);
                self.sd_2.set(v);
                self.sd_3.set(v);
                self.sd_4.set(v);
                self.sd_5.set(v);
                self.sd_6.set(v);
                self.sd_7.set(v);
                self.sd_8.set(v);
                self.sd_9.set(v);
            }
            MraRow::So(_) => {
                self.so_1.set(v);
                self.so_2.set(v);
                self.so_3.set(v);
                self.so_4.set(v);
                self.so_5.set(v);
                self.so_6.set(v);
                self.so_7.set(v);
            }
            MraRow::Ic(_) => {
                self.ic_1.set(v);
                self.ic_2.set(v);
                self.ic_3.set(v);
                self.ic_4.set(v);
                self.ic_5.set(v);
                self.ic_6.set(v);
                self.ic_7.set(v);
                self.ic_8.set(v);
                self.ic_9.set(v);
            }
            MraRow::Hx(_) => {
                self.hx_1.set(v);
                self.hx_2.set(v);
                self.hx_3.set(v);
                self.hx_4.set(v);
                self.hx_5.set(v);
                self.hx_6.set(v);
                self.hx_7.set(v);
                self.hx_8.set(v);
                self.hx_9.set(v);
            }
            MraRow::Pe(_) => {
                self.pe_1.set(v);
                self.pe_2.set(v);
                self.pe_3.set(v);
                self.pe_4.set(v);
                self.pe_5.set(v);
                self.pe_6.set(v);
                self.pe_7.set(v);
                self.pe_8.set(v);
                self.pe_9.set(v);
            }
            MraRow::Pn(_) => {
                self.pn_1.set(v);
                self.pn_2.set(v);
                self.pn_3.set(v);
                self.pn_4.set(v);
                self.pn_5.set(v);
                self.pn_6.set(v);
                self.pn_7.set(v);
                self.pn_8.set(v);
                self.pn_9.set(v);
            }
            MraRow::Cr(_) => {
                self.cr_1.set(v);
                self.cr_2.set(v);
                self.cr_3.set(v);
                self.cr_4.set(v);
                self.cr_5.set(v);
                self.cr_6.set(v);
                self.cr_7.set(v);
                self.cr_8.set(v);
                self.cr_9.set(v);
            }
            MraRow::Ar(_) => {
                self.ar_1.set(v);
                self.ar_2.set(v);
                self.ar_3.set(v);
                self.ar_4.set(v);
                self.ar_5.set(v);
                self.ar_6.set(v);
                self.ar_7.set(v);
                self.ar_8.set(v);
                self.ar_9.set(v);
            }
            MraRow::On(_) => {
                self.on_1.set(v);
                self.on_2.set(v);
                self.on_3.set(v);
                self.on_4.set(v);
                self.on_5.set(v);
                self.on_6.set(v);
                self.on_7.set(v);
                self.on_8.set(v);
                self.on_9.set(v);
            }
            MraRow::Lr(_) => {
                self.lr_1.set(v);
                self.lr_2.set(v);
                self.lr_3.set(v);
                self.lr_4.set(v);
                self.lr_5.set(v);
                self.lr_6.set(v);
                self.lr_7.set(v);
                self.lr_8.set(v);
                self.lr_9.set(v);
            }
            MraRow::Rr(_) => {
                self.rr_1.set(v);
                self.rr_2.set(v);
                self.rr_3.set(v);
                self.rr_4.set(v);
                self.rr_5.set(v);
                self.rr_6.set(v);
                self.rr_7.set(v);
                self.rr_8.set(v);
                self.rr_9.set(v);
            }
            MraRow::Nn(_) => {
                self.nn_1.set(v);
                self.nn_2.set(v);
                self.nn_3.set(v);
                self.nn_4.set(v);
                self.nn_5.set(v);
                self.nn_6.set(v);
                self.nn_7.set(v);
                self.nn_8.set(v);
                self.nn_9.set(v);
                self.nn_sub.set(v.map(|_i| false));
            }
        }
    }

    fn reset_mn(&self, row: &MraRow) {
        match row {
            MraRow::Sd(_) => {
                self.sd_m.set(false);
                self.sd_n.set(false);
            }
            MraRow::So(_) => {
                self.so_m.set(false);
                self.so_n.set(false);
            }
            MraRow::Ic(_) => {
                self.ic_m.set(false);
                self.ic_n.set(false);
            }
            MraRow::Hx(_) => {
                self.hx_m.set(false);
                self.hx_n.set(false);
            }
            MraRow::Pe(_) => {
                self.pe_m.set(false);
                self.pe_n.set(false);
            }
            MraRow::Pn(_) => {
                self.pn_m.set(false);
                self.pn_n.set(false);
            }
            MraRow::Cr(_) => {
                self.cr_m.set(false);
                self.cr_n.set(false);
            }
            MraRow::Ar(_) => {
                self.ar_m.set(false);
                self.ar_n.set(false);
            }
            MraRow::On(_) => {
                self.on_m.set(false);
                self.on_n.set(false);
            }
            MraRow::Lr(_) => {
                self.lr_m.set(false);
                self.lr_n.set(false);
            }
            MraRow::Rr(_) => {
                self.rr_m.set(false);
                self.rr_n.set(false);
            }
            MraRow::Nn(_) => {
                self.nn_m.set(false);
                self.nn_n.set(false);
            }
        }
    }

    fn sd_full(&self) -> usize {
        [
            self.sd_1.get(),
            self.sd_2.get(),
            self.sd_3.get(),
            self.sd_4.get(),
            self.sd_5.get(),
            self.sd_6.get(),
            self.sd_7.get(),
            self.sd_8.get(),
            self.sd_9.get(),
        ]
        .iter()
        .filter_map(|x| *x)
        .count()
    }
    fn so_full(&self) -> usize {
        [self.so_1.get(), self.so_2.get(), self.so_3.get(), self.so_4.get(), self.so_5.get(), self.so_6.get(), self.so_7.get()]
            .iter()
            .filter_map(|x| *x)
            .count()
    }
    fn ic_full(&self) -> usize {
        [
            self.ic_1.get(),
            self.ic_2.get(),
            self.ic_3.get(),
            self.ic_4.get(),
            self.ic_5.get(),
            self.ic_6.get(),
            self.ic_7.get(),
            self.ic_8.get(),
            self.ic_9.get(),
        ]
        .iter()
        .filter_map(|x| *x)
        .count()
    }
    fn hx_full(&self) -> usize {
        [
            self.hx_1.get(),
            self.hx_2.get(),
            self.hx_3.get(),
            self.hx_4.get(),
            self.hx_5.get(),
            self.hx_6.get(),
            self.hx_7.get(),
            self.hx_8.get(),
            self.hx_9.get(),
        ]
        .iter()
        .filter_map(|x| *x)
        .count()
    }
    fn pe_full(&self) -> usize {
        [
            self.pe_1.get(),
            self.pe_2.get(),
            self.pe_3.get(),
            self.pe_4.get(),
            self.pe_5.get(),
            self.pe_6.get(),
            self.pe_7.get(),
            self.pe_8.get(),
            self.pe_9.get(),
        ]
        .iter()
        .filter_map(|x| *x)
        .count()
    }
    fn pn_full(&self) -> usize {
        [
            self.pn_1.get(),
            self.pn_2.get(),
            self.pn_3.get(),
            self.pn_4.get(),
            self.pn_5.get(),
            self.pn_6.get(),
            self.pn_7.get(),
            self.pn_8.get(),
            self.pn_9.get(),
        ]
        .iter()
        .filter_map(|x| *x)
        .count()
    }
    fn cr_full(&self) -> usize {
        if self.cr_na.get() {
            0
        } else {
            [
                self.cr_1.get(),
                self.cr_2.get(),
                self.cr_3.get(),
                self.cr_4.get(),
                self.cr_5.get(),
                self.cr_6.get(),
                self.cr_7.get(),
                self.cr_8.get(),
                self.cr_9.get(),
            ]
            .iter()
            .filter_map(|x| *x)
            .count()
        }
    }
    fn ar_full(&self) -> usize {
        if self.ar_na.get() {
            0
        } else {
            [
                self.ar_1.get(),
                self.ar_2.get(),
                self.ar_3.get(),
                self.ar_4.get(),
                self.ar_5.get(),
                self.ar_6.get(),
                self.ar_7.get(),
                self.ar_8.get(),
                self.ar_9.get(),
            ]
            .iter()
            .filter_map(|x| *x)
            .count()
        }
    }
    fn on_full(&self) -> usize {
        if self.on_na.get() {
            0
        } else {
            [
                self.on_1.get(),
                self.on_2.get(),
                self.on_3.get(),
                self.on_4.get(),
                self.on_5.get(),
                self.on_6.get(),
                self.on_7.get(),
                self.on_8.get(),
                self.on_9.get(),
            ]
            .iter()
            .filter_map(|x| *x)
            .count()
        }
    }
    fn lr_full(&self) -> usize {
        if self.lr_na.get() {
            0
        } else {
            [
                self.lr_1.get(),
                self.lr_2.get(),
                self.lr_3.get(),
                self.lr_4.get(),
                self.lr_5.get(),
                self.lr_6.get(),
                self.lr_7.get(),
                self.lr_8.get(),
                self.lr_9.get(),
            ]
            .iter()
            .filter_map(|x| *x)
            .count()
        }
    }
    fn rr_full(&self) -> usize {
        if self.rr_na.get() {
            0
        } else {
            [
                self.rr_1.get(),
                self.rr_2.get(),
                self.rr_3.get(),
                self.rr_4.get(),
                self.rr_5.get(),
                self.rr_6.get(),
                self.rr_7.get(),
                self.rr_8.get(),
                self.rr_9.get(),
            ]
            .iter()
            .filter_map(|x| *x)
            .count()
        }
    }
    fn nn_full(&self) -> usize {
        [
            self.nn_1.get(),
            self.nn_2.get(),
            self.nn_3.get(),
            self.nn_4.get(),
            self.nn_5.get(),
            self.nn_6.get(),
            self.nn_7.get(),
            self.nn_8.get(),
            self.nn_9.get(),
        ]
        .iter()
        .filter_map(|x| *x)
        .count()
    }

    fn sd_score(&self) -> usize {
        if self.sd_m.get() || self.sd_n.get() {
            0
        } else {
            [
                self.sd_1.get(),
                self.sd_2.get(),
                self.sd_3.get(),
                self.sd_4.get(),
                self.sd_5.get(),
                self.sd_6.get(),
                self.sd_7.get(),
                self.sd_8.get(),
                self.sd_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn so_score(&self) -> usize {
        if self.so_m.get() || self.so_n.get() {
            0
        } else {
            [self.so_1.get(), self.so_2.get(), self.so_3.get(), self.so_4.get(), self.so_5.get(), self.so_6.get(), self.so_7.get()]
                .iter()
                .filter(|x| **x == Some(true))
                .count()
        }
    }
    fn ic_score(&self) -> usize {
        if self.ic_m.get() || self.ic_n.get() {
            0
        } else {
            [
                self.ic_1.get(),
                self.ic_2.get(),
                self.ic_3.get(),
                self.ic_4.get(),
                self.ic_5.get(),
                self.ic_6.get(),
                self.ic_7.get(),
                self.ic_8.get(),
                self.ic_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn hx_score(&self) -> usize {
        if self.hx_m.get() || self.hx_n.get() {
            0
        } else {
            [
                self.hx_1.get(),
                self.hx_2.get(),
                self.hx_3.get(),
                self.hx_4.get(),
                self.hx_5.get(),
                self.hx_6.get(),
                self.hx_7.get(),
                self.hx_8.get(),
                self.hx_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn pe_score(&self) -> usize {
        if self.pe_m.get() || self.pe_n.get() {
            0
        } else {
            [
                self.pe_1.get(),
                self.pe_2.get(),
                self.pe_3.get(),
                self.pe_4.get(),
                self.pe_5.get(),
                self.pe_6.get(),
                self.pe_7.get(),
                self.pe_8.get(),
                self.pe_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn pn_score(&self) -> usize {
        if self.pn_m.get() || self.pn_n.get() {
            0
        } else {
            [
                self.pn_1.get(),
                self.pn_2.get(),
                self.pn_3.get(),
                self.pn_4.get(),
                self.pn_5.get(),
                self.pn_6.get(),
                self.pn_7.get(),
                self.pn_8.get(),
                self.pn_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn cr_score(&self) -> usize {
        if self.cr_na.get() || self.cr_m.get() || self.cr_n.get() {
            0
        } else {
            [
                self.cr_1.get(),
                self.cr_2.get(),
                self.cr_3.get(),
                self.cr_4.get(),
                self.cr_5.get(),
                self.cr_6.get(),
                self.cr_7.get(),
                self.cr_8.get(),
                self.cr_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn ar_score(&self) -> usize {
        if self.ar_na.get() || self.ar_m.get() || self.ar_n.get() {
            0
        } else {
            [
                self.ar_1.get(),
                self.ar_2.get(),
                self.ar_3.get(),
                self.ar_4.get(),
                self.ar_5.get(),
                self.ar_6.get(),
                self.ar_7.get(),
                self.ar_8.get(),
                self.ar_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn on_score(&self) -> usize {
        if self.on_na.get() || self.on_m.get() || self.on_n.get() {
            0
        } else {
            [
                self.on_1.get(),
                self.on_2.get(),
                self.on_3.get(),
                self.on_4.get(),
                self.on_5.get(),
                self.on_6.get(),
                self.on_7.get(),
                self.on_8.get(),
                self.on_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn lr_score(&self) -> usize {
        if self.lr_na.get() || self.lr_m.get() || self.lr_n.get() {
            0
        } else {
            [
                self.lr_1.get(),
                self.lr_2.get(),
                self.lr_3.get(),
                self.lr_4.get(),
                self.lr_5.get(),
                self.lr_6.get(),
                self.lr_7.get(),
                self.lr_8.get(),
                self.lr_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn rr_score(&self) -> usize {
        if self.rr_na.get() || self.rr_m.get() || self.rr_n.get() {
            0
        } else {
            [
                self.rr_1.get(),
                self.rr_2.get(),
                self.rr_3.get(),
                self.rr_4.get(),
                self.rr_5.get(),
                self.rr_6.get(),
                self.rr_7.get(),
                self.rr_8.get(),
                self.rr_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
        }
    }
    fn nn_score(&self) -> usize {
        if self.nn_m.get() || self.nn_n.get() {
            0
        } else {
            [
                self.nn_1.get(),
                self.nn_2.get(),
                self.nn_3.get(),
                self.nn_4.get(),
                self.nn_5.get(),
                self.nn_6.get(),
                self.nn_7.get(),
                self.nn_8.get(),
                self.nn_9.get(),
            ]
            .iter()
            .filter(|x| **x == Some(true))
            .count()
                - (if self.nn_sub.get() == Some(true) { 1 } else { 0 })
        }
    }

    fn full(&self) -> usize {
        self.sd_full.get()
            + self.so_full.get()
            + self.ic_full.get()
            + self.hx_full.get()
            + self.pe_full.get()
            + self.pn_full.get()
            + self.cr_full.get()
            + self.ar_full.get()
            + self.on_full.get()
            + self.lr_full.get()
            + self.rr_full.get()
            + self.nn_full.get()
    }
    fn score(&self) -> usize {
        self.sd_score.get()
            + self.so_score.get()
            + self.ic_score.get()
            + self.hx_score.get()
            + self.pe_score.get()
            + self.pn_score.get()
            + self.cr_score.get()
            + self.ar_score.get()
            + self.on_score.get()
            + self.lr_score.get()
            + self.rr_score.get()
            + self.nn_score.get()
    }

    fn recal_all(&self) {
        let row_cr = MraRow::Cr(MraCol::La);
        let row_ar = MraRow::Ar(MraCol::La);
        let row_on = MraRow::On(MraCol::La);
        let row_lr = MraRow::Lr(MraCol::La);
        let row_rr = MraRow::Rr(MraCol::La);
        if self.cr_na.get() {
            self.reset_mn(&row_cr);
            self.set_all(&row_cr, None);
        } else if self.cr_m.get() || self.cr_n.get() {
            self.set_all(&row_cr, Some(false));
        }
        if self.ar_na.get() {
            self.reset_mn(&row_ar);
            self.set_all(&row_ar, None);
        } else if self.ar_m.get() || self.ar_n.get() {
            self.set_all(&row_ar, Some(false));
        }
        if self.on_na.get() {
            self.reset_mn(&row_on);
            self.set_all(&row_on, None);
        } else if self.on_m.get() || self.on_n.get() {
            self.set_all(&row_on, Some(false));
        }
        if self.lr_na.get() {
            self.reset_mn(&row_lr);
            self.set_all(&row_lr, None);
        } else if self.lr_m.get() || self.lr_n.get() {
            self.set_all(&row_lr, Some(false));
        }
        if self.rr_na.get() {
            self.reset_mn(&row_rr);
            self.set_all(&row_rr, None);
        } else if self.rr_m.get() || self.rr_n.get() {
            self.set_all(&row_rr, Some(false));
        }

        if self.sd_m.get() || self.sd_n.get() {
            self.set_all(&MraRow::Sd(MraCol::La), Some(false));
        }
        if self.so_m.get() || self.so_n.get() {
            self.set_all(&MraRow::So(MraCol::La), Some(false));
        }
        if self.ic_m.get() || self.ic_n.get() {
            self.set_all(&MraRow::Ic(MraCol::La), Some(false));
        }
        if self.hx_m.get() || self.hx_n.get() {
            self.set_all(&MraRow::Hx(MraCol::La), Some(false));
        }
        if self.pe_m.get() || self.pe_n.get() {
            self.set_all(&MraRow::Pe(MraCol::La), Some(false));
        }
        if self.pn_m.get() || self.pn_n.get() {
            self.set_all(&MraRow::Pn(MraCol::La), Some(false));
        }
        if self.nn_m.get() || self.nn_n.get() {
            self.set_all(&MraRow::Nn(MraCol::La), Some(false));
        }

        self.sd_recal.set(true);
        self.so_recal.set(true);
        self.ic_recal.set(true);
        self.hx_recal.set(true);
        self.pe_recal.set(true);
        self.pn_recal.set(true);
        self.cr_recal.set(true);
        self.ar_recal.set(true);
        self.on_recal.set(true);
        self.lr_recal.set(true);
        self.rr_recal.set(true);
        self.nn_recal.set(true);
    }

    fn render(mra: Rc<Self>, list_loaded: Mutable<bool>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(mra.ipd_mra_selected.signal_cloned().for_each(clone!(mra => move |opt| {
                if let Some(mra_selected) = opt {
                    mra.set_mra(mra_selected);
                } else if let Some(mra_new) = mra.new_mra.get_cloned() {
                    mra.set_mra(mra_new);
                }
                mra.changed.set(false);
                async {}
            })))
            .future(mra.recal_all.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.recal_all();
                    mra.recal_all.set(false);
                }
                async {}
            })))
            .future(mra.recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.full.set(mra.full());
                    mra.score.set(mra.score());
                    mra.recal.set(false);
                }
                async {}
            })))
            .future(mra.sd_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.sd_full.set_neq(mra.sd_full());
                    mra.sd_score.set_neq(mra.sd_score());
                    mra.sd_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.so_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.so_full.set_neq(mra.so_full());
                    mra.so_score.set_neq(mra.so_score());
                    mra.so_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.ic_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.ic_full.set_neq(mra.ic_full());
                    mra.ic_score.set_neq(mra.ic_score());
                    mra.ic_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.hx_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.hx_full.set_neq(mra.hx_full());
                    mra.hx_score.set_neq(mra.hx_score());
                    mra.hx_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.pe_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.pe_full.set_neq(mra.pe_full());
                    mra.pe_score.set_neq(mra.pe_score());
                    mra.pe_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.pn_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.pn_full.set_neq(mra.pn_full());
                    mra.pn_score.set_neq(mra.pn_score());
                    mra.pn_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.cr_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.cr_full.set_neq(mra.cr_full());
                    mra.cr_score.set_neq(mra.cr_score());
                    mra.cr_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.ar_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.ar_full.set_neq(mra.ar_full());
                    mra.ar_score.set_neq(mra.ar_score());
                    mra.ar_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.on_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.on_full.set_neq(mra.on_full());
                    mra.on_score.set_neq(mra.on_score());
                    mra.on_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.lr_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.lr_full.set_neq(mra.lr_full());
                    mra.lr_score.set_neq(mra.lr_score());
                    mra.lr_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.rr_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.rr_full.set_neq(mra.rr_full());
                    mra.rr_score.set_neq(mra.rr_score());
                    mra.rr_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .future(mra.nn_recal.signal().for_each(clone!(mra => move |recal| {
                if recal {
                    mra.nn_full.set_neq(mra.nn_full());
                    mra.nn_score.set_neq(mra.nn_score());
                    mra.nn_recal.set(false);
                    mra.recal.set(true);
                }
                async {}
            })))
            .style("width","calc(100% - 5px)")
            .children([
                html!("div", {
                    .class("d-flex")
                    .children([
                        html!("div", {
                            .class(class::FLEX_COL)
                            .class("me-2")
                            .style("width","95px")
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUEO)
                                    .class("mt-2")
                                    .text("คำชี้แจง")
                                    .event(clone!(mra => move |_:events::Click| {
                                        let doc = if mra.is_psychiatry.get() {ipd_mra_psy::IPD_MRA_PSY_INTRO.clone()} else {ipd_mra::IPD_MRA_INTRO.clone()};
                                        mra.document.set(Some(doc));
                                        mra.selected_row.set_neq(None);
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUEO)
                                    .class("mt-2")
                                    .text("การบันทึกคะแนน")
                                    .event(clone!(mra => move |_:events::Click| {
                                        let doc = if mra.is_psychiatry.get() {ipd_mra_psy::IPD_MRA_PSY_SCORE.clone()} else {ipd_mra::IPD_MRA_SCORE.clone()};
                                        mra.document.set(Some(doc));
                                        mra.selected_row.set_neq(None);
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::BOX_T)
                            .class("overflow-auto")
                            .style("height","120px")
                            .style("width","calc(100% - 100px)")
                            .child_signal(mra.document.signal_cloned().map(|opt| opt.as_ref().map(|doc| doc.dom())))
                        }),
                    ])
                }),
                // FORM
                html!("div", {
                    .style("justify-items","center")
                    .children([
                        // patient info
                        html!("div", {
                            .class(class::BOLD_C_PB2)
                            .children([
                                text("HN: "),
                                html!("span", {.class("text-primary").text_signal(mra.hn.signal_cloned().map(|opt| opt.unwrap_or_default()))}),
                                text(" AN: "),
                                html!("span", {.class("text-primary").text_signal(mra.an.signal_cloned())}),
                                text(" Admit: "),
                                html!("span", {.class("text-primary").text_signal(mra.adm_date.signal_cloned().map(|opt| date_th_opt(&opt)))}),
                                text(" Discharge: "),
                                html!("span", {.class("text-primary").text_signal(mra.dch_date.signal_cloned().map(|opt| date_th_opt(&opt)))}),
                            ])
                            .child_signal(mra.audit_type.signal_cloned().map(clone!(mra => move |audit_type| {
                                (audit_type.as_str() == "I").then(|| {
                                    html!("span", {
                                        .children([
                                            text(" Auditor: "),
                                            html!("span", {
                                                .class("text-primary")
                                                .text_signal(mra.auditor.signal_cloned().map(|opt| opt.to_owned().unwrap_or_default()))
                                                .text(" (")
                                                .text_signal(mra.audit_date.signal_cloned().map(|s| date_str_th(&s)))
                                                .text(")")
                                            }),
                                        ])
                                    })
                                })
                            })))
                        }),
                        // optional input
                        html!("div", {
                            .class(class::ROW_AUTO_SM_G2_JCT)
                            .children([
                                html!("div", {
                                    .class("col-auto")
                                    .child(html!("span", {
                                        .class(class::FORM_CHK_SW_PT1)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .class("form-check-input")
                                                .attr("type", "checkbox")
                                                .attr("role","switch")
                                                .attr("id", "is-psychiatry-toggle")
                                                .apply(mixins::checkbox_bool(mra.is_psychiatry.clone(), mra.changed.clone()))
                                                .future(mra.is_psychiatry.signal().dedupe().for_each(clone!(mra => move |is_psychiatry| {
                                                    let doc = if is_psychiatry {ipd_mra_psy::IPD_MRA_PSY_INTRO.clone()} else {ipd_mra::IPD_MRA_INTRO.clone()};
                                                    mra.document.set(Some(doc));
                                                    mra.selected_row.set_neq(None);
                                                    async {}
                                                })))
                                            }),
                                            doms::label_check_for("is-psychiatry-toggle","จิตเวช"),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-auto")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", mra.audit_type.signal_cloned().map(move |t| t == "I"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("Internal")
                                                .event(clone!(mra => move |_: events::Click| {
                                                    mra.audit_type.set(String::from("I"));
                                                    mra.auditor.set(mra.parent_auditor.get_cloned());
                                                    mra.audit_date.set(js_now().date().to_string());
                                                    mra.changed.set(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", mra.audit_type.signal_cloned().map(move |t| t == "E"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("External")
                                                .event(clone!(mra => move |_: events::Click| {
                                                    mra.audit_type.set(String::from("E"));
                                                    mra.auditor.set(None);
                                                    mra.audit_date.set(String::new());
                                                    mra.changed.set(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                            .child_signal(mra.audit_type.signal_cloned().map(clone!(mra => move |audit_type| {
                                (audit_type.as_str() == "E").then(|| {
                                    html!("div", {
                                        .class("col-auto")
                                        .child(html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .children([
                                                doms::span_group_text("Auditor"),
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .attr("maxlength", "250")
                                                    .class(class::FORM_CTRL_SM)
                                                    .attr("placeholder","ชื่อ-สกุล")
                                                    .attr("aria-label","Auditor")
                                                    .apply(mixins::opt_string_value(mra.auditor.clone(), mra.changed.clone()))
                                                }),
                                            ])
                                        }))
                                    })
                                })
                            })))
                            .child_signal(mra.audit_type.signal_cloned().map(clone!(mra => move |audit_type| {
                                (audit_type.as_str() == "E").then(|| {
                                    html!("div", {
                                        .class("col-auto")
                                        .child(html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .children([
                                                doms::span_group_text("Audit Date"),
                                                doms::date_picker(
                                                    mra.audit_date.clone(),
                                                    mra.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("aria-label","Audit Date"),
                                                    |s| s, always(None),
                                                ),
                                                // html!("input" => HtmlInputElement, {
                                                //     .attr("type", "date")
                                                //     .class(class::FORM_CTRL_SM)
                                                //     .attr("aria-label","Audit Date")
                                                //     .prop_signal("value", mra.audit_date.signal_cloned().map(|opt| opt.map(|d| d.to_string()).unwrap_or_default()))
                                                //     .with_node!(element => {
                                                //         .event(clone!(mra => move |_:events::Change| {
                                                //             let value = date_8601(&element.value());
                                                //             if mra.audit_date.get() != value {
                                                //                 mra.audit_date.set(value);
                                                //                 mra.changed.set(true);
                                                //             }
                                                //         }))
                                                //     })
                                                // }),
                                            ])
                                        }))
                                    })
                                })
                            })))
                        }),
                        // score table
                        html!("div", {
                            .children([
                                html!("div", {
                                    .class(class::FLEX_BOLD_C)
                                    .style("height","40px").style("line-height","40px")
                                    .children([
                                        html!("div",{.class("border").style("min-width","120px").text("Content")}),
                                        html!("div",{.class("border").style("min-width","40px").text("NA")}),
                                        html!("div",{.class("border").style("min-width","40px").text("M")}),
                                        html!("div",{.class("border").style("min-width","40px").text("N")}),
                                        html!("div",{.class("border").style("min-width","40px").text("1")}),
                                        html!("div",{.class("border").style("min-width","40px").text("2")}),
                                        html!("div",{.class("border").style("min-width","40px").text("3")}),
                                        html!("div",{.class("border").style("min-width","40px").text("4")}),
                                        html!("div",{.class("border").style("min-width","40px").text("5")}),
                                        html!("div",{.class("border").style("min-width","40px").text("6")}),
                                        html!("div",{.class("border").style("min-width","40px").text("7")}),
                                        html!("div",{.class("border").style("min-width","40px").text("8")}),
                                        html!("div",{.class("border").style("min-width","40px").text("9")}),
                                        html!("div",{.class("border").style("min-width","40px").text("หัก")}),
                                        html!("div",{.class("border").style("min-width","40px").text("รวม")}),
                                        html!("div",{.class("border").style("min-width","200px").text("หมายเหตุ")}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Sd(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Sd(MraCol::M), mra.clone(), mra.sd_m.clone(), None, Some(mra.sd_n.clone()), mra.sd_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Sd(MraCol::N), mra.clone(), mra.sd_n.clone(), None, Some(mra.sd_m.clone()), mra.sd_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P1), mra.clone(), mra.sd_1.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P2), mra.clone(), mra.sd_2.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P3), mra.clone(), mra.sd_3.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P4), mra.clone(), mra.sd_4.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P5), mra.clone(), mra.sd_5.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P6), mra.clone(), mra.sd_6.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P7), mra.clone(), mra.sd_7.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P8), mra.clone(), mra.sd_8.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Sd(MraCol::P9), mra.clone(), mra.sd_9.clone(), None, Some((mra.sd_m.clone(), mra.sd_n.clone())), mra.sd_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.sd_score.signal_cloned().map(|u| Some(value_box(u, Mutable::new(false)))))}),
                                        html!("div",{.child(text_input(mra.sd_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::So(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::So(MraCol::M), mra.clone(), mra.so_m.clone(), None, Some(mra.so_n.clone()), mra.so_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::So(MraCol::N), mra.clone(), mra.so_n.clone(), None, Some(mra.so_m.clone()), mra.so_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::So(MraCol::P1), mra.clone(), mra.so_1.clone(), None, Some((mra.so_m.clone(), mra.so_n.clone())), mra.so_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::So(MraCol::P2), mra.clone(), mra.so_2.clone(), None, Some((mra.so_m.clone(), mra.so_n.clone())), mra.so_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::So(MraCol::P3), mra.clone(), mra.so_3.clone(), None, Some((mra.so_m.clone(), mra.so_n.clone())), mra.so_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::So(MraCol::P4), mra.clone(), mra.so_4.clone(), None, Some((mra.so_m.clone(), mra.so_n.clone())), mra.so_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::So(MraCol::P5), mra.clone(), mra.so_5.clone(), None, Some((mra.so_m.clone(), mra.so_n.clone())), mra.so_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::So(MraCol::P6), mra.clone(), mra.so_6.clone(), None, Some((mra.so_m.clone(), mra.so_n.clone())), mra.so_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::So(MraCol::P7), mra.clone(), mra.so_7.clone(), None, Some((mra.so_m.clone(), mra.so_n.clone())), mra.so_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.so_score.signal_cloned().map(|u| Some(value_box(u, Mutable::new(false)))))}),
                                        html!("div",{.child(text_input(mra.so_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Ic(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Ic(MraCol::M), mra.clone(), mra.ic_m.clone(), None, Some(mra.ic_n.clone()), mra.ic_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Ic(MraCol::N), mra.clone(), mra.ic_n.clone(), None, Some(mra.ic_m.clone()), mra.ic_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P1), mra.clone(), mra.ic_1.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P2), mra.clone(), mra.ic_2.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P3), mra.clone(), mra.ic_3.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P4), mra.clone(), mra.ic_4.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P5), mra.clone(), mra.ic_5.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P6), mra.clone(), mra.ic_6.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P7), mra.clone(), mra.ic_7.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P8), mra.clone(), mra.ic_8.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ic(MraCol::P9), mra.clone(), mra.ic_9.clone(), None, Some((mra.ic_m.clone(), mra.ic_n.clone())), mra.ic_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.ic_score.signal_cloned().map(|u| Some(value_box(u, Mutable::new(false)))))}),
                                        html!("div",{.child(text_input(mra.ic_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Hx(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Hx(MraCol::M), mra.clone(), mra.hx_m.clone(), None, Some(mra.hx_n.clone()), mra.hx_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Hx(MraCol::N), mra.clone(), mra.hx_n.clone(), None, Some(mra.hx_m.clone()), mra.hx_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P1), mra.clone(), mra.hx_1.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P2), mra.clone(), mra.hx_2.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P3), mra.clone(), mra.hx_3.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P4), mra.clone(), mra.hx_4.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P5), mra.clone(), mra.hx_5.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P6), mra.clone(), mra.hx_6.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P7), mra.clone(), mra.hx_7.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P8), mra.clone(), mra.hx_8.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Hx(MraCol::P9), mra.clone(), mra.hx_9.clone(), None, Some((mra.hx_m.clone(), mra.hx_n.clone())), mra.hx_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.hx_score.signal_cloned().map(|u| Some(value_box(u, Mutable::new(false)))))}),
                                        html!("div",{.child(text_input(mra.hx_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Pe(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Pe(MraCol::M), mra.clone(), mra.pe_m.clone(), None, Some(mra.pe_n.clone()), mra.pe_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Pe(MraCol::N), mra.clone(), mra.pe_n.clone(), None, Some(mra.pe_m.clone()), mra.pe_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P1), mra.clone(), mra.pe_1.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P2), mra.clone(), mra.pe_2.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P3), mra.clone(), mra.pe_3.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P4), mra.clone(), mra.pe_4.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P5), mra.clone(), mra.pe_5.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P6), mra.clone(), mra.pe_6.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P7), mra.clone(), mra.pe_7.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P8), mra.clone(), mra.pe_8.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pe(MraCol::P9), mra.clone(), mra.pe_9.clone(), None, Some((mra.pe_m.clone(), mra.pe_n.clone())), mra.pe_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.pe_score.signal_cloned().map(|u| Some(value_box(u, Mutable::new(false)))))}),
                                        html!("div",{.child(text_input(mra.pe_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Pn(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Pn(MraCol::M), mra.clone(), mra.pn_m.clone(), None, Some(mra.pn_n.clone()), mra.pn_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Pn(MraCol::N), mra.clone(), mra.pn_n.clone(), None, Some(mra.pn_m.clone()), mra.pn_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P1), mra.clone(), mra.pn_1.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P2), mra.clone(), mra.pn_2.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P3), mra.clone(), mra.pn_3.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P4), mra.clone(), mra.pn_4.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P5), mra.clone(), mra.pn_5.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P6), mra.clone(), mra.pn_6.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P7), mra.clone(), mra.pn_7.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P8), mra.clone(), mra.pn_8.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Pn(MraCol::P9), mra.clone(), mra.pn_9.clone(), None, Some((mra.pn_m.clone(), mra.pn_n.clone())), mra.pn_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.pn_score.signal_cloned().map(|u| Some(value_box(u, Mutable::new(false)))))}),
                                        html!("div",{.child(text_input(mra.pn_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Cr(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Cr(MraCol::NA), mra.clone(), mra.cr_na.clone(), None, None, mra.cr_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Cr(MraCol::M), mra.clone(), mra.cr_m.clone(), Some(mra.cr_na.clone()), Some(mra.cr_n.clone()), mra.cr_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Cr(MraCol::N), mra.clone(), mra.cr_n.clone(), Some(mra.cr_na.clone()), Some(mra.cr_m.clone()), mra.cr_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P1), mra.clone(), mra.cr_1.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P2), mra.clone(), mra.cr_2.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P3), mra.clone(), mra.cr_3.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P4), mra.clone(), mra.cr_4.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P5), mra.clone(), mra.cr_5.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P6), mra.clone(), mra.cr_6.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P7), mra.clone(), mra.cr_7.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P8), mra.clone(), mra.cr_8.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Cr(MraCol::P9), mra.clone(), mra.cr_9.clone(), Some(mra.cr_na.clone()), Some((mra.cr_m.clone(), mra.cr_n.clone())), mra.cr_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.cr_score.signal_cloned().map(clone!(mra => move |u| Some(value_box(u, mra.cr_na.clone())))))}),
                                        html!("div",{.child(text_input(mra.cr_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Ar(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Ar(MraCol::NA), mra.clone(), mra.ar_na.clone(), None, None, mra.ar_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Ar(MraCol::M), mra.clone(), mra.ar_m.clone(), Some(mra.ar_na.clone()), Some(mra.ar_n.clone()), mra.ar_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Ar(MraCol::N), mra.clone(), mra.ar_n.clone(), Some(mra.ar_na.clone()), Some(mra.ar_m.clone()), mra.ar_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P1), mra.clone(), mra.ar_1.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P2), mra.clone(), mra.ar_2.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P3), mra.clone(), mra.ar_3.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P4), mra.clone(), mra.ar_4.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P5), mra.clone(), mra.ar_5.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P6), mra.clone(), mra.ar_6.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P7), mra.clone(), mra.ar_7.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P8), mra.clone(), mra.ar_8.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Ar(MraCol::P9), mra.clone(), mra.ar_9.clone(), Some(mra.ar_na.clone()), Some((mra.ar_m.clone(), mra.ar_n.clone())), mra.ar_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.ar_score.signal_cloned().map(clone!(mra => move |u| Some(value_box(u, mra.ar_na.clone())))))}),
                                        html!("div",{.child(text_input(mra.ar_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::On(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::On(MraCol::NA), mra.clone(), mra.on_na.clone(), None, None, mra.on_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::On(MraCol::M), mra.clone(), mra.on_m.clone(), Some(mra.on_na.clone()), Some(mra.on_n.clone()), mra.on_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::On(MraCol::N), mra.clone(), mra.on_n.clone(), Some(mra.on_na.clone()), Some(mra.on_m.clone()), mra.on_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P1), mra.clone(), mra.on_1.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P2), mra.clone(), mra.on_2.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P3), mra.clone(), mra.on_3.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P4), mra.clone(), mra.on_4.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P5), mra.clone(), mra.on_5.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P6), mra.clone(), mra.on_6.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P7), mra.clone(), mra.on_7.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P8), mra.clone(), mra.on_8.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::On(MraCol::P9), mra.clone(), mra.on_9.clone(), Some(mra.on_na.clone()), Some((mra.on_m.clone(), mra.on_n.clone())), mra.on_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.on_score.signal_cloned().map(clone!(mra => move |u| Some(value_box(u, mra.on_na.clone())))))}),
                                        html!("div",{.child(text_input(mra.on_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                            ])
                            .child_signal(mra.is_psychiatry.signal_cloned().map(clone!(mra => move |is_psychiatry| {
                                (!is_psychiatry).then(|| {
                                    html!("div", {
                                        .class("d-flex")
                                        .children([
                                            html!("div",{.child(Self::label_box(MraRow::Lr(MraCol::La), mra.clone()))}),
                                            html!("div",{.child(Self::check_bool_btn(MraRow::Lr(MraCol::NA), mra.clone(), mra.lr_na.clone(), None, None, mra.lr_recal.clone()))}),
                                            html!("div",{.child(Self::check_bool_btn(MraRow::Lr(MraCol::M), mra.clone(), mra.lr_m.clone(), Some(mra.lr_na.clone()), Some(mra.lr_n.clone()), mra.lr_recal.clone()))}),
                                            html!("div",{.child(Self::check_bool_btn(MraRow::Lr(MraCol::N), mra.clone(), mra.lr_n.clone(), Some(mra.lr_na.clone()), Some(mra.lr_m.clone()), mra.lr_recal.clone()))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P1), mra.clone(), mra.lr_1.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P2), mra.clone(), mra.lr_2.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P3), mra.clone(), mra.lr_3.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P4), mra.clone(), mra.lr_4.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P5), mra.clone(), mra.lr_5.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P6), mra.clone(), mra.lr_6.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P7), mra.clone(), mra.lr_7.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P8), mra.clone(), mra.lr_8.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(Self::check_opt_bool_btn(MraRow::Lr(MraCol::P9), mra.clone(), mra.lr_9.clone(), Some(mra.lr_na.clone()), Some((mra.lr_m.clone(), mra.lr_n.clone())), mra.lr_recal.clone(), false))}),
                                            html!("div",{.child(blank_box())}),
                                            html!("div",{.child_signal(mra.lr_score.signal_cloned().map(clone!(mra => move |u| Some(value_box(u, mra.lr_na.clone())))))}),
                                            html!("div",{.child(text_input(mra.lr_text.clone(), mra.changed.clone()))}),
                                        ])
                                    })
                                })
                            })))
                            .children([
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Rr(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Rr(MraCol::NA), mra.clone(), mra.rr_na.clone(), None, None, mra.rr_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Rr(MraCol::M), mra.clone(), mra.rr_m.clone(), Some(mra.rr_na.clone()), Some(mra.rr_n.clone()), mra.rr_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Rr(MraCol::N), mra.clone(), mra.rr_n.clone(), Some(mra.rr_na.clone()), Some(mra.rr_m.clone()), mra.rr_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P1), mra.clone(), mra.rr_1.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P2), mra.clone(), mra.rr_2.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P3), mra.clone(), mra.rr_3.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P4), mra.clone(), mra.rr_4.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P5), mra.clone(), mra.rr_5.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P6), mra.clone(), mra.rr_6.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P7), mra.clone(), mra.rr_7.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P8), mra.clone(), mra.rr_8.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Rr(MraCol::P9), mra.clone(), mra.rr_9.clone(), Some(mra.rr_na.clone()), Some((mra.rr_m.clone(), mra.rr_n.clone())), mra.rr_recal.clone(), false))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child_signal(mra.rr_score.signal_cloned().map(clone!(mra => move |u| Some(value_box(u, mra.rr_na.clone())))))}),
                                        html!("div",{.child(text_input(mra.rr_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                                html!("div", {
                                    .class("d-flex")
                                    .children([
                                        html!("div",{.child(Self::label_box(MraRow::Nn(MraCol::La), mra.clone()))}),
                                        html!("div",{.child(blank_box())}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Nn(MraCol::M), mra.clone(), mra.nn_m.clone(), None, Some(mra.nn_n.clone()), mra.nn_recal.clone()))}),
                                        html!("div",{.child(Self::check_bool_btn(MraRow::Nn(MraCol::N), mra.clone(), mra.nn_n.clone(), None, Some(mra.nn_m.clone()), mra.nn_recal.clone()))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P1), mra.clone(), mra.nn_1.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P2), mra.clone(), mra.nn_2.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P3), mra.clone(), mra.nn_3.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P4), mra.clone(), mra.nn_4.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P5), mra.clone(), mra.nn_5.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P6), mra.clone(), mra.nn_6.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P7), mra.clone(), mra.nn_7.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P8), mra.clone(), mra.nn_8.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::P9), mra.clone(), mra.nn_9.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), false))}),
                                        html!("div",{.child(Self::check_opt_bool_btn(MraRow::Nn(MraCol::S), mra.clone(), mra.nn_sub.clone(), None, Some((mra.nn_m.clone(), mra.nn_n.clone())), mra.nn_recal.clone(), true))}),
                                        html!("div",{.child_signal(mra.nn_score.signal_cloned().map(|u| Some(value_box(u, Mutable::new(false)))))}),
                                        html!("div",{.child(text_input(mra.nn_text.clone(), mra.changed.clone()))}),
                                    ])
                                }),
                            ])
                        }),
                        // summary
                        html!("div", {
                            .class(class::BOLD_C_PY2)
                            .children([
                                text("คะแนนเต็ม (Full score) รวม "),
                                html!("span", {.class("text-primary").text_signal(mra.full.signal_cloned().map(|u| u.to_string()))}),
                                html!("span", {
                                    .text(" คะแนน (ต้องไม่น้อยกว่า ")
                                    .text_signal(mra.is_psychiatry.signal_cloned().map(|is_psy| if is_psy {"57"} else {"56"}))
                                    .text(" คะแนน) คะแนนที่ได้ (Sum score) ")
                                }),
                                html!("span", {.class("text-primary").text_signal(mra.score.signal_cloned().map(|u| u.to_string()))}),
                                text(" ร้อยละ "),
                                html!("span", {.class("text-primary").text_signal(map_ref! {
                                    let full = mra.full.signal(),
                                    let score = mra.score.signal() =>
                                    if *full == 0 {
                                        String::from("???")
                                    } else {
                                        f64_rescale((*score as f64).div(*full as f64).mul(100.0),2).to_string()
                                    }
                                })})
                            ])
                        }),
                        // overall
                        html!("div", {
                            .class(class::FLEX_JCC)
                            .child(html!("div", {
                                .class("mb-2")
                                .children([
                                    html!("div", {
                                        .class("fw-bold")
                                        .text("ประเมินคุณภาพการบันทึกเวชระเบียนในภาพรวม")
                                    }),
                                    html!("div", {
                                        .children([
                                            html!("div", {
                                                .class(class::FORM_CHK_SW)
                                                .children([
                                                    html!("input" => HtmlInputElement, {
                                                        .class("form-check-input")
                                                        .attr("type", "checkbox")
                                                        .attr("role","switch")
                                                        .attr("id", "is-not-sorted-toggle")
                                                        .apply(mixins::checkbox_bool(mra.is_not_sorted.clone(), mra.changed.clone()))
                                                    }),
                                                    doms::label_check_for("is-not-sorted-toggle","การจัดเรียงเวชระเบียนไม่เป็นตามมาตรฐานที่กำหนด"),
                                                ])
                                            }),
                                            html!("div", {
                                                .class(class::FORM_CHK_SW)
                                                .children([
                                                    html!("input" => HtmlInputElement, {
                                                        .class("form-check-input")
                                                        .attr("type", "checkbox")
                                                        .attr("role","switch")
                                                        .attr("id", "is-unknown-toggle")
                                                        .apply(mixins::checkbox_bool(mra.is_unknown.clone(), mra.changed.clone()))
                                                    }),
                                                    doms::label_check_for("is-unknown-toggle","เอกสารบางแผ่น ไม่มีชื่อผู้รับบริการ HN AN ทำให้ไม่สามารถระบุได้ว่า เอกสารแผ่นนี้เป็นของใคร จึงไม่สามารถทบทวนเอกสารแผ่นนั้นได้"),
                                                ])
                                            }),
                                            html!("div", {
                                                .class("ms-3")
                                                .class("d-flex")
                                                .child(html!("div", {
                                                    .class(class::FLEX_COL)
                                                    .style("min-width","300px")
                                                    .children([
                                                        html!("div", {
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .class("form-check-input")
                                                                    .attr("type", "radio")
                                                                    .attr("id", "overall-inadequate-radio")
                                                                    .apply(mixins::radio_opt_match(mra.overall.clone(), mra.changed.clone(), "I"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", "overall-inadequate-radio")
                                                                    .style("user-select","none")
                                                                    .text("ข้อมูลไม่เพียงพอสำหรับการทบทวน")
                                                                }),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .class("form-check-input")
                                                                    .attr("type", "radio")
                                                                    .attr("id", "overall-no-problem-radio")
                                                                    .apply(mixins::radio_opt_match(mra.overall.clone(), mra.changed.clone(), "N"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", "overall-no-problem-radio")
                                                                    .style("user-select","none")
                                                                    .text("ไม่มีปัญหาสำคัญจากการทบทวน")
                                                                }),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .class("form-check-input")
                                                                    .attr("type", "radio")
                                                                    .attr("id", "overall-problem-radio")
                                                                    .apply(mixins::radio_opt_match(mra.overall.clone(), mra.changed.clone(), "P"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", "overall-problem-radio")
                                                                    .style("user-select","none")
                                                                    .text("มีปัญหาจากการทบทวนที่ต้องค้นต่อ ระบุ..")
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                }))
                                                .child_signal(mra.overall.signal_cloned().map(clone!(mra => move |opt| {
                                                    opt.as_ref().and_then(|overall| (overall.as_str() == "P").then(|| {
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class("form-control")
                                                            .attr("placeholder","ปัญหาจากการทบทวน")
                                                            .apply(mixins::textarea_opt_value_auto_expand(mra.overall_text.clone(), mra.changed.clone()))
                                                        })
                                                    }))
                                                })))
                                                .future(mra.overall.signal_cloned().for_each(clone!(mra => move |opt| {
                                                    if opt != Some(String::from("P")) {
                                                        mra.overall_text.set(None);
                                                    }
                                                    async {}
                                                })))
                                            })
                                        ])
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::FLEX_JCR)
                    .child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_L_BLUE)
                        .child(html!("i", {.class(class::FA_L_ARROW)}))
                        .text(" กลับ")
                        .event(clone!(app => move |_: events::Click| {
                            if app.go_back_else() {
                                for route in [
                                    Route::IpdPostAdmitList { view_by: String::from("doctor") },
                                    Route::IpdPostAdmitList { view_by: String::from("nurse") },
                                    Route::IpdPostAdmitList { view_by: String::from("pharmacist") },
                                    Route::IpdPostAdmitList { view_by: String::from("other") },
                                    Route::Info,
                                ] {
                                    if route.has_permission(app.state()) {
                                        route.hard_redirect();
                                        break;
                                    }
                                }
                            }
                        }))
                    }))
                    .child_signal(mra.mra_id.signal_cloned().map(clone!(app, mra, list_loaded => move |mra_id| {
                        let is_pre_admit = app.is_pre_admit(&mra.an.lock_ref());
                        (if mra_id > 0 {
                            app.endpoint_is_allow(&Method::PUT, &EndPoint::IpdMra, is_pre_admit)
                        } else {
                            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdMra, is_pre_admit)
                        }).then(|| {
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_L)
                                .class_signal("btn-primary", mra.changed.signal())
                                .class_signal("btn-secondary", not(mra.changed.signal()))
                                .text(if mra_id > 0 { "แก้ไข" } else { "บันทึก" })
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, mra, list_loaded => move || {
                                    Self::save(mra.clone(), list_loaded.clone(), app.clone());
                                }), not(mra.changed.signal()), app.state()))
                            })
                        })
                    })))
                    .child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_GRAY)
                        .text("ยกเลิก")
                        .event(clone!(mra => move |_:events::Click| {
                            mra.ipd_mra_selected.set(mra.ipd_mra_selected.get_cloned());
                            mra.changed.set(false);
                        }))
                    }))
                    .children(PdfButtons::buttons(
                        PdfButtons::new(
                            TypstReport::from_system_with_coercion(SystemReport::IpdMRA, &app.state().report_coercions()),
                            mra.an.clone(),
                            Mutable::new(true),
                            mra.changed.clone(),
                            clone!(mra => move || {serde_json::json!({
                                "id": mra.an.lock_ref().as_str(),
                                "mra": [mra.to_mra()],
                            }).to_string()})
                        ), "PDF", Some("PDF (All)"), app.clone()
                    ))
                    .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdMra, app.is_pre_admit(&mra.an.lock_ref())), |dom| dom
                        .child_signal(mra.mra_id.signal_cloned().map(clone!(mra, list_loaded => move |mra_id| {
                            (mra_id > 0).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_R_RED)
                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                    .text(" ลบ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, mra, list_loaded => move || {
                                        Self::delete(mra.clone(), list_loaded.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                    )
                }),
            ])
        })
    }

    fn check_bool_btn(
        row: MraRow,
        mra: Rc<Self>,
        mutable: Mutable<bool>,
        true_disable_mutable_na: Option<Mutable<bool>>,
        true_disable_mutable_mn: Option<Mutable<bool>>,
        recal_mutable: Mutable<bool>,
    ) -> Dom {
        html!("button" => HtmlButtonElement, {
            .attr("type", "button")
            .style("width","38px")
            .style("height","38px")
            .style("margin","1px")
            .class(class::BTN_GRAYO_P0)
            .class_signal("text-bg-secondary", mutable.signal())
            .child_signal(mutable.signal_cloned().map(|b| b.then(|| html!("i", {.class(class::FA_X)}))))
            .style_signal("border-width", mra.selected_row.signal_cloned().map(clone!(row => move |srow| if srow == Some(row.clone()) {"3px"} else {"1px"})))
            .apply(|dom| {
                if let (Some(disable_na), Some(disable_mn)) = (true_disable_mutable_na, true_disable_mutable_mn.clone()) {
                    with_node!(dom, element => {
                        .future(map_ref! {
                            let is_na = disable_na.signal(),
                            let is_mn = disable_mn.signal() =>
                            *is_na || *is_mn
                        }.for_each(move |v| {
                            element.set_disabled(v);
                            async {}
                        }))
                    })
                } else if let Some(disable) = true_disable_mutable_mn {
                    with_node!(dom, element => {
                        .future(disable.signal().for_each(move |v| {
                            element.set_disabled(v);
                            async {}
                        }))
                    })
                } else {
                    dom
                }
            })
            .event(move |_:events::Click| {
                // edit in the next click after show document
                let select_this = mra.selected_row.lock_ref().as_ref().map(|r| *r == row).unwrap_or_default();
                if select_this {
                    let current = !mutable.get();
                    mutable.set(current);
                    match (row.is_na(), current) {
                        (true,true) => {
                            mra.reset_mn(&row);
                            mra.set_all(&row, None);
                        }
                        (false,true) => mra.set_all(&row, Some(false)),
                        (_,_) => mra.set_all(&row, Some(true)),
                    }
                    recal_mutable.set(true);
                    mra.changed.set_neq(true);
                } else {
                    mra.selected_row.set(Some(row.clone()));
                    let mra_type = if mra.is_psychiatry.get() {Mra::IpdPsy} else {Mra::Ipd};
                    mra.document.set(row.doc(mra_type));
                }
                // update report-preview
                match row.report() {
                    Report::Template(report) => {
                        if !mra.selected_template.get_cloned().map(|selected| selected == report).unwrap_or_default() {
                            mra.selected_template.set(Some(report));
                            mra.selected_document.set(None);
                            mra.load_and_render_report_svg.set(true);
                        }
                    }
                    Report::Document(doc) => {
                        if !mra.selected_document.get_cloned().map(|selected| selected == doc).unwrap_or_default() {
                            mra.selected_document.set(Some(doc));
                            mra.selected_template.set(None);
                            mra.load_and_render_document_svg.set(true);
                        }
                    }
                }
            })
        })
    }

    fn check_opt_bool_btn(
        row: MraRow,
        mra: Rc<Self>,
        mutable: Mutable<Option<bool>>,
        true_disable_mutable_na: Option<Mutable<bool>>,
        true_disable_mutable_mn: Option<(Mutable<bool>, Mutable<bool>)>,
        recal_mutable: Mutable<bool>,
        reverse: bool,
    ) -> Dom {
        html!("button" => HtmlButtonElement, {
            .attr("type", "button")
            .style("width","38px")
            .style("height","38px")
            .style("margin","1px")
            .style_signal("border-width", map_ref! {
                let is_psy = mra.is_psychiatry.signal(),
                let srow = mra.selected_row.signal_cloned() =>
                (*is_psy, srow.clone())
            }.map(clone!(row => move |(is_psy, srow)| {
                let mra_type = if is_psy {Mra::IpdPsy} else {Mra::Ipd};
                if srow == Some(row.clone()) {"3px"} else if row.can_na(mra_type) {"2px"} else {"1px"}
            })))
            .class(class::BTN_P0)
            .class_signal("bg-secondary", mutable.signal_cloned().map(|opt| opt.is_none()))
            .class_signal("btn-outline-primary", mutable.signal_cloned().map(move |opt| opt == Some(!reverse)))
            .class_signal("btn-outline-danger", mutable.signal_cloned().map(move |opt| opt == Some(reverse)))
            .class_signal("fw-bold", mra.selected_row.signal_cloned().map(clone!(row => move |srow| srow == Some(row.clone()))))
            .text_signal(mutable.signal_cloned().map(|opt| {
                match opt {
                    Some(x) => if x {"1"} else {"0"},
                    None => "NA"
                }
            }))
            .apply(|dom| {
                if let (Some(disable_na), Some((disable_m, disable_n))) = (true_disable_mutable_na, true_disable_mutable_mn.clone()) {
                    with_node!(dom, element => {
                        .future(map_ref! {
                            let is_na = disable_na.signal(),
                            let is_m = disable_m.signal(),
                            let is_n = disable_n.signal() =>
                            *is_na || *is_m || *is_n
                        }.for_each(move |v| {
                            element.set_disabled(v);
                            async {}
                        }))
                    })
                } else if let Some((disable_m, disable_n)) = true_disable_mutable_mn {
                    with_node!(dom, element => {
                        .future(map_ref! {
                            let is_m = disable_m.signal(),
                            let is_n = disable_n.signal() =>
                            *is_m || *is_n
                        }.for_each(move |v| {
                            element.set_disabled(v);
                            async {}
                        }))
                    })
                } else {
                    dom
                }
            })
            .event(clone!(row, mra => move |_:events::Click| {
                // edit in the next click after show document
                let mra_type = if mra.is_psychiatry.get() {Mra::IpdPsy} else {Mra::Ipd};
                let select_this = mra.selected_row.lock_ref().as_ref().map(|r| *r == row).unwrap_or_default();
                if select_this {
                    let value = mutable.get();
                    let next = if row.can_na(mra_type) {
                        match value {
                            Some(true) => Some(false),
                            Some(false) => None,
                            None => Some(true),
                        }
                    } else {
                        match value {
                            Some(true) => Some(false),
                            Some(false)
                            | None => Some(true),
                        }
                    };
                    mutable.set(next);
                    recal_mutable.set(true);
                    mra.changed.set_neq(true);
                } else {
                    mra.selected_row.set(Some(row.clone()));
                    mra.document.set(row.doc(mra_type));
                }
                // update report-preview
                match row.report() {
                    Report::Template(report) => {
                        if !mra.selected_template.get_cloned().map(|selected| selected == report).unwrap_or_default() {
                            mra.selected_template.set(Some(report));
                            mra.selected_document.set(None);
                            mra.load_and_render_report_svg.set(true);
                        }
                    }
                    Report::Document(doc) => {
                        if !mra.selected_document.get_cloned().map(|selected| selected == doc).unwrap_or_default() {
                            mra.selected_document.set(Some(doc));
                            mra.selected_template.set(None);
                            mra.load_and_render_document_svg.set(true);
                        }
                    }
                }
            }))
        })
    }

    fn label_box(row: MraRow, mra: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::BORDER_LB)
            .style("width","120px")
            .style("height","40px")
            .style("line-height","40px")
            .style("cursor","pointer")
            .text_signal(mra.is_psychiatry.signal_cloned().map(clone!(row => move |is_psy| {
                let mra_type = if is_psy {Mra::IpdPsy} else {Mra::Ipd};
                row.label(mra_type)
            })))
            .event(move |_:events::Click| {
                mra.selected_row.set(Some(row.clone()));
                let mra_type = if mra.is_psychiatry.get() {Mra::IpdPsy} else {Mra::Ipd};
                mra.document.set(row.doc(mra_type));
            })
        })
    }
}

fn blank_box() -> Dom {
    html!("div", {
        .class("bg-secondary")
        .style("width","38px")
        .style("height","38px")
        .style("margin","1px")
    })
}

fn value_box(value: usize, na: Mutable<bool>) -> Dom {
    html!("div", {
        .class(class::BTN_P0)
        .class_signal("btn-outline-success", not(na.signal()))
        .class_signal("btn-outline-secondary", na.signal())
        .style("width","38px")
        .style("height","38px")
        .style("margin","1px")
        .style("line-height","34px")
        .style("cursor","default")
        .style("pointer-events","none")
        .style("font-weight","700")
        .style("border-width","2px")
        .text_signal(na.signal().map(move |is_na| {
            if is_na {
                String::from("NA")
            } else {
                value.to_string()
            }
        }))
    })
}

fn text_input(mutable: Mutable<Option<String>>, changed: Mutable<bool>) -> Dom {
    html!("div", {
        .class(class::BORDER_RB)
        .style("height","40px")
        .child(html!("input" => HtmlInputElement, {
            .style("width","199px")
            .style("height","38px")
            .style("border-style","none")
            .apply(mixins::opt_string_value(mutable, changed))
        }))
    })
}
