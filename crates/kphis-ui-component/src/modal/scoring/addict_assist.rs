use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const ASSIST_INTRERPRET: &str = r#"        การแปลผล
                        คะแนน    0 - 3  ผลกระทบต่ำ อนุมานว่าเป็นผู้ใช้
                        คะแนน   4 - 26  ผลกระทบปานกลาง อนุมานว่าเป็นผู้เสพ
                        คะแนน  27 - 39  ผลกระทบสูง อนุมานว่าเป็นผู้ติด
        คัดกรองโรคร่วมหรือโรคอื่นๆ ที่สำคัญ
            - การคัดกรองโรคที่ต้องรับยาต่อเนื่อง เช่น ลมชัก เบาหวาน หัวใจ ความดัน
            - การคัดกรองโรคติดต่อในระยะติดต่อ เช่น วัณโรค สุกใส งูสวัด
            - การคัดกรองการเจ็บป่วยทางจิตใจ เช่น โรคซึมเศร้า (2Q, 9Q), แนวโน้มการฆ่าตัวตาย (8Q), โรคจิต (แบบคัดกรองโรคจิต)
            - การคัดกรองความเสี่ยงการเกิดภาวะถอนพิษยารุนแรง เช่น ใช้ยาเสพติดประเภทเฮโรอีน/ดื่มแอลกอฮอล์เป็นประจำ/ใช้ยานอนหลับเป็นประจำ ในช่วง 3 เดือนที่ผ่านมา
"#;

pub struct AddictAssistV2 {
    score_1: Mutable<Option<u8>>,
    score_2: Mutable<Option<u8>>,
    score_3: Mutable<Option<u8>>,
    score_4: Mutable<Option<u8>>,
    score_5: Mutable<Option<u8>>,
    score_6: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_agent: Mutable<String>,
    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl AddictAssistV2 {
    pub fn new(parent_agent: Mutable<String>, parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
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
            parent_agent,
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
                            html!("h5", {.class("modal-title").text("แบบประเมินผลกระทบจากการใช้สารเสพติด V2")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        .style("width", "100%")
                        .children([
                            html!("div", {
                                .text("ยาและสารเสพติดหลักที่ใช้และคัดกรองในครั้งนี้ คือ ")
                                .text_signal(modal.parent_agent.signal_cloned())
                            }),
                            doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ข้อที่")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("ในช่วง 3 เดือนที่ผ่านมา")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ไม่เคย")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .text("เพียง")
                                                    .child(html!("br"))
                                                    .text("1-2 ครั้ง")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .text("เดือนละ")
                                                    .child(html!("br"))
                                                    .text("1-3 ครั้ง")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .text("สัปดาห์ละ")
                                                    .child(html!("br"))
                                                    .text("1-4 ครั้ง")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .text("เกือบทุกวัน")
                                                    .child(html!("br"))
                                                    .text("(สัปดาห์ละ 5-7 วัน)")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_5_items(modal.score_1.clone(), 1, "คุณใช้", modal.parent_agent.clone(), "บ่อยเพียงใด", [2,3,4,6]),
                                            table_row_5_items(modal.score_2.clone(), 2, "คุณมีความต้องการ หรือมีความรู้สึกอยากใช้", modal.parent_agent.clone(), "จนทนไม่ได้บ่อยเพียงใด", [3,4,5,6]),
                                            table_row_5_items(modal.score_3.clone(), 3, "การใช้", modal.parent_agent.clone(), " ทำให้คุณเกิดปัญหาสุขภาพ ครอบครัว สังคม กฎหมาย หรือการเงินบ่อยเพียงใด", [4,5,6,7]),
                                            table_row_5_items(modal.score_4.clone(), 4, "การใช้", modal.parent_agent.clone(), " ทำให้คุณไม่สามารถรับผิดชอบ หรือ ทำกิจกรรมที่คุณเคยทำตามปกติได้บ่อยเพียงใด", [5,6,7,8]),
                                        ])
                                    }),
                                ])
                            })),
                            doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ข้อที่")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("vertical-align", "middle")
                                                    .text("ในช่วงเวลาที่ผ่านมา")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .style("vertical-align", "middle")
                                                    .text("ไม่เคย")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .text("เคยแต่ก่อน 3 เดือน")
                                                    .child(html!("br"))
                                                    .text("ที่ผ่านมา")
                                                }),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col")
                                                    .style("white-space", "nowrap")
                                                    .text("เคยในช่วง 3 เดือน")
                                                    .child(html!("br"))
                                                    .text("ที่ผ่านมา")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_3_items(modal.score_5.clone(), 5, "ญาติ เพื่อน หรือคนที่รู้จัก เคยว่ากล่าวตักเตือน วิพากษ์วิจารณ์ จัดผิด หรือแสดงท่าทีสงสัยว่าคุณเกี่ยวข้องกับการใช้", modal.parent_agent.clone(), "หรือไม่", [3,6]),
                                            table_row_3_items(modal.score_6.clone(), 6, "คุณเคยลด หรือหยุดใช้", modal.parent_agent.clone(), " แต่ไม่ประสบผลสำเร็จหรือไม่", [3,6]),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").attr("colspan", "2").text("คะแนนรวม")}),
                                                html!("th", {
                                                    .class("text-center")
                                                    .attr("scope", "col")
                                                    .attr("colspan", "3")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(ASSIST_INTRERPRET)
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

fn table_row_5_items(mutable: Mutable<Option<u8>>, i: usize, title_left: &str, parent_agent: Mutable<String>, title_right: &str, values: [u8; 4]) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            html!("td", {
                .text(title_left)
                .text_signal(parent_agent.signal_cloned())
                .text(title_right)
            }),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 0),
            doms::td_icon_value_u8_opt_match(mutable.clone(), values[0]),
            doms::td_icon_value_u8_opt_match(mutable.clone(), values[1]),
            doms::td_icon_value_u8_opt_match(mutable.clone(), values[2]),
            doms::td_icon_value_u8_opt_match(mutable.clone(), values[3]),
        ])
    })
}

fn table_row_3_items(mutable: Mutable<Option<u8>>, i: usize, title_left: &str, parent_agent: Mutable<String>, title_right: &str, values: [u8; 2]) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            html!("td", {
                .text(title_left)
                .text_signal(parent_agent.signal_cloned())
                .text(title_right)
            }),
            doms::td_icon_value_u8_opt_match(mutable.clone(), 0),
            doms::td_icon_value_u8_opt_match(mutable.clone(), values[0]),
            doms::td_icon_value_u8_opt_match(mutable.clone(), values[1]),
        ])
    })
}
