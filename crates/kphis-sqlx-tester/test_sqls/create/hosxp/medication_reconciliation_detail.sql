CREATE TABLE `hos`.`medication_reconciliation_detail` (
	`medication_reconciliation_detail_id` INT(11) NOT NULL DEFAULT 0,
	`medication_reconciliation_id` INT(11) NULL DEFAULT NULL,
	`medication_name` VARCHAR(150) NULL DEFAULT NULL,
	`receive_location` VARCHAR(150) NULL DEFAULT NULL,
	`last_receive_date` DATE NULL DEFAULT NULL,
	`doctor_reconciliation_command_id` INT(11) NULL DEFAULT NULL,
	`medication_reconciliation_manage_id` INT(11) NULL DEFAULT NULL,
	`medication_change_cause` VARCHAR(250) NULL DEFAULT NULL,
	`usage_name` VARCHAR(250) NULL DEFAULT NULL,
	`hos_guid` CHAR(38) NULL DEFAULT NULL,
	`qty` INT(11) NULL DEFAULT NULL,
	`first_entry_date` DATETIME NULL DEFAULT NULL,
	PRIMARY KEY (`medication_reconciliation_detail_id`),
	INDEX `ix_hos_guid` (`hos_guid`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;