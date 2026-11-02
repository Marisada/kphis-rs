CREATE TABLE `hos`.`refer_type` (
  `refer_type` int(11) NOT NULL,
  `refer_type_name` varchar(200) NOT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  PRIMARY KEY (`refer_type`),
  UNIQUE KEY `ix_refer_type_name` (`refer_type_name`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci