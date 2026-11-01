CREATE TABLE `hos`.`er_emergency_type` (
  `er_emergency_type` tinyint(4) NOT NULL DEFAULT 0,
  `name` varchar(50) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  `export_code` varchar(1) DEFAULT NULL,
  PRIMARY KEY (`er_emergency_type`),
  KEY `er_emergency_type` (`er_emergency_type`),
  KEY `ix_name` (`name`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci