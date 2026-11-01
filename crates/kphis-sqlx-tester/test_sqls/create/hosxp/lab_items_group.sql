CREATE TABLE `hos`.`lab_items_group` (
  `lab_items_group_code` int(11) NOT NULL,
  `lab_items_group_name` varchar(250) DEFAULT NULL,
  `lab_department_code` int(11) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  PRIMARY KEY (`lab_items_group_code`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci