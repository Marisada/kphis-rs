CREATE TABLE `hos`.`lab_items_visible` (
	`lab_items_code` INT(11) NOT NULL DEFAULT 0,
	`groupname` VARCHAR(250) NOT NULL DEFAULT '',
	`hos_guid` CHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	PRIMARY KEY (`lab_items_code`, `groupname`),
	INDEX `lab_items_code` (`lab_items_code`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`),
	INDEX `ix_lab_items_code_groupname` (`groupname`, `lab_items_code`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;