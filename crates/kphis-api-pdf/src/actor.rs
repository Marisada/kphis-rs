use std::sync::mpsc::TryRecvError;

use kphis_api_core::pdf::core::{ActorGetMessage, JsonActor};

use crate::json_data::get_json_data;

/// loop until disconnected or finish `get_json_data().await`<br>
/// call `rt.block_on()` internally
pub fn run_api_actor(mut actor: JsonActor) {
    loop {
        match actor.receiver.try_recv() {
            Ok(msg) => {
                handle_message(&mut actor, msg);
            }
            Err(TryRecvError::Empty) => {
                continue;
            }
            Err(TryRecvError::Disconnected) => {
                break;
            }
        }
    }
}

fn handle_message(actor: &mut JsonActor, msg: ActorGetMessage) {
    let ActorGetMessage { path, user, app, respond_to } = msg;
    actor.rt.block_on(async move {
        let result = get_json_data(&path, &app, &user).await;
        if let Err(_) = respond_to.send(result) {
            tracing::warn!("failed to process Typst's loading {} due to mpsc channel sending-from-actor error", path.to_str().unwrap_or_default());
        }
    });
}
