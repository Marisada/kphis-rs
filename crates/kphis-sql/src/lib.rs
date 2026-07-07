#[rustfmt::skip]
pub mod avatar;

#[rustfmt::skip]
pub mod data_history_utils;

#[rustfmt::skip]
pub mod drug_use_duration;

#[rustfmt::skip]
pub mod emr;

#[rustfmt::skip]
pub mod image;

#[rustfmt::skip]
pub mod ipd;

#[rustfmt::skip]
pub mod lab;

#[rustfmt::skip]
pub mod log;

#[rustfmt::skip]
pub mod med_reconciliation;

#[rustfmt::skip]
pub mod opd_er;

#[rustfmt::skip]
pub mod post_admit;

#[rustfmt::skip]
pub mod pre_admit;

#[rustfmt::skip]
pub mod pre_order;

#[rustfmt::skip]
pub mod prescription;

#[rustfmt::skip]
pub mod query_utils;

#[rustfmt::skip]
pub mod refer_note;

#[rustfmt::skip]
pub mod refer_out;

#[rustfmt::skip]
pub mod report;

#[rustfmt::skip]
pub mod schema_update;

#[rustfmt::skip]
pub mod search;

#[rustfmt::skip]
pub mod select_utils;

#[rustfmt::skip]
pub mod sse;

#[rustfmt::skip]
pub mod transform;

#[rustfmt::skip]
pub mod user;

#[rustfmt::skip]
pub mod xray;

pub const TABLE_CREATE_COLUMNS: &str = ",create_user,create_datetime,update_user,update_datetime,version";
pub const TABLE_CREATE_PREPARED: &str = ",?,NOW(),?,NOW(),1";
pub const TABLE_UPDATE_SET: &str = ",update_user=?,update_datetime=NOW(),version=(version+1)";

// UPDATE hos.serial SET serial_no=serial_no+1 WHERE name='sp_use';
#[rustfmt::skip]
pub fn bump_serial(serial_name: &str, hosxp: &str) -> String {
    ["UPDATE ",hosxp,".serial SET serial_no=serial_no+1 WHERE name='",serial_name,"';"].concat()
}

// UPDATE hos.serial SET serial_no=serial_no+1 WHERE name='sp_use';
/// serial_no
#[rustfmt::skip]
pub fn update_serial(serial_name: &str, hosxp: &str) -> String {
    ["UPDATE ",hosxp,".serial SET serial_no=? WHERE name='",serial_name,"';"].concat()
}

// SELECT serial_no FROM hos.serial WHERE name='sp_use';
#[rustfmt::skip]
pub fn get_serial(serial_name: &str, hosxp: &str) -> String {
    ["SELECT serial_no FROM ",hosxp,".serial WHERE name='",serial_name,"';"].concat()
}

// SELECT sys_value FROM hos.sys_var WHERE sys_name='SPUSE_PREFIX';
#[rustfmt::skip]
pub fn get_sp_use_prefix(hosxp: &str) -> String {
    ["SELECT sys_value FROM ",hosxp,".sys_var WHERE sys_name='SPUSE_PREFIX';"].concat()
}

// SELECT sys_value FROM hos.sys_var WHERE sys_name='SPUSE_PREFIX';
/// sys_value
#[rustfmt::skip]
pub fn update_sp_use_prefix(hosxp: &str) -> String {
    ["UPDATE ",hosxp,".sys_var SET sys_value=? WHERE sys_name='SPUSE_PREFIX';"].concat()
}
