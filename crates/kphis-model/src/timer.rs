use futures_channel::oneshot;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use wasm_bindgen::{JsCast, closure::Closure};

use crate::app::WINDOW;

#[derive(Debug)]
pub struct Timeout {
    closure: Option<Closure<dyn FnMut()>>,
    id: i32,
}

impl Timeout {
    pub fn new<F>(ms: i32, f: F) -> Self
    where
        F: FnOnce() + 'static,
    {
        let closure = Closure::once(f);

        let id = WINDOW.with(|w| w.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), ms).unwrap());

        Self { closure: Some(closure), id }
    }

    pub fn handle(&self) -> i32 {
        self.id
    }

    pub fn forget(mut self) {
        if let Some(cs) = self.closure.take() {
            cs.forget();
        }
    }

    pub fn manual_drop(handle: i32) {
        WINDOW.with(|w| w.clear_timeout_with_handle(handle));
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        if self.closure.is_some() {
            WINDOW.with(|w| w.clear_timeout_with_handle(self.id));
        }
    }
}

#[derive(Debug)]
pub struct Interval {
    closure: Option<Closure<dyn FnMut()>>,
    id: i32,
}

impl Interval {
    pub fn new<F>(ms: i32, f: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let closure = Closure::new(f);

        let id = WINDOW.with(|w| w.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), ms).unwrap());

        Self { closure: Some(closure), id }
    }

    pub fn handle(&self) -> i32 {
        self.id
    }

    pub fn forget(mut self) {
        if let Some(cs) = self.closure.take() {
            cs.forget();
        }
    }

    pub fn manual_drop(handle: i32) {
        WINDOW.with(|w| w.clear_interval_with_handle(handle));
    }
}

impl Drop for Interval {
    fn drop(&mut self) {
        if self.closure.is_some() {
            WINDOW.with(|w| w.clear_interval_with_handle(self.id));
        }
    }
}

// from gloo_timer
#[derive(Debug)]
#[must_use = "futures do nothing unless polled or spawned"]
pub struct TimeoutFuture {
    _inner: Timeout,
    rx: oneshot::Receiver<()>,
}

impl TimeoutFuture {
    pub fn new(millis: i32) -> TimeoutFuture {
        let (tx, rx) = oneshot::channel();
        let inner = Timeout::new(millis, move || {
            // if the receiver was dropped we do nothing.
            tx.send(()).unwrap();
        });
        TimeoutFuture { _inner: inner, rx }
    }
}

impl Future for TimeoutFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        Future::poll(Pin::new(&mut self.rx), cx).map(|t| t.unwrap())
    }
}
