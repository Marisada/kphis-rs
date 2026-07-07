CREATE TABLE `kphis`.`opd_er_set_fast_track` (
  `set_ft_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `opd_er_order_master_id` INT(11) UNSIGNED DEFAULT NULL,
  `set_ft_date` DATE DEFAULT NULL,
  `set_ft_time` TIME DEFAULT NULL,
  `set_ft_doctorcode` VARCHAR(7) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`set_ft_id`) USING BTREE,
  UNIQUE INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;