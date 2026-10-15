SELECT mr.an,ipt.regdate,ipt.regtime,MIN(mr.med_reconciliation_datetime) AS min_medrec_datetime,mr.phamacist_confirm_datetime,mr.doctor_confirm_datetime,
  IF(mr.pharmacist=mr.doctor,1,0) AS is_self_confirm,d.provider_type_code AS confirm_provider_type
FROM kphis.ipd_med_reconciliation mr
  LEFT JOIN __KPHIS__.ipd_med_reconciliation_item mri ON mri.med_reconciliation_id=mr.med_reconciliation_id
  LEFT JOIN __HOSXP__.ipt ON ipt.an=mr.an 
  LEFT JOIN __HOSXP__.doctor d ON d.code=mr.pharmacist
WHERE mr.doctor_confirm_datetime IS NOT NULL AND ipt.regdate BETWEEN ? AND ?
GROUP BY mr.an ORDER BY mr.an;