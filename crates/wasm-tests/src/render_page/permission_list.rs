use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_permission_list_page() {
    let app = new_app();
    let page = kphis_ui_page::permission_list::PermissionListPage::new();
    let dom = kphis_ui_page::permission_list::PermissionListPage::render(page, app);
    replace_body(dom).await;
}
