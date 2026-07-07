use kphis_model::ipd::mra::MraParams;

// SELECT * FROM kphis_extra.ipd_mra WHERE an=? ORDER BY mra_id DESC;
/// (an)
pub fn select_mra(params: &MraParams, kphis_extra: &str) -> String {
    let an = if params.an.is_some() {" AND ipd_mra.an=?"} else {""};
    [
        "SELECT * FROM ",kphis_extra,".ipd_mra WHERE 1=1 ",an," ORDER BY mra_id DESC LIMIT 100;"
    ].concat()
}

// DELETE kphis_extra.ipd_mra WHERE mra_id=?;
/// mra_id
pub fn delete_mra(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_mra WHERE mra_id=?;"
    ].concat()
}