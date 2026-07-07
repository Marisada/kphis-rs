CREATE TABLE `hos`.`lab_items_group` (
	`lab_items_group_code` INT(11) NOT NULL DEFAULT 0,
	`lab_items_group_name` VARCHAR(250) NULL DEFAULT NULL,
	`lab_department_code` INT(11) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	PRIMARY KEY (`lab_items_group_code`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;