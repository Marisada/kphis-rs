CREATE TABLE `kphis`.`ipd_tmp_group_smp` (
  `smp_id` INT(11) UNSIGNED NOT NULL,
  `smp_name` VARCHAR(255) DEFAULT NULL,
  `smp_group` INT(11) UNSIGNED DEFAULT NULL,
  `smp_order` INT(11) UNSIGNED DEFAULT NULL,
  `smp_status` char(1) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_datetime` DATETIME DEFAULT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `update_datetime` DATETIME DEFAULT NULL,
  `version` SMALLINT(4) DEFAULT NULL,
  PRIMARY KEY (`smp_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;