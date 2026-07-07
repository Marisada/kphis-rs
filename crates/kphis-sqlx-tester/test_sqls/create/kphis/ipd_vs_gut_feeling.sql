CREATE TABLE `kphis`.`ipd_vs_gut_feeling` (
  `gut_feeling_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `gut_feeling_name` VARCHAR(50) DEFAULT NULL,
  PRIMARY KEY (`gut_feeling_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;