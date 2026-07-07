CREATE TABLE `kphis_extra`.`image` (
	`image_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`path` CHAR(33) NOT NULL,
	`title` TEXT NULL DEFAULT NULL,
	`create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`image_id`) USING BTREE,
	UNIQUE INDEX `path` (`path`) USING BTREE,
	INDEX `create_user` (`create_user`) USING BTREE
) ENGINE=InnoDB COLLATE='utf8mb4_general_ci';