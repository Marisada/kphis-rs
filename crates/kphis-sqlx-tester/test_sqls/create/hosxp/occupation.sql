CREATE TABLE `hos`.`occupation` (
  `name` varchar(50) DEFAULT NULL,
  `occupation` varchar(4) NOT NULL DEFAULT '',
  `nhso_code` varchar(4) DEFAULT NULL,
  `code506` varchar(10) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `zip09_code` varchar(10) DEFAULT NULL,
  `surveillance_occupation_id` varchar(2) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  `nhso_eclaim_occupation_code` varchar(3) DEFAULT NULL,
  PRIMARY KEY (`occupation`),
  UNIQUE KEY `ix_name` (`name`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci