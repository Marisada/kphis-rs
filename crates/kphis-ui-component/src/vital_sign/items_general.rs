use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use time::Date;
use wasm_bindgen_futures::spawn_local;

use kphis_model::vital_sign::VitalSign;
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::datetime_th,
    util::{decimal_rescale, opt_empty_none},
};

use super::{row::full_text, vital_sign_data::VitalSignDataCpn};

pub fn table_heads_general() -> [Dom; 10] {
    [
        html!("th", {.class("text-center").attr("scope", "col").text("O2RA")}),
        html!("th", {.class("text-center").attr("scope", "col").text("SAT")}),
        html!("th", {.class("text-center").attr("scope", "col").text("O2")}),
        html!("th", {.class("text-center").attr("scope", "col").style("min-width","140px").text("EWS")}),
        html!("th", {.class("text-center").attr("scope", "col").text("HAD")}),
        html!("th", {.class("text-center").attr("scope", "col").text("PS")}),
        html!("th", {.class("text-center").attr("scope", "col").text("DTX")}),
        html!("th", {.class("text-center").attr("scope", "col").text("HCT")}),
        html!("th", {.class("text-center").attr("scope", "col").text("U")}),
        html!("th", {.class("text-center").attr("scope", "col").text("F")}),
    ]
}

pub fn search_btns_general(page: Rc<VitalSignDataCpn>) -> [Dom; 10] {
    [
        VitalSignDataCpn::search_btn("O2RA", "[O2RA]", page.clone()),
        VitalSignDataCpn::search_btn("SAT", "[SAT]", page.clone()),
        VitalSignDataCpn::search_btn("O2", "[O2]", page.clone()),
        VitalSignDataCpn::search_btn("EWS", "[EWS]", page.clone()),
        VitalSignDataCpn::search_btn("HAD", "[HAD]", page.clone()),
        VitalSignDataCpn::search_btn("PS", "[PS]", page.clone()),
        VitalSignDataCpn::search_btn("DTX", "[DTX]", page.clone()),
        VitalSignDataCpn::search_btn("HCT", "[HCT]", page.clone()),
        VitalSignDataCpn::search_btn("U", "[U]", page.clone()),
        VitalSignDataCpn::search_btn("F", "[F]", page.clone()),
    ]
}

pub fn render_vs_result(row: Rc<VitalSign>, vs_id: Mutable<u32>, form_rendered: Mutable<bool>, birth_day: Option<Date>, app: Rc<App>) -> Dom {
    let (full_text, scores) = full_text(row.clone(), birth_day, app.clone());
    let (_vs_datetime_opt, ews_dom, qsofa_dom, sirs_dom) = doms::badge_scores_and_vs_datetime(&scores);

    let bt = row.bt.map(|d| d.to_string());
    let pr = row.pr.map(|u| u.to_string());
    let rr = row.rr.map(|u| u.to_string());
    let sbp = row.sbp.map(|u| u.to_string());
    let dbp = row.dbp.map(|u| u.to_string());
    let map = row.map.map(|i| i.to_string());

    let sat_room_air = row.sat_room_air.map(|u| u.to_string());
    let sat = row.sat.map(|u| u.to_string());
    let o2 = row.o2_name.clone().map(|o2_name| {
        if o2_name.as_str() == "Tube" {
            let tube_name = row.tube_name.clone().unwrap_or(String::from("Tube"));
            let tube_no = row.tube_no.map(|tubeno| [" ", &decimal_rescale(tubeno, 1).to_string()].concat()).unwrap_or_default();
            let tube_deep = row.tube_mark.map(|tubemark| [" d", &decimal_rescale(tubemark, 1).to_string()].concat()).unwrap_or_default();
            [tube_name, tube_no, tube_deep].concat()
        } else {
            let flow = row.o2_flow.map(|o2flow| [" ", &decimal_rescale(o2flow, 0).to_string(), "LPM"].concat()).unwrap_or_default();
            let fio2 = row.fio2.map(|o2flow| [" ", &decimal_rescale(o2flow, 1).to_string()].concat()).unwrap_or_default();
            [o2_name, flow, fio2].concat()
        }
    });
    let pain = row.pain.map(|i| i.to_string());
    let hct = row.hct.map(|d| d.to_string());
    let urine = if row.catheter.clone().unwrap_or_default().as_str() == "Y" {
        String::from("R")
    } else {
        row.urine.clone().unwrap_or_default()
    };
    let had_name = opt_empty_none(row.had_name.clone());

    html!("tr", {
        .class_signal("table-info", vs_id.signal().map(clone!(row => move |n| n == row.vs_id)))
        .children([
            html!("td", {
                .class("text-nowrap")
                .style("cursor","pointer")
                .text(&datetime_th(&row.vs_datetime))
                .attr("title", &[
                    "บันทึกโดย: ", &row.create_opduser_name.clone().unwrap_or_default(),
                    " (", &datetime_th(&row.create_datetime), ")\nแก้ไขล่าสุด: ", &row.update_opduser_name.clone().unwrap_or_default(), " (", &datetime_th(&row.update_datetime), ")"
                ].concat())
            }),
            html!("td", {.class("text-center").text(&bt.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&pr.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&rr.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&sbp.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&dbp.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&map.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&sat_room_air.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&sat.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&o2.unwrap_or_default())}),
            html!("td", {.class("text-center").child(ews_dom).child(qsofa_dom).child(sirs_dom)}),
            html!("td", {.class("text-center").apply_if(had_name.is_some(), |dom| dom.child(html!("i", {
                .class(class::FA_ALERT_RED)
                .style("font-size", "24px")
                .style("cursor", "help")
                .attr("title", &[&had_name.unwrap_or_default(), " rate ", &row.had_drop.clone().unwrap_or_default()].concat())
            })))}),
            html!("td", {.class("text-center").text(&pain.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&row.dtx.clone().unwrap_or_default())}),
            html!("td", {.class("text-center").text(&hct.unwrap_or_default())}),
            html!("td", {.class(class::NOWRAP_C).text(&urine)}),
            html!("td", {.class("text-center").text(&row.feces.clone().unwrap_or_default())}),
            html!("td", {
                .style_signal("border-right-color", vs_id.signal().map(clone!(row => move |n| {
                    if n == row.vs_id {
                        "red"
                    } else {
                        "inherit"
                    }
                })))
                .style_signal("border-right-width", vs_id.signal().map(clone!(row => move |n| {
                    if n == row.vs_id {
                        "5px"
                    } else {
                        "inherit"
                    }
                })))
                .class("text-center")
                .child(doms::span_with_tooltip(
                    clone!(app, full_text => move |dom| { dom
                        .child(html!("span", {.style("cursor","copy").text("คัดลอก/อ่าน")}))
                        .event(clone!(app, full_text => move |_: events::Click| {
                            spawn_local(clone!(app, full_text => async move {
                                app.set_clipboard(&full_text).await
                            }));
                        }))
                    }),
                    Some(&full_text),
                    clone!(row => move |dom| { dom
                        .apply_if(row.action_id.is_some(), |d| {
                            d.child(html!("i", {.class(class::FA_ALERT_GOLD)}))
                        })
                    }),
                ))
            }),
        ])
        .event_with_options(&EventOptions::preventable(), clone!(vs_id, form_rendered, row => move |event: events::Click| {
            event.prevent_default();
            vs_id.set(row.vs_id);
            form_rendered.set(false);
        }))
    })
}
