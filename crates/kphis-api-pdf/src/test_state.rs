use kphis_api_core::{pdf::core::JsonActorHandle, state::ApiState};
use sqlx::{MySql, Pool};
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

use crate::actor;

pub async fn new_test_state(db_pool: Pool<MySql>, shutdown_sender: broadcast::Sender<()>) -> ApiState {
    // load config
    let config = config::Config::builder()
        .add_source(config::File::with_name("../../volume/config/test.toml"))
        .build()
        .expect("Error create config from file");
    // spawn JsonActorHandle
    let json_handle = Arc::new(RwLock::new(JsonActorHandle::new(actor::run_api_actor)));
    // create ApiState
    ApiState::new(&config, db_pool, json_handle.clone(), shutdown_sender).await
}
