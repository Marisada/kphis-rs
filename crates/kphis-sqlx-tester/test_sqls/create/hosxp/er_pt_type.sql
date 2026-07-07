CREATE TABLE `hos`.`er_pt_type` (
	`er_pt_type` TINYINT(4) NOT NULL DEFAULT 0,
	`name` VARCHAR(50) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	`accident_code` CHAR(1) NULL DEFAULT NULL,
	`ucae` VARCHAR(1) NULL DEFAULT NULL,
	PRIMARY KEY (`er_pt_type`),
	INDEX `ix_name` (`name`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;