CREATE TABLE `kphis`.`ipd_vs_urine_duration` (
  `urine_d_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `urine_d_name` VARCHAR(50) DEFAULT NULL,
  PRIMARY KEY (`urine_d_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;