use crate::TABLE_CREATE_COLUMNS;

// kphis-config-ipd-ward-passcode-data.php
// SELECT w.ward, w.name AS ward_name, IF(p.passcode IS NOT NULL, 'Y', 'N') AS using_passcode
// FROM hos.ward w INNER JOIN kphis.ipd_ward_passcode_user u ON u.ward = w.ward
// LEFT JOIN kphis.ipd_ward_passcode p ON p.ward = w.ward
// WHERE u.loginname = ? AND p.passcode IS NOT NULL ORDER BY name;
pub fn select_config_ipd_ward_passcode_data(
    // using_passcode: bool,
    hosxp: &str,
    kphis: &str,
) -> String {
    // let not = if using_passcode { "NOT" } else { "" };
    [
        "SELECT w.ward, w.name AS ward_name, IF(p.passcode IS NOT NULL, 'Y', 'N') AS using_passcode \
        FROM ",hosxp,".ward w INNER JOIN ",kphis,".ipd_ward_passcode_user u ON u.ward = w.ward \
            LEFT JOIN ",kphis,".ipd_ward_passcode p ON p.ward = w.ward \
        WHERE u.loginname = ? ORDER BY name;"
        // "WHERE u.loginname = ? AND p.passcode IS ",not," NULL ORDER BY name;"
    ].concat()
}

// kphis-config-ipd-ward-passcode-gen.php
// SELECT * FROM kphis.ipd_ward_passcode_user WHERE ward=? AND loginname=?;
pub fn select_can_change_specific_ward_passcode(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_ward_passcode_user WHERE ward=? AND loginname=?;"
    ].concat()
}

// kphis-config-ipd-ward-passcode-gen.php
// REPLACE INTO kphis.ipd_ward_passcode (ward,passcode,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,NOW(),?,NOW(),version+1);
pub fn replace_ward_passcode(kphis: &str) -> String {
    [
        "REPLACE INTO ",kphis,".ipd_ward_passcode (ward,passcode",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,NOW(),?,NOW(),version+1);"
    ].concat()
}

// kphis-config-ipd-ward-passcode-gen.php
// DELETE FROM kphis.ipd_ward_passcode where ward=?;
pub fn delete_ward_passcode(kphis: &str) -> String {
    ["DELETE FROM ", kphis, ".ipd_ward_passcode where ward=?;"].concat()
}
