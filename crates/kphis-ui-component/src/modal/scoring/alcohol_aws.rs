use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const AWS_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน    1 - 4  Mild withdrawal : ไม่จำเป็นต้องใช้ยา
                        คะแนน    5 - 9  Moderate withdrawal : ใช้ยาช่วยลดโอกาสเกิดอาการขาดสุราที่รุนแรง
                        คะแนน  10 - 14  Severe withdrawal : ต้องได้รับการรักษาด้วยยาและติดตามอาการอย่างใกล้ชิด
                        คะแนน  15 - 27  Very severe withdrawal : ต้องได้รับการรักษาด้วยยาขนาดสูง เพื่อทำให้อาการสงบอย่างรวดเร็ว
"#;

pub struct AlcoholAws {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3: Mutable<Option<u8>>,
    score_4: Mutable<Option<u8>>,
    score_5: Mutable<Option<u8>>,
    score_6: Mutable<Option<u8>>,
    score_7: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl AlcoholAws {
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
        let score_total = Self::cal_total(&score_1, &score_2, &score_3, &score_4, &score_5, &score_6, &score_7);
        Rc::new(Self {
            score_1: Mutable::new(score_1),
            score_2: Mutable::new(score_2),
            score_3: Mutable::new(score_3),
            score_4: Mutable::new(score_4),
            score_5: Mutable::new(score_5),
            score_6: Mutable::new(score_6),
            score_7: Mutable::new(score_7),
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
            let score_7 = self.score_7.signal() =>
            Self::cal_total(score_1, score_2, score_3, score_4, score_5, score_6, score_7)
        }
    }

    fn cal_total(score_1: &Option<u8>, score_2: &Option<u8>, score_3: &Option<u8>, score_4: &Option<u8>, score_5: &Option<u8>, score_6: &Option<u8>, score_7: &Option<u8>) -> Option<u8> {
        if let (Some(s1), Some(s2), Some(s3), Some(s4), Some(s5), Some(s6), Some(s7)) = (score_1, score_2, score_3, score_4, score_5, score_6, score_7) {
            Some(s1 + s2 + s3 + s4 + s5 + s6 + s7)
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินอาการขาดสุรา")}),
                                    html!("div", {.text("Alcohol Withdrawal Scale : AWS")}),
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
                                                    .class("text-center").attr("scope", "col").attr("colspan","4")
                                                    .text("เครื่องมือ AWS")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_5(
                                                modal.score_1.clone(),
                                                r#"1. Perspiration (เหงื่อ)"#, // Question
                                                "0. ไม่มีเหงื่อ",
                                                "1. ชื้นเฉพาะที่ฝ่ามือ",
                                                "2. ฝ่ามือชื้น และมีเหงื่อเฉพาะตามใบหน้า ตามตัว",
                                                "3. เหงื่อเปียกชื้นไปทั้งตัว",
                                                "4. เหงื่อออกอย่างมาก จนเสื้อผ้าเปียก",
                                            ),
                                            table_row_4(
                                                modal.score_2.clone(),
                                                r#"2. Tremor (สั่น)"#, // Question
                                                "0. ไม่มีอาการสั่น",
                                                "1. มีอาการสั่น เฉพาะเวลายื่นมือไปจับสิ่งของ หรือถือของ",
                                                "2. มีมือสั่นเล็กน้อยตลอดเวลา",
                                                "3. มีมือสั่นอย่างมากตลอดเวลา",
                                            ),
                                            table_row_5(
                                                modal.score_3.clone(),
                                                r#"3. Anxiety (วิตกกังวล)"#, // Question
                                                "0. สงบ ไม่มีอาการวิตกกังวล",
                                                "1. รู้สึกไม่สบายใจ",
                                                "2. รู้สึกหวาดหวั่น ตกใจง่าย",
                                                "3. วิตกกังวล กลัว สงบได้ยาก",
                                                "4. ไม่สามารถควบคุมอาการวิตกกังวลได้ รวมถึง Panic Attacks",
                                            ),
                                            table_row_5(
                                                modal.score_4.clone(),
                                                r#"4. Agitation (กระสับกระส่าย)"#, // Question
                                                "0. ปกติ ไม่มีอาการกระสับกระส่าย",
                                                "1. งุ่นง่าน อยู่ไม่นิ่ง",
                                                "2. กระวนกระวาย ไม่สามารถนอนพักนิ่งๆ ได้",
                                                "3. กระสับกระส่าย เปลี่ยนท่าบ่อย เดินไปมา สามารถนั่งพัก หรือนอนพักได้ช่วงสั้นๆ",
                                                "4. กระสับกระส่ายอย่างมาก ไม่สามารถอยู่นิ่งได้เลย เดินไปมาตลอดเวลา",
                                            ),
                                            table_row_5(
                                                modal.score_5.clone(),
                                                r#"5. Axilla Temperature (อุณหภูมิร่างกาย)"#, // Question
                                                "0. T < 37.1 °C",
                                                "1. T = 37.1-37.5 °C",
                                                "2. T = 37.6-38 °C",
                                                "3. T = 38.1-38.5 °C",
                                                "4. T > 38.5 °C",
                                            ),
                                            table_row_5(
                                                modal.score_6.clone(),
                                                r#"6. Hallucination (อาการประสาทหลอน)"#, // Question
                                                "0. ไม่มีประสาทหลอนเลย",
                                                "1. มีอาการเห็นสิ่งของรอบข้างบิดเบือนไปเป็นพักๆ ยังรู้ตัวว่าไม่ได้เกิดขึ้นจริง",
                                                "2. มีประสาทหลอนชัดเจน เกิดขึ้นเฉพาะของบางสิ่ง หรือบางเหตุการณ์ และเกิดขึ้นช่วงสั้นๆ ยังคงรับรู้ความเป็นจริง",
                                                "3. ประสาทหลอนชัดเจนเหมือนข้อ 2 เริ่มไม่รับรู้ความจริงมากขึ้น มีความรู้สึกทุกข์ทรมานกับอาการประสาทหลอน แต่ยังรับรู้ความจริงเฉพาะบางเรื่อง",
                                                "4. มีประสาทหลอนชัดเจน ไม่สามารถรับรู้ความเป็นจริงได้",
                                            ),
                                            table_row_5(
                                                modal.score_7.clone(),
                                                r#"7. Orientation (การรับรู้วัน เวลา สถานที่)"#, // Question
                                                "0. รับรู้ บุคคล วัน เวลา สถานที่ได้ดี",
                                                "1. รับรู้ บุคคล และสถานที่ได้ดี แต่มีปัญหาเรื่องวันเวลา",
                                                "2. รับรู้บุคคลได้ดี แต่มีปัญหาการรับรู้สถานที่ และวันเวลาเป็นบางครั้ง",
                                                "3. มีปัญหาการรับรู้บุคคลบางครั้ง แต่การรับรู้สถานที่ และวันเวลาเสียไป",
                                                "4. เสียการรับรู้บุคคล สถานที่ วัน เวลา ไม่ทราบว่าตัวเองอยู่ที่ไหน อยู่กับใคร และไม่รู้วันเวลา",
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
                                .text(AWS_INTRERPRET)
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

fn table_row_5(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str, choice_2: &str, choice_3: &str, choice_4: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 3, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 4, choice_4),
        ])
    })
}

fn table_row_4(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str, choice_2: &str, choice_3: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", false, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 3, choice_3),
        ])
    })
}
