CREATE TABLE `hos`.`refer_cause` (
	`id` INT(11) NOT NULL DEFAULT '0',
	`name` VARCHAR(250) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	PRIMARY KEY (`id`) USING BTREE,
	INDEX `ix_hos_guid` (`hos_guid`) USING BTREE
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;
