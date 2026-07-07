DROP PROCEDURE IF EXISTS `kphis`.`proc_update_all_an`;
CREATE PROCEDURE `kphis`.`proc_update_all_an`(IN old_an VARCHAR(13), IN new_an VARCHAR(13))
    BEGIN
        UPDATE `kphis`.`ipd_doctor_in_charge` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_dr_admission_note` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_dr_admission_note_item` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_dr_consult` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_dr_consult_signature_reply` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_dr_consult_signature_request` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_focus_list` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_focus_note` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_io` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_med_reconciliation` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_med_reconciliation_item` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_nurse_admission_note` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_nurse_index_action` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_nurse_index_note` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_nurse_index_plan` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_order` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_order_item` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_progress_note` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_progress_note_item` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_summary_2` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis`.`ipd_vs_vital_sign` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis_extra`.`ipd_document` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis_extra`.`ipd_mra` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
        UPDATE `kphis_extra`.`ipd_nurse_index_monitor` SET an=new_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE an=old_an;
    END;
DROP PROCEDURE IF EXISTS `kphis`.`proc_any_an_exists`;
CREATE PROCEDURE `kphis`.`proc_any_an_exists`(IN old_an VARCHAR(13), OUT an_exists BOOLEAN)
    BEGIN
        SELECT
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_doctor_in_charge` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_dr_admission_note` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_dr_admission_note_item` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_dr_consult` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_dr_consult_signature_reply` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_dr_consult_signature_request` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_focus_list` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_focus_note` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_io` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_med_reconciliation` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_med_reconciliation_item` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_nurse_admission_note` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_nurse_index_action` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_nurse_index_note` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_nurse_index_plan` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_order` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_order_item` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_progress_note` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_progress_note_item` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_summary_2` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis`.`ipd_vs_vital_sign` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis_extra`.`ipd_document` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis_extra`.`ipd_mra` WHERE an=old_an)) OR
            (SELECT EXISTS(SELECT * FROM `kphis_extra`.`ipd_nurse_index_monitor` WHERE an=old_an))
        INTO an_exists;
    END;
DROP TRIGGER IF EXISTS `kphis_log`.`trg_ipt_log_insert`;
CREATE TRIGGER `kphis_log`.`trg_ipt_log_insert`
    AFTER INSERT ON `",kphis_log,"`.`ipt_log`
    FOR EACH ROW
    BEGIN
        DECLARE old_vn VARCHAR(13) DEFAULT NULL;
        DECLARE old_an VARCHAR(13) DEFAULT NULL;
        IF NEW.an != '' AND NEW.vn IS NOT NULL AND NEW.vn != '' THEN
            SELECT vn,an INTO old_vn,old_an FROM `kphis`.`ipd_pre_admit_master` WHERE vn=NEW.vn;
            IF NEW.ipt_log_type = 'I' THEN
                IF old_vn IS NOT NULL THEN
                    UPDATE `kphis`.`ipd_pre_admit_master` SET an=NEW.an,prev_an=old_an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE vn=NEW.vn;
                    CALL `kphis`.`proc_any_an_exists`(old_vn, @an_exists);
                    IF @an_exists THEN
                        CALL `kphis`.`proc_update_all_an`(old_vn, NEW.an);
                    END IF;
                END IF;
            ELSEIF NEW.ipt_log_type = 'D' THEN
                CALL `kphis`.`proc_any_an_exists`(NEW.an, @an_exists);
                IF old_vn IS NOT NULL THEN
                    UPDATE `kphis`.`ipd_pre_admit_master` SET an=NULL,prev_an=NEW.an,update_user='system',update_datetime=NOW(),version=(version+1) WHERE vn=NEW.vn;
                ELSEIF @an_exists THEN
                    INSERT INTO `kphis`.`ipd_pre_admit_master` (vn,prev_an,create_user,create_datetime,update_user,update_datetime,version) VALUES (NEW.vn,NEW.an,'system',NOW(),'system',NOW(),1);
                END IF;
                IF @an_exists THEN
                    CALL `kphis`.`proc_update_all_an`(NEW.an, NEW.vn);
                END IF;
            END IF;
        END IF;
    END;
DROP TRIGGER IF EXISTS `hos`.`trg_kphis_ipt_log_insert`;
CREATE TRIGGER `hos`.`trg_kphis_ipt_log_insert`
    AFTER INSERT ON `hos`.`ipt`
    FOR EACH ROW
        INSERT INTO `kphis_log`.`ipt_log` (ipt_log_type,an,vn,hn,ward,create_datetime) VALUES ('I',NEW.an,NEW.vn,NEW.hn,NEW.ward,NOW());
DROP TRIGGER IF EXISTS `hos`.`trg_kphis_ipt_log_delete`;
CREATE TRIGGER `hos`.`trg_kphis_ipt_log_delete`
    AFTER DELETE ON `hos`.`ipt`
    FOR EACH ROW
        INSERT INTO `kphis_log`.`ipt_log` (ipt_log_type,an,vn,hn,ward,create_datetime) VALUES ('D',OLD.an,OLD.vn,OLD.hn,OLD.ward,NOW());