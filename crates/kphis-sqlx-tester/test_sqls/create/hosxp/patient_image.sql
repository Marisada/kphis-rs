CREATE TABLE `hos`.`patient_image` (
	`hn` VARCHAR(9) NOT NULL DEFAULT '',
	`image_name` VARCHAR(150) NOT NULL DEFAULT '',
	`image` LONGBLOB NULL DEFAULT NULL,
	`width` INT(11) NULL DEFAULT NULL,
	`height` INT(11) NULL DEFAULT NULL,
	`capture_date` DATETIME NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	PRIMARY KEY (`hn`, `image_name`),
	INDEX `hn` (`hn`),
	INDEX `image_name` (`image_name`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`)
) COLLATE='tis620_thai_ci' ENGINE=MyISAM;