CREATE TABLE `hos`.`refer_cause` (
  `id` int(11) NOT NULL DEFAULT 0,
  `name` varchar(250) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `ix_hos_guid` (`hos_guid`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci