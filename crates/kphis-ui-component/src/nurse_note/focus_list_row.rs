use dominator::{Dom, html, text};
use std::rc::Rc;

use kphis_model::focus_list::FocusList;
use kphis_ui_core::class;
use kphis_util::datetime::{date_th_opt, time_hm, time_hm_opt};

pub fn render(i: usize, focus: Rc<FocusList>, edit_dom: Dom) -> Dom {
    let focus_text = focus.focus_text.clone().unwrap_or_default();
    let goal_text = focus.goal_text.clone().unwrap_or_default();

    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&(i + 1).to_string())}),
            html!("td", {
                .child(text(&focus.focus_name.clone().unwrap_or_default()))
                .apply_if(!focus_text.is_empty(), |dom| {
                    dom.children([
                        text(" : "),
                        html!("U", {.child(text(&focus_text))}),
                    ])
                })
            }),
            html!("td", {
                .apply_if(focus.goals.is_some(), |goal| {
                    // goal.apply_if(has_goal_text, |goal_txt| {
                    //     goal_txt.children(doms::square_bracket_to_span(&focus.goal_text.clone().unwrap_or_default())).child(html!("br"))
                    // })
                    goal.children(focus.goals.clone().unwrap_or_default().split('|').filter(|g| !g.is_empty()).flat_map(|g| {
                        let mut gs = vec![text("- ")];
                        let id_goal = g.split('^').collect::<Vec<&str>>();
                        if id_goal.len() == 2 {
                            gs.push(text(id_goal[1]));
                            // gs.extend(doms::square_bracket_to_span(id_goal[1]));
                            if id_goal[0].parse::<u32>().unwrap_or_default() == 999 && !goal_text.is_empty() {
                                gs.push(text(" : "));
                                gs.push(text(&goal_text));
                                // gs.extend(doms::square_bracket_to_span(&goal_text));
                            }
                        } else {
                            gs.push(text(id_goal[0]));
                            // gs.extend(doms::square_bracket_to_span(id_goal[0]));
                        }
                        gs.push(html!("br"));
                        gs
                    }).collect::<Vec<Dom>>())
                })
            }),
            html!("td", {
                .children([
                    text(&date_th_opt(&focus.fclist_stdate)),
                    html!("br"),
                    text(&time_hm(&focus.fclist_sttime)),
                ])
            }),
            html!("td", {
                .apply(|dom| {
                    if focus.dchdate.is_some() && focus.fclist_status.as_str() == "1" {
                        dom.child(html!("label", {
                            .apply_if(focus.dchtype == Some(String::from("01")), |d| d.class("text-danger"))
                            .children([
                                text(&date_th_opt(&focus.dchdate)),
                                html!("br"),
                                text(&time_hm_opt(&focus.dchtime)),
                            ])
                        }))
                    } else if focus.fclist_enddate.is_some() {
                        dom.children([
                            text(&date_th_opt(&focus.fclist_enddate)),
                            html!("br"),
                            text(&time_hm_opt(&focus.fclist_endtime)),
                        ])
                    } else {
                        dom
                    }
                })
            }),
            html!("td", {
                .class(class::NOWRAP_C)
                .apply(|dom| {
                    let status = focus.fclist_status.as_str();
                    if status == "1" {
                        if focus.dchdate.is_some() {
                            if focus.dchtype == Some(String::from("01")) {
                                dom.child(html!("label", {
                                    .children([
                                        html!("h5", {
                                            .children([
                                                html!("i", {
                                                    .class(class::FA_CHECK_CIRCLE)
                                                    .class("text-success")
                                                }),
                                                html!("span", {
                                                    .class(class::BADGE_WRAP_R_GREEN)
                                                    .style("cursor","default")
                                                    .text("ปัญหาหมดไป")
                                                })
                                            ])
                                        }),
                                        text(&focus.dchtype_name.clone().unwrap_or_default()),
                                    ])
                                }))
                            } else {
                                dom.child(html!("label", {
                                    .class("text-danger")
                                    .text(&focus.dchtype_name.clone().unwrap_or_default())
                                }))
                            }
                        } else {
                            dom.child(html!("h5", {
                                .children([
                                    html!("i", {.class(class::FA_CLIPBOARD)}),
                                    html!("span", {
                                        .class(class::BADGE_WRAP_R_RED)
                                        .style("cursor","default")
                                        .text("ปัญหายังคงอยู่")
                                    })
                                ])
                            }))
                        }
                    } else if status == "2" {
                        dom.child(html!("h5", {
                            .children([
                                html!("i", {.class(class::FA_CHECK_CIRCLE)}),
                                html!("span", {
                                    .class(class::BADGE_WRAP_R_GREEN)
                                    .style("cursor","default")
                                    .text("ปัญหาหมดไป")
                                })
                            ])
                        }))
                    } else {
                        dom
                    }
                })
            }),
            edit_dom,
        ])
    })
}
