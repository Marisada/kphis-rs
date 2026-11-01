CREATE TABLE `hos`.`er_image` (
  `vn` varchar(13) NOT NULL DEFAULT '',
  `image1` longblob DEFAULT NULL,
  `image2` longblob DEFAULT NULL,
  `image3` longblob DEFAULT NULL,
  `image4` longblob DEFAULT NULL,
  `image5` longblob DEFAULT NULL,
  `image1_note` text DEFAULT NULL,
  `image2_note` text DEFAULT NULL,
  `image3_note` text DEFAULT NULL,
  `image4_note` text DEFAULT NULL,
  `image5_note` text DEFAULT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  PRIMARY KEY (`vn`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci