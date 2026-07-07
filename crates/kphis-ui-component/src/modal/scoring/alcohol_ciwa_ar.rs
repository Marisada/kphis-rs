use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const CIWA_AR_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน    1 - 7  Mild withdrawal
                        คะแนน   8 - 14  Moderate withdrawal
                        คะแนน  15 - 19  Severe withdrawal
                        คะแนน  20 - 67  Very severe withdrawal
"#;
//         การประเมินซ้ำ
//                 • < 10 คะแนน : รักษาประคับประคอง ประเมินซ้ำทุก 4 ชั่วโมง เป็นเวลา 24 ชั่วโมง
//                 • 10-18 คะแนน : ให้ รับประทาน Diazepam 10 mg (หรือ Lorazepam 2 mg) ประเมินซ้ำทุก 4 ชั่วโมง
//                 • > 19 คะแนน : ให้ Loading ด้วยการรับประทาน Diazepam 20 mg (หรือ Lorazepam 4 mg) หากกินไม่ได้ ให้ Diazepam 10 mg IV ทุก 10-15 นาที และประเมินซ้ำทุก 1 ชั่วโมง จนกว่าคะแนนจะน้อยกว่า 19 คะแนน
// "#;

pub struct AlcoholCiwaAr {
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
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl AlcoholCiwaAr {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        let score_1 = split.get(1).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(2).and_then(|s| s.parse::<u8>().ok());
        let score_3 = split.get(3).and_then(|s| s.parse::<u8>().ok());
        let score_4 = split.get(4).and_then(|s| s.parse::<u8>().ok());
        let score_5 = split.get(5).and_then(|s| s.parse::<u8>().ok());
        let score_6 = split.get(6).and_then(|s| s.parse::<u8>().ok());
        let score_7 = split.get(7).and_then(|s| s.parse::<u8>().ok());
        let score_8 = split.get(8).and_then(|s| s.parse::<u8>().ok());
        let score_9 = split.get(9).and_then(|s| s.parse::<u8>().ok());
        let score_10 = split.get(10).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2, &score_3, &score_4, &score_5, &score_6, &score_7, &score_8, &score_9, &score_10);
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
                            html!("div", {
                                .children([
                                    html!("h5", {.class("modal-title").text("แบบประเมินอาการถอนพิษสุรารายบุคคล")}),
                                    html!("div", {.text("Clinical Institute Withdrawal Assessment of Alcohol Scale, Revised : CIWA-Ar")}),
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
                                                    .text("อาการ")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan","8")
                                                    .text("เครื่องมือ CIWA-Ar")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_4(
                                                modal.score_1.clone(),
                                                r#"1. อาการคลื่นไส้อาเจียน - ถาม "คุณรู้สึกคลื่นไส้ ผะอืดผะอมบ้างไหม?" "อาเจียนไหม?" สังเกต"#, // Question
                                                "ไม่มีคลื่นไส้ ไม่อาเจียน",
                                                "คลื่นไส้เล็กน้อย ไม่อาเจียน",
                                                "คลื่นไส้เป็นพักๆ อาเจียนแต่ไม่มีอะไร",
                                                "คลื่นไส้อยู่เรื่อยๆ อาเจียนบ่อย",
                                            ),
                                            table_row_8(
                                                modal.score_2.clone(),
                                                r#"2. การรับสัมผัสผิดปกติ - ถาม "คุณรู้สึกคันยุบยิบ เหน็บชา ปวดแสบร้อน ปวดแปล๊บๆ เหมือนแมลงไต่หรือไชตามผิวหนังไหม?" สังเกต"#, // Question
                                                "ไม่มี",
                                                "มีน้อยมาก",
                                                "มีน้อย",
                                                "มีปานกลาง",
                                                "มีประสาทหลอนทางสัมผัสค่อนข้างมาก",
                                                "มีประสาทหลอนทางสัมผัสมาก",
                                                "มีประสาทหลอนทางสัมผัสรุนแรงมาก",
                                                "มีประสาทหลอนทางสัมผัสอยู่ตลอด",
                                            ),
                                            table_row_4(
                                                modal.score_3.clone(),
                                                "3. อาการสั่น - ให้เหยียดแขนตรง กางมือออก สังเกต", // Question
                                                "ไม่มีอาการสั่น",
                                                "ไม่เห็นแต่รู้สึกว่าปลายนิ้วแต่ละนิ้วมีอาการสั่น",
                                                "ปานกลาง เห็นสั่นขณะผู้ป่วยหยียดแขนตรง",
                                                "รุนแรง เห็นแม้ขณะไม่เหยียดแขน",
                                            ),
                                            table_row_8(
                                                modal.score_4.clone(),
                                                r#"4. การรับรู้ทางเสียงผิดปกติ - ถาม "รู้สึกพะวงเกี่ยวกับเสียงรอบตัวมากกว่าเดิมไหม? เสียงฟังแล้วระคายหูไหม? เสียงทำให้กลัวไหม? คุณได้ยินเสียงบางอย่างที่รู้สึกว่ารบกวนไหม? คุณได้ยินเสียงที่รู้ว่าไม่มีตัวตนจริงๆไหม?" สังเกต"#, // Question
                                                "ไม่มีเสียง",
                                                "เสียงระคายหูหรือทำให้กลัวน้อยมาก",
                                                "เสียงระคายหูหรือทำให้กลัวน้อย",
                                                "เสียงระคายหูหรือทำให้กลัวปานกลาง",
                                                "มีอาการหูแว่วค่อนข้างรุนแรง",
                                                "มีอาการหูแว่วรุนแรงมาก",
                                                "มีอาการหูแว่วรุนแรงมากอย่างชัดเจน",
                                                "มีอาการหูแว่วอยู่ตลอดเวลา",
                                            ),
                                            table_row_4(
                                                modal.score_5.clone(),
                                                "5. อาการเหงื่อออกเป็นพักๆ - สังเกต", // Question
                                                "ไม่เห็นเหงื่อ",
                                                "ไม่ค่อยเห็นว่าเหงื่อออก, ฝ่ามือชื้น",
                                                "เห็นเหงื่อเป็นเม็ดๆ ชัดบริเวณหน้าผาก",
                                                "เหงื่อแตกทั่วตัว",
                                            ),
                                            table_row_8(
                                                modal.score_6.clone(),
                                                r#"6. การรับรู้ทางตาผิดปกติ - ถาม "รู้สึกว่าแสงไฟที่เห็นสว่างจ้าเกินปกติไหม? สีเปลี่ยนไปไหม? ทำให้รู้สึกแสบเคืองตาไหม? มีเห็นอะไรที่แปลกๆ ไหม? มีเห็นอะไรที่รู้ว่าไม่มีอยู่จริงไหม?" สังเกต"#, // Question
                                                "ไม่มี",
                                                "ไวต่อแสงกว่าปกติน้อยมาก",
                                                "ไวต่อแสงกว่าปกติเล็กน้อย",
                                                "ไวต่อแสงกว่าปกติปานกลาง",
                                                "มีภาพหลอนค่อนข้างรุนแรง",
                                                "มีภาพหลอนรุนแรง",
                                                "มีภาพหลอนรุนแรงมาก",
                                                "มีภาพหลอนอยู่ตลอด",
                                            ),
                                            table_row_4(
                                                modal.score_7.clone(),
                                                r#"7. อาการวิตกกังวล - ถาม "คุณรู้สึกวิตกกังวลไหม?" สังเกต"#, // Question
                                                "ไม่กังวล, ผ่อนคลาย",
                                                "กังวลเล็กน้อย",
                                                "กังวลปานกลาง หรือปิดบังทำให้สงสัยว่าน่าจะมี",
                                                "ตระหนักกลัวรุนแรงมาก",
                                            ),
                                            table_row_8(
                                                modal.score_8.clone(),
                                                r#"8. ปวดหัว มึนตื้อ - ถาม "มีปวดมึนหัวบ้างไหม? รู้สึกเหมือนมีอะไรมารัดรอบหัวไหม?" ไม่รวมอาการวิงเวียน งงๆ ดูตามความรุนแรงของอาการ"#, // Question
                                                "ไม่มี",
                                                "มีน้อยมาก",
                                                "มีน้อย",
                                                "ปานกลาง",
                                                "ค่อนข้างรุนแรง",
                                                "รุนแรง",
                                                "รุนแรงมาก",
                                                "รุนแรงที่สุด",
                                            ),
                                            table_row_4(
                                                modal.score_9.clone(),
                                                "9. อาการกระวนกระวาย - สังเกต", // Question
                                                "พฤติกรรมปกติ",
                                                "กระวนกระวายกว่าปกติเล็กน้อย",
                                                "ดูกระวนกระวาย อยู่ไม่นิ่ง",
                                                "เดินไปมาขณะตอบคำถาม, หรืออยู่กับที่ไม่ได้เลย",
                                            ),
                                            table_row_5(
                                                modal.score_10.clone(),
                                                r#"10. การรับรู้เรื่องเวลาสถานที่ - ถาม "วันนี้วันอะไร? ขณะนี้คุณอยู่ที่ไหน? คิดว่าผู้ตรวจเป็นใคร?""#, // Question
                                                "ตอบได้ตรง",
                                                "ไม่แน่ใจเรื่องวัน",
                                                "ตอบผิดเรื่องวัน แต่ผิดพลาดไม่เกิน 2 วัน",
                                                "ตอบวันผิดมากว่า 2 วัน",
                                                "ตอบผิดด้านสถานที่ และ/หรือบุคคล",
                                            ),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "8")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(CIWA_AR_INTRERPRET)
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

fn table_row_8(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str, choice_2: &str, choice_3: &str, choice_4: &str, choice_5: &str, choice_6: &str, choice_7: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 3, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 4, choice_4),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 5, choice_5),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 6, choice_6),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 7, choice_7),
        ])
    })
}

fn table_row_5(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str, choice_2: &str, choice_3: &str, choice_4: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 3, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 4, choice_4),
        ])
    })
}

fn table_row_4(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str, choice_4: &str, choice_7: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 4, choice_4),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 7, choice_7),
        ])
    })
}
