CREATE TABLE `kphis`.`system_ac_role_user` (
  `loginname` VARCHAR(100) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `role` VARCHAR(100) NOT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`loginname`,`role`) USING BTREE,
  UNIQUE INDEX `role_id` (`role`,`loginname`) USING BTREE,
  CONSTRAINT `system_ac_role_user_ibfk_1` FOREIGN KEY (`role`) REFERENCES `system_ac_role` (`role`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;