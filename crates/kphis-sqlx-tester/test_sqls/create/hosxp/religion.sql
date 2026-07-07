CREATE TABLE `hos`.`religion` (
	`religion` CHAR(2) NOT NULL DEFAULT '',
	`name` VARCHAR(40) NULL DEFAULT NULL,
	`nhso_code` VARCHAR(10) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	PRIMARY KEY (`religion`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_name` (`name`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;