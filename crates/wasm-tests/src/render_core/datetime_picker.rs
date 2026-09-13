use dominator::Dom;
use futures_signals::signal::Mutable;
use time::Date;
use time_datepicker_core::{
    dialog_view_type::DialogViewType,
    utils::from_ymd,
    viewed_date::{DayNumber, MonthNumber, YearNumber},
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

use kphis_ui_core::datetime_pickers::picker::create_dialog_title_text;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub async fn replace_body(dom: Dom) {
    dominator::replace_dom(
        &dominator::body(),
        &dominator::body().first_child().unwrap(),
        dom,
    );
    // move to next tick
    JsFuture::from(js_sys::Promise::resolve(&JsValue::null())).await.unwrap();
}

fn create_date(year: YearNumber, month: MonthNumber, day: DayNumber) -> Date {
    from_ymd(year, month, day)
}

#[wasm_bindgen_test]
fn test_create_dialog_title_text() {
    assert_eq!("มกราคม 2533", create_dialog_title_text(&DialogViewType::Days, &create_date(1990, 1, 1)));
    assert_eq!("2533", create_dialog_title_text(&DialogViewType::Months, &create_date(1990, 1, 1)));
    assert_eq!("2520 - 2539", create_dialog_title_text(&DialogViewType::Years, &create_date(1990, 1, 1)));
}

#[wasm_bindgen_test]
async fn test_datetime_picker() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_header() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_header(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_date_footer() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_date_footer(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_time_footer() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_time_footer(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_dialog_years() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_dialog_years(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_dialog_months() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_dialog_months(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_dialog_days() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_dialog_days(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_dialog_hours() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_dialog_hours(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_datetime_picker_dialog_minutes() {
    let config = time_datepicker_core::config::PickerConfigBuilder::default().build().unwrap();
    let cpn = kphis_ui_core::datetime_pickers::picker::DatePicker::new_datetime(Mutable::new(String::from("2022-12-31")), Mutable::new(false), Mutable::new(None), |s| s, config);
    let dom = kphis_ui_core::datetime_pickers::picker::DatePicker::render_dialog_minutes(cpn);
    replace_body(dom).await;
}
