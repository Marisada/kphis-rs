CREATE TABLE `kphis`.`ipd_summary_approve_doctor` (
  `summary_id` INT(11) UNSIGNED NOT NULL,
  `summary_approve_doctor` VARCHAR(15) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL DEFAULT '',
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`summary_id`,`summary_approve_doctor`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;