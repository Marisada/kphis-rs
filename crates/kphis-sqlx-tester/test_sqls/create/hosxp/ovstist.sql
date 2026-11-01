CREATE TABLE `hos`.`ovstist` (
  `name` varchar(50) NOT NULL,
  `ovstist` char(2) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `export_code` varchar(5) DEFAULT NULL,
  `opbkk_code` char(1) DEFAULT NULL,
  `csmbs_code` varchar(1) DEFAULT NULL,
  PRIMARY KEY (`name`),
  UNIQUE KEY `ovstist_unique` (`ovstist`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci