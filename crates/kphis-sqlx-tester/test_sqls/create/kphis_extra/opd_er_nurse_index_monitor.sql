CREATE TABLE `kphis_extra`.`opd_er_nurse_index_monitor` (
  `monitor_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `action_id` INT(11) UNSIGNED NOT NULL,
  `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,
  `monitor_datetime` DATETIME DEFAULT NULL,
  `monitor_doctor` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `monitor_abnormal` VARCHAR(1) DEFAULT NULL,
  `monitor_result` TEXT DEFAULT NULL,
  `monitor_remark` TEXT DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`monitor_id`) USING BTREE,
  INDEX `action_id` (`action_id`) USING BTREE,
  INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;