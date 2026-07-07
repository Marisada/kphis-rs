CREATE TABLE `kphis`.`ipd_tmp_focus` (
  `focus_id` INT(11) UNSIGNED NOT NULL,
  `focus_name` TEXT DEFAULT NULL,
  `smp_id` INT(11) UNSIGNED DEFAULT NULL,
  `subgroup` INT(11) UNSIGNED DEFAULT NULL,
  `focus_order` INT(11) UNSIGNED DEFAULT NULL,
  `focus_status` VARCHAR(1) DEFAULT NULL,
  `version` INT(4) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_datetime` DATETIME DEFAULT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `update_datetime` DATETIME DEFAULT NULL,
  PRIMARY KEY (`focus_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;