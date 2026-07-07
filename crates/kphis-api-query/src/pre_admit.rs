use futures_util::StreamExt;
use sqlx::{AssertSqlSafe, Executor, FromRow, MySql, Pool, Row};

use kphis_model::{
    fetch::ExecuteResponse,
    pre_admit::{PreAdmitList, PreAdmitOnly, PreAdmitParams, PreAdmitPatch, PreAdmitSave},
    transform::{AnInPreAdmitAndIpt, VnInPreAdmitAndIpt},
};
use kphis_sql::{pre_admit, transform};
use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

pub async fn get_pre_admit_list(params: &PreAdmitParams, hlen: usize, alen: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<PreAdmitList>, AppError> {
    let sql = pre_admit::select_pre_admit(params, hlen, alen, hosxp, kphis);

    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(doctor_in_charge) = &params.doctor_in_charge {
        query = query.bind(doctor_in_charge);
    }
    if let Some(patient) = params.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        let patient_wildcard = ["%", &patient, "%"].concat();
        if patient.len() == 13 {
            query = query.bind(patient);
        } else if hlen == alen {
            query = query.bind(patient_wildcard.clone()).bind(patient_wildcard);
        } else {
            query = query.bind(patient_wildcard)
        }
    }

    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(PreAdmitList::from_row)
                .collect::<sqlx::Result<Vec<PreAdmitList>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select PreAdmitList"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreAdmitList"))?
}

pub async fn insert_pre_admit(save: &PreAdmitSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = pre_admit::insert_pre_admit(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&save.vn)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|response| ExecuteResponse::from_query_result(response, "Insert PreAdmit"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreAdmit"))
}

// ===== ===== ===== //
// Manual as trigger //
// ===== ===== ===== //

pub async fn patch_pre_admit(save: &PreAdmitPatch, user: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    match save {
        // PreAdmitPatch::SetAn(vn, an) => {
        //     update_many_all_an(vn, an, user, pool, kphis, kphis_extra).await
        // }
        PreAdmitPatch::RevokeAn(an) => revoke_an(an, user, pool, kphis, kphis_extra).await,
        PreAdmitPatch::RevokeVnAn(vn, an) => revoke_vnan(vn, an, user, pool, kphis, kphis_extra).await,
        PreAdmitPatch::SyncAn(an) => sync_an(an, user, pool, hosxp, kphis, kphis_extra).await,
        PreAdmitPatch::SyncVn(vn) => sync_vn(vn, user, pool, hosxp, kphis, kphis_extra).await,
    }
}

async fn revoke_an(an: &str, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    if let Some(pre_admit) = select_pre_admit_by_an(an, pool, kphis).await? {
        if any_an_exists(an, pool, kphis, kphis_extra).await? {
            results.push(update_pre_admit(&pre_admit.vn, "", an, user, pool, kphis).await?);
            results.extend(update_many_all_an(an, &pre_admit.vn, user, pool, kphis, kphis_extra).await?);
        }
    }

    Ok(results)
}

// revoke only matched VN and AN
async fn revoke_vnan(vn: &str, an: &str, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    if any_an_exists(an, pool, kphis, kphis_extra).await? {
        if let Some(pre_admit) = select_pre_admit_by_vn(vn, pool, kphis).await? {
            if let Some(pm_an) = pre_admit.an.as_ref()
                && pm_an == an
            {
                results.push(update_pre_admit(vn, "", an, user, pool, kphis).await?);
                results.extend(update_many_all_an(an, vn, user, pool, kphis, kphis_extra).await?);
            }
        } else {
            results.push(insert_revoked_pre_admit(vn, an, user, pool, kphis).await?);
            results.extend(update_many_all_an(an, vn, user, pool, kphis, kphis_extra).await?);
        }
    }

    Ok(results)
}

async fn select_pre_admit_by_an(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<PreAdmitOnly>, AppError> {
    let sql = pre_admit::select_pre_admit_by_an(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreAdmitOnly"))?
        .as_ref()
        .map(PreAdmitOnly::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreAdmitOnly"))
}

pub async fn select_pre_admit_by_vn(vn: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<PreAdmitOnly>, AppError> {
    let sql = pre_admit::select_pre_admit_by_vn(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(vn)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreAdmitOnly"))?
        .as_ref()
        .map(PreAdmitOnly::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreAdmitOnly"))
}

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
//
// search AN `123` is admited/revoked/changed ?
// | #  | pm_vn | pm_an | pm_ipt_an | ipt_vn | ipt_pm_vn | ipt_pm_an | ipt_an | COMMENT 					 	   | ACTION 																|
// |--- | ---   | ---   | ---       | ---    | ---       | ---       | ---    | ---     						   | ---    																|
// |    | `-2-` | `-1-` | `-3-`     | `(2)`  | `(3)`     | `(4)`     | `(1)`  |                                    |     																	|
// |  1 |       |       |           |        |           |           |        | Not found                          | *Check AN exists -> delete AN                                          |
// |  2 | *111* | `123` | 123       | *111*  | 111       | 123       | `123`  | Admit + Pre-admit admited          | None 																	|
// |  3 | *111* | `123` |           |        |           |           |        | VN 111 UnAdmit                     | change (VN111:AN123) to (VN111:AN-)   								    |
// |  4 | *111* | `123` | 456       |        |           |           |        | AN 123 -> 456, No AN 123 in ipt    | change (VN111:AN123) to (VN111:AN456)   								|
// |  5 |       |       |           | *111*  |           |           | `123`  | Admit + No pre-admit               | None 																	|
// |  6 |       |       |           | *111*  | 111       |           | `123`  | Admit + Pre-admit not admit        | change (VN111:AN-) to (VN111:AN123)   								    |
// |  7 |       |       |           | *111*  | 111       | 789       | `123`  | Admit AN 789 -> 123                | change (VN111:AN789) to (VN111:AN123) 								    |
// |  8 | *111* | `123` | 456       | *444*  |           |           | `123`  | AN 123 -> 456, VN 444 no pre-admit | change (VN111:AN123) to (VN111:AN456)   								|
// |  9 | *111* | `123` |           | *444*  | 444       | 789       | `123`  | VN 111 UnAdmit, AN 789 -> 123      | change (VN111:AN123) to (VN111:AN-), (VN444:AN789) to (VN444:AN123)    |
// | 10 | *111* | `123` | 456       | *444*  | 444       |           | `123`  | AN 123 -> 456, Pre-admit not admit | change (VN111:AN123) to (VN111:AN456), (VN444:AN-) to (VN444:AN123)    |
// | 11 | *111* | `123` | 456       | *444*  | 444       | 789       | `123`  | AN 123 -> 456, AN 789 -> 123       | change (VN111:AN123) to (VN111:AN456), (VN444:AN789) to (VN444:AN123)  |
//
// NOTE:
// * ipt_an`(1)` is PK, and HOSxP always provide VN, so ipt_vn`(2)` always present
// * ipt_vn`(2)` and ipt_pm_vn`(3)` is the JOINED colunm, so if `(3)` exists `(3)` MUST EQUAL `(2)`
// * pm_vn`-2-` is PK, so it always present when pm_an`-1-` exists

// FLOW: match `(2)` and `-2-` is null
// 1. `(2)` and `-2-` is null => *Check AN exists* -> delete AN (`#1`)
// 2. `-2-` is not null => A:
// 	- if `-3-` is null => `-2-` change AN to self (`#3`)
// 	- else => `-3-` change AN to `-3-` (`#4`)
// 3. `(2)` is not null =>
// 	- B: `(3)` is not null => `(3)` change AN to `(1)` (`#6`,`#7`, other is `#5`)
// 4. `(2)` and `-2-` is not null =>
// 	- if `-2-` == `(2)` => OK (`#2`)
// 	- else => do A then B (`#8`,`#9`,`#10`,`#11`)
async fn sync_an(an: &str, user: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    let state = an_in_pre_admit_and_ipt(an, pool, hosxp, kphis).await?;
    match (state.pm_vn.as_ref(), state.ipt_vn.as_ref()) {
        // 1. #1
        (None, None) => {
            if any_an_exists(an, pool, kphis, kphis_extra).await? {
                results.extend(delete_many_all_an(an, pool, kphis, kphis_extra).await?);
            }
        }
        // 2. #3, #4
        (Some(pm_vn), None) => {
            results.extend(sync_an_pm_only(an, pm_vn, &state.pm_ipt_an, user, pool, kphis, kphis_extra).await?);
        }
        // 3. #5, #6, #7
        (None, Some(_ipt_vn)) => {
            results.extend(sync_an_ipt_only(an, &state.ipt_pm_vn, &state.ipt_pm_an, user, pool, kphis, kphis_extra).await?);
        }
        // 4. #2, #8, #9, #10, #11
        (Some(pm_vn), Some(ipt_vn)) => {
            if pm_vn != ipt_vn {
                results.extend(sync_an_pm_only(an, pm_vn, &state.pm_ipt_an, user, pool, kphis, kphis_extra).await?);
                results.extend(sync_an_ipt_only(an, &state.ipt_pm_vn, &state.ipt_pm_an, user, pool, kphis, kphis_extra).await?);
            }
            // else #2 Do nothing
        }
    }

    Ok(results)
}

// sync_an route A (#3, #4)
async fn sync_an_pm_only(pm_an: &str, pm_vn: &str, pm_ipt_an: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    if any_an_exists(pm_an, pool, kphis, kphis_extra).await? {
        if let Some(ipt_an) = pm_ipt_an.as_ref() {
            // #4 change AN to `-3-`
            results.push(update_pre_admit(pm_vn, ipt_an, pm_an, user, pool, kphis).await?);
            results.extend(update_many_all_an(pm_an, ipt_an, user, pool, kphis, kphis_extra).await?);
        } else {
            // #3 change AN to self
            results.push(update_pre_admit(pm_vn, "", pm_an, user, pool, kphis).await?);
            results.extend(update_many_all_an(pm_an, pm_vn, user, pool, kphis, kphis_extra).await?);
        }
    }

    Ok(results)
}

// sync_an route B (#5, #6, #7)
async fn sync_an_ipt_only(
    ipt_an: &str,
    ipt_pm_vn: &Option<String>,
    ipt_pm_an: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
    kphis_extra: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    if let Some(pm_vn) = ipt_pm_vn.as_ref() {
        if let Some(pm_an) = ipt_pm_an.as_ref() {
            // #7 pm_an -> ipt_an
            if any_an_exists(pm_an, pool, kphis, kphis_extra).await? {
                results.push(update_pre_admit(pm_vn, ipt_an, pm_an, user, pool, kphis).await?);
                results.extend(update_many_all_an(pm_an, ipt_an, user, pool, kphis, kphis_extra).await?);
            }
        } else {
            // #6 pm_vn -> ipt_an
            if any_an_exists(pm_vn, pool, kphis, kphis_extra).await? {
                results.push(update_pre_admit(pm_vn, ipt_an, "", user, pool, kphis).await?);
                results.extend(update_many_all_an(pm_vn, ipt_an, user, pool, kphis, kphis_extra).await?);
            }
        }
    }
    // else #5 Do nothing

    Ok(results)
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
//
// search VN `111`
// | #  | pm_vn | pm_an | pm_ipt_vn | ipt_vn | ipt_pm_vn | ipt_an | COMMENT 						        			| ACTION 																|
// |--- | ---   | ---   | ---       | ---    | ---       | ---    | ---     						        			| ---  																	|
// |    | `-1-` | `-2-` | `-3-`     | `(1)`  | `(3)`     | `(2)`  |         						        			|     																	|
// |  1 |       |       |           |        |           |        | Not found                               			| None 																	|
// |  2 | `111` |       |           |        |           |        | VN 111 not admit	                     			| None   																|
// |  3 | `111` | *123* | 111       | `111`  | 111       | *123*  | Admit + Pre-admit admited               			| None 																	|
// |  4 |       |       |           | `111`  |           | *123*  | Admit + No pre-admit                    			| None 																	|
// |  5 | `111` |       |           | `111`  |           | *123*  | Admit + Pre-admit not admit             			| change (VN111:AN-) to (VN111:AN123)   								|
// |  6 |       |       |           | `111`  | 444       | *123*  | VN 444 UnAdmit, VN 111 no pre-admit                 | change (VN444:AN123) to (VN444:AN-)                                	|
// |  7 | `111` | *123* |           |        |           |        | AN 123 revoked                         			    | change (VN111:AN123) to (VN111:AN-)   								|
// |  8 | `111` | *123* | 777       |        |           |        | AN 123 revoked, `VN 777`?                			| change (VN111:AN123) to (VN111:AN-)   								|
// |  9 | `111` | *789* | 777       | `111`  |           | *123*  | AN 789 -> 123, `VN 777`?                 			| change (VN111:AN789) to (VN111:AN123)   								|
// | 10 | `111` | *789* |           | `111`  | 444       | *123*  | VN 444 UnAdmit, AN 789 -> 123           			| change (VN444:AN123) to (VN444:AN-), (VN111:AN789) to (VN111:AN123)   |
// | 11 | `111` | *789* | 777       | `111`  | 444       | *123*  | VN 444 UnAdmit, AN 789 -> 123, `VN 777` 			| change (VN444:AN123) to (VN444:AN-), (VN111:AN789) to (VN111:AN123)   |
//
// NOTE:
// * ipt_an`(2)` is PK, so it always present when ipt_vn`(1)` exists
// * ipt_an`(2)` is JOINED with `pm.an`, so if ipt_pm_vn`(3)` exixts mean `pm.an` also exixts
//
// FLOW:
// 1. `-2-` == `(2)` => OK (`#1`,`#2`,`#3`)
// 2. `(2)` is not null =>
// 	- A1: `-1-` is null => `(3)` is not null => `(1)` change AN to self (`#6`, other is `#4`)
//  - A2: `-1-` is not null => `-1-` change AN to (2) (#5)
// 3. `-2-` is not null => do B then D (`#7`,`#8`)
// 	- B: if `-2-` is not null => `-1-` change AN to self
// 	- D: if `-3-` is not null => recursion of `-3-`
// 4. `(2)` and `-2-` is not null =>
//  - if ipt_pm_vn is not null => A (`#10`,`#11`)
//  - C: `-1-` change AN to `(2)` (`#9`,`#10`,`#11`)
async fn sync_vn(vn: &str, user: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    let state = vn_in_pre_admit_and_ipt(vn, pool, hosxp, kphis).await?;
    match (state.pm_an.as_ref(), state.ipt_an.as_ref()) {
        // 1. #1, #2
        (None, None) => {}
        // 2. #4, #5, #6
        (None, Some(ipt_an)) => {
            results.extend(sync_vn_ipt_only(ipt_an, &state.pm_vn, &state.ipt_pm_vn, user, pool, kphis, kphis_extra).await?);
        }
        // 3. #7, #8
        (Some(pm_an), None) => {
            results.extend(sync_vn_pm_only(pm_an, vn, user, pool, kphis, kphis_extra).await?);
        }
        // 4. #3, #9, #10, #11
        (Some(pm_an), Some(ipt_an)) => {
            if pm_an != ipt_an {
                results.extend(sync_vn_ipt_only(ipt_an, &state.pm_vn, &state.ipt_pm_vn, user, pool, kphis, kphis_extra).await?);
                results.extend(sync_vn_paired(pm_an, ipt_an, user, pool, kphis, kphis_extra).await?);
            }
            // else #3 Do nothing
        }
    }

    Ok(results)
}

// sync_vn route A (#4, #5, #6)
async fn sync_vn_ipt_only(ipt_an: &str, pm_vn: &Option<String>, ipt_pm_vn: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    if let Some(vn) = ipt_pm_vn.as_ref() {
        if any_an_exists(ipt_an, pool, kphis, kphis_extra).await? {
            // #6 change AN to self
            results.push(update_pre_admit(vn, "", ipt_an, user, pool, kphis).await?);
            results.extend(update_many_all_an(ipt_an, vn, user, pool, kphis, kphis_extra).await?);
        }
    } else if let Some(vn) = pm_vn.as_ref() {
        if any_an_exists(vn, pool, kphis, kphis_extra).await? {
            // #5 change AN to (2)
            results.push(update_pre_admit(vn, ipt_an, "", user, pool, kphis).await?);
            results.extend(update_many_all_an(vn, ipt_an, user, pool, kphis, kphis_extra).await?);
        }
    }
    // else Do nothing #4

    Ok(results)
}

// sync_vn route B (#7, #8)
async fn sync_vn_pm_only(
    pm_an: &str,
    pm_vn: &str,
    // pm_ipt_vn: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    // hosxp: &str,
    kphis: &str,
    kphis_extra: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    if any_an_exists(pm_an, pool, kphis, kphis_extra).await? {
        // #7, #8 change AN to self
        results.push(update_pre_admit(pm_vn, "", pm_an, user, pool, kphis).await?);
        results.extend(update_many_all_an(pm_an, pm_vn, user, pool, kphis, kphis_extra).await?);
    }
    // if let Some(ipt_vn) = pm_ipt_vn.as_ref() {
    //     // Do recursion
    //     results.extend(sync_vn(ipt_vn, user, pool, hosxp, kphis, kphis_extra).await?);
    // }

    Ok(results)
}

// sync_vn route C (`#9`,`#10`,`#11`)
async fn sync_vn_paired(pm_an: &str, ipt_an: &str, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();
    if any_an_exists(pm_an, pool, kphis, kphis_extra).await? {
        results.push(update_pre_admit(pm_an, ipt_an, pm_an, user, pool, kphis).await?);
        results.extend(update_many_all_an(pm_an, ipt_an, user, pool, kphis, kphis_extra).await?);
    }

    Ok(results)
}

async fn an_in_pre_admit_and_ipt(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<AnInPreAdmitAndIpt, AppError> {
    let sql = transform::an_in_pre_admit_and_ipt(hosxp, kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .bind(an)
        .fetch_one(pool)
        .await
        .as_ref()
        .map(AnInPreAdmitAndIpt::from_row)
        .map_err(|e| Source::SQLx.to_error(500, e, "Select AnInPreAdmitAndIpt"))?
        .map_err(|e| Source::SQLx.to_error(500, e, "Select AnInPreAdmitAndIpt"))
}

async fn vn_in_pre_admit_and_ipt(vn: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<VnInPreAdmitAndIpt, AppError> {
    let sql = transform::vn_in_pre_admit_and_ipt(hosxp, kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(vn)
        .bind(vn)
        .fetch_one(pool)
        .await
        .as_ref()
        .map(VnInPreAdmitAndIpt::from_row)
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VnInPreAdmitAndIpt"))?
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VnInPreAdmitAndIpt"))
}

async fn update_pre_admit(where_vn: &str, an: &str, prev_an: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = pre_admit::update_pre_admit(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(str_some(an.to_owned()))
        .bind(str_some(prev_an.to_owned()))
        .bind(user)
        .bind(where_vn)
        .execute(pool)
        .await
        .map(|response| ExecuteResponse::from_query_result(response, "Update PreAdmit"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update PreAdmit"))
}

async fn insert_revoked_pre_admit(vn: &str, revoked_an: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = pre_admit::insert_revoked_pre_admit(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(vn)
        .bind(revoked_an)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|response| ExecuteResponse::from_query_result(response, "Insert Revoked PreAdmit"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Revoked PreAdmit"))
}

pub async fn any_an_exists(an: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<bool, AppError> {
    let sql = transform::any_an_exists(an, kphis, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .fetch_one(pool)
        .await
        .map(|row| row.try_get(0).unwrap_or_default())
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Any AN Exists"))
}

async fn update_many_all_an(old_an: &str, new_an: &str, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let sql = transform::update_many_all_an(old_an, new_an, user, kphis, kphis_extra);
    pool.execute_many(AssertSqlSafe(sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|result| result.map(|res| ExecuteResponse::from_query_result(res, "Set all AN")))
        .collect::<sqlx::Result<Vec<ExecuteResponse>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Set all AN"))
}

async fn delete_many_all_an(an: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let sql = transform::delete_many_all_an(an, kphis, kphis_extra);
    pool.execute_many(AssertSqlSafe(sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|result| result.map(|res| ExecuteResponse::from_query_result(res, "Delete all of AN")))
        .collect::<sqlx::Result<Vec<ExecuteResponse>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete all of AN"))
}

// ===== ===== //
// Test Helper //
// ===== ===== //

#[cfg(test)]
#[derive(Default, Debug)]
pub struct AnTester {
    test01: NumComparable,
    test02: NumComparable,
    test03: NumComparable,
    test04: NumComparable,
    test05: NumComparable,
    test06: NumComparable,
    test07: NumComparable,
    test08: NumComparable,
    test09: NumComparable,
    test10: NumComparable,
    test11: NumComparable,
    test12: NumComparable,
    test13: NumComparable,
    test14: NumComparable,
    test15: NumComparable,
    test16: NumComparable,
    test17: NumComparable,
    test18: NumComparable,
    test19: NumComparable,
    // test20: NumComparable,
    test21: NumComparable,
    test22: NumComparable,
    test23: NumComparable,
    test24: NumComparable,
}

#[cfg(test)]
use kphis_sqlx_tester::MySqlTester;

#[cfg(test)]
impl AnTester {
    pub async fn add_before(&mut self, old_an: &str, tester: &MySqlTester) {
        self.test01
            .add_before(get_an_count_of_table("ipd_doctor_in_charge", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test02
            .add_before(get_an_count_of_table("ipd_dr_admission_note", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test03
            .add_before(get_an_count_of_table("ipd_dr_admission_note_item", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test04.add_before(get_an_count_of_table("ipd_dr_consult", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test05
            .add_before(get_an_count_of_table("ipd_dr_consult_signature_reply", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test06
            .add_before(get_an_count_of_table("ipd_dr_consult_signature_request", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test07.add_before(get_an_count_of_table("ipd_focus_list", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test08.add_before(get_an_count_of_table("ipd_focus_note", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test09.add_before(get_an_count_of_table("ipd_io", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test10
            .add_before(get_an_count_of_table("ipd_med_reconciliation", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test11
            .add_before(get_an_count_of_table("ipd_med_reconciliation_item", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test12
            .add_before(get_an_count_of_table("ipd_nurse_admission_note", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test13
            .add_before(get_an_count_of_table("ipd_nurse_index_action", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test14
            .add_before(get_an_count_of_table("ipd_nurse_index_note", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test15
            .add_before(get_an_count_of_table("ipd_nurse_index_plan", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test16.add_before(get_an_count_of_table("ipd_order", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test17.add_before(get_an_count_of_table("ipd_order_item", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test18
            .add_before(get_an_count_of_table("ipd_progress_note", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test19
            .add_before(get_an_count_of_table("ipd_progress_note_item", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        // self.test20.add_before(get_an_count_of_table("ipd_summary", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test21.add_before(get_an_count_of_table("ipd_summary_2", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test22
            .add_before(get_an_count_of_table("ipd_vs_vital_sign", old_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test23
            .add_before(get_an_count_of_table("ipd_document", old_an, &tester.db_pool, &tester.kphis_extra).await.unwrap());
        self.test24.add_before(get_an_count_of_table("ipd_mra", old_an, &tester.db_pool, &tester.kphis_extra).await.unwrap());
    }

    pub async fn add_after(&mut self, new_an: &str, tester: &MySqlTester) {
        self.test01
            .add_after(get_an_count_of_table("ipd_doctor_in_charge", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test02
            .add_after(get_an_count_of_table("ipd_dr_admission_note", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test03
            .add_after(get_an_count_of_table("ipd_dr_admission_note_item", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test04.add_after(get_an_count_of_table("ipd_dr_consult", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test05
            .add_after(get_an_count_of_table("ipd_dr_consult_signature_reply", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test06
            .add_after(get_an_count_of_table("ipd_dr_consult_signature_request", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test07.add_after(get_an_count_of_table("ipd_focus_list", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test08.add_after(get_an_count_of_table("ipd_focus_note", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test09.add_after(get_an_count_of_table("ipd_io", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test10
            .add_after(get_an_count_of_table("ipd_med_reconciliation", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test11
            .add_after(get_an_count_of_table("ipd_med_reconciliation_item", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test12
            .add_after(get_an_count_of_table("ipd_nurse_admission_note", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test13
            .add_after(get_an_count_of_table("ipd_nurse_index_action", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test14
            .add_after(get_an_count_of_table("ipd_nurse_index_note", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test15
            .add_after(get_an_count_of_table("ipd_nurse_index_plan", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test16.add_after(get_an_count_of_table("ipd_order", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test17.add_after(get_an_count_of_table("ipd_order_item", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test18.add_after(get_an_count_of_table("ipd_progress_note", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test19
            .add_after(get_an_count_of_table("ipd_progress_note_item", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        // self.test20.add_after(get_an_count_of_table("ipd_summary", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test21.add_after(get_an_count_of_table("ipd_summary_2", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test22.add_after(get_an_count_of_table("ipd_vs_vital_sign", new_an, &tester.db_pool, &tester.kphis).await.unwrap());
        self.test23
            .add_after(get_an_count_of_table("ipd_document", new_an, &tester.db_pool, &tester.kphis_extra).await.unwrap());
        self.test24.add_after(get_an_count_of_table("ipd_mra", new_an, &tester.db_pool, &tester.kphis_extra).await.unwrap());
    }

    pub fn compare_all(&self) {
        self.test01.compare();
        self.test02.compare();
        self.test03.compare();
        self.test04.compare();
        self.test05.compare();
        self.test06.compare();
        self.test07.compare();
        self.test08.compare();
        self.test09.compare();
        self.test10.compare();
        self.test11.compare();
        self.test12.compare();
        self.test13.compare();
        self.test14.compare();
        self.test15.compare();
        self.test16.compare();
        self.test17.compare();
        self.test18.compare();
        self.test19.compare();
        // self.test20.compare();
        self.test21.compare();
        self.test22.compare();
        self.test23.compare();
        self.test24.compare();
    }

    pub fn is_before_zero(&self) -> bool {
        self.test01.is_before_zero() &&
        self.test02.is_before_zero() &&
        self.test03.is_before_zero() &&
        self.test04.is_before_zero() &&
        self.test05.is_before_zero() &&
        self.test06.is_before_zero() &&
        self.test07.is_before_zero() &&
        self.test08.is_before_zero() &&
        self.test09.is_before_zero() &&
        self.test10.is_before_zero() &&
        self.test11.is_before_zero() &&
        self.test12.is_before_zero() &&
        self.test13.is_before_zero() &&
        self.test14.is_before_zero() &&
        self.test15.is_before_zero() &&
        self.test16.is_before_zero() &&
        self.test17.is_before_zero() &&
        self.test18.is_before_zero() &&
        self.test19.is_before_zero() &&
        // self.test20.is_before_zero() &&
        self.test21.is_before_zero() &&
        self.test22.is_before_zero() &&
        self.test23.is_before_zero() &&
        self.test24.is_before_zero()
    }

    pub fn is_after_zero(&self) -> bool {
        self.test01.is_after_zero() &&
        self.test02.is_after_zero() &&
        self.test03.is_after_zero() &&
        self.test04.is_after_zero() &&
        self.test05.is_after_zero() &&
        self.test06.is_after_zero() &&
        self.test07.is_after_zero() &&
        self.test08.is_after_zero() &&
        self.test09.is_after_zero() &&
        self.test10.is_after_zero() &&
        self.test11.is_after_zero() &&
        self.test12.is_after_zero() &&
        self.test13.is_after_zero() &&
        self.test14.is_after_zero() &&
        self.test15.is_after_zero() &&
        self.test16.is_after_zero() &&
        self.test17.is_after_zero() &&
        self.test18.is_after_zero() &&
        self.test19.is_after_zero() &&
        // self.test20.is_after_zero() &&
        self.test21.is_after_zero() &&
        self.test22.is_after_zero() &&
        self.test23.is_after_zero() &&
        self.test24.is_after_zero()
    }
}

#[cfg(test)]
#[derive(Default, Debug)]
struct NumComparable {
    before: i64,
    after: i64,
}

#[cfg(test)]
impl NumComparable {
    fn add_before(&mut self, value: i64) {
        self.before = value;
    }
    fn add_after(&mut self, value: i64) {
        self.after = value;
    }
    fn compare(&self) {
        assert_eq!(self.before, self.after);
    }
    fn is_before_zero(&self) -> bool {
        self.before == 0
    }
    fn is_after_zero(&self) -> bool {
        self.after == 0
    }
}

#[cfg(test)]
async fn get_an_count_of_table(table: &str, an: &str, pool: &Pool<MySql>, database: &str) -> Option<i64> {
    let sql = ["SELECT COUNT(*) AS c FROM ", database, ".", table, " WHERE an=?;"].concat();
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_optional(pool)
        .await
        .unwrap()
        .map(|row| row.try_get("c"))
        .transpose()
        .unwrap()
}

#[rustfmt::skip]
#[cfg(test)]
pub async fn create_all_an(tester: &MySqlTester) {
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
    // sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
    // sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/system_patient_lock.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
}

#[rustfmt::skip]
#[cfg(test)]
pub async fn insert_all_an(tester: &MySqlTester) {
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap(); // 3
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap(); // 2
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap(); // 2
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap(); // 3
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap(); // 3
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap(); // 3
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap(); // 3
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap(); // 3
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap(); // 7
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap(); // 10
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap(); // 4
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap(); // 6
    // sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap(); // 1
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap(); // 3
    // sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/system_patient_lock.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();
    sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    async fn crate_and_insert_pm_ipt(tester: &MySqlTester) {
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_pre_admit_list() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        // status NEEDED
        let default = get_pre_admit_list(&PreAdmitParams::default(),7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());

        let found_status_pre = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("pre")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_pre.len(), 1);
        // pre in kphis.pre_admit_master is 3 but 670202222222 is not exist in hos.ovst 
        let found_status_pre_all = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("pre")),all: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_pre_all.len(), 2);
        // admited in kphis.pre_admit_master is 10 but only 2 exist in hos.ovst
        let found_status_admited = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_admited.len(), 2);
        // all admited in kphis.pre_admit_master is 10 but only 3 exist in hos.ovst
        let found_status_admited_all = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")),all: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_admited_all.len(), 3);
        let found_status_revoked = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("revoked")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_revoked.len(), 1);
        let not_found_status_unknown = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("foobar")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found_status_unknown.is_empty());

        // status NEEDED
        let found_doctor_in_charge = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")), doctor_in_charge: Some(String::from("007")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_doctor_in_charge.len(), 1);

        // status NEEDED
        let found_patient_hn = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")), patient: Some(String::from("1234")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_hn.len(), 2);
        let found_patient_hn_all = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")), patient: Some(String::from("1234")), all: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_hn_all.len(), 3);
        let found_patient_an = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")), patient: Some(String::from("70001234")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_an.len(), 1);
        let found_patient_cid = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")), patient: Some(String::from("1111111111111")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_cid.len(), 2);
        let found_patient_fullname = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")), patient: Some(String::from("สมมุ")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_fullname.len(), 2);

        // status NEEDED
        let not_found = get_pre_admit_list(&PreAdmitParams {status: Some(String::from("admited")), patient: Some(String::from("foobar")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_admit() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_pre_admit(&PreAdmitSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_ignored = insert_pre_admit(&PreAdmitSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_ignored.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_pre_admit_by_an() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_pre_admit_by_an("660001234",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(found.is_some());
        if let Some(pre_admit) = found {
            assert_eq!(pre_admit.vn, String::from("661231235959"));
        }
        let not_found = select_pre_admit_by_an("660006666",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_pre_admit_by_vn() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_pre_admit_by_vn("661231235959",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(found.is_some());
        if let Some(pre_admit) = found {
            assert_eq!(pre_admit.an, Some(String::from("660001234")));
        }
        let not_found = select_pre_admit_by_vn("660000000000",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    // Summarized `ipt` and `ipd_pre_admit_master` table in `kphis-test` after inserted
    // for checking on `sqlx_an_in_pre_admit_and_ipt` and `sqlx_vn_in_pre_admit_and_ipt` test below
    //
    // `ipt` 
    // 
    // | AN      | VN         | ?AN? | ?VN? |
    // | ---     | ---        | ---  | ---  |
    // |660001234|661231235959| #2   | #3   |
    // |660012345|660101111111|      |      |
    // |660023456|660202222222| #5   | #4   |
    // |670001234|670101111111|      |      |
    // |670002345|670202222222| #6   | #5   |
    // |670003456|670303333333| #7   |      |
    // |680001234|680101111111| #8   | #6   |
    // |680002345|680202222222|      | #9   |
    // |680003456|680303333333| #9   | #10  |
    // |680004567|670111111111| #10  |      | 
    // |680005555|680505555555|      |      |
    // |680006666|680606666666|      |      |
    // |680005678|680707777777| #11  | #11  |
    // |680007777|680808888888|      |      |
    //
    // `ipd_pre_admit_master` in kphis-test
    //
    // | VN         | AN      | ?AN? | ?VN? |
    // | ---        | ---     | ---  | ---  |
    // |661231235959|660001234| #2   | #3   |
    // |660101011111|660002345| #3   | #7   |
    // |660101111111|660003456| #4   |      |
    // |670101011111|670001234|      | #8   |
    // |670101111111|         |      |      |
    // |670202222222|         |      | #5   |
    // |670303333333|670003333|      |      |
    // |670111111111|         |      |      |
    // |671111111111|         |      | #2   |
    // |680202222222|680001234| #8   | #9   |
    // |680303333333|680003333|      | #10  |
    // |680404444444|680003456| #9   |      |
    // |680505555555|680004567| #10  |      |
    // |680606666666|680005678| #11  |      |
    // |680707777777|680007777|      | #11  |

    // | #  | pm_vn | pm_an | pm_ipt_an | ipt_vn | ipt_pm_vn | ipt_pm_an | ipt_an | COMMENT 					 	   | ACTION 																|
    // |--- | ---   | ---   | ---       | ---    | ---       | ---       | ---    | ---     						   | ---    																|
    // |    | `-2-` | `-1-` | `-3-`     | `(2)`  | `(3)`     | `(4)`     | `(1)`  |                                    |     																	|
    // |  1 |       |       |           |        |           |           |        | Not found                          | *Check AN exists -> delete AN              						    |
    // |  2 | *111* | `123` | 123       | *111*  | 111       | 123       | `123`  | Admit + Pre-admit admited          | None 																	|
    // |  3 | *111* | `123` |           |        |           |           |        | VN 111 UnAdmit                     | change (VN111:AN123) to (VN111:AN-)   								    |
    // |  4 | *111* | `123` | 456       |        |           |           |        | AN 123 -> 456, No AN 123 in ipt    | change (VN111:AN123) to (VN111:AN456)   								|
    // |  5 |       |       |           | *111*  |           |           | `123`  | Admit + No pre-admit               | None 																	|
    // |  6 |       |       |           | *111*  | 111       |           | `123`  | Admit + Pre-admit not admit        | change (VN111:AN-) to (VN111:AN123)   								    |
    // |  7 |       |       |           | *111*  | 111       | 789       | `123`  | Admit AN 789 -> 123                | change (VN111:AN789) to (VN111:AN123) 								    |
    // |  8 | *111* | `123` | 456       | *444*  |           |           | `123`  | AN 123 -> 456, VN 444 no pre-admit | change (VN111:AN123) to (VN111:AN456)   								|
    // |  9 | *111* | `123` |           | *444*  | 444       | 789       | `123`  | VN 111 UnAdmit, AN 789 -> 123      | change (VN111:AN123) to (VN111:AN-), (VN444:AN789) to (VN444:AN123)    |
    // | 10 | *111* | `123` | 456       | *444*  | 444       |           | `123`  | AN 123 -> 456, Pre-admit not admit | change (VN111:AN123) to (VN111:AN456), (VN444:AN-) to (VN444:AN123)    |
    // | 11 | *111* | `123` | 456       | *444*  | 444       | 789       | `123`  | AN 123 -> 456, AN 789 -> 123       | change (VN111:AN123) to (VN111:AN456), (VN444:AN789) to (VN444:AN123)  |
    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_01() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "660006666";
        let result_01 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_01, AnInPreAdmitAndIpt {
            pm_vn: None,
            pm_an: None,
            pm_ipt_an: None,
            ipt_vn: None,
            ipt_pm_vn: None,
            ipt_pm_an: None,
            ipt_an: None,
        });
        // Check AN exists -> delete AN
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(!any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_02() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "660001234";
        let result_02 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_02, AnInPreAdmitAndIpt {
            pm_vn: Some(String::from("661231235959")),
            pm_an: Some(String::from(test_an)),
            pm_ipt_an: Some(String::from(test_an)),
            ipt_vn: Some(String::from("661231235959")),
            ipt_pm_vn: Some(String::from("661231235959")),
            ipt_pm_an: Some(String::from(test_an)),
            ipt_an: Some(String::from(test_an)),
        });
        // None
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before(test_an, &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after(test_an, &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_03() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "660002345";
        let result_03 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_03, AnInPreAdmitAndIpt {
            pm_vn: Some(String::from("660101011111")),
            pm_an: Some(String::from(test_an)),
            pm_ipt_an: None,
            ipt_vn: None,
            ipt_pm_vn: None,
            ipt_pm_an: None,
            ipt_an: None,
        });
        // change (VN660101011111:AN660002345) to (VN660101011111:AN660101011111)  
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before(test_an, &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("660101011111", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_04() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "660003456";
        let result_04 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_04, AnInPreAdmitAndIpt {
            pm_vn: Some(String::from("660101111111")),
            pm_an: Some(String::from(test_an)),
            pm_ipt_an: Some(String::from("660012345")),
            ipt_vn: None,
            ipt_pm_vn: None,
            ipt_pm_an: None,
            ipt_an: None,
        });
        // change (VN660101111111:AN660003456) to (VN660101111111:AN660012345)
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before(test_an, &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("660012345", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_05() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "660023456";
        let result_05 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_05, AnInPreAdmitAndIpt {
            pm_vn: None,
            pm_an: None,
            pm_ipt_an: None,
            ipt_vn: Some(String::from("660202222222")),
            ipt_pm_vn: None,
            ipt_pm_an: None,
            ipt_an: Some(String::from(test_an)),
        });
        // None
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before(test_an, &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after(test_an, &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_06() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "670002345";
        let result_06 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_06, AnInPreAdmitAndIpt {
            pm_vn: None,
            pm_an: None,
            pm_ipt_an: None,
            ipt_vn: Some(String::from("670202222222")),
            ipt_pm_vn: Some(String::from("670202222222")),
            ipt_pm_an: None,
            ipt_an: Some(String::from(test_an)),
        });
        // change (VN670202222222:AN670202222222) to (VN670202222222:AN670002345)
        update_many_all_an("660001234","670202222222","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("670202222222",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("670202222222", &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after(test_an, &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists("670202222222",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_07() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "670003456";
        let result_07 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_07, AnInPreAdmitAndIpt {
            pm_vn: None,
            pm_an: None,
            pm_ipt_an: None,
            ipt_vn: Some(String::from("670303333333")),
            ipt_pm_vn: Some(String::from("670303333333")),
            ipt_pm_an: Some(String::from("670003333")),
            ipt_an: Some(String::from(test_an)),
        });
        // change (VN670303333333:AN670003333) to (VN670303333333:AN670003456)
        update_many_all_an("660001234","670003333","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("670003333",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("670003333", &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after(test_an, &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists("670003333",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_08() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "680001234";
        let result_08 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_08, AnInPreAdmitAndIpt {
            pm_vn: Some(String::from("680202222222")),
            pm_an: Some(String::from(test_an)),
            pm_ipt_an: Some(String::from("680002345")),
            ipt_vn: Some(String::from("680101111111")),
            ipt_pm_vn: None,
            ipt_pm_an: None,
            ipt_an: Some(String::from(test_an)),
        });
        // change (VN680202222222:AN680001234) to (VN680202222222:AN680002345)
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before(test_an, &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("680002345", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_09() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "680003456";
        let result_09 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_09, AnInPreAdmitAndIpt {
            pm_vn: Some(String::from("680404444444")),
            pm_an: Some(String::from(test_an)),
            pm_ipt_an: None,
            ipt_vn: Some(String::from("680303333333")),
            ipt_pm_vn: Some(String::from("680303333333")),
            ipt_pm_an: Some(String::from("680003333")),
            ipt_an: Some(String::from(test_an)),
        });
        // change (VN680404444444:AN680003456) to (VN680404444444:AN680404444444), (VN680303333333:AN680003333) to (VN680303333333:AN680003456)
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        update_many_all_an("670001234","680003333","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680003333",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester1 = AnTester::default();
        let mut an_tester2 = AnTester::default();
        an_tester1.add_before(test_an, &tester).await;
        an_tester2.add_before("680003333", &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester1.add_after("680404444444", &tester).await;
        an_tester2.add_after(test_an, &tester).await;
        an_tester1.compare_all();
        an_tester2.compare_all();
        assert!(!any_an_exists("680003333",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_10() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "680004567";
        let result_10 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_10, AnInPreAdmitAndIpt {
            pm_vn: Some(String::from("680505555555")),
            pm_an: Some(String::from(test_an)),
            pm_ipt_an: Some(String::from("680005555")),
            ipt_vn: Some(String::from("670111111111")),
            ipt_pm_vn: Some(String::from("670111111111")),
            ipt_pm_an: None,
            ipt_an: Some(String::from(test_an)),
        });
        // change (VN680505555555:AN680004567) to (VN680505555555:AN680005555), (VN670111111111:AN670111111111) to (VN670111111111:AN680004567)
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        update_many_all_an("670001234","670111111111","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("670111111111",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester1 = AnTester::default();
        let mut an_tester2 = AnTester::default();
        an_tester1.add_before(test_an, &tester).await;
        an_tester2.add_before("670111111111", &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester1.add_after("680005555", &tester).await;
        an_tester2.add_after(test_an, &tester).await;
        an_tester1.compare_all();
        an_tester2.compare_all();
        assert!(!any_an_exists("670111111111",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_an_in_pre_admit_and_ipt_11() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_an = "680005678";
        let result_11 = an_in_pre_admit_and_ipt(test_an,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_11, AnInPreAdmitAndIpt {
            pm_vn: Some(String::from("680606666666")),
            pm_an: Some(String::from(test_an)),
            pm_ipt_an: Some(String::from("680006666")),
            ipt_vn: Some(String::from("680707777777")),
            ipt_pm_vn: Some(String::from("680707777777")),
            ipt_pm_an: Some(String::from("680007777")),
            ipt_an: Some(String::from(test_an)),
        });
        // change (VN680606666666:AN680005678) to (VN680606666666:AN680006666), (VN680707777777:AN680007777) to (VN680707777777:AN680005678)
        update_many_all_an("660001234",test_an,"user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists(test_an,&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        update_many_all_an("670001234","680007777","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680007777",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester1 = AnTester::default();
        let mut an_tester2 = AnTester::default();
        an_tester1.add_before(test_an, &tester).await;
        an_tester2.add_before("680007777", &tester).await;
        sync_an(test_an,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester1.add_after("680006666", &tester).await;
        an_tester2.add_after(test_an, &tester).await;
        an_tester1.compare_all();
        an_tester2.compare_all();
        assert!(!any_an_exists("680007777",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    // | #  | pm_vn | pm_an | pm_ipt_vn | ipt_vn | ipt_pm_vn | ipt_an | COMMENT 						        			| ACTION 																|
    // |--- | ---   | ---   | ---       | ---    | ---       | ---    | ---     						        			| ---  																	|
    // |    | `-1-` | `-2-` | `-3-`     | `(1)`  | `(3)`     | `(2)`  |         						        			|     																	|
    // |  1 |       |       |           |        |           |        | Not found                               			| None 																	|
    // |  2 | `111` |       |           |        |           |        | VN 111 not admit	                     			| None   																|
    // |  3 | `111` | *123* | 111       | `111`  | 111       | *123*  | Admit + Pre-admit admited               			| None 																	|
    // |  4 |       |       |           | `111`  |           | *123*  | Admit + No pre-admit                    			| None 																	|
    // |  5 | `111` |       |           | `111`  |           | *123*  | Admit + Pre-admit not admit             			| change (VN111:AN-) to (VN111:AN123)   								|
    // |  6 |       |       |           | `111`  | 444       | *123*  | VN 444 UnAdmit, VN 111 no pre-admit                 | change (VN444:AN123) to (VN444:AN-)                                	|
    // |  7 | `111` | *123* |           |        |           |        | AN 123 revoked                         			    | change (VN111:AN123) to (VN111:AN-)   								|
    // |  8 | `111` | *123* | 777       |        |           |        | AN 123 revoked, `VN 777`?                			| change (VN111:AN123) to (VN111:AN-)   								|
    // |  9 | `111` | *789* | 777       | `111`  |           | *123*  | AN 789 -> 123, `VN 777`?                 			| change (VN111:AN789) to (VN111:AN123)   								|
    // | 10 | `111` | *789* |           | `111`  | 444       | *123*  | VN 444 UnAdmit, AN 789 -> 123           			| change (VN444:AN123) to (VN444:AN-), (VN111:AN789) to (VN111:AN123)   |
    // | 11 | `111` | *789* | 777       | `111`  | 444       | *123*  | VN 444 UnAdmit, AN 789 -> 123, `VN 777` 			| change (VN444:AN123) to (VN444:AN-), (VN111:AN789) to (VN111:AN123)   |
    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_01() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "660606066666";
        let result_01 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_01, VnInPreAdmitAndIpt {
            pm_vn: None,
            pm_an: None,
            pm_ipt_vn: None,
            ipt_vn: None,
            ipt_pm_vn: None,
            ipt_an: None,
        });
        // None
        assert!(sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_02() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "671111111111";
        let result_02 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_02, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: None,
            pm_ipt_vn: None,
            ipt_vn: None,
            ipt_pm_vn: None,
            ipt_an: None,
        });
        // None
        assert!(sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_03() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "661231235959";
        let result_03 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_03, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: Some(String::from("660001234")),
            pm_ipt_vn: Some(String::from(test_vn)),
            ipt_vn: Some(String::from(test_vn)),
            ipt_pm_vn: Some(String::from(test_vn)),
            ipt_an: Some(String::from("660001234")),
        });
        // None
        assert!(any_an_exists("660001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("660001234", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("660001234", &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_04() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "660202222222";
        let result_04 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_04, VnInPreAdmitAndIpt {
            pm_vn: None,
            pm_an: None,
            pm_ipt_vn: None,
            ipt_vn: Some(String::from(test_vn)),
            ipt_pm_vn: None,
            ipt_an: Some(String::from("660023456")),
        });
        // None
        update_many_all_an("660001234","660023456","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("660023456",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("660023456", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("660023456", &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_05() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "670202222222";
        let result_05 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_05, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: None,
            pm_ipt_vn: None,
            ipt_vn: Some(String::from(test_vn)),
            ipt_pm_vn: None,
            ipt_an: Some(String::from("670002345")),
        });
        // change (VN670202222222:AN670202222222) to (VN670202222222:AN670002345)
        update_many_all_an("660001234","670202222222","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("670202222222",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("670202222222", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("670002345", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists("670202222222",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_06() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "680101111111";
        let result_06 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_06, VnInPreAdmitAndIpt {
            pm_vn: None,
            pm_an: None,
            pm_ipt_vn: None,
            ipt_vn: Some(String::from(test_vn)),
            ipt_pm_vn: Some(String::from("680202222222")),
            ipt_an: Some(String::from("680001234")),
        });
        // change (VN680202222222:AN680001234) to (VN680202222222:AN680202222222)
        update_many_all_an("660001234","680001234","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("680001234", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("680202222222", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists("680001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_07() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "660101011111";
        let result_07 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_07, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: Some(String::from("660002345")),
            pm_ipt_vn: None,
            ipt_vn: None,
            ipt_pm_vn: None,
            ipt_an: None,
        });
        // change (VN660101011111:AN660002345) to (VN660101011111:AN660101011111)
        update_many_all_an("660001234","660002345","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("660002345",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("660002345", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("660101011111", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists("660002345",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_08() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "670101011111";
        let result_08 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_08, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: Some(String::from("670001234")),
            pm_ipt_vn: Some(String::from("670101111111")),
            ipt_vn: None,
            ipt_pm_vn: None,
            ipt_an: None,
        });
        // change (VN670101011111:AN670001234) to (VN670101011111:AN670101011111)
        assert!(any_an_exists("670001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("670001234", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("670101011111", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists("670001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        // TODO check 670101111111
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_09() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "680202222222";
        let result_09 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_09, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: Some(String::from("680001234")),
            pm_ipt_vn: Some(String::from("680101111111")),
            ipt_vn: Some(String::from(test_vn)),
            ipt_pm_vn: None,
            ipt_an: Some(String::from("680002345")),
        });
        // change (VN680202222222:AN680001234) to (VN680202222222:AN680002345)
        update_many_all_an("660001234","680001234","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester = AnTester::default();
        an_tester.add_before("680001234", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester.add_after("680002345", &tester).await;
        an_tester.compare_all();
        assert!(!any_an_exists("680001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        // TODO check 680101111111
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_10() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "680303333333";
        let result_10 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_10, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: Some(String::from("680003333")),
            pm_ipt_vn: None,
            ipt_vn: Some(String::from(test_vn)),
            ipt_pm_vn: Some(String::from("680404444444")),
            ipt_an: Some(String::from("680003456")),
        });
        // change (VN680404444444:AN680003456) to (VN680404444444:AN680404444444), (VN680303333333:AN680003333) to (VN680303333333:AN680003456)
        update_many_all_an("660001234","680003456","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680003456",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        update_many_all_an("670001234","680003333","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680003333",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester1 = AnTester::default();
        let mut an_tester2 = AnTester::default();
        an_tester1.add_before("680003456", &tester).await;
        an_tester2.add_before("680003333", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester1.add_after("680404444444", &tester).await;
        an_tester2.add_after("680003456", &tester).await;
        an_tester1.compare_all();
        an_tester2.compare_all();
        assert!(!any_an_exists("680003333",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_vn_in_pre_admit_and_ipt_11() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        crate_and_insert_pm_ipt(&tester).await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let test_vn = "680707777777";
        let result_11 = vn_in_pre_admit_and_ipt(test_vn,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(result_11, VnInPreAdmitAndIpt {
            pm_vn: Some(String::from(test_vn)),
            pm_an: Some(String::from("680007777")),
            pm_ipt_vn: Some(String::from("680808888888")),
            ipt_vn: Some(String::from(test_vn)),
            ipt_pm_vn: Some(String::from("680606666666")),
            ipt_an: Some(String::from("680005678")),
        });
        // change (VN680606666666:AN680005678) to (VN680606666666:AN680606666666), (VN680707777777:AN680007777) to (VN680707777777:AN680005678)
        update_many_all_an("660001234","680005678","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680005678",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        update_many_all_an("670001234","680007777","user",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(any_an_exists("680007777",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        let mut an_tester1 = AnTester::default();
        let mut an_tester2 = AnTester::default();
        an_tester1.add_before("680005678", &tester).await;
        an_tester2.add_before("680007777", &tester).await;
        sync_vn(test_vn,"user",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        an_tester1.add_after("680606666666", &tester).await;
        an_tester2.add_after("680005678", &tester).await;
        an_tester1.compare_all();
        an_tester2.compare_all();
        assert!(!any_an_exists("680007777",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        // TODO check 680808888888 
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_pre_admit() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_pre_admit("661231235959","661231235959","660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_pre_admit("661231235959","661231235959","660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_revoked_pre_admit() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_revoked_pre_admit("661231235959","660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_ignored = insert_revoked_pre_admit("661231235959","660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_ignored.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_any_an_exists() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        create_all_an(&tester).await;

        assert!(!any_an_exists("660001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());

        insert_all_an(&tester).await;

        assert!(any_an_exists("660001234",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
        assert!(!any_an_exists("660006666",&tester.db_pool,&tester.kphis,&tester.kphis_extra).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_many_all_an() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let mut an_tester = AnTester::default();

        let old_an = "660001234";
        an_tester.add_before(old_an, &tester).await;

        let new_an = "123456789012";
        assert!(update_many_all_an(old_an, new_an, "user", &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(update_many_all_an(old_an, new_an, "user", &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());

        an_tester.add_after(new_an, &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_many_all_an() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let mut an_tester = AnTester::default();

        let an = "660001234";
        an_tester.add_before(an, &tester).await;

        assert!(delete_many_all_an(an, &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(delete_many_all_an(an, &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());

        an_tester.add_after(an, &tester).await;
        assert!(!an_tester.is_before_zero());
        assert!(an_tester.is_after_zero());
    }
}
