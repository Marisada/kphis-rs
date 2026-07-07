CREATE TABLE `hos`.`lab_items_doctor` (
	`lab_items_user_id` INT(11) NOT NULL DEFAULT 0,
	`lab_items_code` INT(11) NULL DEFAULT NULL,
	`doctor_code` VARCHAR(7) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	PRIMARY KEY (`lab_items_user_id`),
	INDEX `ix_ix_doctor_code` (`doctor_code`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;