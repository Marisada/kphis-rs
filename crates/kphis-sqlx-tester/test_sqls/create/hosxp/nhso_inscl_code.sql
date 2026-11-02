CREATE TABLE `hos`.`nhso_inscl_code` (
  `inscl_code` varchar(5) NOT NULL,
  `inscl_name` varchar(200) NOT NULL,
  PRIMARY KEY (`inscl_code`),
  UNIQUE KEY `ix_inscl_name` (`inscl_name`)
) ENGINE=InnoDB DEFAULT CHARSET=tis620 COLLATE=tis620_thai_ci