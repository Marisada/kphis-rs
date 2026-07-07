SELECT tf.focus_name,
  COUNT(fl.fclist_id) AS total,
  COUNT(CASE WHEN fl.fclist_status=1 THEN 1 ELSE NULL END) AS ongoing,
  COUNT(CASE WHEN fl.fclist_status=2 THEN 1 ELSE NULL END) AS closed
FROM __KPHIS__.ipd_focus_list fl
  LEFT JOIN __KPHIS__.ipd_tmp_focus tf ON tf.focus_id=fl.focus_id
  LEFT JOIN __HOSXP__.ipt ON ipt.an=fl.an
WHERE ipt.ward IN (?) AND fl.fclist_stdate between ? AND ?
GROUP BY tf.focus_id ORDER BY total DESC, tf.focus_name;