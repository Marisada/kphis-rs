CREATE TABLE `kphis`.`ipd_pre_admit_master` (
  `pre_admit_master_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `vn` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `prev_an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`pre_admit_master_id`) USING BTREE,
  UNIQUE INDEX `vn` (`vn`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;