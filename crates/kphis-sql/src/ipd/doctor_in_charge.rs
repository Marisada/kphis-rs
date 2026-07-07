use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// // ipd-nurse-doctor-in-charge-table.php
// SELECT c.*,d.`name` AS doctor_name,s.`name` AS spclty_name
// FROM kphis.ipd_doctor_in_charge c
//     LEFT JOIN hos.doctor d ON d.code=c.doctor
//     LEFT JOIN hos.spclty s ON s.spclty=c.spclty
// WHERE c.an=? ORDER BY c.`status` DESC,c.doctor_in_charge_id ASC;
/// an
pub fn select_doctor_in_charges(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT c.*,d.`name` AS doctor_name,s.`name` AS spclty_name \
        FROM ",kphis,".ipd_doctor_in_charge c \
            LEFT JOIN ",hosxp,".doctor d ON d.code=c.doctor \
            LEFT JOIN ",hosxp,".spclty s ON s.spclty=c.spclty \
        WHERE c.an=? ORDER BY c.`status` DESC,c.doctor_in_charge_id ASC;"
    ].concat()
}

// // ipd-nurse-doctor-in-charge-save.php
// INSERT INTO kphis.ipd_doctor_in_charge (an,hn,doctor,spclty,status,activated,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// an, hn, doctor, spclty, status, activated, loginname, loginname
pub fn insert_doctor_in_charge(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_doctor_in_charge (an,hn,doctor,spclty,status,activated",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // ipd-nurse-doctor-in-charge-update.php
// UPDATE kphis.ipd_doctor_in_charge
// SET an=?,hn=?,doctor=?,spclty=?,status=?,activated=?,update_datetime=NOW(),update_user=?,version=(version + 1)
// WHERE doctor_in_charge_id=? AND version=?;
/// an, hn, doctor, spclty, status, activated, loginname, doctor_in_charge_id, version
pub fn update_doctor_in_charge(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_doctor_in_charge \
        SET an=?,hn=?,doctor=?,spclty=?,status=?,activated=?,update_datetime=NOW(),update_user=?,version=(version + 1) \
        WHERE doctor_in_charge_id=? AND version=?;"
    ].concat()
}

// // ipd-nurse-doctor-in-charge-delete.php
// DELETE FROM kphis.ipd_doctor_in_charge WHERE doctor_in_charge_id=? AND version=?;
/// doctor_in_charge_id, version
pub fn delete_doctor_in_charge(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_doctor_in_charge WHERE doctor_in_charge_id=? AND version=?;"
    ].concat()
}
