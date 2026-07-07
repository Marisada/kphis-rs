CREATE TABLE `hos`.`icd9cm1` (
	`code` VARCHAR(9) NOT NULL DEFAULT '',
	`name` VARCHAR(200) NULL DEFAULT NULL,
	`export_proced` CHAR(1) NULL DEFAULT NULL,
	`hos_guid` CHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	`active_status` VARCHAR(1) NULL DEFAULT NULL,
	PRIMARY KEY (`code`),
	INDEX `name` (`name`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;