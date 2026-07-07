CREATE TABLE `hos`.`refer_point_list` (
	`name` VARCHAR(50) NOT NULL DEFAULT '' COLLATE 'tis620_thai_ci',
	`hos_guid` VARCHAR(38) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	PRIMARY KEY (`name`) USING BTREE,
	INDEX `ix_hos_guid` (`hos_guid`) USING BTREE,
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`) USING BTREE
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;
