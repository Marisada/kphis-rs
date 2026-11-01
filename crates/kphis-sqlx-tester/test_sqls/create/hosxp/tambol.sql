CREATE TABLE `hos`.`tambol` (
  `tambol_code` char(6) NOT NULL,
  `tambol_name` varchar(150) NOT NULL,
  `district_code` char(4) NOT NULL,
  `full_address_name` varchar(200) DEFAULT NULL,
  PRIMARY KEY (`tambol_code`),
  UNIQUE KEY `ix_tambol_name_uni` (`district_code`,`tambol_name`),
  KEY `ix_district_code` (`district_code`)
) ENGINE=MyISAM DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci