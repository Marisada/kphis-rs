CREATE TABLE `kphis`.`ipd_dr_admission_note_item` (
  `admission_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `admission_note_id` INT(11) UNSIGNED DEFAULT NULL,
  `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `admission_note_doctor` VARCHAR(7) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`admission_note_item_id`) USING BTREE,
  UNIQUE INDEX `admission_note_id` (`admission_note_id`,`admission_note_doctor`) USING BTREE,
  INDEX `admission_note_id_2` (`admission_note_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;