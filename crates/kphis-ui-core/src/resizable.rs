use dominator::{Dom, DomBuilder, clone, events, html, with_node};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::rc::Rc;
use web_sys::Element;

use crate::class;

#[derive(Clone, Debug, Default)]
pub struct Resizable {
    pub is_vertical: bool,
    pub prev_size: Mutable<f64>,
    pub prev_percent: Mutable<f64>,
    pub prev_percent_memoize: Mutable<f64>,

    pub dragging: Mutable<bool>,
    pub parent_element: Mutable<Option<Element>>,
}

impl Resizable {
    pub fn new(prev_percent: f64, is_vertical: bool) -> Rc<Self> {
        Rc::new(Self {
            prev_percent: Mutable::new(prev_percent),
            prev_percent_memoize: Mutable::new(prev_percent),
            is_vertical,
            ..Default::default()
        })
    }

    pub fn new_with_mutable(prev_percent: Mutable<f64>, prev_percent_memoize: Mutable<f64>, is_vertical: bool) -> Rc<Self> {
        Rc::new(Self {
            prev_percent,
            prev_percent_memoize,
            is_vertical,
            ..Default::default()
        })
    }

    pub fn set_prev_percent(&self, new_percent: f64) {
        self.prev_percent.set_neq(new_percent);
    }

    pub fn set_prev_percent_full(&self) {
        self.prev_percent_memoize.set_neq(self.prev_percent.get());
        self.prev_percent.set_neq(100.0);
    }

    pub fn set_prev_percent_memoize(&self) {
        self.prev_percent.set_neq(self.prev_percent_memoize.get());
    }

    pub fn prev_style(&self) -> impl Signal<Item = String> + use<> {
        // 8px is a resizer width
        self.prev_percent.signal_cloned().map(|w| {
            if w <= 0.0 {
                String::from("0px")
            } else if w >= 100.0 {
                String::from("calc(100% - 8px)")
            } else {
                ["calc(", &w.to_string(), "% - 8px)"].concat()
            }
        })
    }

    pub fn next_style(&self) -> impl Signal<Item = String> + use<> {
        self.prev_percent.signal_cloned().map(|w| {
            if w <= 0.0 {
                String::from("calc(100%)")
            } else if w >= 100.0 {
                String::from("0px")
            } else {
                [&(100.0 - w).to_string(), "%"].concat()
            }
        })
    }

    pub fn prev_mixin<T>(resizable: Rc<Self>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
    where
        T: std::convert::AsRef<web_sys::HtmlElement> + std::convert::AsRef<web_sys::Element>,
    {
        #[inline]
        move |dom| {
            dom.style_signal(class::USER_SELECT, resizable.dragging.signal_cloned().map(|is_dragging| if is_dragging { "none" } else { "text" }))
                .style_signal("pointer-events", resizable.dragging.signal_cloned().map(|is_dragging| if is_dragging { "none" } else { "inherit" }))
                .apply(|d| {
                    if resizable.is_vertical {
                        d.class("resizer-container-top").style_signal("height", resizable.prev_style())
                    } else {
                        d.class("resizer-container-left").style_signal("width", resizable.prev_style())
                    }
                })
        }
    }

    pub fn next_mixin<T>(resizable: Rc<Self>) -> impl FnOnce(DomBuilder<T>) -> DomBuilder<T>
    where
        T: std::convert::AsRef<web_sys::HtmlElement> + std::convert::AsRef<web_sys::Element>,
    {
        #[inline]
        move |dom| {
            dom.style_signal(class::USER_SELECT, resizable.dragging.signal_cloned().map(|is_dragging| if is_dragging { "none" } else { "text" }))
                .style_signal("pointer-events", resizable.dragging.signal_cloned().map(|is_dragging| if is_dragging { "none" } else { "inherit" }))
                .apply(|d| {
                    if resizable.is_vertical {
                        d.class("resizer-container-bottom").style_signal("height", resizable.next_style())
                    } else {
                        d.class("resizer-container-right").style_signal("width", resizable.next_style())
                    }
                })
        }
    }

    pub fn on_mouse_move(e: &events::MouseMove, resize_state: Rc<ResizeState>) {
        if resize_state.is_dragging.get() {
            let dx = (e.mouse_x() - resize_state.mouse_x.get()) as f64;
            let dy = (e.mouse_y() - resize_state.mouse_y.get()) as f64;

            if let Some(resizable) = resize_state.resizable.lock_ref().as_ref() {
                if let Some(parent) = resizable.parent_element.lock_ref().as_ref() {
                    let rect = parent.get_bounding_client_rect();
                    if resizable.is_vertical {
                        let h = rect.height();
                        let mut new_percent = if h > 0.0 { (resizable.prev_size.get() + dy) * 100.0 / h } else { 55.0 };
                        if new_percent < 0.0 {
                            new_percent = 0.0;
                        }
                        resizable.prev_percent.set(new_percent);
                    } else {
                        let w = rect.width();
                        let mut new_percent = if w > 0.0 { (resizable.prev_size.get() + dx) * 100.0 / w } else { 55.0 };
                        if new_percent < 0.0 {
                            new_percent = 0.0;
                        }
                        resizable.prev_percent.set(new_percent);
                    }
                }
            }
        }
    }

    pub fn on_mouse_up(resize_state: Rc<ResizeState>) {
        if resize_state.is_dragging.get() {
            if let Some(resizable) = resize_state.resizable.lock_ref().as_ref() {
                resizable.dragging.set_neq(false);
            }
            resize_state.is_dragging.set_neq(false);
        }
    }

    pub fn render(resizable: Rc<Self>, state: Rc<ResizeState>) -> Dom {
        html!("div",{
            .class(if resizable.is_vertical {"resizer-vertical"} else {"resizer-horizontal"})
            .with_node!(element => {
                .event(clone!(state, resizable => move |e:events::MouseDown| {
                    state.is_dragging.set_neq(true);
                    state.mouse_x.set(e.mouse_x());
                    state.mouse_y.set(e.mouse_y());
                    state.resizable.set(Some(resizable.clone()));
                    resizable.parent_element.set(element.parent_element());
                    resizable.dragging.set_neq(true);
                    if let Some(prev_sib) = element.previous_element_sibling() {
                        let rect = prev_sib.get_bounding_client_rect();
                        if resizable.is_vertical {
                            resizable.prev_size.set(rect.height());
                        } else {
                            resizable.prev_size.set(rect.width());
                        }
                    }
                }))
            })
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ResizeState {
    pub is_dragging: Mutable<bool>,
    pub mouse_x: Mutable<i32>,
    pub mouse_y: Mutable<i32>,
    pub resizable: Mutable<Option<Rc<Resizable>>>,
}
