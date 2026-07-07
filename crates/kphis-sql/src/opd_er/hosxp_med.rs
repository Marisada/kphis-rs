// // opd-er-hosxp-med-data.php
// SELECT o.hos_guid,o.icode,o.item_no,o.item_type,CONCAT(s.`name`,' ',s.strength,' ',s.units) AS item_name,
//     IF(NOT(o.sp_use IS NULL OR TRIM(o.sp_use) = ''),
//         CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),
//         CONCAT(IFNULL(d.name1,''),' ',IFNULL(d.name2,''),' ',IFNULL(d.name3,''))
//     ) AS `usage`,o.qty,o.paidst,d.shortlist,s.displaycolor,u.name1,u.name2,u.name3,o.sp_use,o.rxdate,o.rxtime,o.vn,o.an
// FROM hos.ovst
//     INNER JOIN hos.opitemrece o ON o.vn = ovst.vn AND o.icode LIKE '1%' AND o.an IS NULL
//     LEFT JOIN hos.s_drugitems s ON s.icode = o.icode
//     LEFT JOIN hos.drugusage d ON d.drugusage = o.drugusage
//     LEFT JOIN hos.sp_use u ON u.sp_use = o.sp_use
//     LEFT JOIN hos.drugitems i ON i.icode = o.icode
// WHERE ovst.vn=? UNION
// SELECT o.hos_guid,o.icode,o.item_no,o.item_type,CONCAT(s.`name`,' ',s.strength,' ',s.units ) AS item_name,
//     IF(NOT(o.sp_use IS NULL OR TRIM(o.sp_use) = ''),
//         CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),
//         CONCAT(IFNULL(d.name1,''),' ',IFNULL(d.name2,''),' ',IFNULL(d.name3,''))
//     ) AS `usage`,o.qty,o.paidst,d.shortlist,s.displaycolor,u.name1,u.name2,u.name3,o.sp_use,o.rxdate,o.rxtime,o.vn, o.an
// FROM hos.ovst
//     INNER JOIN hos.opitemrece o ON o.an = ovst.an AND o.icode LIKE '1%' AND o.an IS NOT NULL
//     JOIN hos.ipt_order_no ON o.an = ipt_order_no.an AND o.order_no = ipt_order_no.order_no AND ipt_order_no.order_type = 'TRx'
//     LEFT JOIN hos.s_drugitems s ON s.icode = o.icode
//     LEFT JOIN hos.drugusage d ON d.drugusage = o.drugusage
//     LEFT JOIN hos.sp_use u ON u.sp_use = o.sp_use
//     LEFT JOIN hos.drugitems i ON i.icode = o.icode
// WHERE ovst.vn=? ORDER BY rxdate, rxtime;
/// vn, vn
pub fn select_opd_med(hosxp: &str) -> String {
    [
        "SELECT o.hos_guid,o.icode,o.item_no,o.item_type,CONCAT(s.`name`,' ',s.strength,' ',s.units) AS item_name,\
            IF(NOT(o.sp_use IS NULL OR TRIM(o.sp_use) = ''),\
                CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),\
                CONCAT(IFNULL(d.name1,''),' ',IFNULL(d.name2,''),' ',IFNULL(d.name3,''))\
            ) AS `usage`,o.qty,o.paidst,d.shortlist,s.displaycolor,u.name1,u.name2,u.name3,o.sp_use,o.rxdate,o.rxtime,o.vn,o.an \
        FROM ",hosxp,".ovst \
            INNER JOIN ",hosxp,".opitemrece o ON o.vn = ovst.vn AND o.icode LIKE '1%' AND o.an IS NULL \
            LEFT JOIN ",hosxp,".s_drugitems s ON s.icode = o.icode \
            LEFT JOIN ",hosxp,".drugusage d ON d.drugusage = o.drugusage \
            LEFT JOIN ",hosxp,".sp_use u ON u.sp_use = o.sp_use \
            LEFT JOIN ",hosxp,".drugitems i ON i.icode = o.icode \
        WHERE ovst.vn=? UNION \
        SELECT o.hos_guid,o.icode,o.item_no,o.item_type,CONCAT(s.`name`,' ',s.strength,' ',s.units ) AS item_name,\
            IF(NOT(o.sp_use IS NULL OR TRIM(o.sp_use) = ''),\
                CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),\
                CONCAT(IFNULL(d.name1,''),' ',IFNULL(d.name2,''),' ',IFNULL(d.name3,''))\
            ) AS `usage`,o.qty,o.paidst,d.shortlist,s.displaycolor,u.name1,u.name2,u.name3,o.sp_use,o.rxdate,o.rxtime,o.vn, o.an \
        FROM ",hosxp,".ovst \
            INNER JOIN ",hosxp,".opitemrece o ON o.an = ovst.an AND o.icode LIKE '1%' AND o.an IS NOT NULL \
            JOIN ",hosxp,".ipt_order_no ON o.an = ipt_order_no.an AND o.order_no = ipt_order_no.order_no AND ipt_order_no.order_type = 'TRx' \
            LEFT JOIN ",hosxp,".s_drugitems s ON s.icode = o.icode \
            LEFT JOIN ",hosxp,".drugusage d ON d.drugusage = o.drugusage \
            LEFT JOIN ",hosxp,".sp_use u ON u.sp_use = o.sp_use \
            LEFT JOIN ",hosxp,".drugitems i ON i.icode = o.icode \
        WHERE ovst.vn=? ORDER BY rxdate, rxtime;"
    ].concat()
}
