use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const BRADEN_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน  19 - 23  ไม่มีความเสี่ยง (No risk)
                        คะแนน  15 - 18  มีความเสี่ยง (At risk)
                        คะแนน  13 - 14  มีความเสี่ยงปานกลาง (Moderate risk)
                        คะแนน  10 - 12  มีความเสี่ยงสูง (High risk)
                        คะแนน    6 - 9  มีความเสี่ยงสูงมาก (Very high risk)
        การประเมินซ้ำ
                1. ผู้ป่วยในหอผู้ป่วยวิกฤต (Intensive Care) ประเมินซ้ำทุก 8 ชั่วโมง/ทุกเวร และเมื่อมีการเปลี่ยนแปลงอาการ
                2. ผู้ป่วยทั่วไป ประเมินซ้ำทุก 24 ชั่วโมง และเมื่อมีการเปลี่ยนแปลงอาการ
"#;

pub struct Braden {
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

impl Braden {
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินความเสี่ยงการเกิดแผลกดทับของบราเดน")}),
                                    html!("div", {.text("The Braden Scale for Predicting Pressure Sore Risk")}),
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
                                                    .text("Risk Factor")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan","4")
                                                    .text("ลักษณะของผู้ป่วย")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_4(
                                                modal.score_1.clone(),
                                                "1. การรับความรู้สึก (Sensory perception)", // Question
                                                "1. Complete limited: ถูกจำกัดโดยสมบูรณ์ ไม่สามารถตอบสนองต่อการกระตุ้นด้วยความเจ็บปวด เนื่องจากระดับความรู้สึกลดลง หรือได้รับยาสลบหรือสูญเสียประสาทรับความรู้สึกเจ็บปวดทั่วร่างกาย (ไม่สามารถร้องครวญครางสะดุ้งหรือกำมือแน่น)",
                                                "2. Very limited : ถูกจำกัดมากหรือจำกัดเป็นส่วนใหญ่ ตอบสนองแต่เพียงความรู้สึกเจ็บปวดเท่านั้น ไม่สามารถบอกความรู้สึกไม่สุขสบายโดยคำพูด ได้แต่ร้องครวญครางหรือกระสับกระส่าย หรือมีความบกพร่องของประสาทรับความรู้สึกในครึ่งล่าง หรือครึ่งซีกของร่างกาย",
                                                "3. Slightly limited : ถูกจำกัดเล็กน้อย สามารถตอบสนองด้วยการสื่อสารเป็นคำพูด แต่ทำไม่ได้ทุกครั้งเมื่อรู้สึกไม่สุขสบายหรือเมื่อต้องการเปลี่ยนท่า หรือมีความบกพร่องของประสาทรับความรู้สึกบริเวณแขนหรือขา 1-2 ข้าง",
                                                "4. No Impairment : ไม่มีข้อจำกัด สามารถตอบสนองบอกสิ่งที่ต้องการได้ ประสาทการรับรู้ปกติ สามารถสื่อสารบอกความเจ็บปวดหรือไม่สุขสบายได้",
                                            ),
                                            table_row_4(
                                                modal.score_2.clone(),
                                                "2. ความชื้นของผิวหนัง (Moisture)", // Question
                                                "1. Constantly Moist : ผิวหนังเปียกชื้นตลอดเวลา จากเหงื่อ ปัสสาวะ ฯลฯ ตรวจพบทุกครั้งที่มีการเปลี่ยนท่าหรือพลิกตะแคงตัวผู้ป่วย",
                                                "2. Very Moist : ผิวหนังค่อนข้างเปียกชื้น บ่อยครั้งที่มีการเปียกชุ่ม แต่ไม่ใช่ตลอดเวลา มีการเปลี่ยนผ้าปูที่นอนอย่างน้อย 1 ครั้งต่อเวร",
                                                "3. Occasionally Moist : ผิวหนังเปียกชื้นเป็นครั้งคราว ได้รับการเปลี่ยนผ้าปูที่นอนอย่างน้อยวันละ 1 ครั้ง",
                                                "4. Rarely Moist : ไม่มีภาวะผิวหนังเปียกชื้น ผิวหนังแห้งปกติ เปลี่ยนผ้าปูที่นอนตามปกติ",
                                            ),
                                            table_row_4(
                                                modal.score_3.clone(),
                                                "3. ความสามารถในการทำกิจกรรม (Activity)", // Question
                                                "1. Bedfast : อยู่บนเตียงตลอดเวลา",
                                                "2. Chairfast : ถูกจำกัดอยู่บนเก้าอี้ ผู้ป่วยมีข้อจำกัดไม่สามารถเดินได้ด้วยตัวเอง ไม่สามารถลงน้ำหนักบนเท้าทั้ง 2 ข้าง ต้องมีคนช่วยพยุงไปนั่งเก้าอี้หรือรถเข็น",
                                                "3. Walk Occasionally : สามารถเดินได้เองเป็นครั้งคราว ซึ่งอาจมีหรือไม่มีผู้ช่วยพยุง ระยะทางที่เดินเป็นระยะสั้นๆ เวลาส่วนใหญ่จะอยู่แต่บนที่นอน",
                                                "4. Walk Frequency : เดินได้เอง ลุกเดินออกนอกห้องพักอย่างน้อย 2 ครั้งต่อวัน และเดินเล่นภายในห้องพักได้อย่างน้อยทุกๆ 2 ชั่วโมง",
                                            ),
                                            table_row_4(
                                                modal.score_4.clone(),
                                                "4. ความสามารถในการเคลื่อนไหวของร่างกาย (Mobility)", // Question
                                                "1. Complete Immobile : เคลื่อนไหวไม่ได้เลย ไม่สามารถเปลี่ยนอิริยาบถของร่างกายโดยปราศจากการช่วยเหลือ",
                                                "2. Very limited : มีข้อจำกัดมาก สามารถเปลี่ยนอิริยาบถของร่างกายหรือเคลื่อนไหวแขน ขาได้เองเป็นบางครั้ง และกระทำได้เพียงเล็กน้อย",
                                                "3. Slightly limited : มีข้อจำกัดเล็กน้อย สามารถเปลี่ยนอิริยาบถของร่างกายได้เล็กน้อยและทำได้บ่อย แต่สามารถขยับแขน ขาได้อย่างอิสระ",
                                                "4. No Limitation : ไม่มีข้อจำกัด สามารถเคลื่อนไหวแขน ขาได้อย่างอิสระโดยไม่ต้องการผู้ช่วยเหลือ",
                                            ),
                                            table_row_4(
                                                modal.score_5.clone(),
                                                "5. ภาวะโภชนาการ (Nutrition)", // Question
                                                "1. Very poor : ทุพโภชนาการ ได้รับอาหารไม่เพียงพอ ไม่เคยรับประทานได้หมด มีน้อยครั้งที่จะรับประทานอาหารได้มากกว่า 1/3 ของอาหารที่จัดให้ ได้รับอาหารประเภทโปรตีน (เนื้อสัตว์, นม) วันละ 2 มื้อหรือน้อยกว่า ดื่มน้ำได้น้อย ไม่ได้รับประทานอาหารเหลวเพื่อทดแทนอย่างเพียงพอ หรือไม่ได้รับประทานอาหารทางปาก และ/หรือได้อาหารเหลวใสหรือได้รับเฉพาะสารน้ำทางหลอดเลือดดำเป็นเวลามากกว่า 5 วัน",
                                                "2. Probably Inadequate : ได้รับอาหารค่อนข้างไม่เพียงพอ รับประทานอาหารไม่ค่อยหมด รับประทานเพียง 1/2 ของอาหารที่จัดมาให้ ได้รับอาหารประเภทโปรตีน (เนื้อสัตว์, นม) วันละ 3 มื้อ รับประทานอาหารเสริมเป็นครั้งคราว หรือได้รับอาหารเหลวหรืออาหารทางสายยางน้อยกว่าปริมาณที่สมควรจะได้รับ",
                                                "3. Adequate : ได้รับอาหารเพียงพอ รับประทานอาหารได้มากกว่า 1/2 ของอาหารที่จัดมาให้ ได้รับอาหารประเภทโปรตีน (เนื้อสัตว์, นม) วันละ 4 มื้อ ไม่ค่อยปฏิเสธการรับประทานอาหารและรับประทานอาหารเสริมเพิ่มเติม หรือได้รับอาหารทางสายยางหรือสารอาหารทางหลอดเลือดดำเพียงพอกับความต้องการของร่างกาย",
                                                "4. Excellent : รับประทานอาหารได้ดีมาก หมดทุกมื้อ ไม่ค่อยปฏิเสธ ได้รับอาหารประเภทโปรตีน (เนื้อสัตว์, นม) วันละ 4 มื้อหรือมากกว่า มีอาหารระหว่างมื้อเป็นบางครั้ง ไม่จำเป็นต้องได้รับอาหารเสริม",
                                            ),
                                            table_row_3(
                                                modal.score_6.clone(),
                                                "6. แรงเสียดสีและแรงเฉือน (Friction and Shear) ประเมินแรงเสียดสีจากการดึงลากและแรงเฉือนจากการลื่นไถล", // Question
                                                "1. Problem : มีปัญหา ต้องใช้ผู้ช่วยจำนวนปานกลางไปจนถึงมากในการเคลื่อนย้าย การเคลื่อนย้ายโดยวิธีการยกเป็นไปได้ยาก มีการไหลเลื่อนลงบนเก้าอี้และบนเตียงค่อนข้างบ่อย เมื่อมีการเปลี่ยนท่าต้องใช้ผู้ช่วยจำนวนมาก มีการหดเกร็งหรือสั่นซึ่งจะทำให้เกิดการเสียดสีเป็นระยะ",
                                                "2. Partial Problem : แนวโน้มน่าจะเป็นปัญหา สามารถเคลื่อนย้ายได้อย่างอิสระหรือใช้ผู้ช่วยน้อยระหว่างการเคลื่อนย้าย ผิวหนังอาจเสียดสีหรือกระทบกับผ้าปูที่นอน เก้าอี้หรืออุปกรณ์ต่างๆ ส่วนใหญ่สามารถทรงตัวได้ดีเมื่ออยู่บนเตียงหรือเก้าอี้ มีการไหลเลื่อนเป็นครั้งคราว",
                                                "3. No Apparent Problem : ไม่มีปัญหาอย่างเด่นชัด สามารถเคลื่อนย้ายบนเตียงและเก้าอี้ได้อย่างอิสระ มีความแข็งแรงของกล้ามเนื้อเพียงพอที่จะยกตัวระหว่างการเคลื่อนย้าย สามารถดำรงอยู่ในตำแหน่งที่เหมาะสมทั้งบนเตียงและเก้าอี้ได้ตลอดเวลา",
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
                                .text(BRADEN_INTRERPRET)
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

fn table_row_4(mutable: Mutable<Option<u8>>, title: &str, choice_1: &str, choice_2: &str, choice_3: &str, choice_4: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 3, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 4, choice_4),
        ])
    })
}

fn table_row_3(mutable: Mutable<Option<u8>>, title: &str, choice_1: &str, choice_2: &str, choice_3: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 3, choice_3),
            html!("td"),
        ])
    })
}
