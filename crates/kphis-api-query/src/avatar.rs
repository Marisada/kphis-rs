use sqlx::{AssertSqlSafe, MySql, Pool, Row, mysql::MySqlRow};
use std::cmp::Ordering;

use kphis_model::avatar::{AvatarOpdEr, AvatarParams, AvatarWard};
use kphis_sql::avatar;
use kphis_util::error::{AppError, Source};

use crate::query_all;

pub async fn get_avatar_opd_er(pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<AvatarOpdEr>, AppError> {
    let sql = avatar::select_avatar_opd_er(hosxp, kphis);
    let rows = query_all(&sql, pool, "Select AvatarOpdEr").await?;

    rows.iter().map(avatar_opd_er_row).collect()
}
fn avatar_opd_er_row(row: &MySqlRow) -> Result<AvatarOpdEr, AppError> {
    Ok(AvatarOpdEr {
        opd_er_order_master_id: row.try_get("opd_er_order_master_id").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarOpdEr"))?,
        vn: row.try_get("vn").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarOpdEr"))?,
        hn: row.try_get("hn").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarOpdEr"))?,
        display_bedno: row.try_get("display_bedno").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarOpdEr"))?,
        bed_type_name: row.try_get("bed_type_name").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarOpdEr"))?,
        bed_type_color: row.try_get("bed_type_color").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarOpdEr"))?,
        pname: row.try_get("pname").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarOpdEr"))?,
    })
}

pub async fn get_avatar_ipd(params: &AvatarParams, hlen: usize, alen: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<AvatarWard>, AppError> {
    let sql = avatar::select_avatar_in_ward(params, hlen, alen, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(ward) = &params.ward {
        query = query.bind(ward);
    }
    if let Some(patient) = params.search.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        let wildcard = ["%", patient.trim(), "%"].concat();
        match patient.parse::<u64>().is_ok() {
            true => {
                if patient.len() == 13 {
                    query = query.bind(patient);
                } else {
                    match hlen.cmp(&alen) {
                        Ordering::Equal => {
                            query = query.bind(wildcard.clone()).bind(wildcard.clone());
                        }
                        Ordering::Greater | Ordering::Less => {
                            query = query.bind(wildcard.clone());
                        }
                    }
                }
            }
            false => {
                query = query.bind(wildcard);
            }
        }
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarIpd"))?
        .iter()
        .map(avatar_ipd_row)
        .collect()
}
fn avatar_ipd_row(row: &MySqlRow) -> Result<AvatarWard, AppError> {
    Ok(AvatarWard {
        an: row.try_get("an").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarIpd"))?,
        hn: row.try_get("hn").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarIpd"))?,
        bedno: row.try_get("bedno").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarIpd"))?,
        pname: row.try_get("pname").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarIpd"))?,
        discharge_order_exists: row.try_get("discharge_order_exists").map_err(|e| Source::SQLx.to_error(500, e, "Select AvatarIpd"))?,
    })
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_avatar_opd_er() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_alt/opd_er_order_master_active.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();

        // WHERE om.delete_flag IS NULL OR om.delete_flag <> 'Y' AND om.er_patient_status_id <> 7 ORDER BY bt.display_order,b.display_order;"
        // om.delete_flag = NULL, om.er_patient_status_id == 1
        let found = get_avatar_opd_er(&tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found.len() == 1);

        // om.delete_flag = NULL, om.er_patient_status_id = 7
        sqlx::query("DROP TABLE `kphis`.`opd_er_order_master`;").execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_alt/opd_er_order_master_discharged.sql")).execute(&tester.db_pool).await.unwrap();
        let discharged = get_avatar_opd_er(&tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(discharged.is_empty());

        // om.delete_flag = 'Y', om.er_patient_status_id = 1
        sqlx::query("DROP TABLE `kphis`.`opd_er_order_master`;").execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_alt/opd_er_order_master_deleted.sql")).execute(&tester.db_pool).await.unwrap();
        let deleted = get_avatar_opd_er(&tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(deleted.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_avatar_ipd() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_avatar_ipd(&AvatarParams::default(), 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(default.is_empty());

        // ipt.ward = '01' AND ipt.dchstts = '02'
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp_alt/ipt_discharged.sql")).execute(&tester.db_pool).await.unwrap();

        let discharged = get_avatar_ipd(&AvatarParams {ward: Some(String::from("01")),..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(discharged.is_empty());

        // WHERE ipt.ward=? AND ipt.dchstts IS NULL
        // ipt.ward='01' AND ipt.dchstts IS NULL
        sqlx::query("DROP TABLE `hos`.`ipt`;").execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp_alt/ipt_active.sql")).execute(&tester.db_pool).await.unwrap();
        let active_ward = get_avatar_ipd(&AvatarParams {ward: Some(String::from("01")),..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(active_ward.len(), 1);
        let active_hn = get_avatar_ipd(&AvatarParams {search: Some(String::from("1234")),..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(active_hn.len(), 1);
        let active_an = get_avatar_ipd(&AvatarParams {search: Some(String::from("60001234")),..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(active_an.len(), 1);
        let active_cid = get_avatar_ipd(&AvatarParams {search: Some(String::from("1111111111111")),..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(active_cid.len(), 1);

        // AND wp.passcode IS NULL
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        let active_passcode = get_avatar_ipd(&AvatarParams {ward: Some(String::from("01")),..Default::default()}, 7, 9,  &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(active_passcode.is_empty());
    }
}
