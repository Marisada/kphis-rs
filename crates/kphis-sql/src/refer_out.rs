// SELECT ro.*,h.`name` AS refer_hospcode_name,rc.`name` AS refer_cause_name,rx.moph_refer_expire_type_name AS moph_refer_expire_type_name,d.`name` AS doctor_name
// FROM hos.referout ro
//     LEFT JOIN hos.hospcode h ON h.hospcode=ro.refer_hospcode
//     LEFT JOIN hos.refer_cause rc ON rc.id=ro.refer_cause
//     LEFT JOIN hos.moph_refer_expire_type rx ON rx.moph_refer_expire_type_id=ro.moph_refer_expire_type_id
//     LEFT JOIN hos.doctor d ON d.`code`=ro.doctor
// WHERE ro.vn=?;
/// vnan
pub fn select_his_referout(hosxp: &str) -> String {
    [
        "SELECT ro.*,h.`name` AS refer_hospcode_name,rc.`name` AS refer_cause_name,rx.moph_refer_expire_type_name AS moph_refer_expire_type_name,d.`name` AS doctor_name \
        FROM ",hosxp,".referout ro \
            LEFT JOIN ",hosxp,".hospcode h ON h.hospcode=ro.refer_hospcode \
            LEFT JOIN ",hosxp,".refer_cause rc ON rc.id=ro.refer_cause \
            LEFT JOIN ",hosxp,".moph_refer_expire_type rx ON rx.moph_refer_expire_type_id=ro.moph_refer_expire_type_id \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=ro.doctor \
        WHERE ro.vn=?;"
    ].concat()
}

// SELECT * FROM hos.refer_vital_sign WHERE referout_id=?;
/// referout_id
pub fn select_his_refer_vital_sign(hosxp: &str) -> String {
    [
        "SELECT * FROM ",hosxp,".refer_vital_sign WHERE referout_id=?;"
    ].concat()
}

// INSERT INTO hos.referout (referout_id,vn,hn,refer_hospcode,refer_date,refer_time,due_date,expire_date,pre_diagnosis,pmh,hpi,lab_text,treatment_text,other_text,diagnosis_text,request_text,
//     department,pttype,spclty,refer_type,refer_cause,refer_point,moph_refer_expire_type_id,doctor,update_datetime)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW());
/// referout_id, vn, hn, refer_hospcode, refer_date, refer_time, due_date, expire_date, pre_diagnosis, pmh, hpi, lab_text, treatment_text, other_text, diagnosis_text, request_text, department, pttype, spclty, refer_type, refer_cause, refer_point, moph_refer_expire_type_id, doctorcode
pub fn insert_his_referout(hosxp: &str) -> String {
    [
        "INSERT INTO ",hosxp,".referout (referout_id,vn,hn,refer_hospcode,refer_date,refer_time,due_date,expire_date,pre_diagnosis,pmh,hpi,lab_text,treatment_text,other_text,diagnosis_text,request_text,\
            department,pttype,spclty,refer_type,refer_cause,refer_point,moph_refer_expire_type_id,doctor,update_datetime) \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW());"
    ].concat()
}

// UPDATE hos.referout SET refer_hospcode=?,refer_date=?,refer_time=?,due_date=?,expire_date=?,pre_diagnosis=?,pmh=?,hpi=?,lab_text=?,treatment_text=?,other_text=?,diagnosis_text=?,request_text=?,
//     department=?,pttype=?,spclty=?,refer_type=?,refer_cause=?,refer_point=?,moph_refer_expire_type_id=?,doctor=?,issued_moph_refer='N',update_datetime=NOW()
// WHERE referout_id=?;
/// refer_hospcode, refer_date, refer_time, due_date, expire_date, pre_diagnosis, pmh, hpi, lab_text, treatment_text, other_text, diagnosis_text, request_text, department, pttype, spclty, refer_type, refer_cause, refer_point, moph_refer_expire_type_id, doctorcode, referout_id
pub fn update_his_referout(hosxp: &str) -> String {
    [
        "UPDATE ",hosxp,".referout SET refer_hospcode=?,refer_date=?,refer_time=?,due_date=?,expire_date=?,pre_diagnosis=?,pmh=?,hpi=?,lab_text=?,treatment_text=?,other_text=?,diagnosis_text=?,request_text=?,\
            department=?,pttype=?,spclty=?,refer_type=?,refer_cause=?,refer_point=?,moph_refer_expire_type_id=?,doctor=?,issued_moph_refer='N',update_datetime=NOW() \
        WHERE referout_id=?;"
    ].concat()
}

// INSERT INTO hos.refer_vital_sign (refer_vital_sign_id,referout_id,cc,pe,pre_diagnosis) VALUES (?,?,?,?,?);
/// refer_vital_sign_id, referout_id, cc, pe, pre_diagnosis
pub fn insert_his_refer_vital_sign(hosxp: &str) -> String {
    [
        "INSERT INTO ",hosxp,".refer_vital_sign (refer_vital_sign_id,referout_id,cc,pe,pre_diagnosis) VALUES (?,?,?,?,?);"
    ].concat()
}

// UPDATE hos.refer_vital_sign SET cc=?,pe=?,pre_diagnosis=? WHERE refer_vital_sign_id=?;
/// cc, pe, pre_diagnosis, refer_vital_sign_id
pub fn update_his_refer_vital_sign(hosxp: &str) -> String {
    [
        "UPDATE ",hosxp,".refer_vital_sign SET cc=?,pe=?,pre_diagnosis=? WHERE refer_vital_sign_id=?;"
    ].concat()
}