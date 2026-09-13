use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_image_page() {
    let app = new_app();
    let dom = kphis_ui_page::image::ImagePage::render(app);
    replace_body(dom).await;
}
