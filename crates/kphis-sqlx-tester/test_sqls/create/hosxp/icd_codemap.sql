CREATE TABLE `hos`.`icd_codemap` (
	`code` VARCHAR(250) NOT NULL DEFAULT '',
	`icd10` VARCHAR(7) NULL DEFAULT NULL,
	`hos_guid` CHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	PRIMARY KEY (`code`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`),
	INDEX `icd10` (`icd10`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;