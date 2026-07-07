use dominator::{Dom, html};
use std::rc::Rc;

use kphis_ui_app::App;
use kphis_ui_component::gadget::image::ImageCpn;

#[derive(Clone, Default)]
pub struct ImagePage {}

impl ImagePage {
    pub fn render(app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Image Cache");

        html!("section", {
            .class("m-3")
            .children([
                html!("p", {
                    .class("lead")
                    .text("รูปภาพสำรองใน Application")
                }),
                ImageCpn::render("calc(100vh - 300px)", ImageCpn::new_using_local_storage(), app.clone()),
            ])
        })
    }
}
