CREATE TABLE `hos`.`lab_specimen_items` (
  `specimen_code` int(11) NOT NULL,
  `specimen_name` varchar(150) DEFAULT NULL,
  `specimen_note` varchar(250) DEFAULT NULL,
  `ecode` varchar(10) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `colab_specimen_name` varchar(100) DEFAULT NULL,
  `colab_specimen_id` int(11) DEFAULT NULL,
  PRIMARY KEY (`specimen_code`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci