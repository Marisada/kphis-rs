INSERT INTO `kphis_log`.`message` (`message_id`, `message_datetime`, `message`, `sender_code`, `sender_name`, `person`, `ward`, `spclty_id`, `route`, `reference`) VALUES
	(1, '2024-01-01 11:11:11', 'message1', '007', 'senderA', '009', "01", 1, '#/info', NULL),
    (2, '2024-01-01 11:11:11', 'message2', '007', 'senderA', NULL, NULL, NULL, NULL, NULL),
    (3, '2024-01-01 11:11:11', 'message3', '009', 'senderB', '009', "01", 1, '#/info', NULL),
    (4, '2024-01-01 11:11:11', 'message4', '009', 'senderB', NULL, NULL, NULL, NULL, NULL),
    (5, '2024-01-01 11:11:11', 'message5', '007', 'senderB', '009', "01", 1, '#/info', '{"message_id": 1,"message_datetime": "2024-01-01 11:11:11","message": "message1","sender_code": "007","sender_name": "user","person": "009","ward": "01","spclty_id": 1,"route": "#/info","reference": "{\"message_id\": 1,\"message_datetime\": \"2023-12-31 11:11:11.0\",\"message\": \"Previous\",\"sender_code\": \"009\",\"sender_name\": \"Mr.Previous\",\"person\": \"007\",\"ward\": \"01\",\"spclty_id\": 1,\"route\": \"#/info\",\"reference\": null,\"readed\": 0}","readed": 0}'),
    (6, '2024-01-01 11:11:11', 'message6', '007', 'senderB', NULL, NULL, NULL, NULL, NULL);