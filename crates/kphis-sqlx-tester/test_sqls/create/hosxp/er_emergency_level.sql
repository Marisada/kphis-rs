CREATE TABLE `hos`.`er_emergency_level` (
	`er_emergency_level_id` INT(11) NOT NULL DEFAULT 0,
	`er_emergency_level_name` VARCHAR(200) NULL DEFAULT NULL,
	PRIMARY KEY (`er_emergency_level_id`),
	UNIQUE INDEX `ix_er_emergency_level_name` (`er_emergency_level_name`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;