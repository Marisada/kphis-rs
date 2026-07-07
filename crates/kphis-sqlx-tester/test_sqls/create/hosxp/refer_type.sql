CREATE TABLE `hos`.`refer_type` (
	`refer_type` INT(11) NOT NULL DEFAULT '0',
	`refer_type_name` VARCHAR(200) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	PRIMARY KEY (`refer_type`) USING BTREE,
	UNIQUE INDEX `ix_refer_type_name` (`refer_type_name`) USING BTREE,
	INDEX `ix_hos_guid` (`hos_guid`) USING BTREE
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;
