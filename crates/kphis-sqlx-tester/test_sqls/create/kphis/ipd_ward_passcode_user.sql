CREATE TABLE `kphis`.`ipd_ward_passcode_user` (
  `ward` VARCHAR(4) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL DEFAULT '',
  `loginname` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`ward`,`loginname`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;