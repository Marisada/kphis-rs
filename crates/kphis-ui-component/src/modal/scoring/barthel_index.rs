use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const BARTHEL_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน    0 - 19  ภาวะพึ่งพาโดยสมบูรณ์ : Totally dependent
                        คะแนน   20 - 39  ภาวะพึ่งพารุนแรง : Very dependent
                        คะแนน   40 - 59  ภาวะพึ่งพาปานกลาง : Partially dependent
                        คะแนน   60 - 79  ภาวะพึ่งพาเล็กน้อย : Minimally dependent
                        คะแนน  80 - 100  ไม่มีภาวะพึ่งพา : Independent
"#;

#[derive(Default)]
pub struct BarthelIndex {
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

impl BarthelIndex {
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินกิจวัตรประจำวัน ดัชนีบาร์เธล")}),
                                    html!("div", {.text("Barthel Index for Activities of Daily Living : ADL")}),
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
                                                    .text("หัวข้อประเมิน")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan","4")
                                                    .text("แบบประเมินกิจวัตรประจำวัน ดัชนีบาร์เธล (Barthel Index for ADL)")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_3(
                                                modal.score_1.clone(),
                                                "1. Feeding : การรับประทานอาหาร เมื่อเตรียมสำรับไว้ต่อหน้า", // Question
                                                "ไม่สามารถตักอาหารเข้าปากได้ ต้องมีคนป้อนให้",
                                                "ตักอาหารเองได้ แต่ต้องมีคนช่วย เช่น ใส่ช้อนตักเตรียมให้/ตัดเป็นชิ้นเล็กๆให้",
                                                "ตักอาหารรับประทานตัวยตนเอง",
                                            ),
                                            table_row_4(
                                                modal.score_2.clone(),
                                                "2. Transfers : การลุกจากที่นอน หรือจากเตียงไปยังเก้าอี้", // Question
                                                "ไม่สามารถนั่งได้ (นั่งแล้วจะล้มเสมอ) หรือต้องใช้คน 2 คนช่วยกันยกขึ้น",
                                                "ต้องใช้คนแข็งแรงหรือมีทักษะ 1 คน หรือคนทั่วไป 2 คน พยุงดันขึ้นมาจึงนั่งอยู่ได้",
                                                "ต้องการความช่วยเหลือบ้าง เช่น ช่วยพยุงเล็กน้อย หรือต้องมีคนดูแลเพื่อความปลอดภัย",
                                                "ทำได้เองทุกขั้นตอน",
                                            ),
                                            table_row_2(
                                                modal.score_3.clone(),
                                                "3. Grooming : การล้างหน้า หวีผม แปรงฟัน โกนหนวด ในช่วง 24-48 ชั่วโมงที่ผ่านมา", // Question
                                                "ต้องการความช่วยเหลือ",
                                                "ทำได้เอง (รวมทั้งทำได้เองถ้าเตรียมอุปกรณ์ไว้ให้)",
                                            ),
                                            table_row_3(
                                                modal.score_4.clone(),
                                                "4. Toilet use : การใช้ห้องน้ำ", // Question
                                                "ช่วยตัวเองไม่ได้",
                                                "ทำได้เองบ้าง (อย่างน้อยทำความสะอาดได้เองหลังเสร็จธุระ)",
                                                "ช่วยเหลือตัวเองได้ดี (ชึ้นลงโถส้วม ทำความสะอาด ถอดเสื้อผ้าก่อน-หลังเสร็จธุระ ได้เอง",
                                            ),
                                            table_row_2(
                                                modal.score_5.clone(),
                                                "5. Bathing : การอาบน้ำ", // Question
                                                "ต้องมีคนช่วยเหลือทำให้",
                                                "อาบน้ำได้เอง ทำได้ทุกขั้นตอน",
                                            ),
                                            table_row_4(
                                                modal.score_6.clone(),
                                                "6. Mobility : การเคลื่อนที่ภายในห้องหรือบ้าน", // Question
                                                "เคลื่อนที่ไปไหนไม่ได้",
                                                "ใช้รถเข็นช่วยให้เคลื่อนที่ได้เอง (ไม่ต้องมีคนเข็นให้) เข้า-ออกมุมห้องหรือประตูได้",
                                                "เดินหรือเคลื่อนที่โดยมีคนช่วย เช่น พยุงหรือบอกให้ทำตาม หรือดูแลเพื่อความปลอดภัย",
                                                "เดินหรือเคลื่อนที่ได้เอง",
                                            ),
                                            table_row_3(
                                                modal.score_7.clone(),
                                                "7. Stairs : การขึ้นลงบันได 1 ขั้น", // Question
                                                "ไม่สามารถทำได้",
                                                "ต้องการคนช่วย",
                                                "ขึ้นลงได้เอง (ถ้าต้องการเครื่องช่วยเดิน เช่น walker จะต้องเอาขึ้นลงไปด้วย)",
                                            ),
                                            table_row_3(
                                                modal.score_8.clone(),
                                                "8. Dressing : การสวมใส่เสื้อผ้า", // Question
                                                "ต้องมีคนสวมใส่ให้",
                                                "ช่วยตัวเองได้ประมาณร้อยละ 50 ที่เหลือต้องมีคนช่วย",
                                                "ช่วยเหลือตนเองได้เอง (รวมทั้งการติดกระดุม รูดซิป หรือใส่เสื้อผ้าที่ดัดแปลงให้เหมาะสมได้)",
                                            ),
                                            table_row_3(
                                                modal.score_9.clone(),
                                                "9. Bowel control : การกลั้นอุจจาระ ใน 1 สัปดาห์ที่ผ่านมา", // Question
                                                "กลั้นไม่ได้ หรือต้องการสวนอุจจาระอยู่เสมอ",
                                                "กลั้นไม่ได้เป็นบางครั้ง (ไม่เกิน 1 ครั้งต่อสัปดาห์)",
                                                "กลั้นได้เป็นปกติ",
                                            ),
                                            table_row_3(
                                                modal.score_10.clone(),
                                                "10. Bladder control : การกลั้นปัสสาวะ ใน 1 สัปดาห์ที่ผ่านมา", // Question
                                                "กลั้นไม่ได้",
                                                "กลั้นไม่ได้เป็นบางครั้ง (ไม่เกินวันละ 1 ครั้ง)",
                                                "กลั้นได้เป็นปกติ",
                                            ),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("คะแนนรวม")}),
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
                                .text(BARTHEL_INTRERPRET)
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

fn table_row_4(mutable: Mutable<Option<u8>>, title: &str, choice_1: &str, choice_2: &str, choice_3: &str, choice_4: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 5, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 10, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 15, choice_4),
        ])
    })
}

fn table_row_3(mutable: Mutable<Option<u8>>, title: &str, choice_1: &str, choice_2: &str, choice_3: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 5, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", false, 10, choice_3),
        ])
    })
}

fn table_row_2(mutable: Mutable<Option<u8>>, title: &str, choice_1: &str, choice_2: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", false, 0, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", false, 5, choice_2),
        ])
    })
}
