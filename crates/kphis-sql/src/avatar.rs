use kphis_model::avatar::AvatarParams;

use crate::ipd::and_ipt_patient;

// SELECT om.opd_er_order_master_id,ovst.hn,om.vn,b.bedno AS display_bedno,bt.bed_type_name,bt.bed_type_color,CONCAT(p.pname,p.fname,' ',p.lname) AS pname
// FROM kphis.opd_er_order_master om
//     LEFT JOIN kphis.opd_er_bed b ON b.opd_er_bed_id=om.bedno
//     LEFT JOIN kphis.opd_er_bed_type bt ON b.bed_type=bt.bed_type
//     LEFT JOIN hos.ovst ON ovst.vn=om.vn
//     LEFT JOIN hos.patient p ON p.hn=ovst.hn
// WHERE (om.delete_flag IS NULL OR om.delete_flag <> 'Y') AND om.er_patient_status_id <> 7 ORDER BY bt.display_order,b.display_order;
pub fn select_avatar_opd_er(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT om.opd_er_order_master_id,ovst.hn,om.vn,b.bedno AS display_bedno,bt.bed_type_name,bt.bed_type_color,CONCAT(p.pname,p.fname,' ',p.lname) AS pname \
        FROM ",kphis,".opd_er_order_master om \
            LEFT JOIN ",kphis,".opd_er_bed b ON b.opd_er_bed_id=om.bedno \
            LEFT JOIN ",kphis,".opd_er_bed_type bt ON b.bed_type=bt.bed_type \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=om.vn \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ovst.hn \
        WHERE (om.delete_flag IS NULL OR om.delete_flag <> 'Y') AND om.er_patient_status_id <> 7 ORDER BY bt.display_order,b.display_order;"
    ].concat()
}

// SELECT ipt.hn,ipt.an,iptadm.bedno,CONCAT(patient.pname,patient.fname,' ',patient.lname) AS pname
// FROM hos.ipt
//     LEFT JOIN hos.patient ON patient.hn=ipt.hn
//     LEFT JOIN hos.iptadm ON iptadm.an=ipt.an
// WHERE ipt.ward=? AND ipt.dchstts IS NULL ORDER BY LEFT(iptadm.bedno,3),MID(iptadm.bedno,4,999),ipt.regdate,ipt.regtime;
/// (ward), (search)
pub fn select_avatar_in_ward(
    params: &AvatarParams,
    hlen: usize,
    alen: usize,
    hosxp: &str,
    kphis: &str,
) -> String {
    let ward = if params.ward.is_some() {" AND ipt.ward=? "} else {""};
    let patient = and_ipt_patient(&params.search, hlen, alen, hosxp).unwrap_or_default();
    [
        "SELECT ipt.hn,ipt.an,iptadm.bedno,CONCAT(p.pname,p.fname,' ',p.lname) AS pname,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_order \
                JOIN ",kphis,".ipd_order_item oi ON ipd_order.order_id=oi.order_id AND oi.order_item_type='discharge' \
                LEFT JOIN ",kphis,".ipd_order_item ooi ON ooi.order_item_id=oi.off_order_item_id \
                WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ooi.order_item_id IS NULL)) AS discharge_order_exists \
        FROM ",hosxp,".ipt \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ipt.hn \
            LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
            LEFT JOIN ",kphis,".ipd_ward_passcode wp ON wp.ward=ipt.ward \
        WHERE ipt.dchstts IS NULL ", &patient, ward, " AND ipt.ward NOT IN (SELECT ward FROM ",kphis,".ipd_ward_passcode) AND wp.passcode IS NULL \
        ORDER BY LEFT(iptadm.bedno,3),MID(iptadm.bedno,4,999),ipt.regdate,ipt.regtime;"
    ].concat()
}
