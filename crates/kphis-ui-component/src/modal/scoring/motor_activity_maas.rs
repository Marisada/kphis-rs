use dominator::{Dom, clone, events, html};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const MAAS_INTRO: &str = r#"            สำหรับประเมินความเสี่ยงต่อการเลื่อนหลุดของท่อช่วยหายใจ ด้วยการประเมินระดับความรู้สึกตัวและพฤติกรรมการเคลื่อนไหว เพื่อวางแผนการผูกยึดร่างกายผู้ป่วย"#;
const MAAS_INTRERPRET: &str = r#"            การแปลผล
                        คะแนน     0  No risk :  เฝ้าระวัง
                        คะแนน 1 - 3  Low risk :  แนะนำให้ผูกยึดมือ 2 ข้าง
                        คะแนน     4  Moderate risk :  แนะนำให้ผูกยึดมือ 2 ข้าง ร่วมกับการใส่ปลอกถุงมือ 2 ข้าง
                        คะแนน 5 - 6  High risk :  แนะนำให้ผูกยึดมือ 2 ข้าง ร่วมกับการใส่ปลอกถุงมือ 2 ข้าง และการผูกยึดหน้าอก"#;
#[derive(Default)]
pub struct MotorActivityMaas {
    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl MotorActivityMaas {
    pub fn new(parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        Rc::new(Self { parent_result, parent_changed })
    }

    pub fn render(modal: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::MODAL_DIALOG_XL_FULL)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {.class("modal-title").text("แบบประเมิน Motor Activity Assessment Scale: MAAS)")}),
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
                                .text(MAAS_INTRO)
                            }),
                            doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("คะแนน")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ความหมาย")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.class(class::BOLD_C).text("0")}),
                                                    td_text_value_string_match(
                                                        modal.parent_result.clone(), modal.parent_changed.clone(),
                                                        "8", "ไม่เคลื่อนไหว หรือไม่ตอบสนองต่อสิ่งกระตุ้น",
                                                    ),
                                                ])
                                            }),
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.class(class::BOLD_C).text("1")}),
                                                    td_text_value_string_match(
                                                        modal.parent_result.clone(), modal.parent_changed.clone(),
                                                        "1", "ลืมตา, เลิกคิ้ว, หันศีรษะ หรือขยับแขน เมื่อได้รับสิ่งกระตุ้นที่รุนแรง หรือเมื่อดูดเสมหะ หรือกดหน้าอก",
                                                    ),
                                                ])
                                            }),
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.class(class::BOLD_C).text("2")}),
                                                    td_text_value_string_match(
                                                        modal.parent_result.clone(), modal.parent_changed.clone(),
                                                        "2", "ลืมตา, เลิกคิ้ว, หันศีรษะ หรือขยับแขน เมื่อถูกสัมผัสเบาๆ หรือเรียกชื่อ",
                                                    ),
                                                ])
                                            }),
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.class(class::BOLD_C).text("3")}),
                                                    td_text_value_string_match(
                                                        modal.parent_result.clone(), modal.parent_changed.clone(),
                                                        "3", "รู้สึกตัวดี สงบ และให้ความร่วมมือ",
                                                    ),
                                                ])
                                            }),
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.class(class::BOLD_C).text("4")}),
                                                    td_text_value_string_match(
                                                        modal.parent_result.clone(), modal.parent_changed.clone(),
                                                        "4", "รู้สึกตัว ทำตามสั่งได้ อยู่ไม่นิ่ง เอามือจับท่อช่วยหายใจ ดึงท่อช่วยหายใจ พลาสเตอร์ หรือเชือกผูกท่อช่วยหายใจ",
                                                    ),
                                                ])
                                            }),
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.class(class::BOLD_C).text("5")}),
                                                    td_text_value_string_match(
                                                        modal.parent_result.clone(), modal.parent_changed.clone(),
                                                        "5", "ทำตามสั่งได้ พยายามลุกนั่ง หรือยื่นแขนขาออกนอกเตียง เมื่อร้องขอก็นอนลง แต่ไม่ช้าก็ลุกนั่งและยื่นแขนขาออกนอกเตียงอีก",
                                                    ),
                                                ])
                                            }),
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.class(class::BOLD_C).text("6")}),
                                                    td_text_value_string_match(
                                                        modal.parent_result.clone(), modal.parent_changed.clone(),
                                                        "6", "ไม่ทำตามสั่ง ดิ้นไปมา พยายามลุกนั่ง ปีนลงจากเตียง พยายามดึงท่อช่วยหายใจ สายต่างๆ และอุปกรณ์ต่างๆ หรือทำร้ายเจ้าหน้าที่",
                                                    ),
                                                ])
                                            }),
                                        ])
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(MAAS_INTRERPRET)
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .attr("data-bs-dismiss", "modal")
                            .class(class::BTN_BLUE)
                            .text("ปิด")
                        }))
                    }),
                ])
            }))
        })
    }
}

fn td_text_value_string_match(mutable: Mutable<String>, changed: Mutable<bool>, score: &'static str, title: &str) -> Dom {
    html!("td", {
        .style("cursor", "pointer")
        .style_signal("background-color", mutable.signal_cloned().map(move |s| {
            if s.as_str() == score {
                "gold"
            } else {
                "inherit"
            }
        }))
        .text(title)
        .event(move |_:events::Click| {
            mutable.set_neq(score.to_owned());
            changed.set_neq(true);
        })
    })
}
