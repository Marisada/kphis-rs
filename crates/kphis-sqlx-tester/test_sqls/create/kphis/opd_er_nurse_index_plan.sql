CREATE TABLE `kphis`.`opd_er_nurse_index_plan` (
  `plan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `order_item_id` INT(11) UNSIGNED DEFAULT NULL,
  `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,
  `plan_detail` TEXT DEFAULT NULL,
  `plan_date` DATE DEFAULT NULL,
  `plan_time` TIME DEFAULT NULL,
  `plan_sch_type` VARCHAR(10) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`plan_id`) USING BTREE,
  INDEX `opd_er_order_master_id_plan_date` (`opd_er_order_master_id`,`plan_date`) USING BTREE,
  INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE,
  INDEX `order_item_id` (`order_item_id`) USING BTREE,
  INDEX `order_item_id_plan_date` (`order_item_id`,`plan_date`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;