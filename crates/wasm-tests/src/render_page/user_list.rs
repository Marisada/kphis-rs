use dominator::html;
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_user_list_page() {
    let app = new_app();
    let page = kphis_ui_page::user_list::UserListPage::new();
    let dom = kphis_ui_page::user_list::UserListPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_user_list_page_modal() {
    let app = new_app();
    let page = kphis_ui_page::user_list::UserListPage::new();
    let dom = kphis_ui_page::user_list::UserListPage::render_modal(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_user_list_page_li() {
    let page = kphis_ui_page::user_list::UserListPage::new();
    let doms = kphis_ui_page::user_list::UserListPage::render_li(&Rc::new(kphis_model::user::role::Role::demo()), page);
    replace_body(html!("div", {.children(doms)})).await;
}

#[wasm_bindgen_test]
async fn test_user_list_page_result() {
    let app = new_app();
    let page = kphis_ui_page::user_list::UserListPage::new();
    let dom = kphis_ui_page::user_list::UserListPage::render_result(1, Rc::new(kphis_model::user::role::UserRole::demo()), page, app);
    replace_body(dom).await;
}
