CREATE TABLE `hos`.`nationality` (
	`name` VARCHAR(50) NULL DEFAULT NULL,
	`nationality` CHAR(3) NOT NULL DEFAULT '',
	`nhso_code` VARCHAR(3) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`zip09_code` VARCHAR(10) NULL DEFAULT NULL,
	`inv_unuse` CHAR(1) NULL DEFAULT NULL,
	PRIMARY KEY (`nationality`),
	UNIQUE INDEX `ix_name` (`name`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;