use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const Q8_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน        0  ไม่มีแนวโน้มฆ่าตัวตาย
                        คะแนน    1 - 8  มีแนวโน้มฆ่าตัวตายเล็กน้อย
                        คะแนน   9 - 16  มีแนวโน้มฆ่าตัวตายปานกลาง
                        คะแนน  17 - 52  มีแนวโน้มฆ่าตัวตายรุนแรง
        หมายเหตุ: มีคะแนน ตั้งแต่ 1 ขึ้นไป ให้รายงานแพทย์และปฏิบัติตามแนวทางการดูแลผู้ป่วยเสี่ยงต่อการฆ่าตัวตาย
"#;

pub struct Suicide8Q {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3a: Mutable<Option<u8>>,
    score_3b: Mutable<Option<u8>>,
    score_4: Mutable<Option<u8>>,
    score_5: Mutable<Option<u8>>,
    score_6: Mutable<Option<u8>>,
    score_7: Mutable<Option<u8>>,
    score_8: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl Suicide8Q {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let concat = parent_result.get_cloned();
        let split = concat.split(',').collect::<Vec<&str>>();
        let score_1 = split.get(1).and_then(|s| s.parse::<u8>().ok());
        let score_2 = split.get(2).and_then(|s| s.parse::<u8>().ok());
        let score_3a = split.get(3).and_then(|s| s.parse::<u8>().ok());
        let score_3b = split.get(4).and_then(|s| s.parse::<u8>().ok());
        let score_4 = split.get(5).and_then(|s| s.parse::<u8>().ok());
        let score_5 = split.get(6).and_then(|s| s.parse::<u8>().ok());
        let score_6 = split.get(7).and_then(|s| s.parse::<u8>().ok());
        let score_7 = split.get(8).and_then(|s| s.parse::<u8>().ok());
        let score_8 = split.get(9).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(&score_1, &score_2, &score_3a, &score_3b, &score_4, &score_5, &score_6, &score_7, &score_8);
        Rc::new(Self {
            score_1: Mutable::new(score_1),
            score_2: Mutable::new(score_2),
            score_3a: Mutable::new(score_3a),
            score_3b: Mutable::new(score_3b),
            score_4: Mutable::new(score_4),
            score_5: Mutable::new(score_5),
            score_6: Mutable::new(score_6),
            score_7: Mutable::new(score_7),
            score_8: Mutable::new(score_8),
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
            let score_3a = self.score_3a.signal(),
            let score_3b = self.score_3b.signal(),
            let score_4 = self.score_4.signal(),
            let score_5 = self.score_5.signal(),
            let score_6 = self.score_6.signal(),
            let score_7 = self.score_7.signal(),
            let score_8 = self.score_8.signal() =>
            Self::cal_total(score_1, score_2, score_3a, score_3b, score_4, score_5, score_6, score_7, score_8)
        }
    }

    fn cal_total(
        score_1: &Option<u8>,
        score_2: &Option<u8>,
        score_3a: &Option<u8>,
        score_3b: &Option<u8>,
        score_4: &Option<u8>,
        score_5: &Option<u8>,
        score_6: &Option<u8>,
        score_7: &Option<u8>,
        score_8: &Option<u8>,
    ) -> Option<u8> {
        if let (Some(s1), Some(s2), Some(s3a), Some(s3b), Some(s4), Some(s5), Some(s6), Some(s7), Some(s8)) = (score_1, score_2, score_3a, score_3b, score_4, score_5, score_6, score_7, score_8) {
            Some(s1 + s2 + s3a + s3b + s4 + s5 + s6 + s7 + s8)
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
                            html!("h5", {.class("modal-title").text("แบบประเมินการฆ่าตัวตาย 8 คำถาม (8Q)")}),
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
                                                html!("th", {.attr("scope", "col").text("คำถามข้อ 1-7: ในช่วง 1 เดือนที่ผ่านมารวมทั้งวันนี้ ท่าน...")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ไม่ใช่")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ใช่")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row(modal.score_1.clone(), 1, "คิดอยากตาย หรือ คิดว่าตายไปจะดีกว่า", 1),
                                            table_row(modal.score_2.clone(), 2, "อยากทำร้ายตนเอง หรือ ทำให้ตัวเองบาดเจ็บ", 2),
                                            table_row(modal.score_3a.clone(), 3, "คิดเกี่ยวกับการฆ่าตัวตาย", 6),
                                        ])
                                        .future(modal.score_3a.signal().for_each(clone!(modal => move |s3a| {
                                            if s3a == Some(0) {
                                                modal.score_3b.set(Some(0));
                                            }
                                            async {}
                                        })))
                                        .child_signal(modal.score_3a.signal().map(clone!(modal => move |s| (s == Some(6)).then(|| {
                                            html!("tr", {
                                                .children([
                                                    html!("td"),
                                                    html!("td", {.text("ท่านสามารถควบคุมความอยากฆ่าตัวตายที่ท่านคิดอยู่นั้นได้หรือไม่ หรือ บอกไหมว่าคงจะไม่ทำตามความคิดนั้นในขณะนี้")}),
                                                    doms::td_text_value_u8_opt_match(modal.score_3b.clone(), "1", true, 0, "ได้"),
                                                    doms::td_text_value_u8_opt_match(modal.score_3b.clone(), "1", true, 8, "ไม่ได้"),
                                                ])
                                            })
                                        }))))
                                        .children([
                                            table_row(modal.score_4.clone(), 4, "มีแผนการที่จะฆ่าตัวตาย", 8),
                                            table_row(modal.score_5.clone(), 5, "ได้เตรียมการที่จะทำร้ายตนเอง หรือ เตรียมการจะฆ่าตัวตาย โดยตั้งใจว่าจะให้ตายจริงๆ", 9),
                                            table_row(modal.score_6.clone(), 6, "ได้ทำให้ตนเองบาดเจ็บ แต่ไม่ตั้งใจที่จะทำให้เสียชีวิต", 4),
                                            table_row(modal.score_7.clone(), 7, "ได้พยายามฆ่าตัวตายโดยคาดหวัง/ตั้งใจที่จะให้ตาย", 10),
                                            table_row(modal.score_8.clone(), 8, "ตลอดชีวิตที่ผ่านมา ท่านเคยพยายามฆ่าตัวตาย", 4),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "2")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(Q8_INTRERPRET)
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
                                        modal.score_3a.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_3b.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_4.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_5.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_6.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_7.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_8.get().map(|u| u.to_string()).unwrap_or_default(),
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

fn table_row(mutable: Mutable<Option<u8>>, i: usize, title: &str, value_yes: u8) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            html!("td", {.text(&title)}),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 0),
            doms::td_icon_value_u8_opt_match(mutable.clone(), value_yes),
        ])
    })
}
