CREATE TABLE `kphis`.`ipd_vs_conscious` (
  `conscious_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `conscious_name` VARCHAR(50) DEFAULT NULL,
  `conscious_score` INT(2) DEFAULT NULL,
  `conscious_group` INT(2) DEFAULT NULL,
  PRIMARY KEY (`conscious_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;