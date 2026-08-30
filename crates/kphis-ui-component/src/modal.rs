pub mod consult_form;
pub mod drug_details;
pub mod drug_duplication;
pub mod drug_interaction;
pub mod drug_notify;
pub mod index_note_form;
pub mod index_plan_action_form;
pub mod ipd_passcode;
pub mod lab_history;
pub mod lab_selector;
pub mod lab_wbc;
pub mod med_reconcile_remed;
pub mod medplan_form;
pub mod opd_er_order_new;
pub mod pre_admit_new;
pub mod pre_order_new;
pub mod pre_order_preview;
pub mod pre_order_select;
pub mod report;
pub mod scoring;
pub mod vs_selector;

use dominator::{DomBuilder, clone, events, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use kphis_ui_app::App;
use kphis_ui_core::class;

/// Apply to "modal" element
pub fn modal_show_bool_mixins(show_mutable: Mutable<bool>, app: Rc<App>) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    #[inline]
    move |dom| {
        with_node!(dom, wrapper => {
            .class(class::MODAL_SHOW)
            .attr("role", "dialog")
            .attr("aria-modal", "true")
            .attr("tabindex", "-1")
            // Add global escape key listener
            .global_event(clone!(app, show_mutable => move |e: events::KeyDown| {
                if show_mutable.get() && e.key() == "Escape" {
                    app.clear_modal_backdrop();
                    show_mutable.set(false);
                }
            }))
            // Click outside modal's content (aka click at modal container)
            .event(move |e :events::Click| {
                if show_mutable.get() {
                    let is_container = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                        .map(|node| wrapper.is_same_node(Some(&node)))
                        .unwrap_or(true);
                    if is_container {
                        app.clear_modal_backdrop();
                        show_mutable.set(false);
                    }
                }
            })
        })
    }
}

/// Apply to "modal" element
pub fn modal_show_option_mixins<T: 'static>(show_mutable: Mutable<Option<T>>, app: Rc<App>) -> impl FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
    #[inline]
    move |dom| {
        with_node!(dom, wrapper => {
            .class(class::MODAL_SHOW)
            .attr("role", "dialog")
            .attr("aria-modal", "true")
            .attr("tabindex", "-1")
            // Add global escape key listener
            .global_event(clone!(app, show_mutable => move |e: events::KeyDown| {
                let is_show = show_mutable.lock_ref().is_some();
                if is_show && e.key() == "Escape" {
                    app.clear_modal_backdrop();
                    show_mutable.set(None);
                }
            }))
            // Click outside modal's content (aka click at modal container)
            .event(move |e :events::Click| {
                let is_show = show_mutable.lock_ref().is_some();
                if is_show {
                    let is_container = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                        .map(|node| wrapper.is_same_node(Some(&node)))
                        .unwrap_or(true);
                    if is_container {
                        app.clear_modal_backdrop();
                        show_mutable.set(None);
                    }
                }
            })
        })
    }
}
