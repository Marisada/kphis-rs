use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT rn.*,h.`name` AS refer_hospcode_name,d.`name` AS doctor_name
// FROM kphis_extra.refer_note rn
//     LEFT JOIN hos.hospcode h ON h.hospcode=rn.refer_hospcode
//     LEFT JOIN hos.doctor d ON d.`code`=rn.doctor
// WHERE rn.vn=?;
/// vnan
pub fn select_refer_note(hosxp: &str, kphis_extra: &str) -> String {
    [
        "SELECT rn.*,h.`name` AS refer_hospcode_name,d.`name` AS doctor_name \
        FROM ",kphis_extra,".refer_note rn \
            LEFT JOIN ",hosxp,".hospcode h ON h.hospcode=rn.refer_hospcode \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=rn.doctor \
        WHERE rn.vn=?;"
    ].concat()
}

// INSERT INTO kphis_extra.refer_note (vn,hn,refer_hospcode,refer_date,refer_time,docno,pmh,hpi,lab_text,treatment_text,other_text,diagnosis_text,request_text,doctor,cc,pe,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// vn, hn, refer_hospcode, refer_date, refer_time, docno, pmh, hpi, lab_text, treatment_text, other_text, diagnosis_text, request_text, doctor, cc, pe, loginname, loginname
pub fn insert_refer_note(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".refer_note (vn,hn,refer_hospcode,refer_date,refer_time,docno,pmh,hpi,lab_text,treatment_text,other_text,diagnosis_text,request_text,doctor,cc,pe",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis_extra.refer_note SET vn=?,hn=?,refer_hospcode=?,refer_date=?,refer_time=?,docno=?,pmh=?,hpi=?,lab_text=?,treatment_text=?,other_text=?,diagnosis_text=?,request_text=?,doctor=?,cc=?,pe=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE refernote_id=?;
/// vn, hn, refer_hospcode, refer_date, refer_time, docno, pmh, hpi, lab_text, treatment_text, other_text, diagnosis_text, request_text, doctor, cc, pe, loginname, refernote_id
pub fn update_refer_note(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".refer_note SET vn=?,hn=?,refer_hospcode=?,refer_date=?,refer_time=?,docno=?,pmh=?,hpi=?,lab_text=?,treatment_text=?,other_text=?,diagnosis_text=?,request_text=?,doctor=?,cc=?,pe=?", TABLE_UPDATE_SET,
        " WHERE refernote_id=?;"
    ].concat()
}
