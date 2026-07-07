CREATE TABLE `kphis`.`opd_er_allergy_history` (
  `er_allergy_history_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `opd_er_order_master_id` INT(11) UNSIGNED DEFAULT NULL,
  `er_allergy_history_agent` TEXT DEFAULT NULL,
  `er_allergy_history_symptom` TEXT DEFAULT NULL,
  `er_allergy_history_doctorcode` VARCHAR(7) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`er_allergy_history_id`) USING BTREE,
  INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;