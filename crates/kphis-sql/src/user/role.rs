use kphis_model::user::role::UserRoleParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT role,role_desc,parent_role FROM kphis.system_ac_role ORDER BY role_desc;
pub fn select_roles_zero_count(kphis: &str) -> String {
    [
        "SELECT role,role_desc,parent_role,0 AS user_count \
        FROM ",kphis,".system_ac_role ORDER BY role_desc;"
    ].concat()
}

// SELECT r.role,r.role_desc,r.parent_role,pr.role_desc AS parent_role_desc,
//     (SELECT GROUP_CONCAT(rp.permission) FROM kphis.system_ac_role_permission rp WHERE rp.role=r.role) AS permissions
// FROM kphis.system_ac_role r
//     LEFT JOIN kphis.system_ac_role pr ON pr.role=r.parent_role
// WHERE r.role=? OR r.parent_role=?
// GROUP BY r.role ORDER BY role_desc;
/// (permission),([role..])
pub fn select_roles_permissions(params: &UserRoleParams, kphis: &str) -> String {
    let permission = if params.permission.is_some() {" AND rp.permission=?"} else {""};
    let role = params.role.as_ref().map(|r| {
        let len = r.split(',').collect::<Vec<&str>>().len();
        [" WHERE r.role IN (", &vec!["?"; len].join(","), ") "].concat()
    }).unwrap_or_default();

    [
        "SELECT r.role,r.role_desc,r.parent_role,pr.role_desc AS parent_role_desc,\
            (SELECT GROUP_CONCAT(rp.permission) FROM ",kphis,".system_ac_role_permission rp WHERE rp.role=r.role",permission,") AS permissions \
        FROM ",kphis,".system_ac_role r \
            LEFT JOIN ",kphis,".system_ac_role pr ON pr.role=r.parent_role ",&role,
        "GROUP BY r.role ORDER BY role_desc;"
    ].concat()
}

// SELECT permission FROM kphis.system_ac_permission
// UNION
// SELECT permission FROM kphis.system_ac_role_permission;
pub fn select_all_permissions(kphis: &str) -> String {
    [
        "SELECT permission FROM ",kphis,".system_ac_permission \
        UNION \
        SELECT permission FROM ",kphis,".system_ac_role_permission;"
    ].concat()
}

// SELECT opduser.loginname,GROUP_CONCAT(r.role_desc SEPARATOR '] [') AS role,opduser.name,opduser.groupname AS hosxp_group,opduser.account_disable,IF(c.totp IS NULL,0,1) AS has_totp
// FROM hos.opduser
//     LEFT JOIN kphis.system_ac_role_user ru ON ru.loginname=opduser.loginname
//     LEFT JOIN kphis.system_ac_role r ON ru.role=r.role
//     LEFT JOIN kphis_extra.user_config c ON c.loginname=opduser.loginname
// WHERE 1=1
// GROUP BY opduser.loginname ORDER BY opduser.loginname,ru.role;
/// (%loginname%), (%name%), (role), (hosxp_group)
pub fn select_users_role_list(params: &UserRoleParams, hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    let loginname = if params.loginname.is_some() {" AND opduser.loginname LIKE ? "} else {""};
    let name = if params.name.is_some() {" AND opduser.name LIKE ? "} else {""};
    let role = if params.role.is_some() {" AND ru.role=? "} else {""};
    let hosxp_group = if params.hosxp_group.is_some() {" AND opduser.groupname=? "} else {""};
    let account_disable = params.account_disable.as_ref().map(|s| {
        if s == "Y" {
            " AND opduser.account_disable='Y' "
        } else {
            " AND (opduser.account_disable IS NULL OR opduser.account_disable <> 'Y') "
        }
    }).unwrap_or_default();

    [
        "SELECT opduser.loginname,GROUP_CONCAT(CONCAT(r.role,'^',r.role_desc) ORDER BY r.role SEPARATOR '|') AS role,opduser.name,opduser.groupname AS hosxp_group,opduser.account_disable,IF(c.totp IS NULL,0,1) AS has_totp \
        FROM ",hosxp,".opduser \
            LEFT JOIN ",kphis,".system_ac_role_user ru ON ru.loginname=opduser.loginname \
            LEFT JOIN ",kphis,".system_ac_role r ON ru.role=r.role \
            LEFT JOIN ",kphis_extra,".user_config c ON c.loginname=opduser.loginname \
        WHERE 1=1 ",loginname,name,role,hosxp_group,account_disable,
        "GROUP BY opduser.loginname ORDER BY opduser.loginname,ru.role;"
    ].concat()
}

// SELECT opdgroup.groupname AS hosxp_group FROM hos.opdgroup ORDER BY hosxp_group;
pub fn select_hosxp_groups(hosxp: &str) -> String {
    [
        "SELECT opdgroup.groupname AS hosxp_group FROM ",hosxp,".opdgroup ORDER BY hosxp_group;"
    ].concat()
}

// SELECT r.role,r.role_desc,r.parent_role,
// 	(SELECT COUNT(*) FROM kphis.system_ac_role_user ru WHERE ru.role=r.role) AS user_count
// FROM kphis.system_ac_role r
// ORDER BY role_desc;
pub fn select_roles_with_count(kphis: &str) -> String {
    [
        "SELECT r.role,r.role_desc,r.parent_role,\
            (SELECT COUNT(*) FROM ",kphis,".system_ac_role_user ru WHERE ru.role=r.role) AS user_count \
        FROM ",kphis,".system_ac_role r ORDER BY role_desc;"
    ].concat()
}

// // system-ac-role-array-permission-list-data.php
// SELECT r.role,r.role_desc,rp.permission
// FROM kphis.system_ac_role r
// JOIN kphis.system_ac_role_permission rp ON r.role=rp.role
// JOIN kphis.system_ac_permission p ON p.permission=rp.permission
// WHERE r.role IN (
//     SELECT r1.role
//     FROM kphis.system_ac_role r1
//     WHERE r1.role IN $role_sql
//     UNION
//     SELECT r2.role
//     FROM kphis.system_ac_role r1
//     JOIN kphis.system_ac_role r2 ON r1.parent_role = r2.role
//     WHERE r1.role IN $role_sql
//     UNION
//     SELECT r3.role
//     FROM kphis.system_ac_role r1
//     JOIN kphis.system_ac_role r2 ON r1.parent_role = r2.role
//     JOIN kphis.system_ac_role r3 ON r2.parent_role = r3.role
//     WHERE r1.role IN $role_sql
//     UNION
//     SELECT r4.role
//     FROM kphis.system_ac_role r1
//     JOIN kphis.system_ac_role r2 ON r1.parent_role = r2.role
//     JOIN kphis.system_ac_role r3 ON r2.parent_role = r3.role
//     JOIN kphis.system_ac_role r4 ON r3.parent_role = r4.role
//     WHERE r1.role IN $role_sql
//     UNION
//     SELECT r5.role
//     FROM kphis.system_ac_role r1
//     JOIN kphis.system_ac_role r2 ON r1.parent_role = r2.role
//     JOIN kphis.system_ac_role r3 ON r2.parent_role = r3.role
//     JOIN kphis.system_ac_role r4 ON r3.parent_role = r4.role
//     JOIN kphis.system_ac_role r5 ON r4.parent_role = r5.role
//     WHERE r1.role IN $role_sql
// )
// ORDER BY rp.permission,r.role_desc"
// // we create recursive roles in client
// SELECT rp.permission, GROUP_CONCAT(r.role_desc ORDER BY r.role) AS role_descs
// FROM kphis.system_ac_role r
// JOIN kphis.system_ac_role_permission rp ON r.role=rp.role
// WHERE r.role IN ('DOCTOR','NURSE_ER_RN_EMT','MSO')
// GROUP BY rp.permission
// ORDER BY rp.permission;
// pub fn select_role_permission(roles: &[String], kphis: &str) -> String {
//     [
//         "SELECT rp.permission, GROUP_CONCAT(r.role_desc ORDER BY r.role) AS role_descs \
//         FROM ",kphis,".system_ac_role r \
//             JOIN ",kphis,".system_ac_role_permission rp ON r.role=rp.role \
//         WHERE r.role IN ('",roles.join("','"),"') \
//         GROUP BY rp.permission ORDER BY rp.permission;"
//     )
// }

// SELECT role,permission FROM kphis.system_ac_role_permission ORDER BY role,permission;
pub fn select_role_permission(kphis: &str) -> String {
    [
        "SELECT role,permission FROM ",kphis,".system_ac_role_permission ORDER BY permission,role;"
    ].concat()
}

// // system-ac-role-user-save.php
// DELETE FROM kphis.system_ac_role_user WHERE loginname=?;
/// loginname
pub fn delete_role_user_by_loginname(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".system_ac_role_user WHERE loginname=?;"
    ].concat()
}

// DELETE FROM kphis.system_ac_role_user WHERE role=?;
/// role
pub fn delete_role_user_by_role(kphis: &str) -> String {
    [
        "DELETE FROM ", kphis, ".system_ac_role_user WHERE role=?;"
    ].concat()
}

// INSERT INTO kphis.system_ac_role_user(loginname,role,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,?,NOW(),?,NOW(),0)
/// [loginname, role, user, user]
pub fn insert_roles_user(roles_len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,NOW(),?,NOW(),0)"; roles_len].join(",");
    [
        "INSERT INTO ",kphis,".system_ac_role_user (loginname,role",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// SELECT permission FROM kphis.system_ac_permission ORDER BY permission;
pub fn select_permission(kphis: &str) -> String {
    [
        "SELECT permission FROM ",kphis,".system_ac_permission ORDER BY permission;"
    ].concat()
}

// SELECT r.role,r.role_desc,r.parent_role,pr.role_desc AS parent_role_desc,
//     (SELECT GROUP_CONCAT(rp.permission) FROM kphis.system_ac_role_permission rp WHERE rp.role=r.role) AS permissions
// FROM kphis.system_ac_role r
//     LEFT JOIN kphis.system_ac_role pr ON pr.role=r.parent_role
// WHERE r.role IN ()
// GROUP BY r.role ORDER BY role_desc;
// /// (role..)
// pub fn select_parents_permissions(parent_len: usize, kphis: &str) -> String {
//     let params = vec!["?";parent_len].join(",");
//     [
//         "SELECT r.role,r.role_desc,r.parent_role,pr.role_desc AS parent_role_desc,\
//             (SELECT GROUP_CONCAT(rp.permission) FROM ",kphis,".system_ac_role_permission rp WHERE rp.role=r.role) AS permissions \
//         FROM ",kphis,".system_ac_role r \
//             LEFT JOIN ",kphis,".system_ac_role pr ON pr.role=r.parent_role \
//         WHERE r.role IN (",params,") \
//         GROUP BY r.role ORDER BY role_desc;"
//     )
// }

// // system-ac-role-permission-save.php
// // SUMMMARY
// [NOT has role_prv] -> INSERT INTO system_ac_role
//  ==> fn insert_role_new()
// [has role_prv]
// x [system_ac_role has role=role_prv] x=> error 403
// - -> [old role.role!=new role]
// - - x [system_ac_role NOT has role=role] x=> error 403
// - - - - INSERT INTO system_ac_role with old createuser/datetime
//          ==> fn insert_role_from_prev()
// - - - - UPDATE system_ac_role parent_role=role+updateuser/datetime+(version+1) where parent_role=role_prv
//          ==> fn update_parent_role()
// - - - - DELETE system_ac_role WHERE role=role_prv
//          ==> fn delete_role()
// - -> [old role.role==new role]
// - - - UPDATE system_ac_role role_desc,parent_role,updateuser/datetime,(version+1) where role=role_prv
//          ==> fn update_role()
// DELETE system_ac_role_permission where role=role AND permssion NOT in (permissions)
//  ==> fn delete_role_keep_permission()
// INSERT INTO system_ac_role_permission for each permission
//  ==> fn insert_role_permissions()

// INSERT INTO kphis.system_ac_role (role,role_desc,parent_role,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// role,role_desc,parent_role,loginname,loginname
pub fn insert_role_new(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".system_ac_role (role,role_desc,parent_role",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// INSERT INTO kphis.system_ac_role (role,role_desc,parent_role,create_user,create_datetime,update_user,update_datetime,version)
// SELECT ?,?,?,create_user,create_datetime,?,NOW(),(version+1)
// FROM kphis.system_ac_role WHERE role=?;
/// role,role_desc,parent_role,loginname,role_prev
pub fn insert_role_from_prev(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".system_ac_role (role,role_desc,parent_role",TABLE_CREATE_COLUMNS,") \
            SELECT ?,?,?,create_user,create_datetime,?,NOW(),(version+1) \
            FROM ",kphis,".system_ac_role WHERE role=?;"
    ].concat()
}

// UPDATE kphis.system_ac_role SET parent_role=?,update_user=?,update_datetime=NOW(),version=version+1
// WHERE parent_role=?;
/// new_parent_role, loginname, old_parent_role
pub fn update_parent_role(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".system_ac_role SET parent_role=?",TABLE_UPDATE_SET,
        " WHERE parent_role=?;"
    ].concat()
}

// DELETE FROM kphis.system_ac_role WHERE role=?;
/// role
pub fn delete_role(kphis: &str) -> String {
    [
        "DELETE FROM ", kphis, ".system_ac_role WHERE role=?;"
    ].concat()
}

// UPDATE kphis.system_ac_role SET role_desc=?,parent_role=?,update_user=?,update_datetime=NOW(),version=version+1
// WHERE role=?;
/// role_desc, parent_role, loginname, role_prev
pub fn update_role(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".system_ac_role SET role_desc=?,parent_role=?",TABLE_UPDATE_SET,
        " WHERE role=?;"
    ].concat()
}

// DELETE FROM kphis.system_ac_role_permission WHERE role=?;
/// role, (role_prev), ([permissions..])
pub fn delete_role_keep_permission(
    has_prev: bool,
    permissions_keep_len: usize,
    kphis: &str,
) -> String {
    let role_prev = if has_prev { " OR role=?" } else { "" };
    let permissions = vec!["?"; permissions_keep_len].join(",");
    let permission = if permissions_keep_len > 0 {
        [" AND permission NOT IN (", &permissions, ")"].concat()
    } else {
        String::new()
    };
    [
        "DELETE FROM ",kphis,".system_ac_role_permission WHERE role=?",role_prev,&permission,";"
    ].concat()
}

// INSERT INTO kphis.system_ac_role_permission(role,permission,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,NOW(),?,NOW(),0)
/// ([role, permission, loginname, loginname])
pub fn insert_role_permissions(permissions_len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,NOW(),?,NOW(),0)"; permissions_len].join(",");
    [
        "INSERT INTO ",kphis,".system_ac_role_permission(role,permission",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}
