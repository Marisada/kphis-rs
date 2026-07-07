CREATE TABLE `kphis_extra`.`ipd_dc_plan_tmp_dx` (
    `dx_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`dx_name` VARCHAR(250) NOT NULL,
	`dx_knowledge` TEXT NULL DEFAULT NULL,
	`dx_revisit` TEXT NULL DEFAULT NULL,
	`dx_prevention` TEXT NULL DEFAULT NULL,
	`create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
	PRIMARY KEY (`dx_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;