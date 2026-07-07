CREATE TABLE `hos`.`dchtype` (
	`dchtype` CHAR(2) NOT NULL DEFAULT '',
	`name` VARCHAR(150) NULL DEFAULT NULL,
	`nhso_dchtype` VARCHAR(2) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	PRIMARY KEY (`dchtype`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;