CREATE TABLE `hos`.`patient_image` (
  `hn` varchar(9) NOT NULL,
  `image_name` varchar(150) NOT NULL,
  `image` longblob DEFAULT NULL,
  `width` int(11) DEFAULT NULL,
  `height` int(11) DEFAULT NULL,
  `capture_date` datetime DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`hn`,`image_name`),
  KEY `hn` (`hn`),
  KEY `image_name` (`image_name`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci