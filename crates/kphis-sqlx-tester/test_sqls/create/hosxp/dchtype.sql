CREATE TABLE `hos`.`dchtype` (
  `dchtype` char(2) NOT NULL,
  `name` varchar(150) DEFAULT NULL,
  `nhso_dchtype` varchar(2) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  PRIMARY KEY (`dchtype`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci