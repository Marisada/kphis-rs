use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{rc::Rc, sync::Arc};
use web_sys::HtmlInputElement;

use kphis_drg_worker::{
    drg::model::I10vx,
    i10::{
        claml::{I10Detail, Rubric, RubricKind, UsageKind},
        index::{Code, I10Pointer, NeoplasmTag, Note},
    },
};
use kphis_model::{search::searchbox::Icd10, timer::Timeout};
use kphis_ui_app::{App, DaggerAsteriskState, DaggerAsteriskStatus};
use kphis_ui_core::{class, doms, draggable::Group, mixins};
use kphis_util::util::{find_first_icd10, first_char_uppercase, icd_dash, icd10_dot, is_icd10_resemble};

use super::{red_keywords_in_icd_dot, red_keywords_in_sentense};

#[derive(Clone, Default)]
enum InputMode {
    #[default]
    Manual,
    ExactSearch,
    IndexSearch,
}

#[derive(Default)]
pub struct DxSearchboxCpn {
    is_external_cause: bool,
    /// - full : allow all input mode, has separate search inputs, update parent element
    /// - not full : for searching only, no manual mode, use raw input for searching
    is_full_feature: bool,
    input_mode: Mutable<InputMode>,
    dagger_asterisk_state: Rc<DaggerAsteriskState>,
    dagger_asterisk_status: Mutable<DaggerAsteriskStatus>,

    icd10_raw: Mutable<String>,
    diagnosis_raw: Mutable<String>,
    raw_changed: Mutable<bool>,

    is_searching: Mutable<bool>,
    is_indexing: Mutable<bool>,
    timer_handle: Mutable<Option<i32>>,

    show_search_content: Mutable<bool>,
    search_text: Mutable<String>,
    search_results: MutableVec<(Arc<I10vx>, f32, u8)>,
    search_status_text: Mutable<Option<String>>,
    search_selected_result: Mutable<Option<Rc<Icd10>>>,

    show_index_content: Mutable<bool>,
    is_substance: Mutable<bool>,
    index_text: Mutable<String>,
    index_results: MutableVec<((String, Arc<I10Pointer>), f32, u8)>,
    index_status_text: Mutable<Option<String>>,

    render_i10_detail: Mutable<bool>,
    i10_hx: MutableVec<String>,
    i10_code: Mutable<String>,
    i10_detail: Mutable<Option<Arc<I10Detail>>>,
}

impl DxSearchboxCpn {
    /// full_feature: allow manual input, show `sort` icon instead of `search` icon
    pub fn new(is_external_cause: bool, is_full_feature: bool, value: Mutable<Option<Rc<Icd10>>>, dagger_asterisk_state: Rc<DaggerAsteriskState>) -> Rc<Self> {
        let (icd10, diagnosis) = value.lock_ref().as_ref().map(|v| (v.icd10.clone(), v.ename.clone().unwrap_or_default())).unwrap_or_default();
        let input_mode = if is_full_feature { InputMode::Manual } else { InputMode::ExactSearch };
        Rc::new(Self {
            is_external_cause,
            is_full_feature,
            dagger_asterisk_state,
            input_mode: Mutable::new(input_mode),
            icd10_raw: Mutable::new(icd10),
            diagnosis_raw: Mutable::new(diagnosis),
            search_selected_result: value,
            ..Default::default()
        })
    }

    fn set_search_text(keywords: &str, page: Rc<Self>) {
        let search_text = keywords.replace('.', "");
        let ready = if search_text.chars().count() > 2 {
            page.search_status_text.set_neq(None);
            true
        } else if search_text.is_empty() {
            page.search_status_text.set_neq(None);
            false
        } else {
            page.search_status_text.set_neq(Some(String::from("กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")));
            false
        };
        if ready {
            page.search_text.set_neq(search_text);
            page.is_searching.set_neq(true);
        } else {
            page.search_results.lock_mut().clear();
        }
    }

    fn load_search_data(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            false,
            clone!(app, page => async move {
                let thread = app.drg_worker().await;
                let search_text = page.search_text.get_cloned();
                if !search_text.is_empty() {
                    let bytes = if let Some(code) = find_first_icd10(&search_text) {
                        if page.is_external_cause {
                            thread.search_icd10_ex_code_prefix(code.to_ascii_uppercase()).await
                        } else {
                            thread.search_icd10_code_prefix(code.to_ascii_uppercase()).await
                        }
                    } else if page.is_external_cause {
                        thread.search_icd10_ex_desc(search_text).await
                    } else {
                        thread.search_icd10_desc(search_text).await
                    };
                    let results = bitcode::decode::<Vec<(Arc<I10vx>, f32, u8)>>(&bytes).unwrap_or_default();
                    if results.is_empty() {
                        page.search_results.lock_mut().clear();
                        page.search_status_text.set(Some(String::from("ไม่พบรายการที่ค้นหา")));
                    } else {
                        page.search_results.lock_mut().replace_cloned(results);
                        page.search_status_text.set(None);
                    }
                }
            }),
        )
    }

    fn set_index_text(&self, keywords: &str, is_append: bool) {
        let index_text = keywords.replace([',', '.'], "").to_ascii_lowercase();
        let ready = if index_text.chars().count() > 2 {
            self.index_status_text.set_neq(None);
            true
        } else if index_text.is_empty() {
            self.index_status_text.set_neq(None);
            false
        } else {
            self.index_status_text.set_neq(Some(String::from("กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")));
            false
        };
        if ready {
            if is_append {
                let mut lock = self.index_text.lock_mut();
                if !lock.is_empty() {
                    lock.push(' ');
                }
                lock.push_str(&index_text);
            } else {
                self.index_text.set_neq(index_text.clone());
            }
            self.is_indexing.set_neq(true);
        } else {
            self.index_results.lock_mut().clear();
        }
    }

    /// for NOT is_full_feature only
    fn set_diagnosis_raw(&self, keywords: &str, is_append: bool) {
        if is_append {
            let mut lock = self.diagnosis_raw.lock_mut();
            if lock.is_empty() {
                lock.push_str(&first_char_uppercase(keywords));
            } else {
                lock.push(' ');
                lock.push_str(keywords);
            }
        } else {
            self.diagnosis_raw.set_neq(first_char_uppercase(keywords));
        }
    }

    fn load_index_data(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            false,
            clone!(app, page => async move {
                let thread = app.drg_worker().await;
                let index_text = page.index_text.get_cloned();
                if !index_text.is_empty() {
                    let bytes = match (page.is_external_cause, page.is_substance.get()) {
                        (true, false) => {
                            thread.search_i10_index_external(index_text).await
                        }
                        (false, false) => {
                            thread.search_i10_index_diagnosis(index_text).await
                        }
                        (_, true) => {
                            thread.search_i10_index_substance(index_text).await
                        }
                    };
                    let results = bitcode::decode::<Vec<((String, Arc<I10Pointer>), f32, u8)>>(&bytes).unwrap_or_default();
                    if results.is_empty() {
                        page.index_results.lock_mut().clear();
                        page.index_status_text.set(Some(String::from("ไม่พบรายการที่ค้นหา")));
                    } else {
                        page.index_results.lock_mut().replace_cloned(results);
                        page.index_status_text.set(None);
                    }
                }
            }),
        )
    }

    fn load_i10_detail(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            false,
            clone!(app, page => async move {
                let i10_code = page.i10_code.get_cloned();
                let i10_detail = if !i10_code.is_empty() && i10_code.as_str() != "???" {
                    let thread = app.drg_worker().await;
                    let bytes = thread.get_i10_detail(i10_code).await;
                    bitcode::decode::<Option<Arc<I10Detail>>>(&bytes).unwrap_or_default()
                } else {
                    None
                };
                page.i10_detail.set(i10_detail);
            }),
        )
    }

    fn get_dagger_asterisk_status(&self) {
        let status = self.dagger_asterisk_state.get_status(&self.icd10_raw.lock_ref());
        self.dagger_asterisk_status.set(status);
    }

    /// parent is Option(Group, draggable_id)
    pub fn render(page: Rc<Self>, app: Rc<App>, parent: Option<(Rc<Group<Rc<Icd10>>>, u32, Mutable<bool>)>, changed: Mutable<bool>, enabled: bool) -> Dom {
        html!("div", {
            // update `search_selected_result` and parent's `changed`
            .future(page.raw_changed.signal().for_each(clone!(page, parent, changed => move |raw_changed| {
                if raw_changed {
                    let icd10 = page.icd10_raw.get_cloned();
                    let icd10_fixed = if icd10.is_empty() {String::from("???")} else {icd10};
                    let diagnosis = page.diagnosis_raw.get_cloned();
                    let new_selected = if !diagnosis.is_empty() {
                        Icd10::new(&Some(icd10_fixed.clone()), &Some(diagnosis))
                    } else {
                        None
                    };
                    let old_selected = page.search_selected_result.get_cloned();
                    let old_icd10 = old_selected.as_ref().map(|old| old.icd10.to_owned());
                    let neq = new_selected != old_selected;
                    if neq {
                        if icd10_fixed.as_str() != "???" {
                            if let Some(old) = &old_icd10 {
                                page.dagger_asterisk_state.replace_code(old, &icd10_fixed);
                            } else {
                                page.dagger_asterisk_state.insert_code(&icd10_fixed);
                            }
                            page.dagger_asterisk_state.start_parsing();
                        } else {
                            page.dagger_asterisk_status.set(DaggerAsteriskStatus::None);
                        }
                        page.search_selected_result.set(new_selected);
                        if let Some((group, _, _)) = &parent {
                            group.set_has_empty_by_state();
                        }
                        changed.set_neq(true);
                    }
                    page.raw_changed.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let is_parsed = page.dagger_asterisk_state.is_parsed_signal(),
                let is_parsing = page.dagger_asterisk_state.is_parsing_signal() =>
                *is_parsed && !is_parsing
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    page.get_dagger_asterisk_status();
                }
                async {}
            })))
            // apply `search_selected_result` to search_text
            .future(page.search_selected_result.signal_cloned().for_each(clone!(page => move |opt| {
                if let Some(selected) = opt {
                    let opt = match (selected.icd10.is_empty(), selected.ename.is_none()) {
                        (true, true) => None,
                        (true, false) => Some(selected.ename.clone().unwrap_or_default()),
                        (false, true) => (selected.icd10.as_str() != "???").then(|| selected.icd10.to_owned()),
                        (false, false) => Some(if selected.icd10.as_str() == "???" { selected.ename.clone().unwrap_or_default() } else { selected.icd10.to_owned() }),
                    };
                    // Apply `search_text` for immediate search
                    // NOT apply to `index_text` because it will return many messy results
                    if let Some(search_text) = opt {
                        page.search_text.set_neq(search_text);
                    }
                    page.icd10_raw.set_neq(selected.icd10.clone());
                    page.diagnosis_raw.set_neq(selected.ename.clone().unwrap_or_default());
                }
                async {}
            })))
            .future(page.is_searching.signal().for_each(clone!(app, page => move |loading| {
                if loading {
                    let wait = Timeout::new(500, clone!(app, page => move || {
                        Self::load_search_data(page, app);
                    }));
                    // prevent multiple keyup
                    if let Some(handle) = page.timer_handle.get() {
                        Timeout::manual_drop(handle);
                    }
                    page.timer_handle.set(Some(wait.handle()));
                    wait.forget();
                    page.is_searching.set(false);
                }
                async {}
            })))
            .future(page.is_indexing.signal().for_each(clone!(app, page => move |loading| {
                if loading {
                    let wait = Timeout::new(500, clone!(app, page => move || {
                        Self::load_index_data(page, app);
                    }));
                    // prevent multiple keyup
                    if let Some(handle) = page.timer_handle.get() {
                        Timeout::manual_drop(handle);
                    }
                    page.timer_handle.set(Some(wait.handle()));
                    wait.forget();
                    page.is_indexing.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let is_render = page.render_i10_detail.signal() =>
                !busy && *is_render
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_i10_detail(page.clone(), app.clone());
                    page.render_i10_detail.set(false);
                }
                async {}
            })))
            .style("position","relative")
            .style("width", "100%")
            .child(html!("div", {
                .class(class::INPUT_GROUP)
                .style("user-select","none")
                .apply_if(enabled && page.is_full_feature, |dom| { dom
                    .child(html!("drag-handle", {
                        .class("input-group-text")
                        .child(html!("i", {.class(class::FA_SORT).style("pointer-events","none")}))
                    }))
                })
                // Input mode status
                .apply_if(enabled, |dom| { dom
                    .child_signal(page.input_mode.signal_cloned().map(clone!(app, page => move |input_mode| {
                        match input_mode {
                            InputMode::Manual => {
                                Some(html!("span", {
                                    .class(class::INPUT_GROUP_TEXT_PX1_BG_GOLD)
                                    .child(html!("i", {.class(class::FA_KEYBOARD)}))
                                }))
                            }
                            InputMode::IndexSearch => {
                                Some(html!("span", {
                                    .class(class::INPUT_GROUP_TEXT_PX1_BG_CYAN)
                                    .child(html!("i", {.class(class::FA_SEARCH_PLUS)}))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.index_status_text.set(None);
                                        page.i10_detail.set(None);
                                        page.index_results.lock_mut().clear();
                                        if page.show_index_content.get() {
                                            page.show_index_content.set(false);
                                        } else {
                                            page.show_index_content.set(true);
                                            page.set_index_text(&page.diagnosis_raw.lock_ref(), false);
                                        }
                                    }))
                                }))
                            }
                            InputMode::ExactSearch => {
                                Some(html!("span", {
                                    .class(class::INPUT_GROUP_TEXT_PX1_BG_GRAY)
                                    .child(html!("i", {.class(class::FA_SEARCH)}))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.search_status_text.set(None);
                                        page.i10_detail.set(None);
                                        page.search_results.lock_mut().clear();
                                        if page.show_search_content.get() {
                                            page.show_search_content.set(false);
                                        } else {
                                            page.show_search_content.set(true);
                                            let icd10_raw = page.icd10_raw.get_cloned();
                                            let diagnosis_raw = page.diagnosis_raw.get_cloned();
                                            let opt = match (icd10_raw.is_empty(), diagnosis_raw.is_empty()) {
                                                (true, true) => None,
                                                (true, false) => Some(diagnosis_raw),
                                                (false, true) => (icd10_raw.as_str() != "???").then(|| icd10_raw),
                                                (false, false) => Some(if icd10_raw.as_str() == "???" { diagnosis_raw } else { icd10_raw }),
                                            };
                                            if let Some(new) = opt {
                                                Self::set_search_text(&new, page.clone());
                                            }
                                        }
                                    }))
                                }))
                            }
                        }
                    })))
                })
                // ICD10 input
                .child_signal(page.input_mode.signal_cloned().map(clone!(page => move |input_mode| {
                    // not show in [IndexSearch + NOT is_full_feature]
                    (!(matches!(input_mode, InputMode::IndexSearch) && !page.is_full_feature)).then(|| {
                        html!("input" => HtmlInputElement, {
                            .attr("type", "text")
                            .class(class::FORM_CTRL_SM)
                            .attr("placeholder", "ICD10")
                            .style("max-width","65px")
                            .prop_signal("value", page.icd10_raw.signal_ref(|code| icd10_dot(code)))
                            .with_node!(element => {
                                .event(clone!(page => move |_: events::Change| {
                                    let value = element.value().replace('.', "").to_ascii_uppercase();
                                    let is_neq = page.icd10_raw.lock_ref().as_str() != value.as_str();
                                    if is_neq {
                                        page.icd10_raw.set(if value.is_empty() {String::from("???")} else {value.clone()});
                                        if page.is_full_feature {
                                            if !page.diagnosis_raw.lock_ref().is_empty() {
                                                page.raw_changed.set_neq(true);
                                            }
                                        } else if matches!(page.input_mode.get_cloned(), InputMode::ExactSearch) {
                                            page.show_search_content.set_neq(true);
                                            Self::set_search_text(&value, page.clone());
                                        }
                                    }
                                }))
                            })
                        })
                    })
                })))
                .child_signal(page.dagger_asterisk_status.signal_cloned().map(clone!(page => move |dagger_asterisk_status| {
                    match dagger_asterisk_status {
                        DaggerAsteriskStatus::None => None,
                        DaggerAsteriskStatus::NotFound => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .child(html!("i", {.class(class::FA_QUESTION)}))
                        })),
                        DaggerAsteriskStatus::Invalid => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .child(html!("i", {.class(class::FA_X_CIRCLE_RED)}))
                        })),
                        DaggerAsteriskStatus::Single(code) => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)}))
                            .event(clone!(page => move |_:events::Click| {
                                page.i10_hx.lock_mut().clear();
                                page.i10_code.set(icd10_dot(&code.code));
                                page.render_i10_detail.set_neq(true);
                            }))
                        })),
                        DaggerAsteriskStatus::DaggerWith(dagger, asterisk) => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .children([
                                Self::render_pair_detail_without_code(true, false, dagger.code.clone(), page.clone()),
                                Self::render_code_pair_detail(false, asterisk.code.clone(), page.clone()),
                            ])
                        })),
                        DaggerAsteriskStatus::DaggerWithMultiple(dagger, asters) => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .child(Self::render_pair_detail_without_code(true, false, dagger.code.clone(), page.clone()))
                            .children(asters.into_iter().map(|asterisk| {
                                Self::render_code_pair_detail(false, asterisk.code.clone(), page.clone())
                            }))
                        })),
                        DaggerAsteriskStatus::AsteriskWith(asterisk, dagger) => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .children([
                                Self::render_pair_detail_without_code(false, false, asterisk.code.clone(), page.clone()),
                                Self::render_code_pair_detail(true, dagger.code.clone(), page.clone()),
                            ])
                        })),
                        DaggerAsteriskStatus::AsteriskWithMultiple(asterisk, daggers) => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .child(Self::render_pair_detail_without_code(false, false, asterisk.code.clone(), page.clone()))
                            .children(daggers.into_iter().map(|dagger| {
                                Self::render_code_pair_detail(true, dagger.code.clone(), page.clone())
                            }))
                        })),
                        DaggerAsteriskStatus::AsteriskAlone(asterisk) => {
                            Some(Self::render_pair_detail_without_code(false, true, asterisk.code.clone(), page.clone()))
                        }
                        DaggerAsteriskStatus::Multiple(pairs) => Some(html!("span", {
                            .class(class::INPUT_GROUP_TEXT_PX1)
                            .children(pairs.into_iter().map(|(dagger, asterisk)| {
                                vec![
                                    html!("span", {.text("(")}),
                                    Self::render_code_pair_detail(true, dagger.code.clone(), page.clone()),
                                    Self::render_code_pair_detail(false, asterisk.code.clone(), page.clone()),
                                    html!("span", {.text(")")}),
                                ]
                            }).flatten())
                        })),
                    }
                })))
                // Diagnosis input
                .child(html!("input" => HtmlInputElement, {
                    .attr("type", "text")
                    .class(class::FORM_CTRL_SM)
                    .attr("placeholder", "Diagnosis")
                    .prop_signal("value", page.diagnosis_raw.signal_cloned())
                    .with_node!(element => {
                        .event(clone!(page => move |_: events::Input| {
                            let value = element.value();
                            let is_neq = page.diagnosis_raw.lock_ref().as_str() != value.as_str();
                            if is_neq {
                                page.diagnosis_raw.set(value.clone());
                                if page.is_full_feature {
                                    page.raw_changed.set_neq(true);
                                } else {
                                    match page.input_mode.get_cloned() {
                                        InputMode::ExactSearch => {
                                            page.show_search_content.set_neq(true);
                                            Self::set_search_text(&value, page.clone());
                                        }
                                        InputMode::IndexSearch => {
                                            page.show_index_content.set_neq(true);
                                            page.set_index_text(&value, false);
                                        }
                                        InputMode::Manual => {}
                                    }
                                }

                            }
                        }))
                    })
                }))
                // Clear button
                .apply_if(enabled, |dom| dom
                    .child_signal(map_ref! {
                        let no_code = page.icd10_raw.signal_ref(|s| s.is_empty()),
                        let no_dx = page.diagnosis_raw.signal_ref(|s| s.is_empty()) =>
                        !no_code || !no_dx
                    }.map(clone!(page, parent, changed => move |has_data| {
                        has_data.then(|| {
                            html!("button", {
                                .class(class::BTN_SM_GRAY)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_X)}))
                                .event(clone!(page, parent, changed => move |_: events::Click| {
                                    page.icd10_raw.set_neq(String::new());
                                    page.diagnosis_raw.set_neq(String::new());
                                    if page.is_full_feature {
                                        page.search_selected_result.set(None);
                                        if let Some((group, _, _)) = &parent {
                                            group.set_has_empty_by_state();
                                        }
                                        page.dagger_asterisk_status.set(DaggerAsteriskStatus::None);
                                        changed.set_neq(true);
                                    } else {
                                        page.search_text.set_neq(String::new());
                                        page.search_results.lock_mut().clear();
                                        page.index_text.set_neq(String::new());
                                        page.index_results.lock_mut().clear();
                                        let has_detail = page.i10_detail.lock_ref().is_some();
                                        if has_detail {
                                            page.i10_detail.set(None);
                                        }
                                    }
                                }))
                            })
                        })
                    })))
                )
                // Mode selecting buttons
                .apply_if(enabled, |dom| { dom
                    .children_signal_vec(page.input_mode.signal_cloned().map(clone!(app, page => move |input_mode| {
                        let mut results = Vec::with_capacity(2);
                        let items = match (page.is_full_feature, input_mode) {
                            (true, InputMode::Manual) => [1,2],
                            (true, InputMode::ExactSearch) => [0,2],
                            (true, InputMode::IndexSearch) => [0,1],
                            (false, InputMode::Manual) => [9,9],
                            (false, InputMode::ExactSearch) => [2,9],
                            (false, InputMode::IndexSearch) => [1,9],
                        };
                        // Manual button
                        if page.is_full_feature && items.contains(&0) {
                            results.push(html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_GOLD)
                                .child(html!("i", {.class(class::FA_KEYBOARD)}))
                                .event(clone!(page => move |_:events::Click| {
                                    page.search_status_text.set(None);
                                    page.search_results.lock_mut().clear();
                                    page.show_search_content.set_neq(false);
                                    page.index_status_text.set(None);
                                    page.index_results.lock_mut().clear();
                                    page.show_index_content.set_neq(false);
                                    page.i10_detail.set(None);
                                    page.input_mode.set(InputMode::Manual);
                                }))
                            }))
                        }
                        // Index search button
                        if items.contains(&2) {
                            results.push(html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_CYAN)
                                .child(html!("i", {.class(class::FA_SEARCH_PLUS)}))
                                .event(clone!(page => move |_:events::Click| {
                                    page.search_status_text.set(None);
                                    page.search_results.lock_mut().clear();
                                    page.show_search_content.set_neq(false);
                                    page.index_status_text.set(None);
                                    page.index_results.lock_mut().clear();
                                    page.i10_detail.set(None);
                                    page.input_mode.set(InputMode::IndexSearch);
                                    page.show_index_content.set(true);
                                    let recent_dx = page.diagnosis_raw.lock_ref();
                                    let recent_search = page.search_text.lock_ref();
                                    let old = if !recent_dx.is_empty() {
                                        recent_dx
                                    } else {
                                        recent_search
                                    };
                                    page.set_index_text(&old, false);
                                }))
                            }))
                        }
                        // Exact search button
                        if items.contains(&1) {
                            results.push(html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_GRAY)
                                .child(html!("i", {.class(class::FA_SEARCH)}))
                                .event(clone!(page => move |_:events::Click| {
                                    page.search_status_text.set(None);
                                    page.search_results.lock_mut().clear();
                                    page.index_status_text.set(None);
                                    page.index_results.lock_mut().clear();
                                    page.show_index_content.set_neq(false);
                                    page.i10_detail.set(None);
                                    page.input_mode.set(InputMode::ExactSearch);
                                    page.show_search_content.set(true);

                                    let icd10_raw = page.icd10_raw.get_cloned();
                                    let diagnosis_raw = page.diagnosis_raw.get_cloned();
                                    let opt = match (icd10_raw.is_empty(), diagnosis_raw.is_empty()) {
                                        (true, true) => None,
                                        (true, false) => Some(diagnosis_raw),
                                        (false, true) => (icd10_raw.as_str() != "???").then(|| icd10_raw),
                                        (false, false) => Some(if icd10_raw.as_str() == "???" { diagnosis_raw } else { icd10_raw }),
                                    };
                                    Self::set_search_text(&opt.unwrap_or(page.index_text.get_cloned()), page.clone());
                                }))
                            }))
                        }
                        results
                    })).to_signal_vec())
                })
                // Remove searchbox button
                .apply(clone!(page, parent, changed => move |dom| {
                    if let Some((group, id, _)) = parent {
                        if enabled {
                            dom.child(html!("button", {
                                .class(class::BTN_SM_RED)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_MINUS)}))
                                .event(clone!(page, group, changed => move |_: events::Click| {
                                    group.remove_draggables(&[id]);
                                    group.set_has_empty_by_state();
                                    if page.search_selected_result.lock_ref().is_some() {
                                        changed.set_neq(true);
                                    }
                                    let has_detail = page.i10_detail.lock_ref().is_some();
                                    if has_detail {
                                        page.i10_detail.set(None);
                                    }
                                }))
                            }))
                        } else {
                            dom
                        }
                    } else {
                        dom
                    }
                }))
            }))
            // search table
            .child_signal(page.input_mode.signal_cloned().map(clone!(app, page, parent => move |input_mode| {
                match input_mode {
                    InputMode::Manual => None,
                    InputMode::ExactSearch => {
                        Some(html!("div", {
                            .style("position","absolute")
                            .style("width","100%")
                            .style("border-right","1px solid lightgrey")
                            .style("max-height","250px")
                            .style("overflow-y","auto")
                            .style("z-index","3")
                            .child_signal(page.show_search_content.signal().map(clone!(app, page, parent => move |show_search_content| {
                                show_search_content.then(|| {
                                    html!("table", {
                                        .class(class::TABLE_STRIP)
                                        .style("border-top-style","hidden")
                                        .style("margin-bottom","0")
                                        .apply_if(page.is_full_feature, |dom| dom
                                            .child(Self::render_search_header(page.clone(), enabled))
                                        )
                                        .child(html!("tbody", {
                                            .child_signal(page.search_status_text.signal_cloned().map(|opt| {
                                                opt.as_ref().map(|text| {
                                                    html!("tr", {
                                                        .child(html!("td", {
                                                            .attr("colspan", "2")
                                                            .text(text)
                                                        }))
                                                    })
                                                })
                                            }))
                                            .children_signal_vec(page.search_results.signal_vec_cloned().map(clone!(page, parent => move |search_result| {
                                                Self::render_search_result(search_result, page.clone(), parent.clone())
                                            })))
                                        }))
                                    })
                                })
                            })))
                        }))
                    }
                    InputMode::IndexSearch => {
                        Some(html!("div", {
                            .style("position","absolute")
                            .style("width","100%")
                            .style("border-right","1px solid lightgrey")
                            .style("max-height","250px")
                            .style("overflow-y","auto")
                            .style("z-index","3")
                            .child_signal(page.show_index_content.signal().map(clone!(page, parent => move |show_index_content| {
                                show_index_content.then(|| {
                                    html!("table", {
                                        .class(class::TABLE_STRIP)
                                        .style("border-top-style","hidden")
                                        .style("margin-bottom","0")
                                        .apply_if(page.is_full_feature, |dom| dom
                                            .child(Self::render_index_header(page.clone(), enabled))
                                        )
                                        .child(html!("tbody", {
                                            .child_signal(page.index_status_text.signal_cloned().map(|opt| {
                                                opt.as_ref().map(|text| {
                                                    html!("tr", {
                                                        .child(html!("td", {
                                                            .attr("colspan", "2")
                                                            .text(text)
                                                        }))
                                                    })
                                                })
                                            }))
                                            .children_signal_vec(page.index_results.signal_vec_cloned().map(clone!(page, parent => move |index_result| {
                                                Self::render_index_result(index_result, page.clone(), parent.clone())
                                            })))
                                        }))
                                    })
                                })
                            })))
                        }))
                    }
                }
            })))
            // Book 1 overlay
            // .child_signal(map_ref!{
            //     let no_status_text = page.status_text.signal_ref(|txt| txt.is_none()),
            //     let no_results = page.results.signal_vec_cloned().is_empty(),
            //     let opt = page.i10_detail.signal_cloned() =>
            //     (*no_status_text && *no_results, opt.clone())
            // }.map(clone!(page => move |(not_search, opt)| {
            .child_signal(page.i10_detail.signal_cloned().map(clone!(page => move |opt| {
                opt.map(|i10_detail| {
                    html!("div", {
                        .style("position","absolute")
                        .style("background-color","var(--bs-body-bg)")
                        .style("width","100%")
                        .style("border-radius","9px")
                        .style("box-shadow","1px 3px 9px #777")
                        .style("z-index","5")
                        .child(Self::render_i10_detail(i10_detail, page.clone()))
                    })
                })
            })))
        })
    }

    fn render_search_header(page: Rc<Self>, enabled: bool) -> Dom {
        html!("thead", {
            .child(html!("tr", {
                .children([
                    html!("th", {
                        .style("position","sticky")
                        .style("top","0")
                        .attr("scope", "col")
                        .child(html!("div", {
                            .class("d-flex")
                            .children([
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM)
                                    .children([
                                        html!("span", {
                                            .class("input-group-text")
                                            .text("Search ICD10")
                                        }),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .style("max-width","65px")
                                            .attr("placeholder", "ICD10")
                                            .attr("autocomplete", "off")
                                            .prop_signal("value", page.icd10_raw.signal_ref(|code| icd10_dot(code)))
                                            .apply_if(!enabled, |dom| dom.attr("disabled",""))
                                            .with_node!(element => {
                                                .event(clone!(page => move |_: events::Input| {
                                                    Self::set_search_text(&element.value(), page.clone());
                                                }))
                                            })

                                        }),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("placeholder", "กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")
                                            .attr("autocomplete", "off")
                                            .focused(true)
                                            .prop_signal("value", page.search_text.signal_cloned())
                                            .apply_if(!enabled, |dom| dom.attr("disabled",""))
                                            .with_node!(element => {
                                                .event(clone!(page => move |_: events::Input| {
                                                    Self::set_search_text(&element.value(), page.clone());
                                                }))
                                            })

                                        }),
                                    ])
                                }),
                            ])
                        }))
                    }),
                    html!("th", {
                        .style("border-left-style","hidden")
                        .style("width","50px")
                        .child(html!("button", {
                            .attr("type","button")
                            .class(class::BTN_SM)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(page => move |_:events::Click| {
                                page.search_status_text.set(None);
                                page.i10_detail.set(None);
                                page.search_results.lock_mut().clear();
                                page.show_search_content.set(false);
                                page.input_mode.set(InputMode::Manual);
                            }))
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_search_result(search_result: (Arc<I10vx>, f32, u8), page: Rc<Self>, parent: Option<(Rc<Group<Rc<Icd10>>>, u32, Mutable<bool>)>) -> Dom {
        let txt = page.search_text.lock_ref();
        let result = search_result.0.clone();
        let col = search_result.2;
        // col: 1=icd10, 2=ename
        html!("tr", {
            .children([
                html!("td", {
                    .child(html!("div", {
                        .children([
                            html!("span", {
                                .class("fw-bold")
                                .apply(|dom| {
                                    let icd10 = icd_dash(&icd10_dot(&result.code), result.is_valid);
                                    if col == 1 {
                                        // dom.children(red_chars_in_words(&txt, &icd10))
                                        dom.children(red_keywords_in_icd_dot(false, &txt, &icd10))
                                    } else {
                                        dom.text(&icd10)
                                    }
                                })
                            }),
                            html!("span", {.text(" : ")}),
                            html!("span", {
                                .apply(|dom| {
                                    if col == 2 {
                                        // dom.children(red_chars_in_words(&txt, &result.desc))
                                        dom.children(red_keywords_in_sentense(&txt, &result.desc))
                                    } else {
                                        dom.text(&result.desc)
                                    }
                                })
                            }),
                        ])
                        .apply_if(result.is_tm, |dom|
                            dom.child(html!("span", {.class(class::BADGE_GOLD_R).text("TM")}))
                        )
                        .apply_if(!result.is_valid, |dom|
                            dom.child(html!("i", {.class(class::ALIGN_MIDDLE_R).class(class::FA_R_ARROW_CIRCLE)}))
                        )
                    }))
                    .style("cursor","pointer")
                    .event(clone!(page, parent, result => move |_: events::Click| {
                        if result.is_valid {
                            page.icd10_raw.set_neq(result.code.to_owned());
                            page.diagnosis_raw.set_neq(remove_unspecified(&result.desc));
                            page.show_search_content.set(false);
                            if page.is_full_feature {
                                page.raw_changed.set(true);
                            }
                        } else {
                            page.search_text.set_neq(result.code.to_owned());
                            page.is_searching.set_neq(true);
                        }
                    }))
                }),
                html!("td", {
                    .style("border-left-style","hidden")
                    .style("width","40px")
                    .style("text-align","end")
                    .child(Self::render_code_book(icd10_dot(&result.code), page.clone()))
                }),
            ])
        })
    }

    fn render_index_header(page: Rc<Self>, enabled: bool) -> Dom {
        html!("thead", {
            .child(html!("tr", {
                .style("position","sticky")
                .style("top","0")
                .children([
                    html!("th", {
                        .child(html!("div", {
                            .class("d-flex")
                            .children([
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM)
                                    .children([
                                        html!("span", {
                                            .class("input-group-text")
                                            .text("Search INDEX")
                                        }),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("placeholder", "กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")
                                            .attr("autocomplete", "off")
                                            .focused(true)
                                            .prop_signal("value", page.index_text.signal_cloned())
                                            .apply_if(!enabled, |dom| dom.attr("disabled",""))
                                            .with_node!(element => {
                                                .event(clone!(page, element => move |_: events::Input| {
                                                    page.set_index_text(&element.value(), false);
                                                }))
                                            })
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::FORM_CHK_SW)
                                    .class(class::M_R2B1)
                                    .style("font-weight","300")
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type","checkbox")
                                            .attr("role","switch")
                                            .attr("id","is-substance-checkbox")
                                            .class("form-check-input")
                                            .apply(mixins::checkbox_bool(page.is_substance.clone(), page.is_indexing.clone()))
                                        }),
                                        doms::label_check_for("is-substance-checkbox","ยา/สารเคมี"),
                                    ])
                                }),
                            ])
                        }))
                    }),
                    html!("th", {
                        .style("border-left-style","hidden")
                        .style("width","50px")
                        .child(html!("button", {
                            .attr("type","button")
                            .class(class::BTN_SM)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(page => move |_:events::Click| {
                                page.index_status_text.set(None);
                                page.i10_detail.set(None);
                                page.index_results.lock_mut().clear();
                                page.show_index_content.set(false);
                                page.input_mode.set(InputMode::Manual);
                            }))
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_index_result(index_result: ((String, Arc<I10Pointer>), f32, u8), page: Rc<Self>, parent: Option<(Rc<Group<Rc<Icd10>>>, u32, Mutable<bool>)>) -> Dom {
        let search_texts = page
            .index_text
            .lock_ref()
            .split(' ')
            .filter_map(|s| {
                let r = s.trim().to_ascii_lowercase();
                (!r.is_empty()).then(|| r)
            })
            .collect::<Vec<String>>();
        let (keywords, result) = index_result.0.clone();
        // let col = tuple.2;
        html!("tr", {
            .child(html!("td", {
                .attr("colspan", "2")
                .child(html!("div", {
                    .children(keywords.split(',').map(|s| s.trim().to_owned()).filter_map(clone!(page, parent => move |words| {
                        let has_parentheses = words.starts_with('(');
                        let words_exact = if has_parentheses {
                            if let Some(no_prefix) = words.strip_prefix('(') {
                                if let Some(no_both) = no_prefix.strip_suffix(')') {
                                    no_both.to_owned()
                                } else {
                                    no_prefix.to_owned()
                                }
                            } else {
                                words
                            }
                        } else {
                            words
                        };
                        (!words_exact.is_empty()).then(|| {
                            // search_texts is lowercase
                            let is_matched = search_texts.iter().any(|search_text| words_exact.to_ascii_lowercase().contains(search_text));
                            // is_icd10_resemble() NEED uppercase ICD10
                            if words_exact.split(' ').any(|w| is_icd10_resemble(w)) {
                                let mut contents = Vec::new();
                                for w in words_exact.split(' ') {
                                    if is_icd10_resemble(w) {
                                        for (i, code) in w.split('-').filter_map(|c| {
                                            let n = c.trim().trim_end_matches('.');
                                            (!n.is_empty()).then(|| n.to_ascii_uppercase())
                                        }).enumerate() {
                                            if i > 0 {
                                                contents.push(html!("span", {.text("-")}));
                                            }
                                            contents.push(Self::render_code_badge(code.clone(), None, page.clone(), true));
                                            contents.push(Self::render_code_book(code, page.clone()));
                                        }
                                    } else {
                                        contents.push(html!("span", {
                                            .text(w)
                                            .apply_if(!contents.is_empty(), |dom| dom.class("ms-1"))
                                        }))
                                    }
                                }
                                html!("span", {
                                    .apply(|dom| {
                                        match (is_matched, has_parentheses) {
                                            (true, true) => dom.class(class::BADGE_RED25_L),
                                            (true, false) => dom.class(class::BADGE_RED75_L),
                                            (false, true) => dom.class(class::BADGE_CYAN25_L),
                                            (false, false) => dom.class(class::BADGE_CYAN_L),
                                        }
                                    })
                                    .children(contents)
                                })
                            } else {
                                html!("span", {
                                    .apply(|dom| {
                                        match (is_matched, has_parentheses) {
                                            (true, true) => dom.class(class::BADGE_RED25_L),
                                            (true, false) => dom.class(class::BADGE_RED75_L),
                                            (false, true) => dom.class(class::BADGE_CYAN25_L),
                                            (false, false) => dom.class(class::BADGE_CYAN_L),
                                        }
                                    })
                                    .style("cursor","pointer")
                                    .text(&words_exact)
                                    .event(clone!(page, parent => move |_:events::Click| {
                                        page.set_diagnosis_raw(&words_exact, true);
                                        if page.is_full_feature {
                                            page.raw_changed.set(true);
                                            if let Some((group, _, _)) = &parent {
                                                group.has_empty.set_neq(false);
                                            }
                                        } else {
                                            page.set_index_text(&words_exact, true);
                                        }
                                    }))
                                })
                            }
                        })
                    })))
                    .apply(|dom| {
                        if let Some(note) = result.note.clone() {
                            match note {
                                Note::See(s) => {
                                    dom.children([
                                        html!("span", {.class("small").text("See ")}),
                                        Self::render_note(s.clone(), page.clone()),
                                    ])
                                }
                                Note::SeeAlso(s) => {
                                    dom.children([
                                        html!("span", {.class("small").text("See also ")}),
                                        Self::render_note(s.clone(), page.clone()),
                                    ])
                                }
                                Note::SeeCategory(s) => {
                                    dom.children([
                                        html!("span", {.class("small").text("See category ")}),
                                        html!("span", {
                                            .class(class::BADGE_GRAY_L)
                                            .style("cursor","pointer")
                                            .text(&s)
                                            .event(clone!(page => move |_:events::Click| {
                                                page.i10_hx.lock_mut().clear();
                                                page.i10_code.set(s.to_owned());
                                                page.render_i10_detail.set_neq(true);
                                            }))
                                        }),
                                    ])
                                }
                                Note::Code(s) => {
                                    dom.children([
                                        html!("span", {.class(class::SMALL_L).text(&s)}),
                                    ])
                                }
                            }
                        } else {
                            dom
                        }
                    })
                    .children(result.bracket_notes.clone().into_iter().map(clone!(page => move |note| {
                        match note {
                            Note::See(s) => {
                                vec![
                                    html!("span", {.class("small").text("(See ")}),
                                    Self::render_note(s.clone(), page.clone()),
                                    html!("span", {.class(class::SMALL_L).text(")")}),
                                ]
                            }
                            Note::SeeAlso(s) => {
                                vec![
                                    html!("span", {.class("small").text("(See also ")}),
                                    Self::render_note(s.clone(), page.clone()),
                                    html!("span", {.class(class::SMALL_L).text(")")}),
                                ]
                            }
                            Note::SeeCategory(s) => {
                                vec![
                                    html!("span", {.class("small").text("(See category ")}),
                                    html!("span", {
                                        .class(class::BADGE_GRAY)
                                        .style("cursor","pointer")
                                        .text(&s)
                                        .event(clone!(page => move |_:events::Click| {
                                            page.i10_hx.lock_mut().clear();
                                            page.i10_code.set(s.to_owned());
                                            page.render_i10_detail.set_neq(true);
                                        }))
                                    }),
                                    html!("span", {.class(class::SMALL_L).text(")")}),
                                ]
                            }
                            Note::Code(s) => {
                                vec![
                                    html!("span", {.class("small").text("(")}),
                                    html!("span", {.class("small").text(&s)}),
                                    html!("span", {.class(class::SMALL_L).text(")")}),
                                ]
                            }
                        }
                    })).flatten())
                    .apply(|dom| {
                        if let Some(code) = result.code.clone() {
                            match code {
                                Code::Single(s) => {
                                    dom.children([
                                        Self::render_code_badge(s.clone(), None, page.clone(), false),
                                        Self::render_code_book(s, page.clone()),
                                    ])
                                }
                                Code::DaggerAster(dagger, aster) => {
                                    dom.children([
                                        Self::render_code_badge(dagger.clone(), Some(html!("i", {.class(class::FA_CROSS_RED)})), page.clone(), false),
                                        Self::render_code_book(dagger, page.clone()),
                                        Self::render_code_badge(aster.clone(), Some(html!("i", {.class(class::FA_ASTERISK_RED)})), page.clone(), false),
                                        Self::render_code_book(aster, page.clone()),
                                    ])
                                }
                                Code::Neoplasm(codes, tag_opt) => {
                                    dom.children(codes.into_iter().filter_map(clone!(page, tag_opt => move |c| {
                                        if c.len() > 2 {
                                            let (start, tail) = c.split_at(1);
                                            let (mid, _) = tail.split_at(2);
                                            let num = mid.parse::<u8>().unwrap_or_default();
                                            let label_opt = match (start, num) {
                                                ("C", ..76 | 81..) => Some("• Primary "),
                                                ("C", 76..81) => Some("• Secondary "),
                                                ("D", ..10) => Some("• In situ "),
                                                ("D", 10..37) => Some("• Benign "),
                                                ("D", 37..) => Some("• Unknown "),
                                                (_, _) => None,
                                            };
                                            label_opt.map(clone!(page, tag_opt => move |label| {
                                                html!("span", {
                                                    .children([
                                                        html!("span", {.class(class::SMALL_L).text(label)}),
                                                        Self::render_code_badge(c.clone(), tag_opt.map(|tag| neoplasn_tag(&tag)), page.clone(), false),
                                                        Self::render_code_book(c, page.clone()),
                                                    ])
                                                })
                                            }))
                                        } else {
                                            None
                                        }
                                    })))
                                }
                                Code::Substance(codes) => {
                                    dom.children(codes.into_iter().filter_map(clone!(page => move |c| {
                                        if c.len() > 2 {
                                            let (start, tail) = c.split_at(1);
                                            let (mid, _) = tail.split_at(2);
                                            let num = mid.parse::<u8>().unwrap_or_default();
                                            let label_opt = match (start, num) {
                                                ("T", 36..66) => Some("• Poison/Toxic effect "),
                                                ("X", 40..50) => Some("• Accident "),
                                                ("X", 60..70) => Some("• Self-harm "),
                                                ("Y", 10..20) => Some("• Undetermine "),
                                                ("Y", 40..60) => Some("• Adverse effect in Rx "),
                                                (_, _) => None,
                                            };
                                            label_opt.map(|label| {
                                                vec![
                                                    html!("span", {.class(class::SMALL_L).text(label)}),
                                                    Self::render_code_badge(c.clone(), None, page.clone(), false),
                                                    Self::render_code_book(c, page.clone()),
                                                ]
                                            })
                                        } else {
                                            None
                                        }
                                    })).flatten())
                                }
                            }
                        } else {
                            dom
                        }
                    })
                }))
            }))
        })
    }

    /// code with dot
    fn render_code_badge(code: String, child_opt: Option<Dom>, page: Rc<Self>, with_ms_1: bool) -> Dom {
        html!("span", {
            .class(class::BADGE_GOLD)
            .style("cursor","pointer")
            .apply_if(with_ms_1, |dom| dom.class("ms-1"))
            .text(&code)
            .apply(|dom| {
                if let Some(child) = child_opt {
                    dom.child(child)
                } else {
                    dom
                }
            })
            .event(clone!(page => move |_:events::Click| {
                if let Some(first) = code.split('-').next() {
                    page.icd10_raw.set_neq(first.trim().replace('.', ""));
                    if page.is_full_feature {
                        page.raw_changed.set(true);
                    }
                }
            }))
        })
    }

    /// code with dot
    fn render_code_book(code: String, page: Rc<Self>) -> Dom {
        html!("span", {
            .class("me-1")
            .style("cursor","pointer")
            .child(html!("i", {.class(class::FA_BOOK_MED)}))
            .event(clone!(page => move |_:events::Click| {
                page.i10_hx.lock_mut().clear();
                page.i10_code.set(code.trim_end_matches(['-', '.']).to_owned());
                page.render_i10_detail.set_neq(true);
            }))
        })
    }

    fn render_code_pair_detail(is_dagger: bool, code: String, page: Rc<Self>) -> Dom {
        html!("span", {
            .class(class::BADGE_GOLD)
            .style("cursor","pointer")
            .text(&icd10_dot(&code))
            .child(html!("i", {.class(if is_dagger {class::FA_CROSS_RED} else {class::FA_ASTERISK_RED})}))
            .event(move |_:events::Click| {
                page.i10_hx.lock_mut().clear();
                page.i10_code.set(icd10_dot(&code));
                page.render_i10_detail.set_neq(true);
            })
        })
    }

    fn render_pair_detail_without_code(is_dagger: bool, is_input_group: bool, code: String, page: Rc<Self>) -> Dom {
        html!("span", {
            .apply_if(is_input_group, |dom| dom.class(class::INPUT_GROUP_TEXT_PX1))
            .child(html!("i", {.class(if is_dagger {class::FA_CROSS_RED} else {class::FA_ASTERISK_RED})}))
            .event(move |_:events::Click| {
                page.i10_hx.lock_mut().clear();
                page.i10_code.set(icd10_dot(&code));
                page.render_i10_detail.set_neq(true);
            })
        })
    }

    fn render_i10_detail(i10_detail: Arc<I10Detail>, page: Rc<Self>) -> Dom {
        let class_usage_opt = render_usage(&i10_detail.usage);
        html!("div", {
            .class("p-2")
            .children([
                html!("div", {
                    .children([
                        html!("button", {
                            .attr("type","button")
                            .class(class::BTN_SM_FR_GRAY_P10)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(page => move |_:events::Click| {
                                page.i10_detail.set(None);
                            }))
                        }),
                        html!("button", {
                            .attr("type","button")
                            .class(class::BTN_SM_FR_GREEN_P10)
                            .child(html!("i", {.class(class::FA_CHECK)}))
                            .event(clone!(page, i10_detail => move |_:events::Click| {
                                page.icd10_raw.set(i10_detail.code.trim().replace('.', ""));
                                page.diagnosis_raw.set(i10_detail.r_prefered.iter().map(|r| r.text.to_owned()).collect::<Vec<String>>().join(" "));
                                page.i10_detail.set(None);
                                page.raw_changed.set(true);
                            }))
                        }),
                    ])
                    .child_signal(page.i10_hx.signal_vec_cloned().is_empty().map(clone!(page => move |no_hx| {
                        (!no_hx).then(|| {
                            html!("span", {
                                .class(class::BADGE_GRAY_L)
                                .style("cursor","pointer")
                                .style("vertical-align","text-bottom")
                                .child(html!("i", {.class(class::FA_BACKWARD)}))
                                .event(clone!(page => move |_:events::Click| {
                                    if let Some(code) = page.i10_hx.lock_mut().pop() {
                                        page.i10_code.set_neq(code.to_owned());
                                        page.render_i10_detail.set_neq(true);
                                    }
                                }))
                            })
                        })
                    })))
                    .apply(|dom| {
                        if let Some(parent) = i10_detail.superclass.clone() {
                            dom.child(html!("span", {
                                .class(class::BADGE_CYAN_L)
                                .style("cursor","pointer")
                                .style("vertical-align","text-bottom")
                                .text(&parent)
                                .event(clone!(page => move |_: events::Click| {
                                    page.i10_hx.lock_mut().push_cloned(page.i10_code.get_cloned());
                                    page.i10_code.set_neq(parent.to_owned());
                                    page.render_i10_detail.set_neq(true);
                                }))
                            }))
                        } else {
                            dom
                        }
                    })
                    .child(html!("span", {
                        .class(class::BADGE_BLUE_L)
                        .style("vertical-align","text-bottom")
                        .text(&i10_detail.code)
                        .apply(|dom| {
                            if let Some(usage) = class_usage_opt {
                                dom.child(usage)
                            } else {
                                dom
                            }
                        })

                    }))
                    .children(i10_detail.r_prefered.iter().flat_map(|r| {[
                        html!("span", {.class("mx-1").text("•")}),
                        Self::render_rubric(r, page.clone()),
                    ]}).skip(1))
                }),
                html!("div", {
                    .style("font-size","small")
                    .style("max-height","400px")
                    .style("overflow-y","auto")
                    // subclass
                    .child(html!("div", {
                        .class("ms-3")
                        .apply(|dom| {
                            if i10_detail.subclasses.is_empty() {
                                // no subclass but has modified_by
                                if let Some(modified_by) = i10_detail.modified_by.as_ref() {
                                    dom.children(modified_by.subclasses.iter().map(clone!(page => move |(code, prefers)| {
                                        Self::render_subclass(code.to_owned(), prefers, page.clone())
                                    })))
                                    .children(modified_by.rubrics.iter().map(|(_, r)| {
                                        Self::render_rubric(r, page.clone())
                                    }))
                                // has sub_modifier
                                } else if let Some(sub_modifier) = i10_detail.sub_modifier.as_ref() {
                                    dom.children(sub_modifier.subclasses.iter().map(clone!(page => move |(code, prefers)| {
                                        Self::render_subclass(code.to_owned(), prefers, page.clone())
                                    })))
                                    .children(sub_modifier.rubrics.iter().map(|(_, r)| {
                                        Self::render_rubric(r, page.clone())
                                    }))
                                } else {
                                    dom
                                }
                            } else {
                                // normal subclass
                                dom.children(i10_detail.subclasses.iter().map(clone!(page => move |(code, prefers)| {
                                    Self::render_subclass(code.to_owned(), prefers, page.clone())
                                })))
                            }
                        })
                    }))
                    .children([
                        html!("div", {
                            .class("ms-3")
                            .children(i10_detail.r_inclusions.iter().map(|r| Self::render_rubric(r, page.clone())))
                        }),
                        html!("div", {
                            .class("ms-3")
                            .children(i10_detail.r_exclusions.iter().map(|r| Self::render_rubric(r, page.clone())))
                        }),
                        html!("div", {
                            .class("ms-3")
                            .children(i10_detail.r_definitions.iter().map(|r| Self::render_rubric(r, page.clone())))
                        }),
                        html!("div", {
                            .class("ms-3")
                            .children(i10_detail.r_texts.iter().map(|r| Self::render_rubric(r, page.clone())))
                        }),
                        html!("div", {
                            .class("ms-3")
                            .children(i10_detail.r_coding_hints.iter().map(|r| Self::render_rubric(r, page.clone())))
                        }),
                        html!("div", {
                            .class("ms-3")
                            .children(i10_detail.r_notes.iter().map(|r| Self::render_rubric(r, page.clone())))
                        }),
                        html!("div", {
                            .class("ms-3")
                            .children(i10_detail.r_foot_notes.iter().map(|r| Self::render_rubric(r, page.clone())))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_rubric(rubric: &Rubric, page: Rc<Self>) -> Dom {
        let (badge, is_preferred) = match rubric.kind {
            RubricKind::Footnote | RubricKind::Note => (html!("span", {.class(class::BADGE_GRAY_L).text("NOTE")}), false),
            RubricKind::Text => (html!("span", {.class(class::BADGE_GRAY_L).text("INFO")}), false),
            RubricKind::CodingHint => (html!("span", {.class(class::BADGE_GRAY_L).text("CODING HINT")}), false),
            RubricKind::Definition => (html!("span", {.class(class::BADGE_GRAY_L).text("DEFINITION")}), false),
            RubricKind::Exclusion => (html!("span", {.class(class::BADGE_RED_L).text("EXCLUDE")}), false),
            RubricKind::Inclusion => (html!("span", {.class(class::BADGE_BLUE_L).text("INCLUDE")}), false),
            RubricKind::PreferredLong | RubricKind::Preferred => (Dom::empty(), true),
            RubricKind::Introduction | RubricKind::Modifierlink => (Dom::empty(), false),
        };

        let details = if rubric.reference.is_empty() {
            let usage_opt = render_usage(&rubric.usage);
            vec![html!("span", {
                .class("me-1")
                .text(&rubric.text)
                .apply_if(is_preferred, |dom| dom
                    .class("fw-bold")
                )
                .apply(|dom| {
                    if let Some(usage) = usage_opt {
                        dom.child(usage)
                    } else {
                        dom
                    }
                })
            })]
        } else {
            let rubric_usage_opt = rubric.usage.clone();
            rubric
                .reference_with_lebel()
                .into_iter()
                .map(|(label, reference)| {
                    let usage_opt = render_usage(&rubric_usage_opt);
                    let ref_usage_opt = render_usage(&reference.usage);
                    vec![
                        html!("span", {
                            .text(&label)
                            .apply_if(is_preferred, |dom| dom
                                .class("fw-bold")
                            )
                            .apply(|dom| {
                                if let Some(usage) = usage_opt {
                                    dom.child(usage)
                                } else {
                                    dom
                                }
                            })
                        }),
                        html!("span", {
                            .class(class::BADGE_GOLD_R)
                            .style("cursor","pointer")
                            .text(&reference.label)
                            .apply(|dom| {
                                if let Some(usage) = ref_usage_opt {
                                    dom.child(usage)
                                } else {
                                    dom
                                }
                            })
                            .event(clone!(page => move |_: events::Click| {
                                if let Some(code) = reference.code.as_ref() {
                                    page.i10_hx.lock_mut().push_cloned(page.i10_code.get_cloned());
                                    // claml use ICD10 with dot as a key
                                    // remove '-' and '.' for some key like 'A01.-' to be 'A01'
                                    page.i10_code.set_neq(code.trim_end_matches(['-','.']).to_owned());
                                    page.render_i10_detail.set_neq(true);
                                }
                            }))
                        }),
                    ]
                })
                .flatten()
                .collect::<Vec<Dom>>()
        };

        html!(if is_preferred {"span"} else {"div"}, {
            .child(badge)
            .children(details)
        })
    }

    fn render_subclass(code: String, prefers: &[Rubric], page: Rc<Self>) -> Dom {
        html!("div", {
            .class("ms-3")
            .child(html!("span", {
                .class(class::BADGE_CYAN_L)
                .style("cursor","pointer")
                .text(&code)
                .event(clone!(page => move |_: events::Click| {
                    page.i10_hx.lock_mut().push_cloned(page.i10_code.get_cloned());
                    page.i10_code.set_neq(code.to_owned());
                    page.render_i10_detail.set_neq(true);
                }))
            }))
            .children(prefers.iter().map(clone!(page => move |preferred| {
                Self::render_rubric(preferred, page.clone())
            })))
        })
    }

    fn render_note(s: String, page: Rc<Self>) -> Dom {
        let has_code = s.split(' ').any(|w| is_icd10_resemble(w));
        html!("span", {
            .class(class::BADGE_GRAY_L)
            .text(&s)
            .apply_if(!has_code, |dom| dom
                .style("cursor","pointer")
                .event(move |_:events::Click| {
                    page.set_index_text(&s, false);
                    if !page.is_full_feature {
                        page.set_diagnosis_raw(&s, false);
                    }
                })
            )
        })
    }
}

fn render_usage(opt: &Option<UsageKind>) -> Option<Dom> {
    opt.as_ref().map(|usage_kind| match usage_kind {
        UsageKind::Dagger => html!("i", {.class(class::FA_CROSS_RED)}),
        UsageKind::Aster => html!("i", {.class(class::FA_ASTERISK_RED)}),
    })
}

fn remove_unspecified(desc: &str) -> String {
    desc.trim_end_matches(", unspecified").replace(", NEC", "").replace(" NEC", "")
}

fn neoplasn_tag(tag: &NeoplasmTag) -> Dom {
    match tag {
        NeoplasmTag::Hash => {
            html!("i", {
                .class(class::FA_HASHTAG)
                .attr("title","Should be classified to malignant neoplasm of skin of these sites if the variety of neoplasm is a squamous cell carcinoma or an epidermoid carcinoma and to benign neoplasm of skin of these sites if the variety of neoplasm is a papilloma (any type).")
            })
        }
        NeoplasmTag::Star => {
            html!("i", {
                .class(class::FA_STAR)
                .attr("title","Carcinomas and adenocarcinomas, of any type other than intraosseous or odontogenic, of sites marked with the sign should be considered as metastatic from an unspecified primary site and coded to C79.5.")
            })
        }
    }
}
