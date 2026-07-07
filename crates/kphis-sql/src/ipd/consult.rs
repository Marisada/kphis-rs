use std::cmp::Ordering;

use kphis_model::ipd::consult::IpdConsultListParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

pub fn sql_and_filter(params: IpdConsultListParams, hlen: usize, alen: usize, hosxp: &str, kphis: &str) -> (String, ConsultFilter) {
    let not_empty = params.not_empty();
    let has_patient = not_empty.patient.is_some();
    let select = select(hosxp, kphis);

    let (where_patient, filter) = match not_empty.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        Some(patient) => {
            let filter = ConsultFilter::default().has_patient();
            let (where_pt, filter) = match patient.parse::<u64>().is_ok() {
                true => {
                    let pt_len = patient.len();
                    if pt_len == 13 {
                        (" AND patient.cid=?", filter.pt_is_num()) // patient
                    } else {
                        match hlen.cmp(&alen) {
                            Ordering::Equal => (" AND ipt.hn LIKE ? OR ipt.an LIKE ?", filter.anlen_eq_hnlen()), // patient, patient
                            Ordering::Less => match pt_len.cmp(&hlen) {
                                Ordering::Greater => (" AND ipt.an LIKE ?", filter),                // patient
                                Ordering::Equal | Ordering::Less => (" AND ipt.hn LIKE ?", filter), // patient
                            },
                            Ordering::Greater => match pt_len.cmp(&alen) {
                                Ordering::Greater => (" AND ipt.hn LIKE ?", filter),                // patient
                                Ordering::Equal | Ordering::Less => (" AND ipt.an LIKE ?", filter), // patient
                            },
                        }
                    }
                }
                false => (" AND CONCAT(CONCAT(CONCAT(TRIM(patient.pname),TRIM(patient.fname)),' '),TRIM(patient.lname)) LIKE ?", filter), // patient
            };
            (where_pt.to_owned(), filter)
        }
        None => {
            let where_no_pt = String::from(" AND ipt.dchstts IS NULL");
            let filter = ConsultFilter::default();
            let (where_no_pt, filter) = match not_empty.spclty {
                Some(_spclty) => ([&where_no_pt, " AND ipd_c.consult_spclty=?"].concat(), filter.has_spclty()), // spclty
                None => (where_no_pt, filter),
            };
            let (where_no_pt, filter) = match not_empty.search_consult_status {
                Some(_search_consult_status) => ([&where_no_pt, " AND ipd_c.consult_status=?"].concat(), filter.has_search_consult_status()), // search_consult_status
                None => (where_no_pt, filter),
            };
            let (where_no_pt, filter) = match not_empty.consult_dr_search {
                Some(_consult) => ([&where_no_pt, " AND ipd_c.consult_doctorcode_mention=?"].concat(), filter.has_consult()), // consult
                None => (where_no_pt, filter),
            };
            let (where_no_pt, filter) = match not_empty.consult_dr_reply_search {
                Some(_consult_dr_reply_search) => (
                    [&where_no_pt, " AND (idc_reply.consult_doctorcode_reply=? OR idc_reply.consult_doctorcode_reply_person2=?)"].concat(),
                    filter.has_consult_dr_reply_search(),
                ), // consult_dr_reply_search, consult_dr_reply_search
                None => (where_no_pt, filter),
            };
            match not_empty.search_consult_emergency {
                Some(_search_consult_emergency) => ([&where_no_pt, " AND ipd_c.consult_emergency=?"].concat(), filter.has_search_consult_emergency()), // search_consult_emergency
                None => (where_no_pt, filter),
            }
        }
    };
    let where_with_order = match has_patient {
        true => [&where_patient, " GROUP BY ipt.an,ipd_c.consult_id ORDER BY ipt.an DESC LIMIT 200;"].concat(),
        false => [
            &where_patient,
            " GROUP BY ipt.an,ipd_c.consult_id ORDER BY ipd_c.consult_status ASC,ipd_c.consult_emergency ASC,ipd_c.consult_date ASC,ipd_c.consult_time ASC;",
        ]
        .concat(),
    };

    ([select, where_with_order].concat(), filter)
}

#[derive(Debug, Default)]
pub struct ConsultFilter {
    pub has_patient: bool,
    pub pt_is_num: bool,
    pub anlen_eq_hnlen: bool,
    pub has_spclty: bool,
    pub has_search_consult_status: bool,
    pub has_consult: bool,
    pub has_consult_dr_reply_search: bool,
    pub has_search_consult_emergency: bool,
}
impl ConsultFilter {
    fn has_patient(self) -> Self {
        Self { has_patient: true, ..self }
    }
    fn pt_is_num(self) -> Self {
        Self { pt_is_num: true, ..self }
    }
    fn anlen_eq_hnlen(self) -> Self {
        Self { anlen_eq_hnlen: true, ..self }
    }
    fn has_spclty(self) -> Self {
        Self { has_spclty: true, ..self }
    }
    fn has_search_consult_status(self) -> Self {
        Self {
            has_search_consult_status: true,
            ..self
        }
    }
    fn has_consult(self) -> Self {
        Self { has_consult: true, ..self }
    }
    fn has_consult_dr_reply_search(self) -> Self {
        Self {
            has_consult_dr_reply_search: true,
            ..self
        }
    }
    fn has_search_consult_emergency(self) -> Self {
        Self {
            has_search_consult_emergency: true,
            ..self
        }
    }
}

// // ipd-consult-list-table.php
// SELECT ipt.hn,ipt.an,iptadm.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.income AS income,aa.admdate,aa.age_y,aa.age_m,aa.age_d,
//     GROUP_CONCAT(d3.name SEPARATOR ', ') AS kphis_incharge_doctor_name,
//     (SELECT GROUP_CONCAT(CONCAT(doctor_reply.`name`,IF(doctor_reply2.`name` IS NULL,'',CONCAT(' / ',doctor_reply2.`name`)))
//         ORDER BY idc_reply.consult_reply_id ASC SEPARATOR ',') AS string_consult_reply_name
//     FROM kphis.ipd_dr_consult ipc
//         LEFT JOIN kphis.ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=ipc.consult_id
//         LEFT JOIN hos.doctor doctor_reply ON doctor_reply.`code`=idc_reply.consult_doctorcode_reply
//         LEFT JOIN hos.doctor doctor_reply2 ON doctor_reply2.`code`=idc_reply.consult_doctorcode_reply_person2
//     WHERE ipc.consult_id=ipd_c.consult_id) AS string_consult_reply_name,
//     s.spclty_name AS spclty_name,d4.`name` AS consult_doctorcode_mention_name,w.`name` AS ward_name,
//     ipd_c.consult_status,ipd_c.consult_date,ipd_c.consult_time,ipd_c.consult_emergency,ipd_c.consult_datetime_create_reply,ipd_c.consult_datetime_update_reply
// FROM hos.ipt
//     LEFT JOIN hos.iptadm ON iptadm.an=ipt.an
//     LEFT JOIN hos.patient ON patient.hn=ipt.hn
//--   LEFT JOIN hos.roomno ON roomno.roomno=iptadm.roomno
//     LEFT JOIN hos.an_stat aa ON aa.an=ipt.an
//     LEFT JOIN kphis.ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on'
//     LEFT JOIN hos.doctor d3 ON d3.`code`=ipd_dr.doctor
//     LEFT JOIN kphis.ipd_dr_consult ipd_c ON ipd_c.an=ipt.an
//     LEFT JOIN hos.doctor d4 ON d4.`code`=ipd_c.consult_doctorcode_mention
//     LEFT JOIN kphis.kphis_spclty s ON s.spclty_id=ipd_c.consult_spclty
//     LEFT JOIN hos.ward w ON w.ward=ipt.ward
//     LEFT JOIN kphis.ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=ipd_c.consult_id
pub fn select(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT ipd_c.consult_id,ipt.hn,ipt.an,iptadm.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.income AS income,aa.admdate,aa.age_y,aa.age_m,aa.age_d,\
            GROUP_CONCAT(d3.name SEPARATOR ', ') AS kphis_incharge_doctor_name,ADDTIME(CONVERT(ipt.regdate,DATETIME),ipt.regtime) AS regdatetime,ptt.pcode AS rtcode,ptt.name AS rtname,sex.`name` AS sex_name,\
            (SELECT GROUP_CONCAT(CONCAT(doctor_reply.`name`,IF(doctor_reply2.`name` IS NULL,'',CONCAT(' / ',doctor_reply2.`name`))) \
                ORDER BY idc_reply.consult_reply_id ASC SEPARATOR ',') AS string_consult_reply_name \
            FROM ",kphis,".ipd_dr_consult ipc \
                LEFT JOIN ",kphis,".ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=ipc.consult_id \
                LEFT JOIN ",hosxp,".doctor doctor_reply ON doctor_reply.`code`=idc_reply.consult_doctorcode_reply \
                LEFT JOIN ",hosxp,".doctor doctor_reply2 ON doctor_reply2.`code`=idc_reply.consult_doctorcode_reply_person2 \
            WHERE ipc.consult_id=ipd_c.consult_id) AS string_consult_reply_name,\
            s.spclty_name AS spclty_name,d4.`name` AS consult_doctorcode_mention_name,w.`name` AS ward_name,\
            ipd_c.consult_status,ipd_c.consult_date,ipd_c.consult_time,ipd_c.consult_emergency,ipd_c.consult_datetime_create_reply,ipd_c.consult_datetime_update_reply \
        FROM ",kphis,".ipd_dr_consult ipd_c \
            LEFT JOIN ",hosxp,".ipt ON ipt.an=ipd_c.an \
            LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
            LEFT JOIN ",hosxp,".patient ON patient.hn=ipt.hn \
            LEFT JOIN ",hosxp,".sex ON sex.code=patient.sex \
            LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ipt.pttype \
            LEFT JOIN ",hosxp,".an_stat aa ON aa.an=ipt.an \
            LEFT JOIN ",kphis,".ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on' \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=ipd_dr.doctor \
            LEFT JOIN ",hosxp,".doctor d4 ON d4.`code`=ipd_c.consult_doctorcode_mention \
            LEFT JOIN ",kphis,".kphis_spclty s ON s.spclty_id=ipd_c.consult_spclty \
            LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
            LEFT JOIN ",kphis,".ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=ipd_c.consult_id \
        WHERE 1=1"
    ].concat()
}

// // ipd-dr-consult-data.php
// // we merge 3 queries into 1 query
// // 1st main query
// SELECT idc.*,spclty.spclty_name AS spcltyname,idc_type.consult_type_name,emer.emergency_name AS consult_emergency_name,doctor.`name` AS consult_doctorcode_mention_name,
// FROM kphis.ipd_dr_consult idc
//     LEFT JOIN kphis.kphis_spclty spclty ON spclty.spclty_id=idc.consult_spclty
//     LEFT JOIN kphis.ipd_dr_consult_type idc_type ON idc_type.consult_type_id=idc.consult_type
//     LEFT JOIN kphis.ipd_emergency emer ON emer.emergency_id=idc.consult_emergency
//     LEFT JOIN kphis.doctor ON doctor.`code`=idc.consult_doctorcode_mention
// WHERE idc.an=? ORDER BY idc.consult_status ASC, idc.consult_emergency ASC, idc.consult_date ASC, idc.consult_time ASC;
// // 2nd sub query
// SELECT GROUP_CONCAT(CONCAT(doctor_request.`name`,IF(doctor_request2.`name` IS NULL,'',CONCAT(' / ',doctor_request2.`name`)))
//     ORDER BY idc_request.consult_signature_id ASC SEPARATOR ',')  AS string_consult_request_name
// FROM kphis.ipd_dr_consult idc
//     LEFT JOIN kphis.ipd_dr_consult_signature_request idc_request ON idc_request.consult_id = idc.consult_id
//     LEFT JOIN hos.doctor doctor_request ON doctor_request.`code` = idc_request.consult_doctorcode_request
//     LEFT JOIN hos.doctor doctor_request2 ON doctor_request2.`code` = idc_request.consult_doctorcode_request_person2
// WHERE idc.consult_id = :consult_id;
// // 3rd sub query
// SELECT COUNT(idc_reply.consult_doctorcode_reply) AS count_consult_doctorcode_reply,
//     GROUP_CONCAT(CONCAT(doctor_reply.`name`,IF(doctor_reply2.`name` IS NULL, '',CONCAT(' / ',doctor_reply2.`name`))) ORDER BY idc_reply.consult_reply_id ASC SEPARATOR ', ') AS string_consult_reply_name,
//     idc_reply.create_datetime AS reply_create_datetime,
//     idc_reply.update_datetime AS reply_update_datetime
// FROM kphis.ipd_dr_consult idc
//     LEFT JOIN kphis.ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id = idc.consult_id
//     LEFT JOIN hos.doctor doctor_reply ON doctor_reply.`code` = idc_reply.consult_doctorcode_reply
//     LEFT JOIN hos.doctor doctor_reply2 ON doctor_reply2.`code` = idc_reply.consult_doctorcode_reply_person2
// WHERE idc.consult_id = :consult_id;
// // by changing
// // 1. htmls below will render at client
// // from 2nd sub query
// GROUP_CONCAT(CONCAT('<div  class=\"text-sm text-truncate\" style=\"max-width:240px\">',doctor_request.`name`, IF(doctor_request2.`name` IS NULL, '',CONCAT(' / ',doctor_request2.`name`)), '</div>')
//     ORDER BY idc_request.consult_signature_id ASC SEPARATOR '') AS html_consult_request_name,
// // from 3rd sub query
// GROUP_CONCAT(CONCAT('<div  class=\"text-sm text-truncate\" style=\"max-width:172px\">',doctor_reply.`name`, IF(doctor_reply2.`name` IS NULL, '',CONCAT(' / ',doctor_reply2.`name`)), '</div>')
//     ORDER BY idc_reply.consult_reply_id ASC SEPARATOR '') AS html_consult_reply_name,
// // 2. subquery will concat row with pipe(|) and concat column with cap(^)
// // *** but reply_create_datetime and reply_update_datetime NOT USED so we comment old cap+pipe code ***
// SELECT idc.*,spclty.spclty_name AS spcltyname,idc_type.consult_type_name,emer.emergency_name AS consult_emergency_name,doctor.`name` AS consult_doctorcode_mention_name,
// (SELECT GROUP_CONCAT(CONCAT(doctor_request.`name`,IF(doctor_request2.`name` IS NULL,'',CONCAT(' / ',doctor_request2.`name`))) ORDER BY idc_request.consult_signature_id ASC SEPARATOR ',')
//     FROM kphis.ipd_dr_consult idc_rq
//         LEFT JOIN kphis.ipd_dr_consult_signature_request idc_request ON idc_request.consult_id=idc_rq.consult_id
//         LEFT JOIN hos.doctor doctor_request ON doctor_request.`code`=idc_request.consult_doctorcode_request
//         LEFT JOIN hos.doctor doctor_request2 ON doctor_request2.`code`=idc_request.consult_doctorcode_request_person2
//     WHERE idc_rq.consult_id=idc.consult_id) AS string_consult_request_name,
// -- (SELECT GROUP_CONCAT(CONCAT(doctor_reply.`name`,IF(doctor_reply2.`name` IS NULL,'',CONCAT(' / ',doctor_reply2.`name`)),'^',idc_reply.create_datetime,'^',idc_reply.update_datetime) ORDER BY idc_reply.consult_reply_id ASC SEPARATOR '|')
// (SELECT GROUP_CONCAT(CONCAT(doctor_reply.`name`,IF(doctor_reply2.`name` IS NULL,'',CONCAT(' / ',doctor_reply2.`name`))) ORDER BY idc_reply.consult_reply_id ASC SEPARATOR ',')
//     FROM kphis.ipd_dr_consult idc_rp
//         LEFT JOIN kphis.ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=idc_rp.consult_id
//         LEFT JOIN hos.doctor doctor_reply ON doctor_reply.`code`=idc_reply.consult_doctorcode_reply
//         LEFT JOIN hos.doctor doctor_reply2 ON doctor_reply2.`code`=idc_reply.consult_doctorcode_reply_person2
//     WHERE idc_rp.consult_id=idc.consult_id) AS string_consult_reply_name
// FROM kphis.ipd_dr_consult idc
//     LEFT JOIN kphis.kphis_spclty spclty ON spclty.spclty_id=idc.consult_spclty
//     LEFT JOIN kphis.ipd_dr_consult_type idc_type ON idc_type.consult_type_id=idc.consult_type
//     LEFT JOIN kphis.ipd_emergency emer ON emer.emergency_id=idc.consult_emergency
//     LEFT JOIN hos.doctor ON doctor.`code`=idc.consult_doctorcode_mention
// WHERE idc.an=? ORDER BY idc.consult_status ASC, idc.consult_emergency ASC, idc.consult_date ASC, idc.consult_time ASC;
/// an
pub fn select_consult_by_an(hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT idc.*,spclty.spclty_name AS spcltyname,idc_type.consult_type_name,emer.emergency_name AS consult_emergency_name,doctor.`name` AS consult_doctorcode_mention_name,\
        (SELECT GROUP_CONCAT(CONCAT(doctor_request.`name`,IF(doctor_request2.`name` IS NULL,'',CONCAT(' / ',doctor_request2.`name`))) ORDER BY idc_request.consult_signature_id ASC SEPARATOR ',') \
            FROM ",kphis,".ipd_dr_consult idc_rq \
                LEFT JOIN ",kphis,".ipd_dr_consult_signature_request idc_request ON idc_request.consult_id=idc_rq.consult_id \
                LEFT JOIN ",hosxp,".doctor doctor_request ON doctor_request.`code`=idc_request.consult_doctorcode_request \
                LEFT JOIN ",hosxp,".doctor doctor_request2 ON doctor_request2.`code`=idc_request.consult_doctorcode_request_person2 \
            WHERE idc_rq.consult_id=idc.consult_id) AS string_consult_request_name,\
        (SELECT GROUP_CONCAT(CONCAT(doctor_reply.`name`,IF(doctor_reply2.`name` IS NULL,'',CONCAT(' / ',doctor_reply2.`name`))) ORDER BY idc_reply.consult_reply_id ASC SEPARATOR ',') \
            FROM ",kphis,".ipd_dr_consult idc_rp \
                LEFT JOIN ",kphis,".ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=idc_rp.consult_id \
                LEFT JOIN ",hosxp,".doctor doctor_reply ON doctor_reply.`code`=idc_reply.consult_doctorcode_reply \
                LEFT JOIN ",hosxp,".doctor doctor_reply2 ON doctor_reply2.`code`=idc_reply.consult_doctorcode_reply_person2 \
            WHERE idc_rp.consult_id=idc.consult_id) AS string_consult_reply_name,\
        (SELECT GROUP_CONCAT(i.`path` ORDER BY u.image_usage_id) FROM ",kphis_extra,".image AS i \
  		    LEFT JOIN ",kphis_extra,".image_usage AS u ON u.image_id=i.image_id WHERE u.usage_id=1 AND u.usage_key_id=idc.consult_id) AS d_imgs,\
        (SELECT GROUP_CONCAT(i.`path` ORDER BY u.image_usage_id) FROM ",kphis_extra,".image AS i \
  		    LEFT JOIN ",kphis_extra,".image_usage AS u ON u.image_id=i.image_id WHERE u.usage_id=1 AND u.usage_key_id=idc.consult_id) AS f_imgs \
        FROM ",kphis,".ipd_dr_consult idc \
            LEFT JOIN ",kphis,".kphis_spclty spclty ON spclty.spclty_id=idc.consult_spclty \
            LEFT JOIN ",kphis,".ipd_dr_consult_type idc_type ON idc_type.consult_type_id=idc.consult_type \
            LEFT JOIN ",kphis,".ipd_emergency emer ON emer.emergency_id=idc.consult_emergency \
            LEFT JOIN ",hosxp,".doctor ON doctor.`code`=idc.consult_doctorcode_mention \
        WHERE idc.an=? ORDER BY idc.consult_status ASC, idc.consult_emergency ASC, idc.consult_date ASC, idc.consult_time ASC;"
    ].concat()
}

// // ipd-dr-consult-edit.php
// // 1st query
// SELECT * FROM kphis.ipd_dr_consult WHERE consult_id = ?;
// // 2nd query
// SELECT consult_r.consult_doctorcode_request, doctor.`name` AS consult_doctorcode_request_name, consult_doctorcode_request_person2
// FROM kphis.ipd_dr_consult_signature_request consult_r
//     LEFT JOIN hos.doctor ON doctor.code=consult_r.consult_doctorcode_request
// WHERE consult_r.consult_id=:consult_id;
// // 3rd query
// SELECT consult_r.consult_doctorcode_reply, doctor.`name` AS consult_doctorcode_reply_name, consult_doctorcode_reply_person2
// FROM kphis.ipd_dr_consult_signature_reply consult_r
//     LEFT JOIN hos.doctor ON doctor.code=consult_r.consult_doctorcode_reply
// WHERE consult_r.consult_id=:consult_id;
// // merge into
// SELECT idc.*,
// (SELECT GROUP_CONCAT(CONCAT(crq.consult_doctorcode_request,'^',doctor.`name`,'^',IF(crq.consult_doctorcode_request_person2 IS NULL,'',crq.consult_doctorcode_request_person2)) SEPARATOR '|')
//     FROM kphis.ipd_dr_consult_signature_request crq LEFT JOIN hos.doctor ON doctor.code=crq.consult_doctorcode_request
//     WHERE crq.consult_id=idc.consult_id) AS string_consult_request_name,
// (SELECT GROUP_CONCAT(CONCAT(crp.consult_doctorcode_reply,'^',doctor.`name`,'^',IF(crp.consult_doctorcode_reply_person2 IS NULL,'',crp.consult_doctorcode_reply_person2)) SEPARATOR '|')
//     FROM kphis.ipd_dr_consult_signature_reply crp LEFT JOIN hos.doctor ON doctor.code=crp.consult_doctorcode_reply
//     WHERE crp.consult_id=idc.consult_id) AS string_consult_reply_name
// FROM kphis.ipd_dr_consult idc WHERE idc.consult_id=?;
/// consult_id
pub fn select_consult_by_id(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT idc.*,\
        (SELECT GROUP_CONCAT(CONCAT(crq.consult_doctorcode_request,'^',doctor.`name`,'^',IF(crq.consult_doctorcode_request_person2 IS NULL,'',crq.consult_doctorcode_request_person2)) SEPARATOR '|') \
            FROM ",kphis,".ipd_dr_consult_signature_request crq LEFT JOIN ",hosxp,".doctor ON doctor.code=crq.consult_doctorcode_request \
            WHERE crq.consult_id=idc.consult_id) AS string_consult_request_name,\
        (SELECT GROUP_CONCAT(CONCAT(crp.consult_doctorcode_reply,'^',doctor.`name`,'^',IF(crp.consult_doctorcode_reply_person2 IS NULL,'',crp.consult_doctorcode_reply_person2)) SEPARATOR '|') \
            FROM ",kphis,".ipd_dr_consult_signature_reply crp LEFT JOIN ",hosxp,".doctor ON doctor.code=crp.consult_doctorcode_reply \
            WHERE crp.consult_id=idc.consult_id) AS string_consult_reply_name \
        FROM ",kphis,".ipd_dr_consult idc WHERE idc.consult_id=?;"
    ].concat()
}

// // ipd-dr-consult-delete.php
// DELETE FROM kphis.ipd_dr_consult WHERE consult_id=?;
/// consult_id
pub fn delete_consult(kphis: &str) -> String {
    ["DELETE FROM ", kphis, ".ipd_dr_consult WHERE consult_id=?;"].concat()
}

// DELETE FROM kphis.ipd_dr_consult_signature_request WHERE consult_id=?;
/// consult_id
pub fn delete_consult_signature_request(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_dr_consult_signature_request WHERE consult_id=?;"
    ].concat()
}

// // ipd-dr-consult-update.php
// DELETE FROM kphis.ipd_dr_consult_signature_reply WHERE consult_id=?;
/// consult_id
pub fn delete_consult_signature_reply(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_dr_consult_signature_reply WHERE consult_id=?;"
    ].concat()
}

// // ipd-dr-consult-save.php
// INSERT INTO kphis.ipd_dr_consult (consult_type,consult_ward,consult_emergency,consult_doctorcode_mention,consult_spclty,
//     consult_date,consult_time,consult_data,consult_status,an,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// consult_type, consult_ward, consult_emergency, consult_doctorcode_mention, consult_spclty, consult_date, consult_time, consult_data, consult_status, an, loginname, loginname
pub fn insert_consult_request(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_dr_consult (consult_type,consult_ward,consult_emergency,consult_doctorcode_mention,consult_spclty,\
            consult_date,consult_time,consult_data,consult_status,an",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // ipd-dr-consult-save.php + ipd-dr-consult-update.php
// each consult_doctorcode_request
// INSERT INTO kphis.ipd_dr_consult_signature_request (consult_id,consult_doctorcode_request,consult_doctorcode_request_person2,an,
//     create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,?,?,?,NOW(),?,NOW(),1);
/// consult_id, consult_doctorcode_request, consult_doctorcode_request_person2, an, loginname, loginname
pub fn insert_consult_signature_request(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_dr_consult_signature_request (consult_id,consult_doctorcode_request,consult_doctorcode_request_person2,an",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // ipd-dr-consult-update.php
// // edit mode, we add ` AND consult_status='N' AND version=?`
// UPDATE kphis.ipd_dr_consult SET consult_type=?,consult_ward=?,consult_emergency=?,consult_doctorcode_mention=?,consult_spclty=?,
//     consult_date=?,consult_time=?,consult_data=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE consult_id=? AND consult_status='N';
/// consult_type, consult_ward, consult_emergency, consult_doctorcode_mention, consult_spclty, consult_date, consult_time, consult_data, loginname, consult_id, version
pub fn update_consult_request(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_dr_consult SET consult_type=?,consult_ward=?,consult_emergency=?,consult_doctorcode_mention=?,consult_spclty=?,\
            consult_date=?,consult_time=?,consult_data=?",TABLE_UPDATE_SET," WHERE consult_id=? AND consult_status='N' AND version=?;"
    ].concat()
}
// // reply mode
// UPDATE kphis.ipd_dr_consult SET consult_datetime_create_reply=?,consult_datetime_update_reply=?,consult_finding=?,consult_diagnosis=?,
//     consult_recommendation=?,consult_status=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE consult_id=?;
/// consult_datetime_create_reply, consult_datetime_update_reply, consult_finding, consult_diagnosis, consult_recommendation, consult_status, loginname, consult_id, version
pub fn update_consult_reply(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_dr_consult SET consult_datetime_create_reply=?,consult_datetime_update_reply=?,consult_finding=?,consult_diagnosis=?,\
            consult_recommendation=?,consult_status=?",TABLE_UPDATE_SET," WHERE consult_id=? AND version=?;"
    ].concat()
}

// // ipd-dr-consult-update.php
// INSERT INTO kphis.ipd_dr_consult_signature_reply (consult_id,consult_doctorcode_reply,consult_doctorcode_reply_person2,an,
//     create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,?,?,?,NOW(),?,NOW(),1);
/// consult_id, consult_doctorcode_reply, consult_doctorcode_reply_person2, an, loginname, loginname
pub fn insert_consult_signature_reply(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_dr_consult_signature_reply (consult_id,consult_doctorcode_reply,consult_doctorcode_reply_person2,an",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // ipd-dr-consult-delete.php
// INSERT INTO kphis.history_ipd_dr_consult
//     SELECT NULL,NOW(),'D',?,ipd_dr_consult.* FROM kphis.ipd_dr_consult WHERE consult_id=? AND consult_status='N';
// // ipd-dr-consult-save.php
// INSERT INTO kphis.history_ipd_dr_consult
//     SELECT NULL,NOW(),'I',?, ipd_dr_consult.* FROM kphis.ipd_dr_consult WHERE consult_id=?;
// // ipd-dr-consult-update.php
// INSERT INTO kphis.history_ipd_dr_consult
//     SELECT NULL,NOW(),'U',?,ipd_dr_consult.* FROM kphis.ipd_dr_consult WHERE consult_id=?
// /// loginname, consult_id, (where_update_user => loginname)
// pub fn insert_consult_to_history(history_type: &str, where_status_n: bool, where_update_user: bool, kphis: &str) -> String {
//     let consult_status = if where_status_n {" AND consult_status='N'"} else {""};
//     let update_user = if where_update_user {" AND update_user=?"} else {""};
//     [
//         "INSERT INTO ",kphis,".history_ipd_dr_consult \
//             SELECT NULL,NOW(),'",history_type,"',?,ipd_dr_consult.* FROM ",kphis,".ipd_dr_consult WHERE consult_id=?",consult_status,update_user,";"
//     )
// }
// /// loginname, consult_id
// pub fn insert_consult_signature_to_history(kphis: &str) -> String {
//     [
//         "INSERT INTO ",kphis,".history_ipd_dr_consult_signature_request \
//             SELECT NULL,NOW(),'D',?,ipd_dr_consult_signature_request.* FROM ",kphis,".ipd_dr_consult_signature_request WHERE consult_id=?;"
//     )
// }

// // ipd-dr-consult-delete.php
// INSERT INTO kphis.history_ipd_dr_consult_signature_request
//     SELECT NULL,NOW(),'D',?,ipd_dr_consult_signature_request.* FROM kphis.ipd_dr_consult_signature_request WHERE consult_id=?;
// // ipd-dr-consult-update.php
// // we change `WHERE consult_signature_id=?` to `WHERE consult_id=?` to reuse it
// INSERT INTO kphis.history_ipd_dr_consult_signature_request
//     SELECT NULL,NOW(),'U',?,ipd_dr_consult_signature_request.* FROM kphis.ipd_dr_consult_signature_request WHERE consult_signature_id=?;
// /// loginname, consult_id
// pub fn insert_consult_signature_request_to_history(history_type: &str, kphis: &str) -> String {
//     [
//         "INSERT INTO ",kphis,".history_ipd_dr_consult_signature_request \
//             SELECT NULL,NOW(),'",history_type,"',?,ipd_dr_consult_signature_request.* FROM ",kphis,".ipd_dr_consult_signature_request WHERE consult_id=?;"
//     )
// }

// // ipd-dr-consult-update.php
// INSERT INTO kphis.history_ipd_dr_consult_signature_reply
//     SELECT NULL,NOW(),'U',?,ipd_dr_consult_signature_reply.* FROM kphis.ipd_dr_consult_signature_reply WHERE consult_reply_id=?;
// /// loginname, consult_reply_id
// pub fn insert_consult_signature_reply_to_history(history_type: &str, kphis: &str) -> String {
//     [
//         "INSERT INTO kphis.history_ipd_dr_consult_signature_reply \
//             SELECT NULL,NOW(),'",history_type,"',?,ipd_dr_consult_signature_reply.* FROM ",kphis,".ipd_dr_consult_signature_reply WHERE consult_reply_id=?;"
//     )
// }
