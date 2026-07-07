CREATE TABLE `hos`.`ovstist` (
	`name` VARCHAR(50) NOT NULL DEFAULT '',
	`ovstist` CHAR(2) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`export_code` VARCHAR(5) NULL DEFAULT NULL,
	`opbkk_code` CHAR(1) NULL DEFAULT NULL,
	`csmbs_code` VARCHAR(1) NULL DEFAULT NULL,
	PRIMARY KEY (`name`),
	UNIQUE INDEX `ovstist_unique` (`ovstist`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;