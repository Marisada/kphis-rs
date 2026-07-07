CREATE TABLE `hos`.`sex` (
	`code` CHAR(1) NOT NULL DEFAULT '',
	`name` VARCHAR(10) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	PRIMARY KEY (`code`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;