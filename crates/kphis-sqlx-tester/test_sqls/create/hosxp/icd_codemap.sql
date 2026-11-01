CREATE TABLE `hos`.`icd_codemap` (
  `code` varchar(250) NOT NULL DEFAULT '',
  `icd10` varchar(7) DEFAULT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`code`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`),
  KEY `ix_icd10` (`icd10`),
  KEY `ix_code_icd10` (`code`,`icd10`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci