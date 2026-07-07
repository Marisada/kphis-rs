use crate::TABLE_CREATE_COLUMNS;

// SELECT sa.*,u.`name` AS create_username
// FROM kphis_extra.ipd_summary_audit AS sa
//     LEFT JOIN hos.opduser u ON u.loginname=sa.create_user
// WHERE sa.an=? ORDER BY sa.summary_audit_id;
/// summary_id | an
pub fn select_summary_audit_by(by_id: bool, hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    let by = if by_id { "s.summary_id=?" } else { "s2.an=?" };
    [
        "SELECT s.*,u.`name` AS create_username \
        FROM ",kphis_extra,".ipd_summary_audit AS s \
            LEFT JOIN ",kphis,".ipd_summary_2 s2 ON s2.summary_id=s.summary_id \
            LEFT JOIN ",hosxp,".opduser u ON u.loginname=s.create_user \
        WHERE ",by," ORDER BY s.summary_audit_id;"
    ].concat()
}

// SELECT * FROM kphis_extra.ipd_summary_audit_item WHERE summary_audit_id=?;
/// summary_audit_id
pub fn select_summary_audit_item(kphis_extra: &str) -> String {
    [
        "SELECT * FROM ",kphis_extra,".ipd_summary_audit_item WHERE summary_audit_id=?;"
    ].concat()
}

// INSERT INTO kphis_extra.ipd_summary_audit_item (
//     summary_audit_id,summary_id,ty,sum_dx,sum_icd,com_icd,rev_dx,rev_icd,sa,ca,remark,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// summary_audit_id, summary_id, ty, sum_dx, sum_icd, com_icd, rev_dx, rev_icd, sa, ca, remark, loginname, loginname
pub fn insert_summary_audit_items(count: usize, kphis_extra: &str) -> String {
    let values = vec!["(?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1)"; count].join(",");
    [
        "INSERT INTO ",kphis_extra,".ipd_summary_audit_item (\
            summary_audit_id,summary_id,ty,sum_dx,sum_icd,com_icd,rev_dx,rev_icd,sa,ca,remark",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values
    ].concat()
}

// DELETE FROM kphis_extra.ipd_summary_audit_item WHERE summary_audit_id=?;
/// summary_audit_id
pub fn delete_summary_audit_items(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_summary_audit_item WHERE summary_audit_id=?;"
    ].concat()
}

// DELETE kphis_extra.ipd_summary_audit, kphis_extra.ipd_summary_audit_item
// FROM kphis_extra.ipd_summary_audit
//     LEFT JOIN kphis_extra.ipd_summary_audit_item ON kphis_extra.ipd_summary_audit.summary_audit_id = kphis_extra.ipd_summary_audit_item.summary_audit_id
// WHERE kphis_extra.ipd_summary_audit.summary_audit_id=? AND kphis_extra.ipd_summary_audit.create_user=?;
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// summary_audit_id, loginname
pub fn delete_summary_audit(kphis_extra: &str) -> String {
    [
        "DELETE ",kphis_extra,".ipd_summary_audit, ",kphis_extra,".ipd_summary_audit_item \
        FROM ",kphis_extra,".ipd_summary_audit \
            LEFT JOIN ",kphis_extra,".ipd_summary_audit_item ON ",kphis_extra,".ipd_summary_audit.summary_audit_id = ",kphis_extra,".ipd_summary_audit_item.summary_audit_id \
        WHERE ",kphis_extra,".ipd_summary_audit.summary_audit_id=? AND ",kphis_extra,".ipd_summary_audit.create_user=?;"
    ].concat()
}