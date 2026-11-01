CREATE TABLE `hos`.`religion` (
  `religion` char(2) NOT NULL,
  `name` varchar(40) DEFAULT NULL,
  `nhso_code` varchar(10) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  PRIMARY KEY (`religion`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci