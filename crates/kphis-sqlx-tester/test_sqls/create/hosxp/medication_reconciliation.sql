CREATE TABLE `hos`.`medication_reconciliation` (
	`medication_reconciliation_id` INT(11) NOT NULL DEFAULT 0,
	`an` VARCHAR(13) NULL DEFAULT NULL,
	`medication_survey` CHAR(1) NULL DEFAULT NULL,
	`last_receive_medication_date` DATE NULL DEFAULT NULL,
	`last_recieve_medication_duration_day` INT(11) NULL DEFAULT NULL,
	`staff` VARCHAR(25) NULL DEFAULT NULL,
	`doctor` VARCHAR(25) NULL DEFAULT NULL,
	`note` TEXT NULL DEFAULT NULL,
	`hos_guid` CHAR(38) NULL DEFAULT NULL,
	`last_receive_date` DATE NULL DEFAULT NULL,
	`last_receive_duration_day` INT(11) NULL DEFAULT NULL,
	PRIMARY KEY (`medication_reconciliation_id`),
	UNIQUE INDEX `ix_an` (`an`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;