use kphis_util::util::{sanity_alphanumeric, sanity_int};

use kphis_model::search::searchbox::{DrugCheckParams, OpdVisitSearchType};

// common-searchbox-lab-data.php

// SELECT component_caption,component_type,form_name,lab_items_code
// FROM hos.lab_form
// WHERE component_type IN ('checkbox_group','checkbox') AND form_name IN (
//     SELECT form_name FROM hos.lab_form_head WHERE active_status = 'Y'
// ) AND component_caption LIKE '%a%'
// GROUP BY component_caption ORDER BY component_type desc, component_caption ASC;
/// '%' + search_text + '%'
pub fn select_lab_searchbox(hosxp: &str) -> String {
    [
        "SELECT component_caption,component_type,form_name,lab_items_code \
        FROM ",hosxp,".lab_form \
        WHERE component_type IN ('checkbox_group','checkbox') AND form_name IN ( \
            SELECT form_name FROM ",hosxp,".lab_form_head WHERE active_status = 'Y' \
        ) AND component_caption LIKE ? \
        GROUP BY component_caption ORDER BY component_type desc, component_caption ASC;"
    ].concat()
}

// SELECT xi.xray_items_code,xi.xray_items_name,xig.`name` AS group_name
// FROM hos.xray_items xi
// LEFT JOIN hos.xray_items_group xig ON xi.xray_items_group = xig.xray_items_group
// WHERE active_status <> 'N' AND (xi.xray_items_name LIKE ? OR xig.`name` LIKE ?)
// ORDER BY xig.`name`, xi.xray_items_name;
/// '%' + search_text + '%', '%' + search_text + '%'
pub fn select_xray_searchbox(hosxp: &str) -> String {
    [
        "SELECT xi.xray_items_code,xi.xray_items_name,xig.`name` AS group_name \
        FROM ",hosxp,".xray_items xi \
            LEFT JOIN ",hosxp,".xray_items_group xig ON xi.xray_items_group = xig.xray_items_group \
        WHERE xi.active_status <> 'N' AND (xi.xray_items_name LIKE ? OR xig.`name` LIKE ?) \
        ORDER BY xig.`name`, xi.xray_items_name;"
    ].concat()
}

// SELECT icode, CONCAT(`name`, ' ', strength, ' ',units) AS ivfluid_name, displaycolor FROM hos.drugitems
// WHERE dosageform = 'INTRAVENOUS SOLUTION' AND istatus = 'Y' AND (`name` LIKE ? OR drugnote LIKE ?) ORDER BY `name`;
/// '%' + search_text + '%', '%' + search_text + '%'
pub fn select_ivfluid_searchbox(ivfluid: &str, hosxp: &str) -> String {
    [
        "SELECT icode, CONCAT(`name`, ' ', strength, ' ',units) AS ivfluid_name, displaycolor FROM ",hosxp,".drugitems \
        WHERE dosageform = '",ivfluid,"' AND istatus = 'Y' AND (`name` LIKE ? OR drugnote LIKE ?) ORDER BY `name`;"
    ].concat()
}

// SELECT GROUP_CONCAT(DISTINCT(allergy.agent) ORDER BY allergy.agent) AS allergy_agent,
//   GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,
//   SUM(IF(allergy.force_no_order = 'Y',1,0)) AS allergy_count_force_no_order,
//   CONCAT(di.`name`, ' ', di.strength, ' ',di.units) AS med_name,di.icode,di.displaycolor,dud.`usage` AS due_usage,dud.info,dud.info_status,
//   CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,'')),di.generic_name,di.show_notify,di.show_notify_text
// FROM hos.drugitems di
//   LEFT JOIN kphis.kphis_drug_use_duration dud ON dud.icode=di.icode
//   LEFT JOIN hos.drugusage du ON di.drugusage = du.drugusage
//   LEFT JOIN hos.opd_allergy allergy ON (
//     (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=? AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '')
//     OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=? AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> ''))
// WHERE di.icode LIKE '1%' AND di.istatus = 'Y' AND (di.`name` LIKE %?% OR di.drugnote LIKE %?%) GROUP BY di.icode ORDER BY di.`name`;
/// hn, hn, '%' + search_text + '%', '%' + search_text + '%'
pub fn select_med_searchbox(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT GROUP_CONCAT(DISTINCT(allergy.agent) ORDER BY allergy.agent) AS allergy_agent,\
            GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,\
            SUM(IF(allergy.force_no_order = 'Y',1,0)) AS allergy_count_force_no_order,\
            CONCAT(di.`name`, ' ', di.strength, ' ',di.units) AS med_name,di.icode,di.displaycolor,di.dosageform,dud.`usage` AS due_usage,dud.`status` AS due_status,dud.info,dud.info_status,\
            du.drugusage,CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,'')) AS `usage`,di.generic_name,di.show_notify,di.show_notify_text \
        FROM ",hosxp,".drugitems di \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=di.icode \
            LEFT JOIN ",hosxp,".drugusage du ON di.drugusage = du.drugusage \
            LEFT JOIN ",hosxp,".opd_allergy allergy ON (\
                (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=? AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') \
                OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=? AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> '')) \
        WHERE di.icode LIKE '1%' AND di.istatus = 'Y' AND (di.`name` LIKE ? OR di.drugnote LIKE ?) GROUP BY di.icode ORDER BY di.`name`;"
    ].concat()
}

// SELECT NULL AS allergy_agent,NULL AS allergy_agent_symptom,'0' AS allergy_count_force_no_order,
//   CONCAT(di.`name`, ' ', di.strength, ' ',di.units) AS med_name,di.icode,di.displaycolor,di.dosageform,dud.`usage` AS due_usage,dud.info,dud.info_status,
//   du.drugusage,CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,'')) AS `usage`,di.generic_name,di.show_notify,di.show_notify_text
// FROM hos.drugitems di
//   LEFT JOIN kphis.kphis_drug_use_duration dud ON dud.icode=di.icode
//   LEFT JOIN hos.drugusage du ON di.drugusage = du.drugusage
// WHERE di.icode LIKE '1%' AND di.istatus = 'Y' AND (di.`name` LIKE %?% OR di.drugnote LIKE %?%) GROUP BY di.icode ORDER BY di.`name`;
/// '%' + search_text + '%', '%' + search_text + '%'
pub fn select_med_searchbox_without_hn(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT NULL AS allergy_agent,NULL AS allergy_agent_symptom,0.0 AS allergy_count_force_no_order,\
            CONCAT(di.`name`, ' ', di.strength, ' ',di.units) AS med_name,di.icode,di.displaycolor,di.dosageform,dud.`usage` AS due_usage,dud.`status` AS due_status,dud.info,dud.info_status,\
            du.drugusage,CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,'')) AS `usage`,di.generic_name,di.show_notify,di.show_notify_text \
        FROM ",hosxp,".drugitems di \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=di.icode \
            LEFT JOIN ",hosxp,".drugusage du ON di.drugusage = du.drugusage \
        WHERE di.icode LIKE '1%' AND di.istatus = 'Y' AND (di.`name` LIKE ? OR di.drugnote LIKE ?) GROUP BY di.icode ORDER BY di.`name`;"
    ].concat()
}

// // ipd-dr-order-item-drug-duplication-check.php
// SELECT oi.icode,CONCAT(di.`name`, ' ', di.strength, ' ', di.units) AS med_name,oi.order_item_detail,o.order_date,o.order_time
// FROM kphis.ipd_order_item oi
// INNER JOIN kphis.ipd_order o ON o.order_id = oi.order_id
// LEFT JOIN hos.drugitems di ON oi.icode = di.icode
// WHERE o.an = ?
// AND o.order_type = 'continuous' AND oi.order_item_type IN ('med','injection') AND oi.order_item_id NOT IN (
//     SELECT ofi.off_order_item_id FROM ",kphis
//     JOIN kphis.ipd_order off ON ofi.order_id = off.order_id AND (off.order_confirm = 'Y' OR off.order_date = DATE(NOW()))
//     WHERE off.an = o.an AND ofi.off_order_item_id = oi.order_item_id)
// AND di.generic_name = ? ORDER BY med_name;
/// require an and generic_name;
/// an, (exclude_order_id), generic_name
pub fn select_drug_duplication_check(params: &DrugCheckParams, hosxp: &str, kphis: &str) -> String {
    let exclude_order_id = if params.exclude_order_id.is_some() {" AND o.order_id <> ? "} else {""};
    let off_order_item_ids = params.off_order_item_ids.clone().map(|ids| {
        let sanitized = ids.split(',').map(sanity_int).collect::<Vec<String>>().join(",");
        [" AND oi.order_item_id NOT IN (", &sanitized, ") "].concat()
    }).unwrap_or_default();

    [
        "SELECT oi.icode,CONCAT(di.`name`, ' ', di.strength, ' ', di.units) AS med_name,oi.order_item_detail,o.order_date,o.order_time \
        FROM ",kphis,".ipd_order_item oi \
            INNER JOIN ",kphis,".ipd_order o ON o.order_id = oi.order_id \
            LEFT JOIN ",hosxp,".drugitems di ON oi.icode = di.icode \
        WHERE o.an = ? ",exclude_order_id,
        "AND o.order_type = 'continuous' AND oi.order_item_type IN ('med','injection') AND oi.order_item_id NOT IN (\
            SELECT ofi.off_order_item_id FROM ",kphis,".ipd_order_item ofi \
            JOIN ",kphis,".ipd_order off ON ofi.order_id = off.order_id AND (off.order_confirm = 'Y' OR off.order_date = DATE(NOW())) \
            WHERE off.an = o.an AND ofi.off_order_item_id = oi.order_item_id) ", &off_order_item_ids,
        "AND di.generic_name = ? ORDER BY med_name;"
    ].concat()
}

// // ipd-dr-order-item-drug-interaction-check.php
// SELECT drugname1,drugname2,note,severity,not_allow FROM hos.drug_interaction WHERE (drugname1 = 'Amoxy' AND drugname2 IN (
//     SELECT DISTINCT generic_name FROM hos.drugitems WHERE (
//     icode IN (
//       SELECT icode FROM kphis.ipd_order_item oi
//       INNER JOIN kphis.ipd_order o ON o.order_id = oi.order_id
//       WHERE o.an = '66001234'
//         AND o.order_id <> 1
//         AND (
//           (o.order_type = 'oneday' AND order_date = DATE(NOW()))
//           OR o.order_type <> 'oneday'
//         )
//         AND oi.order_item_type = 'med'
//         AND oi.order_item_id NOT IN (
//           SELECT ofi.off_order_item_id
//           FROM kphis.ipd_order_item ofi
//             JOIN kphis.ipd_order off ON ofi.order_id = off.order_id
//               AND (off.order_confirm = 'Y' OR off.order_date = DATE(NOW()))
//           WHERE off.an = o.an
//             AND ofi.off_order_item_id = oi.order_item_id
//         )
//         AND oi.order_item_id NOT IN (1,2)
//         OR icode IN (1,2)
//     ))
// )) OR (drugname2 = 'Amoxy' AND drugname1 IN (
//     SELECT DISTINCT generic_name FROM hos.drugitems WHERE (
//     icode IN (
//         SELECT icode FROM kphis.ipd_order_item oi
//         INNER JOIN kphis.ipd_order o ON o.order_id = oi.order_id
//         WHERE o.an = '66001234'
//         AND o.order_id <> 1
//         AND ( (o.order_type = 'oneday' AND order_date = DATE(NOW()))
//         OR o.order_type <> 'oneday' )
//     AND oi.order_item_type IN ('med','injection')
//     AND oi.order_item_id NOT IN (
//         SELECT ofi.off_order_item_id
//         FROM kphis.ipd_order_item ofi
//         JOIN kphis.ipd_order off ON ofi.order_id = off.order_id
//             AND (off.order_confirm = 'Y' OR off.order_date = DATE(NOW()))
//         WHERE off.an = o.an
//         AND ofi.off_order_item_id = oi.order_item_id
//     )
//     AND oi.order_item_id NOT IN (1,2)
//     OR icode IN (1,2)
// )))) ORDER BY drugname1, drugname2;
/// generic_name, an, (exclude_order_id), generic_name, an, (exclude_order_id)
pub fn select_drug_interaction_check(params: &DrugCheckParams, hosxp: &str, kphis: &str) -> String {
    let exclude_order_id = if params.exclude_order_id.is_some() {" AND o.order_id <> ? "} else {""};
    let off_order_item_ids = params.off_order_item_ids.clone().map(|ids| {
        let sanitized = ids.split(',').map(sanity_int).collect::<Vec<String>>().join(",");
        [" AND oi.order_item_id NOT IN (", &sanitized, ") "].concat()
    }).unwrap_or_default();
    let additional_icodes = params.additional_icodes.clone().map(|icodes| {
        let sanitized = icodes.split(',').map(sanity_alphanumeric).collect::<Vec<String>>().join("','");
        [" OR icode IN ('", &sanitized, "') "].concat()
    }).unwrap_or_default();

    let where_clause = [
        "icode IN (SELECT icode FROM ",kphis,".ipd_order_item oi INNER JOIN ",kphis,".ipd_order o ON o.order_id = oi.order_id WHERE o.an = ? ", exclude_order_id,
            "AND ((o.order_type = 'oneday' AND order_date = DATE(NOW())) OR o.order_type <> 'oneday') AND oi.order_item_type IN ('med','injection') AND oi.order_item_id NOT IN ( \
                SELECT ofi.off_order_item_id FROM kphis.ipd_order_item ofi JOIN kphis.ipd_order off ON ofi.order_id = off.order_id AND (off.order_confirm = 'Y' OR off.order_date = DATE(NOW())) \
                WHERE off.an = o.an AND ofi.off_order_item_id = oi.order_item_id) ", &off_order_item_ids, &additional_icodes,")"
    ].concat();

    [
        "SELECT drugname1,drugname2,note,severity,not_allow FROM ",hosxp,".drug_interaction \
        WHERE (drugname1 = ? AND drugname2 IN (SELECT DISTINCT generic_name FROM ",hosxp,".drugitems WHERE (",&where_clause,"))) \
            OR (drugname2 = ? AND drugname1 IN (SELECT DISTINCT generic_name FROM ",hosxp,".drugitems WHERE (",&where_clause,"))) \
        ORDER BY drugname1, drugname2;"
    ].concat()
}

// SELECt hn,concat(TRIM(pname),TRIM(fname),' ',TRIM(lname)) AS ptname, fathername, mathername, admit, passport_no, cid
// FROM hos.patient
// ORDER BY hn, fname, lname LIMIT 100;
/// % + search_text + %
pub fn select_patient_searchbox(text: &str, hosxp_hn_len: usize, hosxp: &str) -> String {
    let where_sql = if text.parse::<u64>().is_ok() {
        if text.len() <= hosxp_hn_len {"WHERE hn LIKE ? "} else {"WHERE cid LIKE ? "}
    } else {
        "WHERE CONCAT(pname,fname,' ',lname) LIKE ? "
    };
    [
        "SELECT hn,CONCAT(TRIM(pname),TRIM(fname),' ',TRIM(lname)) AS ptname,fathername,mathername,admit,passport_no,cid \
        FROM ",hosxp,".patient ",where_sql, " ORDER BY hn,fname,lname LIMIT 100;"
    ].concat()
}

// SELECT o.vn,o.hn,o.vstdate,o.vsttime,CONCAT(TRIM(p.pname),TRIM(p.fname),' ',TRIM(p.lname)) AS ptname,p.fathername,p.mathername,p.admit,p.passport_no,p.cid
// FROM hos.ovst o LEFT JOIN hos.patient p ON o.hn = p.hn
// ORDER BY o.vstdate DESC,o.vsttime DESC,o.hn,o.vn DESC,p.fname,p.lname LIMIT 100;
/// mode != All{(search_text)}
pub fn select_opd_visit_searchbox(mode: &OpdVisitSearchType, hosxp: &str) -> String {
    let where_clause = match mode {
        OpdVisitSearchType::Hn => " WHERE o.hn LIKE ? ",
        OpdVisitSearchType::Qn => " WHERE o.vstdate = DATE(NOW()) AND o.oqueue = ? ",
        OpdVisitSearchType::Vn => " WHERE o.vn LIKE ? ",
        OpdVisitSearchType::PtName => " WHERE CONCAT(p.pname,p.fname,' ',p.lname) LIKE ? ",
        OpdVisitSearchType::Cid => " WHERE p.cid LIKE ? ",
        OpdVisitSearchType::All => "",
    };

    [
        "SELECT o.vn,o.hn,o.vstdate,o.vsttime,CONCAT(TRIM(p.pname),TRIM(p.fname),' ',TRIM(p.lname)) AS ptname,p.fathername,p.mathername,p.admit,p.passport_no,p.cid \
        FROM ",hosxp,".ovst o LEFT JOIN ",hosxp,".patient p ON o.hn = p.hn ",
        where_clause," ORDER BY o.vstdate DESC,o.vsttime DESC,o.hn,o.vn DESC,p.fname,p.lname LIMIT 100;"
    ].concat()
}

// ipd-summary-2-dx-data.php
// SELECT icd101.`code` AS icd10,icd101.`name` AS ename,icd101.tname,
//     GROUP_CONCAT(DISTINCT sumdx.keyword) AS sumdx_keyword,
//     GROUP_CONCAT(DISTINCT icd_codemap.`code`) AS icd_codemap_code
// FROM hos.icd101
//     LEFT JOIN kphis.ipd_summary_dx sumdx ON icd101.`code`=sumdx.icd10
//     LEFT JOIN hos.icd_codemap ON icd101.`code`=icd_codemap.icd10
// GROUP BY icd101.`name`;
pub fn select_icd10_searchbox(is_external_cause: bool, hosxp: &str, kphis: &str) -> String {
    let (select_name, is_ext) = if is_external_cause {
        (["CASE \
		    WHEN LENGTH(i.`code`)=5 THEN CONCAT((SELECT `name` FROM ",hosxp,".icd101 WHERE `code`=SUBSTRING(i.`code`,1,3)),' - ',(SELECT `name` FROM ",hosxp,".icd101 WHERE `code`=SUBSTRING(i.`code`,1,4)),' - ',i.`name`) \
		    WHEN LENGTH(i.`code`)=4 THEN CONCAT((SELECT `name` FROM ",hosxp,".icd101 WHERE `code`=SUBSTRING(i.`code`,1,3)),' - ',i.`name`) \
		    ELSE i.`name` \
	    END"].concat(),"")
    } else {
        (["IF(i.`code` REGEXP '^[M]' AND LENGTH(i.`code`)=5,CONCAT((SELECT `name` FROM ",hosxp,".icd101 WHERE `code`=SUBSTRING(i.`code`,1,4)),' - ',i.`name`),i.`name`)"].concat()," NOT")
    };
    [
        "SELECT i.`code` AS icd10,",&select_name," AS ename,i.tname,\
            GROUP_CONCAT(DISTINCT sumdx.keyword) AS sumdx_keyword,\
            GROUP_CONCAT(DISTINCT icd_codemap.`code`) AS icd_codemap_code \
        FROM ",hosxp,".icd101 i \
            LEFT JOIN ",kphis,".ipd_summary_dx sumdx ON i.`code`=sumdx.icd10 \
            LEFT JOIN ",hosxp,".icd_codemap ON i.`code`=icd_codemap.icd10 \
        WHERE i.`code`",is_ext," REGEXP '^[VWXY]' GROUP BY i.`code` ORDER BY i.`code`;"
    ].concat()
}

// // ipd-summary-2-hospcode-data.php
// // SELECT h.hospcode AS `id`,CONCAT(h.hospcode,' ',h.hosptype,' ',h.NAME ) AS `text`,CONCAT(t1.NAME,' ',t2.NAME ) AS `addrname`
// SELECT h.hospcode AS `id`,h.hosptype,h.NAME AS hospname,CONCAT(t1.NAME,' ',t2.NAME ) AS `addrname`
// FROM hos.hospcode h
//     LEFT JOIN hos.thaiaddress t1 ON t1.chwpart=h.chwpart AND t1.codetype='1'
//     LEFT JOIN hos.thaiaddress t2 ON t2.chwpart=h.chwpart AND t2.amppart=h.amppart AND t2.codetype='2'
// WHERE h.NAME LIKE ? OR CONCAT( h.chwpart, h.amppart ) LIKE ? OR h.hospcode LIKE ?
// ORDER BY h.hospcode LIMIT ? OFFSET ?;
/// %search%, %search%, %search%, limit, offset
pub fn select_hosp_searchbox(hosxp: &str) -> String {
    [
        "SELECT h.hospcode AS `id`,h.hosptype,h.NAME AS hospname,CONCAT(t1.NAME,' ',t2.NAME ) AS `addrname` \
        FROM ",hosxp,".hospcode h \
            LEFT JOIN ",hosxp,".thaiaddress t1 ON t1.chwpart=h.chwpart AND t1.codetype='1' \
            LEFT JOIN ",hosxp,".thaiaddress t2 ON t2.chwpart=h.chwpart AND t2.amppart=h.amppart AND t2.codetype='2' \
        WHERE h.NAME LIKE ? OR CONCAT( h.chwpart, h.amppart ) LIKE ? OR h.hospcode LIKE ? \
        ORDER BY h.hospcode LIMIT ? OFFSET ?;"
    ].concat()
}

// SELECT drugusage,`code`,name1,name2,name3 FROM hos.drugusage WHERE status='Y';
pub fn select_drugusage_searchbox(hosxp: &str) -> String {
    [
        "SELECT drugusage,`code`,name1,name2,name3 FROM ",hosxp,".drugusage WHERE status='Y' ORDER BY `code`;"
    ].concat()
}