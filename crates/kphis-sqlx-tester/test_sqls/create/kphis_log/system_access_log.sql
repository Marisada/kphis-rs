CREATE TABLE `kphis_log`.`system_access_log` (
  `access_log_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `access_datetime` DATETIME NULL DEFAULT NULL,
  `access_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `access_host` VARCHAR(100) NULL DEFAULT NULL,
  `access_detail` TEXT NULL DEFAULT NULL,
  PRIMARY KEY (`access_log_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;