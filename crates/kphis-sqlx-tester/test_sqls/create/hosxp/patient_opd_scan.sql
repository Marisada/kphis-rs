CREATE TABLE `hos`.`patient_opd_scan` (
	`patient_opd_scan_id` INT(11) NOT NULL DEFAULT 0,
	`vn` VARCHAR(13) NULL DEFAULT NULL,
	`hn` VARCHAR(9) NULL DEFAULT NULL,
	`vstdate` DATE NULL DEFAULT NULL,
	`vsttime` TIME NULL DEFAULT NULL,
	`image` LONGBLOB NULL DEFAULT NULL,
	`scan_date_time` DATETIME NULL DEFAULT NULL,
	`staff` VARCHAR(20) NULL DEFAULT NULL,
	`page_no` INT(11) NULL DEFAULT NULL,
	`thumbnail` LONGBLOB NULL DEFAULT NULL,
	PRIMARY KEY (`patient_opd_scan_id`),
	INDEX `ix_hn` (`hn`),
	INDEX `ix_scan_date_time` (`scan_date_time`),
	INDEX `ix_staff` (`staff`),
	INDEX `ix_vn` (`vn`),
	INDEX `ix_vstdate` (`vstdate`)
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;