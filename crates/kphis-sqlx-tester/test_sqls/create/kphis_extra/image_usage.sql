CREATE TABLE `kphis_extra`.`image_usage` (
	`image_usage_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`usage_id` TINYINT(3) UNSIGNED NOT NULL,
	`usage_key_id` INT(11) UNSIGNED NOT NULL,
	`image_id` INT(11) UNSIGNED NOT NULL,
	`create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`image_usage_id`) USING BTREE,
	UNIQUE INDEX `usage_triple` (`usage_id`,`usage_key_id`,`image_id`) USING BTREE,
	INDEX `image_id` (`image_id`) USING BTREE
) ENGINE=InnoDB COLLATE='utf8mb4_general_ci';