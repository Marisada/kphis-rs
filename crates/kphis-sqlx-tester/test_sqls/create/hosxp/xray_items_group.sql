CREATE TABLE `hos`.`xray_items_group` (
  `xray_items_group` int(11) NOT NULL DEFAULT 0,
  `name` varchar(200) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `ecode` varchar(20) DEFAULT NULL,
  PRIMARY KEY (`xray_items_group`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci