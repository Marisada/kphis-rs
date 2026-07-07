CREATE TABLE `hos`.`occupation` (
	`name` VARCHAR(50) NULL DEFAULT NULL,
	`occupation` VARCHAR(4) NOT NULL DEFAULT '',
	`nhso_code` VARCHAR(4) NULL DEFAULT NULL,
	`code506` VARCHAR(10) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`surveillance_occupation_id` VARCHAR(2) NULL DEFAULT NULL,
	`zip09_code` VARCHAR(10) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	`nhso_eclaim_occupation_code` VARCHAR(3) NULL DEFAULT NULL,
	PRIMARY KEY (`occupation`),
	UNIQUE INDEX `ix_name` (`name`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;