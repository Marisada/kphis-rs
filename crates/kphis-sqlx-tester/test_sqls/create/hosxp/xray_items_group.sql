CREATE TABLE `hos`.`xray_items_group` (
	`xray_items_group` INT(11) NOT NULL DEFAULT 0,
	`name` VARCHAR(200) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`ecode` VARCHAR(20) NULL DEFAULT NULL,
	PRIMARY KEY (`xray_items_group`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;