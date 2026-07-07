// // ipd-summary-2-hosxp-or-data.php
// SELECT ol.operation_id,ol.an,oi.name,oi.icd9,od.begin_datetime,od.end_datetime, doctor.name AS doctor_name
// FROM hos.operation_detail od
//     LEFT JOIN hos.operation_list ol ON ol.operation_id=od.operation_id
//     LEFT JOIN hos.doctor ON doctor.code=ol.request_doctor
//     LEFT JOIN hos.operation_item oi ON oi.operation_item_id=od.operation_item_id
// WHERE ol.an=? AND ol.status_id=3 ORDER BY ol.enter_date,ol.enter_time;
/// an
pub fn select_hosxp_operation(operation_success: &[u64], hosxp: &str) -> String {
    let success = operation_success.iter()
        .map(|u| u.to_string()).collect::<Vec<String>>()
        .join(",");
    [
        "SELECT ol.operation_id,ol.an,oi.name,oi.icd9,od.begin_datetime,od.end_datetime,doctor.name AS doctor_name \
        FROM ",hosxp,".operation_detail od \
            LEFT JOIN ",hosxp,".operation_list ol ON ol.operation_id=od.operation_id \
            LEFT JOIN ",hosxp,".doctor ON doctor.code=ol.request_doctor \
            LEFT JOIN ",hosxp,".operation_item oi ON oi.operation_item_id=od.operation_item_id \
        WHERE ol.an=? AND ol.status_id IN (",&success,") ORDER BY ol.enter_date,ol.enter_time;"
    ].concat()
}

// SELECT mpi.med_plan_number, mpi.icode, mpi.orderstatus,mpi.orderdate,IF(ndi.`name` IS NOT NULL,ndi.`name`,CONCAT(di.`name`, ' ', di.strength, ' ',di.units)) AS med_name,di.dosageform,
//     IF(mpi.sp_use <> '',CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,''))) AS drug_usage
// FROM hos.medplan_ipd mpi
//     LEFT JOIN hos.drugitems di ON di.icode=mpi.icode
//     LEFT JOIN hos.nondrugitems ndi ON ndi.icode=mpi.icode
//     LEFT JOIN hos.drugusage du ON du.drugusage=mpi.drugusage
//     LEFT JOIN hos.sp_use u ON u.sp_use=mpi.sp_use
// WHERE mpi.an=? AND (mpi.offdate IS NULL OR mpi.offdate > NOW());
/// an
pub fn select_hosxp_medplan_ipd_remains(hosxp: &str) -> String {
    ["SELECT mpi.med_plan_number, mpi.icode, mpi.orderstatus,mpi.orderdate,IF(ndi.`name` IS NOT NULL,ndi.`name`,CONCAT(di.`name`, ' ', di.strength, ' ',di.units)) AS med_name,di.dosageform,\
        IF(mpi.sp_use <> '',CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,''))) AS drug_usage \
    FROM ",hosxp,".medplan_ipd mpi \
        LEFT JOIN ",hosxp,".drugitems di ON di.icode=mpi.icode \
        LEFT JOIN ",hosxp,".nondrugitems ndi ON ndi.icode=mpi.icode \
        LEFT JOIN ",hosxp,".drugusage du ON du.drugusage=mpi.drugusage \
        LEFT JOIN ",hosxp,".sp_use u ON u.sp_use=mpi.sp_use \
    WHERE mpi.an=? AND (mpi.offdate IS NULL OR mpi.offdate > NOW());"].concat()
}

// SELECT * FROM hos.iptdiag WHERE an=?;
/// an
pub fn select_hosxp_ipt_diag(hosxp: &str) -> String {
    ["SELECT ipt_diag_id,an,diagtype,icd10 FROM ",hosxp,".iptdiag WHERE an=?;"].concat()
}

// SELECT * FROM hos.iptoprt WHERE an=?;
/// an
pub fn select_hosxp_ipt_oprt(hosxp: &str) -> String {
    ["SELECT iptoprt_id,an,icd9 FROM ",hosxp,".iptoprt WHERE an=?;"].concat()
}