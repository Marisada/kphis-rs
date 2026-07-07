CREATE TABLE `hos`.`lab_order_image` (
	`lab_order_number` INT(11) NOT NULL DEFAULT 0,
	`image1` LONGBLOB NULL DEFAULT NULL,
	`image1_note` TEXT NULL DEFAULT NULL,
	`image2` LONGBLOB NULL DEFAULT NULL,
	`image2_note` TEXT NULL DEFAULT NULL,
	`image3` LONGBLOB NULL DEFAULT NULL,
	`image3_note` TEXT NULL DEFAULT NULL,
	`image4` LONGBLOB NULL DEFAULT NULL,
	`image4_note` TEXT NULL DEFAULT NULL,
	`image5` LONGBLOB NULL DEFAULT NULL,
	`image5_note` TEXT NULL DEFAULT NULL,
	`hos_guid` CHAR(38) NULL DEFAULT NULL,
	PRIMARY KEY (`lab_order_number`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=MyISAM;