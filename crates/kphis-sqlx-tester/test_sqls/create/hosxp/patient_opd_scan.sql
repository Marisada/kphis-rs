CREATE TABLE `hos`.`patient_opd_scan` (
  `patient_opd_scan_id` int(11) NOT NULL,
  `vn` varchar(13) NOT NULL,
  `hn` varchar(9) DEFAULT NULL,
  `vstdate` date DEFAULT NULL,
  `vsttime` time DEFAULT NULL,
  `image` longblob DEFAULT NULL,
  `scan_date_time` datetime DEFAULT NULL,
  `staff` varchar(20) DEFAULT NULL,
  `page_no` int(11) NOT NULL,
  `thumbnail` longblob DEFAULT NULL,
  PRIMARY KEY (`patient_opd_scan_id`),
  KEY `ix_hn` (`hn`),
  KEY `ix_scan_date_time` (`scan_date_time`),
  KEY `ix_staff` (`staff`),
  KEY `ix_vn` (`vn`),
  KEY `ix_vstdate` (`vstdate`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci