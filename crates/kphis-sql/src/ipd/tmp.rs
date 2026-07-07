use kphis_model::ipd::tmp::TmpParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // setting-template-nurse-note-smp-data.php, setting-template-nurse-note-smp-dropdown.php
// SELECT * FROM kphis.ipd_tmp_group_smp ORDER BY smp_id;
/// (smp_id)
pub fn select_group(params: &TmpParams, kphis: &str) -> String {
    let where_id = if params.smp_id.is_some() {"WHERE smp_id=? "} else {""};
    [
        "SELECT * FROM ",kphis,".ipd_tmp_group_smp ",where_id,"ORDER BY smp_name;"
    ].concat()
}

// // setting-template-nurse-note-smp-data.php
// SELECT smp_id AS max_id FROM kphis.ipd_tmp_group_smp ORDER BY smp_id DESC LIMIT 1;
// `ORDER BY x DESC LIMIT 1` faster than `MAX(x)`
// pub fn select_max_group_id(kphis: &str) -> String {
//     [
//         "SELECT smp_id AS max_id FROM ",kphis,".ipd_tmp_group_smp ORDER BY smp_id DESC LIMIT 1;"
//     )
// }

// INSERT INTO kphis.ipd_tmp_group_smp VALUE (smp_id,smp_name,smp_group,smp_order,smp_status,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (IFNULL((SELECT s1.smp_id+1 FROM kphis.ipd_tmp_group_smp s1 WHERE s1.smp_id < 98 ORDER BY s1.smp_id DESC LIMIT 1),1),?,
// IFNULL((SELECT s2.smp_group+1 FROM kphis.ipd_tmp_group_smp s2 WHERE s2.smp_group < 98 ORDER BY s2.smp_group DESC LIMIT 1),1),
// IFNULL((SELECT s3.smp_order+1 FROM kphis.ipd_tmp_group_smp s3 WHERE s3.smp_order < 98 ORDER BY s3.smp_order DESC LIMIT 1),1),?,?,NOW(),?,NOW(),1);
/// smp_name, smp_status, loginname, loginname
pub fn insert_group(kphis: &str) -> String {
    // initial data contains smp_id [1,99]
    [
        "INSERT INTO ",kphis,".ipd_tmp_group_smp (smp_id,smp_name,smp_group,smp_order,smp_status",TABLE_CREATE_COLUMNS,") \
        VALUES (IFNULL(\
            (SELECT s1a.smp_id+2 FROM ",kphis,".ipd_tmp_group_smp s1a WHERE s1a.smp_id = 98),\
            (SELECT s1b.smp_id+1 FROM ",kphis,".ipd_tmp_group_smp s1b WHERE s1b.smp_id <> 99 ORDER BY s1b.smp_id DESC LIMIT 1)\
        ),?,\
        IFNULL(\
            (SELECT s2a.smp_group+2 FROM ",kphis,".ipd_tmp_group_smp s2a WHERE s2a.smp_group = 98),\
            (SELECT s2b.smp_group+1 FROM ",kphis,".ipd_tmp_group_smp s2b WHERE s2b.smp_group <> 99 ORDER BY s2b.smp_group DESC LIMIT 1)\
        ),\
        IFNULL(\
            (SELECT s3a.smp_order+2 FROM ",kphis,".ipd_tmp_group_smp s3a WHERE s3a.smp_order = 98),
            (SELECT s3b.smp_order+1 FROM ",kphis,".ipd_tmp_group_smp s3b WHERE s3b.smp_order <> 99 ORDER BY s3b.smp_order DESC LIMIT 1)
        ),?",TABLE_CREATE_PREPARED,") RETURNING smp_id;"
    ].concat()
}

// UPDATE kphis.ipd_tmp_group_smp SET smp_name=?,smp_status=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE smp_id=?;
/// smp_name, smp_status, loginname, smp_id
pub fn update_group(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_tmp_group_smp SET smp_name=?,smp_status=?",TABLE_UPDATE_SET," WHERE smp_id=?;"
    ].concat()
}

// setting-template-nurse-note-smp-delete.php
// we change `version` checking to `update_user` checking
// // we merged 3 query
// DELETE FROM kphis.ipd_tmp_group_smp WHERE update_user=? AND smp_id=?;
// (SELECT COUNT(*) FROM kphis.ipd_focus_list WHERE smp_id=?)=0
// (SELECT COUNT(*) FROM kphis.opd_er_focus_list WHERE smp_id=?)=0
// // into one
// DELETE FROM kphis.ipd_tmp_group_smp WHERE update_user=? AND smp_id=?
//     AND (SELECT COUNT(*) FROM kphis.ipd_focus_list WHERE smp_id=?)=0 AND (SELECT COUNT(*) FROM kphis.opd_er_focus_list WHERE smp_id=?)=0;
/// loginname, smp_id, smp_id, smp_id
pub fn delete_group(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_tmp_group_smp \
        WHERE update_user=? AND smp_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis,".ipd_focus_list WHERE smp_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_list WHERE smp_id=?);"
    ].concat()
}

// // setting-template-nurse-note-subgroup-data.php, setting-template-nurse-note-subgroup-dropdown.php
// SELECT * FROM kphis.ipd_tmp_subgroup WHERE smp_id=? AND subgroup=? ORDER BY smp_id ASC,subgroup ASC;
/// (smp_id), (subgroup)
pub fn select_subgroup(params: &TmpParams, kphis: &str) -> String {
    let smp_id = if params.smp_id.is_some() {" AND smp_id=? "} else {""};
    let subgroup = if params.subgroup.is_some() {" AND subgroup=? "} else {""};
    [
        "SELECT * FROM ",kphis,".ipd_tmp_subgroup WHERE 1=1 ",smp_id,subgroup," ORDER BY subgroup_name;"
    ].concat()
}

// // setting-template-nurse-note-subgroup-save.php
// // we merged 3 queries
// SELECT subgroup_order+1 FROM kphis.ipd_tmp_subgroup ORDER BY subgroup_order DESC LIMIT 1;
// SELECT subgroup+1 FROM kphis.ipd_tmp_subgroup WHERE smp_id=? ORDER BY subgroup DESC LIMIT 1;
// INSERT INTO kphis.ipd_tmp_subgroup (smp_id,subgroup,subgroup_name,subgroup_order,subgroup_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,?,?,?,?,NOW(),?,NOW(),1);
// // into one and returning subgroup
// INSERT INTO kphis.ipd_tmp_subgroup (smp_id,subgroup,subgroup_name,subgroup_order,subgroup_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,IFNULL((SELECT s1.subgroup+1 FROM kphis.ipd_tmp_subgroup s1 WHERE s1.smp_id=? ORDER BY s1.subgroup DESC LIMIT 1),1),?,
//         IFNULL((SELECT s2.subgroup_order+1 FROM kphis.ipd_tmp_subgroup s2 ORDER BY s2.subgroup_order DESC LIMIT 1),1),?,?,NOW(),?,NOW(),1);
/// smp_id, smp_id, subgroup_name, subgroup_status, loginname, loginname
pub fn insert_subgroup(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_tmp_subgroup (smp_id,subgroup,subgroup_name,subgroup_order,subgroup_status",TABLE_CREATE_COLUMNS,") \
            VALUES (?,IFNULL((SELECT s1.subgroup+1 FROM ",kphis,".ipd_tmp_subgroup s1 WHERE s1.smp_id=? ORDER BY s1.subgroup DESC LIMIT 1),1),?,\
                IFNULL((SELECT s2.subgroup_order+1 FROM ",kphis,".ipd_tmp_subgroup s2 ORDER BY s2.subgroup_order DESC LIMIT 1),1)\
            ,?",TABLE_CREATE_PREPARED,") RETURNING subgroup;"
    ].concat()
}

// UPDATE kphis.ipd_tmp_subgroup SET subgroup_name=?,subgroup_status=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE smp_id=? AND subgroup=?;
/// subgroup_name, subgroup_status, loginname, smp_id, subgroup
pub fn update_subgroup(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_tmp_subgroup SET subgroup_name=?,subgroup_status=?",TABLE_UPDATE_SET," WHERE smp_id=? AND subgroup=?;"
    ].concat()
}

// setting-template-nurse-note-subgroup-delete.php
// we change `version` checking to `update_user` checking
// // we merged 3 query
// DELETE FROM kphis.ipd_tmp_subgroup WHERE update_user=? AND smp_id=? AND subgroup=?;
// (SELECT COUNT(*) FROM kphis.ipd_focus_list WHERE smp_id=?)=0
// (SELECT COUNT(*) FROM kphis.opd_er_focus_list WHERE smp_id=?)=0
// // into one
// DELETE FROM kphis.ipd_tmp_subgroup WHERE update_user=? AND smp_id=? AND subgroup=?
//     AND (SELECT COUNT(*) FROM kphis.ipd_focus_list WHERE smp_id=?)=0 AND (SELECT COUNT(*) FROM kphis.opd_er_focus_list WHERE smp_id=?)=0;
/// loginname, smp_id, subgroup, smp_id, smp_id
pub fn delete_subgroup(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_tmp_subgroup WHERE update_user=? AND smp_id=? AND subgroup=? \
            AND NOT EXISTS(SELECT * FROM ",kphis,".ipd_focus_list WHERE smp_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_list WHERE smp_id=?);"
    ].concat()
}

// // setting-template-nurse-note-focus-data.php, setting-template-nurse-note-focus-dropdown.php
// SELECT * FROM kphis.ipd_tmp_focus WHERE smp_id=? AND subgroup=? ORDER BY focus_order ASC,subgroup ASC;
/// (smp_id), (subgroup), (focus_id)
pub fn select_focus(params: &TmpParams, kphis: &str) -> String {
    let smp_id = if params.smp_id.is_some() {" AND smp_id=? "} else {""};
    let strict = if params.strict.unwrap_or_default() {""} else {" OR subgroup=0"};
    let subgroup = if params.subgroup.is_some() {[" AND (subgroup=?", strict, ") "].concat()} else {String::new()};
    let focus_id = if params.id.is_some() {" AND focus_id=? "} else {""};
    [
        "SELECT * FROM ",kphis,".ipd_tmp_focus WHERE 1=1 ",smp_id,&subgroup,focus_id," ORDER BY focus_name;"
    ].concat()
}

// // setting-template-nurse-note-focus-save.php
// // we merged 2 queries
// SELECT focus_id+1 FROM kphis.ipd_tmp_focus ORDER BY focus_id DESC LIMIT 1;
// INSERT INTO kphis.ipd_tmp_focus (focus_id,focus_name,smp_id,subgroup,focus_order,focus_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,?,?,?,?,?,NOW(),?,NOW(),1);
// // and changed using focus_id as focus_order by adding
// SELECT focus_order+1 FROM kphis.ipd_tmp_focus WHERE smp_id=? AND subgroup=? ORDER BY focus_order DESC LIMIT 1;
// // into one
// INSERT INTO kphis.ipd_tmp_focus (focus_id,focus_name,smp_id,subgroup,focus_order,focus_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (IFNULL((SELECT f1.focus_id+1 FROM kphis.ipd_tmp_focus f1 ORDER BY f1.focus_id DESC LIMIT 1),1),?,?,?,
//         IFNULL((SELECT f2.focus_order+1 FROM kphis.ipd_tmp_focus f2 WHERE f2.smp_id=? AND f2.subgroup=? ORDER BY f2.focus_order DESC LIMIT 1),1),?,?,NOW(),?,NOW(),1);
/// focus_name, smp_id, subgroup, smp_id, smp_id, focus_status, loginname, loginname
pub fn insert_focus(kphis: &str) -> String {
    // initial data contains focus_id [1,2,3,4,5,999]
    [
        "INSERT INTO ",kphis,".ipd_tmp_focus (focus_id,focus_name,smp_id,subgroup,focus_order,focus_status",TABLE_CREATE_COLUMNS,") \
        VALUES (IFNULL(\
            (SELECT f1a.focus_id+2 FROM ",kphis,".ipd_tmp_focus f1a WHERE f1a.focus_id = 998),\
            (SELECT f1b.focus_id+1 FROM ",kphis,".ipd_tmp_focus f1b WHERE f1b.focus_id <> 999 ORDER BY f1b.focus_id DESC LIMIT 1)\
        ),?,?,?,IFNULL(\
            (SELECT f2a.focus_order+2 FROM ",kphis,".ipd_tmp_focus f2a WHERE f2a.smp_id=? AND f2a.focus_order = 998),\
            (SELECT f2b.focus_order+1 FROM ",kphis,".ipd_tmp_focus f2b WHERE f2b.smp_id=? AND f2b.focus_order <> 999 ORDER BY f2b.focus_order DESC LIMIT 1)\
        ),?",TABLE_CREATE_PREPARED,") RETURNING focus_id;"
    ].concat()
}

// UPDATE kphis.ipd_tmp_focus SET focus_name=?,focus_status=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE focus_id=?;
/// focus_name, focus_status, loginname, focus_id
pub fn update_focus(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_tmp_focus SET focus_name=?,focus_status=?",TABLE_UPDATE_SET," WHERE focus_id=?;"
    ].concat()
}

// // setting-template-nurse-note-focus-delete.php
// // we change `version` checking to `update_user` checking
// // we merged 3 query
// DELETE FROM kphis.ipd_tmp_focus WHERE update_user=? AND focus_id=?;
// (SELECT COUNT(*) FROM kphis.ipd_focus_list WHERE focus_id=?)=0
// (SELECT COUNT(*) FROM kphis.opd_er_focus_list WHERE focus_id=?)=0
// // into one
// DELETE FROM kphis.ipd_tmp_focus WHERE update_user=? AND focus_id=?
//     AND (SELECT COUNT(*) FROM kphis.ipd_focus_list WHERE focus_id=?)=0 AND (SELECT COUNT(*) FROM kphis.opd_er_focus_list WHERE focus_id=?)=0;
/// loginname, focus_id, focus_id, focus_id
pub fn delete_focus(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_tmp_focus WHERE update_user=? AND focus_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis,".ipd_focus_list WHERE focus_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_list WHERE focus_id=?);"
    ].concat()
}

// // setting-template-nurse-note-goal-data.php, setting-template-nurse-note-goal-dropdown.php
// SELECT * FROM kphis.ipd_tmp_goal WHERE smp_id=? AND subgroup=? ORDER BY goal_order ASC,subgroup ASC;
/// (smp_id), (subgroup), (goal_id)
pub fn select_goal(params: &TmpParams, kphis: &str) -> String {
    let smp_id = if params.smp_id.is_some() {" AND smp_id=? "} else {""};
    let strict = if params.strict.unwrap_or_default() {""} else {" OR subgroup=0"};
    let subgroup = if params.subgroup.is_some() {[" AND (subgroup=?", strict, ") "].concat()} else {String::new()};
    let goal_id = if params.id.is_some() {" AND goal_id=? "} else {""};
    [
        "SELECT * FROM ",kphis,".ipd_tmp_goal WHERE 1=1 ",smp_id,&subgroup,goal_id," ORDER BY goal_name;"
    ].concat()
}

// // setting-template-nurse-note-goal-save.php
// // we merged 2 queries
// SELECT goal_id+1 FROM kphis.ipd_tmp_goal ORDER BY goal_id DESC LIMIT 1;
// INSERT INTO kphis.ipd_tmp_goal (goal_id,goal_name,smp_id,subgroup,goal_order,goal_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,?,?,?,?,?,NOW(),?,NOW(),1);
// // and changed using goal_id as goal_order by adding
// SELECT goal_order+1 FROM kphis.ipd_tmp_goal WHERE smp_id=? AND subgroup=? ORDER BY goal_order DESC LIMIT 1;
// // into one
// INSERT INTO kphis.ipd_tmp_goal (goal_id,goal_name,smp_id,subgroup,goal_order,goal_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (IFNULL((SELECT g1.goal_id+1 FROM kphis.ipd_tmp_goal g1 ORDER BY g1.goal_id DESC LIMIT 1),1),?,?,?,
//         IFNULL((SELECT g2.goal_order+1 FROM kphis.ipd_tmp_goal g2 WHERE g2.smp_id=? AND g2.subgroup=? ORDER BY g2.goal_order DESC LIMIT 1),1),?,?,NOW(),?,NOW(),1);
/// goal_name, smp_id, subgroup, smp_id, smp_id, goal_status, loginname, loginname
pub fn insert_goal(kphis: &str) -> String {
    // initial data contains goal_id [1,2,3,4,5,999]
    [
        "INSERT INTO ",kphis,".ipd_tmp_goal (goal_id,goal_name,smp_id,subgroup,goal_order,goal_status",TABLE_CREATE_COLUMNS,") \
        VALUES (IFNULL(\
            (SELECT g1a.goal_id+2 FROM ",kphis,".ipd_tmp_goal g1a WHERE g1a.goal_id = 998),\
            (SELECT g1b.goal_id+1 FROM ",kphis,".ipd_tmp_goal g1b WHERE g1b.goal_id <> 999 ORDER BY g1b.goal_id DESC LIMIT 1)\
        ),?,?,?,IFNULL(\
            (SELECT g2a.goal_order+2 FROM ",kphis,".ipd_tmp_goal g2a WHERE g2a.smp_id=? AND g2a.goal_order = 998),\
            (SELECT g2b.goal_order+1 FROM ",kphis,".ipd_tmp_goal g2b WHERE g2b.smp_id=? AND g2b.goal_order <> 999 ORDER BY g2b.goal_order DESC LIMIT 1)\
        ),?",TABLE_CREATE_PREPARED,") RETURNING goal_id;"
    ].concat()
}

// UPDATE kphis.ipd_tmp_goal SET goal_name=?,goal_status=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE goal_id=?;
/// goal_name, goal_status, loginname, goal_id
pub fn update_goal(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_tmp_goal SET goal_name=?,goal_status=?",TABLE_UPDATE_SET," WHERE goal_id=?;"
    ].concat()
}

// // setting-template-nurse-note-goal-delete.php
// // we change `version` checking to `update_user` checking
// // we merged 3 query
// DELETE FROM kphis.ipd_tmp_goal WHERE update_user=? AND goal_id=?;
// (SELECT COUNT(*) FROM kphis.ipd_focus_list_goal_item WHERE goal_id=?)=0
// (SELECT COUNT(*) FROM kphis.opd_er_focus_list_goal_item WHERE goal_id=?)=0
// // into one
// DELETE FROM kphis.ipd_tmp_goal WHERE update_user=? AND goal_id=?
//     AND (SELECT COUNT(*) FROM kphis.ipd_focus_list_goal_item WHERE goal_id=?)=0 AND (SELECT COUNT(*) FROM kphis.opd_er_focus_list_goal_item WHERE goal_id=?)=0;
/// loginname, goal_id, goal_id, goal_id
pub fn delete_goal(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_tmp_goal WHERE update_user=? AND goal_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis,".ipd_focus_list_goal_item WHERE goal_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_list_goal_item WHERE goal_id=?);"
    ].concat()
}

// // setting-template-nurse-note-intvt-data.php, setting-template-nurse-note-intvt-dropdown.php
// SELECT * FROM kphis.ipd_tmp_intvt WHERE smp_id=? AND subgroup=? ORDER BY intvt_order ASC,subgroup ASC;
/// (smp_id), (subgroup), (intvt_id)
pub fn select_intvt(params: &TmpParams, kphis: &str) -> String {
    let smp_id = if params.smp_id.is_some() {" AND smp_id=? "} else {""};
    let strict = if params.strict.unwrap_or_default() {""} else {" OR subgroup=0"};
    let subgroup = if params.subgroup.is_some() {[" AND (subgroup=?", strict, ") "].concat()} else {String::new()};
    let intvt_id = if params.id.is_some() {" AND intvt_id=? "} else {""};
    [
        "SELECT * FROM ",kphis,".ipd_tmp_intvt WHERE 1=1 ",smp_id,&subgroup,intvt_id," ORDER BY intvt_name;"
    ].concat()
}

// // setting-template-nurse-note-intvt-save.php
// // we merged 2 queries
// SELECT intvt_id+1 FROM kphis.ipd_tmp_intvt ORDER BY intvt_id DESC LIMIT 1;
// INSERT INTO kphis.ipd_tmp_intvt (intvt_id,intvt_name,smp_id,subgroup,intvt_order,intvt_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,?,?,?,?,?,NOW(),?,NOW(),1);
// // and changed using intvt_id as intvt_order by adding
// SELECT intvt_order+1 FROM kphis.ipd_tmp_intvt WHERE smp_id=? AND subgroup=? ORDER BY intvt_order DESC LIMIT 1;
// // into one
// INSERT INTO kphis.ipd_tmp_intvt (intvt_id,intvt_name,smp_id,subgroup,intvt_order,intvt_status,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (IFNULL((SELECT g1.intvt_id+1 FROM kphis.ipd_tmp_intvt g1 ORDER BY g1.intvt_id DESC LIMIT 1),1),?,?,?,
//         IFNULL((SELECT g2.intvt_order+1 FROM kphis.ipd_tmp_intvt g2 WHERE g2.smp_id=? AND g2.subgroup=? ORDER BY g2.intvt_order DESC LIMIT 1),1),?,?,NOW(),?,NOW(),1);
/// intvt_name, smp_id, subgroup, smp_id, smp_id, intvt_status, loginname, loginname
pub fn insert_intvt(kphis: &str) -> String {
    // initial data contains goal_id [1,2,3,4,5,6,7,8,9,10,9999]
    [
        "INSERT INTO ",kphis,".ipd_tmp_intvt (intvt_id,intvt_name,smp_id,subgroup,intvt_order,intvt_status",TABLE_CREATE_COLUMNS,") \
        VALUES (IFNULL(
            (SELECT g1a.intvt_id+2 FROM ",kphis,".ipd_tmp_intvt g1a WHERE g1a.intvt_id = 9998),
            (SELECT g1b.intvt_id+1 FROM ",kphis,".ipd_tmp_intvt g1b WHERE g1b.intvt_id <> 9999 ORDER BY g1b.intvt_id DESC LIMIT 1)
        ),?,?,?,IFNULL(
            (SELECT g2a.intvt_order+2 FROM ",kphis,".ipd_tmp_intvt g2a WHERE g2a.smp_id=? AND g2a.intvt_order = 9998),
            (SELECT g2b.intvt_order+1 FROM ",kphis,".ipd_tmp_intvt g2b WHERE g2b.smp_id=? AND g2b.intvt_order <> 9999 ORDER BY g2b.intvt_order DESC LIMIT 1)
        ),?",TABLE_CREATE_PREPARED,") RETURNING intvt_id;"
    ].concat()
}

// UPDATE kphis.ipd_tmp_intvt SET intvt_name=?,intvt_status=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE intvt_id=?;
/// intvt_name, intvt_status, loginname, intvt_id
pub fn update_intvt(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_tmp_intvt SET intvt_name=?,intvt_status=?",TABLE_UPDATE_SET," WHERE intvt_id=?;"
    ].concat()
}

// // setting-template-nurse-note-intvt-delete.php
// // we change `version` checking to `update_user` checking
// // we merged 3 query
// DELETE FROM kphis.ipd_tmp_intvt WHERE update_user=? AND intvt_id=?;
// (SELECT COUNT(*) FROM kphis.ipd_focus_note_intvt_item WHERE intvt_id=?)=0
// (SELECT COUNT(*) FROM kphis.opd_er_focus_note_intvt_item WHERE intvt_id=?)=0
// // into one
// DELETE FROM kphis.ipd_tmp_intvt WHERE update_user=? AND intvt_id=?
//     AND (SELECT COUNT(*) FROM kphis.ipd_focus_note_intvt_item WHERE intvt_id=?)=0 AND (SELECT COUNT(*) FROM kphis.opd_er_focus_note_intvt_item WHERE intvt_id=?)=0;
/// loginname, intvt_id, intvt_id, intvt_id
pub fn delete_intvt(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_tmp_intvt WHERE update_user=? AND intvt_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis,".ipd_focus_note_intvt_item WHERE intvt_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_note_intvt_item WHERE intvt_id=?);"
    ].concat()
}

// SELECT * FROM kphis.ipd_tmp_dlc ORDER BY dlc_order ASC;
/// (dlc_id)
pub fn select_dlc(params: &TmpParams, kphis: &str) -> String {
    let dlc_id = if params.id.is_some() {" WHERE dlc_id=?"} else {""};
    [
        "SELECT * FROM ",kphis,".ipd_tmp_dlc",dlc_id," ORDER BY dlc_name;"
    ].concat()
}

// // dlc_id AUTO_INCREMENT version
// INSERT INTO kphis.ipd_tmp_dlc (dlc_name,dlc_order,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,IFNULL((SELECT d1.dlc_order+1 FROM kphis.ipd_tmp_dlc d1 ORDER BY d1.dlc_order DESC LIMIT 1),1),?,NOW(),?,NOW(),1);
/// dlc_name, loginname, loginname
pub fn insert_dlc(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_tmp_dlc (dlc_name,dlc_order",TABLE_CREATE_COLUMNS,") \
        VALUES (?,IFNULL((SELECT d2.dlc_order+1 FROM ",kphis,".ipd_tmp_dlc d2 ORDER BY d2.dlc_order DESC LIMIT 1),1)",TABLE_CREATE_PREPARED,");"
    ].concat()
}
// // dlc_id NOT AUTO_INCREMENT version
// INSERT INTO kphis.ipd_tmp_dlc (dlc_id,dlc_name,dlc_order,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (IFNULL((SELECT d1.dlc_id+1 FROM kphis.ipd_tmp_dlc d1 ORDER BY d1.dlc_id DESC LIMIT 1),1),?,
//         IFNULL((SELECT d2.dlc_order+1 FROM kphis.ipd_tmp_dlc d2 ORDER BY d2.dlc_order DESC LIMIT 1),1),?,NOW(),?,NOW(),1);
// /// dlc_name, loginname, loginname
// pub fn insert_dlc(kphis: &str) -> String {
//     [
//         "INSERT INTO ",kphis,".ipd_tmp_dlc (dlc_id,dlc_name,dlc_order,create_user,create_datetime,update_user,update_datetime,version) ",
//         "VALUES (IFNULL((SELECT d1.dlc_id+1 FROM ",kphis,".ipd_tmp_dlc d1 ORDER BY d1.dlc_id DESC LIMIT 1),1),?,",
//             "IFNULL((SELECT d2.dlc_order+1 FROM ",kphis,".ipd_tmp_dlc d2 ORDER BY d2.dlc_order DESC LIMIT 1),1),?,NOW(),?,NOW(),1);"
//     )
// }

// UPDATE kphis.ipd_tmp_dlc SET dlc_name=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE dlc_id=?;
/// dlc_name, loginname, dlc_id
pub fn update_dlc(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_tmp_dlc SET dlc_name=?",TABLE_UPDATE_SET," WHERE dlc_id=?;"
    ].concat()
}

// // setting-template-nurse-note-intvt-delete.php
// // we change `version` checking to `update_user` checking
// DELETE FROM kphis.ipd_tmp_dlc WHERE update_user=? AND dlc_id=?
//     AND (SELECT COUNT(*) FROM kphis.ipd_focus_note_dlc_item WHERE dlc_id=?)=0 AND (SELECT COUNT(*) FROM kphis.opd_er_focus_note_dlc_item WHERE dlc_id=?)=0;
/// loginname, dlc_id, dlc_id, dlc_id
pub fn delete_dlc(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_tmp_dlc WHERE update_user=? AND dlc_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis,".ipd_focus_note_dlc_item WHERE dlc_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_note_dlc_item WHERE dlc_id=?);"
    ].concat()
}
