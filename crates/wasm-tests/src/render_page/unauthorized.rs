use wasm_bindgen_test::wasm_bindgen_test;

use crate::replace_body;

#[wasm_bindgen_test]
async fn test_unauthorized_page() {
    let page = kphis_ui_page::unauthorized::UnAuthorizedPage { hash: String::from("/unknown") };
    let dom = kphis_ui_page::unauthorized::UnAuthorizedPage::render(&page);
    replace_body(dom).await;
}
