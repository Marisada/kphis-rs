CREATE TABLE `hos`.`medication_reconciliation_detail` (
  `medication_reconciliation_detail_id` int(11) NOT NULL,
  `medication_reconciliation_id` int(11) NOT NULL,
  `medication_name` varchar(150) NOT NULL,
  `receive_location` varchar(150) DEFAULT NULL,
  `last_receive_date` date DEFAULT NULL,
  `doctor_reconciliation_command_id` int(11) DEFAULT NULL,
  `medication_reconciliation_manage_id` int(11) DEFAULT NULL,
  `medication_change_cause` varchar(250) DEFAULT NULL,
  `usage_name` varchar(250) DEFAULT NULL,
  `hos_guid` char(38) DEFAULT NULL,
  `qty` int(11) DEFAULT NULL,
  `first_entry_date` datetime DEFAULT NULL,
  PRIMARY KEY (`medication_reconciliation_detail_id`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci