// SELECT imr.an AS vnid_an,CONCAT(ipt.regdate,' ',ipt.regtime) AS visit_datetime,ipt.hn
//   FROM kphis.ipd_med_reconciliation imr
//     LEFT JOIN hos.ipt ON ipt.an=imr.an
//   WHERE ipt.hn=?
//   GROUP BY imr.an
//   HAVING visit_datetime IS NOT NULL
// UNION
// SELECT CONCAT(om.vn,',',omr.opd_er_order_master_id) AS vnid_an,CONCAT(om.order_date,' ',om.order_time) AS visit_datetime,ovst.hn
//   FROM kphis.opd_er_med_reconciliation omr
//     LEFT JOIN kphis.opd_er_order_master om ON om.opd_er_order_master_id=omr.opd_er_order_master_id
//     LEFT JOIN ovst ON ovst.vn=om.vn
//   WHERE ovst.hn=?
//   GROUP BY omr.opd_er_order_master_id
//   HAVING visit_datetime IS NOT NULL LIMIT 20;
/// hn, hn
pub fn select_med_reconcile_header_by_hn(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT imr.an AS vnid_an,ADDTIME(CONVERT(ipt.regdate,DATETIME),ipt.regtime) AS visit_datetime,ipt.hn \
            FROM ",kphis,".ipd_med_reconciliation imr \
                LEFT JOIN ",hosxp,".ipt ON ipt.an=imr.an \
            WHERE ipt.hn=? \
            GROUP BY imr.an \
            HAVING visit_datetime IS NOT NULL \
            UNION \
            SELECT CONCAT(om.vn,',',omr.opd_er_order_master_id) AS vnid_an,ADDTIME(CONVERT(om.order_date,DATETIME),om.order_time) AS visit_datetime,ovst.hn \
            FROM ",kphis,".opd_er_med_reconciliation omr \
                LEFT JOIN ",kphis,".opd_er_order_master om ON om.opd_er_order_master_id=omr.opd_er_order_master_id \
                LEFT JOIN ",hosxp,".ovst ON ovst.vn=om.vn \
            WHERE ovst.hn=? \
            GROUP BY omr.opd_er_order_master_id \
            HAVING visit_datetime IS NOT NULL LIMIT 20;"
    ].concat()
}