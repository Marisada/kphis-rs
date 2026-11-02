CREATE TABLE `hos`.`lab_items_doctor` (
  `lab_items_user_id` int(11) NOT NULL,
  `lab_items_code` int(11) NOT NULL DEFAULT 0,
  `doctor_code` varchar(7) NOT NULL DEFAULT '',
  `hos_guid` varchar(38) DEFAULT NULL,
  PRIMARY KEY (`lab_items_user_id`),
  KEY `ix_doctor_code` (`doctor_code`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci