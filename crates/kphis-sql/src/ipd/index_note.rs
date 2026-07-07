use kphis_model::ipd::index_note::IndexNoteParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // ipd-nurse-index-note-data.php
// SELECT * FROM kphis.ipd_nurse_index_note WHERE an=? ORDER BY nurse_index_note_id;
/// (nurse_index_note_id), (an)
pub fn select_ipd_nurse_index_note(params: &IndexNoteParams, kphis: &str) -> String {
    let nurse_index_note_id = if params.nurse_index_note_id.is_some() {"AND nurse_index_note_id=? "} else {""};
    let an = if params.an.is_some() { "AND an=? " } else { "" };
    [
        "SELECT * FROM ",kphis,".ipd_nurse_index_note \
        WHERE 1=1 ",nurse_index_note_id,an,
        "ORDER BY nurse_index_note_id;"
    ].concat()
}

/// an
pub fn select_ipd_nurse_index_note_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_nurse_index_note WHERE an=? ORDER BY nurse_index_note_id;"
    ].concat()
}

// // ipd-nurse-index-note-save.php
// INSERT INTO kphis.ipd_nurse_index_note (an,nurse_index_note,create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,?,NOW(),?,NOW(),1);
/// an, nurse_index_note, loginname, loginname
pub fn insert_ipd_nurse_index_note(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_nurse_index_note (an,nurse_index_note",TABLE_CREATE_COLUMNS,") VALUES (?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis.ipd_nurse_index_note SET nurse_index_note=?, update_user=?, update_datetime=NOW(),version=(version+1) WHERE nurse_index_note_id=?;
/// nurse_index_note, loginname, nurse_index_note_id
pub fn update_ipd_nurse_index_note(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_nurse_index_note SET nurse_index_note=?",TABLE_UPDATE_SET," WHERE nurse_index_note_id=?;"
    ].concat()
}

// // ipd-nurse-index-note-delete.php
// DELETE FROM kphis.ipd_nurse_index_note WHERE nurse_index_note_id=?;
/// nurse_index_note_id
pub fn delete_ipd_nurse_index_note(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_nurse_index_note WHERE nurse_index_note_id=?;"
    ].concat()
}
