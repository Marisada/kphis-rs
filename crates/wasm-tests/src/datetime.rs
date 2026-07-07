use futures_signals::signal::Mutable;
use time::{
    Date, Month,
    macros::{date, datetime, time},
};
use wasm_bindgen_test::*;

use kphis_util::{
    datetime::{date_from_pat, date_pat, datetime_from_pat, datetime_pat, js_now, time_from_pat, time_pat},
    util::{set_day_last, set_days_next},
};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_date_to_pat() {
    assert_eq!(date_pat(&date!(2022 - 01 - 30)), String::from("30/01/2565"));
}

#[wasm_bindgen_test]
pub fn test_time_to_pat() {
    assert_eq!(time_pat(&time!(14:55)), String::from("14:55"));
}

#[wasm_bindgen_test]
pub fn test_datetime_to_pat() {
    assert_eq!(datetime_pat(&datetime!(2022-01-30 14:55)), String::from("30/01/2565 14:55"));
}

#[wasm_bindgen_test]
pub fn test_date_from_pat() {
    let decate = ((js_now().year() + 543) / 100) * 100;
    // DD/MM/YYYY'
    assert_eq!(date_from_pat("30/01/2565"), Some(date!(2022 - 01 - 30)));
    assert_eq!(date_from_pat("30/01/65"), Date::from_calendar_date(decate + 65 - 543, Month::January, 30).ok());
    assert_eq!(date_from_pat("2/1/65"), Date::from_calendar_date(decate + 65 - 543, Month::January, 2).ok());
    assert_eq!(date_from_pat("30012565"), Some(date!(2022 - 01 - 30)));
    assert_eq!(date_from_pat("300165"), Date::from_calendar_date(decate + 65 - 543, Month::January, 30).ok());
    // MM/DD/YYYY
    assert_eq!(date_from_pat("11/13/2565"), Some(date!(2022 - 11 - 13)));
    assert_eq!(date_from_pat("11/13/65"), Date::from_calendar_date(decate + 65 - 543, Month::November, 13).ok());
    assert_eq!(date_from_pat("11132565"), Some(date!(2022 - 11 - 13)));
    assert_eq!(date_from_pat("111365"), Date::from_calendar_date(decate + 65 - 543, Month::November, 13).ok());
    // seperators
    assert_eq!(date_from_pat("30-01-2565"), Some(date!(2022 - 01 - 30)));
    assert_eq!(date_from_pat("30.01.2565"), Some(date!(2022 - 01 - 30)));
    assert_eq!(date_from_pat("30:01:2565"), Some(date!(2022 - 01 - 30)));
    assert_eq!(date_from_pat("30.01:2565"), Some(date!(2022 - 01 - 30)));
    assert_eq!(date_from_pat("30x01z2565"), Some(date!(2022 - 01 - 30)));
    // overflow
    assert_eq!(date_from_pat("300125659"), Some(date!(2022 - 01 - 30)));
    assert_eq!(date_from_pat("3001659"), Date::from_calendar_date(decate + 65 - 543, Month::January, 30).ok());
    // failed
    assert_eq!(date_from_pat("40/1/2565"), None);
    assert_eq!(date_from_pat("13/13/2565"), None);
    assert_eq!(date_from_pat("131313"), None);
    assert_eq!(date_from_pat("12121"), None);
}

#[wasm_bindgen_test]
pub fn test_time_from_pat() {
    assert_eq!(time_from_pat("14:55"), Some(time!(14:55)));
    assert_eq!(time_from_pat("2:55"), Some(time!(02:55)));
    assert_eq!(time_from_pat("2:5"), Some(time!(02:05)));
    assert_eq!(time_from_pat("1455"), Some(time!(14:55)));
    assert_eq!(time_from_pat("025"), Some(time!(00:25))); // not 02:05
    assert_eq!(time_from_pat("125"), Some(time!(01:25))); // not 12:05
    assert_eq!(time_from_pat("235"), Some(time!(02:35))); // not 23:05
    assert_eq!(time_from_pat("245"), Some(time!(02:45)));
    assert_eq!(time_from_pat("345"), Some(time!(03:45)));
    assert_eq!(time_from_pat("066"), Some(time!(06:06)));
    assert_eq!(time_from_pat("166"), Some(time!(16:06)));
    assert_eq!(time_from_pat("02"), Some(time!(00:02)));
    assert_eq!(time_from_pat("10"), Some(time!(10:00)));
    assert_eq!(time_from_pat("23"), Some(time!(23:00)));
    assert_eq!(time_from_pat("24"), Some(time!(02:04)));
    assert_eq!(time_from_pat("99"), Some(time!(09:09)));
    assert_eq!(time_from_pat("0"), Some(time!(00:00)));
    // seperators
    assert_eq!(time_from_pat("14-55"), Some(time!(14:55)));
    assert_eq!(time_from_pat("14/55"), Some(time!(14:55)));
    assert_eq!(time_from_pat("14.55"), Some(time!(14:55)));
    assert_eq!(time_from_pat("14x55"), Some(time!(14:55)));
    // overflow
    assert_eq!(time_from_pat("14559"), Some(time!(14:55)));
    // failed
    assert_eq!(time_from_pat("25:55"), None);
    assert_eq!(time_from_pat("14:65"), None);
    assert_eq!(time_from_pat("9999"), None);
    assert_eq!(time_from_pat("999"), None);
    assert_eq!(time_from_pat(""), None);
}

#[wasm_bindgen_test]
pub fn test_datetime_from_pat() {
    assert_eq!(datetime_from_pat("30/01/2565 14:55"), Some(datetime!(2022-01-30 14:55)));
    // failed
    assert_eq!(datetime_from_pat("30/01/2565T14:55"), None);
}

#[wasm_bindgen_test]
pub fn test_set_day_last() {
    let start_date_mutable = Mutable::new(String::new());
    let end_date_mutable = Mutable::new(String::new());
    let changed_mutable = Mutable::new(false);

    // no start_date, no last_date, day = 0
    set_day_last(None, None, start_date_mutable.clone(), end_date_mutable.clone(), changed_mutable.clone(), 0);
    assert_eq!(start_date_mutable.get_cloned(), js_now().date().to_string());
    assert_eq!(end_date_mutable.get_cloned(), js_now().date().to_string());
    assert!(changed_mutable.get());

    // fixed start_date, no last_date, day = 0
    set_day_last(Some(date!(2024 - 01 - 01)), None, start_date_mutable.clone(), end_date_mutable.clone(), changed_mutable.clone(), 0);
    assert_eq!(start_date_mutable.get_cloned(), String::from("2024-01-01"));
    assert_eq!(end_date_mutable.get_cloned(), js_now().date().to_string());

    // no start_date, fixed last_date, day = 0
    set_day_last(None, Some(date!(2024 - 01 - 30)), start_date_mutable.clone(), end_date_mutable.clone(), changed_mutable.clone(), 0);
    assert_eq!(start_date_mutable.get_cloned(), js_now().date().to_string());
    assert_eq!(end_date_mutable.get_cloned(), String::from("2024-01-30"));

    // fixed start_date/end_date
    set_day_last(
        Some(date!(2024 - 01 - 01)),
        Some(date!(2024 - 01 - 30)),
        start_date_mutable.clone(),
        end_date_mutable.clone(),
        changed_mutable.clone(),
        10,
    );
    assert_eq!(start_date_mutable.get_cloned(), String::from("2024-01-21"));
    assert_eq!(end_date_mutable.get_cloned(), String::from("2024-01-30"));
}

#[wasm_bindgen_test]
pub fn test_set_days_next() {
    let start_date_mutable = Mutable::new(String::new());
    let end_date_mutable = Mutable::new(String::new());
    let changed_mutable = Mutable::new(false);

    // no start_date, no last_date, forward = false
    set_days_next(start_date_mutable.clone(), end_date_mutable.clone(), changed_mutable.clone(), false);
    assert_eq!(start_date_mutable.get_cloned(), String::new());
    assert_eq!(end_date_mutable.get_cloned(), String::new());
    assert!(!changed_mutable.get());

    start_date_mutable.set(String::from("2024-01-01"));

    // has start_date, no last_date, forward = false
    set_days_next(start_date_mutable.clone(), end_date_mutable.clone(), changed_mutable.clone(), false);
    assert_eq!(start_date_mutable.get_cloned(), String::from("2024-01-01"));
    assert_eq!(end_date_mutable.get_cloned(), String::new());
    assert!(!changed_mutable.get());

    end_date_mutable.set(String::from("2024-01-10"));

    // has start_date, has last_date, forward = true
    set_days_next(start_date_mutable.clone(), end_date_mutable.clone(), changed_mutable.clone(), true);
    assert_eq!(start_date_mutable.get_cloned(), String::from("2024-01-11"));
    assert_eq!(end_date_mutable.get_cloned(), String::from("2024-01-20"));
    assert!(changed_mutable.get());

    // has start_date, has last_date, forward = false
    set_days_next(start_date_mutable.clone(), end_date_mutable.clone(), changed_mutable.clone(), false);
    assert_eq!(start_date_mutable.get_cloned(), String::from("2024-01-01"));
    assert_eq!(end_date_mutable.get_cloned(), String::from("2024-01-10"));
    assert!(changed_mutable.get());
}
