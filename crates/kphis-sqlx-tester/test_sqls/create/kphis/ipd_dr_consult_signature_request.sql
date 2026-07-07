CREATE TABLE `kphis`.`ipd_dr_consult_signature_request` (
  `consult_signature_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `consult_id` INT(11) UNSIGNED DEFAULT NULL,
  `consult_doctorcode_request` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `consult_doctorcode_request_person2` VARCHAR(7) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `an` VARCHAR(13) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' DEFAULT NULL,
  `create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `create_datetime` DATETIME NOT NULL,
  `update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
  `update_datetime` DATETIME NOT NULL,
  `version` INT(11) NOT NULL,
  PRIMARY KEY (`consult_signature_id`) USING BTREE,
  INDEX `consult_id` (`consult_id`) USING BTREE,
  INDEX `an` (`an`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;