pub use time_datepicker_core::{
    config::{PickerConfigBuilder, date_constraints::DateConstraintsBuilder},
    dialog_view_type::DialogViewType,
};

use dominator::{Dom, DomBuilder, clone, events, html, svg, text, traits::MultiStr, window_offset, window_size};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use std::rc::Rc;
use time::PrimitiveDateTime;
use time_datepicker_core::config::{PickerConfig, date_constraints::DateConstraints};
use web_sys::{DomRect, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    DEFAULT_USER_IMAGE, PATH_PREFIX_PATIENT_IMAGE,
    antibiogram::Antibiograms,
    app::{AppState, VisitTypeId},
    avatar::AvatarEnum,
    index_action::IndexAction,
    ipd::summary::AuditStatus,
    order::OrderItem,
    score::{ScoreDispatch, Scores},
    select_utils::{ColorSelectOption, SelectOption},
};

use crate::{class, datetime_pickers, doms, mixins};

//=====  ===== //
// With mixins //
//=====  ===== //

pub fn alert_row<F>(mixins: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class(class::ALERT_GRAY_TX)
        .attr("role", "alert")
        .apply(mixins)
    })
}

pub fn form_inline<F>(mixins: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class(class::ROW_AUTO_SM_G2_CT)
        .apply(mixins)
    })
}

// doms::form_inline_group_sm([]),
/// child signature are
/// - input-group-text : elm.class("input-group-text")
/// - NiceSelect : div.class(class::FLEX_GROW1).child(NiceSelect.class(class::FORM_CTRL_SM))
/// - select, input : select.class(class::FORM_SELECT_SM)
/// - other : div.class("col-xx").child(other)
pub fn form_inline_group_sm<F>(mixins: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("col-12")
        .child(html!("div", {
            .class(class::INPUT_GROUP_SM)
            .apply(mixins)
        }))
    })
}

/// button.class(class::BTN_SM_L_GRAY)
pub fn form_inline_end<F>(mixins: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class(class::COL12_RRX)
        .apply(mixins)
    })
}

/// child signature are<br>
/// 1. input : input.class("form-check-input")<br>
/// 2. label : label.class("form-check-label")
pub fn form_inline_radio<F>(mixins: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("col-12")
        .child(html!("div", {
            .class("form-check")
            .apply(mixins)
        }))
    })
}

/// child signature are<br>
/// 1. input : input.class("form-check-input").attr("role","switch") // *MUST* above label<br>
/// 2. label : label.class("form-check-label")
pub fn form_inline_switch<F>(mixins: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("col-12")
        .child(html!("div", {
            .class(class::FORM_CHK_SW)
            .apply(mixins)
        }))
    })
}

pub fn span_with_tooltip<F, G>(mixins_left: F, message: Option<&String>, mixins_right: G) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    G: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("span", {
        .class("app-tooltip-hoverable")
        .apply(mixins_left)
        .apply(|dom| {
            if let Some(msg) = message {
                if msg.len() > 9 {
                    dom.child(html!("span", {
                        .class("ms-1")
                        .children([
                            html!("i", {.class(class::FA_INFO)}),
                            html!("span", {
                                .class("app-tooltip-message")
                                .text(&msg)
                            }),
                        ])
                    }))
                } else {
                    dom.child(html!("span", {
                        .class(class::TXT_BLUE_R)
                        .text(&msg)
                    }))
                }
            } else {
                dom
            }
        })
        .apply(mixins_right)
    })
}

pub fn table_responsive<B, F>(table_class: B, mixins: F) -> Dom
where
    B: MultiStr,
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class("table-responsive")
        .child(html!("table", {
            .class("mb-2")
            .class(table_class)
            .apply(mixins)
        }))
    })
}

/// Box that will `fixed` appear under another `box with id`
pub fn under_box<F>(anchor_rect: DomRect, max_width: f64, max_height: f64, page_y_offset: f64, mixins: F) -> Dom
where
    F: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
{
    html!("div", {
        .class(class::M0_P0)
        .style("position","fixed")
        .style("z-index","3")
        .style_signal("width", window_size().map(move |ws| {
            let width = if ws.width < max_width {ws.width} else {max_width};
            [&width.to_string(),"px"].concat()
        }))
        .style_signal("left", map_ref!{
            let size = window_size(),
            let offset = window_offset() =>
            (*size, *offset)
        }.map(clone!(anchor_rect => move |(ws, wo)| {
            // we assume that starting window.scrollX = 0
            let est_left = anchor_rect.left();
            let width = if ws.width < max_width {ws.width} else {max_width};
            let left = if (est_left + width) < ws.width {
                est_left - wo.x
            } else {
                ws.width - width
            };
            [&left.to_string(),"px"].concat()
        })))
        .style_signal("top", map_ref!{
            let size = window_size(),
            let offset = window_offset() =>
            (*size, *offset)
        }.map(clone!(anchor_rect => move |(ws, wo)| {
            let y_diff = page_y_offset - wo.y;
            let anchor_top = anchor_rect.top();
            let anchor_bottom = anchor_rect.bottom();
            let top = if (anchor_bottom + max_height) < ws.height {
                anchor_bottom + y_diff
            } else if anchor_top > max_height {
                anchor_top - max_height + y_diff
            } else {
                0.0
            };
            [&top.to_string(),"px"].concat()
        })))
        .apply(mixins)
    })
}

//===== ===== =====//
// DateTime Picker //
//===== ===== =====//

/// `DateTime` input with picker from `Mutable<String>`, update `Mutable<bool>` at the end of input/exit<br>
/// - `disable_signal`: ex. `futures_signal::signal::always(false)`
/// - `container_mixin`: ex. `|dom| dom.style("min-width","190px")`, `NOTE`: sm is `175px`)
/// - `label_mixin`: apply to label element, ex. `|dom| dom.class("form-control-sm")`
/// - `input_mixin`: apply to input element, ex. `|dom| dom.class("form-control-sm")`
///
/// `NOTE`: under `input-group` parent
/// - use `d-flex` and `flex-grow-1` classes to `container_mixin`
/// - use `rounded-0` or `rounded-start-0` or `rounded-end-0` class to `label_mixin` and `input_mixin`
pub fn datetime_picker<B, C, D, F, S, T>(
    datetime_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,
    disable_signal: S,
    container_mixin: B,
    label_mixin: C,
    input_mixin: D,
    update_fn: F,
    config_signal: T,
) -> Dom
where
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + Clone + 'static,
    F: Fn(String) -> String + Clone + 'static,
    S: Signal<Item = bool> + 'static,
    T: Signal<Item = Option<PickerConfig<DateConstraints>>> + 'static,
{
    datetime_pickers::datetime_input_with_picker(
        datetime_pickers::Picker::DateTime,
        datetime_mutable,
        changed_mutable,
        disable_signal,
        None,
        container_mixin,
        label_mixin,
        input_mixin,
        update_fn,
        config_signal,
    )
}

/// `Date` input with picker from `Mutable<String>`, update `Mutable<bool>` at the end of input/exit<br>
/// - `disable_signal`: ex. `futures_signal::signal::always(false)`
/// - `paired_mutable`: mutable of paired `Time` for calculate the same constrain
/// - `container_mixin`: ex. `|dom| dom.style("min-width","135px")`, `NOTE`: sm is `120px`)
/// - `label_mixin`: apply to label element, ex. `|dom| dom.class("form-control-sm")`
/// - `input_mixin`: apply to input element, ex. `|dom| dom.class("form-control-sm")`
///
/// `NOTE`: under `input-group` parent
/// - use `d-flex` and `flex-grow-1` classes to `container_mixin`
/// - use `rounded-0` or `rounded-start-0` or `rounded-end-0` class to `label_mixin` and `input_mixin`
pub fn date_picker<B, C, D, F, S, T>(
    date_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,
    disable_signal: S,
    paired_mutable: Option<Mutable<String>>,
    container_mixin: B,
    label_mixin: C,
    input_mixin: D,
    update_fn: F,
    config_signal: T,
) -> Dom
where
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + Clone + 'static,
    F: Fn(String) -> String + Clone + 'static,
    S: Signal<Item = bool> + 'static,
    T: Signal<Item = Option<PickerConfig<DateConstraints>>> + 'static,
{
    datetime_pickers::datetime_input_with_picker(
        datetime_pickers::Picker::Date,
        date_mutable,
        changed_mutable,
        disable_signal,
        paired_mutable,
        container_mixin,
        label_mixin,
        input_mixin,
        update_fn,
        config_signal,
    )
}

/// `Time` input with picker from `Mutable<String>`, update `Mutable<bool>` at the end of input/exit<br>
/// - `disable_signal`: ex. `futures_signal::signal::always(false)`
/// - `paired_mutable`: mutable of paired `Date` for calculate the same constrain
/// - `container_mixin`: ex. `|dom| dom.style("min-width","110px")`, `NOTE`: sm is `95px`)
/// - `label_mixin`: apply to label element, ex. `|dom| dom.class("form-control-sm")`
/// - `input_mixin`: apply to input element, ex. `|dom| dom.class("form-control-sm")`
///
/// `NOTE`: under `input-group` parent
/// - use `d-flex` and `flex-grow-1` classes to `container_mixin`
/// - use `rounded-0` or `rounded-start-0` or `rounded-end-0` class to `label_mixin` and `input_mixin`
pub fn time_picker<B, C, D, F, S, T>(
    time_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,
    disable_signal: S,
    paired_mutable: Option<Mutable<String>>,
    container_mixin: B,
    label_mixin: C,
    input_mixin: D,
    update_fn: F,
    config_signal: T,
) -> Dom
where
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + Clone + 'static,
    F: Fn(String) -> String + Clone + 'static,
    S: Signal<Item = bool> + 'static,
    T: Signal<Item = Option<PickerConfig<DateConstraints>>> + 'static,
{
    datetime_pickers::datetime_input_with_picker(
        datetime_pickers::Picker::Time,
        time_mutable,
        changed_mutable,
        disable_signal,
        paired_mutable,
        container_mixin,
        label_mixin,
        input_mixin,
        update_fn,
        config_signal,
    )
}

//=====//
// DOM //
//=====//

pub fn antibiogram_dropdown(antibiograms: &[Rc<Antibiograms>]) -> Dom {
    html!("li", {
        .class(class::NAV_ITEM_DROP_PY)
        .children([
            html!("a", {
                .class(class::NAV_LINK_DROP_TGL)
                .attr("href", "#")
                .attr("id", "antibiogram-menu")
                .attr("data-bs-toggle", "dropdown")
                .attr("aria-haspopup", "true")
                .attr("aria-expanded", "false")
                .text("Antibiogram")
                .child(html!("i", {.class(class::FA_FILE_R)}))
            }),
            html!("div", {
                .class(class::DROP_MENU_END)
                .attr("aria-labelledby", "antibiogram-menu")
                .children(antibiograms.iter().map(|antibiogram| {
                    html!("a", {
                        .class("dropdown-item")
                        .attr("href", &antibiogram.url)
                        .attr("target", "_blank")
                        .child(html!("i", {.class(class::FA_FILE_L)}))
                        .text(&antibiogram.label)
                    })
                }))
            }),
        ])
    })
}

pub fn badge_count_red(count: usize) -> Option<Dom> {
    (count > 0).then(|| {
        html!("span", {
            .class(class::BADGE_FIX_RT_RED)
            .style("cursor","default")
            .text(&count.to_string())
            // .child(html!("span", {
            //     .class("visually-hidden")
            //     .text("unread messages")
            // }))
        })
    })
}

pub fn badge_count_blue(count: usize) -> Option<Dom> {
    (count > 0).then(|| {
        html!("span", {
            .class(class::BADGE_FIX_RT_BLUE)
            .style("cursor","default")
            .text(&count.to_string())
            // .child(html!("span", {
            //     .class("visually-hidden")
            //     .text("unread messages")
            // }))
        })
    })
}

pub fn badge_info_center(text: &str) -> Dom {
    html!("div", {
        .class(class::COLA_C)
        .child(html!("span", {
            .class(class::BADGE_WRAP_R_CYAN)
            .style("cursor","default")
            .text(text)
        }))
    })
}

/// พบข้อมูล count รายการ, ไม่พบข้อมูล, พบข้อมูลเบื้องต้น limit รายการ
pub fn badge_count_with_limit(count: usize, limit: usize) -> Dom {
    let (cls, txt) = if count == 0 {
        (class::BADGE_WRAP_R_GOLD, String::from("ไม่พบข้อมูล"))
    } else if count >= limit {
        (class::BADGE_WRAP_R_GOLD, ["พบข้อมูลเบื้องต้น ", &limit.to_string(), " รายการ"].concat())
    } else {
        (class::BADGE_WRAP_R_BLUE, ["พบข้อมูล ", &count.to_string(), " รายการ"].concat())
    };
    html!("div", {
        .class(class::COLA_C)
        .child(html!("span", {
            .class(cls)
            .style("cursor","default")
            .text(&txt)
        }))
    })
}

pub fn badge_dchstts(dchstts: &str) -> Dom {
    match dchstts {
        "01" => html!("span", {.class(class::BADGE_TRUNC_GREEN).style("margin-top","1px").style("cursor","default").text("Complete Recovery")}),
        "02" => html!("span", {.class(class::BADGE_TRUNC_CYAN).style("margin-top","1px").style("cursor","default").text("Improved")}),
        "03" => html!("span", {.class(class::BADGE_TRUNC_GOLD).style("margin-top","1px").style("cursor","default").text("Not Improved")}),
        "04" => html!("span", {.class(class::BADGE_TRUNC_GREEN).style("margin-top","1px").style("cursor","default").text("Normal Delivery")}),
        "05" => html!("span", {.class(class::BADGE_TRUNC_CYAN).style("margin-top","1px").style("cursor","default").text("Un-Delivery")}),
        "06" => html!("span", {.class(class::BADGE_TRUNC_GREEN).style("margin-top","1px").style("cursor","default").text("Normal Child Discharged with Mother")}),
        "07" => html!("span", {.class(class::BADGE_TRUNC_GOLD).style("margin-top","1px").style("cursor","default").text("Normal Child Discharged separately")}),
        "08" => html!("span", {.class(class::BADGE_TRUNC_GRAY).style("margin-top","1px").style("cursor","default").text("Dead Still Birth")}),
        "09" => html!("span", {.class(class::BADGE_TRUNC_GRAY).style("margin-top","1px").style("cursor","default").text("Dead")}),
        _ => Dom::empty(),
    }
}

pub fn badge_dchtype(dchtype: &str) -> Dom {
    match dchtype {
        "01" => html!("span", {.class(class::BADGE_TRUNC_GREEN).style("margin-top","1px").style("cursor","default").text("With Approval")}),
        "02" => html!("span", {.class(class::BADGE_TRUNC_GOLD).style("margin-top","1px").style("cursor","default").text("Against Advice")}),
        "03" => html!("span", {.class(class::BADGE_TRUNC_RED).style("margin-top","1px").style("cursor","default").text("By Escape")}),
        "04" => html!("span", {.class(class::BADGE_TRUNC_BLUE).style("margin-top","1px").style("cursor","default").text("By Transfer")}),
        "05" => html!("span", {.class(class::BADGE_TRUNC_CYAN).style("margin-top","1px").style("cursor","default").text("Other")}),
        "08" => html!("span", {.class(class::BADGE_TRUNC_GRAY).style("margin-top","1px").style("cursor","default").text("Dead Autopsy")}),
        "09" => html!("span", {.class(class::BADGE_TRUNC_GRAY).style("margin-top","1px").style("cursor","default").text("Dead Non Autopsy")}),
        _ => Dom::empty(),
    }
}

pub fn badge_score(title: &str, score: u32, color: &'static str, bg_color: &'static str) -> Dom {
    html!("div", {
        .class(class::BADGE_RT_PX2)
        .style("font-size","100%")
        .style("color", color)
        .style("background-color", bg_color)
        .style("cursor","help")
        .style("width","36px")
        .attr("title", title)
        .text(&score.to_string())
    })
}

pub fn badge_score_null(title: &str) -> Dom {
    html!("div", {
        .class(class::BADGE_RT_PX2)
        .class("bg-secondary")
        .style("font-size","100%")
        .style("cursor","help")
        .attr("title", title)
        .child(html!("i", {.class(class::FA_X)}))
    })
}

pub fn badge_scores_and_vs_datetime(scores_opt: &Option<Scores>) -> (Option<PrimitiveDateTime>, Dom, Dom, Dom) {
    scores_opt
        .as_ref()
        .map(|scores| {
            let ews_dom = scores
                .ews
                .score()
                .map(|u| badge_score(scores.ews.label(), u, scores.ews.color_total(), scores.ews.bg_color_total()))
                .unwrap_or(badge_score_null(scores.ews.label()));
            let qsofa_dom = scores
                .qsofa
                .score()
                .map(|u| badge_score(scores.qsofa.label(), u, scores.qsofa.color_total(), scores.qsofa.bg_color_total()))
                .unwrap_or(badge_score_null(scores.qsofa.label()));
            let sirs_dom = scores
                .sirs
                .score()
                .map(|u| badge_score(scores.sirs.label(), u, scores.sirs.color_total(), scores.sirs.bg_color_total()))
                .unwrap_or(badge_score_null(scores.sirs.label()));
            (scores.vs_datetime, ews_dom, qsofa_dom, sirs_dom)
        })
        .unwrap_or((None, badge_score_null("EWS"), badge_score_null("qSOFA"), badge_score_null("SIRS")))
}

// same as IndexAction.had_monitor_status() but return Dom
pub fn had_monitor_status(action: &IndexAction, order_item: &OrderItem, show_only_had: bool) -> Dom {
    match action.last_monitor_status(order_item) {
        (Some(is_abnormal), had) => {
            match had {
                // Complete HAD count/duration
                Some(true) => {
                    if is_abnormal {
                        html!("i", {.class(class::FA_ALERT_RED)})
                    } else {
                        html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)})
                    }
                }
                // Incomplete HAD count/duration
                Some(false) => {
                    if is_abnormal {
                        html!("i", {.class(class::FA_HOURGLASS_RED)})
                    } else {
                        html!("i", {.class(class::FA_HOURGLASS_GREEN)})
                    }
                }
                // No HAD count/duration
                None => {
                    if is_abnormal {
                        html!("i", {.class(class::FA_ALERT_RED)})
                    } else {
                        html!("i", {.class(class::FA_CHECK_GREEN)})
                    }
                }
            }
        }
        (None, had) => {
            if show_only_had {
                Dom::empty()
            } else if had.is_some() {
                html!("i", {.class(class::FA_HOURGLASS_GOLD)})
            } else {
                html!("i", {.class(class::FA_CHECK)})
            }
        }
    }
}

pub fn close_modal_btn() -> Dom {
    html!("button", {
        .attr("type", "button")
        .class(class::BTN_GRAY)
        .attr("data-bs-dismiss", "modal")
        .text("Close")
    })
}

pub fn close_modal_x_btn() -> Dom {
    html!("button", {
        .attr("type", "button")
        .class("btn-close")
        .attr("data-bs-dismiss", "modal")
        .attr("aria-label", "Close")
    })
}

/// use with `<input type="color" list="color-list">`
pub fn color_picker() -> Dom {
    html!("datalist", {
        .attr("id", "color-list")
        .children([
            html!("option", {.attr("value","#000000")}),
            html!("option", {.attr("value","#666666")}),
            html!("option", {.attr("value","#aaaaaa")}),
            html!("option", {.attr("value","#880000")}),
            html!("option", {.attr("value","#ff0000")}),
            html!("option", {.attr("value","#ff00ff")}),
            html!("option", {.attr("value","#ff8800")}),
            html!("option", {.attr("value","#ffff00")}),
            html!("option", {.attr("value","#00ff00")}),
            html!("option", {.attr("value","#008800")}),
            html!("option", {.attr("value","#00ffff")}),
            html!("option", {.attr("value","#0088ff")}),
            html!("option", {.attr("value","#0000ff")}),
            html!("option", {.attr("value","#000088")}),
            html!("option", {.attr("value","#8800ff")}),
        ])
    })
}

// color in circle
pub fn color_prefix_span(color: &str) -> Dom {
    let bg = if color.is_empty() { "inherit" } else { color };
    html!("span", {
        .class(class::CIRCLE_L_PX)
        .style("background-color",bg)
        .text("\u{00a0}")
    })
}

// text in round color box
pub fn color_box_span(color: &str, txt: &str) -> Dom {
    let bg = if color.is_empty() { "inherit" } else { color };
    html!("span", {
        .apply_if(!txt.is_empty(), |dom| dom.class(class::BOX_ROUND_DARKS_BOLD_R_PX3))
        .class("text-dark")
        .style("background-color",bg)
        .text(txt)
    })
}

/// width: 90px
pub fn patient_image(hn: &Option<String>, width: &str) -> Dom {
    html!("img", {
        .class("rounded")
        //.style("vertical-align","top")
        .apply(|dom| {
            let pt_img = if let Some(hn) = &hn {
                [PATH_PREFIX_PATIENT_IMAGE, hn].concat()
            } else {
                DEFAULT_USER_IMAGE.to_owned()
            };
            dom.attr("src", &pt_img)
        })
        .attr("width", width)
        .attr("alt", "รูปผู้ป่วย")
    })
}

pub fn nurse_assign_dropdown(nurse_assign_mutable: Mutable<String>, changed: Mutable<bool>, app: Rc<AppState>) -> Dom {
    html!("div", {
        .class(class::INPUT_GROUP_SM)
        .children([
            doms::span_group_text("Assign"),
            html!("select" => HtmlSelectElement, {
                .class(class::FORM_SELECT_SM)
                .child(html!("option", {.attr("value","").text("ทั้งหมด")}))
                .children(app.nurse_assign_groups().iter().map(|assign| {
                    html!("option", {
                        .attr("value",assign)
                        .text(assign)
                    })
                }))
                .apply(mixins::string_value_select(nurse_assign_mutable.clone(), changed.clone()))
            }),
        ])
    })
}

pub fn order_item_types_radio(order_item_type_mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    html!("div", {
        .class(class::INPUT_GROUP_SM)
        .children([
            doms::span_group_text("ประเภท"),
            order_item_types_btn("", order_item_type_mutable.clone(), changed.clone()),
            order_item_types_btn("injection", order_item_type_mutable.clone(), changed.clone()),
            order_item_types_btn("ivfluid", order_item_type_mutable.clone(), changed.clone()),
            order_item_types_btn("med", order_item_type_mutable.clone(), changed.clone()),
            order_item_types_btn("other", order_item_type_mutable.clone(), changed.clone()),
        ])
    })
}
fn order_item_types_btn(order_item_type: &'static str, order_item_type_mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    let (icon, label) = match order_item_type {
        "injection" => (class::FA_SYRINGE, " ยาฉีด"),
        "ivfluid" => (class::FA_DROPLET, " สารน้ำ"),
        "med" => (class::FA_PILLS, " ยา"),
        "other" => (class::FA_HAND_MED, " อื่นๆ"),
        _ => (class::FA_LIST_CHECK, " ทั้งหมด"),
    };
    html!("button", {
        .class(class::BTN_SM_BLUEO)
        .class_signal("active", order_item_type_mutable.signal_cloned().map(move |t| t == order_item_type))
        .attr("type", "button")
        .attr("data-bs-toggle", "button")
        .child(html!("i", {.class(icon)}))
        .text(label)
        .event(move |_: events::Click| {
            order_item_type_mutable.set(order_item_type.to_owned());
            changed.set(true);
         })
    })
}

pub fn is_discharged_radio(is_discharged_mutable: Mutable<String>, changed: Mutable<bool>, app: Rc<AppState>) -> Dom {
    html!("div", {
        .class(class::INPUT_GROUP_SM)
        .children([
            html!("span", {.class("input-group-text").text("สถานะ")}),
            is_discharged_btn("", is_discharged_mutable.clone(), changed.clone(), app.clone()),
            is_discharged_btn("N", is_discharged_mutable.clone(), changed.clone(), app.clone()),
            is_discharged_btn("Y", is_discharged_mutable.clone(), changed.clone(), app.clone()),
        ])
    })
}
fn is_discharged_btn(is_discharged: &'static str, is_discharged_mutable: Mutable<String>, changed: Mutable<bool>, app: Rc<AppState>) -> Dom {
    let (icon, label) = match is_discharged {
        "Y" => (class::FA_HOUSE, " จำหน่ายแล้ว"),
        "N" => (class::FA_BED, " รักษา"),
        _ => (class::FA_LIST_CHECK, " ทั้งหมด"),
    };
    html!("button", {
        .class(class::BTN_SM_BLUEO)
        .class_signal("active", is_discharged_mutable.signal_cloned().map(move |t| t == is_discharged))
        .attr("type", "button")
        .attr("data-bs-toggle", "button")
        .child(html!("i", {.class(icon)}))
        .text(label)
        .event(move |_: events::Click| {
            is_discharged_mutable.set(is_discharged.to_owned());
            app.to_local_storage();
            changed.set(true);
         })
    })
}

pub fn index_plan_status_radio(status: Mutable<Option<String>>, status_changed: Mutable<bool>) -> Dom {
    html!("div", {
        .class(class::INPUT_GROUP_SM)
        .attr("role","group")
        .attr("aria-label","Status radio toggle button group")
        .children([
            html!("span", {.class("input-group-text").text("สถานะ")}),
            html!("input" => HtmlInputElement, {
                .attr("type", "radio")
                .class("btn-check")
                .attr("id", "status-all")
                .attr("autocomplete","off")
                .apply(mixins::radio_opt_match_or_none(status.clone(), status_changed.clone(), "all"))
            }),
            html!("label", {
                .class(class::BTN_SM_BLUEO)
                .attr("for", "status-all")
                .child(html!("i", {.class(class::FA_LIST_CHECK)}))
                .text(" ทั้งหมด")
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "radio")
                .class("btn-check")
                .attr("id", "status-wait")
                .attr("autocomplete","off")
                .apply(mixins::radio_opt_match(status.clone(), status_changed.clone(), "wait"))
            }),
            html!("label", {
                .class(class::BTN_SM_BLUEO)
                .attr("for", "status-wait")
                .child(html!("i", {.class(class::FA_HOURGLASS)}))
                .text(" รอในเวร")
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "radio")
                .class("btn-check")
                .attr("id", "status-missed")
                .attr("autocomplete","off")
                .apply(mixins::radio_opt_match(status.clone(), status_changed.clone(), "missed"))
            }),
            html!("label", {
                .class(class::BTN_SM_BLUEO)
                .attr("for", "status-missed")
                .child(html!("i", {.class(class::FA_ALERT_GOLD)}))
                .text(" พลาด/ผิดเวลา")
            }),
            html!("input" => HtmlInputElement, {
                .attr("type", "radio")
                .class("btn-check")
                .attr("id", "status-done")
                .attr("autocomplete","off")
                .apply(mixins::radio_opt_match(status.clone(), status_changed.clone(), "done"))
            }),
            html!("label", {
                .class(class::BTN_SM_BLUEO)
                .attr("for", "status-done")
                .child(html!("i", {.class(class::FA_CHECK_CIRCLE_BLUE)}))
                .text(" เรียบร้อย/OFF")
            }),
        ])
    })
}

pub fn status_btn(status: AuditStatus, status_mutable: Mutable<AuditStatus>, changed: Mutable<bool>) -> Dom {
    html!("button", {
        .class(["btn", status.btn_class()])
        .class_signal("active", status_mutable.signal_cloned().map(clone!(status => move |st| st == status)))
        .attr("type", "button")
        .attr("data-bs-toggle", "button")
        .text(status.status_text())
        .event(move |_: events::Click| {
            if status_mutable.get_cloned() != status {
                status_mutable.set(status.clone());
                changed.set_neq(true);
            }
         })
    })
}

/// label with `form-check-label` for id
pub fn label_check_for(id: &str, text: &str) -> Dom {
    html!("label", {
        .class("form-check-label")
        .attr("for", id)
        .style("user-select","none")
        .style("white-space","nowrap")
        .text(text)
    })
}

/// label with `form-check-label` for id
pub fn label_check_for_selectable(id: &str, text: &str) -> Dom {
    html!("label", {
        .class("form-check-label")
        .attr("for", id)
        .text(text)
    })
}

/// label with `input-group-text` for id
pub fn label_group_for(id: &str, text: &str) -> Dom {
    html!("label", {
        .class("input-group-text")
        .attr("for", id)
        .text(text)
    })
}

/// div class="form-group row" with inner label class="col-sm-12"
pub fn label_row_12(text: &str) -> Dom {
    html!("div", {
        .class(class::ROW)
        .child(html!("label", {
            .class("col-sm-12")
            .child(html!("b", {.text(text)}))
        }))
    })
}

pub fn nav_item_external_url(url: &str, label: &str) -> Dom {
    html!("li", {
        .class(class::NAV_ITEM_PY)
        .child(html!("a", {
            .class("nav-link")
            .attr("href", url)
            .attr("rel","noopener noreferrer")
            .attr("target","_blank")
            .text(label)
            .child(html!("i", { .class(class::FA_EXT_LINK)}))
        }))
    })
}

// SelectUtils::getSelectOptionFromArray
pub fn select_option(option: &SelectOption, selected_value: &str) -> Dom {
    html!("option", {
        .attr("value", &option.key.to_owned())
        .apply_if(!selected_value.is_empty() && selected_value == option.key, |dom| { dom
            .attr("selected","")
        })
        .text(&option.value)
    })
}

// SelectUtils::getColorSelectOptionFromArray
pub fn select_option_color(option: &ColorSelectOption, selected_value: &str) -> Dom {
    html!("option", {
        .attr("value", &option.key)
        .apply_if(!option.color.is_empty(), |dom| { dom
            .class("fw-bold")
            .attr("data-color", &option.color)
            .style("color","black")
            .style("background-color", &option.color)
        })
        .apply_if(!selected_value.is_empty() && selected_value == option.key, |dom| { dom
            .attr("selected","")
        })
        .text(&option.value)
    })
}

/// span with `input-group-text` with id
pub fn span_group_id(id: &str, text: &str) -> Dom {
    html!("span", {
        .class("input-group-text")
        .attr("id", id)
        .text(text)
    })
}

/// span with `input-group-text`
pub fn span_group_text(text: &str) -> Dom {
    html!("span", {
        .class("input-group-text")
        .text(text)
    })
}

/// convert text to Dom list with "[XXX]" in red
pub fn square_bracket_to_span(input: &str) -> impl Iterator<Item = Dom> {
    input.split('[').flat_map(|s| {
        let brace = s.split(']').collect::<Vec<&str>>();
        if brace.len() == 2 {
            vec![html!("span",{.class(class::BOLD_RED).text(brace[0])}), text(brace[1])]
        } else {
            vec![text(s)]
        }
    })
}

pub fn timer_svg(timer_second: Mutable<f32>) -> Dom {
    svg!("svg", {
        .attr("width", "20")
        .attr("height", "20")
        .attr("viewBox", "0 0 24 24")
        .children([
            svg!("circle", {
                .attr("cx", "12")
                .attr("cy", "12")
                .attr("r", "12")
                .attr("fill", "#CCC")
            }),
            svg!("path", {
                .attr("transform", "translate(12, 12) scale(.9)")
                .attr("fill", "#333")
                .attr_signal("d", timer_second.signal_cloned().map(|s| {
                    let zeta = s * std::f32::consts::PI / 30.0;
                    let x = zeta.sin() * 12.0;
                    let y = zeta.cos() * (-12.0);
                    let mid = if s > 30.0 {1} else {0};
                    ["M 0 0 v -12 A 12 12 1 ", &mid.to_string(), " 1 ", &x.to_string(), " ", &y.to_string(), " z"].concat()
                }))
            }),
        ])
    })
}

pub fn radio_container(mutable: Mutable<String>, changed: Mutable<bool>, id: &str, value: &'static str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "radio")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::radio_match(mutable, changed, value))
    })
}

/// disabled this input when by_not != by_not_value
pub fn radio_disable_by_not_container(mutable: Mutable<String>, by_not: Mutable<String>, by_not_value: &'static str, changed: Mutable<bool>, id: &str, value: &'static str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "radio")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::radio_match(mutable, changed, value))
        .apply(mixins::other_not_match_disable(by_not, by_not_value))
    })
}

/// if radio is != value -> all of texts are ""
pub fn radio_binding_texts_container(mutable: Mutable<String>, texts: Vec<Mutable<String>>, changed: Mutable<bool>, id: &str, value: &'static str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "radio")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::radio_match(mutable.clone(), changed, value))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes != value {
                for text in texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if radio is != value -> all of texts are ""<br>
/// and disabled this input when by_not != by_not_value
pub fn radio_binding_texts_disable_by_not_container(
    mutable: Mutable<String>,
    texts: Vec<Mutable<String>>,
    by_not: Mutable<String>,
    by_not_value: &'static str,
    changed: Mutable<bool>,
    id: &str,
    value: &'static str,
) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "radio")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::radio_match(mutable.clone(), changed, value))
        .apply(mixins::other_not_match_disable(by_not, by_not_value))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes != value {
                for text in texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if radio is value -> all of texts are ""
pub fn radio_toggle_texts_container(mutable: Mutable<String>, texts: Vec<Mutable<String>>, changed: Mutable<bool>, id: &str, value: &'static str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "radio")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::radio_match(mutable.clone(), changed, value))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes == value {
                for text in texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if radio is value -> all of texts are ""
pub fn radio_binding_toggle_texts_container(
    mutable: Mutable<String>,
    binding_texts: Vec<Mutable<String>>,
    toggle_texts: Vec<Mutable<String>>,
    changed: Mutable<bool>,
    id: &str,
    value: &'static str,
) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "radio")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::radio_match(mutable.clone(), changed, value))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes == value {
                for text in toggle_texts.iter() {
                    text.set_neq(String::new());
                }
            } else {
                for text in binding_texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

pub fn checkbox_container(mutable: Mutable<String>, changed: Mutable<bool>, id: &str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable, changed, "Y",""))
    })
}

/// disabled this input when by_not != by_not_value
pub fn checkbox_disable_by_not_container(mutable: Mutable<String>, by_not: Mutable<String>, by_not_value: &'static str, changed: Mutable<bool>, id: &str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable, changed, "Y",""))
        .apply(mixins::other_not_match_disable(by_not, by_not_value))
    })
}

/// if checkbox is "" -> all of texts are ""
pub fn checkbox_binding_texts_container(mutable: Mutable<String>, texts: Vec<Mutable<String>>, changed: Mutable<bool>, id: &str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable.clone(), changed, "Y",""))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes.is_empty() {
                for text in texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if checkbox is "" -> all of texts are ""<br>
/// and disabled this input when by_not != by_not_value
pub fn checkbox_binding_texts_disable_by_not_container(
    mutable: Mutable<String>,
    texts: Vec<Mutable<String>>,
    by_not: Mutable<String>,
    by_not_value: &'static str,
    changed: Mutable<bool>,
    id: &str,
) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable.clone(), changed, "Y",""))
        .apply(mixins::other_not_match_disable(by_not, by_not_value))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes.is_empty() {
                for text in texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if checkbox is "ํY" -> all of texts are ""
pub fn checkbox_toggle_texts_container(mutable: Mutable<String>, texts: Vec<Mutable<String>>, changed: Mutable<bool>, id: &str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable.clone(), changed, "Y",""))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes == "Y" {
                for text in texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if checkbox is "ํY" -> all of texts are ""<br>
/// and disabled this input when by_not != by_not_value
pub fn checkbox_toggle_texts_disable_by_not_container(
    mutable: Mutable<String>,
    texts: Vec<Mutable<String>>,
    by_not: Mutable<String>,
    by_not_value: &'static str,
    changed: Mutable<bool>,
    id: &str,
) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable.clone(), changed, "Y",""))
        .apply(mixins::other_not_match_disable(by_not, by_not_value))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes == "Y" {
                for text in texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if checkbox is "ํY" -> all of binding_texts are ""<br>
/// if checkbox is "ํ" -> all of toggle_texts are ""
pub fn checkbox_binding_toggle_texts_container(mutable: Mutable<String>, binding_texts: Vec<Mutable<String>>, toggle_texts: Vec<Mutable<String>>, changed: Mutable<bool>, id: &str) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable.clone(), changed, "Y",""))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes == "Y" {
                for text in toggle_texts.iter() {
                    text.set_neq(String::new());
                }
            } else {
                for text in binding_texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

/// if checkbox is "ํY" -> all of binding_texts are ""<br>
/// if checkbox is "ํ" -> all of toggle_texts are ""<br>
/// and disabled this input when by_not != by_not_value
pub fn checkbox_binding_toggle_texts_disable_by_not_container(
    mutable: Mutable<String>,
    binding_texts: Vec<Mutable<String>>,
    toggle_texts: Vec<Mutable<String>>,
    by_not: Mutable<String>,
    by_not_value: &'static str,
    changed: Mutable<bool>,
    id: &str,
) -> Dom {
    html!("input" => HtmlInputElement, {
        .attr("type", "checkbox")
        .class("form-check-input")
        .attr("id", id)
        .apply(mixins::checkbox_toggle(mutable.clone(), changed, "Y",""))
        .apply(mixins::other_not_match_disable(by_not, by_not_value))
        .future(mutable.signal_cloned().for_each(move |yes| {
            if yes == "Y" {
                for text in toggle_texts.iter() {
                    text.set_neq(String::new());
                }
            } else {
                for text in binding_texts.iter() {
                    text.set_neq(String::new());
                }
            }
            async {}
        }))
    })
}

pub fn checkbox_not_empty_with_text_container(mutable: Mutable<String>, changed: Mutable<bool>, id: &str, label: &str) -> Vec<Dom> {
    vec![
        html!("div", {
            .class("form-check")
            .children([
                html!("input" => HtmlInputElement, {
                    .attr("type", "checkbox")
                    .class("form-check-input")
                    .attr("id", id)
                    .apply(mixins::checkbox_not_empty(mutable.clone(), changed.clone()))
                }),
                label_check_for(id, label),
            ])
        }),
        html!("input" => HtmlInputElement, {
            .attr("type", "text")
            .class(class::FORM_CTRL_SM)
            .apply(mixins::string_value(mutable.clone(), changed))
            .apply(mixins::other_empty_disable(mutable))
        }),
    ]
}

pub fn checkbox_not_empty_texts_with_text_container(mutable: Mutable<String>, texts: Vec<Mutable<String>>, changed: Mutable<bool>, id: &str, label: &str) -> Vec<Dom> {
    vec![
        html!("div", {
            .class("form-check")
            .children([
                html!("input" => HtmlInputElement, {
                    .attr("type", "checkbox")
                    .class("form-check-input")
                    .attr("id", id)
                    .apply(mixins::checkbox_not_empty(mutable.clone(), changed.clone()))
                    .future(mutable.signal_cloned().for_each(move |yes| {
                        if !yes.is_empty() {
                            for text in texts.iter() {
                                text.set_neq(String::new());
                            }
                        }
                        async {}
                    }))
                }),
                label_check_for(id, label),
            ])
        }),
        html!("input" => HtmlInputElement, {
            .attr("type", "text")
            .class(class::FORM_CTRL_SM)
            .apply(mixins::string_value(mutable.clone(), changed))
            .apply(mixins::other_empty_disable(mutable))
        }),
    ]
}

pub fn texts_container(mutable: Mutable<String>, changed: Mutable<bool>, class: &str, max_length: Option<u32>) -> Dom {
    html!("div", {
        .class(class)
        .child(html!("div", {
            .class("row")
            .child(html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .apply(|d| if let Some(max_len) = max_length {
                    d.attr("maxlength", &max_len.to_string())
                } else {
                    d
                })
                .class(class::FORM_CTRL_SM)
                .apply(mixins::string_value(mutable, changed))
            }))
        }))
    })
}

pub fn text_disable_by_not_container(mutable: Mutable<String>, by_not: Mutable<String>, by_not_value: &'static str, changed: Mutable<bool>, class: &str, max_length: Option<u32>) -> Dom {
    html!("div", {
        .class(class)
        .child(html!("div", {
            .class("row")
            .child(html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .apply(|d| if let Some(max_len) = max_length {
                    d.attr("maxlength", &max_len.to_string())
                } else {
                    d
                })
                .class(class::FORM_CTRL_SM)
                .apply(mixins::string_value(mutable, changed))
                .apply(mixins::other_not_match_disable(by_not, by_not_value))
            }))
        }))
    })
}

pub fn textarea_disable_by_not_container(mutable: Mutable<String>, by_not: Mutable<String>, by_not_value: &'static str, changed: Mutable<bool>, class: &str, max_length: Option<u32>) -> Dom {
    html!("div", {
        .class(class)
        .child(html!("div", {
            .class("row")
            .child(html!("textarea" => HtmlTextAreaElement, {
                .attr("rows","3")
                .apply(|d| if let Some(max_len) = max_length {
                    d.attr("maxlength", &max_len.to_string())
                } else {
                    d
                })
                .class("form-control")
                .apply(mixins::string_value(mutable, changed))
                .apply(mixins::other_not_match_disable(by_not, by_not_value))
            }))
        }))
    })
}

pub fn render_avatar(row: Rc<AvatarEnum>, search_selected: Mutable<Option<VisitTypeId>>, app: Rc<AppState>) -> Dom {
    let (hn_opt, visit_type, detail, is_dc) = match row.as_ref() {
        AvatarEnum::Ipd(avatar) => {
            let an = avatar.an.clone();
            let visit_type = if an.len() == app.hosxp_an_len() { VisitTypeId::Ipd(an) } else { VisitTypeId::PreAdmit(an) };
            (
                avatar.hn.clone(),
                visit_type,
                html!("div", {
                    .class("float-start")
                    .children([
                        html!("div", {
                            .children([
                                html!("span", {.class("fw-bold").text("เตียง: ")}),
                                html!("span", {.text(&avatar.bedno.clone().unwrap_or_default())}),
                            ])
                        }),
                        html!("div", {
                            .class("text-truncate")
                            .children([
                                html!("span", {.class("fw-bold").text("AN: ")}),
                                html!("span", {.text(&avatar.an)}),
                            ])
                        }),
                        html!("div", {
                            .children([
                                html!("span", {.class("fw-bold").text("HN: ")}),
                                html!("span", {.text(&avatar.hn.clone().unwrap_or_default())}),
                            ])
                        }),
                        html!("div", {
                            .class(class::TRUNC_BOLD)
                            .text(&avatar.pname.clone().unwrap_or_default())
                        }),
                    ])
                }),
                avatar.discharge_order_exists,
            )
        }
        AvatarEnum::OpdEr(avatar) => (
            avatar.hn.clone(),
            VisitTypeId::OpdEr(avatar.vn.clone().unwrap_or_default(), avatar.opd_er_order_master_id),
            html!("div", {
                .class("float-start")
                .children([
                    html!("div", {
                        .children([
                            html!("span", {.class("fw-bold").text("เตียง: ")}),
                            html!("span", {
                                .class(class::BADGE_TB_C)
                                .style("cursor","default")
                                .style("font-size","100%")
                                .apply_if(avatar.bed_type_color.is_some(), |dom| {
                                    dom.style("background-color", avatar.bed_type_color.clone().unwrap_or_default())
                                })
                                .text(&[avatar.bed_type_name.clone().unwrap_or_default(), avatar.display_bedno.clone().unwrap_or_default()].join(" "))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("text-truncate")
                        .children([
                            html!("span", {.class("fw-bold").text("VN: ")}),
                            html!("span", {.text(&avatar.vn.clone().unwrap_or_default())}),
                        ])
                    }),
                    html!("div", {
                        .children([
                            html!("span", {.class("fw-bold").text("HN: ")}),
                            html!("span", {.text(&avatar.hn.clone().unwrap_or_default())}),
                        ])
                    }),
                    html!("div", {
                        .class(class::TRUNC_BOLD)
                        .text(&avatar.pname.clone().unwrap_or_default())
                    }),
                ])
            }),
            false,
        ),
    };

    html!("tr", {
        .style_signal("border-right-color", search_selected.signal_cloned().map(clone!(visit_type => move |opt| {
            if opt.as_ref().map(|ss| ss.vnan() == visit_type.vnan()).unwrap_or_default() {
                "red"
            } else {
                "inherit"
            }
        })))
        .style_signal("border-right-width", search_selected.signal_cloned().map(clone!(visit_type => move |opt| {
            if opt.as_ref().map(|ss| ss.vnan() == visit_type.vnan()).unwrap_or_default() {
                "5px"
            } else {
                "inherit"
            }
        })))
        .child(html!("td", {
            .style("position", "relative")
            .style("cursor", "pointer")
            .apply_if(is_dc, |dom| {
                dom.class("bg-info-subtle")
                .child(html!("span", {
                    .class(class::BADGE_FIX_T_CYAN)
                    .style("right", "10px")
                    .text("D/C")
                }))
            })
            .children([
                html!("div", {
                    .class(class::FLOAT_L)
                    .child(doms::patient_image(&hn_opt, "80px"))
                }),
                detail,
            ])
        }))
        .event(move |_: events::Click| {
            search_selected.set(Some(visit_type.clone()));
        })
    })
}

pub fn td_icon_value_u8_opt_match(mutable: Mutable<Option<u8>>, score: u8) -> Dom {
    html!("td", {
        .class(class::TXT_C_P0)
        .style("vertical-align", "middle")
        .child(html!("i", {
            .class(class::FA_CHECK_CIRCLE_GREEN)
            .style("font-size", "32px")
            .style_signal("visibility", mutable.signal().map(move |opt| {
                if opt == Some(score) {
                    "visible"
                } else {
                    "hidden"
                }
            }))
        }))
        .event(move |_:events::Click| {
            mutable.set(Some(score));
        })
    })
}

pub fn td_text_value_u8_opt_match(mutable: Mutable<Option<u8>>, colspan: &str, is_center: bool, score: u8, title: &str) -> Dom {
    html!("td", {
        .apply_if(is_center, |d| d
            .class("text-center")
            .style("vertical-align", "middle")
        )
        .style("cursor", "pointer")
        .style_signal("background-color", mutable.signal().map(move |s| {
            if s == Some(score) {
                "#E0FFFF"
            } else {
                "inherit"
            }
        }))
        .attr("colspan", colspan)
        .text(title)
        .event(move |_:events::Click| {
            mutable.set(Some(score));
        })
    })
}
