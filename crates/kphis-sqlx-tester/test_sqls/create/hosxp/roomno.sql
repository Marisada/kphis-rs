CREATE TABLE `hos`.`roomno` (
  `roomno` varchar(4) NOT NULL DEFAULT '',
  `name` varchar(150) DEFAULT NULL,
  `ward` varchar(4) DEFAULT NULL,
  `spclty` char(2) DEFAULT NULL,
  `an` varchar(9) DEFAULT NULL,
  `display_number` int(11) DEFAULT NULL,
  `roomtype` int(11) DEFAULT NULL,
  `room_status_type_id` int(11) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `lock_reserve` char(1) DEFAULT NULL,
  `export_code` varchar(20) DEFAULT NULL,
  PRIMARY KEY (`roomno`),
  KEY `an` (`an`),
  KEY `ix_hos_guid` (`hos_guid`),
  KEY `ix_ward` (`ward`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci