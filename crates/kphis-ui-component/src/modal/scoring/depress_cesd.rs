use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const DEPRESS_CES_D_INTRERPRET: &str = r#"        การแปลผล
                        คะแนนรวมสูงกว่า 22 ถือว่าอยู่ในข่ายภาวะซึมเศร้า สมควรได้รับการตรวจวินิจฉัย เพื่อช่วยเหลือต่อไป
"#;

pub struct DepressCesD {
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
    score_11: Mutable<Option<u8>>,
    score_12: Mutable<Option<u8>>,
    score_13: Mutable<Option<u8>>,
    score_14: Mutable<Option<u8>>,
    score_15: Mutable<Option<u8>>,
    score_16: Mutable<Option<u8>>,
    score_17: Mutable<Option<u8>>,
    score_18: Mutable<Option<u8>>,
    score_19: Mutable<Option<u8>>,
    score_20: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl DepressCesD {
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
        let score_11 = split.get(11).and_then(|s| s.parse::<u8>().ok());
        let score_12 = split.get(12).and_then(|s| s.parse::<u8>().ok());
        let score_13 = split.get(13).and_then(|s| s.parse::<u8>().ok());
        let score_14 = split.get(14).and_then(|s| s.parse::<u8>().ok());
        let score_15 = split.get(15).and_then(|s| s.parse::<u8>().ok());
        let score_16 = split.get(16).and_then(|s| s.parse::<u8>().ok());
        let score_17 = split.get(17).and_then(|s| s.parse::<u8>().ok());
        let score_18 = split.get(18).and_then(|s| s.parse::<u8>().ok());
        let score_19 = split.get(19).and_then(|s| s.parse::<u8>().ok());
        let score_20 = split.get(20).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(
            &score_1, &score_2, &score_3, &score_4, &score_5, &score_6, &score_7, &score_8, &score_9, &score_10, &score_11, &score_12, &score_13, &score_14, &score_15, &score_16, &score_17,
            &score_18, &score_19, &score_20,
        );
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
            score_11: Mutable::new(score_11),
            score_12: Mutable::new(score_12),
            score_13: Mutable::new(score_13),
            score_14: Mutable::new(score_14),
            score_15: Mutable::new(score_15),
            score_16: Mutable::new(score_16),
            score_17: Mutable::new(score_17),
            score_18: Mutable::new(score_18),
            score_19: Mutable::new(score_19),
            score_20: Mutable::new(score_20),
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
            let score_10 = self.score_10.signal(),
            let score_11 = self.score_11.signal(),
            let score_12 = self.score_12.signal(),
            let score_13 = self.score_13.signal(),
            let score_14 = self.score_14.signal(),
            let score_15 = self.score_15.signal(),
            let score_16 = self.score_16.signal(),
            let score_17 = self.score_17.signal(),
            let score_18 = self.score_18.signal(),
            let score_19 = self.score_19.signal(),
            let score_20 = self.score_20.signal() =>
            Self::cal_total(score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9, score_10, score_11, score_12, score_13, score_14, score_15, score_16, score_17, score_18, score_19, score_20)
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
        score_11: &Option<u8>,
        score_12: &Option<u8>,
        score_13: &Option<u8>,
        score_14: &Option<u8>,
        score_15: &Option<u8>,
        score_16: &Option<u8>,
        score_17: &Option<u8>,
        score_18: &Option<u8>,
        score_19: &Option<u8>,
        score_20: &Option<u8>,
    ) -> Option<u8> {
        if let (
            Some(s1),
            Some(s2),
            Some(s3),
            Some(s4),
            Some(s5),
            Some(s6),
            Some(s7),
            Some(s8),
            Some(s9),
            Some(s10),
            Some(s11),
            Some(s12),
            Some(s13),
            Some(s14),
            Some(s15),
            Some(s16),
            Some(s17),
            Some(s18),
            Some(s19),
            Some(s20),
        ) = (
            score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9, score_10, score_11, score_12, score_13, score_14, score_15, score_16, score_17, score_18, score_19,
            score_20,
        ) {
            Some(s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9 + s10 + s11 + s12 + s13 + s14 + s15 + s16 + s17 + s18 + s19 + s20)
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินภาวะซึมเศร้าในวัยรุ่น อายุ 15-18 ปี ฉบับภาษาไทย")}),
                                    html!("div", {.text("Center for Epidemiologic Studies-Depression Scale : CES-D")}),
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
                                                    .style("vertical-align", "middle")
                                                    .text("ลำดับที่")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("ในระยะ 1 สัปดาห์ที่ผ่านมา ท่านมีความรู้สึกดังต่อไปนี้ บ่อยเพียงใด")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("ไม่เลย")
                                                    .child(html!("br"))
                                                    .text("(<1 วัน)")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("นานๆ ครั้ง")
                                                    .child(html!("br"))
                                                    .text("(1-2 วัน)")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("บ่อยๆ")
                                                    .child(html!("br"))
                                                    .text("(3-4 วัน)")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .text("ตลอดเวลา")
                                                    .child(html!("br"))
                                                    .text("(5-7 วัน)")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_gr_1(modal.score_1.clone(), 1, "ฉันรู้สึกหงุดหงิดง่าย"),
                                            table_row_gr_1(modal.score_2.clone(), 2, "ฉันรู้สึกเบื่ออาหาร"),
                                            table_row_gr_1(modal.score_3.clone(), 3, "ฉันไม่สามารถขจัดความเศร้าออกจากใจได้ แม้จะมีคนคอยช่วยเหลือก็ตาม"),
                                            table_row_gr_2(modal.score_4.clone(), 4, "ฉันรู้สึกว่าตนเองดีพอๆ กับคนอื่น"),
                                            table_row_gr_1(modal.score_5.clone(), 5, "ฉันไม่มีสมาธิ"),
                                            table_row_gr_1(modal.score_6.clone(), 6, "ฉันรู้สึกหดหู่"),
                                            table_row_gr_1(modal.score_7.clone(), 7, "ทุกๆ สิ่งที่ฉันกระทำจะต้องฝืนใจ"),
                                            table_row_gr_2(modal.score_8.clone(), 8, "ฉันมีความหวังเกี่ยวกับอนาคต"),
                                            table_row_gr_1(modal.score_9.clone(), 9, "ฉันรู้สึกว่าชีวิตมีแต่สิ่งล้มเหลว"),
                                            table_row_gr_1(modal.score_10.clone(), 10, "ฉันรู้สึกหวาดกลัว"),
                                            table_row_gr_1(modal.score_11.clone(), 11, "ฉันนอนไม่เคยหลับ"),
                                            table_row_gr_2(modal.score_12.clone(), 12, "ฉันมีความสุข"),
                                            table_row_gr_1(modal.score_13.clone(), 13, "ฉันไม่ค่อยอยากคุยกับใคร"),
                                            table_row_gr_1(modal.score_14.clone(), 14, "ฉันรู้สึกเหงา"),
                                            table_row_gr_1(modal.score_15.clone(), 15, "ผู้คนทั่วไปไม่ค่อยเป็นมิตรกับฉัน"),
                                            table_row_gr_2(modal.score_16.clone(), 16, "ฉันรู้สึกว่าชีวิตนี้สนุกสนาน"),
                                            table_row_gr_1(modal.score_17.clone(), 17, "ฉ้นร้องไห้"),
                                            table_row_gr_1(modal.score_18.clone(), 18, "ฉันรู้สึกเศร้า"),
                                            table_row_gr_1(modal.score_19.clone(), 19, "ผู้คนรอบข้างไม่ชอบฉัน"),
                                            table_row_gr_1(modal.score_20.clone(), 20, "ฉันรู้สึกท้อถอยในชีวิต"),
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
                                .text(DEPRESS_CES_D_INTRERPRET)
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
                                        modal.score_11.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_12.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_13.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_14.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_15.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_16.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_17.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_18.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_19.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_20.get().map(|u| u.to_string()).unwrap_or_default(),
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

fn table_row_gr_1(mutable: Mutable<Option<u8>>, i: usize, title: &str) -> Dom {
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

fn table_row_gr_2(mutable: Mutable<Option<u8>>, i: usize, title: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            html!("td", {.text(title)}),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 3),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 2),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 1),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 0),
        ])
    })
}
