CREATE TABLE `hos`.`er_emergency_level` (
  `er_emergency_level_id` int(11) NOT NULL,
  `er_emergency_level_name` varchar(200) NOT NULL,
  PRIMARY KEY (`er_emergency_level_id`),
  UNIQUE KEY `ix_er_emergency_level_name` (`er_emergency_level_name`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci