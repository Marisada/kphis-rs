SELECT vn, DATE(pharmacist_accept_time) AS accept_date, TIME(pharmacist_accept_time) AS accept_time, TIME(pharmacist_check_time) AS check_time, TIME(pharmacist_done_time) AS done_time,
    TIMESTAMPDIFF(SECOND,pharmacist_accept_time,pharmacist_check_time) AS check_secs,
    TIMESTAMPDIFF(SECOND,pharmacist_check_time,pharmacist_done_time) AS done_secs,
    TIMESTAMPDIFF(SECOND,pharmacist_accept_time,pharmacist_done_time) AS all_secs
FROM __KPHIS_EXTRA__.prescription_screen
WHERE (postal_status IS NULL OR postal_status='N')
    AND pharmacist_check_time IS NOT NULL AND pharmacist_done_time IS NOT NULL
    AND DATE(pharmacist_accept_time) BETWEEN ? AND ?
    AND TIME(pharmacist_accept_time) BETWEEN ? AND ?
ORDER BY pharmacist_accept_time;