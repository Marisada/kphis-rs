CREATE TABLE `hos`.`icd9cm1` (
  `code` varchar(9) NOT NULL DEFAULT '',
  `name` varchar(200) DEFAULT NULL,
  `export_proced` char(1) DEFAULT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  `active_status` varchar(1) DEFAULT NULL,
  PRIMARY KEY (`code`),
  KEY `name` (`name`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci