use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch_vec, fetch_json_api},
    user::permission::Permission,
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct UserRoleParams {
    pub loginname: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
    pub parent_role: Option<String>,
    pub permission: Option<String>,
    pub hosxp_group: Option<String>,
    /// if not 'Y' means 'N' or NULL
    pub account_disable: Option<String>,
}

impl QueryString for UserRoleParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            loginname: find_qs(params, "loginname"),
            name: find_qs(params, "name"),
            role: find_qs(params, "role"),
            parent_role: find_qs(params, "parent_role"),
            permission: find_qs(params, "permission"),
            hosxp_group: find_qs(params, "hosxp_group"),
            account_disable: find_qs(params, "account_disable"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(7);
        if let Some(loginname) = &self.loginname {
            queries.push(["loginname=", loginname].concat());
        }
        if let Some(name) = &self.name {
            queries.push(["name=", name].concat());
        }
        if let Some(role) = &self.role {
            queries.push(["role=", role].concat());
        }
        if let Some(parent_role) = &self.parent_role {
            queries.push(["parent_role=", parent_role].concat());
        }
        if let Some(permission) = &self.permission {
            queries.push(["permission=", permission].concat());
        }
        if let Some(hosxp_group) = &self.hosxp_group {
            queries.push(["hosxp_group=", hosxp_group].concat());
        }
        if let Some(account_disable) = &self.account_disable {
            queries.push(["account_disable=", account_disable].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// User Role
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema, FromRow)]
#[schema(example = json!(UserRole::demo()))]
pub struct UserRole {
    #[Demo(value = r#"String::from("user")"#)]
    pub loginname: String,
    #[Demo(value = r#"Some(String::from("DOCTOR_STAFF^แพทย์ STAFF|AUDITOR^Auditor"))"#)]
    pub role: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub name: Option<String>,
    #[Demo(value = r#"Some(String::from("แพทย์"))"#)]
    pub hosxp_group: Option<String>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub account_disable: Option<String>,
    #[Demo(value = "true")]
    pub has_totp: bool,
}

/// List of User Roles
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(UserRoleList::demo()))]
pub struct UserRoleList {
    #[Demo(value = "vec![UserRole::demo()]")]
    pub user_roles: Vec<UserRole>,
    #[Demo(value = "vec![Role::demo()]")]
    pub roles: Vec<Role>,
}

impl UserRoleList {
    /// GET `EndPoint::UserRoleUser`
    pub async fn call_api_get(params: &UserRoleParams, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[EndPoint::UserRoleUser.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch UserRoleList"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch UserRoleList"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// User Role for save
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(UserRoleSave::demo()))]
pub struct UserRoleSave {
    #[Demo(value = r#"String::from("user")"#)]
    pub loginname: String,
    #[Demo(value = r#"vec![String::from("DOCTOR_STAFF"), String::from("AUDITOR")]"#)]
    pub roles: Vec<String>,
}

impl UserRoleSave {
    /// POST `EndPoint::UserRoleUser`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send UserRoleSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send UserRoleSave"))?;

        execute_fetch_vec(&EndPoint::UserRoleUser.base(), "POST", Some(&body), app).await
    }
}

/// Role
#[derive(Clone, Demo, FromRow, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(Role::demo()))]
pub struct Role {
    #[Demo(value = r#"String::from("DOCTOR_STAFF")"#)]
    pub role: String,
    #[Demo(value = r#"Some(String::from("แพทย์ STAFF"))"#)]
    pub role_desc: Option<String>,
    #[Demo(value = r#"Some(String::from("DOCTOR"))"#)]
    pub parent_role: Option<String>,
    #[Demo(value = "1")]
    pub user_count: i64,
}

/// Role-Permission
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(RolePermission::demo()))]
pub struct RolePermission {
    #[Demo(value = r#"String::from("LAB")"#)]
    pub role: String,
    #[Demo(value = "Permission::demo_lab_view()")]
    pub permission: Permission,
}

/// User Role start data
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(UserRoleOptions::demo()))]
pub struct UserRoleOptions {
    #[Demo(value = "vec![Role::demo()]")]
    pub roles: Vec<Role>,
    #[Demo(value = r#"vec![String::from("แพทย์")]"#)]
    pub hosxp_groups: Vec<String>,
    #[Demo(value = "vec![RolePermission::demo()]")]
    pub role_permissions: Vec<RolePermission>,
    #[Demo(value = "vec![Permission::demo_lab_view()]")]
    pub permissions: Vec<Permission>,
}

impl UserRoleOptions {
    /// GET `EndPoint::UserRolePrelude`
    pub async fn call_api_get(app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&EndPoint::UserRolePrelude.base(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch UserRoleOptions"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch UserRoleOptions"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Role-Permission for List
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(RolePermissionList::demo()))]
pub struct RolePermissionList {
    #[Demo(value = r#"String::from("DOCTOR_STAFF")"#)]
    pub role: String,
    #[Demo(value = r#"Some(String::from("แพทย์ STAFF"))"#)]
    pub role_desc: Option<String>,
    #[Demo(value = r#"Some(String::from("DOCTOR"))"#)]
    pub parent_role: Option<String>,
    #[Demo(value = r#"Some(String::from("แพทย์"))"#)]
    pub parent_role_desc: Option<String>,
    #[Demo(value = "Some(vec![Permission::demo_lab_view()])")]
    pub permissions: Option<Vec<Permission>>,
}

impl RolePermissionList {
    /// GET `EndPoint::UserRoleRole`
    pub async fn call_api_get(params: &UserRoleParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::UserRoleRole.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch RolePermissionList"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch RolePermissionList"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Role-Permission for save
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(RolePermissionSave::demo()))]
pub struct RolePermissionSave {
    #[Demo(value = r#"Some(String::from("DOCTOR_STAF"))"#)]
    pub role_prev: Option<String>,
    #[Demo(value = r#"String::from("LAB")"#)]
    pub role: String,
    #[Demo(value = r#"String::from("Lab")"#)]
    pub role_desc: String,
    #[Demo(value = r#"Some(String::from("OTHER"))"#)]
    pub parent_role: Option<String>,
    #[Demo(value = "vec![Permission::demo_lab_view()]")]
    pub permissions: Vec<Permission>,
}

impl RolePermissionSave {
    /// POST `EndPoint::UserRoleRole`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send RolePermissionSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send RolePermissionSave"))?;

        execute_fetch_vec(&EndPoint::UserRoleRole.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::UserRoleRole`<br>
    /// role=role to delete, parent_role=new parent_role for children role (can be None)
    pub async fn call_api_delete(params: &UserRoleParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(&[EndPoint::UserRoleRole.base(), params.clone().query_string()].concat(), "DELETE", None, app).await
    }
}
