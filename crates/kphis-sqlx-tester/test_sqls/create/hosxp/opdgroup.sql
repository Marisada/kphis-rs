CREATE TABLE `hos`.`opdgroup` (
  `groupname` varchar(250) NOT NULL DEFAULT '',
  `accessright` text DEFAULT NULL,
  `drug_access_level` tinyint(4) DEFAULT NULL,
  `visible_menu` text DEFAULT NULL,
  `viewallmenu` char(1) DEFAULT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`groupname`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci