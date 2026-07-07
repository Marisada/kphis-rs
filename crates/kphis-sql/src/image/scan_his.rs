// SELECT * FROM hos.pe_image WHERE vn=?;
/// vn
pub fn select_pe_image(hosxp: &str) -> String {
    ["SELECT * FROM ", hosxp, ".pe_image WHERE vn=?;"].concat()
}

// SELECT * FROM hos.er_image WHERE vn=?;
/// vn
pub fn select_er_image(hosxp: &str) -> String {
    ["SELECT * FROM ", hosxp, ".er_image WHERE vn=?;"].concat()
}

// SELECT loi.* FROM hos.lab_order_image loi
//   JOIN hos.lab_head lh ON loi.lab_order_number=lh.lab_order_number
// WHERE lh.vn=?;
/// vn, ?an
pub fn select_lab_image(has_an: bool, hosxp: &str) -> String {
    let vn = if has_an { " IN (?,?);" } else { "=?;" };
    [
        "SELECT loi.* FROM ",hosxp,".lab_order_image loi \
            JOIN ",hosxp,".lab_head lh ON loi.lab_order_number=lh.lab_order_number \
        WHERE lh.vn",vn
    ].concat()
}

// SELECT * FROM hos.lab_order_image WHERE lab_order_number=?;
/// lab_order_number
pub fn select_lab_image_from_lab_order_number(hosxp: &str) -> String {
    [
        "SELECT * FROM ",hosxp,".lab_order_image WHERE lab_order_number=?;"
    ].concat()
}

// SELECT image FROM hos.patient_opd_scan WHERE vn=? ORDER BY page_no;
/// vn
pub fn select_opd_scan_image(hosxp: &str) -> String {
    [
        "SELECT image FROM ",hosxp,".patient_opd_scan WHERE vn=? ORDER BY page_no;"
    ].concat()
}

// SELECT
// (SELECT COUNT(vn) FROM hos.pe_image WHERE vn='670924132040' AND image1 IS NOT NULL) AS pe_count,
// (SELECT COUNT(vn) FROM hos.er_image WHERE vn='670924132040' AND image1 IS NOT NULL) AS er_count,
// (SELECT COUNT(loi.lab_order_number) FROM hos.lab_order_image loi JOIN hos.lab_head lh ON loi.lab_order_number = lh.lab_order_number WHERE lh.vn='670924132040' AND loi.image1 IS NOT NULL) AS lab_count,
// (SELECT COUNT(patient_opd_scan_id) FROM hos.patient_opd_scan WHERE vn='670924132040' AND image IS NOT NULL) AS scan_count;
/// vn x3, (an), vn
pub fn select_his_image_exists(has_an: bool, hosxp: &str) -> String {
    let vn = if has_an { " IN (?,?)" } else { "=?" };
    [
        "SELECT \
            (SELECT EXISTS(SELECT * FROM ",hosxp,".pe_image WHERE vn=? AND image1 IS NOT NULL)) AS has_pe,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".er_image WHERE vn=? AND image1 IS NOT NULL)) AS has_er,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".lab_order_image loi \
                JOIN ",hosxp,".lab_head lh ON loi.lab_order_number=lh.lab_order_number \
                WHERE lh.vn",vn," AND loi.image1 IS NOT NULL)) AS has_lab,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".patient_opd_scan WHERE vn=? AND image IS NOT NULL)) AS has_scan;"
    ].concat()
}
