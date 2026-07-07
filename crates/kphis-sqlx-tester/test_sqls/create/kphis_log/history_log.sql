CREATE TABLE `kphis_log`.`history_log` (
  `history_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `history_datetime` DATETIME NULL DEFAULT NULL,
  `history_table_name` VARCHAR(255) NULL DEFAULT NULL,
  `history_type` VARCHAR(1) NULL DEFAULT NULL,
  `history_user` VARCHAR(50) NULL DEFAULT NULL,
  `data` LONGTEXT NULL DEFAULT NULL,
  PRIMARY KEY (`history_id`) USING BTREE,
  INDEX `history_datetime`(`history_datetime`) USING BTREE,
  INDEX `history_table_name`(`history_table_name`(191)) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;