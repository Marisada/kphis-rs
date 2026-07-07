SELECT ps.telemed_time,ps.telemed_add,ps.telemed_dose_up,ps.telemed_dose_down,ps.telemed_off,ps.telemed_other,ovst.vn,p.hn,p.pname,p.fname,p.lname
FROM __KPHIS_EXTRA__.prescription_screen ps
    LEFT JOIN __HOSXP__.ovst ON ovst.vn=ps.vn
    LEFT JOIN __HOSXP__.patient p ON p.hn=ovst.hn
WHERE DATE(ps.telemed_time) BETWEEN ? AND ? ORDER BY ps.telemed_time;