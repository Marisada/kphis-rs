use kphis_model::drug_use_duration::DrugUseDurationParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// SELECT dd.*,CONCAT(di.`name`, ' ', di.strength, ' ',di.units) AS med_name
//    FROM kphis.kphis_drug_use_duration dd
// LEFT JOIN hos.drugitems di ON di.icode=dd.icode
// WHERE dd.icode=? AND dd.status=? ORDER BY dd.icode;
/// (icode),(due_status),(monitor_status),(info_status),(med_name)
pub fn get_drug_use_duration(params: &DrugUseDurationParams, hosxp: &str, kphis: &str) -> String {
    let icode = if params.icode.is_some() {" AND dd.icode=?"} else {""};
    let med_name = if params.med_name.is_some() {" HAVING med_name LIKE ?"} else {""};
    let due_status = if params.due_status.is_some() {" AND dd.status=?"} else {""};
    let monitor_status = if params.monitor_status.is_some() {" AND dd.monitor_status=?"} else {""};
    let info_status = if params.info_status.is_some() {" AND dd.info_status=?"} else {""};
    ["SELECT dd.*,CONCAT(di.`name`, ' ', di.strength, ' ',di.units) AS med_name \
    FROM ",kphis,".kphis_drug_use_duration dd \
        LEFT JOIN ",hosxp,".drugitems di ON di.icode=dd.icode
    WHERE 1=1 ", icode, due_status, monitor_status, info_status, med_name, " ORDER BY dd.icode;"].concat()
}

/// (icode,usage,duration1,exceed_duration1_color,duration2,exceed_duration2_color,duration3,exceed_duration3_color,status,monitor,monitor_count,monitor_duration,monitor_status,info,info_status,loginname,loginname)
pub fn insert_duplicate_drug_use_duration(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".kphis_drug_use_duration (icode,`usage`,duration1,exceed_duration1_color,duration2,exceed_duration2_color,duration3,exceed_duration3_color,status,\
            monitor,monitor_count,monitor_duration,monitor_status,info,info_status",TABLE_CREATE_COLUMNS,") \
            VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,") \
        ON DUPLICATE KEY UPDATE \
            `usage`=VALUE(`usage`),duration1=VALUE(duration1),exceed_duration1_color=VALUE(exceed_duration1_color),duration2=VALUE(duration2),exceed_duration2_color=VALUE(exceed_duration2_color),duration3=VALUE(duration3),exceed_duration3_color=VALUE(exceed_duration3_color),status=VALUE(status),\
            monitor=VALUE(monitor),monitor_count=VALUE(monitor_count),monitor_duration=VALUE(monitor_duration),monitor_status=VALUE(monitor_status),info=VALUE(info),info_status=VALUE(info_status),\
            update_user=VALUE(update_user),update_datetime=VALUE(update_datetime),version=(version+1);"
    ].concat()
}