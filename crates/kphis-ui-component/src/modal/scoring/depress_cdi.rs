use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;

use kphis_ui_core::{class, doms};

const DEPRESS_CDI_INTRO: &str = r#"      แบบคัดกรองภาวะซึมเศร้าในเด็กอายุ 7-17 ปี มีข้อจำกัด ได้แก่
                        1. กลุ่มเป้าหมายต้องอ่านหนังสือออก และเล่าเรื่องราวเกี่ยวกับตนเองได้
                        2. ในกลุ่มเป้าหมายที่เป็นวัยรุ่น อาจมีข้อคำถามบางข้อที่ไม่เหมาะสมกับอายุ ดังนั้น อาจพิจารณาใช้แบบคัดกรอง CES-D แทนได้
"#;
const DEPRESS_CDI_INTRERPRET: &str = r#"        การแปลผล
                        คะแนนรวมที่สูงกว่า 15 ขึ้นไป ถึอว่า มีภาวะซึมเศร้า ที่มีนัยสำคัญทางคลินิก
"#;

pub struct DepressCdi {
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
    score_21: Mutable<Option<u8>>,
    score_22: Mutable<Option<u8>>,
    score_23: Mutable<Option<u8>>,
    score_24: Mutable<Option<u8>>,
    score_25: Mutable<Option<u8>>,
    score_26: Mutable<Option<u8>>,
    score_27: Mutable<Option<u8>>,

    score_total: Mutable<u8>,
    is_complete: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl DepressCdi {
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
        let score_21 = split.get(21).and_then(|s| s.parse::<u8>().ok());
        let score_22 = split.get(22).and_then(|s| s.parse::<u8>().ok());
        let score_23 = split.get(23).and_then(|s| s.parse::<u8>().ok());
        let score_24 = split.get(24).and_then(|s| s.parse::<u8>().ok());
        let score_25 = split.get(25).and_then(|s| s.parse::<u8>().ok());
        let score_26 = split.get(26).and_then(|s| s.parse::<u8>().ok());
        let score_27 = split.get(27).and_then(|s| s.parse::<u8>().ok());
        let score_total = Self::cal_total(
            &score_1, &score_2, &score_3, &score_4, &score_5, &score_6, &score_7, &score_8, &score_9, &score_10, &score_11, &score_12, &score_13, &score_14, &score_15, &score_16, &score_17,
            &score_18, &score_19, &score_20, &score_21, &score_22, &score_23, &score_24, &score_25, &score_26, &score_27,
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
            score_21: Mutable::new(score_21),
            score_22: Mutable::new(score_22),
            score_23: Mutable::new(score_23),
            score_24: Mutable::new(score_24),
            score_25: Mutable::new(score_25),
            score_26: Mutable::new(score_26),
            score_27: Mutable::new(score_27),
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
            let score_20 = self.score_20.signal(),
            let score_21 = self.score_21.signal(),
            let score_22 = self.score_22.signal(),
            let score_23 = self.score_23.signal(),
            let score_24 = self.score_24.signal(),
            let score_25 = self.score_25.signal(),
            let score_26 = self.score_26.signal(),
            let score_27 = self.score_27.signal() =>
            Self::cal_total(score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9, score_10, score_11, score_12, score_13, score_14, score_15, score_16, score_17, score_18, score_19, score_20, score_21, score_22, score_23, score_24, score_25, score_26, score_27)
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
        score_21: &Option<u8>,
        score_22: &Option<u8>,
        score_23: &Option<u8>,
        score_24: &Option<u8>,
        score_25: &Option<u8>,
        score_26: &Option<u8>,
        score_27: &Option<u8>,
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
            Some(s21),
            Some(s22),
            Some(s23),
            Some(s24),
            Some(s25),
            Some(s26),
            Some(s27),
        ) = (
            score_1, score_2, score_3, score_4, score_5, score_6, score_7, score_8, score_9, score_10, score_11, score_12, score_13, score_14, score_15, score_16, score_17, score_18, score_19,
            score_20, score_21, score_22, score_23, score_24, score_25, score_26, score_27,
        ) {
            Some(s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8 + s9 + s10 + s11 + s12 + s13 + s14 + s15 + s16 + s17 + s18 + s19 + s20 + s21 + s22 + s23 + s24 + s25 + s26 + s27)
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
                                    html!("h5", {.class("modal-title").text("แบบประเมินภาวะซึมเศร้าในเด็ก ฉบับภาษาไทย")}),
                                    html!("div", {.text("Children's Depression Inventory : CDI")}),
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
                                .class(&*class::MONO_PRE_WRAP)
                                .text(DEPRESS_CDI_INTRO)
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
                                                    .class("text-center").attr("scope", "col").attr("colspan","3")
                                                    .text("เลือกประโยคที่ตรงกับความรู้สึก หรือความคิดของท่าน ในช่วง 2 สัปดาห์ที่ผ่านมา มากที่สุด")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children([
                                            table_row_gr_1(
                                                modal.score_1.clone(),
                                                1,
                                                "ฉันรู้สึกเศร้านานๆ ครั้ง",
                                                "ฉันรู้สึกเศร้าบ่อยครั้ง",
                                                "ฉันรู้สึกเศร้าตลอดเวลา",
                                            ),
                                            table_row_gr_2(
                                                modal.score_2.clone(),
                                                2,
                                                "อะไรๆ ก็มีอุปสรรคไปเสียหมด",
                                                "ฉันไม่แน่ใจว่าสิ่งต่างๆ จะเป็นไปด้วยดี",
                                                "สิ่งต่างๆ จะเป็นไปด้วยดีสำหรับฉัน",
                                            ),
                                            table_row_gr_1(
                                                modal.score_3.clone(),
                                                3,
                                                "ฉันทำอะไรๆ ได้ค่อนข้างดี",
                                                "ฉันทำผิดพลาดหลายอย่าง",
                                                "ฉันทำอะไรผิดพลาดไปหมด",
                                            ),
                                            table_row_gr_1(
                                                modal.score_4.clone(),
                                                4,
                                                "ฉันรู้สึกสนุกกับหลายสิ่งหลายอย่าง",
                                                "ฉันรู้สึกสนุกกับบางสิ่งบางอย่าง",
                                                "ไม่มีอะไรสนุกสนานเลยสำหรับฉัน",
                                            ),
                                            table_row_gr_2(
                                                modal.score_5.clone(),
                                                5,
                                                "ฉันทำตัวไม่ดีเสมอ",
                                                "ฉันทำตัวไม่ดีบ่อยครั้ง",
                                                "ฉันทำตัวไม่ดีนานๆ ที",
                                            ),
                                            table_row_gr_1(
                                                modal.score_6.clone(),
                                                6,
                                                "นานๆ ครั้ง ฉันจะคิดถึงสิ่งไม่ดีที่อาจเกิดขึ้นกับฉัน",
                                                "ฉันวิตกว่าจะมีสิ่งไม่ดีเกิดขึ้นกับฉัน",
                                                "จะต้องมีสิ่งเลวร้ายเกิดขึ้นกับฉันแน่ๆ",
                                            ),
                                            table_row_gr_2(
                                                modal.score_7.clone(),
                                                7,
                                                "ฉันเกลียดตัวเอง",
                                                "ฉันไม่ชอบตัวเอง",
                                                "ฉันชอบตัวเอง",
                                            ),
                                            table_row_gr_2(
                                                modal.score_8.clone(),
                                                8,
                                                "สิ่งเลวร้ายทั้งหมดที่เกิดขึ้นเป็นความผิดของฉัน",
                                                "สิ่งเลวร้ายหลายสิ่งที่เกิดขึ้นเป็นความผิดของฉัน",
                                                "สิ่งเลวร้ายที่เกิดขึ้นมักไม่ใช่ความผิดของฉัน",
                                            ),
                                            table_row_gr_1(
                                                modal.score_9.clone(),
                                                9,
                                                "ฉันไม่คิดจะฆ่าตัวตาย",
                                                "ฉันคิดถึงการฆ่าตัวตาย",
                                                "ฉันต้องการฆ่าตัวตาย",
                                            ),
                                            table_row_gr_2(
                                                modal.score_10.clone(),
                                                10,
                                                "ฉันรู้สึกอยากร้องไห้ทุกวัน",
                                                "ฉันรู้สึกอยากร้องไห้บ่อยครั้ง",
                                                "ฉันรู้สึกอยากร้องไห้นานๆ ครั้ง",
                                            ),
                                            table_row_gr_2(
                                                modal.score_11.clone(),
                                                11,
                                                "ฉันรู้สึกหงุดหงิดใจตลอดเวลา",
                                                "ฉันรู้สึกหงุดหงิดใจบ่อยครั้ง",
                                                "ฉันรู้สึกหงุดหงิดใจนานๆ ครั้ง",
                                            ),
                                            table_row_gr_1(
                                                modal.score_12.clone(),
                                                12,
                                                "ฉันชอบอยู่กับคนอื่น",
                                                "ฉันไม่ค่อยชอบอยู่กับคนอื่น",
                                                "ฉันไม่ต้องการอยู่กับใครเลย",
                                            ),
                                            table_row_gr_2(
                                                modal.score_13.clone(),
                                                13,
                                                "ฉันไม่สามารถตัดสินใจอะไรต่างๆ ด้วยตนเอง",
                                                "ฉันตัดสินใจเรื่องต่างๆ ได้ลำบาก",
                                                "ฉันตัดสินใจเรื่องต่างๆ ได้ง่าย",
                                            ),
                                            table_row_gr_1(
                                                modal.score_14.clone(),
                                                14,
                                                "ฉันเป็นคนหน้าตาดี",
                                                "ฉันเป็นคนหน้าตาไม่ค่อยดี",
                                                "ฉันเป็นคนหน้าตาหน้าเกลียด",
                                            ),
                                            table_row_gr_2(
                                                modal.score_15.clone(),
                                                15,
                                                "ฉันต้องใช้ความพยายามอย่างหนักทุกครั้งที่ทำการบ้าน",
                                                "ฉันต้องใช้ความพยายามอย่างหนักบ่อยครั้งเวลาทำการบ้าน",
                                                "การทำการบ้านไม่ใช่ปัญหาใหญ่สำหรับฉัน",
                                            ),
                                            table_row_gr_2(
                                                modal.score_16.clone(),
                                                16,
                                                "ฉันนอนไม่หลับทุกคืน",
                                                "ฉันนอนไม่หลับหลายคืน",
                                                "ฉันนอนหลับสบาย",
                                            ),
                                            table_row_gr_1(
                                                modal.score_17.clone(),
                                                17,
                                                "ฉันรู้สึกเหนื่อยนานๆ ครั้ง",
                                                "ฉันรู้สึกเหนื่อยบ่อยครั้ง",
                                                "ฉันรู้สึกเหนื่อยตลอดเวลา",
                                            ),
                                            table_row_gr_2(
                                                modal.score_18.clone(),
                                                18,
                                                "มีหลายวันที่ฉันไม่รู้สึกอยากกินอาหาร",
                                                "มีบางวันที่ฉันไม่รู้สึกอยากกินอาหาร",
                                                "ฉันกินอาหารได้ดี",
                                            ),
                                            table_row_gr_1(
                                                modal.score_19.clone(),
                                                19,
                                                "ฉันไม่กังวลกับการเจ็บป่วย",
                                                "ฉันกังวลกับการเจ็บป่วยบ่อยครั้ง",
                                                "ฉันกังวลกับการเจ็บป่วยตลอดเวลา",
                                            ),
                                            table_row_gr_1(
                                                modal.score_20.clone(),
                                                20,
                                                "ฉันไม่รู้สึกเหงา",
                                                "ฉันรู้สึกเหงาบ่อยๆ",
                                                "ฉันรู้สึกเหงาตลอดเวลา",
                                            ),
                                            table_row_gr_2(
                                                modal.score_21.clone(),
                                                21,
                                                "ฉันไม่รู้สึกสนุกเลย เวลาอยุ่ที่โรงเรียน",
                                                "ฉันรู้สึกสนุกนานๆ ครั้ง เวลาอยู่ที่โรงเรียน",
                                                "ฉันรู้สึกสนุกบ่อยครั้ง เวลาอยุ่ที่โรงเรียน",
                                            ),
                                            table_row_gr_1(
                                                modal.score_22.clone(),
                                                22,
                                                "ฉันมีเพื่อนมาก",
                                                "ฉันมีเพื่อนไม่กี่คน และอยากมีมากกว่านี้",
                                                "ฉันไม่มีเพื่อนเลย",
                                            ),
                                            table_row_gr_1(
                                                modal.score_23.clone(),
                                                23,
                                                "การเรียนของฉันอยุ่ในขั้นใช้ได้ดี",
                                                "การเรียนของฉันไม่ค่อยดีเหมือนเมื่อก่อน",
                                                "การเรียนของฉันแย่ลงมาก",
                                            ),
                                            table_row_gr_2(
                                                modal.score_24.clone(),
                                                24,
                                                "ฉันทำอะไรไม่ได้ดีเท่าคนอื่น",
                                                "ฉันทำอะไร ได้ดีเท่าคนอื่น ถ้าฉันพยายาม",
                                                "ฉันทำได้ดีพอๆ กับคนอื่นอยู่แล้ว ในขณะนี้",
                                            ),
                                            table_row_gr_2(
                                                modal.score_25.clone(),
                                                25,
                                                "ไม่มีใครรักฉันจริง",
                                                "ฉันไม่แน่ใจว่ามีใครรักฉันหรือเปล่า",
                                                "ฉันรู้สึกว่ามีคนรักฉัน",
                                            ),
                                            table_row_gr_1(
                                                modal.score_26.clone(),
                                                26,
                                                "ฉันทำตามคำสั่งที่ได้รับเสมอ",
                                                "ฉันไม่ทำตามคำสั่งบ่อยครั้ง",
                                                "ฉันไม่เคยทำตามคำสั่ง",
                                            ),
                                            table_row_gr_1(
                                                modal.score_27.clone(),
                                                27,
                                                "ฉันเข้ากับคนอื่นได้ดี",
                                                "ฉันทะเลาะกับคนอื่นบ่อยครั้ง",
                                                "ฉันทะเลาะกับคนอื่นตลอดเวลา",
                                            ),
                                        ])
                                    }),
                                    html!("tfoot", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("คะแนน")}),
                                                html!("th", {
                                                    .class("text-center").attr("scope", "col").attr("colspan", "3")
                                                    .text_signal(modal.total_score_signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            })),
                            html!("div", {
                                .class(&*class::MONO_PRE_WRAP)
                                .text(DEPRESS_CDI_INTRERPRET)
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
                                        modal.score_21.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_22.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_23.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_24.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_25.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_26.get().map(|u| u.to_string()).unwrap_or_default(),
                                        modal.score_27.get().map(|u| u.to_string()).unwrap_or_default(),
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

fn table_row_gr_1(mutable: Mutable<Option<u8>>, i: usize, choice_1: &str, choice_2: &str, choice_3: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 1, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_3),
        ])
    })
}

fn table_row_gr_2(mutable: Mutable<Option<u8>>, i: usize, choice_1: &str, choice_2: &str, choice_3: &str) -> Dom {
    html!("tr", {
        .children([
            html!("td", {.class("text-center").text(&i.to_string())}),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 2, choice_1),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 1, choice_2),
            doms::td_text_value_u8_opt_match(mutable.clone(), "1", false, 0, choice_3),
        ])
    })
}
