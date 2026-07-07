use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
    signal_map::MutableBTreeMap,
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{
    collections::{BTreeMap, HashSet},
    rc::Rc,
};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    user::{
        config::UserConfigCommand,
        permission::Permission,
        role::{Role, RolePermission, UserRole, UserRoleList, UserRoleOptions, UserRoleParams, UserRoleSave},
    },
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::util::str_some;

/// - GET `EndPoint::UserRolePrelude`
/// - GET `EndPoint::UserRoleUser`
/// - POST `EndPoint::UserRoleUser` (guarded, remove 'บันทึก' btn)
/// - PATCH `EndPoint::UserConfig` (guarded, remove 'ยกเลิก 2FA' btn)
#[derive(Clone, Default)]
pub struct UserListPage {
    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    roles: MutableVec<Rc<Role>>,
    roles_option: MutableVec<Rc<Role>>,
    hosxp_groups: MutableVec<String>,
    role_permissions: MutableVec<Rc<RolePermission>>,

    loginname: Mutable<String>,
    name: Mutable<String>,
    role: Mutable<String>,
    hosxp_group: Mutable<String>,
    account_disable: Mutable<String>,
    search_result: MutableVec<Rc<UserRole>>,

    modal_changed: Mutable<bool>,

    modal_loginname: Mutable<String>,
    modal_name: Mutable<String>,
    modal_roles: MutableVec<String>,
    modal_hosxp_group: Mutable<String>,
    modal_account_disable: Mutable<String>,
    modal_user_permissions: MutableBTreeMap<Permission, Vec<String>>,
    modal_has_totp: Mutable<bool>,
}

impl UserListPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            hosxp_group: Mutable::new(String::from("ALL_GROUP")),
            ..Default::default()
        })
    }

    fn modal_roles_with_parents(&self) -> Vec<String> {
        let roles = self.roles.lock_ref().to_vec();
        let selected = self.modal_roles.lock_ref().to_vec();
        let mut exacts = HashSet::new();
        for select in selected {
            fill_parent_recursive(&select, &roles, &mut exacts);
            exacts.insert(select);
        }
        exacts.into_iter().collect()
    }

    // return BTreeMap<permission, Vec<role_desc>>
    fn role_permission(&self) -> BTreeMap<Permission, Vec<String>> {
        let roles = self.roles.lock_ref().to_vec();
        let role_permissions = self.role_permissions.lock_ref().to_vec();
        let exact_roles = self.modal_roles_with_parents();
        let mut result: BTreeMap<Permission, Vec<String>> = BTreeMap::new();
        role_permissions.iter().filter(|p| exact_roles.contains(&p.role)).for_each(|role_permission| {
            if let Some(role_desc) = roles.iter().find(|role| role.role == role_permission.role).and_then(|role| role.role_desc.clone()) {
                if let Some(role_descs) = result.get_mut(&role_permission.permission) {
                    role_descs.push(role_desc);
                } else {
                    result.insert(role_permission.permission.clone(), vec![role_desc]);
                }
            }
        });
        result
    }

    // run once after loaded
    fn load_options(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::UserRolePrelude`
                match UserRoleOptions::call_api_get(app.state()).await {
                    Ok(response) => {
                        page.roles.lock_mut().extend(response.roles.clone().into_iter().map(Rc::new));
                        page.roles_option.lock_mut().extend(response.roles.into_iter().map(Rc::new));
                        page.hosxp_groups.lock_mut().replace_cloned(response.hosxp_groups);
                        page.role_permissions.lock_mut().extend(response.role_permissions.into_iter().map(Rc::new));
                        page.changed.set_neq(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    // send GET method
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let group = page.hosxp_group.get_cloned();
        let hosxp_group = if group.as_str() == "ALL_GROUP" { None } else { Some(group) };
        let params = UserRoleParams {
            loginname: str_some(page.loginname.get_cloned()),
            name: str_some(page.name.get_cloned()),
            role: str_some(page.role.get_cloned()),
            hosxp_group,
            account_disable: str_some(page.account_disable.get_cloned()),
            ..Default::default()
        };
        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::UserRoleUser`
                match UserRoleList::call_api_get(&params, app.state()).await {
                    Ok(response) => {
                        {
                            let mut list_lock = page.search_result.lock_mut();
                            list_lock.clear();
                            list_lock.extend(response.user_roles.into_iter().map(Rc::new));
                        }
                        {
                            let mut roles_lock = page.roles.lock_mut();
                            roles_lock.clear();
                            roles_lock.extend(response.roles.into_iter().map(Rc::new));
                        }
                    }
                    Err(e) => {
                        if e.status == 401 {
                            app.alert("ท่านได้รับการเปลี่ยนแปลงบทบาทไหม่", "กรุณาเข้าสู่ระบบใหม่อีกครั้ง เพื่อใช้งานบทบาทใหม่")
                        } else {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    // system-ac-role-user-save.php
    fn save(page: Rc<Self>, app: Rc<App>) {
        let save = UserRoleSave {
            loginname: page.modal_loginname.get_cloned(),
            roles: page.modal_roles.lock_ref().to_vec(),
        };
        app.async_load(
            true,
            clone!(app, page => async move {
                // POST `EndPoint::UserRoleUser`
                match save.call_api_post(app.state()).await {
                    Ok(responses) => {
                        app.alert_execute_responses(&responses, async move {
                            page.changed.set_neq(true);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    fn remove_2fa(target_loginname: String, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                // PATCH `EndPoint::UserConfig`
                match UserConfigCommand::Clear2fa(target_loginname).call_api_patch(app.state()).await {
                    Ok(response) => {
                        if response.rows_affected > 0 {
                            page.changed.set_neq(true);
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
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
            .attr("id", "content")
            .children([
                doms::alert_row(clone!(page => move |alert| { alert
                    .children([
                        doms::form_inline(clone!(page => move |form| { form
                            .children([
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("loginname","Login name"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class("form-control")
                                            .attr("id", "loginname")
                                            .apply(mixins::string_value_end(page.loginname.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("name","ชื่อ-นามสกุล"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class("form-control")
                                            .attr("id", "name")
                                            .apply(mixins::string_value_end(page.name.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("role","Role"),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM)
                                            .attr("id", "role")
                                            .child(html!("option", {.attr("value", "").text("เลือก") }))
                                            .children_signal_vec(page.roles_option.signal_vec_cloned().map(|role| {
                                                html!("option", {.attr("value", &role.role).text(&role.role_desc.clone().unwrap_or_default())})
                                            }))
                                            .apply(mixins::string_value_select(page.role.clone(), page.changed.clone()))
                                            // .attr("onchange", "onchangeParameter(event)")
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("hosxp-group","HOSxP group"),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM)
                                            .attr("id", "hosxp-group")
                                            .child(html!("option", {.attr("value", "ALL_GROUP").text("เลือก")}))
                                            .children_signal_vec(page.hosxp_groups.signal_vec_cloned().map(|group| {
                                                html!("option", {.attr("value", &group).text(&group)})
                                            }))
                                            .apply(mixins::string_value_select(page.hosxp_group.clone(), page.changed.clone()))
                                            // .attr("onchange", "onchangeParameter(event)")
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("account-disable","ปิดใช้งาน"),
                                        html!("select" => HtmlSelectElement, {
                                            .class("form-select")
                                            .attr("id", "account-disable")
                                            .children([
                                                html!("option", {.attr("value", "").text("เลือก")}),
                                                html!("option", {.attr("value", "Y").text("ปิดใช้งาน")}),
                                                html!("option", {.attr("value", "N").text("เปิดใช้งาน")}),
                                            ])
                                            .apply(mixins::string_value_select(page.account_disable.clone(), page.changed.clone()))
                                            // .attr("onchange", "onchangeParameter(event)")
                                        }),
                                    ])
                                })),
                                doms::form_inline_end(clone!(page => move |end| {end
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_SEARCH)}))
                                        .text(" ค้นหา")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.changed.set_neq(true);
                                        }))
                                    }))
                                })),
                            ])
                        })),
                    ])
                })),
                doms::table_responsive(class::TABLE_STRIP, clone!(app, page => move |table| { table
                    .children([
                        html!("thead", {
                            .child(html!("tr", {
                                .class("text-center")
                                .children([
                                    html!("th", {.attr("scope","col").text("#")}),
                                    html!("th", {.attr("scope","col").text("loginname")}),
                                    html!("th", {.attr("scope","col").text("ชื่อ - นามสกุล")}),
                                    html!("th", {.attr("scope","col").text("Role")}),
                                    html!("th", {.attr("scope","col").text("HOSxP Group")}),
                                    html!("th", {.class("text-nowrap").attr("scope","col").text("ปิดใช้งาน")}),
                                ])
                                .apply_if(app.has_permission(Permission::SystemAcRoleUserEdit), |dom| { dom
                                    .child(html!("th", {.class("text-nowrap").attr("scope","col").text("2FA")}))
                                })
                            }))
                        }),
                        html!("tbody", {
                            .children_signal_vec(page.search_result.signal_vec_cloned().enumerate().map(clone!(app, page => move |(i,row)| {
                                Self::render_result(i.get().unwrap_or_default(), row, page.clone(), app.clone())
                            })))
                        }),
                    ])
                })),
                Self::render_modal(page.clone(), app.clone()),
            ])
        })
    }

    fn render_modal(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("modal")
            .attr("id", "userManageModal")
            .attr("role", "dialog")
            .attr("tabindex", "-1")
            .child(html!("div", {
                .class(class::MODAL_DIALOG_XL_FULL)
                .children([
                    html!("div", {
                        .class("modal-content")
                        .children([
                            html!("div", {
                                .class("modal-header")
                                .children([
                                    html!("h4", {
                                        .class("modal-title")
                                        .text("จัดการผู้ใช้งาน")
                                    }),
                                    doms::close_modal_x_btn(),
                                ])
                            }),
                            html!("div", {
                                .class("modal-body")
                                .children([
                                    html!("div", {
                                        .class("row")
                                        .child(html!("div", {
                                            .class("col")
                                            .children([
                                                html!("label", {
                                                    .attr("for", "modal_loginname")
                                                    .class(class::BOLD)
                                                    .text("Login Name:")
                                                }),
                                                html!("span", {
                                                    .attr("id", "modal_loginname")
                                                    .class("pe-2")
                                                    .text_signal(page.modal_loginname.signal_cloned())
                                                }),
                                                html!("label", {
                                                    .attr("for", "modal_name")
                                                    .class(class::BOLD)
                                                    .text("ชื่อ-นามสกุล:")
                                                }),
                                                html!("span", {
                                                    .attr("id", "modal_name")
                                                    .class("pe-2")
                                                    .text_signal(page.modal_name.signal_cloned())
                                                }),
                                                html!("label", {
                                                    .attr("for", "modal_hosxp_group")
                                                    .class(class::BOLD)
                                                    .text("HOSxP Group:")
                                                }),
                                                html!("span", {
                                                    .attr("id", "modal_hosxp_group")
                                                    .class("pe-2")
                                                    .text_signal(page.modal_hosxp_group.signal_cloned())
                                                }),
                                                html!("span", {
                                                    //.attr("id", "modal_account_disable")
                                                    .child(html!("span", {
                                                        .class("badge")
                                                        .class_signal("text-bg-danger", page.modal_account_disable.signal_cloned().map(|d| d == "Y"))
                                                        .class_signal("text-bg-success", page.modal_account_disable.signal_cloned().map(|d| d != "Y"))
                                                        .text_signal(page.modal_account_disable.signal_cloned().map(|d| if d == "Y" {"ปิดใช้งาน"} else {"เปิดใช้งาน"}))
                                                    }))
                                                }),
                                            ])
                                            .apply_if(app.endpoint_is_allow(&Method::PATCH, &EndPoint::UserConfig, false), |dom| { dom
                                                .child_signal(page.modal_has_totp.signal_cloned().map(clone!(app, page => move |has_totp| {
                                                    has_totp.then(|| {
                                                        html!("button" => HtmlButtonElement, {
                                                            .attr("type","button")
                                                            .class(class::BTN_SM_R_REDO)
                                                            .text("ยกเลิก 2FA")
                                                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                Self::remove_2fa(page.modal_loginname.get_cloned(), page.clone(), app.clone());
                                                                page.modal_has_totp.set(false);
                                                            }), app.state()))
                                                        })
                                                    })
                                                })))
                                            })
                                        }))
                                    }),
                                    html!("hr"),
                                    html!("div", {
                                        .class("row")
                                        .children([
                                            html!("div", {
                                                .class("col-6")
                                                .child(html!("p", {.text("Role")}))
                                            }),
                                            html!("div", {
                                                .class("col-6")
                                                .child(html!("p", {.text("User Permission")}))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class("row")
                                        .children([
                                            html!("div", {
                                                .class("col-6")
                                                .style("height", "calc(100vh - 270px)")
                                                .style("overflow-y","auto")
                                                .children([
                                                    html!("ul", {
                                                        //.attr("id", "role_checkbox")
                                                        .style("list-style","none")
                                                        .children_signal_vec(page.roles.signal_vec_cloned().to_signal_cloned().map(clone!(page => move |roles| {
                                                            let (heads, tails): (Vec<Rc<Role>>,Vec<Rc<Role>>) = roles.into_iter().partition(|role| role.parent_role.is_none());
                                                            heads.iter().map(clone!(page => move |head| {
                                                                html!("li", {
                                                                    .class("form-check")
                                                                    .children(Self::render_li(head, page.clone()))
                                                                    .apply_if(tails.iter().any(|x| x.parent_role.as_ref().map(|r| r == &head.role).unwrap_or_default()), clone!(page, tails => move |dom| {
                                                                        dom.child(html!("ul", {
                                                                            .style("list-style","none")
                                                                            .children(Self::render_li_recursive(head, &tails, page))
                                                                        }))
                                                                    }))
                                                                    // .children(Self::render_li_recursive(head, &tails, page.clone()))
                                                                })
                                                            })).collect::<Vec<Dom>>()
                                                        })).to_signal_vec())
                                                    }),
                                                ])
                                            }),
                                            html!("div", {
                                                .class("col-6")
                                                .style("height", "calc(100vh - 270px)")
                                                .style("overflow-y", "auto")
                                                .child(html!("ul", {
                                                    //.attr("id", "user_permission_body")
                                                    .children_signal_vec(page.modal_user_permissions.entries_cloned().map(|perm| {
                                                        html!("li", {
                                                            .class("mb-2")
                                                            .child(html!("span", {.text(perm.0.str())}))
                                                            .children(perm.1.iter().map(|role_desc| {
                                                                html!("span", {
                                                                    .class(class::BADGE_CYAN_R)
                                                                    .style("cursor","default")
                                                                    .text(role_desc)
                                                                })
                                                            }))
                                                        })
                                                    }))
                                                }))
                                            }),
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class("modal-footer")
                                .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::UserRoleUser, false), |dom| {
                                    dom.child(html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class("btn")
                                        .class_signal("btn-primary", page.modal_changed.signal())
                                        .class_signal("btn-secondary", not(page.modal_changed.signal()))
                                        //.attr("id", "button-save-role")
                                        .attr("data-bs-dismiss", "modal")
                                        .text("บันทึก")
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                            Self::save(page.clone(), app.clone());
                                        }), not(page.modal_changed.signal()), app.state()))
                                        // .attr("onclick", "modal_onclickSaveSelectRole(event)")
                                    }))
                                })
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

    fn render_li_recursive(head: &Rc<Role>, tails: &[Rc<Role>], page: Rc<Self>) -> impl Iterator<Item = Dom> {
        tails.iter().filter_map(clone!(page, tails => move |role| {
            role.parent_role.as_ref().map(|r| r == &head.role).unwrap_or_default().then(|| {
                html!("li", {
                    .class("form-check")
                    .children(Self::render_li(role, page.clone()))
                    .apply_if(tails.iter().any(|x| x.parent_role.as_ref().map(|r| r == &role.role).unwrap_or_default()), clone!(page, tails => move |dom| {
                        dom.child(html!("ul", {
                            .style("list-style","none")
                            .children(Self::render_li_recursive(role, tails, page))
                        }))
                    }))
                })
            })
        }))
    }

    fn render_li(role: &Rc<Role>, page: Rc<Self>) -> Vec<Dom> {
        vec![
            html!("input" => HtmlInputElement, {
                .attr("type", "checkbox")
                .attr("id", &["roles_",&role.role].concat())
                .class("form-check-input")
                .with_node!(element => {
                    .future(page.modal_roles.signal_vec_cloned().to_signal_cloned().for_each(clone!(element, role => move |items| {
                        element.set_checked(items.contains(&role.role));
                        async {}
                    })))
                    .event(clone!(page, element, role => move |_: events::Change| {
                        if element.checked() {
                            page.modal_roles.lock_mut().push_cloned(role.role.clone());
                        } else {
                            page.modal_roles.lock_mut().retain(|x| x != &role.role)
                        }
                        let permissions = page.role_permission();
                        page.modal_user_permissions.lock_mut().replace_cloned(permissions);
                        page.modal_changed.set_neq(true);
                    }))
                })
            }),
            doms::label_check_for(&["roles_", &role.role].concat(), &role.role_desc.clone().unwrap_or_default()),
            html!("span", {
                .class(class::BADGE_CYAN_R)
                .style("cursor","default")
                .apply_if(role.user_count > 0, |dom| dom.text(&[&role.user_count.to_string(), " user"].concat()))
            }),
        ]
    }

    fn render_result(i: usize, row: Rc<UserRole>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let (roles, roles_desc): (Vec<String>, Vec<String>) = row
            .role
            .as_ref()
            .map(|roles| {
                roles
                    .split('|')
                    .filter_map(|concat| {
                        let tuple = concat.split('^').collect::<Vec<&str>>();
                        (tuple.len() == 2).then(|| (tuple[0].to_owned(), tuple[1].to_owned()))
                    })
                    .unzip()
            })
            .unwrap_or_default();

        let roles_dom = roles_desc.iter().map(|role_desc| html!("li", {.text(role_desc)})).collect::<Vec<Dom>>();

        let is_disable = row.account_disable.as_ref().map(|r| r == "Y").unwrap_or_default();

        html!("tr", {
            .style("cursor","pointer")
            .attr("data-bs-toggle", "modal")
            .attr("data-bs-target", "#userManageModal")
            .children([
                html!("td", {.class("text-center").text(&(i + 1).to_string())}),
                html!("td", {.text(&row.loginname.clone())}),
                html!("td", {.text(&row.name.clone().unwrap_or_default())}),
                html!("td", {
                    .child(html!("ul", {
                        .style("margin-bottom","0px")
                        .children(roles_dom)
                    }))
                }),
                html!("td", {.text(&row.hosxp_group.clone().unwrap_or_default())}),
                html!("td", {
                    .apply_if(is_disable, |dom| {
                        dom.child(html!("span", {
                            .class(class::BADGE_RED)
                            .style("cursor","default")
                            .text("ปิดใช้งาน")
                        }))
                    })
                }),
            ])
            .apply_if(app.has_permission(Permission::SystemAcRoleUserEdit), |dom| { dom
                .child(html!("td", {
                    .class(class::TXT_C_P1)
                    .apply_if(row.has_totp, |d| { d
                        .child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).style("font-size","30px")}))
                    })
                }))
            })
            .event(move |_:events::Click| {
                page.modal_loginname.set_neq(row.loginname.clone());
                page.modal_name.set_neq(row.name.clone().unwrap_or_default());
                page.modal_hosxp_group.set_neq(row.hosxp_group.clone().unwrap_or_default());
                page.modal_account_disable.set_neq(row.account_disable.clone().unwrap_or_default());
                {
                    let mut lock = page.modal_roles.lock_mut();
                    lock.replace_cloned(roles.clone());
                }
                let permissions = page.role_permission();
                page.modal_user_permissions.lock_mut().replace_cloned(permissions);
                page.modal_has_totp.set_neq(row.has_totp);
                page.modal_changed.set_neq(false);
            })
        })
    }
}

pub fn fill_parent_recursive(text: &str, roles: &[Rc<Role>], set: &mut HashSet<String>) {
    if let Some(selected_role) = roles.iter().find(|r| r.role == text) {
        if let Some(parent) = &selected_role.parent_role {
            fill_parent_recursive(parent, roles, set);
            set.insert(parent.to_owned());
        }
    }
}
