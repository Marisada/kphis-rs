use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use time::Date;
use wasm_bindgen_futures::spawn_local;

use kphis_model::vital_sign::VitalSign;
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{datetime::datetime_th, util::opt_empty_none};

use super::{
    row::{cha_id_to_short, full_text},
    vital_sign_data::VitalSignDataCpn,
};

const TABLE_HEAD_NEURO: [&str; 9] = ["GCS", "RP", "LP", "RA", "LA", "RL", "LL", "ADL", "HAD"];

pub fn table_heads_neuro() -> impl Iterator<Item = Dom> {
    TABLE_HEAD_NEURO.iter().map(|s| html!("th", {.class("text-center").attr("scope", "col").text(s)}))
}

pub fn search_btns_neuro(page: Rc<VitalSignDataCpn>) -> [Dom; 9] {
    [
        VitalSignDataCpn::search_btn("GCS", "[GCS]", page.clone()),
        VitalSignDataCpn::search_btn("RP", "[RP]", page.clone()),
        VitalSignDataCpn::search_btn("LP", "[LP]", page.clone()),
        VitalSignDataCpn::search_btn("RA", "[RA]", page.clone()),
        VitalSignDataCpn::search_btn("LA", "[LA]", page.clone()),
        VitalSignDataCpn::search_btn("RL", "[RL]", page.clone()),
        VitalSignDataCpn::search_btn("LL", "[LL]", page.clone()),
        VitalSignDataCpn::search_btn("ADL", "[ADL]", page.clone()),
        VitalSignDataCpn::search_btn("HAD", "[HAD]", page.clone()),
    ]
}

pub fn render_neuro_result(row: Rc<VitalSign>, vs_id: Mutable<u32>, form_rendered: Mutable<bool>, birth_day: Option<Date>, app: Rc<App>) -> Dom {
    let (full_text, _scores) = full_text(row.clone(), birth_day, app.clone());

    let bt = row.bt.map(|d| d.to_string());
    let pr = row.pr.map(|u| u.to_string());
    let rr = row.rr.map(|u| u.to_string());
    let sbp = row.sbp.map(|u| u.to_string());
    let dbp = row.dbp.map(|u| u.to_string());
    let map = row.map.map(|i| i.to_string());

    let gcs = row.eye.map(|i| {
        [
            "E",
            &i.to_string(),
            "V",
            &row.verbal.clone().unwrap_or(String::from("-")),
            "M",
            &row.movement.map(|m| m.to_string()).unwrap_or(String::from("-")),
        ]
        .concat()
    });
    let rt_pupil = row.right_pupil.map(|d| [&d.to_string(), " ", cha_id_to_short(row.right_cha_id.unwrap_or_default())].concat());
    let lt_pupil = row.left_pupil.map(|d| [&d.to_string(), " ", cha_id_to_short(row.left_cha_id.unwrap_or_default())].concat());
    let rt_arm = row.rt_arm_name.clone();
    let lt_arm = row.lt_arm_name.clone();
    let rt_leg = row.rt_leg_name.clone();
    let lt_leg = row.lt_leg_name.clone();
    let barthel = row.barthel_index.as_ref().and_then(|concat| concat.split(',').nth(0));

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
            html!("td", {.class("text-center").text(&gcs.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&rt_pupil.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&lt_pupil.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&rt_arm.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&lt_arm.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&rt_leg.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&lt_leg.unwrap_or_default())}),
            html!("td", {.class("text-center").text(barthel.unwrap_or_default())}),
            html!("td", {.class("text-center").apply_if(had_name.is_some(), |dom| dom.child(html!("i", {
                .class(class::FA_ALERT_RED)
                .style("font-size", "24px")
                .style("cursor", "help")
                .attr("title", &[&had_name.unwrap_or_default(), " rate ", &row.had_drop.clone().unwrap_or_default()].concat())
            })))}),
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
