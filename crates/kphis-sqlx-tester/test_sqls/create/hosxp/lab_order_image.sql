CREATE TABLE `hos`.`lab_order_image` (
  `lab_order_number` int(11) NOT NULL,
  `image1` longblob DEFAULT NULL,
  `image1_note` text DEFAULT NULL,
  `image2` longblob DEFAULT NULL,
  `image2_note` text DEFAULT NULL,
  `image3` longblob DEFAULT NULL,
  `image3_note` text DEFAULT NULL,
  `image4` longblob DEFAULT NULL,
  `image4_note` text DEFAULT NULL,
  `image5` longblob DEFAULT NULL,
  `image5_note` text DEFAULT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  PRIMARY KEY (`lab_order_number`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci