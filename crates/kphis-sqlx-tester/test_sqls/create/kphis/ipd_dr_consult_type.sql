CREATE TABLE `kphis`.`ipd_dr_consult_type` (
  `consult_type_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `consult_type_name` VARCHAR(255) DEFAULT NULL,
  `consult_type_order` SMALLINT(4) DEFAULT NULL,
  `version` SMALLINT(4) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_datetime` DATETIME DEFAULT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `update_datetime` DATETIME DEFAULT NULL,
  PRIMARY KEY (`consult_type_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;