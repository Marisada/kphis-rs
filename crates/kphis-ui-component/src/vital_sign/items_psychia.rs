use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use time::Date;
use wasm_bindgen_futures::spawn_local;

use kphis_model::vital_sign::VitalSign;
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{datetime::datetime_th, util::opt_empty_none};

use super::{row::full_text, vital_sign_data::VitalSignDataCpn};

pub const TABLE_HEAD_PSYCHIA: [&str; 11] = ["AWQv2", "AWQ-H", "AWQ-A", "AWQ-R", "OAS", "MS", "CVS", "SOC", "CIWA", "AWS", "HAD"];

pub fn table_heads_psychia() -> impl Iterator<Item = Dom> {
    TABLE_HEAD_PSYCHIA.iter().map(|s| html!("th", {.class("text-center").attr("scope", "col").text(s)}))
}

pub fn search_btns_psychia(page: Rc<VitalSignDataCpn>) -> [Dom; 11] {
    [
        VitalSignDataCpn::search_btn("AWQv2", "[AWQv2]", page.clone()),
        VitalSignDataCpn::search_btn("AWQ-H", "[AWQ-H]", page.clone()),
        VitalSignDataCpn::search_btn("AWQ-A", "[AWQ-A]", page.clone()),
        VitalSignDataCpn::search_btn("AWQ-R", "[AWQ-R]", page.clone()),
        VitalSignDataCpn::search_btn("OAS", "[OAS]", page.clone()),
        VitalSignDataCpn::search_btn("MS", "[MS]", page.clone()),
        VitalSignDataCpn::search_btn("CVS", "[CVS]", page.clone()),
        VitalSignDataCpn::search_btn("SOC", "[SOC]", page.clone()),
        VitalSignDataCpn::search_btn("CIWA", "[CIWA]", page.clone()),
        VitalSignDataCpn::search_btn("AWS", "[AWS]", page.clone()),
        VitalSignDataCpn::search_btn("HAD", "[HAD]", page.clone()),
    ]
}

pub fn render_psychia_result(row: Rc<VitalSign>, vs_id: Mutable<u32>, form_rendered: Mutable<bool>, birth_day: Option<Date>, app: Rc<App>) -> Dom {
    let (full_text, _scores) = full_text(row.clone(), birth_day, app.clone());

    let bt = row.bt.map(|d| d.to_string());
    let pr = row.pr.map(|u| u.to_string());
    let rr = row.rr.map(|u| u.to_string());
    let sbp = row.sbp.map(|u| u.to_string());
    let dbp = row.dbp.map(|u| u.to_string());
    let map = row.map.map(|i| i.to_string());

    let (awq, awq_h, awq_a, awq_r) = row
        .amphetamine_awq
        .as_ref()
        .map(|concat| {
            let mut iter = concat.split(',');
            (iter.next(), iter.next(), iter.next(), iter.next())
        })
        .unwrap_or_default();
    let oas = row.aggression_oas.as_ref().and_then(|concat| concat.split(',').nth(0));
    let motivation = row.motivation_scale.map(|u| u.to_string());
    let craving = row.craving_scale.map(|u| u.to_string());
    let stage_of_change = row.stage_of_change_name.clone();
    let ciwa = row.alcohol_ciwa.as_ref().and_then(|concat| concat.split(',').nth(0));
    let aws = row.alcohol_aws.as_ref().and_then(|concat| concat.split(',').nth(0));

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
            html!("td", {.class("text-center").text(awq.unwrap_or_default())}),
            html!("td", {.class("text-center").text(awq_h.unwrap_or_default())}),
            html!("td", {.class("text-center").text(awq_a.unwrap_or_default())}),
            html!("td", {.class("text-center").text(awq_r.unwrap_or_default())}),
            html!("td", {.class("text-center").text(oas.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&motivation.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&craving.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&stage_of_change.unwrap_or_default())}),
            html!("td", {.class("text-center").text(ciwa.unwrap_or_default())}),
            html!("td", {.class("text-center").text(aws.unwrap_or_default())}),
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
