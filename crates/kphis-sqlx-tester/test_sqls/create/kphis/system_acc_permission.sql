CREATE TABLE `kphis`.`system_ac_permission` (
  `permission` VARCHAR(100) NOT NULL,
  `resource` VARCHAR(100) DEFAULT NULL,
  `operation` VARCHAR(100) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`permission`) USING BTREE,
  UNIQUE INDEX `resource` (`resource`,`operation`) USING BTREE,
  INDEX `system_ac_permission_ibfk_2` (`operation`) USING BTREE,
  CONSTRAINT `system_ac_permission_ibfk_1` FOREIGN KEY (`resource`) REFERENCES `system_ac_resource` (`resource`) ON UPDATE CASCADE,
  CONSTRAINT `system_ac_permission_ibfk_2` FOREIGN KEY (`operation`) REFERENCES `system_ac_operation` (`operation`) ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;