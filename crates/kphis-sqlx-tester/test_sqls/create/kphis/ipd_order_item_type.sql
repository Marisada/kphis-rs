CREATE TABLE `kphis`.`ipd_order_item_type` (
  `order_type` VARCHAR(20) NOT NULL DEFAULT '',
  `order_item_type` VARCHAR(20) NOT NULL,
  `order_item_type_name` VARCHAR(20) NOT NULL,
  `display_order` INT(11) DEFAULT NULL,
  PRIMARY KEY (`order_type`,`order_item_type`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;