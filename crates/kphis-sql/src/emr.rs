// // ipd-emr.php
// SELECT ovst.vn,ovst.vstdate,ovst.vsttime,ipt.an,om.opd_er_order_master_id
// FROM hos.ovst
//    LEFT JOIN hos.ipt ON ipt.vn=ovst.vn
//    LEFT JOIN kphis.opd_er_order_master om ON om.vn=ovst.vn AND (om.delete_flag IS NULL OR om.delete_flag <> 'Y')
// WHERE ovst.hn=? ORDER BY vstdate DESC,vsttime DESC;
/// hn
pub fn select_visits(hosxp: &str,  kphis: &str) -> String {
    [
        "SELECT ovst.vn,ovst.vstdate,ovst.vsttime,ipt.an,om.opd_er_order_master_id \
        FROM ",hosxp,".ovst \
            LEFT JOIN ",hosxp,".ipt ON ipt.vn=ovst.vn \
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ovst.vn AND (om.delete_flag IS NULL OR om.delete_flag <> 'Y') \
        WHERE ovst.hn=? ORDER BY vstdate DESC,vsttime DESC;"
    ].concat()
}

// // ipd-emr-detail.php
// SELECT o.vn,o.vstdate,o.vsttime,ipt.an,p.hn,CONCAT(p.pname,p.fname,' ',p.lname) AS ptname,
//     CONCAT(v.age_y,' ปี ',v.age_m,'  เดือน ',v.age_d,'  วัน') AS age_th,o.pttype,pt.name AS pttype_name,
//     od.bps,od.bpd,od.height,od.bw,od.pulse,od.temperature,od.cc,od.hr,od.pe,od.rr,od.bmi,
//     ovstist.name AS ovstist_name,d.name AS doctor_name,
//     (SELECT EXISTS(SELECT * FROM hos.referout WHERE vn=o.vn)) AS has_data_refer_out
// FROM hos.ovst o
//     LEFT JOIN hos.ipt ON ipt.vn=o.vn
//     LEFT JOIN hos.vn_stat v ON v.vn=o.vn
//     LEFT JOIN hos.doctor d ON d.`code`=o.doctor
//     LEFT JOIN hos.patient p ON p.hn=o.hn
//     LEFT JOIN hos.pttype pt ON pt.pttype=o.pttype
//     LEFT JOIN hos.opdscreen od ON od.vn=o.vn
//     LEFT JOIN hos.ovstist ON ovstist.ovstist=o.ovstist
// WHERE o.vn=?;
/// vn
pub fn select_visit_detail(hosxp: &str) -> String {
    [
        "SELECT o.vn,o.vstdate,o.vsttime,ipt.an,p.hn,CONCAT(p.pname,p.fname,' ',p.lname) AS ptname,\
            CONCAT(v.age_y,' ปี ',v.age_m,'  เดือน ',v.age_d,'  วัน') AS age_th,o.pttype,pt.name AS pttype_name,\
            od.bps,od.bpd,od.height,od.bw,od.pulse,od.temperature,od.cc,od.hpi,od.pmh,od.fh,od.sh,od.hr,od.pe,od.rr,od.bmi,\
            ovstist.name AS ovstist_name,d.name AS doctor_name,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".referout WHERE vn=o.vn)) AS has_data_refer_out \
        FROM ",hosxp,".ovst o \
            LEFT JOIN ",hosxp,".ipt ON ipt.vn=o.vn \
            LEFT JOIN ",hosxp,".vn_stat v ON v.vn=o.vn \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=o.doctor \
            LEFT JOIN ",hosxp,".patient p ON p.hn=o.hn \
            LEFT JOIN ",hosxp,".pttype pt ON pt.pttype=o.pttype \
            LEFT JOIN ",hosxp,".opdscreen od ON od.vn=o.vn \
            LEFT JOIN ",hosxp,".ovstist ON ovstist.ovstist=o.ovstist \
        WHERE o.vn=?;"
    ].concat()
}

// SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd101.name,''),' (PDX)'),CONCAT(IFNULL(ovstdiag.icd10,''),':',IFNULL(icd101.name,''))) AS diagnosis,diagtype,ovst_diag_id
//      FROM hos.ovstdiag LEFT JOIN hos.icd101 ON icd101.code=ovstdiag.icd10
//      WHERE vn=? ORDER BY ovstdiag.diagtype,ovstdiag.icd10;
// UNION
// SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),':',IFNULL(icd9cm1.name,''),' (PDX)'),CONCAT(IFNULL(ovstdiag.icd10,''),':',IFNULL(icd9cm1.name,''))) AS diagnosis, diagtype, ovst_diag_id
//      FROM hos.ovstdiag INNER JOIN hos.icd9cm1 ON icd9cm1.code=ovstdiag.icd10
//      WHERE vn=?
// ORDER BY diagtype,ovst_diag_id;
/// vn,vn<br>
/// return 'diagnosis'
pub fn select_diagnosis(hosxp: &str) -> String {
    [
        "SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd101.name,''),' (PDX)'),CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd101.name,''))) AS diagnosis,diagtype,ovst_diag_id \
            FROM ",hosxp,".ovstdiag INNER JOIN ",hosxp,".icd101 ON icd101.code=ovstdiag.icd10 \
            WHERE vn=? \
        UNION \
        SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd9cm1.name,''),' (PDX)'),CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd9cm1.name,''))) AS diagnosis,diagtype,ovst_diag_id \
            FROM ",hosxp,".ovstdiag INNER JOIN ",hosxp,".icd9cm1 ON icd9cm1.code=ovstdiag.icd10 \
            WHERE vn=? \
        ORDER BY diagtype,ovst_diag_id;"
    ].concat()
}

// SELECT CONCAT(IFNULL(d.name,''),' ',IFNULL(d.strength,''),' ',
//     IF(o1.sp_use <> '',CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),''),
//     IFNULL(du.shortlist,''),' X ',IFNULL(o1.qty,'')) AS drug
// FROM hos.opitemrece o1
//     INNER JOIN hos.drugitems d ON o1.icode=d.icode
//     LEFT JOIN hos.drugusage du ON du.drugusage=o1.drugusage
//     LEFT JOIN hos.sp_use u ON u.sp_use=o1.sp_use
// WHERE o1.vn=? ORDER BY o1.item_no;
/// vn|an(is_home_med)<br>
/// return 'drug'
pub fn select_drug(hosxp: &str, is_home_med: bool) -> String {
    let w = if is_home_med {"o1.an=? AND o1.item_type='H'"} else {"o1.vn=?"};
    [
        "SELECT CONCAT(IFNULL(d.name,''),' ',IFNULL(d.strength,''),' ',\
            IF(o1.sp_use <> '',CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),''),\
            IFNULL(du.shortlist,''),' X ',IFNULL(o1.qty,'')) AS drug \
        FROM ",hosxp,".opitemrece o1 \
            INNER JOIN ",hosxp,".drugitems d ON o1.icode=d.icode \
            LEFT JOIN ",hosxp,".drugusage du ON du.drugusage=o1.drugusage \
            LEFT JOIN ",hosxp,".sp_use u ON u.sp_use=o1.sp_use \
        WHERE ",w," ORDER BY o1.item_no;"
    ].concat()
}

// SELECT CONCAT(d.name,' X ',o2.qty) AS nondrug
//     FROM hos.opitemrece o2
// INNER JOIN hos.nondrugitems d ON o2.icode=d.icode
// WHERE o2.vn=? ORDER BY o2.item_no;
/// vn|an(is_home_med)<br>
/// return 'nondrug'
pub fn select_nondrug(hosxp: &str, is_home_med: bool) -> String {
    let w = if is_home_med {"o2.an=? AND o2.item_type='H'"} else {"o2.vn=?"};
    [
        "SELECT CONCAT(d.name,' X ',o2.qty) AS nondrug \
            FROM ",hosxp,".opitemrece o2 \
        INNER JOIN ",hosxp,".nondrugitems d ON o2.icode=d.icode \
        WHERE ",w," ORDER BY o2.item_no"
    ].concat()
}
