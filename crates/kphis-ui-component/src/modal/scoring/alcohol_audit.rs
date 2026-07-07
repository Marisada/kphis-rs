use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const AUDIT_INTRO: &str = r#"        คำชี้แจง : คำถามแต่ละข้อต่อไปนี้จะถามถึุงประสบการณ์การดื่มสุราในรอบ 1 ปีที่ผ่านมา โดยสุรา หมายถึงเครื่องดื่มที่มีแอลกอฮอล์ทุกชนิด ได้แก่ เบียร์ เหล้า สาโท กระแช่ วิสกี้ สปายไวน์ เป็นต้น ขอให้ตอบตามความเป็นจริง
        การเปรียบเทียบปริมาณแอลกอฮอล์ในเครื่องดื่มเป็น 1 ดื่มมาตรฐาน (Standard Drink) ที่เท่ากับแอลกอฮอล์ 10 กรัม ได้แก่
                • เหล้าแดง 35 ดีกรี : 2 ฝาใหญ่ หรือ 30 cc. = 1 ดื่มมาตรฐาน (เช่น 1 แบน หรือ 350 cc. = 12 ดื่มมาตรฐาน)
                • เหล้าขาว หรือเหล้าวิสกี้ 40 ดีกรี : 1 เป๊ก หรือ 50 cc. = 1.5 ดื่มมาตรฐาน
                • เบียร์ 5% เช่น สิงห์ ไฮเนเกน ลีโอ เชียร์ ไทเกอร์ ช้างดราฟ : 1 ขวดใหญ่ 660 cc. = 2.5 ดื่มมาตรฐาน
                • เบียร์ 6.4% เช่น ช้าง : ครึ่งกระป๋อง หรือ ⅓ ขวดใหญ่ = 1 ดื่มมาตรฐาน
                • ไวน์ 12% : 1 แก้ว หรือ 100 cc. = 1 ดื่มมาตรฐาน, ไวน์คูลเลอร์ 1 ขวด = 1 ดื่มมาตรฐาน
                • น้ำขาว อุ กระแช่ 10% : 3 เป๊ก/ตอง/ก๊ง หรือ 150 cc. = 1 ดื่มมาตรฐาน
                • สาโท สุราแช่ สุราพื้นเมือง 6% : 4 เป๊ก/ตอง/ก๊ง หรือ 200 cc. = 1 ดื่มมาตรฐาน
"#;
const AUDIT_INTRERPRET: &str = r#"            การแปลผล
                        คะแนน    0 - 7  ผู้ดื่มแบบเสี่ยงต่ำ (Low risk drinker) : ควรให้ความรู้เกี่ยวกับการดื่มสุรา และอันตรายที่อาจเกิดขึ้นหากดื่มมากกว่านี้ และชื่นชมพฤติกรรมการดื่มที่เสี่ยงต่ำ ใช้เวลาไม่มากกว่าหนึ่งนาที
                        คะแนน   8 - 15  ผู้ดื่มแบบเสี่ยง (Hazardous drinker) : ควรให้คำแนะนำแบบสั้น (Brief Advice or Simple Advice)
                        คะแนน  16 - 19  ผู้ดื่มแบบอันตราย (Harmful use) : ควรให้การบำบัดแบบสั้น (Brief Intervention/Brief Counseling)
                        คะแนน  20 - 40  ผู้ดื่มแบบติด (Alcohol dependence) : ควรได้รับการส่งต่อพบแพทย์ เพื่อการตรวจวินิจฉัยและวางแผนการบำบัดรักษา
"#;

pub struct AlcoholAudit {
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

impl AlcoholAudit {
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินปัญหาการดื่มสุรา")}),
                                    html!("div", {.text("Alcohol Use Disorders Identification Test : AUDIT")}),
                                ])
                            }),
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
                                .text(AUDIT_INTRO)
                            }),
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
                                                    .text("เครื่องมือ AUDIT")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_5(
                                                modal.score_1.clone(),
                                                "1. คุณดื่มสุราบ่อยเพียงไร", // Question
                                                "ไม่เคยเลย",
                                                "เดือนละครั้ง หรือน้อยกว่า",
                                                "2-4 ครั้ง ต่อเดือน",
                                                "2-3 ครั้ง ต่อสัปดาห์",
                                                "4 ครั้งขึ้นไป ต่อสัปดาห์",
                                            ),
                                            table_row_5(
                                                modal.score_2.clone(),
                                                "2.1 เวลาที่คุณดื่มสุรา โดยทั่วไปแล้วคุณดื่มประมาณเท่าใรต่อวัน หรือ", // Question
                                                "1-2 ดื่มมาตรฐาน",
                                                "3-4 ดื่มมาตรฐาน",
                                                "5-6 ดื่มมาตรฐาน",
                                                "7-9 ดื่มมาตรฐาน",
                                                "ตั้งแต่ 10 ดื่มมาตรฐานขึ้นไป",
                                            ),
                                            table_row_5(
                                                modal.score_2.clone(),
                                                "2.2 ถ้าโดยทั่วไปดื่มเบียร์ เช่น สิงห์ ไฮเนเกน ลีโอ เชียร์ ไทเกอร์ ช้าง ดื่มประมาณเท่าไร ต่อวัน หรือ", // Question
                                                "1-1.5 กระป๋อง หรือ ½-¾ ขวด",
                                                "2-3 กระป๋อง หรือ 1-1.5 ขวด",
                                                "3.5-4 กระป๋อง หรือ 2 ขวด",
                                                "4.5-7 กระป๋อง หรือ 3-4 ขวด",
                                                "7 กระป๋อง หรือ 4 ขวดขึ้นไป",
                                            ),
                                            table_row_5(
                                                modal.score_2.clone(),
                                                "2.3 ถ้าโดยทั่วไปดื่มเหล้า เช่น แม่โขง หงส์ทอง หงส์ทิพย์ เหล้าขาว 40 ดีกรี ดื่มประมาณเท่าไร ต่อวัน", // Question
                                                "2-3 ฝา",
                                                "¼ แบน",
                                                "½ แบน",
                                                "¾ แบน",
                                                "1 แบนขึ้นไป",
                                            ),
                                            table_row_5(
                                                modal.score_3.clone(),
                                                "3. บ่อยครั้งเพียงไรที่คุณดื่มตั้งแต่ 6 ดื่มมาตรฐานขึ้นไป หรือเบียร์ 4 กระป๋องหรือ 2 ขวดใหญ่ขึ้นไป หรือเหล้าวิสกี้ 3 เป๊กขึ้นไป", // Question
                                                "ไม่เคยเลย",
                                                "น้อยกว่าเดือนละครั้ง",
                                                "เดือนละครั้ง",
                                                "สัปดาห์ละครั้ง",
                                                "ทุกวัน หรือ เกือบทุกวัน",
                                            ),
                                            table_row_5(
                                                modal.score_4.clone(),
                                                "4. ในช่วงหนึ่งปีที่แล้ว มีบ่อยครั้งเพียงไร ที่คุณพบว่า คุณไม่สามารถหยุดดื่มได้ หากคุณได้เริ่มดื่มไปแล้ว", // Question
                                                "ไม่เคยเลย",
                                                "น้อยกว่าเดือนละครั้ง",
                                                "เดือนละครั้ง",
                                                "สัปดาห์ละครั้ง",
                                                "ทุกวัน หรือ เกือบทุกวัน",
                                            ),
                                            table_row_5(
                                                modal.score_5.clone(),
                                                "5. ในช่วงหนึ่งปีที่แล้ว มีบ่อยครั้งเพียงไร ที่คุณไม่ได้ทำสิ่งที่คุณควรจะทำตามปกติ เพราะคุณมัวแต่ไปดื่มสุราเสีย", // Question
                                                "ไม่เคยเลย",
                                                "น้อยกว่าเดือนละครั้ง",
                                                "เดือนละครั้ง",
                                                "สัปดาห์ละครั้ง",
                                                "ทุกวัน หรือ เกือบทุกวัน",
                                            ),
                                            table_row_5(
                                                modal.score_6.clone(),
                                                "6. ในช่วงหนึ่งปีที่แล้ว มีบ่อยครั้งเพียงไร ที่คุณต้องรีบดื่มสุราทันทีในตอนเช้า เพื่อจะได้ดำเนินชีวิตตามปกติ หรือถอนอาการเมาค้างจากการดื่มหนักในคืนที่ผ่านมา", // Question
                                                "ไม่เคยเลย",
                                                "น้อยกว่าเดือนละครั้ง",
                                                "เดือนละครั้ง",
                                                "สัปดาห์ละครั้ง",
                                                "ทุกวัน หรือ เกือบทุกวัน",
                                            ),
                                            table_row_5(
                                                modal.score_7.clone(),
                                                "7. ในช่วงหนึ่งปีที่แล้ว มีบ่อยครั้งเพียงไร ที่คุณรู้สึกไม่ดี โกรธ หรือเสียใจ เนื่องจากคุณได้ทำบางสิ่งบางอย่างลงไป ขณะที่คุณดื่มสุราเข้าไป", // Question
                                                "ไม่เคยเลย",
                                                "น้อยกว่าเดือนละครั้ง",
                                                "เดือนละครั้ง",
                                                "สัปดาห์ละครั้ง",
                                                "ทุกวัน หรือ เกือบทุกวัน",
                                            ),
                                            table_row_5(
                                                modal.score_8.clone(),
                                                "8. ในช่วงหนึ่งปีที่แล้ว มีบ่อยครั้งเพียงไร ที่คุณไม่สามารถจำได้ว่าเกิดอะไรขึ้นในคืนที่ผ่านมา เพราะว่าคุณได้ดื่มสุราเข้าไป", // Question
                                                "ไม่เคยเลย",
                                                "น้อยกว่าเดือนละครั้ง",
                                                "เดือนละครั้ง",
                                                "สัปดาห์ละครั้ง",
                                                "ทุกวัน หรือ เกือบทุกวัน",
                                            ),
                                            table_row_3(
                                                modal.score_9.clone(),
                                                "9. ตัวคุณเองหรือคนอื่น เคยได้รับบาดเจ็บ ซึ่งเป็นผลจากการดื่มสุราของคุณหรือไม่", // Question
                                                "ไม่เคยเลย",
                                                "เคย แต่ไม่ได้เกิดขึ้นในปีที่แล้ว",
                                                "เคยเกิดขึ้น ในช่วงหนึ่งปีที่แล้ว",
                                            ),
                                            table_row_3(
                                                modal.score_10.clone(),
                                                "10. เคยมีแพทย์ หรือบุคลากรทางการแพทย์ หรือเพื่อนฝูง หรือญาติพี่น้อง แสดงความเป็นห่วงเป็นใยต่อการดื่มสุราของคุณหรือไม่", // Question
                                                "ไม่เคยเลย",
                                                "เคย แต่ไม่ได้เกิดขึ้นในปีที่แล้ว",
                                                "เคยเกิดขึ้น ในช่วงหนึ่งปีที่แล้ว",
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
                                .text(AUDIT_INTRERPRET)
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

fn table_row_5(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_1: &str, choice_2: &str, choice_3: &str, choice_4: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 3, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 4, choice_4),
        ])
    })
}

fn table_row_3(mutable: Mutable<Option<u8>>, title: &str, choice_0: &str, choice_2: &str, choice_4: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", true, 0, choice_0),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "2", true, 4, choice_4),
        ])
    })
}
