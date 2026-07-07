use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const Q2_INTRERPRET: &str = r#"        หมายเหตุ: กรณี มี 1 ข้อขึ้นไป ให้ประเมินภาวะซึมเศร้า 9 คำถาม (9Q) ต่อ
"#;

pub struct Depress2Q {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl Depress2Q {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        let score_1 = split.get(1).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(2).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2);
        Rc::new(Self {
            score_1: Mutable::new(score_1),
            score_2: Mutable::new(score_2),
            score_total: Mutable::new(score_total.unwrap_or_default()),
            is_complete: Mutable::new(score_total.is_some()),
            parent_result,
            parent_changed,
        })
    }

    fn total_score_signal(&self) -> impl Signal<Item = Option<u8>> + use<> {
        map_ref! {
            let score_1 = self.score_1.signal(),
            let score_2 = self.score_2.signal() =>
            Self::cal_total(score_1, score_2)
        }
    }

    fn cal_total(score_1: &Option<u8>, score_2: &Option<u8>) -> Option<u8> {
        if let (Some(s1), Some(s2)) = (score_1, score_2) { Some(s1 + s2) } else { None }
    }

    pub fn render(modal: Rc<Self>) -> Dom {
        html!("div", {
            .future(modal.total_score_signal().for_each(clone!(modal => move |total_score| {
                if let Some(total) = total_score {
                    modal.score_total.set_neq(total);
                    modal.is_complete.set_neq(true);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_XL_FULL)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {.class("modal-title").text("แบบประเมินภาวะซึมเศร้า 2 คำถาม (2Q)")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        .style("width", "100%")
                        .children([
                            doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("ลำดับที่")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("คำถาม")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("มี")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ไม่มี")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row(modal.score_1.clone(), 1, "ใน 2 สัปดาห์ที่ผ่านมารวมวันนี้ ท่านรู้สึกหดหู่ เศร้า หรือท้อแท้สิ้นหวังหรือไม่"),
                                            table_row(modal.score_2.clone(), 2, "ใน 2 สัปดาห์ที่ผ่านมารวมวันนี้ ท่านรู้สึกเบื่อ ทำอะไรก็ไม่เพลิดเพลินหรือไม่"),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "2")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(Q2_INTRERPRET)
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .attr("data-bs-dismiss", "modal")
                            .class("btn")
                            .class_signal("btn-secondary", not(modal.is_complete.signal()))
                            .class_signal("btn-primary", modal.is_complete.signal())
                            .text_signal(modal.is_complete.signal().map(|is_complete| {
                                if is_complete {
                                    "เรียบร้อย"
                                } else {
                                    "ยกเลิก"
                                }
                            }))
                            .event(move |_:events::Click| {
                                if modal.is_complete.get() {
                                    let concat = [
                                        modal.score_total.get().to_string(),
                                        modal.score_1.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_2.get().map(|u| u.to_string()).unwrap_or_default(),
                                    ].join(",");
                                    modal.parent_result.set_neq(concat);
                                    modal.parent_changed.set_neq(true);
                                }
                            })
                        }))
                    }),
                ])
            }))
        })
    }
}

fn table_row(mutable: Mutable<Option<u8>>, i: usize, title: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            html!("td", {.text(title)}),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 1),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 0),
        ])
    })
}
