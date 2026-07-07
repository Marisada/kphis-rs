CREATE TABLE `hos`.`moph_refer_expire_type` (
	`moph_refer_expire_type_id` INT(11) NOT NULL,
	`moph_refer_expire_type_name` VARCHAR(255) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`moph_refer_expire_type_ename` VARCHAR(255) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	PRIMARY KEY (`moph_refer_expire_type_id`) USING BTREE
) COLLATE='tis620_thai_ci' ENGINE=InnoDB;
