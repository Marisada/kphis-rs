use dominator::html;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_permission_list_page() {
    let app = new_app();
    let page = kphis_ui_page::permission_list::PermissionListPage::new();
    let dom = kphis_ui_page::permission_list::PermissionListPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_permission_list_page_li() {
    let app = new_app();
    let page = kphis_ui_page::permission_list::PermissionListPage::new();
    let doms = kphis_ui_page::permission_list::PermissionListPage::render_li(Rc::new(kphis_model::user::role::RolePermissionList::demo()), page, app);
    replace_body(html!("div", {.children(doms)})).await;
}

#[wasm_bindgen_test]
async fn test_permission_list_page_parent() {
    let app = new_app();
    let page = kphis_ui_page::permission_list::PermissionListPage::new();
    let dom = kphis_ui_page::permission_list::PermissionListPage::render_parent_view(1, Rc::new(kphis_model::user::role::RolePermissionList::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_permission_list_page_table() {
    let app = new_app();
    let page = kphis_ui_page::permission_list::PermissionListPage::new();
    let dom = kphis_ui_page::permission_list::PermissionListPage::render_table_view(1, Mutable::new(1), Rc::new(kphis_model::user::role::RolePermissionList::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_permission_list_page_manage_modal() {
    let app = new_app();
    let page = kphis_ui_page::permission_list::PermissionListPage::new();
    let dom = kphis_ui_page::permission_list::PermissionListPage::render_manage_modal(page, Mutable::new(true), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_permission_list_page_delete_modal() {
    let app = new_app();
    let page = kphis_ui_page::permission_list::PermissionListPage::new();
    let dom = kphis_ui_page::permission_list::PermissionListPage::render_delete_modal(page, Mutable::new(true), app);
    replace_body(dom).await;
}
