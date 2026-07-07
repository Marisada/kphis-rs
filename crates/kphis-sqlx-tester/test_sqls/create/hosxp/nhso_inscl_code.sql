CREATE TABLE `hos`.`nhso_inscl_code` (
	`inscl_code` VARCHAR(5) COLLATE 'tis620_thai_ci' NOT NULL,
	`inscl_name` VARCHAR(200) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	PRIMARY KEY (`inscl_code`) USING BTREE,
	UNIQUE INDEX `ix_inscl_name` (`inscl_name`) USING BTREE
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;