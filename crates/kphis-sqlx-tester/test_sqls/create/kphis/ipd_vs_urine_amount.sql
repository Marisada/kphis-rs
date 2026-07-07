CREATE TABLE `kphis`.`ipd_vs_urine_amount` (
  `urine_amount_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `urine_amount_name` VARCHAR(50) DEFAULT NULL,
  PRIMARY KEY (`urine_amount_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;