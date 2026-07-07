use axum_test::TestServer;
use std::net::SocketAddr;

use kphis_api_core::state::ApiState;
use kphis_api_router::new_router;
use kphis_model::{
    endpoint::EndPoint,
    user::his::{LoginResponse, UserRequest},
};

#[allow(dead_code)]
pub async fn new_test_app_login(state: &ApiState) -> TestServer {
    let mut server = new_test_app(state).await;
    let user = login(&server).await;
    server.add_header("Authorization", &["bearer ", &user.token].concat());
    server
}

#[allow(dead_code)]
pub async fn new_test_app(state: &ApiState) -> TestServer {
    let app = new_router(&state);
    TestServer::builder().save_cookies().http_transport().build(app.into_make_service_with_connect_info::<SocketAddr>())
}

#[allow(dead_code)]
pub async fn login(server: &TestServer) -> LoginResponse {
    let response = server.post(&EndPoint::User.base()).json(&UserRequest::demo()).await;

    response.json::<LoginResponse>()
}
