CREATE TABLE `kphis`.`opd_er_bed` (
  `opd_er_bed_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `bedno` VARCHAR(20) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `bed_type` VARCHAR(10) DEFAULT NULL,
  `display_order` INT(5) DEFAULT NULL,
  `active` VARCHAR(1) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`opd_er_bed_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;