CREATE TABLE `kphis`.`ipd_doctor_in_charge` (
  `doctor_in_charge_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `hn` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `spclty` VARCHAR(2) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `status` VARCHAR(1) DEFAULT NULL,
  `activated` VARCHAR(5) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `update_datetime` DATETIME DEFAULT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`doctor_in_charge_id`) USING BTREE,
  INDEX `doctor` (`doctor`) USING BTREE,
  INDEX `an` (`an`) USING BTREE,
  INDEX `hn` (`hn`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;