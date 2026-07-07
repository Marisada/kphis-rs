use kphis_model::focus_list::{FocusListGoalItemOnly, FocusListParams};

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // ipd-nurse-focus-list-update.php
// SELECT COUNT(*) AS note_count FROM kphis.ipd_focus_note WHERE fclist_id=?;
/// fclist_id
pub fn select_note_exists(kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".ipd_focus_note WHERE fclist_id=?) AS exs;"
    ].concat()
}

// // ipd-nurse-focus-list-table.php
// SELECT fc_l.*,fc_l.create_user AS create_user_fclist,t_fc.focus_name,dchtype.`name` AS dchtype_name,dchtype.dchtype,ipt.dchdate AS dchdate
//     FROM kphis.ipd_focus_list fc_l
//     LEFT JOIN kphis.ipd_tmp_focus t_fc ON t_fc.focus_id = fc_l.focus_id
//     LEFT JOIN hos.ipt ipt ON ipt.an = fc_l.an
//     LEFT JOIN hos.dchtype dchtype ON dchtype.dchtype=ipt.dchtype
//     WHERE fc_l.an = :an
//         AND fc_l.fclist_stdate = :search_startdate
//         AND fc_l.fclist_stdate = :search_enddate
//         AND fc_l.fclist_stdate BETWEEN :search_startdate AND :search_enddate
//         AND fc_l.fclist_status = :search_status
//     ORDER BY fc_l.fclist_stdate, fc_l.fclist_sttime ASC
// SELECT item_g.goal_id,tmp_g.goal_name
//     FROM kphis.ipd_focus_list_goal_item item_g
//         LEFT JOIN kphis.ipd_tmp_goal tmp_g ON tmp_g.goal_id = item_g.goal_id
//     WHERE item_g.fclist_id =:fclist_id ORDER BY item_g.goal_id ASC
// // ipd-nurse-focus-list-edit.php
// SELECT * FROM ipd_focus_list WHERE fclist_id = :fclist_id AND an = :an
// SELECT item_g.goal_id FROM kphis.ipd_focus_list_goal_item item_g
//     WHERE item_g.fclist_id =:fclist_id ORDER BY item_g.goal_id ASC
// // into one + added `KphisQueryUtils::checkForeignKeyUsage("ipd_focus_note","fclist_id",$fclist_id);`
// SELECT fcl.*,fcl.create_user AS create_user_fclist,t_fc.focus_name,dchtype.`name` AS dchtype_name,dchtype.dchtype,ipt.dchdate AS dchdate,
//     (SELECT GROUP_CONCAT(CONCAT(gi.goal_id,'^',tg.goal_name) ORDER BY gi.goal_id ASC SEPARATOR '|')
//         FROM kphis.ipd_focus_list_goal_item gi LEFT JOIN kphis.ipd_tmp_goal tg ON tg.goal_id=gi.goal_id
//         WHERE gi.fclist_id=fcl.fclist_id) AS goals,
//     (SELECT COUNT(*) FROM kphis.ipd_focus_note fn WHERE fn.fclist_id=fcl.fclist_id) AS used
// FROM kphis.ipd_focus_list fcl
//     LEFT JOIN kphis.ipd_tmp_focus t_fc ON t_fc.focus_id=fcl.focus_id
//     LEFT JOIN hos.ipt ipt ON ipt.an=fcl.an
//     LEFT JOIN hos.dchtype dchtype ON dchtype.dchtype=ipt.dchtype
// WHERE fcl.an = :an
//     AND fcl.fclist_stdate = :search_startdate
//     AND fcl.fclist_stdate = :search_enddate
//     AND fcl.fclist_stdate BETWEEN :search_startdate AND :search_enddate
//     AND fcl.fclist_status = :search_status
//     AND fcl.fclist_id = :fclist_id
// ORDER BY fcl.fclist_stdate, fcl.fclist_sttime ASC
/// an, (start_date), (end_date), (status), (fclist_id)
pub fn select_focus_list(params: &FocusListParams, hosxp: &str, kphis: &str) -> String {
    let date = match (params.start_date.is_some(), params.end_date.is_some()) {
        (true, true) => " AND fcl.fclist_stdate BETWEEN ? AND ? ",
        (false, false) => "",
        _ => " AND fcl.fclist_stdate=? ",
    };
    let fclist_status = if params.status.is_some() {" AND fcl.fclist_status=? "} else {""};
    let fclist_id = if params.fclist_id.is_some() {" AND fcl.fclist_id=? "} else {""};

    [
        "SELECT fcl.*,fcl.create_user AS create_user_fclist,tf.subgroup,tf.focus_name,tf.focus_status,d.`name` AS dchtype_name,d.dchtype,i.dchdate,i.dchtime,\
            d1.`name` AS create_user_name,d1.licenseno AS create_user_licenseno,u1.entryposition AS create_user_entryposition,\
            d2.`name` AS update_user_name,d2.licenseno AS update_user_licenseno,u2.entryposition AS update_user_entryposition,\
            (SELECT GROUP_CONCAT(CONCAT(gi.goal_id,'^',tg.goal_name) ORDER BY gi.goal_id ASC SEPARATOR '|') \
                FROM ",kphis,".ipd_focus_list_goal_item gi LEFT JOIN ",kphis,".ipd_tmp_goal tg ON tg.goal_id=gi.goal_id \
                WHERE gi.fclist_id=fcl.fclist_id) AS goals,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_focus_note fn WHERE fn.fclist_id=fcl.fclist_id)) AS used \
        FROM ",kphis,".ipd_focus_list fcl \
            LEFT JOIN ",kphis,".ipd_tmp_focus tf ON tf.focus_id=fcl.focus_id \
            LEFT JOIN ",hosxp,".ipt i ON i.an=fcl.an \
            LEFT JOIN ",hosxp,".dchtype d ON d.dchtype=i.dchtype \
            LEFT JOIN ",hosxp,".opduser u1 ON u1.loginname=fcl.create_user \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.`code`=u1.doctorcode \
            LEFT JOIN ",hosxp,".opduser u2 ON u2.loginname=fcl.update_user \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.`code`=u2.doctorcode \
        WHERE fcl.an=? ",date,fclist_status,fclist_id,
        "ORDER BY fcl.fclist_stdate,fcl.fclist_sttime ASC;"
    ].concat()
}

/// an
pub fn select_focus_list_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_focus_list WHERE an=? ORDER BY fclist_stdate,fclist_sttime ASC;"
    ].concat()
}

/// fclist_id
pub fn select_focus_list_goal_item_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_focus_list_goal_item WHERE fclist_id=? ORDER BY fclist_item_id ASC;"
    ].concat()
}

// // ipd-nurse-focus-list-save.php
// INSERT INTO kphis.ipd_focus_list (smp_id,focus_id,focus_text,goal_text,fclist_stdate,fclist_sttime,fclist_enddate,fclist_endtime,fclist_status,
//     hn,an,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,?,'',?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// smp_id, focus_id, focus_text, goal_text, fclist_stdate, fclist_sttime, fclist_enddate, fclist_endtime, fclist_status, hn, an, loginname, loginname
pub fn insert_focus_list(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_focus_list (smp_id,focus_id,focus_text,goal_text,fclist_stdate,fclist_sttime,fclist_enddate,fclist_endtime,fclist_status,\
            hn,an",TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

/// smp_id,focus_id,focus_text,goal_text,fclist_stdate,fclist_sttime,fclist_enddate,fclist_endtime,fclist_status,hn,an,create_user,create_datetime,update_user,update_datetime,version
pub fn insert_focus_list_only(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_focus_list (smp_id,focus_id,focus_text,goal_text,fclist_stdate,fclist_sttime,fclist_enddate,fclist_endtime,fclist_status,\
            hn,an",TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);"
    ].concat()
}

// INSERT INTO kphis.history_ipd_focus_list
//     SELECT NULL,NOW(),'U',?,fcl.* FROM kphis.ipd_focus_list fcl WHERE fcl.fclist_id=?;
// INSERT INTO kphis.history_ipd_focus_list
//     SELECT NULL,NOW(),'I',?,fcl.* FROM kphis.ipd_focus_list fcl WHERE fcl.fclist_id=?;
// INSERT INTO kphis.history_ipd_focus_list
//     SELECT NULL,NOW(),'D',?,fcl.* FROM kphis.ipd_focus_list fcl WHERE fcl.fclist_id=?;
// /// loginname, fclist_id, (version)
// pub fn insert_history_focus_list(mode: &str, check_version: bool, kphis: &str) -> String {
//     let version = if check_version {" AND fcl.version=?"} else {""};
//     [
//         "INSERT INTO ",kphis,".history_ipd_focus_list ",
//             "SELECT NULL,NOW(),'",mode,"',?,fcl.* FROM ",kphis,".ipd_focus_list fcl WHERE fcl.fclist_id=?",version,";"
//     )
// }

// INSERT INTO kphis.ipd_focus_list_goal_item (fclist_id,goal_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:fclist_id,:goal_id,:create_user,NOW(),:update_user,NOW(),:version)
pub fn insert_goal_items(
    goals: &[u32],
    fclist_id: u32,
    user: &str,
    version: i32,
    kphis: &str,
) -> String {
    let goal = goals.iter().map(|id| ["(",&fclist_id.to_string(),",",&id.to_string(),",'",user,"',NOW(),'",user,"',NOW(),",&version.to_string(),")"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".ipd_focus_list_goal_item (fclist_id,goal_id",TABLE_CREATE_COLUMNS,") VALUES ",&goal
    ].concat()
}

pub fn insert_goal_items_only(
    goals: &[FocusListGoalItemOnly],
    fclist_id: u32,
    kphis: &str,
) -> String {
    let values = goals.iter().map(|item| {
        [
            "(",&fclist_id.to_string(),",",
            &item.goal_id.map(|s| s.to_string()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".ipd_focus_list_goal_item (fclist_id,goal_id",TABLE_CREATE_COLUMNS,") VALUES ",&values
    ].concat()
}

// INSERT INTO kphis.history_ipd_focus_list_goal_item
//     SELECT NULL,NOW(),'U',?, i.* FROM kphis.ipd_focus_list_goal_item i
//     WHERE i.fclist_item_id = ?;
// INSERT INTO kphis.history_ipd_focus_list_goal_item
//     SELECT NULL,NOW(),'I',?, i.* FROM kphis.ipd_focus_list_goal_item i
//     WHERE i.fclist_item_id = ?;
// INSERT INTO kphis.history_ipd_focus_list_goal_item
//     SELECT NULL,NOW(),'D',?, i.* FROM kphis.ipd_focus_list_goal_item i
//     WHERE i.fclist_id = ?;
// // we change to insert multple row once, not a single at each iteration
// /// loginname, fclist_id, (version)
// pub fn insert_history_goal_item(mode: &str, check_version: bool, kphis: &str) -> String {
//     let version = if check_version {" AND i.version=?"} else {""};
//     [
//         "INSERT INTO ",kphis,".history_ipd_focus_list_goal_item ",
//             "SELECT NULL,NOW(),'",mode,"',?, i.* FROM ",kphis,".ipd_focus_list_goal_item i WHERE i.fclist_id=?",version,";"
//     )
// }

// // ipd-nurse-focus-list-update.php
// UPDATE kphis.ipd_focus_list
//     SET smp_id=?,focus_id=?,focus_text=?,goal_text=?,fclist_stdate=?,fclist_sttime=?,fclist_enddate=?,fclist_endtime=?,fclist_status=?,
//     hn=?,an=?,update_user=?,update_datetime=NOW(),version=(version+1)
//     WHERE fclist_id=? AND version=?;
/// smp_id, focus_id, focus_text, goal_text, fclist_stdate, fclist_sttime, fclist_enddate, fclist_endtime, fclist_status, hn, an, loginname, fclist_id, version
pub fn update_focus_list_new(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_focus_list SET smp_id=?,focus_id=?,focus_text=?,goal_text=?,fclist_stdate=?,fclist_sttime=?,fclist_enddate=?,fclist_endtime=?,fclist_status=?,\
            hn=?,an=?",TABLE_UPDATE_SET," WHERE fclist_id=? AND version=?;"
    ].concat()
}
// UPDATE kphis.ipd_focus_list
//     SET fclist_enddate=?,fclist_endtime=?,fclist_status=?,
//     hn=?,an=?,version=?,update_datetime=NOW(),update_user=? WHERE fclist_id=?;
/// fclist_enddate, fclist_endtime, fclist_status, hn, an, loginname, fclist_id, version
pub fn update_focus_list_used(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_focus_list SET fclist_enddate=?,fclist_endtime=?,fclist_status=?,\
            hn=?,an=?",TABLE_UPDATE_SET," WHERE fclist_id=? AND version=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_focus_list_goal_item WHERE fclist_id=? AND version=?;
/// fclist_id, version
pub fn delete_goal_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_focus_list_goal_item WHERE fclist_id=? AND version=?;"
    ].concat()
}

// // ipd-nurse-focus-list-delete.php
// DELETE FROM kphis.DELETE FROM kphis.ipd_focus_list WHERE fclist_id = '$fclist_id'
/// fclist_id, version
pub fn delete_focus_list(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_focus_list WHERE fclist_id=? AND version=?;"
    ].concat()
}
