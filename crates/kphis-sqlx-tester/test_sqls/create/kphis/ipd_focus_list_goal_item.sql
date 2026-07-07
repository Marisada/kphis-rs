CREATE TABLE `kphis`.`ipd_focus_list_goal_item` (
  `fclist_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `fclist_id` INT(11) UNSIGNED DEFAULT NULL,
  `goal_id` INT(11) UNSIGNED DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`fclist_item_id`) USING BTREE,
  UNIQUE INDEX `fclist_goal` (`fclist_id`,`goal_id`) USING BTREE,
  INDEX `consult_id` (`fclist_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;