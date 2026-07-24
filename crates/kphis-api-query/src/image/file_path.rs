use std::collections::{HashMap, HashSet};

use sqlx::{
    AssertSqlSafe, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};
use time::PrimitiveDateTime;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::trace;
use ulid::Ulid;

use kphis_model::{
    PATH_PREFIX_IMAGE, PATH_PREFIX_THUMB,
    image::file_path::{ImagePath, ImageSave, ImageUsage},
};
use kphis_sql::image::file_path;
use kphis_util::error::{AppError, Source};

pub async fn post_image_file(files: &[ImageSave], user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<ImagePath>, AppError> {
    let filenames = files.iter().map(|file| &file.file_name).collect::<HashSet<&String>>();
    let filename_map = filenames.into_iter().map(|name| (name, new_ulid_to_path())).collect::<HashMap<&String, String>>();
    let mut image_paths = Vec::with_capacity(files.len());
    // file first
    for file in files.iter() {
        if let Some(file_name) = filename_map.get(&file.file_name) {
            let file_path = ["volume", &file.name, file_name].join("/");
            let current = std::env::current_dir().map_err(|e| Source::Io.to_error(500, e, "PostImageFile"))?;
            let path = current.join(file_path);
            if let Some(prefix) = path.parent() {
                // save file to disk
                tokio::fs::create_dir_all(prefix).await.map_err(|e| Source::Io.to_error(500, e, "PostImageFile"))?;
                let mut f = File::create(&path).await.map_err(|e| Source::Io.to_error(500, e, "PostImageFile"))?;
                f.write_all(&file.bytes).await.map_err(|e| Source::Io.to_error(500, e, "PostImageFile"))?;
                trace!("Received field: {} {} {} ({} bytes)", &file.name, &file.file_name, &file.content_type, file.bytes.len());
                if file.name.as_str() == PATH_PREFIX_IMAGE {
                    image_paths.push(file_name.to_owned());
                }
            }
        }
    }
    // then database
    let results = if !image_paths.is_empty() {
        post_image_path(&image_paths, user, pool, kphis_extra).await?
    } else {
        Vec::new()
    };

    Ok(results)
}

pub async fn post_image_path(image_paths: &[String], user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<ImagePath>, AppError> {
    let sql = file_path::insert_image(image_paths.len(), kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    for image_path in image_paths.iter() {
        query = query.bind(image_path).bind(user).bind(user);
    }
    // now we cannot use `from_row`` by `FromRow`` trait with `RETURNING` in MariaDB
    // because MariaDB returning without column name (https://github.com/launchbadge/sqlx/issues/1530)
    // but we can `try_get` by index manually
    let results = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ImagePath1"))?
        .iter()
        .map(image_return_from_row)
        .collect::<sqlx::Result<Vec<ImagePath>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ImagePath"))?;

    Ok(results)
}

pub async fn patch_image_path(title: &Option<String>, image_id: u32, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = file_path::update_image_title(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(title)
        .bind(image_id)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ImagePath"))
}

pub async fn delete_image_file(image_ids: &[u32], user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    // database first
    let db_result = delete_image_path(image_ids, user, pool, kphis_extra).await?;

    // then file
    let paths = get_image_path_by_ids(image_ids, pool, kphis_extra).await?;
    for path_suffix in paths.iter() {
        let current = std::env::current_dir().map_err(|e| Source::Io.to_error(500, e, "DeleteImageFile"))?;
        // delete image
        let image_file_path = ["volume", PATH_PREFIX_IMAGE, path_suffix].join("/");
        let im_path = current.join(&image_file_path);
        tokio::fs::remove_file(&im_path).await.map_err(|e| Source::Io.to_error(500, e, "DeleteImageFile"))?;
        trace!("file {} deleted", im_path.to_string_lossy());
        // delete thumbnail
        let thumb_file_path = ["volume", PATH_PREFIX_THUMB, path_suffix].join("/");
        let tm_path = current.join(&thumb_file_path);
        tokio::fs::remove_file(&tm_path).await.map_err(|e| Source::Io.to_error(500, e, "DeleteImageFile"))?;
        trace!("file {} deleted", tm_path.to_string_lossy());
    }
    Ok(db_result)
}

async fn get_image_path_by_ids(image_ids: &[u32], pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<String>, AppError> {
    let sql = file_path::select_image_path_by_ids(image_ids.len(), kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    for image_id in image_ids.iter() {
        query = query.bind(image_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::new_server(500, &e.to_string(), "Select ImagePathByIDs"))?
        .iter()
        .filter_map(|row| row.try_get::<Option<String>, usize>(0).transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| AppError::new_server(500, &e.to_string(), "Select ImagePathByIDs"))
}

// async fn get_image_by_loginname(
//     user: &str,
//     pool: &Pool<MySql>,
//     hosxp: &str, kphis_extra: &str,
// ) -> Result<Vec<ImagePath>, AppError> {
//     let sql = file_path::select_image_by_loginname(hosxp, kphis_extra);
//     sqlx::query(AssertSqlSafe(sql)).bind(user).fetch_all(pool).await
//         .map_err(|e| AppError::new_server(500, &e.to_owned(), "Select ImagePathByUser"))?
//         .iter().map(image_from_row_with_username).collect::<sqlx::Result<Vec<ImagePath>>>()
//         .map_err(|e| AppError::new_server(500, &e.to_owned(), "Select ImagePathByUser"))
// }

pub async fn delete_image_path(image_ids: &[u32], user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = file_path::delete_image(image_ids.len(), kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    for image_id in image_ids.iter() {
        query = query.bind(image_id);
    }
    query.bind(user).execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Delete ImagePath"))
}

pub async fn get_image_usage_id(usage_id: u32, usage_key_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis_extra: &str) -> Result<Vec<ImagePath>, AppError> {
    let sql = file_path::select_image_usage_by_usage_id(hosxp, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(usage_id)
        .bind(usage_key_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ImageUsage"))?
        .iter()
        .map(image_path_from_row)
        .collect::<sqlx::Result<Vec<ImagePath>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ImageUsage"))
}

pub async fn post_image_usage(images: &[ImagePath], user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = file_path::insert_image_usage(images.len(), kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    for image in images.iter() {
        query = query
            .bind(image.usage_id.clone().map(|usage| usage as u32))
            .bind(image.usage_key_id)
            .bind(image.image_id)
            .bind(user)
            .bind(user);
    }
    query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert ImageUsage"))
}

pub async fn delete_image_usage(image_usage_ids: &[u32], user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = file_path::delete_image_usage(image_usage_ids.len(), kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    for image_usage_id in image_usage_ids.iter() {
        query = query.bind(image_usage_id);
    }
    query.bind(user).execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Delete ImageUsage"))
}

fn image_return_from_row(row: &MySqlRow) -> sqlx::Result<ImagePath> {
    Ok(ImagePath {
        image_id: row.try_get::<u32, usize>(0)?,
        path: row.try_get::<String, usize>(1)?,
        title: row.try_get::<Option<String>, usize>(2)?,
        create_datetime: row.try_get::<PrimitiveDateTime, usize>(3)?,
        image_usage_id: None,
        usage_id: None,
        usage_key_id: None,
        create_username: None,
    })
}

// fn image_from_row_with_username(row: &MySqlRow) -> sqlx::Result<ImagePath> {
//     Ok(ImagePath {
//         image_id: row.try_get::<u32, &str>("image_id")?,
//         path: row.try_get::<String, &str>("path")?,
//         title: row.try_get::<Option<String>, &str>("title")?,
//         create_datetime: row.try_get::<PrimitiveDateTime, &str>("create_datetime")?,
//         image_usage_id: None,
//         usage_id: None,
//         usage_key_id: None,
//         create_username: row.try_get::<Option<String>, &str>("create_username")?,
//     })
// }

fn image_path_from_row(row: &MySqlRow) -> sqlx::Result<ImagePath> {
    Ok(ImagePath {
        image_id: row.try_get::<u32, &str>("image_id")?,
        path: row.try_get::<String, &str>("path")?,
        title: row.try_get::<Option<String>, &str>("title")?,
        create_datetime: row.try_get::<PrimitiveDateTime, &str>("create_datetime")?,
        image_usage_id: row.try_get::<Option<u32>, &str>("image_usage_id")?,
        usage_id: row.try_get::<Option<u8>, &str>("usage_id")?.map(ImageUsage::new_from_u8),
        usage_key_id: row.try_get::<Option<u32>, &str>("usage_key_id")?,
        create_username: row.try_get::<Option<String>, &str>("create_username")?,
    })
}

/// new Ulid to `01J/G0/M004KYHATX7J2W7MB28X4.webp`
fn new_ulid_to_path() -> String {
    let mut s = Ulid::generate().to_string();
    // please copy 3 lines below to test_new_ulid_to_path()
    s.insert_str(s.len(), ".webp");
    s.insert(5, '/');
    s.insert(3, '/');
    s
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_image_path_by_ids() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_image_path_by_ids(&[1,2], &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_image_path_by_ids(&[999], &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    // #[tokio::test]
    // async fn sqlx_get_image_by_loginname() {
    //     let tester = MySqlTester::new_hosxp_and_kphis_extra().await;
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

    //     let found = get_image_by_loginname("user", &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
    //     assert_eq!(found.len(), 2);
    //     let not_found = get_image_by_loginname("unknown", &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
    //     assert!(not_found.is_empty());
    // }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_image_path() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();

        let paths_1 = [String::from("01J/G0/M004KYHATX7J2W7MB28X1.webp"),String::from("01J/G0/M004KYHATX7J2W7MB28X2.webp")];
        let found = post_image_path(&paths_1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 2);
        let paths_2 = [String::from("01J/G0/M004KYHATX7J2W7MB28X1.webp"),String::from("01J/G0/M004KYHATX7J2W7MB28X3.webp")];
        let again_ignored = post_image_path(&paths_2, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_ignored.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_patch_image_path() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();

        let default = patch_image_path(&None, 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.rows_affected(), 1);
        let success = patch_image_path(&Some(String::from("NEW TITLE")), 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let fail_image_id = patch_image_path(&Some(String::from("NEW TITLE")), 999, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(fail_image_id.rows_affected(), 0);
        let fail_user = patch_image_path(&Some(String::from("NEW TITLE")), 1, "tester", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(fail_user.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_image_path() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let success_both = delete_image_path(&[1], "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success_both.rows_affected(), 3); // 1 image + 2 image_usage
        let success = delete_image_path(&[4], "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1); // 1 image
        let fail_image_id = delete_image_path(&[999], "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(fail_image_id.rows_affected(), 0);
        let fail_user = delete_image_path(&[2], "tester", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(fail_user.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_image_usage_id() {
        let tester = MySqlTester::new_hosxp_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_image_usage_id(1, 1, &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found_key_id = get_image_usage_id(1, 999, &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert!(not_found_key_id.is_empty());
        let not_found_usage = get_image_usage_id(0, 1, &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert!(not_found_usage.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_image_usage() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_image_usage(&[ImagePath::demo()], "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_ignored = post_image_usage(&[ImagePath::demo()], "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_ignored.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_image_usage() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_image_usage(&[1], "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let fail_image_id = delete_image_usage(&[999], "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(fail_image_id.rows_affected(), 0);
        let fail_user = delete_image_usage(&[2], "tester", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(fail_user.rows_affected(), 0);
    }

    #[test]
    pub fn test_new_ulid_to_path() {
        // mock Ulid
        let mut s = String::from("01JG0M004KYHATX7J2W7MB28X4");
        // copy from new_ulid_to_path()
        s.insert_str(s.len(), ".webp");
        s.insert(5, '/');
        s.insert(3, '/');

        assert_eq!(s, String::from("01J/G0/M004KYHATX7J2W7MB28X4.webp"));
    }
}
