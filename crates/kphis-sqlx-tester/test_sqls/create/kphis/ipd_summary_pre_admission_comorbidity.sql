CREATE TABLE `kphis`.`ipd_summary_pre_admission_comorbidity` (
  `pre_admission_comorbidity_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `summary_id` INT(11) UNSIGNED NOT NULL,
  `pre_admission_comorbidity_detail` TEXT DEFAULT NULL,
  `pre_admission_comorbidity_icd10` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`pre_admission_comorbidity_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;