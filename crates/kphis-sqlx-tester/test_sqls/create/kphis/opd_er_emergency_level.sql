CREATE TABLE `kphis`.`opd_er_emergency_level` (
  `emergency_level_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `emergency_level_name` VARCHAR(255) DEFAULT NULL,
  `emergency_level_order` SMALLINT(4) DEFAULT NULL,
  `version` SMALLINT(4) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_datetime` DATETIME DEFAULT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `update_datetime` DATETIME DEFAULT NULL,
  PRIMARY KEY (`emergency_level_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;