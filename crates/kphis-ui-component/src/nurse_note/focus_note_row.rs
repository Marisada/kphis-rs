use dominator::{Dom, clone, events, html, text};
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{ops::Deref, rc::Rc};

use kphis_model::{
    app::VisitTypeId,
    {focus_list::FocusList, focus_note::FocusNote, image::file_path::ImageUsage},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::{date_th_opt, js_now, time_hm_opt},
    util::zero_none,
};

use crate::gadget::image::ImageCpn;

pub fn render(
    i: usize,
    note_tuple: Rc<(VisitTypeId, Rc<FocusNote>)>,
    parent_focus_note: Option<Mutable<Option<Rc<FocusNote>>>>,
    focus_list: MutableVec<Rc<FocusList>>,
    can_edit: bool,
    app: Rc<App>,
) -> Dom {
    let is_ipd = note_tuple.0.is_ipd();
    let note = note_tuple.1.clone();

    let type_color = match (app.app_asset.lock_ref().as_ref(), &note.fcnote_patient_type) {
        (Some(asset), Some(note_pt_type)) => asset
            .fcnote_patient_type_select_options
            .iter()
            .find(|op| op.key.as_str() == note_pt_type)
            .map(|op| op.color.clone())
            .unwrap_or_default(),
        _ => String::new(),
    };

    html!("tr", {
        .child(html!("td", {.class("text-center").text(&(i + 1).to_string())}))
        .apply(|dom| {
            if let Some(parent_note) = &parent_focus_note {
                dom.class_signal("table-info", parent_note.signal_cloned().map(clone!(note => move |opt| {
                    opt.as_ref().map(|parent| parent.fcnote_id == note.fcnote_id).unwrap_or_default()
                })))
            } else {
                dom
            }
        })
        .children([
            html!("td", {
                .children([
                    text(&date_th_opt(&note.fcnote_date)),
                    html!("br"),
                    text(&time_hm_opt(&note.fcnote_time)),
                ])
            }),
            html!("td", {
                .class("text-center")
                .child(doms::color_prefix_span(&type_color))
                .text(&note.fcnote_patient_type.clone().unwrap_or_default())
            }),
            html!("td", {
                .children([
                    text(&note.focus_name.clone().unwrap_or_default()),
                    text(" "),
                    html!("U", {.child(text(&note.focus_text.clone().unwrap_or_default()))}),
                ])
            }),
            html!("td", {
                .class("p-0")
                .child(html!("div", {
                    .class("p-2")
                    .style("max-height","30vh")
                    .style("overflow-y","auto")
                    .child(html!("div", {
                        .style("white-space","pre-wrap")
                        .apply(|dom| {
                            if let Some(general_symptoms) = &note.general_symptoms {
                                dom.children(doms::square_bracket_to_span(general_symptoms))
                                .child(html!("br"))
                            } else {
                                dom
                            }
                        })
                        .apply_if(note.fclist_id.is_some(), |dom| dom
                            .apply(|fcl| {
                                let image_usage = if is_ipd {
                                    ImageUsage::IpdFocusNoteAssessment
                                } else {
                                    ImageUsage::OpdErFocusNoteAssessment
                                };
                                if let Some(assessment) = &note.assessment {
                                    fcl.child(html!("B", {.text("A : ")}))
                                    .child(html!("br"))
                                    .children(doms::square_bracket_to_span(assessment))
                                    .child(html!("div", {
                                        .child(ImageCpn::render("170px", ImageCpn::new_with_key(
                                            image_usage,
                                            note.fcnote_id,
                                            false, // read-only
                                            Mutable::new(None),
                                            None, // TODO fix if has any AN or VN here
                                            "", // will use ImageUsage internally, so we add nothing here
                                        ), app.clone()))
                                    }))
                                } else {
                                    fcl.children([
                                        html!("B", {.text("A : -")}),
                                        html!("div"),
                                    ])
                                }
                            })
                            .child(html!("B", {.text("I :")}))
                            .child(html!("br"))
                            .apply(|intvt| {
                                if let Some(intvts) = &note.intvts {
                                    let names = intvts.split('|').filter_map(|i| {
                                        let id_intvt = i.split('^').collect::<Vec<&str>>();
                                        if id_intvt.len() == 2 {
                                            Some(if id_intvt[0].parse::<u32>().unwrap_or_default() == 9999 {
                                                // add support for inner list style
                                                note.intvt_text.as_ref().map(|s| s.trim_start_matches("- ").to_owned()).unwrap_or_default()
                                                // [id_intvt[1], &note.intvt_text.as_ref().map(|txt| [" : ", txt].concat()).unwrap_or_default()].concat()
                                            } else {
                                                String::from(id_intvt[1])
                                            })
                                        } else {
                                            None
                                        }
                                    }).collect::<Vec<String>>();
                                    intvt.children(names.iter().flat_map(|name| {
                                        let mut ints = vec![text("- ")];
                                        ints.extend(doms::square_bracket_to_span(name));
                                        ints.push(html!("br"));
                                        ints
                                    }))
                                    // .apply_if(intvts_len > 0, |sp| sp.child(text(", ")))
                                    // .apply_if(note.intvt_text.is_some(), |intvt_txt| {
                                    //     intvt_txt.children(doms::square_bracket_to_span(&note.intvt_text.clone().unwrap_or_default()))
                                    // })
                                } else {
                                    intvt
                                }
                            })
                            //.child(html!("br"))
                            .apply(|dlc| {
                                if let Some(dlcs) = &note.dlcs {
                                    dlc.children(dlcs.split('|').flat_map(|id_name| {
                                        id_name.trim().split('^').next_back().map(|name| {
                                            let mut ds = vec![text("- ")];
                                            ds.push(text(name));
                                            // ds.extend(doms::square_bracket_to_span(name));
                                            ds.push(html!("br"));
                                            ds
                                        }).unwrap_or_default()
                                    }).collect::<Vec<Dom>>())
                                } else {
                                    dlc
                                }
                            })
                            .apply(|dlc_txt| {
                                if let Some(dlc_text) = &note.dlc_text {
                                    let mut ds = vec![text("- ")];
                                    ds.push(text(dlc_text));
                                    // ds.extend(doms::square_bracket_to_span(dlc_text));
                                    ds.push(html!("br"));
                                    dlc_txt.children(ds)
                                } else {
                                    dlc_txt
                                }
                            })
                            .apply(|eva| {
                                let image_usage = if is_ipd {
                                    ImageUsage::IpdFocusNoteEvaluation
                                } else {
                                    ImageUsage::OpdErFocusNoteEvaluation
                                };
                                if let Some(evalution) = &note.evalution {
                                    eva.child(html!("B", {.text("E : ")}))
                                    .children(doms::square_bracket_to_span(evalution))
                                    .child(html!("div", {
                                        .child(ImageCpn::render("170px", ImageCpn::new_with_key(
                                            image_usage,
                                            note.fcnote_id,
                                            false, // read-only
                                            Mutable::new(None),
                                            None, // TODO fix if has any AN or VN here
                                            "", // will use ImageUsage internally, so we add nothing here
                                        ), app.clone()))
                                    }))
                                } else {
                                    eva.children([
                                        html!("B", {.text("E : -")}),
                                        html!("div"),
                                    ])
                                }
                            })
                            .apply(|oth| {
                                if let Some(other) = &note.other {
                                    oth.child(html!("br"))
                                    .child(html!("B", {.text("หมายเหตุ : ")}))
                                    .children(doms::square_bracket_to_span(other))
                                } else {
                                    oth
                                }
                            })
                        )
                    }))
                }))
            }),
            html!("td", {
                .child(html!("div", {
                    .style("font-size","14px")
                    .children([
                        text(&note.user_name.clone().unwrap_or_default()),
                        // html!("br"),
                        // text(&note.entryposition.clone().unwrap_or_default()),
                    ])
                }))
                // show edit and clone buttons when
                // - can send focus_note
                // - can edit
                // - has fclist_id that FocusList.fclist_status == 1 in focus_list
                .apply(|dom| {
                    if can_edit {
                        if let Some(parent_fcnote) = parent_focus_note {
                            dom.child_signal(focus_list.signal_vec_cloned().to_signal_cloned().map(move |list| {
                                let focus_not_done = note.fclist_id.and_then(zero_none).is_none() || list.iter().any(|focus| Some(focus.fclist_id) == note.fclist_id && focus.fclist_status == *"1");
                                focus_not_done.then(|| {
                                    html!("div", {
                                        .class(class::TXT_CB)
                                        .apply_if(app.doctor_code().map(|code| code == note.doctorcode.clone().unwrap_or_default()).unwrap_or_default(), |d|{
                                            d.child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_L_GRAY)
                                                .child(html!("i", {.class(class::FA_EDIT)}))
                                                .event(clone!(parent_fcnote, note => move |_: events::Click| {
                                                    parent_fcnote.set(Some(note.clone()));
                                                }))
                                            }))
                                        })
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_GRAY)
                                            .child(html!("i", {.class(class::FA_CLONE)}))
                                            .event(clone!(parent_fcnote, note => move |_: events::Click| {
                                                let now = js_now();
                                                let mut new_note = note.deref().to_owned();
                                                new_note.fcnote_id = 0;
                                                new_note.fcnote_date = Some(now.date());
                                                new_note.fcnote_time = None;
                                                new_note.version = 0;
                                                new_note.user_name = None;
                                                new_note.entryposition = None;
                                                parent_fcnote.set(Some(Rc::new(new_note)));
                                            }))
                                        }))
                                    })
                                })
                            }))
                        } else {
                            dom
                        }
                    } else {
                        dom
                    }
                })
            }),
        ])
    })
}
