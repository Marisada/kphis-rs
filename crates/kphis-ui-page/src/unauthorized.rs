use dominator::{Dom, html, link};

use kphis_model::route::Route;
use kphis_ui_core::class;

#[derive(Clone)]
pub struct UnAuthorizedPage {
    pub hash: String,
}

impl UnAuthorizedPage {
    pub fn render(&self) -> Dom {
        html!("section", {
            .class(class::FLEX_COL_C)
            .children([
                html!("h1", {
                    .class(class::BOLD_D3_C)
                    .text("ไม่ได้รับอนุญาต")
                }),
                html!("br"),
                html!("p", {
                    .class("fs-3")
                    .text(&["ท่านไม่ได้รับอนุญาตเข้าถึงข้อมูล ", &self.hash].concat())
                }),
                html!("p", {
                    .class("fs-3")
                    .text("หากต้องการความช่วยเหลือหรือรายละเอียดเพิ่มเติม กรุณาติดต่อผู้ดูแลระบบ")
                }),
                html!("br"),
                link!(Route::Info.string(), {
                    .class(class::BTN_BLUE)
                    .text("กลับสู่หน้าหลัก")
                })
            ])
        })
    }
}
