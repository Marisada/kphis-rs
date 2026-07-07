CREATE TABLE `kphis`.`opd_er_document_scan` (
  `opd_er_document_scan_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `opd_er_order_master_id` INT(11) UNSIGNED NOT NULL,
  `opd_er_document_scan` VARCHAR(1) DEFAULT NULL,
  `opd_er_document_scan_doctorcode` VARCHAR(7) DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_datetime` DATETIME DEFAULT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `update_datetime` DATETIME DEFAULT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`opd_er_document_scan_id`) USING BTREE,
  UNIQUE INDEX `opd_er_order_master_id` (`opd_er_order_master_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=COMPACT;