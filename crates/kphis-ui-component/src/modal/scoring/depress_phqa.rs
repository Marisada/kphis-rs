use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const DEPRESS_PHQ_A_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน    0 - 4  ไม่มีภาวะซึมเศร้า
                        คะแนน    5 - 9  มีภาวะซึมเศร้าเล็กน้อย  ควรหากิจกรรมที่ช่วยผ่อนคลายอารมณ์ หรือปรึกษาบุคคลใกล้ชิดที่ไว้ใจ
                        คะแนน  10 - 14  มีภาวะซึมเศร้าปานกลาง ควรปรึกษาแพทย์
                        คะแนน  15 - 19  มีภาวะซึมเศร้ามาก ควรปรึกษาแพทย์
                        คะแนน  20 - 27  มีภาวะซึมเศร้ารุนแรง ควรปรึกษาแพทย์
        หมายเหตุ : หากพบความเสี่ยงต่อการฆ่าตัวตาย จากข้อ 9 หรือ 2 ข้อคำถามเพิ่ม วัยรุ่นควรได้รับการประเมินความเสี่ยงการฆ่าตัวตายและเฝ้าระวังการฆ่าตัวตาย แม้คะแนนรวมจะไม่ถึงเกณฑ์ก็ตาม
        การวินิจฉัยโรคซึมเศร้าด้วยแบบประเมิน PHQ-A โดยอ้างอิงตามเกณฑ์การวินิจฉัยโรคซึมเศร้า คือ 5 ใน 9 ข้อ ได้ 2 คะแนนขึ้นไป (ยกเว้น ข้อ 9 ได้ 1 คะแนนขึ้นไป) โดยใน 5 ข้อนั้น ต้องมีข้อ 1 และข้อ 2 รวมอยู่ด้วย 
"#;

pub struct DepressPhqA {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3: Mutable<Option<u8>>,
    score_4: Mutable<Option<u8>>,
    score_5: Mutable<Option<u8>>,
    score_6: Mutable<Option<u8>>,
    score_7: Mutable<Option<u8>>,
    score_8: Mutable<Option<u8>>,
    score_9: Mutable<Option<u8>>,
    score_s1: Mutable<Option<u8>>,
    score_s2: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_diagnosis: Mutable<bool>,
    is_suicide: Mutable<bool>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl DepressPhqA {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        // 0 is total, 1 is diagnosis, 2 = is suicide
        let score_1 = split.get(3).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(4).and_then(|s| s.parse::<u8>().ok());
        let score_3 = split.get(5).and_then(|s| s.parse::<u8>().ok());
        let score_4 = split.get(6).and_then(|s| s.parse::<u8>().ok());
        let score_5 = split.get(7).and_then(|s| s.parse::<u8>().ok());
        let score_6 = split.get(8).and_then(|s| s.parse::<u8>().ok());
        let score_7 = split.get(9).and_then(|s| s.parse::<u8>().ok());
        let score_8 = split.get(10).and_then(|s| s.parse::<u8>().ok());
        let score_9 = split.get(11).and_then(|s| s.parse::<u8>().ok());
        let score_s1 = split.get(12).and_then(|s| s.parse::<u8>().ok());
        let score_s2 = split.get(13).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2, &score_3, &score_4, &score_5, &score_6, &score_7, &score_8, &score_9);
        let is_dianosis = Self::cal_diagnosis(&score_1, &score_2, &score_3, &score_4, &score_5, &score_6, &score_7, &score_8, &score_9);
        let is_suicide = Self::cal_suicide(&score_9, &score_s1, &score_s2);
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
            score_s1: Mutable::new(score_s1),
            score_s2: Mutable::new(score_s2),
            score_total: Mutable::new(score_total.unwrap_or_default()),
            is_diagnosis: Mutable::new(is_dianosis),
            is_suicide: Mutable::new(is_suicide),
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
            let score_9 = self.score_9.signal()  =>
            Self::cal_total(score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9)
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
    ) -> Option<u8> {
        if let (Some(s1), Some(s2), Some(s3), Some(s4), Some(s5), Some(s6), Some(s7), Some(s8), Some(s9)) = (score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9) {
            Some(s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9)
        } else {
            None
        }
    }

    fn diagnosis_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let score_1 = self.score_1.signal(),
            let score_2 = self.score_2.signal(),
            let score_3 = self.score_3.signal(),
            let score_4 = self.score_4.signal(),
            let score_5 = self.score_5.signal(),
            let score_6 = self.score_6.signal(),
            let score_7 = self.score_7.signal(),
            let score_8 = self.score_8.signal(),
            let score_9 = self.score_9.signal()  =>
            Self::cal_diagnosis(score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9)
        }
    }

    fn cal_diagnosis(
        score_1: &Option<u8>,
        score_2: &Option<u8>,
        score_3: &Option<u8>,
        score_4: &Option<u8>,
        score_5: &Option<u8>,
        score_6: &Option<u8>,
        score_7: &Option<u8>,
        score_8: &Option<u8>,
        score_9: &Option<u8>,
    ) -> bool {
        if let (Some(s1), Some(s2), Some(s3), Some(s4), Some(s5), Some(s6), Some(s7), Some(s8), Some(s9)) = (score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9) {
            let b1 = *s1 > 1;
            let b2 = *s2 > 1;
            if b1 || b2 {
                ((b1 as u8) + (b2 as u8) + ((*s3 > 1) as u8) + ((*s4 > 1) as u8) + ((*s5 > 1) as u8) + ((*s6 > 1) as u8) + ((*s7 > 1) as u8) + ((*s8 > 1) as u8) + ((*s9 > 0) as u8)) > 4
            } else {
                false
            }
        } else {
            false
        }
    }

    fn add_score_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let score_9 = self.score_9.signal(),
            let score_s1 = self.score_s1.signal(),
            let score_s2 = self.score_s2.signal() =>
            Self::cal_suicide(score_9, score_s1, score_s2)
        }
    }

    fn cal_suicide(score_9: &Option<u8>, score_s1: &Option<u8>, score_s2: &Option<u8>) -> bool {
        if let (Some(s9), Some(x1), Some(x2)) = (score_9, score_s1, score_s2) {
            (s9 + x1 + x2) > 0
        } else {
            false
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
            .future(modal.diagnosis_signal().dedupe().for_each(clone!(modal => move |is_diagnosis| {
                modal.is_diagnosis.set_neq(is_diagnosis);
                async {}
            })))
            .future(modal.add_score_signal().dedupe().for_each(clone!(modal => move |is_suicide| {
                modal.is_suicide.set_neq(is_suicide);
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินภาวะซึมเศร้าในวัยรุ่น อายุ 11-20 ปี")}),
                                    html!("div", {.text("Thai version of The Patient Health Questionnaire for Adolescents : PHQ-A")}),
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
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ลำดับที่")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("ในช่วง 2 สัปดาห์ คุณมีอาการต่อไปนี้บ่อยแค่ไหน")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ไม่มีเลย")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("มีบางวัน")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("มีมากกว่า")
                                                    .child(html!("br"))
                                                    .text("7 วัน")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("มีแทบ")
                                                    .child(html!("br"))
                                                    .text("ทุกวัน")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_4(modal.score_1.clone(), 1, "รู้สึกซึม เศร้า หงุดหงิด หรือสิ้นหวัง"),
                                            table_row_4(modal.score_2.clone(), 2, "เบื่อ ไม่ค่อยสนใจ หรือไม่เพลิดเพลินเวลาทำสิ่งต่างๆ"),
                                            table_row_4(modal.score_3.clone(), 3, "นอนหลับยาก รู้สึกง่วงทั้งวัน หรือนอนมากเกินไป"),
                                            table_row_4(modal.score_4.clone(), 4, "ไม่อยากอาหาร น้ำหนักลด หรือกินมากกว่าปกติ"),
                                            table_row_4(modal.score_5.clone(), 5, "รู้สึกเหนื่อยล้า หรือไม่ค่อยมีพลัง"),
                                            table_row_4(modal.score_6.clone(), 6, "รู้สึกแย่กับตัวเอง หรือรู้สึกว่าตัวเองล้มเหลว หรือทำให้ตัวเองหรือครอบครัวผิดหวัง"),
                                            table_row_4(modal.score_7.clone(), 7, "จดจ่อกังสิ่งต่างๆ ได้ยาก เช่น ทำการบ้าน อ่านหนังสือ หรือดูโทรทัศน์"),
                                            table_row_4(modal.score_8.clone(), 8, "พูดหรือทำอะไรช้าลงมากจนคนอื่นสังเกตได้ หรือในทางตรงกันข้าม คือ กระสับกระส่ายหรือกระวนกระวาย จนต้องเคลื่อนไหวไปมามากกว่าปกติ"),
                                            table_row_4(modal.score_9.clone(), 9, "คิดว่าถ้าตายไปเสียจะดีกว่า หรือคิดจะทำร้ายตัวเองด้วยวิธีใดวิธีหนึ่ง"),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "4")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                    .text_signal(modal.is_diagnosis.signal().map(|can_dx| {
                                                        if can_dx {
                                                            " (อาจจะเป็นโรคซึมเศร้า)"
                                                        } else {
                                                            " (ไม่น่าจะเป็นโรคซึมเศร้า)"
                                                        }
                                                    }))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ลำดับที่")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("คำถามเพิ่ม")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ใช่")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ไม่ใช่")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_2(modal.score_s1.clone(), 1, "ใน 1 เดือนที่ผ่านมา มีช่วงไหนที่คุณมีความคิดอยากตาย หรือไม่อยากมีชีวิตอยู่อย่างจริงจังหรือไม่"),
                                            table_row_2(modal.score_s2.clone(), 2, "ตลอดชีวิตที่ผ่านมา คุณเคยพยายามที่จะทำให้ตัวเองตาย หรือลงมือฆ่าตัวตายหรือไม่"),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "2")
                                                    .text_signal(modal.is_suicide.signal().map(|has_risk| {
                                                        if has_risk {
                                                            "มีความเสี่ยงฆ่าตัวตาย"
                                                        } else {
                                                            "ไม่มีความเสี่ยงฆ่าตัวตาย"
                                                        }
                                                    }))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(DEPRESS_PHQ_A_INTRERPRET)
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
                                        if modal.is_diagnosis.get() {String::from("1")} else {String::from("0")},
                                        if modal.is_suicide.get() {String::from("1")} else {String::from("0")},
                                        modal.score_1.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_2.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_3.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_4.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_5.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_6.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_7.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_8.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_9.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_s1.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_s2.get().map(|u| u.to_string()).unwrap_or_default(),
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

fn table_row_4(mutable: Mutable<Option<u8>>, i: usize, title: &str) -> Dom {
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

fn table_row_2(mutable: Mutable<Option<u8>>, i: usize, title: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            html!("td", {.text(title)}),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 1),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 0),
        ])
    })
}
