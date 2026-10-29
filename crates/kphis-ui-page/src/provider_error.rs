use dominator::{Dom, html, link};

use kphis_model::route::Route;
use kphis_ui_core::class;
use kphis_util::error::ProviderError;

#[derive(Clone)]
pub struct ProviderErrorPage {
    pub code: String,
}

impl ProviderErrorPage {
    pub fn render(&self) -> Dom {
        html!("section", {
            .class(class::FLEX_COL_C)
            .children([
                html!("img", {
                    .attr("src","statics/picture/logo-provider-id.webp")
                }),
                html!("br"),
                html!("p", {
                    .class("fs-3")
                    .children([
                        html!("span", {
                            .class("me-1")
                            .text("เข้าใช้งาน KPHIS ผ่าน Provider ID ไม่สำเร็จ เนื่องจาก")
                        }),
                        html!("span", {
                            .text(ProviderError::new(self.code.as_str()).label())
                        }),
                    ])
                }),
                html!("p", {
                    .class("fs-3")
                    .text("หากต้องการความช่วยเหลือหรือรายละเอียดเพิ่มเติม กรุณาติดต่อผู้ดูแลระบบ")
                }),
                html!("br"),
                link!(Route::Index.string(), {
                    .class(class::BTN_BLUE)
                    .text("กลับสู่หน้าหลัก")
                })
            ])
        })
    }
}
