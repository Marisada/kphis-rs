CREATE TABLE `hos`.`marrystatus` (
	`code` CHAR(1) NOT NULL DEFAULT '',
	`name` VARCHAR(20) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`code506` CHAR(1) NULL DEFAULT NULL,
	`nhso_marriage_code` VARCHAR(3) NULL DEFAULT NULL,
	PRIMARY KEY (`code`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;