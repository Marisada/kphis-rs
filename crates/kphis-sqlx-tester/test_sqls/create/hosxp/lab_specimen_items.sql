CREATE TABLE `hos`.`lab_specimen_items` (
	`specimen_code` INT(11) NOT NULL DEFAULT 0,
	`specimen_name` VARCHAR(150) NULL DEFAULT NULL,
	`specimen_note` VARCHAR(250) NULL DEFAULT NULL,
	`ecode` VARCHAR(10) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`colab_specimen_name` VARCHAR(100) NULL DEFAULT NULL,
	`colab_specimen_id` INT(11) NULL DEFAULT NULL,
	PRIMARY KEY (`specimen_code`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;