use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row};

use kphis_model::transform::IptLog;
use kphis_sql::transform;
use kphis_util::error::{AppError, ErrorTitle, Source};

// ===== ===== //
//   ipt-log   //
// ===== ===== //

pub async fn select_ipt_log(pool: &Pool<MySql>, kphis_log: &str) -> Result<Vec<IptLog>, AppError> {
    let sql = transform::select_ipt_log(kphis_log);
    sqlx::query(AssertSqlSafe(sql))
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "SelectIptLog"))?
        .iter()
        .map(IptLog::from_row)
        .collect::<sqlx::Result<Vec<IptLog>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "SelectIptLog"))
}

// ===== ===== //
//   check AN  //
// ===== ===== //

/// return `400` when an is empty<br>
/// return `404` when an cannot execute due to an was revoked or pre-admit was admited
pub async fn check_an_can_execute(an: &str, an_len: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<(), AppError> {
    if an.is_empty() {
        Err(AppError::app_400("ExecuteWithAn"))
    } else {
        check_an_exists(an, an_len, pool, hosxp, kphis).await
    }
}

/// return `400` when an is None or Some("")<br>
/// return `404` when an cannot execute due to an was revoked or pre-admit was admited
pub async fn check_an_opt_can_execute(an_opt: &Option<String>, an_len: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<(), AppError> {
    if let Some(an) = &an_opt
        && !an.is_empty()
    {
        check_an_exists(an, an_len, pool, hosxp, kphis).await
    } else {
        Err(AppError::app_400("ExecuteWithAn"))
    }
}

async fn check_an_exists(an: &str, an_len: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<(), AppError> {
    if an.len() > an_len {
        // kphis.ipd_pre_admit_master has VN as AN with NULL AN == PreAdmit NOT Admited
        // false when PreAdmit IS Admited
        if exists_pre_admit_not_admit(&an, pool, kphis).await? {
            Ok(())
        } else {
            Err(Source::App
                .to_error(403, "ท่านสามารถบันทึกข้อมูลต่อได้ ด้วยการเลือกผู้ป่วยที่เมนู 'รอ Admit' >> 'ประเภท: Admit แล้ว'", "CheckAnExists")
                .with_title(ErrorTitle::PreAdmitAdmited))
        }
    // hos.ipt not has AN == Admited in HOSxP
    // false when Not admit in HOSxP
    } else if exists_ipt_was_admited(&an, pool, hosxp).await? {
        Ok(())
    } else {
        Err(Source::App
            .to_error(403, "ท่านสามารถบันทึกข้อมูลต่อได้ ด้วยการเลือกผู้ป่วยที่เมนู 'รอ Admit' >> 'ประเภท: ยกเลิก Admit'", "CheckAnExists")
            .with_title(ErrorTitle::AdmitRevoked))
    }
}

async fn exists_pre_admit_not_admit(vn: &str, pool: &Pool<MySql>, kphis: &str) -> Result<bool, AppError> {
    let sql = transform::exists_pre_admit_not_admit(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(vn)
        .fetch_one(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "ExistsPreAdmitNotAdmit"))?
        .try_get(0)
        .map_err(|e| Source::SQLx.to_error(500, e, "ExistsPreAdmitNotAdmit"))
}

async fn exists_ipt_was_admited(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<bool, AppError> {
    let sql = transform::exists_ipt_was_admited(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_one(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "ExistsIptWasAdmit"))?
        .try_get(0)
        .map_err(|e| Source::SQLx.to_error(500, e, "ExistsIptWasAdmit"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_ipt_log() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_ipt_log(&tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(found.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_exists_pre_admit_not_admit() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let exists = exists_pre_admit_not_admit("670101111111",&tester.db_pool, &tester.kphis).await.unwrap();
        assert!(exists);
        let not_exists = exists_pre_admit_not_admit("661231235959",&tester.db_pool, &tester.kphis).await.unwrap();
        assert!(!not_exists);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_exists_ipt_was_admited() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        let exists = exists_ipt_was_admited("660001234",&tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(exists);
        let not_exists = exists_ipt_was_admited("660006666",&tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(!not_exists);
    }
}
