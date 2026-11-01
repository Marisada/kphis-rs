CREATE TABLE `hos`.`lab_items_visible` (
  `lab_items_code` int(11) NOT NULL,
  `groupname` varchar(250) NOT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`lab_items_code`,`groupname`),
  KEY `lab_items_code` (`lab_items_code`),
  KEY `lab_items_code_groupname` (`lab_items_code`,`groupname`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`),
  KEY `ix_lab_items_code_groupname` (`groupname`,`lab_items_code`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci