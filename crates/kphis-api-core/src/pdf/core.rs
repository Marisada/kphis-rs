// idea from https://ryhl.io/blog/actors-with-tokio/
// actor for getting api data (async context) from Typst (sync in async context)
// by seperate dedicate thread + tokio runtime to block_on each request message

use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, TryRecvError},
    thread::JoinHandle,
};
use typst_library::diag::{EcoString, FileError};

use crate::state::{ApiState, UserState};

pub struct JsonActor {
    pub receiver: mpsc::Receiver<ActorGetMessage>,
    pub rt: tokio::runtime::Runtime,
}

impl JsonActor {
    pub fn new(receiver: mpsc::Receiver<ActorGetMessage>) -> Self {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("Failed to build Current-Thread Runtime");
        JsonActor { receiver, rt }
    }
}

pub struct ActorGetMessage {
    pub path: PathBuf,
    pub user: UserState,
    pub app: ApiState,
    pub respond_to: mpsc::Sender<Result<Vec<u8>, FileError>>,
}

pub struct JsonActorHandle {
    sender: Option<mpsc::Sender<ActorGetMessage>>,
    handle: Option<JoinHandle<()>>,
}

impl JsonActorHandle {
    /// - create new `std::sync::mpsc` channel
    /// - `sender`: stored as `Self::sender`
    /// - `reveiver`: stored in `Actor` and passed into `run_fn` call within a spawned thread
    pub fn new<F>(run_fn: F) -> Self
    where
        F: Fn(JsonActor) + std::marker::Send + std::marker::Sync + 'static,
    {
        let (sender, receiver) = mpsc::channel();
        let actor = JsonActor::new(receiver);
        let handle = Some(std::thread::spawn(move || run_fn(actor)));

        Self { sender: Some(sender), handle }
    }

    /// - create new `std::sync::mpsc` channel
    /// - `sender`: stored in `message`, passed `message` into `Self::sender` (for send result back later)
    /// - `receiver`: blocking loop in this function call
    pub fn process_blocking(&self, path: &Path, user: &UserState, app: &ApiState) -> Result<Vec<u8>, FileError> {
        let (send, recv) = mpsc::channel();
        let msg = ActorGetMessage {
            path: path.to_path_buf(),
            user: user.clone(),
            app: app.clone(),
            respond_to: send,
        };

        if let Some(s) = &self.sender {
            if let Err(_e) = s.send(msg) {
                tracing::warn!("failed to process Typst's loading {} due to mpsc channel sending-to-actor error", path.to_str().unwrap_or_default());
            }
        }

        loop {
            match recv.try_recv() {
                Ok(msg) => {
                    break msg;
                }
                Err(TryRecvError::Empty) => {
                    continue;
                }
                Err(TryRecvError::Disconnected) => {
                    break Err(FileError::Other(Some(EcoString::from("data channel disconnected"))));
                }
            }
        }
    }

    pub fn take_join_handle(&mut self) -> Option<JoinHandle<()>> {
        // we need to drop sender to stop run_api_actor loop
        // before joining thread handle
        let _ = self.sender.take();
        self.handle.take()
    }
}
