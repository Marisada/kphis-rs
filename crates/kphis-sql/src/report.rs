use kphis_model::report::ReportTemplateParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT rt.template_id,rt.template_name,,rt.title,rt.content,rt.statement,rt.statement_params,rt.info,rt.disabled,u.`name` AS update_username,rt.update_datetime
// FROM kphis_extra.report_template rt
//     LEFT JOIN hos.opduser u ON u.loginname=rt.update_user
// WHERE rt.template_id=? AND rt.template_name=? AND (rt.disabled IS NULL OR rt.disabled=0);
/// (template_id),(template_name)
pub fn get_report_template(params: &ReportTemplateParams, hosxp: &str, kphis_extra: &str) -> String {
    let template_id = if params.template_id.is_some() {" AND rt.template_id=?"} else {""};
    let template_name = if params.template_name.is_some() {" AND rt.template_name=?"} else {""};
    let disabled = match params.disabled {
        Some(true) => " AND rt.disabled=1",
        Some(false) => " AND (rt.disabled IS NULL OR rt.disabled=0)",
        None => "",
    };
    ["SELECT rt.template_id,rt.template_name,rt.title,rt.content,rt.statement,rt.statement_params,rt.info,rt.disabled,u.`name` AS update_username,rt.update_datetime \
        FROM ",kphis_extra,".report_template rt \
            LEFT JOIN ",hosxp,".opduser u ON u.loginname=rt.update_user \
        WHERE 1=1 ",template_id,template_name,disabled," ORDER BY rt.template_name;"].concat()
}

// SELECT template_id,template_name,title,disabled FROM kphis_extra.report_template WHERE template_id=? AND template_name=? AND (disabled IS NULL OR disabled=0);
/// (template_id),(template_name)
pub fn get_report_template_compact(params: &ReportTemplateParams, kphis_extra: &str) -> String {
    let template_id = if params.template_id.is_some() {" AND template_id=?"} else {""};
    let template_name = if params.template_name.is_some() {" AND template_name=?"} else {""};
    let disabled = match params.disabled {
        Some(true) => " AND disabled=1",
        Some(false) => " AND (disabled IS NULL OR disabled=0)",
        None => "",
    };
    ["SELECT template_id,template_name,title,disabled FROM ",kphis_extra,".report_template WHERE 1=1 ",template_id,template_name,disabled," ORDER BY template_name;"].concat()
}

// SELECT content FROM kphis_extra.report_template WHERE template_name=? AND (disabled IS NULL OR disabled=0);
/// template_name
pub fn get_report_template_content(kphis_extra: &str) -> String {
    ["SELECT content FROM ",kphis_extra,".report_template WHERE template_name=? AND (disabled IS NULL OR disabled=0);"].concat()
}

// SELECT EXISTS(SELECT * FROM kphis_extra.report_template WHERE template_name=?) AS exs;
/// template_name
pub fn get_report_template_exists(kphis_extra: &str) -> String {
    ["SELECT EXISTS(SELECT * FROM ",kphis_extra,".report_template WHERE template_name=?) AS exs;"].concat()
}

// INSERT INTO kphis_extra.report_template (template_name,title,content,statement,statement_params,info,disabled,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// template_name,title,content,statement,statement_params,info,disabled,loginname,loginname
pub fn insert_report_template(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".report_template (template_name,title,content,statement,statement_params,info,disabled",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis_extra.report_template SET template_name=?,title=?,content=?,statement=?,statement_params=?,info=?,disabled=?,update_user=?,update_datetime=NOW(),version=1 WHERE template_id=?;
/// template_name,title,content,statement,statement_params,info,disabled,loginname,template_id
pub fn update_report_template(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".report_template SET template_name=?,title=?,content=?,statement=?,statement_params=?,info=?,disabled=?",TABLE_UPDATE_SET,
        " WHERE template_id=?;"
    ].concat()
}

// DELETE FROM kphis_extra.report_template WHERE template_id=?;
/// template_id
pub fn delete_report_template(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".report_template WHERE template_id=?;"
    ].concat()
}