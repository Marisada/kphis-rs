CREATE TABLE `kphis`.`opd_er_order_progress_note_item` (
  `progress_note_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `progress_note_id` INT(11) UNSIGNED DEFAULT NULL,
  `opd_er_order_master_id` INT(11) UNSIGNED DEFAULT NULL,
  `progress_note_item_type` VARCHAR(20) DEFAULT NULL,
  `progress_note_item_detail` TEXT DEFAULT NULL,
  `progress_note_item_detail_2` TEXT DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`progress_note_item_id`) USING BTREE,
  INDEX `progress_note_id` (`progress_note_id`) USING BTREE,
  INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;