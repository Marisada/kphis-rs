CREATE TABLE `kphis`.`system_ac_role` (
  `role` VARCHAR(100) NOT NULL,
  `role_desc` VARCHAR(100) DEFAULT NULL,
  `parent_role` VARCHAR(100) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`role`) USING BTREE,
  INDEX `system_ac_role_ibfk_1` (`parent_role`) USING BTREE,
  CONSTRAINT `system_ac_role_ibfk_1` FOREIGN KEY (`parent_role`) REFERENCES `system_ac_role` (`role`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;