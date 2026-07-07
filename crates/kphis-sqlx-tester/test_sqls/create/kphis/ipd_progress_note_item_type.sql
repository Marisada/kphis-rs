CREATE TABLE `kphis`.`ipd_progress_note_item_type` (
  `progress_note_item_type` VARCHAR(20) NOT NULL,
  `progress_note_item_type_name` VARCHAR(20) NOT NULL,
  `display_order` INT(11) DEFAULT NULL,
  PRIMARY KEY (`progress_note_item_type`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;