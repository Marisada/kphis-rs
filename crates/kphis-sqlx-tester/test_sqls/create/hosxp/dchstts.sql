CREATE TABLE `hos`.`dchstts` (
  `dchstts` char(2) NOT NULL,
  `name` varchar(150) DEFAULT NULL,
  `nhso_dchstts` varchar(2) DEFAULT NULL,
  `oldcode` varchar(5) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  PRIMARY KEY (`dchstts`),
  KEY `ix_oldcode` (`oldcode`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci