// SELECT nurse_adm.*,opduser.name AS nurse_name,opduser.entryposition AS nurse_pos
// FROM kphis.ipd_nurse_admission_note nurse_adm LEFT JOIN hos.opduser ON nurse_adm.update_user=opduser.loginname WHERE an=?;"
/// an
pub fn select_admission_note_from_an(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT nurse_adm.*,doctor.`name` AS nurse_name,opduser.entryposition AS nurse_pos,doctor.licenseno AS nurse_licenseno,dan.receiver_medication_date,dan.receiver_medication_time \
        FROM ",kphis,".ipd_nurse_admission_note nurse_adm \
            LEFT JOIN ",kphis,".ipd_dr_admission_note dan ON dan.an=nurse_adm.an \
            LEFT JOIN ",hosxp,".opduser ON nurse_adm.update_user=opduser.loginname \
            LEFT JOIN ",hosxp,".doctor ON doctor.`code`=opduser.doctorcode \
        WHERE nurse_adm.an=?;"
    ].concat()
}

// SELECT oc.cc,oc.hpi FROM hos.opdscreen oc LEFT JOIN hos.ipt ON ipt.vn=oc.vn WHERE ipt.an=?;
/// an
pub fn select_cc_hpi_from_an(hosxp: &str) -> String {
    [
        "SELECT oc.cc,oc.hpi FROM ",hosxp,".opdscreen oc LEFT JOIN ",hosxp,".ipt ON ipt.vn=oc.vn WHERE ipt.an=?;"
    ].concat()
}