use dominator::{Dom, html, link};

use kphis_model::route::Route;
use kphis_ui_core::class;

#[derive(Clone)]
pub struct NotFoundPage {
    pub path: String,
}

impl NotFoundPage {
    pub fn render(&self) -> Dom {
        html!("section", {
            .class(class::FLEX_COL_C)
            .children([
                html!("h1", {
                    .class(class::BOLD_D3_C)
                    .text("ค้นหาไม่พบ")
                }),
                html!("br"),
                html!("p", {
                    .class("fs-3")
                    .text(&["ไม่มีข้อมูล ", &self.path," ในระบบ"].concat())
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
