use sqlx::{AssertSqlSafe, MySql, Pool, Row};

use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

pub async fn get_exists(keyword: &str, id: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<bool, AppError> {
    // (query string, count column name, where type)
    if let Some((sql, key, value_type)) = match keyword {
        "pre-order" => Some((kphis_sql::pre_order::master::select_pre_order_exists(kphis), "exs", "string")),
        "ipd-focus-list-used" => Some((kphis_sql::ipd::focus_list::select_note_exists(kphis), "exs", "u32")),
        "opd-er-focus-list-used" => Some((kphis_sql::opd_er::focus_list::select_note_exists(kphis), "exs", "u32")),
        "ipd-med-reconcile" => Some((kphis_sql::ipd::med_reconcile::get_med_reconcile_exists(kphis), "exs", "string")),
        "opd-er-med-reconcile" => Some((kphis_sql::opd_er::med_reconcile::get_med_reconcile_exists(kphis), "exs", "u32")),
        "ipd-med-reconcile-dr-unconfirm" => Some((kphis_sql::ipd::med_reconcile::get_med_reconcile_doctor_unconfirm_exists(kphis), "exs", "string")),
        "opd-er-med-reconcile-dr-unconfirm" => Some((kphis_sql::opd_er::med_reconcile::get_med_reconcile_doctor_unconfirm_exists(kphis), "exs", "u32")),
        "lab-unread" => Some((kphis_sql::lab::get_lab_unread_exists(hosxp, kphis), "exs", "string")),
        "lab-unreport" => Some((kphis_sql::lab::get_lab_unreport_exists(hosxp), "exs", "string")),
        "ipd-xray-unread" => Some((kphis_sql::xray::get_xray_unread_an_exists(hosxp, kphis), "exs", "string")),
        "opd-er-xray-unread" => Some((kphis_sql::xray::get_xray_unread_vn_exists(hosxp, kphis), "exs", "string")),
        _ => None,
    } {
        let mut query = sqlx::query(AssertSqlSafe(sql));
        match value_type {
            // "i32" => {
            //     if let Some(id_i32) = id.parse::<i32>().ok().and_then(zero_none) {
            //         query = query.bind(id_i32);
            //     } else {
            //         return Err(AppError::new_server(500, "Invalid id","GetExists").with_400());
            //     }
            // }
            "u32" => {
                if let Some(id_u32) = id.parse::<u32>().ok().and_then(zero_none) {
                    query = query.bind(id_u32);
                } else {
                    return Err(AppError::new_server(400, "Invalid id", "GetExists"));
                }
            }
            _ => {
                query = query.bind(id);
            }
        }
        query
            .fetch_one(pool)
            .await
            .map(|row| row.try_get(key))
            .map_err(|e| Source::SQLx.to_error(500, e, "GetExists"))?
            .map_err(|e| Source::SQLx.to_error(500, e, "GetExists"))
    } else {
        Err(AppError::app_404("GetExists"))
    }
}

pub async fn bump_and_get_serial(serial_name: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<i32>, AppError> {
    let bump_sql = kphis_sql::bump_serial(serial_name, hosxp);
    let get_sql = kphis_sql::get_serial(serial_name, hosxp);
    let mut tx = pool.begin().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump Serial"))?;
    if let Err(e) = sqlx::query(AssertSqlSafe(bump_sql)).execute(&mut *tx).await {
        tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump Serial"))?;
        return Err(Source::SQLx.to_error(500, e, "Bump Serial"));
    }
    match sqlx::query(AssertSqlSafe(get_sql)).fetch_optional(&mut *tx).await {
        Ok(opt) => {
            tx.commit().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump Serial"))?;
            Ok(opt.and_then(|row| row.try_get::<i32, usize>(0).ok()))
        }
        Err(e) => {
            tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump Serial"))?;
            Err(Source::SQLx.to_error(500, e, "Bump Serial"))
        }
    }
}

pub async fn bump_and_get_sp_use(pool: &Pool<MySql>, hosxp: &str) -> Result<Option<String>, AppError> {
    let mut tx = pool.begin().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump Serial"))?;
    let old_prefix = match sqlx::query(AssertSqlSafe(kphis_sql::get_sp_use_prefix(hosxp))).fetch_optional(&mut *tx).await {
        Ok(row_opt) => match row_opt.map(|row| row.try_get::<String, usize>(0)).transpose() {
            Ok(opt) => opt.and_then(|op| op.chars().last()).unwrap_or('0'),
            Err(e) => {
                tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
                return Err(Source::SQLx.to_error(500, e, "Select Prefix"));
            }
        },
        Err(e) => {
            tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
            return Err(Source::SQLx.to_error(500, e, "Select Prefix"));
        }
    };
    let old_serial_opt = match sqlx::query(AssertSqlSafe(kphis_sql::get_serial("sp_use", hosxp))).fetch_optional(&mut *tx).await {
        Ok(row_opt) => match row_opt.map(|row| row.try_get::<i32, usize>(0)).transpose() {
            Ok(os) => os,
            Err(e) => {
                tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
                return Err(Source::SQLx.to_error(500, e, "Select Serial"));
            }
        },
        Err(e) => {
            tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
            return Err(Source::SQLx.to_error(500, e, "Select Serial"));
        }
    };
    if let Some(old_serial) = old_serial_opt {
        if old_serial == 999999 {
            // will overflow at 999999, extend prefix to 63 chars == 63M
            let prefixs = "0123456789abcedfghijklmnopqrstuvwxyzABCEDFGHIJKLMNOPQRSTUVWXYZ";
            let pos = prefixs.chars().position(|c| c == old_prefix).unwrap_or_default();
            // if pos == prefixs.len() {
            //     pos = 0;
            // }
            let new_prefix = prefixs.chars().skip(pos + 1).next().unwrap_or('ก');
            if let Err(e) = sqlx::query(AssertSqlSafe(kphis_sql::update_sp_use_prefix(hosxp))).bind(new_prefix.to_string()).execute(&mut *tx).await {
                tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
                return Err(Source::SQLx.to_error(500, e, "Update Prefix"));
            }
            if let Err(e) = sqlx::query(AssertSqlSafe(kphis_sql::update_serial("sp_use", hosxp))).bind(0).execute(&mut *tx).await {
                tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
                return Err(Source::SQLx.to_error(500, e, "Update Serial"));
            }
            tx.commit().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump Serial"))?;
            Ok(Some(format!("{}{:0>6}", new_prefix, 0)))
        } else {
            // bumpable
            if let Err(e) = sqlx::query(AssertSqlSafe(kphis_sql::bump_serial("sp_use", hosxp))).execute(&mut *tx).await {
                tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
                return Err(Source::SQLx.to_error(500, e, "Bump Serial"));
            }
            tx.commit().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump Serial"))?;
            Ok(Some(format!("{}{:0>6}", old_prefix, old_serial + 1)))
        }
    } else {
        tx.rollback().await.map_err(|e| Source::SQLx.to_error(500, e, "Bump SpUse"))?;
        Err(Source::SQLx.to_error(500, "Serial Not Found", "Select Serial"))
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_pre_order_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_exists("pre-order", "0001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("pre-order", "0006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_note_exists_ipd() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_exists("ipd-focus-list-used", "1", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("ipd-focus-list-used", "9", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_note_exists_opd_er() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_exists("opd-er-focus-list-used","1",&tester.db_pool,&tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("opd-er-focus-list-used","9",&tester.db_pool,&tester.hosxp, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_med_reconcile_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        let found = get_exists("ipd-med-reconcile", "660001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("ipd-med-reconcile", "660006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_med_reconcile_doctor_unconfirm_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        let found = get_exists("ipd-med-reconcile-dr-unconfirm","660001234",&tester.db_pool,&tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("ipd-med-reconcile-dr-unconfirm","660006666",&tester.db_pool,&tester.hosxp, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_er_med_reconcile_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        let found = get_exists("opd-er-med-reconcile", "1", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("opd-er-med-reconcile", "999", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_er_med_reconcile_doctor_unconfirm_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        let found = get_exists("opd-er-med-reconcile-dr-unconfirm","1",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("opd-er-med-reconcile-dr-unconfirm","999",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lab_unread_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();
        let found = get_exists("lab-unread","650001234",&tester.db_pool,&tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("lab-unread","651231235959",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lab_unreport_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        let found = get_exists("lab-unreport","660001234",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(found);
        let not_found = get_exists("lab-unreport","650001234",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_xray_unread_exists() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();
        let found_an = get_exists("ipd-xray-unread","650001234",&tester.db_pool,&tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found_an);
        let found_vn = get_exists("opd-er-xray-unread","661231235959",&tester.db_pool,&tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found_vn);
        let not_found = get_exists("ipd-xray-unread","660006666",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_exists_unknown() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        let will_404 = get_exists("unknown", "1", &tester.db_pool, &tester.hosxp, &tester.kphis).await;
        assert!(will_404.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_bump_and_get_serial() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();

        let result = bump_and_get_serial("med_plan_number", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(result, Some(889));
        let again = bump_and_get_serial("med_plan_number", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(again, Some(890));
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_bump_and_get_sp_use() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/sys_var.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/serial.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/sys_var.sql")).execute(&tester.db_pool).await.unwrap();
        
        let result = bump_and_get_sp_use(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(result, Some(String::from("9999999")));
        let again = bump_and_get_sp_use(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(again, Some(String::from("a000000")));
    }
}
