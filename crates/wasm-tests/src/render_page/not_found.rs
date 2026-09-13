use wasm_bindgen_test::wasm_bindgen_test;

use crate::replace_body;

#[wasm_bindgen_test]
async fn test_not_found_page() {
    let page = kphis_ui_page::not_found::NotFoundPage { path: String::from("unknown") };
    let dom = kphis_ui_page::not_found::NotFoundPage::render(&page);
    replace_body(dom).await;
}
