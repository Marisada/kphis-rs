SELECT ipt.an,sa.com_adjrw,sa.rev_adjrw,sa.sa,sa.ca,
    (SELECT GROUP_CONCAT(DISTINCT(sai.sa)) FROM __KPHIS_EXTRA__.ipd_summary_audit_item sai WHERE sai.summary_audit_id=sa.summary_audit_id) AS sas,
    (SELECT GROUP_CONCAT(DISTINCT(sai.ca)) FROM __KPHIS_EXTRA__.ipd_summary_audit_item sai WHERE sai.summary_audit_id=sa.summary_audit_id) AS cas
FROM __HOSXP__.ipt
    LEFT JOIN (SELECT *, MIN(summary_audit_id) FROM __KPHIS_EXTRA__.ipd_summary_audit WHERE audit_type IN (?) GROUP BY com_an) sa ON sa.com_an=ipt.an
WHERE ipt.ward IN (?) AND ipt.dchdate BETWEEN ? AND ?;