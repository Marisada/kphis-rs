CREATE TABLE `kphis_extra`.`opd_er_dc_plan_env_item` (
	`env_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,
	`env_id` INT(11) UNSIGNED NULL DEFAULT NULL,
	`create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`env_item_id`) USING BTREE,
	UNIQUE INDEX `plan_env` (`dc_plan_id`,`env_id`) USING BTREE,
	INDEX `env_id` (`env_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;