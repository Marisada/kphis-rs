CREATE TABLE IF NOT EXISTS `kphis_log`.`message` (
  `message_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `message_datetime` DATETIME DEFAULT NULL,
  `message` TEXT NULL DEFAULT NULL,
  `sender_code` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL DEFAULT '',
  `sender_name` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `person` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `ward` VARCHAR(4) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `spclty_id` INT(11) UNSIGNED DEFAULT NULL,
  `route` TEXT NULL DEFAULT NULL,
  `reference` LONGTEXT NULL DEFAULT NULL,
  PRIMARY KEY (`message_id`) USING BTREE,
  INDEX `message_datetime` (`message_datetime`) USING BTREE,
  INDEX `person` (`person`) USING BTREE,
  INDEX `ward` (`ward`) USING BTREE,
  INDEX `spclty_id` (`spclty_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci ROW_FORMAT = DYNAMIC;

CREATE TABLE IF NOT EXISTS `kphis_log`.`message_read` (
	`message_read_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`message_id` INT(11) UNSIGNED NOT NULL,
	`read_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`read_datetime` DATETIME NOT NULL,
	PRIMARY KEY (`message_read_id`) USING BTREE,
	UNIQUE INDEX `id_user` (`message_id`, `read_user`) USING BTREE
) COLLATE='utf8mb4_general_ci' ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS `kphis_log`.`ipt_log` (
  `ipt_log_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `ipt_log_type` VARCHAR(1) NOT NULL,
  `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `vn` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `hn` VARCHAR(9) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `ward` VARCHAR(4) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
  `create_datetime` DATETIME NOT NULL,
	PRIMARY KEY (`ipt_log_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;