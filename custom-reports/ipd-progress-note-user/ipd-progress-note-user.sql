SELECT an,progress_note_date,progress_note_time,d.`name`,
  (SELECT GROUP_CONCAT(progress_note_item_detail) FROM __KPHIS__.ipd_progress_note_item pni WHERE pni.progress_note_id=pn.progress_note_id AND progress_note_item_type='note') AS notes,
  (SELECT GROUP_CONCAT(progress_note_item_detail) FROM __KPHIS__.ipd_progress_note_item pni WHERE pni.progress_note_id=pn.progress_note_id AND progress_note_item_type='problem-list') AS problem_lists,
  (SELECT GROUP_CONCAT(progress_note_item_detail) FROM __KPHIS__.ipd_progress_note_item pni WHERE pni.progress_note_id=pn.progress_note_id AND progress_note_item_type='subjective') AS subjectives,
  (SELECT GROUP_CONCAT(progress_note_item_detail) FROM __KPHIS__.ipd_progress_note_item pni WHERE pni.progress_note_id=pn.progress_note_id AND progress_note_item_type='objective') AS objectives,
  (SELECT GROUP_CONCAT(progress_note_item_detail) FROM __KPHIS__.ipd_progress_note_item pni WHERE pni.progress_note_id=pn.progress_note_id AND progress_note_item_type='assessment') AS assessments,
  (SELECT GROUP_CONCAT(progress_note_item_detail) FROM __KPHIS__.ipd_progress_note_item pni WHERE pni.progress_note_id=pn.progress_note_id AND progress_note_item_type='plan') AS plans
FROM __KPHIS__.ipd_progress_note pn
  LEFT JOIN __HOSXP__.doctor d ON d.`code`=pn.progress_note_doctor
WHERE pn.progress_note_doctor IN (?) AND pn.progress_note_date BETWEEN ? AND ?
ORDER BY pn.progress_note_date,pn.progress_note_time;