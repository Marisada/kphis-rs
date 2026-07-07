SELECT ipt.an AS ipt_id,s2.summary_id AS s_id,m.*
FROM __HOSXP__.ipt
  LEFT JOIN __KPHIS__.ipd_summary_2 s2 ON s2.an=ipt.an
  LEFT JOIN (SELECT *, MIN(mra_id) FROM __KPHIS_EXTRA__.ipd_mra GROUP BY an) m ON m.an=s2.an
WHERE ipt.ward IN (?) AND m.audit_type IN (?) AND m.is_psychiatry IN (?) AND ipt.dchdate BETWEEN ? AND ?;