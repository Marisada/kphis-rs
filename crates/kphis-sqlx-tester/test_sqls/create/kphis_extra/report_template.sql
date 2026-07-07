CREATE TABLE `kphis_extra`.`report_template` (
	`template_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`template_name` VARCHAR(250) NOT NULL,
	`title` VARCHAR(250) NOT NULL,
	`content` TEXT NOT NULL,
	`statement` TEXT NULL DEFAULT NULL,
	`statement_params` TEXT NULL DEFAULT NULL,
	`info` TEXT NULL DEFAULT NULL,
    `disabled` TINYINT(1) NULL DEFAULT NULL,
	`create_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`template_id`) USING BTREE,
    UNIQUE INDEX `template_name` (`template_name`) USING BTREE
) ENGINE=InnoDB CHARACTER SET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;