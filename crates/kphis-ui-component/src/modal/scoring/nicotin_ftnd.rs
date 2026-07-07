use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const NICOTIN_FTND_INTRERPRET: &str = r#"            การแปลผล
                        คะแนน   0 - 2  หมายถึง  ติดนิโคตินระดับต่ำ
                        คะแนน   3 - 4  หมายถึง  ติดนิโคตินระดับปานกลาง
                        คะแนน  5 - 10  หมายถึง  ติดนิโคตินระดับสูง
"#;

pub struct NicotinFtnd {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3: Mutable<Option<u8>>,
    score_4: Mutable<Option<u8>>,
    score_5: Mutable<Option<u8>>,
    score_6: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl NicotinFtnd {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        let score_1 = split.get(1).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(2).and_then(|s| s.parse::<u8>().ok());
        let score_3 = split.get(3).and_then(|s| s.parse::<u8>().ok());
        let score_4 = split.get(4).and_then(|s| s.parse::<u8>().ok());
        let score_5 = split.get(5).and_then(|s| s.parse::<u8>().ok());
        let score_6 = split.get(6).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2, &score_3, &score_4, &score_5, &score_6);
        Rc::new(Self {
            score_1: Mutable::new(score_1),
            score_2: Mutable::new(score_2),
            score_3: Mutable::new(score_3),
            score_4: Mutable::new(score_4),
            score_5: Mutable::new(score_5),
            score_6: Mutable::new(score_6),
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
            let score_6 = self.score_6.signal() =>
            Self::cal_total(score_1, score_2, score_3, score_4, score_5, score_6)
        }
    }

    fn cal_total(score_1: &Option<u8>, score_2: &Option<u8>, score_3: &Option<u8>, score_4: &Option<u8>, score_5: &Option<u8>, score_6: &Option<u8>) -> Option<u8> {
        if let (Some(s1), Some(s2), Some(s3), Some(s4), Some(s5), Some(s6)) = (score_1, score_2, score_3, score_4, score_5, score_6) {
            Some(s1 + s2 + s3 + s4 + s5 + s6)
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินระดับการติดนิโคติน")}),
                                    html!("div", {.text("Fagerstrom Test for Nicotin Dependence : FTND")}),
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
                                                    .text("ข้อคำถาม")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan","8")
                                                    .text("เครื่องมือ FTND")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_4(
                                                modal.score_1.clone(),
                                                "1. โดยปกติคุณสูบบุหรี่วันละกี่มวนต่อวัน", // Question
                                                "10 มวน หรือน้อยกว่า",
                                                "11-12 มวน",
                                                "21-30 มวน",
                                                "31 มวน ขึ้นไป",
                                            ),
                                            table_row_4(
                                                modal.score_2.clone(),
                                                "2. คุณมักสูบบุหรี่มวนแรก หลังตื่นนอนตอนเช้านานแค่ไหน", // Question
                                                "มากกว่า 60 นาที หลังตื่นนอน",
                                                "31-60 นาที หลังตื่นนอน",
                                                "6-30 นาที หลังตื่นนอน",
                                                "ภายใน 5 นาที หลังตื่นนอน",
                                            ),
                                            table_row_2(
                                                modal.score_3.clone(),
                                                "3. คุณสูบบุหรี่จัดในชั่วโมงแรกหลังตื่นนอน มากกว่าในช่วงอื่นของวัน ใช่หรือไม่", // Question
                                                "ไม่ใช่",
                                                "ใช่",
                                            ),
                                            table_row_2(
                                                modal.score_4.clone(),
                                                "4. บุหรี่มวนไหน ที่คุณไม่อยากเลิกมากที่สุด", // Question
                                                "มวนอื่นๆ",
                                                "มวนแรกในตอนเช้า",
                                            ),
                                            table_row_2(
                                                modal.score_5.clone(),
                                                "5. คุณรู้สึกหงุดหงิดหรือยุ่งยากไหม ที่ต้องอยู่ในเขตปลอดบุหรี่ เช่น ในโรงภาพยนตร์ รถเมล์ ร้านอาหาร", // Question
                                                "ไม่รู้สึกยุ่งยาก",
                                                "รู้สึกยุ่งยาก",
                                            ),
                                            table_row_2(
                                                modal.score_6.clone(),
                                                "6. คุณยังต้องสูบบุหรี่ แม้จะเจ็บป่วยจนต้องพักรักษาตัวในโรงพยาบาล ใช่หรือไม่", // Question
                                                "ไม่ใช่",
                                                "ใช่",
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
                                .text(NICOTIN_FTND_INTRERPRET)
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

fn table_row_4(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str, choice_2: &str, choice_3: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 3, choice_3),
        ])
    })
}

fn table_row_2(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 1, choice_1),
        ])
    })
}
