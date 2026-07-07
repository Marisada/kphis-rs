CREATE TABLE `kphis`.`opd_er_consult` (
  `er_consult_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `opd_er_order_master_id` INT(11) UNSIGNED DEFAULT NULL,
  `er_consult_ward` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `er_consult_date` DATE DEFAULT NULL,
  `er_consult_time` TIME DEFAULT NULL,
  `er_consult_doctor_reply` VARCHAR(7) DEFAULT NULL,
  `er_consult_date_reply` DATE DEFAULT NULL,
  `er_consult_time_reply` TIME DEFAULT NULL,
  `er_consult_doctorcode` VARCHAR(7) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`er_consult_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;