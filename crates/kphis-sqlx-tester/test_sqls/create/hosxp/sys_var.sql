CREATE TABLE `hos`.`sys_var` (
	`sys_name` VARCHAR(250) NOT NULL DEFAULT '',
	`sys_value` VARCHAR(250) NULL DEFAULT NULL,
	`sys_var_guid` VARCHAR(38) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	PRIMARY KEY (`sys_name`),
	INDEX `ix_sys_var_guid` (`sys_var_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;