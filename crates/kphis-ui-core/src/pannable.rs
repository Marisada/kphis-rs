use dominator::{DomBuilder, events, with_node};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::rc::Rc;
use web_sys::Element;

use crate::class;

#[derive(Debug, Default)]
pub struct PanState {
    pub scroll_top: Mutable<i32>,
    pub scroll_left: Mutable<i32>,
    pub mouse_x: Mutable<i32>,
    pub mouse_y: Mutable<i32>,
    pub element: Mutable<Option<Rc<Element>>>,
    pub is_panning: Mutable<bool>,
}

impl PanState {
    pub fn cursor_style(&self) -> impl Signal<Item = &'static str> + use<> {
        self.is_panning.signal_cloned().map(|is_panning| if is_panning { "grabbing" } else { "grab" })
    }

    pub fn user_select_style(&self) -> impl Signal<Item = &'static str> + use<> {
        self.is_panning.signal_cloned().map(|is_panning| if is_panning { "none" } else { "text" })
    }

    pub fn on_mouse_move(e: &events::MouseMove, pan_state: Rc<Self>) {
        if pan_state.is_panning.get() {
            let dx = e.mouse_x() - pan_state.mouse_x.get();
            let dy = e.mouse_y() - pan_state.mouse_y.get();
            if (dx as f32).hypot(dy as f32) > 7.0 {
                if let Some(element) = pan_state.element.lock_ref().as_ref() {
                    element.set_scroll_left(pan_state.scroll_left.get() - dx);
                    element.set_scroll_top(pan_state.scroll_top.get() - dy);
                }
            }
        }
    }

    pub fn on_mouse_up(pan_state: Rc<Self>) {
        if pan_state.is_panning.get() {
            pan_state.element.set(None);
            pan_state.is_panning.set(false);
        }
    }

    pub fn pan_container_mixins<T>(pan_state: Rc<Self>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
    where
        T: wasm_bindgen::JsCast + Clone + std::convert::AsRef<web_sys::HtmlElement> + std::convert::AsRef<web_sys::EventTarget> + 'static,
    {
        #[inline]
        move |dom| {
            with_node!(dom, element => {
                .style("width", "100%")
                .style("height","100%")
                .style("overflow","auto")
                .style_signal("cursor", pan_state.cursor_style())
                .style_signal(class::USER_SELECT, pan_state.user_select_style())
                .event(move |e: events::MouseDown| {
                    if let Ok(elm) = element.clone().dyn_into::<web_sys::Element>() {
                        pan_state.mouse_x.set(e.mouse_x());
                        pan_state.mouse_y.set(e.mouse_y());
                        pan_state.scroll_left.set(elm.scroll_left());
                        pan_state.scroll_top.set(elm.scroll_top());
                        pan_state.element.set(Some(Rc::new(elm)));
                        pan_state.is_panning.set(true);
                    }
                })
            })
        }
    }
}
