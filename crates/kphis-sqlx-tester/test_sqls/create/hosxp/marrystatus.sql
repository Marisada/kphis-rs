CREATE TABLE `hos`.`marrystatus` (
  `code` char(1) NOT NULL,
  `name` varchar(20) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `code506` char(1) DEFAULT NULL,
  `nhso_marriage_code` varchar(3) DEFAULT NULL,
  PRIMARY KEY (`code`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci