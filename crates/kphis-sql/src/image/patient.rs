// SELECT image FROM hos.patient_image WHERE hn=?;
/// hn
pub fn select_patient_image(hosxp: &str) -> String {
    [
        "SELECT image FROM ",hosxp,".patient_image WHERE hn=?;"
    ].concat()
}