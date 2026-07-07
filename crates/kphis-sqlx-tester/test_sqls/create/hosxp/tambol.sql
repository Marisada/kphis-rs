CREATE TABLE `hos`.`tambol` (
	`tambol_code` CHAR(6) NOT NULL,
	`tambol_name` VARCHAR(150) NOT NULL,
	`district_code` CHAR(4) NOT NULL,
	`full_address_name` VARCHAR(200) NULL DEFAULT NULL,
	PRIMARY KEY (`tambol_code`),
	UNIQUE INDEX `ix_tambol_name_uni` (`tambol_name`, `district_code`),
	INDEX `ix_district_code` (`district_code`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;
