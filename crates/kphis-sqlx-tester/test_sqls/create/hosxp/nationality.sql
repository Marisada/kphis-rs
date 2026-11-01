CREATE TABLE `hos`.`nationality` (
  `name` varchar(50) DEFAULT NULL,
  `nationality` char(3) NOT NULL,
  `nhso_code` varchar(3) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `zip09_code` varchar(10) DEFAULT NULL,
  `inv_unuse` char(1) DEFAULT NULL,
  PRIMARY KEY (`nationality`),
  UNIQUE KEY `ix_name` (`name`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci