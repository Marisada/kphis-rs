use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const OAS_INTRERPRET: &str = r#"การแปลผล
                การตัดสินระดับความรุนแรง จะพิจารณาใช้คะแนนสูงสุดตามที่ประเมินได้ตามลักษณะพฤติกรรม เพียงข้อเดียวเท่านั้น และคะแนนระดับความรุนแรงที่ประเมินได้มีความหมายดังนี้
                1. กึ่งเร่งด่วน (Semi-urgency) OAS = 1 คะแนน หมายถึง ผู้ป่วยมีพฤติกรรมก้าวร้าวรุนแรงที่ยังสามารถรับฟังคำเตือนแล้วสงบลงได้ ซึ่งจะถูกจัดอยู่ในกลุ่มปานกลาง (Moderate) ต้องจัดการภายใน 24 ชั่วโมง
                2. เร่งด่วน (Urgency) OAS = 2 คะแนน หมายถึง ผู้ป่วยมีพฤติกรรมก้าวร้าวรุนแรง ที่เริ่มควบคุมตนเองไม่ได้ มีท่าทีที่อาจเกิดอันตรายต่อตนเอง ผู้อื่น และทรัพย์สิน ซึ่งจะถูกจัดอยู่ในกลุ่มหนัก ต้องจัดการภายใน 2 ชั่วโมง
                3. ฉุกเฉิน (Emergency) OAS = 3 คะแนน หมายถึง ผู้ป่วยมีพฤติกรรมก้าวร้าวรุนแรง ที่ไม่สามารถควบคุมตนเองได้ จนอาจเกิดอันตรายต่อตนเอง หรือผู้อื่น หรือทรัพย์สิน ซึ่งจะถูกจัดอยู่ในกลุ่มหนักมาก ตัองจัดการทันทีทันใด
"#;

pub struct AggressionOAS {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl AggressionOAS {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        let score_1 = split.get(1).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(2).and_then(|s| s.parse::<u8>().ok());
        let score_3 = split.get(3).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2, &score_3);
        Rc::new(Self {
            score_1: Mutable::new(score_1),
            score_2: Mutable::new(score_2),
            score_3: Mutable::new(score_3),
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
            let score_3 = self.score_3.signal() =>
            Self::cal_total(score_1, score_2, score_3)
        }
    }

    fn cal_total(score_1: &Option<u8>, score_2: &Option<u8>, score_3: &Option<u8>) -> Option<u8> {
        [score_1.unwrap_or_default(), score_2.unwrap_or_default(), score_3.unwrap_or_default()].into_iter().max()
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินพฤติกรรมก้าวร้าวรุนแรง")}),
                                    html!("div", {.text("Overt Aggression Scale : OAS")}),
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
                                        .children([
                                            html!("tr", {
                                                .children([
                                                    html!("th", {
                                                        .class("text-center").attr("scope", "col").attr("rowspan","2")
                                                        .style("white-space", "nowrap")
                                                        .style("vertical-align", "middle")
                                                        .text("ลักษณะพฤติกรรม")
                                                        .child(html!("br"))
                                                        .text("ก้าวร้าวรุนแรง")
                                                    }),
                                                    html!("th", {.class("text-center").attr("scope", "col").attr("colspan","4").text("พฤติกรรม / ระดับความก้าวร้าวรุนแรง")}),
                                                ])
                                            }),
                                            html!("tr", {
                                                .children([
                                                    html!("th", {
                                                        .class("text-center").attr("scope", "col")
                                                        .style("white-space", "nowrap")
                                                        .style("vertical-align", "middle")
                                                        .text("3. ฉุกเฉิน (Emergency)")
                                                        .child(html!("br"))
                                                        .text("OAS = 3 คะแนน")
                                                    }),
                                                    html!("th", {
                                                        .class("text-center").attr("scope", "col")
                                                        .style("white-space", "nowrap")
                                                        .style("vertical-align", "middle")
                                                        .text("2. เร่งด่วน (Urgency)")
                                                        .child(html!("br"))
                                                        .text("OAS = 2 คะแนน")
                                                    }),
                                                    html!("th", {
                                                        .class("text-center").attr("scope", "col")
                                                        .style("white-space", "nowrap")
                                                        .text("1. กึ่งเร่งด่วน")
                                                        .child(html!("br"))
                                                        .text("(Semi-ungency)")
                                                        .child(html!("br"))
                                                        .text("OAS = 1 คะแนน")
                                                    }),
                                                    html!("th", {.style("min-width","99px")}),
                                                ])
                                            }),
                                        ])
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_32(
                                                modal.score_1.clone(),
                                                "1. พฤติกรรมก้าวร้าวรุนแรงต่อตนเอง", // Question
                                                "ทำร้ายตนเองรุนแรง เช่น มีรอยช้ำ มีรอยกรีดลึก เลือดออก หรือมีการบาดเจ็บอวัยวะภายใน หรือหมดสติ ฯลฯ", // 3
                                                "ขีดข่วนผิวหนัง ตีตนเอง ดึงผม โขกศีรษะตัวเองเป็นรอยขนาดเล็ก", // 2
                                                "ไม่มี", // 0
                                            ),
                                            table_row(
                                                modal.score_2.clone(),
                                                "2. พฤติกรรมก้าวร้าวรุนแรงต่อผู้อื่น ทั้งทางคำพูด และการแสดงออก", // Question
                                                "พูดข่มขู่จะทำร้ายผู้อื่นชัดเจน เช่น ฉันจะฆ่าแก ฯลฯ ทำร้ายผู้อื่นจนได้รับบาดเจ็บ เช่น ช้ำ เคล็ด บวม เกิดบาดแผล กระดูกหัก หรือเกิดการบาดเจ็บของอวัยวะภายใน หรือหมดสติ ฯลฯ", // 3
                                                "ด่าคำหยาบคาย ใช้คำสกปรกรุนแรง แสดงท่าทางคุกคาม เช่น ถลกเสื้อผ้า ทำท่าต่อยลม หรือกระชากคอเสื้อผู้อื่น พุ่งชน เตะ ผลัก หรือดึงผมผู้อื่น แต่ไม่ได้รับบาดเจ็บ", // 2
                                                "หงุดหงิด ส่งเสียงดัง ตะโกนด้วยความโกรธ หรือตะโกนด่าผู้อื่นด้วยถ้อยคำไม่รุนแรง", // 1
                                                "ไม่มี", // 0
                                            ),
                                            table_row(
                                                modal.score_3.clone(),
                                                "3. พฤติกรรมก้าวร้าวรุนแรงต่อทรัพย์สิน", // Question
                                                "ทำสิ่งของแตกหัก กระจัดกระจาย เช่น ทุบกระจก ขว้างแก้ว จาน มีด หรือสิ่งของที่เป็นอันตราย หรือจุดไฟเผา ฯลฯ", // 3
                                                "ขว้าง เตะ ทุบวัตถุ หรือสิ่งของ", // 2
                                                "ปิดประตูเสียงดัง รื้อข้าวของกระจัดกระจาย", // 1
                                                "ไม่มี", // 0
                                            ),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("คะแนนที่ได้")}),
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
                                .text(OAS_INTRERPRET)
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

fn table_row(mutable: Mutable<Option<u8>>, title: &str, choice_3: &str, choice_2: &str, choice_1: &str, choice_0: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 3, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 1, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_0),
        ])
    })
}

fn table_row_32(mutable: Mutable<Option<u8>>, title: &str, choice_3: &str, choice_2: &str, choice_0: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("fw-bold").text(&title)}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 3, choice_3),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_2),
            html!("td"),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_0),
        ])
    })
}
