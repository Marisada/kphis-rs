use crate::TABLE_CREATE_COLUMNS;

const KPHIS_TABLES_WITH_AN: [&str;21] = [
    "ipd_doctor_in_charge", // has HN
    "ipd_dr_admission_note", // has HN, AN is unique
    "ipd_dr_admission_note_item",
    "ipd_dr_consult",
    "ipd_dr_consult_signature_reply",
    "ipd_dr_consult_signature_request",
    "ipd_focus_list", // has HN
    "ipd_focus_note", // has HN
    "ipd_io",
    "ipd_med_reconciliation",
    "ipd_med_reconciliation_item",
    "ipd_nurse_admission_note", // has HN, AN is unique
    "ipd_nurse_index_action",
    "ipd_nurse_index_note",// AN is unique
    "ipd_nurse_index_plan",
    "ipd_order",
    "ipd_order_item",
    "ipd_progress_note",
    "ipd_progress_note_item",
    // "ipd_summary", // has HN
    "ipd_summary_2", // AN is unique
    "ipd_vs_vital_sign", // has HN
    // "system_patient_lock",
];

const KPHIS_EXTRA_TABLES_WITH_AN: [&str;3] = [
    "ipd_document",
    "ipd_mra",
    "ipd_nurse_index_monitor"
];

// ===== ===== //
//   trigger   //
// ===== ===== //

pub fn select_exists_trg_kphis_ipt_log_insert(hosxp: &str) -> String {
    ["SELECT EXISTS(SELECT * FROM information_schema.TRIGGERS WHERE TRIGGER_NAME='trg_kphis_ipt_log_insert' AND TRIGGER_SCHEMA='",hosxp,"');"].concat()
}

pub fn select_exists_trg_kphis_ipt_log_delete(hosxp: &str) -> String {
    ["SELECT EXISTS(SELECT * FROM information_schema.TRIGGERS WHERE TRIGGER_NAME='trg_kphis_ipt_log_delete' AND TRIGGER_SCHEMA='",hosxp,"');"].concat()
}

pub fn drop_trg_kphis_ipt_log_insert(hosxp: &str) -> String {
    ["DROP TRIGGER IF EXISTS `", hosxp, "`.`trg_kphis_ipt_log_insert`;"].concat()
}

pub fn create_trg_kphis_ipt_log_insert(hosxp: &str, kphis_log: &str) -> String {
    [
        "CREATE TRIGGER `",hosxp,"`.`trg_kphis_ipt_log_insert` \
        AFTER INSERT ON `",hosxp,"`.`ipt` \
        FOR EACH ROW \
            INSERT INTO `",kphis_log,"`.`ipt_log` (ipt_log_type,an,vn,hn,ward,create_datetime) VALUES ('I',NEW.an,NEW.vn,NEW.hn,NEW.ward,NOW());",
    ].concat()
}

pub fn drop_trg_kphis_ipt_log_delete(hosxp: &str) -> String {
    ["DROP TRIGGER IF EXISTS `", hosxp, "`.`trg_kphis_ipt_log_delete`;"].concat()
}

pub fn create_trg_kphis_ipt_log_delete(hosxp: &str, kphis_log: &str) -> String {
    [
        "CREATE TRIGGER `",hosxp,"`.`trg_kphis_ipt_log_delete` \
        AFTER DELETE ON `",hosxp,"`.`ipt` \
        FOR EACH ROW \
            INSERT INTO `",kphis_log,"`.`ipt_log` (ipt_log_type,an,vn,hn,ward,create_datetime) VALUES ('D',OLD.an,OLD.vn,OLD.hn,OLD.ward,NOW());",
    ]
    .concat()
}

pub fn drop_trg_ipt_log_insert(kphis_log: &str) -> String {
    ["DROP TRIGGER IF EXISTS `", kphis_log, "`.`trg_ipt_log_insert`;"].concat()
}

pub fn create_trg_ipt_log_insert(kphis: &str, kphis_log: &str) -> String {
    [
        "CREATE TRIGGER `",kphis_log,"`.`trg_ipt_log_insert` \
        AFTER INSERT ON `",kphis_log,"`.`ipt_log` \
        FOR EACH ROW \
        BEGIN \
            DECLARE old_vn VARCHAR(13) DEFAULT NULL;\
            DECLARE old_an VARCHAR(13) DEFAULT NULL;\
            IF NEW.an != '' AND NEW.vn IS NOT NULL AND NEW.vn != '' THEN \
                SELECT vn,an INTO old_vn,old_an FROM `",kphis,"`.`ipd_pre_admit_master` WHERE vn=NEW.vn;\
                IF NEW.ipt_log_type = 'I' THEN \
                    IF old_vn IS NOT NULL THEN \
                        UPDATE `",kphis,"`.`ipd_pre_admit_master` SET an=NEW.an,prev_an=old_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE vn=NEW.vn;\
                        CALL `",kphis,"`.`proc_any_an_exists`(old_vn, @an_exists);\
                        IF @an_exists THEN \
                            CALL `",kphis,"`.`proc_update_all_an`(old_vn, NEW.an);\
                        END IF;\
                    END IF;\
                ELSEIF NEW.ipt_log_type = 'D' THEN \
                    CALL `",kphis,"`.`proc_any_an_exists`(NEW.an, @an_exists);\
                    IF old_vn IS NOT NULL THEN \
                        UPDATE `",kphis,"`.`ipd_pre_admit_master` SET an=NULL,prev_an=NEW.an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE vn=NEW.vn;\
                    ELSEIF @an_exists THEN \
                        INSERT INTO `",kphis,"`.`ipd_pre_admit_master` (vn,prev_an",TABLE_CREATE_COLUMNS,") VALUES (NEW.vn,NEW.an,'system',NOW(),'system',NOW(),1);\
                    END IF;\
                    IF @an_exists THEN \
                        CALL `",kphis,"`.`proc_update_all_an`(NEW.an, NEW.vn);\
                    END IF;\
                END IF;\
            END IF;\
        END;",
    ].concat()
}

pub fn drop_proc_update_all_an(kphis: &str) -> String {
    ["DROP PROCEDURE IF EXISTS `", kphis, "`.`proc_update_all_an`;"].concat()
}

pub fn create_proc_update_all_an(kphis: &str, kphis_extra: &str) -> String {
    let kphis_updates = KPHIS_TABLES_WITH_AN
        .iter()
        .flat_map(|table| {
            ["UPDATE `",kphis,"`.`",table,"` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;"]
        })
        .collect::<Vec<&str>>()
        .concat();
    let kphis_extra_updates = KPHIS_EXTRA_TABLES_WITH_AN
        .iter()
        .flat_map(|table| {
            ["UPDATE `",kphis_extra,"`.`",table,"` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;"]
        })
        .collect::<Vec<&str>>()
        .concat();
    ["CREATE PROCEDURE `", kphis, "`.`proc_update_all_an`(IN old_an VARCHAR(13), IN new_an VARCHAR(13)) BEGIN ", &kphis_updates, &kphis_extra_updates, " END;"].concat()
}

pub fn drop_proc_any_an_exists(kphis: &str) -> String {
    ["DROP PROCEDURE IF EXISTS `", kphis, "`.`proc_any_an_exists`;"].concat()
}

pub fn create_proc_any_an_exists(kphis: &str, kphis_extra: &str) -> String {
    let kphis_exists = KPHIS_TABLES_WITH_AN
        .iter()
        .map(|table| ["(SELECT EXISTS(SELECT * FROM `", kphis, "`.`", table, "` WHERE an=old_an))"].concat())
        .collect::<Vec<String>>()
        .join(" OR ");
    let kphis_extra_exists = KPHIS_EXTRA_TABLES_WITH_AN
        .iter()
        .map(|table| ["(SELECT EXISTS(SELECT * FROM `", kphis_extra, "`.`", table, "` WHERE an=old_an))"].concat())
        .collect::<Vec<String>>()
        .join(" OR ");
    ["CREATE PROCEDURE `",kphis,"`.`proc_any_an_exists`(IN old_an VARCHAR(13), OUT an_exists BOOLEAN) BEGIN SELECT ",&kphis_exists," OR ",&kphis_extra_exists," INTO an_exists; END;"].concat()
}

/// old_an, new_an
pub fn call_proc_update_all_an(kphis: &str) -> String {
    ["CALL ", kphis, ".proc_update_all_an(?,?);"].concat()
}

// ===== ===== ===== //
// manual as trigger //
// ===== ===== ===== //

// SELECT * FROM
// 	(SELECT pm1.vn AS pm_vn, pm1.an AS pm_an, ipt1.an AS pm_ipt_an
// 		FROM kphis.ipd_pre_admit_master pm1
// 			LEFT JOIN hos.ipt ipt1 ON ipt1.vn = pm1.vn
// 		WHERE pm1.an = '660001537'
// 	UNION 
// 		SELECT NULL AS pm_vn, NULL AS pm_an, NULL AS pm_ipt_an
// 	) AS a
// CROSS JOIN
// 	(SELECT ipt2.vn AS ipt_vn, pm2.vn AS ipt_pm_vn, pm2.an AS ipt_pm_an, ipt2.an AS ipt_an
// 		FROM hos.ipt ipt2
// 			LEFT JOIN kphis.ipd_pre_admit_master pm2 ON pm2.vn = ipt2.vn
// 		WHERE ipt2.an = '660001537'
// 	UNION
// 		SELECT NULL AS ipt_pm_vn, NULL AS ipt_pm_vn, NULL AS ipt_pm_an, NULL AS ipt_an
// 	) AS b
// LIMIT 1;
/// an, an
pub fn an_in_pre_admit_and_ipt(hosxp: &str, kphis: &str) -> String {
    ["SELECT * FROM \
        (SELECT pm1.vn AS pm_vn,pm1.an AS pm_an,ipt1.an AS pm_ipt_an \
            FROM ",kphis,".ipd_pre_admit_master pm1 \
                LEFT JOIN ",hosxp,".ipt ipt1 ON ipt1.vn=pm1.vn \
            WHERE pm1.an=? \
        UNION \
            SELECT NULL AS pm_vn,NULL AS pm_an,NULL AS pm_ipt_an \
        ) AS a \
    CROSS JOIN \
        (SELECT ipt2.vn AS ipt_vn,pm2.vn AS ipt_pm_vn,pm2.an AS ipt_pm_an,ipt2.an AS ipt_an \
            FROM ",hosxp,".ipt ipt2 \
                LEFT JOIN ",kphis,".ipd_pre_admit_master pm2 ON pm2.vn=ipt2.vn \
            WHERE ipt2.an=? \
        UNION \
            SELECT NULL AS ipt_pm_vn,NULL AS ipt_pm_vn,NULL AS ipt_pm_an,NULL AS ipt_an \
        ) AS b \
    LIMIT 1;"].concat()
}

// SELECT * FROM
// 	(SELECT pm1.vn AS pm_vn, pm1.an AS pm_an, ipt1.vn AS pm_ipt_vn
// 		FROM kphis.ipd_pre_admit_master pm1
// 			LEFT JOIN hos.ipt ipt1 ON ipt1.an = pm1.an
// 		WHERE pm1.vn = '660726155858'
// 	UNION 
// 		SELECT NULL AS pm_vn, NULL AS pm_an, NULL AS pm_ipt_vn
// 	) AS a
// CROSS JOIN
// 	(SELECT ipt2.vn AS ipt_vn, pm2.vn AS ipt_pm_vn, ipt2.an AS ipt_an
// 		FROM hos.ipt ipt2
// 			LEFT JOIN kphis.ipd_pre_admit_master pm2 ON pm2.an = ipt2.an
// 		WHERE ipt2.vn = '660726155858'
// 	UNION
// 		SELECT NULL AS ipt_pm_vn, NULL AS ipt_pm_vn, NULL AS ipt_an
// 	) AS b
// LIMIT 1;
/// vn, vn
pub fn vn_in_pre_admit_and_ipt(hosxp: &str, kphis: &str) -> String {
    ["SELECT * FROM \
        (SELECT pm1.vn AS pm_vn,pm1.an AS pm_an,ipt1.vn AS pm_ipt_vn \
            FROM ",kphis,".ipd_pre_admit_master pm1 \
                LEFT JOIN ",hosxp,".ipt ipt1 ON ipt1.an=pm1.an \
            WHERE pm1.vn=? \
        UNION \
            SELECT NULL AS pm_vn,NULL AS pm_an,NULL AS pm_ipt_vn \
        ) AS a \
    CROSS JOIN \
        (SELECT ipt2.vn AS ipt_vn,pm2.vn AS ipt_pm_vn,ipt2.an AS ipt_an \
            FROM ",hosxp,".ipt ipt2 \
                LEFT JOIN ",kphis,".ipd_pre_admit_master pm2 ON pm2.an=ipt2.an \
            WHERE ipt2.vn=? \
        UNION \
            SELECT NULL AS ipt_pm_vn,NULL AS ipt_pm_vn,NULL AS ipt_an \
        ) AS b \
    LIMIT 1;"].concat()
}

pub fn any_an_exists(an: &str, kphis: &str, kphis_extra: &str) -> String {
    let kphis_exists = KPHIS_TABLES_WITH_AN
        .iter()
        .map(|table| ["(SELECT EXISTS(SELECT * FROM `", kphis, "`.`", table, "` WHERE an='",an,"'))"].concat())
        .collect::<Vec<String>>()
        .join(" OR ");
    let kphis_extra_exists = KPHIS_EXTRA_TABLES_WITH_AN
        .iter()
        .map(|table| ["(SELECT EXISTS(SELECT * FROM `", kphis_extra, "`.`", table, "` WHERE an='",an,"'))"].concat())
        .collect::<Vec<String>>()
        .join(" OR ");
    ["SELECT ",&kphis_exists," OR ",&kphis_extra_exists,";"].concat()
}

pub fn update_many_all_an(old_an: &str, new_an: &str, user: &str, kphis: &str, kphis_extra: &str) -> String {
    let mut kphis_updates = KPHIS_TABLES_WITH_AN
        .iter()
        .flat_map(|table| {
            ["UPDATE `",kphis,"`.`",table,"` SET an='",new_an,"',update_user='",user,"',update_datetime=NOW(),version=(version+1) WHERE an='",old_an,"';"]
        })
        .collect::<Vec<&str>>();
    let kphis_extra_updates = KPHIS_EXTRA_TABLES_WITH_AN
        .iter()
        .flat_map(|table| {
            ["UPDATE `",kphis_extra,"`.`",table,"` SET an='",new_an,"',update_user='",user,"',update_datetime=NOW(),version=(version+1) WHERE an='",old_an,"';"]
        });
    kphis_updates.extend(kphis_extra_updates);
    kphis_updates.concat()
}

pub fn delete_many_all_an(an: &str, kphis: &str, kphis_extra: &str) -> String {
    let mut kphis_deletes = KPHIS_TABLES_WITH_AN
        .iter()
        .flat_map(|table| {
            ["DELETE FROM `",kphis,"`.`",table,"` WHERE an='",an,"';"]
        })
        .collect::<Vec<&str>>();
    let kphis_extra_deletes = KPHIS_EXTRA_TABLES_WITH_AN
        .iter()
        .flat_map(|table| {
            ["DELETE FROM `",kphis_extra,"`.`",table,"` WHERE an='",an,"';"]
        });
    kphis_deletes.extend(kphis_extra_deletes);
    kphis_deletes.concat()
}

// ===== ===== //
//   ipt-log   //
// ===== ===== //

// SELECT * FROM kphis_log.ipt_log;
pub fn select_ipt_log(kphis_log: &str) -> String {
    ["SELECT * FROM ",kphis_log,".ipt_log;"].concat()
}

// ===== ===== //
//   check an  //
// ===== ===== //

/// vn
pub fn exists_pre_admit_not_admit(kphis: &str) -> String {
    ["SELECT EXISTS(SELECT * FROM ",kphis,".ipd_pre_admit_master WHERE vn=? AND an IS NULL) AS ok;"].concat()
}

/// an
pub fn exists_ipt_was_admited(hosxp: &str) -> String {
    ["SELECT EXISTS(SELECT * FROM ",hosxp,".ipt WHERE an=?) AS ok;"].concat()
}

// /// new_an, old_an
// pub fn update_an_into(table: &str, kphis: &str) -> String {
//     ["UPDATE ",kphis,".",table," SET an=? WHERE an=?;"].concat()
// }

// ===== ===== ===== ===== //
//   opd_er_order_master   //
// ===== ===== ===== ===== //

// // SELECT opd_er_order_master_id,an,admit_flag FROM kphis.opd_er_order_master WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y');;
// /// vn
// pub fn select_an_flag_in_opd_er_order_master(kphis: &str) -> String {
//     ["SELECT opd_er_order_master_id,an,admit_flag FROM ",kphis,".opd_er_order_master WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y');"].concat()
// }

// // UPDATE kphis.opd_er_order_master SET an=?,admit_flag=? WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y');;
// /// an, admit_flag, vn
// pub fn update_an_flag_in_opd_er_order_master(kphis: &str) -> String {
//     ["UPDATE ",kphis,".opd_er_order_master SET an=?,admit_flag=? WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y');"].concat()
// }

// // SELECT vn FROM kphis.opd_er_order_master WHERE an=? AND admit_flag='Y' AND (delete_flag IS NULL OR delete_flag <> 'Y');
// /// an
// pub fn select_admited_opd_er_order_master_with_an(kphis: &str) -> String {
//     ["SELECT vn FROM ",kphis,".opd_er_order_master WHERE an=? AND admit_flag='Y' AND (delete_flag IS NULL OR delete_flag <> 'Y');"].concat()
// }

// // INSERT INTO opd_er_order_master (vn,an,er_patient_status_id,admit_flag) VALUES (?,?,10,'Y');
// /// vn, an
// pub fn insert_admited_opd_er_order_master(kphis: &str) -> String {
//     ["INSERT INTO ",kphis,".opd_er_order_master (vn,an,er_patient_status_id,admit_flag) VALUES (?,?,10,'Y');"].concat()
// }

// // UPDATE opd_er_order_master SET an=?,er_patient_status_id=10,admit_flag='Y' WHERE opd_er_order_master_id=?;
// /// an, opd_er_order_master_id
// pub fn update_admited_opd_er_order_master(kphis: &str) -> String {
//     ["UPDATE ",kphis,".opd_er_order_master SET an=?,er_patient_status_id=10,admit_flag='Y' WHERE opd_er_order_master_id=?;"].concat()
// }

// ===== ===== //
//    vn_an    //
// ===== ===== //

// // SELECT vn_an.vn,vn_an.an,iv.vn AS ipt_vn,iv.hn AS ipt_hn,iv.ward AS ipt_ward,ovst.an AS ovst_an,io.hn AS ovst_hn,io.ward AS ovst_ward
// // FROM kphis_extra.vn_an
// // 	LEFT JOIN hos.ipt iv ON iv.an=vn_an.an
// //  LEFT JOIN hos.ovst ON ovst.vn=vn_an.vn
// //  LEFT JOIN hos.ipt io ON io.an=ovst.an
// // WHERE iv.an IS NULL OR ovst.vn IS NULL OR vn_an.vn != iv.vn OR vn_an.an != ovst.an;
// pub fn find_vn_an_mismatch(hosxp: &str, kphis_extra: &str) -> String {
//     ["SELECT vn_an.vn,vn_an.an,iv.vn AS ipt_vn,iv.hn AS ipt_hn,iv.ward AS ipt_ward,ovst.an AS ovst_an,io.hn AS ovst_hn,io.ward AS ovst_ward \
//         FROM ",kphis_extra,".vn_an \
// 	        LEFT JOIN ",hosxp,".ipt iv ON iv.an=vn_an.an \
//             LEFT JOIN ",hosxp,".ovst ON ovst.vn=vn_an.vn \
//             LEFT JOIN ",hosxp,".ipt io ON io.an=ovst.an \
//         WHERE iv.an IS NULL OR ovst.vn IS NULL OR vn_an.vn != iv.vn OR vn_an.an != ovst.an;"
//     ].concat()
// }

// // DELETE FROM kphis_extra.vn_an WHERE vn IN ('');
// pub fn delete_vn_an(vns: &[String], kphis_extra: &str) -> String {
//     ["DELETE FROM ", kphis_extra, ".vn_an WHERE vn IN ('", &vns.join("','"), "');"].concat()
// }

// // INSERT IGNORE INTO kphis_extra.vn_an SET `vn`=?,`an`=?;
// /// vn, an
// pub fn insert_ignore_vn_an(kphis_extra: &str) -> String {
//     ["INSERT IGNORE INTO ", kphis_extra, ".vn_an SET `vn`=?,`an`=?;"].concat()
// }

// ===== ===== //
// an_redirect //
// ===== ===== //

// /// an, redirect
// pub fn replace_redirect(kphis_extra: &str) -> String {
//     ["REPLACE INTO ",kphis_extra,".an_redirect (an, redirect) VALUES (?,?);"].concat()
// }

// /// an
// pub fn delete_redirect_an(kphis_extra: &str) -> String {
//     ["DELETE FROM ",kphis_extra,".an_redirect WHERE an=?;"].concat()
// }