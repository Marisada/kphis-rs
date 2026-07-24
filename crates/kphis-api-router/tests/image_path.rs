// test kphis-handler::image::file_path::*

mod common;

use axum::http::StatusCode;
use axum_test::multipart::{MultipartForm, Part};
use tokio::sync::broadcast;
use ulid::Ulid;

use kphis_api_pdf::test_state::new_test_state;
use kphis_model::{PATH_PREFIX_IMAGE, PATH_PREFIX_THUMB, endpoint::EndPoint, fetch::ExecuteResponse, image::file_path::ImagePath};
use kphis_sqlx_tester::MySqlMocker;
use kphis_util::error::{AppError, Source};

use common::new_test_app_login;

// NOTE: axum-test multipart's Part cannot empty `name` and `content-type`
// we cannot test them now
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore]
async fn api_image_path() {
    // mocker will crate/insert all databases and tables, so we try to run many test as possible
    let mocker = MySqlMocker::new_all().await;
    let (shutdown_sender, _shutdown_recv) = broadcast::channel(5);
    let state = new_test_state(mocker.db_pool.clone(), shutdown_sender).await;
    let server = new_test_app_login(&state).await;

    // POST empty Multipart
    let multipart_empty = MultipartForm::new();
    let post_multipart_empty = server.post(&EndPoint::Image.base()).multipart(multipart_empty).expect_success().await;
    let post_multipart_empty_paths = post_multipart_empty.json::<Vec<ImagePath>>();
    assert!(post_multipart_empty_paths.is_empty());

    // POST empty-part-filename Multipart
    let multipart_empty_filename = MultipartForm::new().add_part(PATH_PREFIX_IMAGE, Part::bytes(b"test_thumb".as_slice()));
    let post_multipart_empty_filename = server.post(&EndPoint::Image.base()).multipart(multipart_empty_filename).expect_failure().await;
    assert_eq!(post_multipart_empty_filename.status_code(), StatusCode::BAD_REQUEST);
    let post_multipart_empty_filename_error = post_multipart_empty_filename.json::<AppError>();
    assert_eq!(post_multipart_empty_filename_error.source, Source::App);
    assert_eq!(post_multipart_empty_filename_error.message.as_str(), "No field 'file_name'");

    // POST not webp part-content-type Multipart
    let multipart_not_webp = MultipartForm::new().add_part(PATH_PREFIX_IMAGE, Part::bytes(b"test_thumb".as_slice()).file_name(&Ulid::generate().to_string()).mime_type("image/png"));
    let post_multipart_not_webp = server.post(&EndPoint::Image.base()).multipart(multipart_not_webp).expect_failure().await;
    assert_eq!(post_multipart_not_webp.status_code(), StatusCode::BAD_REQUEST);
    let post_multipart_not_webp_error = post_multipart_not_webp.json::<AppError>();
    assert_eq!(post_multipart_not_webp_error.source, Source::App);
    assert_eq!(post_multipart_not_webp_error.message.as_str(), "'content_type' not `webp`");

    // POST image + thumbnail files with Multipart
    let filename_success = Ulid::generate().to_string();
    let multipart_success = MultipartForm::new()
        .add_part(PATH_PREFIX_IMAGE, Part::bytes(b"test_image".as_slice()).file_name(&filename_success).mime_type("image/webp"))
        .add_part(PATH_PREFIX_THUMB, Part::bytes(b"test_thumb".as_slice()).file_name(&filename_success).mime_type("image/webp"));
    let post_image_success = server.post(&EndPoint::Image.base()).multipart(multipart_success).expect_success().await;
    let post_image_success_paths = post_image_success.json::<Vec<ImagePath>>();
    assert_eq!(post_image_success_paths.len(), 1);

    // DELETE image
    let image_ids = post_image_success_paths.iter().map(|im| im.image_id).collect::<Vec<u32>>();
    let delete_image_success = server.delete(&EndPoint::Image.base()).json(&image_ids).expect_success().await;
    assert_eq!(delete_image_success.json::<ExecuteResponse>().rows_affected, 1);

    // DELETE image again
    let delete_image_again_success = server.delete(&EndPoint::Image.base()).json(&image_ids).expect_success().await;
    assert_eq!(delete_image_again_success.json::<ExecuteResponse>().rows_affected, 0);

    // POST empty ImageUsage
    let post_empty_usage = server.post(&EndPoint::ImageUsage.base()).json(&Vec::<ImagePath>::new()).expect_success().await;
    assert_eq!(post_empty_usage.json::<ExecuteResponse>().rows_affected, 0);

    // POST ImageUsage
    let post_usage_success = server.post(&EndPoint::ImageUsage.base()).json(&vec![ImagePath::demo()]).expect_success().await;
    let post_usage_success_result = post_usage_success.json::<ExecuteResponse>();
    assert_eq!(post_usage_success_result.rows_affected, 1);

    // DELETE empty image_id
    let delete_empty_usage = server.delete(&EndPoint::ImageUsage.base()).json(&Vec::<u32>::new()).expect_success().await;
    assert_eq!(delete_empty_usage.json::<ExecuteResponse>().rows_affected, 0);

    // DELETE image_id
    let delete_usage_success = server
        .delete(&EndPoint::ImageUsage.base())
        .json(&vec![post_usage_success_result.last_insert_id as u32])
        .expect_success()
        .await;
    assert_eq!(delete_usage_success.json::<ExecuteResponse>().rows_affected, 1);
}
