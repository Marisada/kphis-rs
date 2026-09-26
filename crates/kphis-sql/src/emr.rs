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
        "SELECT o.vn,o.vstdate,o.vsttime,ipt.dchdate,ipt.an,p.hn,CONCAT(p.pname,p.fname,' ',p.lname) AS ptname,\
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

// SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd101.name,''),' (PDx)'),CONCAT(IFNULL(ovstdiag.icd10,''),':',IFNULL(icd101.name,''))) AS diagnosis,diagtype,ovst_diag_id
//      FROM hos.ovstdiag LEFT JOIN hos.icd101 ON icd101.code=ovstdiag.icd10
//      WHERE vn=? ORDER BY ovstdiag.diagtype,ovstdiag.icd10;
// UNION
// SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),':',IFNULL(icd9cm1.name,''),' (PDx)'),CONCAT(IFNULL(ovstdiag.icd10,''),':',IFNULL(icd9cm1.name,''))) AS diagnosis, diagtype, ovst_diag_id
//      FROM hos.ovstdiag INNER JOIN hos.icd9cm1 ON icd9cm1.code=ovstdiag.icd10
//      WHERE vn=?
// ORDER BY diagtype,ovst_diag_id;
/// vn,vn<br>
/// return 'diagnosis'
pub fn select_diagnosis(hosxp: &str) -> String {
    [
        "SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd101.name,''),' (PDx)'),CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd101.name,''))) AS diagnosis,diagtype,ovst_diag_id \
            FROM ",hosxp,".ovstdiag INNER JOIN ",hosxp,".icd101 ON icd101.code=ovstdiag.icd10 \
            WHERE vn=? \
        UNION \
        SELECT IF(ovstdiag.diagtype=1,CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd9cm1.name,''),' (PDx)'),CONCAT(IFNULL(ovstdiag.icd10,''),' : ',IFNULL(icd9cm1.name,''))) AS diagnosis,diagtype,ovst_diag_id \
            FROM ",hosxp,".ovstdiag INNER JOIN ",hosxp,".icd9cm1 ON icd9cm1.code=ovstdiag.icd10 \
            WHERE vn=? \
        ORDER BY diagtype,ovst_diag_id;"
    ].concat()
}

// // SELECT CONCAT(IFNULL(d.name,''),' ',IFNULL(d.strength,''),' ',
// //     IF(o1.sp_use <> '',CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),''),
// //     IFNULL(du.shortlist,''),' X ',IFNULL(o1.qty,'')) AS drug
// // FROM hos.opitemrece o1
// //     INNER JOIN hos.drugitems d ON o1.icode=d.icode
// //     LEFT JOIN hos.drugusage du ON du.drugusage=o1.drugusage
// //     LEFT JOIN hos.sp_use u ON u.sp_use=o1.sp_use
// // WHERE o1.vn=? ORDER BY o1.item_no;
// /// vn|an(is_home_med)
// pub fn select_drug(hosxp: &str, is_home_med: bool) -> String {
//     let w = if is_home_med {"o1.an=? AND o1.item_type='H'"} else {"o1.vn=?"};
//     [
//         "SELECT CONCAT(IFNULL(d.name,''),' ',IFNULL(d.strength,''),' ',\
//             IF(o1.sp_use <> '',CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),''),\
//             IFNULL(du.shortlist,''),' X ',IFNULL(o1.qty,'')) AS drug \
//         FROM ",hosxp,".opitemrece o1 \
//             INNER JOIN ",hosxp,".drugitems d ON o1.icode=d.icode \
//             LEFT JOIN ",hosxp,".drugusage du ON du.drugusage=o1.drugusage \
//             LEFT JOIN ",hosxp,".sp_use u ON u.sp_use=o1.sp_use \
//         WHERE ",w," ORDER BY o1.item_no;"
//     ].concat()
// }

// SELECT CONCAT(d.`name`,' ',d.strength,' ',d.units) AS name_drugitems,d.generic_name,d.strength,o.qty,o.icode,o.rxdate,o.rxtime,o.drugusage,o.vn,o.an,o.hn,o.sp_use,
//     IF(o.sp_use IS NULL OR TRIM(o.sp_use)='',du.name1,sp.name1) AS name1,
//     IF(o.sp_use IS NULL OR TRIM(o.sp_use)='',du.name2,sp.name2) AS name2,
//     IF(o.sp_use IS NULL OR TRIM(o.sp_use)='',du.name3,sp.name3) AS name3,
//     du.shortlist, IF(d.icode='1111111',1,0) AS is_medrec 
// FROM hos.opitemrece o
//     INNER JOIN hos.drugitems d ON d.icode=o.icode
//     LEFT JOIN hos.sp_use sp ON sp.sp_use=o.sp_use
//     LEFT JOIN hos.drugusage dr ON dr.drugusage=o.drugusage
// WHERE o.hn=? AND (o.an IS NULL OR (o.an IS NOT NULL AND o.item_type='H')) AND o.vstdate BETWEEN ? AND ? ORDER BY o.rxdate DESC, o.rxtime DESC, d.`name`;;
/// vn|an(is_home_med)
pub fn select_drug(is_home_med: bool, med_rec_icode: &str, hosxp: &str) -> String {
    let w = if is_home_med {"o.an=? AND o.item_type='H'"} else {"o.vn=?"};
    [
        "SELECT CONCAT(d.`name`,' ',d.strength,' ',d.units) AS name_drugitems,d.generic_name,d.strength,o.qty,o.icode,o.rxdate,o.rxtime,o.drugusage,o.vn,o.an,o.hn,o.sp_use,\
            IF(o.sp_use IS NULL OR TRIM(o.sp_use)='',du.name1,sp.name1) AS name1,\
            IF(o.sp_use IS NULL OR TRIM(o.sp_use)='',du.name2,sp.name2) AS name2,\
            IF(o.sp_use IS NULL OR TRIM(o.sp_use)='',du.name3,sp.name3) AS name3,\
            du.shortlist, IF(d.icode='",med_rec_icode,"',1,0) AS is_medrec \
        FROM ",hosxp,".opitemrece o \
            INNER JOIN ",hosxp,".drugitems d ON d.icode=o.icode \
            LEFT JOIN ",hosxp,".sp_use sp ON sp.sp_use=o.sp_use \
            LEFT JOIN ",hosxp,".drugusage du ON du.drugusage=o.drugusage \
        WHERE ",w," ORDER BY o.item_no;"
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
