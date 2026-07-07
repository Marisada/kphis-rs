CREATE TABLE `kphis_extra`.`ipd_document` (
    `document_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
    `an` VARCHAR(13) COLLATE 'tis620_thai_ci' NOT NULL,
	`document_type_id` TINYINT(3) UNSIGNED NOT NULL,
	`create_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`create_datetime` DATETIME NOT NULL,
	`update_user` VARCHAR(250) CHARACTER SET tis620 COLLATE 'tis620_thai_ci' NOT NULL,
	`update_datetime` DATETIME NOT NULL,
	`version` INT(11) NOT NULL,
    PRIMARY KEY (`document_id`) USING BTREE,
	UNIQUE INDEX `an_type_id` (`an`,`document_type_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;