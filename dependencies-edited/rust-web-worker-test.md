[rust-web-worker-test](https://github.com/Pauan/rust-web-worker-test)
- Added send console.log from web worker

## src/lib.rs
WorkerCall.send()
```diff
        async move {
            while let Some(message) = messages.next().await {
-                if message.id() == id {
-                    return A::from_js(&message.value());
-                }
+                if message.id() == 0 {
+                    let message = String::from_js(&message.value());
+                    log::debug!("{:?}", &message);
+                    web_sys::window().unwrap_throw().alert_with_message(&message).unwrap_throw();
+                } else if message.id() == id {
+                    return A::from_js(&message.value());
+                }
            }

            panic!("Message call {} failed: stream closed", name);
        }
```