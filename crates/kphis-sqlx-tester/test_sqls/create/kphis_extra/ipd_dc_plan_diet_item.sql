CREATE TABLE `kphis_extra`.`ipd_dc_plan_diet_item` (
	`diet_item_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`dc_plan_id` INT(11) UNSIGNED NULL DEFAULT NULL,
	`diet_id` INT(11) UNSIGNED NULL DEFAULT NULL,
	`create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`diet_item_id`) USING BTREE,
	UNIQUE INDEX `plan_diet` (`dc_plan_id`,`diet_id`) USING BTREE,
	INDEX `diet_id` (`diet_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;