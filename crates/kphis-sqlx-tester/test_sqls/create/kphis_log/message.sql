CREATE TABLE `kphis_log`.`message` (
	`message_id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
	`message_datetime` DATETIME NULL DEFAULT NULL,
	`message` TEXT NULL DEFAULT NULL,
	`sender_code` VARCHAR(7) NOT NULL DEFAULT '' COLLATE 'tis620_thai_ci',
	`sender_name` VARCHAR(250) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`person` VARCHAR(7) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`ward` VARCHAR(4) COLLATE 'tis620_thai_ci' NULL DEFAULT NULL,
	`spclty_id` INT(11) UNSIGNED NULL DEFAULT NULL,
	`route` TEXT NULL DEFAULT NULL,
	`reference` LONGTEXT NULL DEFAULT NULL,
	PRIMARY KEY (`message_id`),
	INDEX `ward` (`ward`),
	INDEX `spclty_id` (`spclty_id`),
	INDEX `message_datetime` (`message_datetime`),
	INDEX `person` (`person`),
	INDEX `sender_code` (`sender_code`)
) COLLATE='utf8mb4_general_ci' ENGINE=InnoDB;