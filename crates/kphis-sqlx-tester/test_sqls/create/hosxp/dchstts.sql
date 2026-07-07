CREATE TABLE `hos`.`dchstts` (
	`dchstts` CHAR(2) NOT NULL DEFAULT '',
	`name` VARCHAR(150) NULL DEFAULT NULL,
	`nhso_dchstts` VARCHAR(2) NULL DEFAULT NULL,
	`oldcode` VARCHAR(5) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	PRIMARY KEY (`dchstts`),
	INDEX `ix_oldcode` (`oldcode`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;