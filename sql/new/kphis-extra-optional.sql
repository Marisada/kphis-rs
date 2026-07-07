CREATE TABLE IF NOT EXISTS `usage` (
    `usage_id` TINYINT(3) UNSIGNED NOT NULL,
    `usage_name` VARCHAR(50) NOT NULL,
    PRIMARY KEY (`usage_id`) USING BTREE
) ENGINE = MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;

INSERT INTO `usage` (`usage_id`, `usage_name`) VALUES
	(1, 'IpdDrAdmissionNote'),
	(2, 'OpdErMedicalHistory'),
	(3, 'IpdProgressNote'),
	(4, 'OpdErProgressNote'),
	(5, 'IpdFocusNoteAssessment'),
	(6, 'OpdErFocusNoteAssessment'),
	(7, 'IpdFocusNoteEvaluation'),
	(8, 'OpdErFocusNoteEvaluation'),
	(9, 'IpdConsultData'),
	(10, 'IpdConsultFinding'),
	(11, 'IpdDocument'),
	(12, 'OpdErDocument');

CREATE TABLE IF NOT EXISTS `document_type` (
    `document_type_id` TINYINT(3) UNSIGNED NOT NULL,
    `document_name` VARCHAR(250) NOT NULL,
    PRIMARY KEY (`document_type_id`) USING BTREE
) ENGINE=MyISAM DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci ROW_FORMAT=DYNAMIC;

INSERT INTO `document_type` (`document_type_id`, `document_name`) VALUES
	(1, 'InformedConsent'),
	(2, 'InsureCheck'),
	(3, 'ReferIn'),
	(4, 'ReferOut'),
	(5, 'CulturePatho'),
	(6, 'Blood'),
	(7, 'SpecialLab'),            
	(8, 'EKG'),
	(9, 'Xray'),
	(10, 'CT'),
	(11, 'MRI'),
	(12, 'Operation'),
	(13, 'Anesthesia'),
	(14, 'Labour'),
	(15, 'Physiotherapy'),
	(16, 'AlternativeRx'),
    (17, 'Nutrition');
	(18, 'Others');