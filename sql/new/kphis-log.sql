CREATE TABLE IF NOT EXISTS `history_log` (
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

CREATE TABLE IF NOT EXISTS `system_access_log` (
  `access_log_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `access_datetime` DATETIME NULL DEFAULT NULL,
  `access_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `access_host` VARCHAR(100) NULL DEFAULT NULL,
  `access_detail` TEXT NULL DEFAULT NULL,
  PRIMARY KEY (`access_log_id`) USING BTREE
) ENGINE = InnoDB CHARACTER SET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

CREATE TABLE IF NOT EXISTS `message` (
  `message_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `message_datetime` DATETIME DEFAULT NULL,
  `message` TEXT NULL DEFAULT NULL,
  `sender_code` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL DEFAULT '',
  `sender_name` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `person` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `ward` VARCHAR(4) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `spclty_id` INT(11) UNSIGNED NULL DEFAULT NULL,
  `route` TEXT NULL DEFAULT NULL,
  `reference` LONGTEXT NULL DEFAULT NULL,
  PRIMARY KEY (`message_id`) USING BTREE,
  INDEX `message_datetime` (`message_datetime`) USING BTREE,
  INDEX `person` (`person`) USING BTREE,
  INDEX `ward` (`ward`) USING BTREE,
  INDEX `spclty_id` (`spclty_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

CREATE TABLE IF NOT EXISTS `message_read` (
	`message_read_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`message_id` INT(11) UNSIGNED NOT NULL,
	`read_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`read_datetime` DATETIME NOT NULL,
	PRIMARY KEY (`message_read_id`) USING BTREE,
	UNIQUE INDEX `id_user` (`message_id`, `read_user`) USING BTREE
) COLLATE='utf8mb4_general_ci' ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS `ipt_log` (
  `ipt_log_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `ipt_log_type` VARCHAR(1) NOT NULL,
  `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `vn` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `hn` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `ward` VARCHAR(4) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `create_datetime` DATETIME NOT NULL,
	PRIMARY KEY (`ipt_log_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;