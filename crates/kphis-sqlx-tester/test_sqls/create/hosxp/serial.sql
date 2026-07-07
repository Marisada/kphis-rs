CREATE TABLE `hos`.`serial` (
	`name` VARCHAR(50) NOT NULL DEFAULT '',
	`serial_no` INT(11) NULL DEFAULT NULL,
	`node_id` CHAR(1) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	PRIMARY KEY (`name`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;