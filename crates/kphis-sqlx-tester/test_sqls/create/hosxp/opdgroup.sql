CREATE TABLE `hos`.`opdgroup` (
	`groupname` VARCHAR(250) NOT NULL DEFAULT '',
	`accessright` TEXT NULL DEFAULT NULL,
	`drug_access_level` TINYINT(4) NULL DEFAULT NULL,
	`visible_menu` TEXT NULL DEFAULT NULL,
	`viewallmenu` CHAR(1) NULL DEFAULT NULL,
	`hos_guid` CHAR(38) NULL DEFAULT NULL,
	`hos_guid_ext` VARCHAR(64) NULL DEFAULT NULL,
	PRIMARY KEY (`groupname`),
	INDEX `ix_hos_guid` (`hos_guid`),
	INDEX `ix_hos_guid_ext` (`hos_guid_ext`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;