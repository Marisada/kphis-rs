CREATE TABLE `kphis`.`opd_er_focus_note_dlc_item` (
  `fcnote_dlc_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `fcnote_id` INT(11) UNSIGNED DEFAULT NULL,
  `dlc_id` INT(11) UNSIGNED DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`fcnote_dlc_item_id`) USING BTREE,
  UNIQUE INDEX `fcnote_dlc` (`fcnote_id`,`dlc_id`) USING BTREE,
  INDEX `consult_id` (`fcnote_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;