CREATE TABLE `kphis`.`ipd_tmp_subgroup` (
  `smp_id` INT(11) UNSIGNED NOT NULL,
  `subgroup` INT(11) UNSIGNED NOT NULL,
  `subgroup_name` TEXT DEFAULT NULL,
  `subgroup_order` INT(11) UNSIGNED DEFAULT NULL,
  `subgroup_status` VARCHAR(1) DEFAULT NULL,
  `version` INT(4) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_datetime` DATETIME DEFAULT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `update_datetime` DATETIME DEFAULT NULL,
  PRIMARY KEY (`smp_id`,`subgroup`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;