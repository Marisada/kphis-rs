CREATE TABLE `kphis_extra`.`opd_er_dc_plan_med_item` (
	`med_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,
	`med_id` INT(11) UNSIGNED NULL DEFAULT NULL,
	`create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`med_item_id`) USING BTREE,
	UNIQUE INDEX `plan_med` (`dc_plan_id`,`med_id`) USING BTREE,
	INDEX `med_id` (`med_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;