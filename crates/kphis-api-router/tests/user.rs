// test kphis-handler::user::token::*

mod common;

use axum::http::StatusCode;
use tokio::sync::broadcast;
use ulid::Ulid;

use kphis_api_core::token::get_claim_public;
use kphis_api_handler::user::his::COOKIE_TOKEN_NAME;
use kphis_api_pdf::test_state::new_test_state;
use kphis_model::{
    endpoint::EndPoint,
    user::his::{LoginResponse, UserRequest, UserRequestFull},
};
use kphis_sqlx_tester::MySqlMocker;
use kphis_util::error::{AppError, Source};

use common::new_test_app;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore]
async fn api_user() {
    // mocker will crate/insert all databases and tables, so we try to run many test as possible
    let mocker = MySqlMocker::new_all().await;
    let (shutdown_sender, _shutdown_recv) = broadcast::channel(5);
    let state = new_test_state(mocker.db_pool.clone(), shutdown_sender).await;
    let mut server = new_test_app(&state).await;

    // Try GET Access Token without Refresh Token
    let try_get_refresh = server.get(&EndPoint::User.base()).expect_failure().await;
    assert_eq!(try_get_refresh.status_code(), StatusCode::UNAUTHORIZED);
    let try_get_refresh_error = try_get_refresh.json::<AppError>();
    assert_eq!(try_get_refresh_error.source, Source::App);
    assert_eq!(try_get_refresh_error.status, 401);

    // Try PUT without Refresh Token and username is not state_id
    let mut user = UserRequestFull::demo();
    user.username = String::from("user");
    user.token_2fa = String::new();
    let try_put_refresh = server.put(&EndPoint::User.base()).json(&user).expect_failure().await;
    assert_eq!(try_put_refresh.status_code(), StatusCode::UNAUTHORIZED);
    let try_put_refresh_error = try_put_refresh.json::<AppError>();
    assert_eq!(try_put_refresh_error.source, Source::UlidDecode);
    assert_eq!(try_put_refresh_error.message.as_str(), "invalid length");

    // Try Logout without Token
    let try_logout = server.delete(&EndPoint::Sse.base()).expect_failure().await;
    assert_eq!(try_logout.status_code(), StatusCode::BAD_REQUEST);
    let try_logout_error = try_logout.json::<AppError>();
    assert_eq!(try_logout_error.source, Source::App);
    assert_eq!(try_logout_error.message.as_str(), "Token Not Found");

    // Try PUT without Refresh Token and username is a random state_id
    user.username = Ulid::new().to_string();
    let try_put_refresh_random = server.put(&EndPoint::User.base()).json(&user).expect_failure().await;
    assert_eq!(try_put_refresh_random.status_code(), StatusCode::UNAUTHORIZED);
    let try_put_refresh_random_error = try_put_refresh_random.json::<AppError>();
    assert_eq!(try_put_refresh_random_error.source, Source::App);
    assert_eq!(try_put_refresh_random_error.status, 401);

    // Login with Plain password ("123")
    user.username = String::from("user");
    user.password = String::from("123");
    let plain_password = server.post(&EndPoint::User.base()).json(&user).expect_failure().await;
    assert_eq!(plain_password.status_code(), StatusCode::UNAUTHORIZED);
    let plain_password_error = plain_password.json::<AppError>();
    assert_eq!(plain_password_error.source, Source::PasswordHash);
    assert_eq!(plain_password_error.message.as_str(), "password hash string missing field");

    // Login with Wrong password hash ("123" ended with "Z", not "Y")
    let mut user = UserRequest::demo();
    user.password = String::from("$argon2id$v=19$m=19456,t=2,p=1$MSP7gDlibs2z8LXcRS3O+g$1wgPvzXi6ghGcpn5ZlPN8cfMOk+yvGD7boqgzeDN2ZZ");
    let wrong_hash = server.post(&EndPoint::User.base()).json(&user).expect_failure().await;
    assert_eq!(wrong_hash.status_code(), StatusCode::UNAUTHORIZED);
    let wrong_hash_error = wrong_hash.json::<AppError>();
    assert_eq!(wrong_hash_error.source, Source::PasswordHash);
    assert_eq!(wrong_hash_error.message.as_str(), "invalid Base64 encoding");

    // Login with Wrong password ("123")
    user.password = String::from("$argon2id$v=19$m=19456,t=2,p=1$MSP7gDlibs2z8LXcRS3O+g$1wgPvzXi6ghGcpn5ZlPN8cfMOk+yvGD7boqgzeDN2ZY");
    let wrong_password = server.post(&EndPoint::User.base()).json(&user).expect_failure().await;
    assert_eq!(wrong_password.status_code(), StatusCode::UNAUTHORIZED);
    let wrong_password_error = wrong_password.json::<AppError>();
    assert_eq!(wrong_password_error.source, Source::App);
    assert_eq!(wrong_password_error.message.as_str(), "invalid password");

    // Login Success ("1234")
    user.password = String::from("$argon2id$v=19$m=19456,t=2,p=1$/XFasj8WyfzzGzV2fnouWQ$OfHASwUrzgJmchn9LvM+T7IHtvI//W+BMgBe70jDnqU");
    let login_success = server.post(&EndPoint::User.base()).json(&user).expect_success().await;
    let new_refresh_cookie = login_success.cookie(COOKIE_TOKEN_NAME);
    assert_ne!(new_refresh_cookie.value(), "");

    // Cannot get access token within 9 seconds after created, we need to wait 10 seconds here
    std::thread::sleep(std::time::Duration::from_millis(10000));
    // Use Refresh Token to get Access Token
    let get_refresh_success = server.get(&EndPoint::User.base()).expect_success().await;
    let has_refresh_cookie = get_refresh_success.maybe_cookie(COOKIE_TOKEN_NAME);
    assert!(has_refresh_cookie.is_some());

    // Try PUT with Refresh Token and username is a random state_id
    let mut user = UserRequestFull::demo();
    let try_put_refresh_random_2 = server.put(&EndPoint::User.base()).json(&user).expect_failure().await;
    assert_eq!(try_put_refresh_random_2.status_code(), StatusCode::UNAUTHORIZED);
    let try_put_refresh_random_2_error = try_put_refresh_random_2.json::<AppError>();
    assert_eq!(try_put_refresh_random_2_error.source, Source::App);
    assert_eq!(try_put_refresh_random_2_error.status, 401);

    // Use Refresh Token and state_id to renew Refresh Token
    let last_success_response = get_refresh_success.json::<LoginResponse>();
    let claims = get_claim_public(&last_success_response.token, &state.paseto.public).unwrap();
    user.username = claims.sub;
    // iat is TIMESTAMP(second) so we need to wait 1 second to change it
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let put_refresh_success = server.put(&EndPoint::User.base()).json(&user).expect_success().await;
    let refresh_cookie = put_refresh_success.cookie(COOKIE_TOKEN_NAME);
    let old_iat = get_claim_public(new_refresh_cookie.value(), &state.paseto.public).unwrap().iat;
    let new_iat = get_claim_public(refresh_cookie.value(), &state.paseto.public).unwrap().iat;
    assert_ne!(old_iat, new_iat);

    // Logout
    server.add_header("Authorization", &["bearer ", &last_success_response.token].concat());
    let logout_success = server.delete(&EndPoint::Sse.base()).expect_success().await;
    assert_eq!(logout_success.json::<String>().as_str(), "OK");

    // Try Use Refresh Token to get Access Token after logout
    let get_refresh_failure = server.get(&EndPoint::User.base()).expect_failure().await;
    assert_eq!(get_refresh_failure.status_code(), StatusCode::UNAUTHORIZED);
    let get_refresh_failure_error = get_refresh_failure.json::<AppError>();
    assert_eq!(get_refresh_failure_error.source, Source::App);
    assert_eq!(get_refresh_failure_error.status, 401);

    // Try PUT with Refresh Token and state_id after logout
    let put_refresh_failure = server.put(&EndPoint::User.base()).json(&user).expect_failure().await;
    assert_eq!(put_refresh_failure.status_code(), StatusCode::UNAUTHORIZED);
    let put_refresh_failure_error = put_refresh_failure.json::<AppError>();
    assert_eq!(put_refresh_failure_error.source, Source::App);
    assert_eq!(put_refresh_failure_error.status, 401);
}
