use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const ST5_INTRO: &str = r#"        ความเครียดเกิดขึ้นได้กับทุกคน สาเหตุที่ทำให้เกิดความเครียดมีหลายอย่าง เช่น รายได้ที่ไม่เพียงพอ หนี้สิน ภัยพิบัติต่างๆ ที่ทำให้เกิดความสูญเสีย ความเจ็บป่วย เป็นต้น ความเครียดมีทั้งประโยชน์และโทษ หากมากเกินไปจะเกิดผลเสียต่อร่างกายและจิตใจของท่านได้"#;
const ST5_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน    0 - 4  เครียดน้อย
                        คะแนน    5 - 7  เครียดปานกลาง
                        คะแนน    8 - 9  เครียดมาก
                        คะแนน  10 - 15  เครียดมากที่สุด
"#;

pub struct StressST5 {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3: Mutable<Option<u8>>,
    score_4: Mutable<Option<u8>>,
    score_5: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl StressST5 {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        let score_1 = split.get(1).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(2).and_then(|s| s.parse::<u8>().ok());
        let score_3 = split.get(3).and_then(|s| s.parse::<u8>().ok());
        let score_4 = split.get(4).and_then(|s| s.parse::<u8>().ok());
        let score_5 = split.get(5).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2, &score_3, &score_4, &score_5);
        Rc::new(Self {
            score_1: Mutable::new(score_1),
            score_2: Mutable::new(score_2),
            score_3: Mutable::new(score_3),
            score_4: Mutable::new(score_4),
            score_5: Mutable::new(score_5),
            score_total: Mutable::new(score_total.unwrap_or_default()),
            is_complete: Mutable::new(score_total.is_some()),
            parent_result,
            parent_changed,
        })
    }

    fn total_score_signal(&self) -> impl Signal<Item = Option<u8>> + use<> {
        map_ref! {
            let score_1 = self.score_1.signal(),
            let score_2 = self.score_2.signal(),
            let score_3 = self.score_3.signal(),
            let score_4 = self.score_4.signal(),
            let score_5 = self.score_5.signal() =>
            Self::cal_total(score_1, score_2, score_3, score_4, score_5)
        }
    }

    fn cal_total(score_1: &Option<u8>, score_2: &Option<u8>, score_3: &Option<u8>, score_4: &Option<u8>, score_5: &Option<u8>) -> Option<u8> {
        if let (Some(s1), Some(s2), Some(s3), Some(s4), Some(s5)) = (score_1, score_2, score_3, score_4, score_5) {
            Some(s1 + s2 + s3 + s4 + s5)
        } else {
            None
        }
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
                            html!("h5", {.class("modal-title").text("แบบประเมินความเครียด (ST-5)")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        .style("width", "100%")
                        .children([
                            html!("div", {
                                .class("mb-2")
                                .class(&*class::MONO_PRE_WRAP)
                                .text(ST5_INTRO)
                            }),
                            doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("ข้อที่")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("อาการหรือความรู้สึกที่เกิดในระยะ 2 - 4 สัปดาห์")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("เป็นน้อยมาก")
                                                    .child(html!("br"))
                                                    .text("หรือแทบไม่มี")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("เป็นบางครั้ง")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("เป็นบ่อยครั้ง")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("เป็นประจำ")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row(modal.score_1.clone(), 1, "มีปัญหาการนอน นอนไม่หลับหรือนอนมาก"),
                                            table_row(modal.score_2.clone(), 2, "มีสมาธิน้อยลง"),
                                            table_row(modal.score_3.clone(), 3, "หงุดหงิด / กระวนกระวาย / ว้าวุ่นใจ"),
                                            table_row(modal.score_4.clone(), 4, "รู้สึกเบื่อ เซ็ง"),
                                            table_row(modal.score_5.clone(), 5, "ไม่อยากพบปะผู้คน"),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center")
                                                    .attr("scope", "col")
                                                    .attr("colspan", "4")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(ST5_INTRERPRET)
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
                                        modal.score_3.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_4.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_5.get().map(|u| u.to_string()).unwrap_or_default(),
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
            doms::td_icon_value_u8_opt_match(mutable.clone(), 0),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 1),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 2),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 3),
        ])
    })
}
