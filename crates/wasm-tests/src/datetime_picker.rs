use time::Date;
use time_datepicker_core::{
    dialog_view_type::DialogViewType,
    utils::from_ymd,
    viewed_date::{DayNumber, MonthNumber, YearNumber},
};
use wasm_bindgen_test::*;

use kphis_ui_core::datetime_pickers::picker::create_dialog_title_text;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn create_date(year: YearNumber, month: MonthNumber, day: DayNumber) -> Date {
    from_ymd(year, month, day)
}

#[wasm_bindgen_test]
fn test_create_dialog_title_text() {
    assert_eq!("มกราคม 2533", create_dialog_title_text(&DialogViewType::Days, &create_date(1990, 1, 1)));
    assert_eq!("2533", create_dialog_title_text(&DialogViewType::Months, &create_date(1990, 1, 1)));
    assert_eq!("2520 - 2539", create_dialog_title_text(&DialogViewType::Years, &create_date(1990, 1, 1)));
}
