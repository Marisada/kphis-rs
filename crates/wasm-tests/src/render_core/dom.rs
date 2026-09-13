use dominator::{Dom, html};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use kphis_model::app::AppState;

use crate::replace_body;

#[wasm_bindgen_test]
async fn test_alert_row() {
    let dom = kphis_ui_core::doms::alert_row(|d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_form_inline() {
    let dom = kphis_ui_core::doms::form_inline(|d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_form_inline_group() {
    let dom = kphis_ui_core::doms::form_inline_group(|d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_form_inline_group_sm() {
    let dom = kphis_ui_core::doms::form_inline_group_sm(|d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_form_inline_end() {
    let dom = kphis_ui_core::doms::form_inline_end(|d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_form_inline_radio() {
    let dom = kphis_ui_core::doms::form_inline_radio(|d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_form_inline_switch() {
    let dom = kphis_ui_core::doms::form_inline_switch(|d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_span_with_tooltip() {
    let dom = kphis_ui_core::doms::span_with_tooltip(|d| d.class("ms-1"), Some(&String::from("message")), |d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_table_responsive() {
    let dom = kphis_ui_core::doms::table_responsive(["text-center"], |d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_under_box() {
    let dom = kphis_ui_core::doms::under_box(web_sys::DomRect::new().unwrap(), 400.0, 200.0, 200.0, |d| d.class("ms-1"));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_datetime_picker() {
    let dom = kphis_ui_core::doms::datetime_picker(Mutable::new(String::from("2023-12-31T23:59:59")), Mutable::new(false), Mutable::new(false).signal(), |d| d, |d| d, |d| d, |s| s, Mutable::new(None).signal_cloned());
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_date_picker() {
    let dom = kphis_ui_core::doms::date_picker(Mutable::new(String::from("2023-12-31")), Mutable::new(false), Mutable::new(false).signal(), Some(Mutable::new(String::from("23:59:59"))), |d| d, |d| d, |d| d, |s| s, Mutable::new(None).signal_cloned());
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_time_picker() {
    let dom = kphis_ui_core::doms::time_picker(Mutable::new(String::from("23:59:59")), Mutable::new(false), Mutable::new(false).signal(), Some(Mutable::new(String::from("2023-12-31"))), |d| d, |d| d, |d| d, |s| s, Mutable::new(None).signal_cloned());
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_antibiogram_dropdown() {
    let dom = kphis_ui_core::doms::antibiogram_dropdown(&[Rc::new(kphis_model::antibiogram::Antibiograms {label: String::from("/"), url: String::from("ATB")})]);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_badge_count_red() {
    let dom = kphis_ui_core::doms::badge_count_red(1);
    replace_body(dom.unwrap_or(Dom::empty())).await;
}

#[wasm_bindgen_test]
async fn test_badge_count_blue() {
    let dom = kphis_ui_core::doms::badge_count_blue(1);
    replace_body(dom.unwrap_or(Dom::empty())).await;
}

#[wasm_bindgen_test]
async fn test_badge_info_center() {
    let dom = kphis_ui_core::doms::badge_info_center("text");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_badge_count_with_limit() {
    let dom = kphis_ui_core::doms::badge_count_with_limit(9, 10);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_badge_dchstts() {
    let dom = kphis_ui_core::doms::badge_dchstts("09");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_badge_dchtype() {
    let dom = kphis_ui_core::doms::badge_dchtype("09");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_badge_score() {
    let dom = kphis_ui_core::doms::badge_score("title", 1, "red", "white");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn badge_score_null() {
    let dom = kphis_ui_core::doms::badge_score_null("title");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn badge_had_monitor_status() {
    let dom = kphis_ui_core::doms::had_monitor_status(&kphis_model::index_action::IndexAction::demo(), &kphis_model::order::OrderItem::demo(), false);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn badge_color_picker() {
    let dom = kphis_ui_core::doms::color_picker();
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn badge_color_prefix_span() {
    let dom = kphis_ui_core::doms::color_prefix_span("red");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn badge_color_box_span() {
    let dom = kphis_ui_core::doms::color_box_span("red", "99px");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_patient_image() {
    let dom = kphis_ui_core::doms::patient_image(&Some(String::from("0001234")), "333px");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_nurse_assign_dropdown() {
    let app_state = AppState::new_from_local_storage("/");
    let dom = kphis_ui_core::doms::nurse_assign_dropdown(Mutable::new(String::from("leader")), Mutable::new(false), app_state);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_order_item_types_radio() {
    let dom = kphis_ui_core::doms::order_item_types_radio(Mutable::new(String::from("injection")), Mutable::new(false));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_is_discharged_radio() {
    let app_state = AppState::new_from_local_storage("/");
    let dom = kphis_ui_core::doms::is_discharged_radio(Mutable::new(String::from("Y")), Mutable::new(false), app_state);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_index_plan_status_radio() {
    let dom = kphis_ui_core::doms::index_plan_status_radio(Mutable::new(Some(String::from("wait"))), Mutable::new(false));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_status_btn() {
    let dom = kphis_ui_core::doms::status_btn(kphis_model::ipd::summary::AuditStatus::Audit, Mutable::new(kphis_model::ipd::summary::AuditStatus::Audit), Mutable::new(false));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_label_check_for() {
    let dom = kphis_ui_core::doms::label_check_for("label", "text");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_label_check_for_selectable() {
    let dom = kphis_ui_core::doms::label_check_for_selectable("label", "text");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_label_group_for() {
    let dom = kphis_ui_core::doms::label_group_for("label", "text");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_label_row_12() {
    let dom = kphis_ui_core::doms::label_row_12("text");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_nav_item_external_url() {
    let dom = kphis_ui_core::doms::nav_item_external_url("/", "label");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_select_option() {
    let dom = kphis_ui_core::doms::select_option(&kphis_model::select_utils::SelectOption::demo(), "Item1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_select_option_color() {
    let dom = kphis_ui_core::doms::select_option_color(&kphis_model::select_utils::ColorSelectOption::demo(), "Item1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_span_group_id() {
    let dom = kphis_ui_core::doms::span_group_id("1", "text");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_span_group_text() {
    let dom = kphis_ui_core::doms::span_group_text("text");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_square_bracket_to_span() {
    let dom = kphis_ui_core::doms::square_bracket_to_span("text");
    replace_body(html!("div", {.children(dom.collect::<Vec<Dom>>())})).await;
}

#[wasm_bindgen_test]
async fn test_timer_svg() {
    let dom = kphis_ui_core::doms::timer_svg(Mutable::new(1.0));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_radio_container() {
    let dom = kphis_ui_core::doms::radio_container(Mutable::new(String::from("1")), Mutable::new(false), "id", "1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_radio_disable_by_not_container() {
    let dom = kphis_ui_core::doms::radio_disable_by_not_container(Mutable::new(String::from("1")), Mutable::new(String::from("2")), "2", Mutable::new(false), "id", "1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_radio_binding_texts_container() {
    let dom = kphis_ui_core::doms::radio_binding_texts_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(false), "id", "1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_radio_binding_texts_disable_by_not_container() {
    let dom = kphis_ui_core::doms::radio_binding_texts_disable_by_not_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(String::from("2")), "2", Mutable::new(false), "id", "1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_radio_toggle_texts_container() {
    let dom = kphis_ui_core::doms::radio_toggle_texts_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(false), "id", "1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_radio_binding_toggle_texts_container() {
    let dom = kphis_ui_core::doms::radio_binding_toggle_texts_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], vec![Mutable::new(String::from("text"))], Mutable::new(false), "id", "1");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_container() {
    let dom = kphis_ui_core::doms::checkbox_container(Mutable::new(String::from("1")), Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_disable_by_not_container() {
    let dom = kphis_ui_core::doms::checkbox_disable_by_not_container(Mutable::new(String::from("1")), Mutable::new(String::from("2")), "2", Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_binding_texts_container() {
    let dom = kphis_ui_core::doms::checkbox_binding_texts_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_binding_texts_disable_by_not_container() {
    let dom = kphis_ui_core::doms::checkbox_binding_texts_disable_by_not_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(String::from("2")), "2", Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_toggle_texts_container() {
    let dom = kphis_ui_core::doms::checkbox_toggle_texts_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_toggle_texts_disable_by_not_container() {
    let dom = kphis_ui_core::doms::checkbox_toggle_texts_disable_by_not_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(String::from("2")), "2", Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_binding_toggle_texts_container() {
    let dom = kphis_ui_core::doms::checkbox_binding_toggle_texts_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], vec![Mutable::new(String::from("text"))], Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_binding_toggle_texts_disable_by_not_container() {
    let dom = kphis_ui_core::doms::checkbox_binding_toggle_texts_disable_by_not_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], vec![Mutable::new(String::from("text"))], Mutable::new(String::from("2")), "2", Mutable::new(false), "id");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_not_empty_with_text_container() {
    let doms = kphis_ui_core::doms::checkbox_not_empty_with_text_container(Mutable::new(String::from("1")), Mutable::new(false), "id", "label");
    replace_body(html!("div", {.children(doms)})).await;
}

#[wasm_bindgen_test]
async fn test_checkbox_not_empty_texts_with_text_container() {
    let doms = kphis_ui_core::doms::checkbox_not_empty_texts_with_text_container(Mutable::new(String::from("1")), vec![Mutable::new(String::from("text"))], Mutable::new(false), "id", "label");
    replace_body(html!("div", {.children(doms)})).await;
}

#[wasm_bindgen_test]
async fn test_texts_container() {
    let dom = kphis_ui_core::doms::texts_container(Mutable::new(String::from("1")), Mutable::new(false), "ms-1", Some(9));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_text_disable_by_not_container() {
    let dom = kphis_ui_core::doms::text_disable_by_not_container(Mutable::new(String::from("1")), Mutable::new(String::from("2")), "2", Mutable::new(false), "ms-1", Some(9));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_textarea_disable_by_not_container() {
    let dom = kphis_ui_core::doms::textarea_disable_by_not_container(Mutable::new(String::from("1")), Mutable::new(String::from("2")), "2", Mutable::new(false), "ms-1", Some(9));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_render_avatar() {
    let app_state = AppState::new_from_local_storage("/");
    let dom = kphis_ui_core::doms::render_avatar(Rc::new(kphis_model::avatar::AvatarEnum::Ipd(kphis_model::avatar::AvatarWard::demo())), Mutable::new(Some(kphis_model::app::VisitTypeId::Ipd(String::from("660001234")))), app_state);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_td_icon_value_u8_opt_match() {
    let dom = kphis_ui_core::doms::td_icon_value_u8_opt_match(Mutable::new(Some(1)), 1);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_td_text_value_u8_opt_match() {
    let dom = kphis_ui_core::doms::td_text_value_u8_opt_match(Mutable::new(Some(1)), "3", true, 1, "title");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_seelct_box() {
    let dom = kphis_ui_core::doms::select_box("id", Some("ทั้งหมด"), false, Mutable::new(String::from("1")), Mutable::new(false), |d| d.class("ms-1"), || {}, vec![kphis_model::select_utils::SelectOption::demo()]);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_seelct_box_multi() {
    let dom = kphis_ui_core::doms::select_box("id", Some("ทั้งหมด"), true, Mutable::new(String::from("1")), Mutable::new(false), |d| d.class("ms-1"), || {}, vec![kphis_model::select_utils::SelectOption::demo()]);
    replace_body(dom).await;
}
