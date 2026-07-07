CREATE TABLE `kphis`.`ipd_vs_stage_of_change` (
  `stage_of_change_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `stage_of_change_name` VARCHAR(50) DEFAULT NULL,
  PRIMARY KEY (`stage_of_change_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;
