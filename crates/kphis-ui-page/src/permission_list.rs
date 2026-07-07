use dominator::{Dom, EventOptions, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{collections::HashSet, rc::Rc};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlOptionElement, HtmlSelectElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    user::{
        permission::Permission,
        role::{Role, RolePermissionList, RolePermissionSave, UserRoleOptions, UserRoleParams},
    },
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::util::str_some;

use crate::user_list::fill_parent_recursive;

/// - GET `EndPoint::UserRolePrelude`
/// - GET `EndPoint::UserRoleRole`
/// - POST `EndPoint::UserRoleRole` (guarded, remove 'บันทึก' btn)
/// - DELETE `EndPoint::UserRoleRole` (guarded, remove 'ลบ' btn)
#[derive(Clone, Default)]
pub struct PermissionListPage {
    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    roles: Mutable<Vec<Rc<Role>>>,
    permissions: MutableVec<Permission>,

    set_all: Mutable<bool>,
    all_role_permissions: MutableVec<Rc<RolePermissionList>>,

    role: Mutable<String>,
    permission: Mutable<String>,
    role_permissions: MutableVec<Rc<RolePermissionList>>,

    modal_changed: Mutable<bool>,
    modal_parents: MutableVec<String>,
    modal_role_prev: Mutable<String>,
    modal_parent_selected_option: Mutable<Option<(String, String)>>,

    modal_role: Mutable<String>,
    modal_role_desc: Mutable<String>,
    modal_parent_role: Mutable<String>,
    modal_permissions: MutableVec<Permission>,

    modal_confirm_changed: Mutable<bool>,
    modal_confirm_text: Mutable<String>,
}

impl PermissionListPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            modal_parent_selected_option: Mutable::new(Some((String::new(), String::from("ไม่มี")))),
            ..Default::default()
        })
    }

    fn modal_confirm_ready(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let changed = self.modal_changed.signal(),
            let role = self.modal_role.signal_cloned(),
            let confirm = self.modal_confirm_text.signal_cloned() =>
            !changed && !role.is_empty() && !confirm.is_empty()
        }
    }

    fn load_options(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::UserRolePrelude`
                match UserRoleOptions::call_api_get(app.state()).await {
                    Ok(response) => {
                        {
                            let mut roles_lock = page.roles.lock_mut();
                            roles_lock.clear();
                            roles_lock.extend(response.roles.into_iter().map(Rc::new));
                        }
                        {
                            let mut permissions_lock = page.permissions.lock_mut();
                            permissions_lock.replace_cloned(response.permissions);
                        }
                        page.changed.set_neq(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn role_option(roles: Vec<Rc<Role>>, modal_role: &str, page: Rc<Self>) -> Vec<Dom> {
        let mut options: Vec<Dom> = Vec::new();
        let mut space = 0;
        let modal_role_with_child = page.roles_with_children(modal_role.to_owned());
        let (heads, tails): (Vec<Rc<Role>>, Vec<Rc<Role>>) = roles.clone().into_iter().partition(|role| role.parent_role.is_none());
        let heads_len = heads.len();
        if heads_len > 0 {
            for (i, head) in heads.iter().enumerate() {
                let tree_branch = if i == (heads_len - 1) { "└─ " } else { "├─ " };
                options.push(html!("option", {
                    .attr("value",&head.role)
                    .text(&[tree_branch, &head.role_desc.clone().unwrap_or_default()].concat())
                    .apply_if(modal_role_with_child.contains(&head.role), |dom| dom.attr("disabled",""))
                }));
                if i == (heads_len - 1) {
                    space += 1;
                }
                Self::render_option_recursive(1, space, head, &tails, &modal_role_with_child, &mut options);
            }
        }
        options
    }

    fn render_option_recursive(indent: usize, mut space: usize, head: &Rc<Role>, tails: &[Rc<Role>], modal_role_with_child: &[String], options: &mut Vec<Dom>) {
        let heads = tails
            .iter()
            .filter(|role| role.parent_role.as_ref().map(|r| r == &head.role).unwrap_or_default())
            .collect::<Vec<&Rc<Role>>>();
        let heads_len = heads.len();
        if heads_len == 0 {
            space = space.saturating_sub(1);
        }
        if heads_len > 0 && indent > space {
            heads.iter().enumerate().for_each(|(i, role)| {
                let tree_trunk = if indent > 0 { vec!["│ "; indent - space].concat() } else { String::new() };
                let tree_space = if space > 0 { vec!["\u{00a0}"; space * 2].concat() } else { String::new() };
                let tree_branch = if i == (heads_len - 1) { "└─ " } else { "├─ " };
                options.push(html!("option", {
                    .attr("value",&role.role)
                    .text(&[&tree_trunk, &tree_space, tree_branch, &role.role_desc.clone().unwrap_or_default()].concat())
                    .apply_if(modal_role_with_child.contains(&head.role), |dom| dom.attr("disabled",""))
                }));
                if i == (heads_len - 1) {
                    space += 1;
                }
                if tails.iter().any(|x| x.parent_role.as_ref().map(|r| r == &role.role).unwrap_or_default()) {
                    Self::render_option_recursive(indent + 1, space, role, tails, modal_role_with_child, options);
                }
            })
        }
    }

    fn create_parent_badge(perm: &Permission, page: Rc<Self>) -> impl Iterator<Item = Dom> {
        page.all_role_permissions
            .lock_ref()
            .to_vec()
            .into_iter()
            .filter_map(clone!(page, perm => move |role: Rc<RolePermissionList>| {
                (page.modal_parents.lock_ref().contains(&role.role) && role.permissions.as_ref().map(|perms| perms.contains(&perm)).unwrap_or_default()).then(|| {
                    html!("span", {
                        .class(class::BADGE_CYAN_R)
                        .style("cursor","default")
                        .text(&role.role_desc.clone().unwrap_or_default())
                    })
                })
            }))
    }

    fn get_parents(&self) -> impl Iterator<Item = String> {
        let roles = self.roles.lock_ref().to_vec();
        let mut result = HashSet::new();
        let selected = self.modal_role.get_cloned();
        fill_parent_recursive(&selected, &roles, &mut result);
        result.insert(selected);
        result.into_iter()
    }

    fn roles_with_children(&self, role: String) -> Vec<String> {
        if let Some(selected) = str_some(role) {
            let roles = self.roles.lock_ref().to_vec();
            let mut exacts = HashSet::new();
            fill_child_recursive(&selected, &roles, &mut exacts);
            exacts.insert(selected);
            exacts.into_iter().collect()
        } else {
            Vec::new()
        }
    }

    // send GET method
    fn submit(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                let params = UserRoleParams {
                    role: str_some(page.roles_with_children(page.role.get_cloned()).join(",")),
                    permission: str_some(page.permission.get_cloned()),
                    ..Default::default()
                };
                // GET `EndPoint::UserRoleRole`
                match RolePermissionList::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        {
                            let mut roles_lock = page.role_permissions.lock_mut();
                            roles_lock.clear();
                            roles_lock.extend(responses.clone().into_iter().map(Rc::new));
                        }
                        if !page.set_all.get() {
                            page.all_role_permissions.lock_mut().extend(responses.into_iter().map(Rc::new));
                            page.set_all.set(true);
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    fn set_modal_role(role: &Rc<RolePermissionList>, page: Rc<Self>) {
        page.modal_role_prev.set_neq(role.role.clone());
        page.modal_role.set_neq(role.role.clone());
        page.modal_role_desc.set_neq(role.role_desc.clone().unwrap_or_default());
        if let Some(parent) = role.parent_role.clone() {
            let parent_selected_option = page
                .roles
                .lock_ref()
                .iter()
                .find(|r| r.role == parent)
                .map(|r| (r.role.clone(), r.role_desc.clone().unwrap_or_default()));
            page.modal_parent_selected_option.set(parent_selected_option);
            page.modal_parent_role.set_neq(parent.clone());
        } else {
            page.modal_parent_selected_option.set(Some((String::new(), String::from("ไม่มี"))));
            page.modal_parent_role.set_neq(String::new());
        }
        if let Some(permissions) = role.permissions.clone() {
            let mut perms_lock = page.modal_permissions.lock_mut();
            perms_lock.replace_cloned(permissions);
        }
        {
            let mut parents_lock = page.modal_parents.lock_mut();
            parents_lock.clear();
            parents_lock.extend(page.get_parents())
        }
        // flush permission to re-render modal permissions
        let permissions = page.permissions.lock_ref().to_vec();
        page.permissions.lock_mut().replace_cloned(permissions);
        page.modal_changed.set_neq(false);
    }

    fn save(page: Rc<Self>, app: Rc<App>) {
        let save = RolePermissionSave {
            role_prev: str_some(page.modal_role_prev.get_cloned()),
            role: page.modal_role.get_cloned(),
            role_desc: page.modal_role_desc.get_cloned(),
            parent_role: str_some(page.modal_parent_role.get_cloned()),
            permissions: page.modal_permissions.lock_ref().to_vec(),
        };
        app.async_load(
            true,
            clone!(app => async move {
                // POST `EndPoint::UserRoleRole`
                match save.call_api_post(app.state()).await {
                    Ok(responses) => {
                        app.alert_execute_responses(&responses, async move {
                            // reload all page
                            page.loaded.set_neq(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn delete(page: Rc<Self>, app: Rc<App>) {
        let params = UserRoleParams {
            role: str_some(page.modal_role.get_cloned()),
            parent_role: str_some(page.modal_parent_role.get_cloned()),
            ..Default::default()
        };
        app.async_load(
            true,
            clone!(app => async move {
                // DELETE `EndPoint::UserRoleRole`
                match RolePermissionSave::call_api_delete(&params, app.state()).await {
                    Ok(responses) => {
                        app.alert_execute_responses(&responses, async move {
                            // reload all page
                            page.loaded.set_neq(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Permission List");

        html!("section", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_options(page.clone(), app.clone());
                    page.loaded.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit(page.clone(), app.clone());
                    page.changed.set(false);
                }
                async {}
            })))
            .class("container-fluid")
            .children([
                doms::alert_row(clone!(app, page => move |alert| { alert
                    .children([
                        doms::form_inline(clone!(app, page => move |form| { form
                            .children([
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("role","Role"),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM_MONO)
                                            // .style("width","250px")
                                            .attr("id", "role")
                                            .children([
                                                html!("option", {
                                                    .attr("value", "")
                                                    .attr("selected","selected")
                                                    .style("display","none")
                                                    .text("เลือก")
                                                }),
                                                html!("option", {
                                                    .attr("value", "")
                                                    .text("ทั้งหมด")
                                                }),
                                            ])
                                            .children_signal_vec(page.roles.signal_cloned().map(clone!(page => move |roles| {
                                                Self::role_option(roles, "", page.clone())
                                            })).to_signal_vec())
                                            .with_node!(element => {
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    let v = element.value();
                                                    if let Some(role) = page.roles.lock_ref().iter().find(|role| role.role == v) {
                                                        let option = HtmlOptionElement::new_with_text_and_value_and_default_selected_and_selected(
                                                            &role.role_desc.clone().unwrap_or_default(),
                                                            &v,
                                                            true,
                                                            true,
                                                        ).ok();
                                                        if let Err(e) = element.options().set(0, option.as_ref()) {
                                                            app.show_jsvalue_message(&e);
                                                        }
                                                    }
                                                    let neq = page.role.lock_ref().as_str() != v.as_str();
                                                    if neq {
                                                        page.role.set(v);
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                            })
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("permission","Permission"),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM)
                                            // .style("width","250px")
                                            .attr("id", "permission")
                                            .child(html!("option", {.attr("value", "").text("เลือก")}))
                                            .children_signal_vec(page.permissions.signal_vec_cloned().map(|permission| {
                                                html!("option", {.attr("value", permission.str()).text(permission.str())})
                                            }))
                                            .apply(mixins::string_value_select(page.permission.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_end(clone!(app, page => move |end| {end
                                    .apply_if(app.has_permission(Permission::SystemAcRolePermissionAdd), |can_add| { can_add
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_L_BLUE)
                                            //.attr("id", "buttonCreateNewRole")
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#permissionManageModal")
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .text(" สร้าง Role")
                                            .event(clone!(page => move |_:events::Click| {
                                                page.modal_role_prev.set_neq(String::new());
                                                page.modal_role.set_neq(String::new());
                                                page.modal_role_desc.set_neq(String::new());
                                                page.modal_parent_role.set_neq(String::new());
                                                page.modal_permissions.lock_mut().clear();
                                                page.modal_changed.set_neq(false);
                                            }))
                                        }))
                                    })
                                    .child(html!("ul", {
                                        .class(class::NAV_PILLS_R)
                                        .children([
                                            html!("li", {
                                                .class("nav-item")
                                                .child(html!("a", {
                                                    .class(class::NAV_LINK_ACTIVE)
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#treeView")
                                                    .child(html!("i", {.class(class::FA_TREE)}))
                                                    .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                                        event.prevent_default();
                                                    })
                                                }))
                                            }),
                                            html!("li", {
                                                .class("nav-item")
                                                .child(html!("a", {
                                                    .class("nav-link")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#tableParentView")
                                                    .child(html!("i", {.class(class::FA_TH_LG)}))
                                                    .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                                        event.prevent_default();
                                                    })
                                                }))
                                            }),
                                            html!("li", {
                                                .class("nav-item")
                                                .child(html!("a", {
                                                    .class("nav-link")
                                                    .attr("data-bs-toggle", "pill")
                                                    .attr("href", "#tableView")
                                                    .child(html!("i", {.class(class::FA_TH)}))
                                                    .event_with_options(&EventOptions::preventable(), |event: events::Click| {
                                                        event.prevent_default();
                                                    })
                                                }))
                                            }),
                                        ])
                                    }))
                                })),
                            ])
                        })),
                    ])
                })),
                html!("div", {
                    .class("tab-content")
                    .children([
                        html!("div", {
                            .class(class::TAB_ACTIVE)
                            .attr("id", "treeView")
                            .child(html!("div", {
                                .class("card")
                                .children([
                                    html!("div", {.class("card-header").text("Role")}),
                                    html!("div", {
                                        .class("card-body")
                                        .child(html!("div", {
                                            .class("row")
                                            .style("height","calc(100vh - 300px")
                                            .style("overflow-y","auto")
                                            .child(html!("div", {
                                                .class("col")
                                                .style("column-width","480px")
                                                .children([
                                                    html!("ul", {
                                                        .children_signal_vec(page.role_permissions.signal_vec_cloned().to_signal_cloned().map(clone!(page => move |roles| {
                                                            match str_some(page.role.get_cloned()) {
                                                                // show only selected role
                                                                Some(index_role) => {
                                                                    if let Some(head) = roles.iter().find(|role| role.role == index_role) {
                                                                        vec![html!("li", {
                                                                            .style("break-inside","avoid")
                                                                            .children(Self::render_li(head.clone(), page.clone()))
                                                                            .apply_if(roles.iter().any(|x| x.parent_role.as_ref().map(|r| r == &head.role).unwrap_or_default()), clone!(page, roles => move |dom| {
                                                                                dom.child(html!("ul", {
                                                                                    .children(Self::render_li_recursive(head, &roles, page))
                                                                                }))
                                                                            }))
                                                                        })]
                                                                    } else {
                                                                        Vec::new()
                                                                    }
                                                                }
                                                                // show all
                                                                None => {
                                                                    let (heads, tails): (Vec<Rc<RolePermissionList>>,Vec<Rc<RolePermissionList>>) = roles.into_iter().partition(|role| role.parent_role.is_none());
                                                                    heads.into_iter().map(clone!(page => move |head| {
                                                                        html!("li", {
                                                                            .style("break-inside","avoid")
                                                                            .children(Self::render_li(head.clone(), page.clone()))
                                                                            .apply_if(tails.iter().any(|x| x.parent_role.as_ref().map(|r| r == &head.role).unwrap_or_default()), clone!(page, tails => move |dom| {
                                                                                dom.child(html!("ul", {
                                                                                    .children(Self::render_li_recursive(&head, &tails, page))
                                                                                }))
                                                                            }))
                                                                        })
                                                                    })).collect::<Vec<Dom>>()
                                                                }
                                                            }
                                                        })).to_signal_vec())
                                                    }),
                                                ])
                                            }))
                                        }))
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("tab-pane")
                            .attr("id", "tableParentView")
                            .child(doms::table_responsive(class::TABLE_STRIP, clone!(page => move |table| { table
                                .children([
                                    html!("thead", {
                                        .class("text-center")
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("th-sm").attr("scope", "col").text("#")}),
                                                html!("th", {.class("th-sm").attr("scope", "col").text("Role")}),
                                                html!("th", {.class("th-sm").attr("scope", "col").text("Parent role")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children_signal_vec(page.role_permissions.signal_vec_cloned().enumerate().map(clone!(page => move |(i,role)| {
                                            Self::render_parent_view(i.get().unwrap_or_default(), role, page.clone())
                                        })))
                                    }),
                                ])
                            })))
                        }),
                        html!("div", {
                            .class(class::TAB_FADE)
                            .attr("id", "tableView")
                            .child(doms::table_responsive(class::TABLE_STRIP, clone!(page => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .class("text-center")
                                            .children([
                                                html!("th", {.class("th-sm").attr("scope", "col").text("#")}),
                                                html!("th", {.class("th-sm").attr("scope", "col").text("Role")}),
                                                html!("th", {.class("th-sm").attr("scope", "col").text("Permission")}),
                                                html!("th", {.class("th-sm").attr("scope", "col").text("Parent role")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children_signal_vec(page.role_permissions.signal_vec_cloned().enumerate().map(clone!(page => move |(i,role)| {
                                            Self::render_table_view(i.get().unwrap_or_default(), role, page.clone())
                                        })))
                                    }),
                                ])
                            })))
                        }),
                    ])
                }),
                Self::render_manage_modal(page.clone(), app.clone()),
                Self::render_delete_modal(page.clone(), app.clone()),
            ])
        })
    }

    fn render_li_recursive(head: &Rc<RolePermissionList>, tails: &[Rc<RolePermissionList>], page: Rc<Self>) -> impl Iterator<Item = Dom> {
        tails
            .iter()
            .filter(|role| role.parent_role.as_ref().map(|r| r == &head.role).unwrap_or_default())
            .map(clone!(page, tails => move |role| html!("li", {
                .children(Self::render_li(role.clone(), page.clone()))
                .apply_if(tails.iter().any(|x| x.parent_role.as_ref().map(|r| r == &role.role).unwrap_or_default()), clone!(page, tails => move |dom| {
                    dom.child(html!("ul", {
                        .children(Self::render_li_recursive(role, tails, page))
                    }))
                }))
            })))
    }

    fn render_li(role: Rc<RolePermissionList>, page: Rc<Self>) -> Vec<Dom> {
        vec![
            html!("span", {
                .style("cursor","pointer")
                .attr("data-bs-toggle", "modal")
                .attr("data-bs-target", "#permissionManageModal")
                .text(&role.role_desc.clone().unwrap_or_default())
                .event(clone!(role => move |_:events::Click| {
                    Self::set_modal_role(&role, page.clone());
                }))
            }),
            html!("span", {
                .class(class::BADGE_CYAN_R)
                .style("cursor","default")
                .apply_if(role.permissions.is_some(), |dom| {
                    let perms_len = role.permissions.as_ref().map(|perms| perms.len()).unwrap_or_default();
                    dom.text(&[&perms_len.to_string(), " สิทธิ์"].concat())
                })
            }),
        ]
    }

    fn render_parent_view(i: usize, role: Rc<RolePermissionList>, page: Rc<Self>) -> Dom {
        html!("tr", {
            .style("cursor","pointer")
            .attr("data-bs-toggle", "modal")
            .attr("data-bs-target", "#permissionManageModal")
            .children([
                html!("td", {.class("text-center").text(&(i + 1).to_string())}),
                html!("td", {.text(&role.role_desc.clone().unwrap_or_default())}),
                html!("td", {.text(&role.parent_role_desc.clone().unwrap_or_default())}),
            ])
            .event(clone!(role => move |_:events::Click| {
                Self::set_modal_role(&role, page.clone());
            }))
        })
    }

    fn render_table_view(i: usize, role: Rc<RolePermissionList>, page: Rc<Self>) -> Dom {
        let perms_dom = role
            .permissions
            .as_ref()
            .map(|perms| perms.iter().map(|perm| html!("li", {.text(perm.str())})).collect::<Vec<Dom>>())
            .unwrap_or_default();

        html!("tr", {
            .style("cursor","pointer")
            .attr("data-bs-toggle", "modal")
            .attr("data-bs-target", "#permissionManageModal")
            .children([
                html!("td", {.class("text-center").text(&(i + 1).to_string())}),
                html!("td", {.text(&role.role_desc.clone().unwrap_or_default())}),
                html!("td", {
                    .child(html!("ul", {
                        .style("margin-bottom","0px")
                        .children(perms_dom)
                    }))
                }),
                html!("td", {.text(&role.parent_role_desc.clone().unwrap_or_default())}),
            ])
            .event(clone!(role => move |_:events::Click| {
                Self::set_modal_role(&role, page.clone());
            }))
        })
    }

    fn render_manage_modal(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("modal")
            .attr("id", "permissionManageModal")
            .attr("role", "dialog")
            .attr("tabindex", "-1")
            .child(html!("div", {
                .class(class::MODAL_DIALOG_XL)
                .children([
                    html!("div", {
                        .class("modal-content")
                        .children([
                            html!("div", {
                                .class("modal-header")
                                .children([
                                    html!("h4", {
                                        .class("modal-title")
                                        .text("จัดการบทบาท")
                                    }),
                                    doms::close_modal_x_btn(),
                                ])
                            }),
                            html!("div", {
                                .class("modal-body")
                                .children([
                                    html!("div", {
                                        //.attr("id", "modal_form")
                                        .child(html!("div", {
                                            .class("row")
                                            .children([
                                                html!("div", {
                                                    .class(class::COL_T)
                                                    .children([
                                                        html!("label", {.attr("for", "modal_role").text("Role")}),
                                                        html!("input" => HtmlInputElement, {
                                                            .attr("type", "text")
                                                            .class("form-control")
                                                            .attr("id", "modal_role")
                                                            .apply(mixins::string_value(page.modal_role.clone(), page.modal_changed.clone()))
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(class::COL_T)
                                                    .children([
                                                        html!("label", {.attr("for", "modal_role_desc").text("Role Description")}),
                                                        html!("input" => HtmlInputElement, {
                                                            .attr("type", "text")
                                                            .class("form-control")
                                                            .attr("id", "modal_role_desc")
                                                            .apply(mixins::string_value(page.modal_role_desc.clone(), page.modal_changed.clone()))
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(class::COL_T)
                                                    .children([
                                                        html!("label", {.attr("for", "modal_parent_role_select").text("Parent Role")}),
                                                        html!("select" => HtmlSelectElement, {
                                                            .class(class::FORM_SELECT_MONO)
                                                            .attr("id", "modal_parent_role_select")
                                                            .child(html!("option", {
                                                                .attr("value", "")
                                                                .attr("selected","selected")
                                                                .style("display","none")
                                                                .text("เลือก")
                                                            }))
                                                            .children_signal_vec(map_ref!{
                                                                let modal_role = page.modal_role.signal_cloned(),
                                                                let roles = page.roles.signal_cloned() =>
                                                                (modal_role.clone(), roles.clone())
                                                            }.map(clone!(page => move |(modal_role, roles)| {
                                                                Self::role_option(roles, &modal_role, page.clone())
                                                            })).to_signal_vec())
                                                            .prop_signal("value", page.modal_parent_role.signal_cloned())
                                                            .with_node!(element => {
                                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                                    let v = element.value();
                                                                    if let Some(role) = page.roles.lock_ref().iter().find(|role| role.role == v) {
                                                                        let option = HtmlOptionElement::new_with_text_and_value_and_default_selected_and_selected(
                                                                            &role.role_desc.clone().unwrap_or_default(),
                                                                            &v,
                                                                            true,
                                                                            true,
                                                                        ).ok();
                                                                        if let Err(e) = element.options().set(0, option.as_ref()) {
                                                                            app.show_jsvalue_message(&e);
                                                                        }
                                                                    }
                                                                    let neq = page.modal_parent_role.lock_ref().as_str() != v.as_str();
                                                                    if neq {
                                                                        let option = page.roles.lock_ref().iter().find(|role| role.role == v)
                                                                            .map(|role| (role.role.clone(), role.role_desc.clone().unwrap_or_default()));
                                                                        page.modal_parent_selected_option.set(option);
                                                                        page.modal_parent_role.set(v);
                                                                        page.modal_changed.set_neq(true);
                                                                    }
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("div", {
                                        .class("card")
                                        .children([
                                            html!("div", {.class("card-header").text("Permissions")}),
                                            html!("div", {
                                                .class("card-body")
                                                .style("height","calc(100vh - 400px)")
                                                .style("overflow-y","auto")
                                                .child(html!("div", {
                                                    .class("col")
                                                    .style("column-count","auto")
                                                    .style("column-width","480px")
                                                    .children([
                                                        html!("div", {
                                                            //.attr("id", "permission_checkbox_form")
                                                            .children_signal_vec(page.permissions.signal_vec_cloned().map(clone!(page => move |perm| {
                                                                html!("div", {
                                                                    .class("form-check")
                                                                    .style("padding-left","50px")
                                                                    .children([
                                                                        html!("input" => HtmlInputElement, {
                                                                            .attr("type", "checkbox")
                                                                            .attr("id",perm.str())
                                                                            .class("form-check-input")
                                                                            .with_node!(element => {
                                                                                .future(page.modal_permissions.signal_vec_cloned().to_signal_cloned().for_each(clone!(element, perm => move |items| {
                                                                                    element.set_checked(items.contains(&perm));
                                                                                    async {}
                                                                                })))
                                                                                .event(clone!(page, element, perm => move |_: events::Change| {
                                                                                    if element.checked() {
                                                                                        page.modal_permissions.lock_mut().push_cloned(perm.clone());
                                                                                    } else {
                                                                                        page.modal_permissions.lock_mut().retain(|x| x != &perm)
                                                                                    }
                                                                                    page.modal_changed.set_neq(true);
                                                                                }))
                                                                            })
                                                                        }),
                                                                        doms::label_check_for(perm.str(), perm.str()),
                                                                    ])
                                                                    .children(Self::create_parent_badge(&perm, page.clone()))
                                                                })
                                                            })))
                                                        }),
                                                    ])
                                                }))
                                            }),
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class("modal-footer")
                                .child_signal(page.modal_role_prev.signal_cloned().map(clone!(app, page => move |prev| {
                                    (app.endpoint_is_allow(&Method::POST, &EndPoint::UserRoleRole, false)
                                    && if prev.is_empty() {
                                        app.has_permission(Permission::SystemAcRolePermissionAdd)
                                    } else {
                                        app.has_permission(Permission::SystemAcRolePermissionEdit)
                                    }).then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class("btn")
                                            .class_signal("btn-primary", page.modal_changed.signal())
                                            .class_signal("btn-secondary", not(page.modal_changed.signal()))
                                            .attr("data-bs-dismiss", "modal")
                                            .text("บันทึก")
                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                Self::save(page.clone(), app.clone());
                                            }), map_ref!{
                                                let changed = page.modal_changed.signal(),
                                                let role = page.modal_role.signal_cloned(),
                                                let role_desc = page.modal_role_desc.signal_cloned() =>
                                                !changed || role.is_empty() || role_desc.is_empty()
                                            }, app.state()))
                                        })
                                    })
                                })))
                                .child_signal(page.modal_role.signal_cloned().map(clone!(app, page => move |role| {
                                    (!role.is_empty() && app.has_permission(Permission::SystemAcRolePermissionRemove)).then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .class(class::BTN_RED)
                                            .attr("type", "button")
                                            //.attr("id", "button-delete-role")
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#confirmDeleteModal")
                                            .text("ลบ")
                                            .with_node!(element => {
                                                .future(page.modal_changed.signal().for_each(move |changed| {
                                                    element.set_disabled(changed);
                                                    async {}
                                                }))
                                            })
                                            .event(clone!(page => move |_:events::Click| {
                                                page.modal_confirm_text.set_neq(String::new());
                                                page.modal_confirm_changed.set_neq(false);
                                            }))
                                        })
                                    })
                                })))
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .attr("data-bs-dismiss", "modal")
                                    .class(class::BTN_GRAY)
                                    .text("ปิด")
                                }))
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    fn render_delete_modal(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("modal")
            .attr("id", "confirmDeleteModal")
            .attr("role", "dialog")
            .attr("tabindex", "-1")
            .children([
                html!("div", {
                    .class(class::MODAL_DIALOG_SM_C)
                    .children([
                        html!("div", {
                            .class("modal-content")
                            .children([
                                html!("div", {
                                    .class("modal-header")
                                    .children([
                                        html!("h4", {
                                            .class("modal-title")
                                            .text("ยืนยันการลบข้อมูล")
                                        }),
                                        doms::close_modal_x_btn(),
                                    ])
                                }),
                                html!("div", {
                                    .class("modal-body")
                                    .children([
                                        html!("div", {
                                            .class("row")
                                            .children([
                                                html!("div", {
                                                    .class("col")
                                                    .children([
                                                        html!("div", {
                                                            //.attr("id", "confirmDeleteRole")
                                                            .children([
                                                                html!("div", {
                                                                    .class("mb-3")
                                                                    .children([
                                                                        html!("label", {.attr("for", "confirm_delete_role").text("พิมพ์ 'DELETE'")}),
                                                                        html!("input" => HtmlInputElement, {
                                                                            .attr("type", "text")
                                                                            .class("form-control")
                                                                            .attr("required", "")
                                                                            .attr("autocomplete", "off")
                                                                            .apply(mixins::string_value(page.modal_confirm_text.clone(), page.modal_confirm_changed.clone()))
                                                                        }),
                                                                    ])
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                            ])
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("modal-footer")
                                    .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::UserRoleRole, false), |dom| {
                                        dom.child(html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class("btn")
                                            .class_signal("btn-danger", page.modal_confirm_changed.signal())
                                            .class_signal("btn-secondary", not(page.modal_confirm_changed.signal()))
                                            .attr("data-bs-dismiss", "modal")
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#permissionManageModal")
                                            .text("ลบ")
                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                Self::delete(page.clone(), app.clone());
                                            }), not(page.modal_confirm_ready()), app.state()))
                                        }))
                                    })
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_GRAY)
                                        .attr("data-bs-dismiss", "modal")
                                        .text("ยกเลิก")
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
            ])
        })
    }
}

fn fill_child_recursive(text: &str, roles: &[Rc<Role>], set: &mut HashSet<String>) {
    roles.iter().filter(|r| r.parent_role.as_ref().map(|pr| pr == text).unwrap_or_default()).for_each(|r| {
        fill_child_recursive(&r.role, roles, set);
        set.insert(r.role.clone());
    })
}
