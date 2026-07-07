CREATE TABLE `hos`.`roomno` (
	`roomno` VARCHAR(4) NOT NULL DEFAULT '' COLLATE 'tis620_thai_ci',
	`name` VARCHAR(150) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`ward` VARCHAR(4) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`spclty` CHAR(2) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`an` VARCHAR(9) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`display_number` INT(11) NULL DEFAULT NULL,
	`roomtype` INT(11) NULL DEFAULT NULL,
	`room_status_type_id` INT(11) NULL DEFAULT NULL,
	`hos_guid` VARCHAR(38) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`lock_reserve` CHAR(1) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`export_code` VARCHAR(20) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	PRIMARY KEY (`roomno`) USING BTREE,
	INDEX `an` (`an`) USING BTREE,
	INDEX `ix_hos_guid` (`hos_guid`) USING BTREE
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;