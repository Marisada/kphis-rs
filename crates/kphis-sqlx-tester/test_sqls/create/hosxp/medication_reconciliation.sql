CREATE TABLE `hos`.`medication_reconciliation` (
  `medication_reconciliation_id` int(11) NOT NULL,
  `an` varchar(9) NOT NULL,
  `medication_survey` char(1) DEFAULT NULL,
  `last_receive_medication_date` date DEFAULT NULL,
  `last_recieve_medication_duration_day` int(11) DEFAULT NULL,
  `staff` varchar(25) DEFAULT NULL,
  `doctor` varchar(25) DEFAULT NULL,
  `note` text DEFAULT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  `last_receive_date` date DEFAULT NULL,
  `last_receive_duration_day` int(11) DEFAULT NULL,
  PRIMARY KEY (`medication_reconciliation_id`),
  UNIQUE KEY `ix_an` (`an`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci