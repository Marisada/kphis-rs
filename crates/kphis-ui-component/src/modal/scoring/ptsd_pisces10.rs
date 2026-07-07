use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const PTSD_PISCES_INTRO: &str = r#"        เลือกรายการที่ตรงกับความเป็นจริงในปัจจุบันของท่านมากที่สุด โดยพิจารณาตามเกณฑ์ต่อไปนี้
                        0  =  ไม่มีอาการ
                        1  =  มีอาการเล็กน้อย แต่ไม่รบกวนการดำเนินชีวิตตามปกติ
                        2  =  มีอาการมาก จนรบกวนการดำเนินชีวิตตามปกติ
                        3  =  มีอาการรุนแรง จนไม่สามารถดำเนินชีวิตได้ตามปกติ (หรือเป็นอุปสรรคอย่างมากต่อการดำเนินชีวิต)
        หมายเหตุ : การดำเนินชีวิต ได้แก่ กิจกรรมด้านส่วนตัว ครอบครัว การเรียน การทำงาน หรือสังคม
"#;

const PTSD_PISCES_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน    0 - 8  หมายถึง  ปกติ
                        คะแนน   9 - 13  หมายถึง  ทุกข์ทรมานทางจิตใจเล็กน้อย
                        คะแนน  14 - 18  หมายถึง  ทุกข์ทรมานทางจิตใจมาก
                        คะแนน  19 - 30  หมายถึง  ทุกข์ทรมานทางจิตใจรุนแรง
        ผู้ที่มีความทุกข์ทรมานทางจิตใจในระดับ มากและรุนแรง ควรได้รับการช่วยเหลือทางจิตใจและติดตามเฝ้าระวังต่อไป
        หมายเหตุ : 
                        ด้านอารมณ์/ความรู้สึก  ได้แก่  ข้อที่ 1-3
                        ด้านความคิด  ได้แก่  ข้อที่ 4-6
                        ด้านพฤติกรรม  ได้แก่  ข้อที่ 7-8
                        ด้านอาการทางกาย  ได้แก่  ข้อที่ 9-10
"#;

pub struct PtsdPisces10 {
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

impl PtsdPisces10 {
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินผลกระทบทางจิตใจหลังเกิดเหตุการณ์สะเทือนขวัญ")}),
                                    html!("div", {.text("The Psychological Impact Scale for Crisis Events - 10 : PISCES-10")}),
                                ])
                            }),
                            html!("br"),
                            html!("h5", {.class("modal-title").text("")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        .style("width", "100%")
                        .children([
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(PTSD_PISCES_INTRO)
                            }),
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
                                                    .text("ไม่มีอาการ")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("อาการเล็กน้อย")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("อาการมาก")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("อาการรุนแรง")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row(modal.score_1.clone(), 1, "ตึงเครียด"),
                                            table_row(modal.score_2.clone(), 2, "ไม่มีความสุข/ไม่ร่าเริง"),
                                            table_row(modal.score_3.clone(), 3, "กังวล/หวาดเสียว/เกรงว่าเหตุการณ์รุนแรงจะเกิดขึ้น"),
                                            table_row(modal.score_4.clone(), 4, "คิดถึงเหตุการณ์หรือภาพเหตุการณ์ผุดขึ้นมาซ้ำๆ ทั้งตื่น ทั้งหลับ (ฝันถึง)"),
                                            table_row(modal.score_5.clone(), 5, "วิตกกังวล คิดวนเวียนซ้ำๆ เรื่องเดิม"),
                                            table_row(modal.score_6.clone(), 6, "ขาดความมั่นใจ/ความเชื่อมั่นในตนเองลดลง"),
                                            table_row(modal.score_7.clone(), 7, "ระแวดระวัง ไม่ไว้ใจสิ่งแวดล้อม"),
                                            table_row(modal.score_8.clone(), 8, "รู้สึกตนองหรือสิ่งแวดล้อมเปลี่ยนไป"),
                                            table_row(modal.score_9.clone(), 9, "เมื่อยล้า ปวดเมื่อยตามส่วนต่างๆ ของร่างกาย"),
                                            table_row(modal.score_10.clone(), 10, "อ่อนเพลีย ไม่มีเรี่ยวแรง"),
                                        ])
                                    }),
                                    html!("tfoot", {
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
                                .text(PTSD_PISCES_INTRERPRET)
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
