use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const AWQ_V2_INTRERPRET: &str = r#"
        • ควรประเมินวันละ 1 ครั้ง และสิ้นสุดการประเมินเมื่อไม่มีอาการ ในทุกข้อคำถาม
        • เมื่อประเมินได้คะแนนระดับ "มีมาก" ขึ้นไปในแต่ละหัวข้อ ให้รายงานแพทย์ประจำตึกทันที
        • เมื่อประเมินอาการในข้อ 2 หรือ 3 ได้คะแนนระดับ "น้อยมาก" ขึ้นไป ให้ประเมินภาวะซึมเศร้า (9Q) และการฆ่าตัวตาย (8Q) ร่วมด้วย

"#;

pub struct AmphetamineAwqV2 {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3: Mutable<Option<u8>>,
    score_4: Mutable<Option<u8>>,
    score_5: Mutable<Option<u8>>,
    score_6: Mutable<Option<u8>>,
    score_7: Mutable<Option<u8>>,
    score_8: Mutable<Option<u8>>,
    score_9: Mutable<Option<u8>>,
    score_10: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    score_total_h: Mutable<u8>,
    score_total_a: Mutable<u8>,
    score_total_r: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl AmphetamineAwqV2 {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        // 0 is total, 1 is awq_h, 2 is awq_a, 3 is awq_r
        let score_1 = split.get(4).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(5).and_then(|s| s.parse::<u8>().ok());
        let score_3 = split.get(6).and_then(|s| s.parse::<u8>().ok());
        let score_4 = split.get(7).and_then(|s| s.parse::<u8>().ok());
        let score_5 = split.get(8).and_then(|s| s.parse::<u8>().ok());
        let score_6 = split.get(9).and_then(|s| s.parse::<u8>().ok());
        let score_7 = split.get(10).and_then(|s| s.parse::<u8>().ok());
        let score_8 = split.get(11).and_then(|s| s.parse::<u8>().ok());
        let score_9 = split.get(12).and_then(|s| s.parse::<u8>().ok());
        let score_10 = split.get(13).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2, &score_3, &score_4, &score_5, &score_6, &score_7, &score_8, &score_9, &score_10);
        let score_total_h = Self::cal_hyperarousal(&score_1, &score_6, &score_9);
        let score_total_a = Self::cal_anxiety(&score_3, &score_4, &score_5);
        let score_total_r = Self::cal_rev_vegetative(&score_7, &score_8, &score_10);
        Rc::new(Self {
            score_1: Mutable::new(score_1),
            score_2: Mutable::new(score_2),
            score_3: Mutable::new(score_3),
            score_4: Mutable::new(score_4),
            score_5: Mutable::new(score_5),
            score_6: Mutable::new(score_6),
            score_7: Mutable::new(score_7),
            score_8: Mutable::new(score_8),
            score_9: Mutable::new(score_9),
            score_10: Mutable::new(score_10),
            score_total: Mutable::new(score_total.unwrap_or_default()),
            score_total_h: Mutable::new(score_total_h.unwrap_or_default()),
            score_total_a: Mutable::new(score_total_a.unwrap_or_default()),
            score_total_r: Mutable::new(score_total_r.unwrap_or_default()),
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
            let score_5 = self.score_5.signal(),
            let score_6 = self.score_6.signal(),
            let score_7 = self.score_7.signal(),
            let score_8 = self.score_8.signal(),
            let score_9 = self.score_9.signal(),
            let score_10 = self.score_10.signal() =>
            Self::cal_total(score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9, score_10)
        }
    }

    fn cal_total(
        score_1: &Option<u8>,
        score_2: &Option<u8>,
        score_3: &Option<u8>,
        score_4: &Option<u8>,
        score_5: &Option<u8>,
        score_6: &Option<u8>,
        score_7: &Option<u8>,
        score_8: &Option<u8>,
        score_9: &Option<u8>,
        score_10: &Option<u8>,
    ) -> Option<u8> {
        if let (Some(s1), Some(s2), Some(s3), Some(s4), Some(s5), Some(s6), Some(s7), Some(s8), Some(s9), Some(s10)) =
            (score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9, score_10)
        {
            Some(s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9 + s10)
        } else {
            None
        }
    }

    fn hyperarousal_score_signal(&self) -> impl Signal<Item = Option<u8>> + use<> {
        map_ref! {
            let score_1 = self.score_1.signal(),
            let score_6 = self.score_6.signal(),
            let score_9 = self.score_9.signal() =>
            Self::cal_hyperarousal(score_1, score_6, score_9)
        }
    }

    fn cal_hyperarousal(score_1: &Option<u8>, score_6: &Option<u8>, score_9: &Option<u8>) -> Option<u8> {
        if let (Some(s1), Some(s6), Some(s9)) = (score_1, score_6, score_9) {
            Some(s1 + s6 + s9)
        } else {
            None
        }
    }

    fn anxiety_score_signal(&self) -> impl Signal<Item = Option<u8>> + use<> {
        map_ref! {
            let score_3 = self.score_3.signal(),
            let score_4 = self.score_4.signal(),
            let score_5 = self.score_5.signal() =>
            Self::cal_anxiety(score_3, score_4, score_5)
        }
    }

    fn cal_anxiety(score_3: &Option<u8>, score_4: &Option<u8>, score_5: &Option<u8>) -> Option<u8> {
        if let (Some(s3), Some(s4), Some(s5)) = (score_3, score_4, score_5) {
            Some(s3 + s4 + s5)
        } else {
            None
        }
    }

    fn rev_vegetative_score_signal(&self) -> impl Signal<Item = Option<u8>> + use<> {
        map_ref! {
            let score_7 = self.score_7.signal(),
            let score_8 = self.score_8.signal(),
            let score_10 = self.score_10.signal() =>
            Self::cal_rev_vegetative(score_7, score_8, score_10)
        }
    }

    fn cal_rev_vegetative(score_7: &Option<u8>, score_8: &Option<u8>, score_10: &Option<u8>) -> Option<u8> {
        if let (Some(s7), Some(s8), Some(s10)) = (score_7, score_8, score_10) {
            Some(s7 + s8 + s10)
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
            .future(modal.hyperarousal_score_signal().for_each(clone!(modal => move |total_score| {
                if let Some(total) = total_score {
                    modal.score_total_h.set_neq(total);
                }
                async {}
            })))
            .future(modal.anxiety_score_signal().for_each(clone!(modal => move |total_score| {
                if let Some(total) = total_score {
                    modal.score_total_a.set_neq(total);
                }
                async {}
            })))
            .future(modal.rev_vegetative_score_signal().for_each(clone!(modal => move |total_score| {
                if let Some(total) = total_score {
                    modal.score_total_r.set_neq(total);
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
                            html!("div", {
                                .children([
                                    html!("h5", {.class("modal-title").text("แบบประเมินอาการถอนพิษยาแอมเฟตามีน")}),
                                    html!("div", {.text("Amphetamine Withdrawal Questionnaire Version 2 : AWQv2")}),
                                ])
                            }),
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
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("ลำดับที่")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .text("อาการ")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("ไม่มีเลย")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("มีน้อยมาก")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("มีพอควร")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("มีมาก")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("มีมากอย่างยิ่ง")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row(modal.score_1.clone(), 1, "ความรู้สึกอยากยา"),
                                            table_row(modal.score_2.clone(), 2, "รู้สึกซึมเศร้า"),
                                            table_row(modal.score_3.clone(), 3, "รู้สึกเบื่อ หมดความหวัง หรือความสุขใจ"),
                                            table_row(modal.score_4.clone(), 4, "รู้สึกวิตกกังวล"),
                                            table_row(modal.score_5.clone(), 5, "รู้สึกเคลื่อนไหวเชื่องช้า"),
                                            table_row(modal.score_6.clone(), 6, "รู้สึกกระวนกระวาย"),
                                            table_row(modal.score_7.clone(), 7, "ไม่มีเรี่ยงแรง หรืออ่อนเพลีย"),
                                            table_row(modal.score_8.clone(), 8, "รู้สึกอยากอาหารมากขึ้น หรือทานอาหารมากขึ้น"),
                                            table_row(modal.score_9.clone(), 9, "ฝันร้าย หรือรู้สึกว่าฝันเหมือนจริง"),
                                            table_row(modal.score_10.clone(), 10, "รู้สึกอยากนอน หรือนอนมาก"),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนกลุ่มอาการ Hyperarousal (ข้อ 1, 6, 9)")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "4")
                                                    .text_signal(modal.hyperarousal_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนกลุ่มอาการ Anxiety (ข้อ 3, 4, 5)")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "4")
                                                    .text_signal(modal.anxiety_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนกลุ่มอาการ Reversed Vegetative (ข้อ 7, 8, 10)")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "4")
                                                    .text_signal(modal.rev_vegetative_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "4")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(AWQ_V2_INTRERPRET)
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
                                        modal.score_total_h.get().to_string(),
                                        modal.score_total_a.get().to_string(),
                                        modal.score_total_r.get().to_string(),
                                        modal.score_1.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_2.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_3.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_4.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_5.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_6.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_7.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_8.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_9.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_10.get().map(|u| u.to_string()).unwrap_or_default(),
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
            doms::td_icon_value_u8_opt_match(mutable.clone(), 4),
        ])
    })
}
