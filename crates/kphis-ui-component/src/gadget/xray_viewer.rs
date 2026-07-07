use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

use kphis_model::{
    PATH_PREFIX_API_XRAY_IMAGE,
    endpoint::QueryString,
    pacs::{PacsImageData, PacsParams},
};

use kphis_ui_core::{binding::Zoomist, class, mixins};
use kphis_util::util::str_some;

#[derive(Default)]
pub struct XrayViewer {
    changed: Mutable<bool>,
    image_spinner: Mutable<bool>,
    render_zoomist: Mutable<bool>,
    zoomist: Mutable<Option<Rc<Zoomist>>>,
    selected_image: Mutable<Option<PacsImageData>>,
    mono: Mutable<String>,     // 0%, 100%
    bright: Mutable<String>,   // 30-200
    contrast: Mutable<String>, // 20-200
}

impl XrayViewer {
    pub fn new(selected_image: Mutable<Option<PacsImageData>>) -> Rc<Self> {
        Rc::new(Self {
            selected_image,
            mono: Mutable::new(String::from("0%")),
            bright: Mutable::new(String::from("100")),
            contrast: Mutable::new(String::from("100")),
            ..Default::default()
        })
    }

    pub fn render(cpn_id: &'static str, page: Rc<Self>) -> Dom {
        html!("div", {
            .class("xray-viewer")
            .children([
                html!("div", {
                    .class("xray-viewer-head")
                    .child_signal(page.image_spinner.signal().map(clone!(page => move |spin| {
                        if spin {
                            Some(html!("i",{
                                .class(class::FA_SPIN)
                                .style("font-size","32px")
                                .style("padding-top", "10px")
                            }))
                        } else {
                            Some(html!("i", {
                                .class(class::FA_UNDO)
                                .style("font-size", "32px")
                                .style("padding-top", "10px")
                                .event(clone!(page => move |_:events::Click| {
                                    page.mono.set_neq(String::from("0%"));
                                    page.bright.set_neq(String::from("100"));
                                    page.contrast.set_neq(String::from("100"));
                                    page.render_zoomist.set(true);
                                }))
                            }))
                        }
                    })))
                    .children([
                        html!("div", {
                            .class("mono-switch")
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .attr("id", &["vMono-", cpn_id].concat())
                                    .apply(mixins::checkbox_toggle(page.mono.clone(), page.changed.clone(), "100%", "0%"))
                                }),
                                html!("label", {
                                    .attr("for", &["vMono-", cpn_id].concat())
                                    .attr("title", "Monochrome")
                                    .attr("data-on", "B/W")
                                    .attr("data-off", "W/B")
                                    .class("mono-switch-inner")
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("v-range")
                            .child(html!("label", {
                                .attr("for", &["vBright-", cpn_id].concat())
                                .children([
                                    html!("span", {
                                        .class("v-range-label")
                                        .text("Brightness")
                                    }),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "range")
                                        .attr("id", &["vBright-", cpn_id].concat())
                                        .attr("min", "30")
                                        .attr("max", "200")
                                        .attr("step", "1")
                                        .apply(mixins::string_value(page.bright.clone(), page.changed.clone()))
                                    }),
                                    html!("span", {
                                        .class("v-triangle")
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("v-range")
                            .child(html!("label", {
                                .attr("for", &["vContrast-", cpn_id].concat())
                                .children([
                                    html!("span", {
                                        .class("v-range-label")
                                        .text("Contrast")
                                    }),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "range")
                                        .attr("id", &["vContrast-", cpn_id].concat())
                                        .attr("min", "30")
                                        .attr("max", "200")
                                        .attr("step", "1")
                                        .apply(mixins::string_value(page.contrast.clone(), page.changed.clone()))
                                    }),
                                    html!("span", {
                                        .class("v-triangle")
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .attr("id", &["pacs-image-", cpn_id].concat())
                    .class("zoomist-container")
                    // .style("height", "calc(100% - 50px)")
                    .future(page.selected_image.signal_cloned().map(|opt| opt.is_some()).dedupe().for_each(clone!(page => move |is_selected| {
                        if is_selected {
                            page.render_zoomist.set(is_selected);
                        }
                        async {}
                    })))
                    .future(page.render_zoomist.signal().for_each(clone!(page => move |is_render| {
                        if is_render {
                            if let Some(zoomist) = page.zoomist.get_cloned() {
                                zoomist.destroy(&JsValue::from_bool(true));
                            }
                            page.zoomist.set(Some(Rc::new(Zoomist::new(&["#pacs-image-", cpn_id].concat()))));
                            page.render_zoomist.set(false);
                        }
                        async {}
                    })))
                    .child(html!("div", {
                        .class("zoomist-wrapper")
                        .style("background-color", "black")
                        .child(html!("div", {
                            .class("zoomist-image")
                            .style("width", "100%")
                            .child_signal(page.selected_image.signal_cloned().map(clone!(page => move |opt| {
                                opt.map(clone!(page => move |image| {
                                    page.image_spinner.set(true);
                                    html!("img", {
                                        .style("width", "100%")
                                        .style("height", "100%")
                                        .style("object-fit", "cover")
                                        .style("object-position", "center")
                                        .style_signal("filter", map_ref!(
                                            let mono = page.mono.signal_cloned(),
                                            let bright = page.bright.signal_cloned(),
                                            let contrast = page.contrast.signal_cloned() =>
                                            ["invert(", mono, ") brightness(", bright, "%) contrast(", contrast, "%)"].concat()
                                        ))
                                        .attr("src", &[PATH_PREFIX_API_XRAY_IMAGE, &PacsParams {
                                            study_uid: str_some(image.study_uid.to_owned()),
                                            series_uid: str_some(image.series_uid.to_owned()),
                                            object_uid: str_some(image.object_uid.to_owned()),
                                            file_path: str_some(image.file_path.to_owned()),
                                            ..Default::default()
                                        }.query_string()].concat())
                                        .attr("alt", "image")
                                        .event(clone!(page => move |_:events::Load| {
                                            page.image_spinner.set(false);
                                        }))
                                        .with_node!(element => {
                                            .event(move |_:events::Error| {
                                                element.set_hidden(true);
                                                page.image_spinner.set(false);
                                            })
                                        })
                                    })
                                }))
                            })))
                        }))
                    }))
                }),
            ])
        })
    }
}
