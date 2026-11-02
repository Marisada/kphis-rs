CREATE TABLE `hos`.`sys_var` (
  `sys_name` varchar(250) NOT NULL DEFAULT '',
  `sys_value` varchar(250) DEFAULT NULL,
  `sys_var_guid` varchar(38) DEFAULT NULL,
  `hos_guid` varchar(38) DEFAULT NULL,
  `hos_guid_ext` varchar(64) DEFAULT NULL,
  PRIMARY KEY (`sys_name`),
  KEY `ix_sys_var_guid` (`sys_var_guid`),
  KEY `ix_hos_guid_ext` (`hos_guid_ext`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci