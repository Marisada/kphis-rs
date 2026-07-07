use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use time::Date;
use wasm_bindgen_futures::spawn_local;

use kphis_model::vital_sign::VitalSign;
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::datetime::datetime_th;

use super::{row::full_text, vital_sign_data::VitalSignDataCpn};

const TABLE_HEAD_LABOUR: [&str; 10] = ["POS", "FHS", "CX", "I", "D", "ST", "EFF", "MEM", "AF", "HAD"];

pub fn table_heads_labour() -> impl Iterator<Item = Dom> {
    TABLE_HEAD_LABOUR.iter().map(|s| html!("th", {.class("text-center").attr("scope", "col").text(s)}))
}

pub fn search_btns_labour(page: Rc<VitalSignDataCpn>) -> [Dom; 10] {
    [
        VitalSignDataCpn::search_btn("POS", "[POS]", page.clone()),
        VitalSignDataCpn::search_btn("FHS", "[FHS]", page.clone()),
        VitalSignDataCpn::search_btn("CX", "[CX]", page.clone()),
        VitalSignDataCpn::search_btn("I", "[I]", page.clone()),
        VitalSignDataCpn::search_btn("D", "[D]", page.clone()),
        VitalSignDataCpn::search_btn("ST", "[ST]", page.clone()),
        VitalSignDataCpn::search_btn("EFF", "[EFF]", page.clone()),
        VitalSignDataCpn::search_btn("MEM", "[MEM]", page.clone()),
        VitalSignDataCpn::search_btn("AF", "[AF]", page.clone()),
        VitalSignDataCpn::search_btn("HAD", "[HAD]", page.clone()),
    ]
}

pub fn render_lr_result(row: Rc<VitalSign>, vs_id: Mutable<u32>, form_rendered: Mutable<bool>, birth_day: Option<Date>, app: Rc<App>) -> Dom {
    let (full_text, _scores) = full_text(row.clone(), birth_day, app.clone());

    let bt = row.bt.map(|d| d.to_string());
    let pr = row.pr.map(|u| u.to_string());
    let rr = row.rr.map(|u| u.to_string());
    let sbp = row.sbp.map(|u| u.to_string());
    let dbp = row.dbp.map(|u| u.to_string());
    let map = row.map.map(|i| i.to_string());

    let pos = row.lr_pos.clone();
    let fhs = row.lr_fsh.map(|i| i.to_string());
    let cx = row.lr_cer.clone();
    let int = row.lr_int.clone();
    let dur = row.lr_dur.map(|d| d.to_string());
    let sta = row.lr_sta_name.clone();
    let eff = row.lr_eff.map(|d| d.to_string());
    let mem = row.lr_mem_name.clone();
    let af = row.lr_af.clone();

    let had_name = row.had_name.clone().map(|had_name| [&had_name, " rate ", &row.had_drop.clone().unwrap_or_default()].concat());
    let oxytocin = row.lr_oxytocin_unit.map(|u| {
        [
            "Oxytocin ",
            &u.to_string(),
            " U/L rate ",
            &row.lr_oxytocin_rate.map(|u| u.to_string()).unwrap_or_default(),
            " drops/min",
        ]
        .concat()
    });
    let had_sep = if had_name.is_some() && oxytocin.is_some() { ", " } else { "" };

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
            html!("td", {.class("text-center").text(&pos.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&fhs.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&cx.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&int.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&dur.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&sta.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&eff.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&mem.unwrap_or_default())}),
            html!("td", {.class("text-center").text(&af.unwrap_or_default())}),
            html!("td", {.class("text-center").apply_if(had_name.is_some() || oxytocin.is_some(), |dom| dom.child(html!("i", {
                .class(class::FA_ALERT_RED)
                .style("font-size", "24px")
                .style("cursor", "help")
                .attr("title", &[&had_name.unwrap_or_default(), had_sep, &oxytocin.unwrap_or_default()].concat())
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
