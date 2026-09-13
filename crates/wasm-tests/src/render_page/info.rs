use time::macros::date;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_info_page() {
    let app = new_app();
    let page = kphis_ui_page::info::InfoPage::new();
    let dom = kphis_ui_page::info::InfoPage::render(page.clone(), app.clone());
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_info_page_announcement() {
    let page = kphis_ui_page::info::Announcement {title: String::from("title"), date: date!(2023-12-31), items: vec![String::from("item")]};
    let dom = kphis_ui_page::info::Announcement::render(&page);
    replace_body(dom).await;
}
