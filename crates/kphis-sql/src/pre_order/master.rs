use kphis_model::pre_order::master::PreOrderMasterParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // ipd-dr-pre-order-check.php
// SELECT COUNT(*) AS pre_order_count FROM kphis.ipd_pre_order_master WHERE hn=? AND (used IS NULL OR used <> 'Y');
/// hn
pub fn select_pre_order_exists(kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".ipd_pre_order_master WHERE hn=? AND (used IS NULL OR used <> 'Y')) AS exs;"
    ].concat()
}

// // ipd-dr-pre-order-list-data.php
// SELECT pom.pre_order_master_id,pom.hn,pom.used,pom.order_date,pom.order_time,
//     CONCAT(pom.order_date,' ',pom.order_time) AS order_date_time,
//     pom.order_for_date,pom.order_for_time,
//     CONCAT(pom.order_for_date,' ',pom.order_for_time) AS order_for_date_time,
//     pom.template_name, pom.shared_template,pom.order_doctor, d.`name` AS order_doctor_name, pom.pre_order_type,
//     CONCAT(p.pname,p.fname,' ',p.lname) AS fullname
// FROM kphis.ipd_pre_order_master pom
//     LEFT JOIN hos.patient p ON p.hn = pom.hn
//     LEFT JOIN hos.doctor d ON d.code = pom.order_doctor
// WHERE 1=1
/// (pre_order_master_id),(hn),(start_order_date),(end_order_date),(order_doctor),(?pre_order_type),(% + template_name + %),(used)
pub fn select_pre_order_list(params: &PreOrderMasterParams, intern_roles: &[String], hosxp: &str, kphis: &str) -> String {
    let pre_order_master_id = if params.pre_order_master_id.is_some() {" AND pom.pre_order_master_id=? "} else {""};
    let hn = if params.hn.is_some() {" AND pom.hn=? "} else {""};
    let start_order_date = if params.start_order_date.is_some() {" AND pom.order_for_date>=? "} else {""};
    let end_order_date = if params.end_order_date.is_some() {" AND pom.order_for_date<=? "} else {""};
    let order_doctor = if params.order_doctor.is_some() {
        let prefix = " AND (pom.order_doctor=? ";
        let suffix = match &params.include_shared_template {
            Some(text) => if text == "Y" {"OR pom.shared_template='Y'"} else {""},
            None => "",
        };
        [prefix, suffix, ")"].concat()
    } else {
        String::new()
    };
    let pre_order_type = match &params.pre_order_type {
        Some(text) => {
            if text == "pre_order" {
                " AND pom.pre_order_type in ('appointment','opd') "
            } else {
                " AND pom.pre_order_type=? "
            }
        }
        None => "",
    };
    let template_name = if params.template_name.is_some() {" AND pom.template_name like ? "} else {""};
    let used = if params.used.is_some() {" AND pom.used=? "} else {""};
    let limit = if params.start_order_date.is_none() && params.end_order_date.is_none() {" LIMIT 200;"} else {";"};

    [
        "SELECT pom.pre_order_master_id,pom.hn,pom.used,pom.order_date,pom.order_time,ADDTIME(CONVERT(pom.order_date,DATETIME),pom.order_time) AS order_date_time,\
            pom.order_for_date,pom.order_for_time,ADDTIME(CONVERT(pom.order_for_date,DATETIME),pom.order_for_time) AS order_for_date_time,\
            pom.template_name, pom.shared_template,pom.order_doctor, d.`name` AS order_doctor_name, pom.pre_order_type, CONCAT(p.pname,p.fname,' ',p.lname) AS fullname,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
                WHERE ou.doctorcode=pom.order_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern \
        FROM ",kphis,".ipd_pre_order_master pom \
            LEFT JOIN ",hosxp,".patient p ON p.hn = pom.hn \
            LEFT JOIN ",hosxp,".doctor d ON d.code = pom.order_doctor \
        WHERE 1=1 ",pre_order_master_id,hn,start_order_date,end_order_date,&order_doctor,pre_order_type,template_name,used," ORDER BY pom.order_date DESC, pom.order_time DESC ",limit
    ].concat()
}

// // ipd-dr-pre-order-master-delete.php
// SELECT used FROM kphis.ipd_pre_order_master WHERE pre_order_master_id=?;
/// pre_order_master_id
pub fn select_pre_order_used_by_master_id(kphis: &str) -> String {
    [
        "SELECT used FROM ",kphis,".ipd_pre_order_master WHERE pre_order_master_id=?;"
    ].concat()
}

// // ipd-dr-pre-order-master-save.php
// INSERT INTO kphis.ipd_pre_order_master (hn,order_date,order_time,order_for_date,order_for_time,order_doctor,pre_order_type,template_name,shared_template,used,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,NOW(),NOW(),?,?,?,?,?,?,'N',?,NOW(),?,NOW(),1);
/// hn,order_for_date,order_for_time,order_doctor,pre_order_type,template_name,shared_template,loginuser,loginuser
pub fn insert_pre_order_master(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_pre_order_master (hn,order_date,order_time,order_for_date,order_for_time,order_doctor,pre_order_type,template_name,shared_template,used",TABLE_CREATE_COLUMNS,") \
        VALUES (?,NOW(),NOW(),?,?,?,?,?,?,'N'",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis.ipd_pre_order_master SET hn=?,order_for_date=?,order_for_time=?,template_name=?,shared_template=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE pre_order_master_id=? AND used != 'Y';
/// hn,order_for_date,order_for_time,template_name,shared_template,loginname,pre_order_master_id
pub fn update_pre_order_master(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_pre_order_master SET hn=?,order_for_date=?,order_for_time=?,template_name=?,shared_template=?",TABLE_UPDATE_SET,
        " WHERE pre_order_master_id=? AND used != 'Y';"
    ].concat()
}

// UPDATE kphis.ipd_pre_order_master SET used='Y',update_user=?, update_datetime=NOW(), version=version+1 WHERE pre_order_master_id=?;
/// loginname, pre_order_master_id
pub fn update_pre_order_master_used(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_pre_order_master SET used='Y'",TABLE_UPDATE_SET," WHERE pre_order_master_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_pre_order_master WHERE pre_order_master_id=?;
/// pre_order_master_id
pub fn delete_pre_order_master_by_master_id(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_pre_order_master WHERE pre_order_master_id=?;"
    ].concat()
}
