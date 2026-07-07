CREATE TABLE `kphis_log`.`message_read` (
	`message_read_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`message_id` INT(11) UNSIGNED NOT NULL,
	`read_user` VARCHAR(250) COLLATE 'tis620_thai_ci' NOT NULL,
	`read_datetime` DATETIME NOT NULL,
	PRIMARY KEY (`message_read_id`) USING BTREE,
	UNIQUE INDEX `id_user` (`message_id`, `read_user`) USING BTREE
) COLLATE='utf8mb4_general_ci' ENGINE=InnoDB;