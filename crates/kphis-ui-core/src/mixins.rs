use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, or},
    signal_vec::{MutableVec, SignalVecExt},
};
use js_sys::JsString;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{DomParser, EventTarget, HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, SupportedType};

use kphis_model::{
    A4_WIDTH,
    app::{AppState, DragStartState},
    report::TypstSvg,
};

pub trait TextInput {
    fn is_textarea(&self) -> bool;
    fn set_disabled(&self, value: bool);
    fn value(&self) -> String;
    fn selection_start(&self) -> Result<Option<u32>, JsValue>;
    fn selection_end(&self) -> Result<Option<u32>, JsValue>;
    fn set_value(&self, value: &str);
    fn set_selection_start(&self, value: Option<u32>) -> Result<(), JsValue>;
    fn set_selection_end(&self, value: Option<u32>) -> Result<(), JsValue>;
}

impl TextInput for HtmlInputElement {
    fn is_textarea(&self) -> bool {
        false
    }
    #[inline]
    fn set_disabled(&self, value: bool) {
        self.set_disabled(value)
    }
    #[inline]
    fn value(&self) -> String {
        self.value()
    }
    #[inline]
    fn selection_start(&self) -> Result<Option<u32>, JsValue> {
        self.selection_start()
    }
    #[inline]
    fn selection_end(&self) -> Result<Option<u32>, JsValue> {
        self.selection_end()
    }
    #[inline]
    fn set_value(&self, value: &str) {
        self.set_value(value)
    }
    #[inline]
    fn set_selection_start(&self, value: Option<u32>) -> Result<(), JsValue> {
        self.set_selection_start(value)
    }
    #[inline]
    fn set_selection_end(&self, value: Option<u32>) -> Result<(), JsValue> {
        self.set_selection_end(value)
    }
}
impl TextInput for HtmlSelectElement {
    fn is_textarea(&self) -> bool {
        false
    }
    #[inline]
    fn set_disabled(&self, value: bool) {
        self.set_disabled(value)
    }
    #[inline]
    fn value(&self) -> String {
        self.value()
    }
    #[inline]
    fn selection_start(&self) -> Result<Option<u32>, JsValue> {
        Ok(None)
    }
    #[inline]
    fn selection_end(&self) -> Result<Option<u32>, JsValue> {
        Ok(None)
    }
    #[inline]
    fn set_value(&self, value: &str) {
        self.set_value(value)
    }
    #[inline]
    fn set_selection_start(&self, _value: Option<u32>) -> Result<(), JsValue> {
        Ok(())
    }
    #[inline]
    fn set_selection_end(&self, _value: Option<u32>) -> Result<(), JsValue> {
        Ok(())
    }
}
impl TextInput for HtmlTextAreaElement {
    fn is_textarea(&self) -> bool {
        true
    }
    #[inline]
    fn set_disabled(&self, value: bool) {
        self.set_disabled(value)
    }
    #[inline]
    fn value(&self) -> String {
        self.value()
    }
    #[inline]
    fn selection_start(&self) -> Result<Option<u32>, JsValue> {
        self.selection_start()
    }
    #[inline]
    fn selection_end(&self) -> Result<Option<u32>, JsValue> {
        self.selection_end()
    }
    #[inline]
    fn set_value(&self, value: &str) {
        self.set_value(value)
    }
    #[inline]
    fn set_selection_start(&self, value: Option<u32>) -> Result<(), JsValue> {
        self.set_selection_start(value)
    }
    #[inline]
    fn set_selection_end(&self, value: Option<u32>) -> Result<(), JsValue> {
        self.set_selection_end(value)
    }
}

pub trait CanDisable {
    fn set_disabled(&self, value: bool);
}

impl CanDisable for HtmlButtonElement {
    #[inline]
    fn set_disabled(&self, value: bool) {
        self.set_disabled(value)
    }
}
impl CanDisable for HtmlInputElement {
    #[inline]
    fn set_disabled(&self, value: bool) {
        self.set_disabled(value)
    }
}
impl CanDisable for HtmlSelectElement {
    #[inline]
    fn set_disabled(&self, value: bool) {
        self.set_disabled(value)
    }
}
impl CanDisable for HtmlTextAreaElement {
    #[inline]
    fn set_disabled(&self, value: bool) {
        self.set_disabled(value)
    }
}

//===============//
//  not builder  //
//===============//

/// changed is true when Mutable != value
pub fn with_string(value: &str, mutable: Mutable<String>, changed: Mutable<bool>) {
    let neq = mutable.lock_ref().as_str() != value;
    if neq {
        mutable.set(value.to_owned());
        changed.set_neq(true);
    }
}

/// - changed is true when Mutable != value
/// - changed is false when value is empty
pub fn with_string_not_empty(value: &str, mutable: Mutable<String>, changed: Mutable<bool>) {
    let neq = mutable.lock_ref().as_str() != value;
    if neq {
        mutable.set(value.to_owned());
    }
    changed.set_neq(!value.is_empty());
}

// not allow Some("")
/// - Mutable is Some(not keyword) + empty value  => Mutable is None + changed is true
/// - Mutable is Some(not keyword) + not empty value  => Mutable is Some(keyword) + changed is true
/// - Mutable is Some(keyword) => do nothing
/// - Mutable is None + empty value => do nothing
/// - Mutable is None + not empty value => Mutable is Some(keyword) + changed is true
pub fn with_string_opt(value: &str, mutable: Mutable<Option<String>>, changed: Mutable<bool>) {
    let neq = match mutable.lock_ref().as_ref().map(|s| s.as_str()) {
        Some(s) => s != value,
        None => !value.is_empty(),
    };
    if neq {
        // we cannot use str_some fn because it trim() internally
        mutable.set((!value.is_empty()).then_some(value.to_owned()));
        changed.set_neq(true);
    }
}

// allow Some("")
/// - Mutable is Some(not keyword) => Mutable is Some(keyword) + changed is true
/// - Mutable is Some(keyword) => do nothing
/// - Mutable is None => Mutable is Some(keyword) + changed is true
pub fn with_string_opt_exact(value: &str, mutable: Mutable<Option<String>>, changed: Mutable<bool>) {
    let eq = mutable.lock_ref().as_ref().map(|s| s.as_str() == value).unwrap_or_default();
    if !eq {
        mutable.set(Some(value.to_owned()));
        changed.set_neq(true);
    }
}

//================//
//  text builder  //
//================//

// 'enter' => prevent_default + update changed
// 'tab' => prevent_default + insert space
// default behavior
// - input:     [x]'enter' [ ]'tab'
// - select:    [x]'enter' [ ]'tab'
// - textarea:  [ ]'enter' [x]'tab'
fn tab_action<T: TextInput>(element: &T) {
    if let Ok(Some(start)) = element.selection_start() {
        let end = element.selection_end().unwrap_or(Some(start)).unwrap_or_default() as usize;
        let value = element.value();
        let left = value.chars().take(start as usize).collect::<String>();
        let right = value.chars().skip(end).collect::<String>();
        element.set_value(&[left, right].join("    "));
        match element.set_selection_start(Some(start + 4)) {
            Err(e) => {
                let message = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("cannot set_selection_start"));
                log::error!("{}", message);
            }
            Ok(()) => {
                if let Err(e) = element.set_selection_end(Some(start + 4)) {
                    let message = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("cannot set_selection_end"));
                    log::error!("{}", message);
                }
            }
        }
    }
}

/// update (mutable + changed) every Input
pub fn string_value<T>(mutable: Mutable<String>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .prop_signal("value", mutable.signal_cloned())
            .event(clone!(element, mutable, changed => move |_: events::Input| with_string(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if element.is_textarea() && event.key() == "Tab" {
                    event.prevent_default();
                    tab_action(&element);
                    with_string(&element.value(), mutable.clone(), changed.clone())
                }
            })
        })
    }
}

/// update (mutable + changed) every Input
pub fn opt_string_value<T>(mutable: Mutable<Option<String>>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .prop_signal("value", mutable.signal_cloned().map(|v| v.unwrap_or_default()))
            .event(clone!(element, mutable, changed => move |_: events::Input| with_string_opt(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if element.is_textarea() && event.key() == "Tab" {
                    event.prevent_default();
                    tab_action(&element);
                    with_string_opt(&element.value(), mutable.clone(), changed.clone());
                }
            })
        })
    }
}

/// - update (mutable + changed) every Input
/// - changed is false when value is empty
pub fn string_value_not_empty<T>(mutable: Mutable<String>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .prop_signal("value", mutable.signal_cloned())
            .event(clone!(element, mutable, changed => move |_: events::Input| with_string_not_empty(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if element.is_textarea() && event.key() == "Tab" {
                    event.prevent_default();
                    tab_action(&element);
                    with_string_not_empty(&element.value(), mutable.clone(), changed.clone());
                }
            })
        })
    }
}

/// - update (mutable + changed) every Input
/// - display nothing when mutable has a matched value
pub fn opt_string_match_show_empty<T>(mutable: Mutable<Option<String>>, changed: Mutable<bool>, keyword: &'static str) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .prop_signal("value", mutable.signal_cloned().map(|v| {
                match v {
                    Some(s) => if s == *keyword {
                        String::new()
                    } else {
                        s
                    }
                    None => String::new(),
                }
            }))
            .event(clone!(element, mutable, changed => move |_: events::Input| with_string_opt(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if element.is_textarea() && event.key() == "Tab" {
                    event.prevent_default();
                    tab_action(&element);
                    with_string_opt(&element.value(), mutable.clone(), changed.clone())
                }
            })
        })
    }
}

/// - update (mutable + changed) when key 'enter' or lost focus
/// - not update (mutable + changed) every KeyDown
pub fn string_value_end(mutable: Mutable<String>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .prop_signal("value", mutable.signal_cloned())
            .event(clone!(element, mutable, changed => move |_: events::Change| with_string(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if event.key() == "Enter" {
                    event.prevent_default();
                    with_string(&element.value(), mutable.clone(), changed.clone());
                }
            })
        })
    }
}

/// - update (mutable + changed) when selected
pub fn string_value_select(mutable: Mutable<String>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlSelectElement>) -> DomBuilder<HtmlSelectElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .prop_signal("value", mutable.signal_cloned())
            .event(clone!(element, mutable, changed => move |_: events::Change| with_string(&element.value(), mutable.clone(), changed.clone())))
        })
    }
}

/// - update (mutable + changed) when key 'enter' or lost focus
/// - not update (mutable + changed) every KeyDown
pub fn opt_string_value_end(mutable: Mutable<Option<String>>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .prop_signal("value", mutable.signal_cloned().map(|v| v.unwrap_or_default()))
            .event(clone!(element, mutable, changed => move |_: events::Change| with_string_opt(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if !element.is_textarea() && event.key() == "Enter" {
                    event.prevent_default();
                    with_string_opt(&element.value(), mutable.clone(), changed.clone());
                }
            })
        })
    }
}

/// update (mutable + changed) every Input
pub fn textarea_value_auto_expand(mutable: Mutable<String>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlTextAreaElement>) -> DomBuilder<HtmlTextAreaElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .style("overflow","hidden")
            .prop_signal("value", mutable.signal_cloned())
            .style_signal("height", mutable.signal_ref(clone!(element => move |_| {
                element.style().set_property("height", "auto").unwrap();
                let min_height = 38 + ((element.rows() - 1) * 24) as i32;
                let scroll_height = element.scroll_height();
                let height = if scroll_height < min_height {min_height} else {scroll_height};
                [&height.to_string(), "px"].concat()
            })))
            .event(clone!(element, mutable, changed => move |_: events::Input| with_string(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if event.key() == "Tab" {
                    event.prevent_default();
                    tab_action(&element);
                    with_string(&element.value(), mutable.clone(), changed.clone())
                }
            })
        })
    }
}

/// update (mutable + changed) every Input
pub fn textarea_opt_value_auto_expand(mutable: Mutable<Option<String>>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlTextAreaElement>) -> DomBuilder<HtmlTextAreaElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .style("overflow","hidden")
            .prop_signal("value", mutable.signal_cloned().map(|opt| opt.unwrap_or_default()))
            .style_signal("height", mutable.signal_ref(clone!(element => move |_| {
                element.style().set_property("height", "auto").unwrap();
                let min_height = 38 + ((element.rows() - 1) * 24) as i32;
                let scroll_height = element.scroll_height();
                let height = if scroll_height < min_height {min_height} else {scroll_height};
                [&height.to_string(), "px"].concat()
            })))
            .event(clone!(element, mutable, changed => move |_: events::Input| with_string_opt(&element.value(), mutable.clone(), changed.clone())))
            .event_with_options(&EventOptions::preventable(), move |event: events::KeyDown| {
                if event.key() == "Tab" {
                    event.prevent_default();
                    tab_action(&element);
                    with_string_opt(&element.value(), mutable.clone(), changed.clone())
                }
            })
        })
    }
}

//====================//
//  disabled builder  //
//====================//

/// this input element will disabled when other signal is true
pub fn other_true_signal_disable<T, S>(other: S) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: CanDisable + std::clone::Clone + 'static,
    S: Signal<Item = bool> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.for_each(move |v| {
                element.set_disabled(v);
                async {}
            }))
        })
    }
}

/// this input element will disabled when other is true
pub fn other_true_disable<T>(other: Mutable<bool>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.signal().for_each(move |v| {
                element.set_disabled(v);
                async {}
            }))
        })
    }
}

// /// this input element will disabled when other is keyword or empty
// pub fn other_match_disable<T>(
//     b: DomBuilder<T>,
//     other: Mutable<String>,
//     keyword: &'static str,
// ) -> DomBuilder<T>
// where   T: TextInput +
//             std::clone::Clone +
//             std::convert::AsRef<wasm_bindgen::JsValue> +
//             std::convert::AsRef<web_sys::EventTarget> + 'static
// {
//     with_node!(b, element => {
//         .future(other.signal_cloned().for_each(clone!(element => move |v| {
//             if v == *keyword {
//                 element.set_disabled(true);
//             } else {
//                 element.set_disabled(false);
//             }
//             async {}
//         })))
//     })
// }

/// this input element will disabled when other is NOT keyword
pub fn other_not_match_disable<T>(other: Mutable<String>, keyword: &'static str) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.signal_cloned().for_each(move |v| {
                element.set_disabled(v != keyword);
                async {}
            }))
        })
    }
}

/// this input element will disabled when other is NOT keyword, if forced=true will always disabled
pub fn other_not_match_disable_or_forced<T>(other: Mutable<String>, keyword: &'static str, forced: bool) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.signal_cloned().for_each(move |v| {
                element.set_disabled(forced || v != keyword);
                async {}
            }))
        })
    }
}

/// this input element will disabled when other is keyword or empty
pub fn other_match_empty_disable<T>(other: Mutable<String>, keyword: &'static str) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.signal_cloned().for_each(move |v| {
                element.set_disabled(v == *keyword || v.is_empty());
                async {}
            }))
        })
    }
}

/// this input element will disabled when other is None
pub fn other_none_disable<T>(other: Mutable<Option<String>>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.signal_cloned().for_each(move |v| {
                element.set_disabled(v.is_none());
                async {}
            }))
        })
    }
}

/// this input element will disabled when other is Empty
pub fn other_empty_disable<T>(other: Mutable<String>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.signal_cloned().for_each(move |v| {
                element.set_disabled(v.is_empty());
                async {}
            }))
        })
    }
}

/// this input element will disabled when other is Some(keyword) or None
pub fn other_match_none_disable<T>(other: Mutable<Option<String>>, keyword: &'static str) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: TextInput + std::clone::Clone + std::convert::AsRef<wasm_bindgen::JsValue> + std::convert::AsRef<web_sys::EventTarget> + 'static,
{
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(other.signal_cloned().for_each(move |v| {
                element.set_disabled(v == Some(String::from(keyword)) || v.is_none());
                async {}
            }))
        })
    }
}

//====================//
//  checkbox builder  //
//====================//

pub fn checkbox_toggle(mutable: Mutable<String>, changed: Mutable<bool>, yes: &'static str, no: &'static str) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(v == yes);
                async {}
            })))
            .event(move |_: events::Change| {
                if element.checked() {
                    mutable.set_neq(String::from(yes));
                } else {
                    mutable.set_neq(String::from(no));
                }
                changed.set_neq(true);
            })
        })
    }
}

pub fn checkbox_bool(mutable: Mutable<bool>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal().for_each(clone!(element => move |v| {
                element.set_checked(v);
                async {}
            })))
            .event(move |_: events::Change| {
                mutable.set_neq(element.checked());
                changed.set_neq(true);
            })
        })
    }
}

pub fn checkbox_some_bool(mutable: Mutable<Option<bool>>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal().for_each(clone!(element => move |opt| {
                if let Some(v) = opt {
                    element.set_checked(v);
                }
                async {}
            })))
            .event(move |_: events::Change| {
                mutable.set_neq(Some(element.checked()));
                changed.set_neq(true);
            })
        })
    }
}

pub fn checkbox_some(mutable: Mutable<Option<String>>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(v.is_some());
                async {}
            })))
            .event(move |_: events::Change| {
                if element.checked() {
                    mutable.set_neq(Some(String::new()));
                } else {
                    mutable.set_neq(None);
                }
                changed.set(true);
            })
        })
    }
}

pub fn checkbox_not_empty(mutable: Mutable<String>, changed: Mutable<bool>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(!v.is_empty());
                async {}
            })))
            .event(move |_: events::Change| {
                if element.checked() {
                    mutable.set_neq(String::from(" "));
                } else {
                    mutable.set_neq(String::new());
                }
                changed.set(true);
            })
        })
    }
}

//=================//
//  radio builder  //
//=================//

pub fn radio_match(mutable: Mutable<String>, changed: Mutable<bool>, keyword: &'static str) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(v == keyword);
                async {}
            })))
            .event(move |_: events::Click| {
                with_string(keyword, mutable.clone(), changed.clone())
            })
        })
    }
}

pub fn radio_match_empty(mutable: Mutable<String>, changed: Mutable<bool>, keyword: &'static str) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(v == keyword || v.is_empty());
                async {}
            })))
            .event(move |_: events::Click| {
                with_string(keyword, mutable.clone(), changed.clone())
            })
        })
    }
}

pub fn radio_opt_match(mutable: Mutable<Option<String>>, changed: Mutable<bool>, keyword: &'static str) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(v == Some(String::from(keyword)));
                async {}
            })))
            .event(move |_: events::Click| {
                with_string_opt(keyword, mutable.clone(), changed.clone())
            })
        })
    }
}

/// checked when mutable is Some(keyword) or None
pub fn radio_opt_match_or_none(mutable: Mutable<Option<String>>, changed: Mutable<bool>, keyword: &'static str) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(v == Some(String::from(keyword)) || v.is_none());
                async {}
            })))
            .event(move |_: events::Click| {
                with_string_opt(keyword, mutable.clone(), changed.clone())
            })
        })
    }
}

/// checked when mutable is Some(not keyword) and set Mutable with Some(String::new())
pub fn radio_opt_match_some_neq(mutable: Mutable<Option<String>>, changed: Mutable<bool>, keyword: &'static str) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .future(mutable.signal_cloned().for_each(clone!(element => move |v| {
                element.set_checked(v.is_some() && v != Some(String::from(keyword)));
                async {}
            })))
            .event(move |_: events::Click| {
                // Some(keyword) or None is eq
                let eq = mutable.lock_ref().as_ref().map(|s| s.as_str() == keyword).unwrap_or(true);
                if eq {
                    mutable.set(Some(String::new()));
                    changed.set_neq(true);
                }
            })
        })
    }
}

//==================//
//  button builder  //
//==================//

/// - disabled when loader_is_loading
/// - Click event will check loader_is_loading before call f
/// - NOT use with `other_true_signal_disable` USE
///   `click_with_loader_checked_or_true_disable_signal` below instead
pub fn click_with_loader_checked<F, E>(f: F, app: Rc<AppState>) -> impl FnOnce(DomBuilder<E>) -> DomBuilder<E>
where
    F: Fn() + std::clone::Clone + 'static,
    E: CanDisable + std::convert::AsRef<EventTarget> + std::clone::Clone + 'static,
{
    #[inline]
    move |dom| {
        dom.apply(other_true_signal_disable(app.loader_is_loading())).apply(|dom| {
            let act_mut = Mutable::new(false);
            dom.event(clone!(act_mut => move |_: events::Click| {
                act_mut.set_neq(true);
            }))
            .future(
                map_ref! {
                    let busy = app.loader_is_loading(),
                    let act = act_mut.signal() =>
                    !busy && *act
                }
                .for_each(clone!(f, act_mut => move |ready| {
                    if ready {
                        act_mut.set(false);
                        f();
                    }
                    async {}
                })),
            )
        })
    }
}

/// - disabled when `loader_is_loading` is true OR `true_disable_signal` is true
/// - Click event will check loader_is_loading before call f
pub fn click_with_loader_checked_or_true_disable_signal<F, S, E>(f: F, true_disable_signal: S, app: Rc<AppState>) -> impl FnOnce(DomBuilder<E>) -> DomBuilder<E>
where
    F: Fn() + std::clone::Clone + 'static,
    S: Signal<Item = bool> + 'static,
    E: CanDisable + std::convert::AsRef<EventTarget> + std::clone::Clone + 'static,
{
    #[inline]
    move |dom| {
        dom.apply(other_true_signal_disable(or(app.loader_is_loading(), true_disable_signal))).apply(|dom| {
            let act_mut = Mutable::new(false);
            dom.event(clone!(act_mut => move |_: events::Click| {
                act_mut.set_neq(true);
            }))
            .future(
                map_ref! {
                    let busy = app.loader_is_loading(),
                    let act = act_mut.signal() =>
                    !busy && *act
                }
                .for_each(clone!(f, act_mut => move |ready| {
                    if ready {
                        act_mut.set(false);
                        f();
                    }
                    async {}
                })),
            )
        })
    }
}

pub fn typst_svg_mixins<T>(report_width_percent: Mutable<f64>, report_svg: MutableVec<Rc<TypstSvg>>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
where
    T: wasm_bindgen::JsCast + std::convert::AsRef<web_sys::Node> + std::convert::AsRef<web_sys::HtmlElement> + Clone + 'static,
{
    #[inline]
    move |dom| {
        dom.style_signal(
            "width",
            map_ref! {
                let report_width_percent = report_width_percent.signal_cloned(),
                let max_width = report_svg.signal_vec_cloned().to_signal_cloned().map(|reports| reports.iter().max_by_key(|i| i.width as u64).map(|i| i.width).unwrap_or(A4_WIDTH)) =>
                report_width_percent * max_width / 100.0
            }
            .map(move |width| {
                let w = width.ceil() as u32;
                [&w.saturating_sub(18).to_string(), "px"].concat()
            }),
        )
        .children_signal_vec(report_svg.signal_vec_cloned().map(|typst_svg| {
            // we use DomParser instead of innerHTML to prevent injection
            let doc = DomParser::new().unwrap().parse_from_string(&typst_svg.svg, SupportedType::ImageSvgXml).unwrap();
            let child = doc
                .document_element()
                .and_then(|svg| {
                    // tagName of XML dom is not upper-case
                    (svg.tag_name().as_str() == "svg").then(|| Dom::new(svg.into()))
                })
                .unwrap_or(Dom::empty());
            html!("div", {
                .class("p-1") // show page's border with grey padding
                .style("width", "100%")
                .child(child)
            })
        }))
    }
}

//===============//
//  Drag'n Drop  //
//===============//

/// for one-way drag-and-drop `from` this element
/// - use with AppState.drag_start_state
pub fn drag_start_only(drag_start_state: Mutable<Option<DragStartState>>) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .event(move |event: events::DragStart| {
                drag_start_state.set(Some(DragStartState::Text(element.text_content().unwrap_or_default())));
                if let Some(data_transfer) = event.data_transfer() {
                    data_transfer.set_data("text", &element.text_content().unwrap_or_default()).unwrap();
                }
            })
        })
    }
}
