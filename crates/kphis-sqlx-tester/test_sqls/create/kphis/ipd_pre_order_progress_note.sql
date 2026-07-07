CREATE TABLE `kphis`.`ipd_pre_order_progress_note` (
  `progress_note_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `pre_order_master_id` INT(11) UNSIGNED NOT NULL,
  `progress_note_date` DATE NOT NULL,
  `progress_note_time` TIME NOT NULL,
  `progress_note_owner_type` VARCHAR(20) NOT NULL COMMENT 'doctor, nurse, pharmacist',
  `progress_note_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`progress_note_id`) USING BTREE,
  INDEX `progress_note_date` (`progress_note_date`) USING BTREE,
  INDEX `pre_order_master_id` (`pre_order_master_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;