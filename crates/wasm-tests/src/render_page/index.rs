use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_index_page() {
    let app = new_app();
    let page = kphis_ui_page::index::IndexPage::new();
    let dom = kphis_ui_page::index::IndexPage::render(page, app);
    replace_body(dom).await;
}
