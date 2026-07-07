SELECT ps.postal_time,ovst.vn,p.hn,p.pname,p.fname,p.lname,p.hometel,p.po_code,
    CONCAT_WS(' ',p.addrpart,'หมู่',p.moopart,tambol.full_address_name) AS homeaddr
FROM __KPHIS_EXTRA__.prescription_screen ps
    LEFT JOIN __HOSXP__.ovst ON ovst.vn=ps.vn
    LEFT JOIN __HOSXP__.patient p ON p.hn=ovst.hn
    LEFT JOIN __HOSXP__.tambol ON tambol.tambol_code=CONCAT(p.chwpart,p.amppart,p.tmbpart)
WHERE ps.postal_status='Y' AND DATE(ps.postal_time) BETWEEN ? AND ? ORDER BY ps.postal_time;