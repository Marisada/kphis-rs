[rust-dominator](https://github.com/Pauan/rust-dominator)
- window.size() get size without scroll bar
    - use document.documentElement.clientWidth instead of window.innerWidth
    - use document.documentElement.clientHeight instead of window.innerHeight
- add window_offset()
- fix test

add `.cargo/config.toml`
```toml
[target.wasm32-unknown-unknown]
runner = 'wasm-bindgen-test-runner'
```

## Cargo.toml
- add
```toml
[lib]
doctest = false

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

```diff
-   futures-signals = "0.3"
+   futures-signals = { git = "https://codeberg.org/marisada/rust-signals" }
```

## src/dom.rs::234
- edit
```diff
-            let width = window.inner_width().unwrap_throw().as_f64().unwrap_throw();
-            let height = window.inner_height().unwrap_throw().as_f64().unwrap_throw();
+            let document_element = window.document().unwrap_throw().document_element().unwrap_throw();
+            let width = document_element.client_width() as f64;
+            let height = document_element.client_height() as f64;
```

- add
```rust
/// This is returned by the [`window_offset`] function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowOffset {
    pub x: f64,
    pub y: f64,
}

impl WindowOffset {
    fn new() -> Self {
        WINDOW.with(|window| {
            let x = window.scroll_x().unwrap_throw();
            let y = window.scroll_y().unwrap_throw();

            Self { x, y }
        })
    }
}

thread_local! {
    static WINDOW_OFFSET: RefCounter<MutableListener<WindowOffset>> = RefCounter::new();
}

#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
struct WindowOffsetSignal {
    signal: MutableSignal<WindowOffset>,
}

impl Signal for WindowOffsetSignal {
    type Item = WindowOffset;

    #[inline]
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.signal).poll_change(cx)
    }
}

impl Drop for WindowOffsetSignal {
    fn drop(&mut self) {
        WINDOW_OFFSET.with(|size| {
            size.decrement();
        });
    }
}

/// `Signal` which gives the current scrollX / scrollY of the window.
///
/// When the window is scroll, it will automatically update with the new offset.
pub fn window_offset() -> impl Signal<Item = WindowOffset> {
    let signal = WINDOW_OFFSET.with(|offset| {
        let offset = offset.increment(|| {
            let offset = Mutable::new(WindowOffset::new());

            let listener = {
                let offset = offset.clone();

                WINDOW.with(move |window| {
                    on(window, &EventOptions::default(), move |_: crate::events::Scroll| {
                        offset.set_neq(WindowOffset::new());
                    })
                })
            };

            MutableListener::new(offset, listener)
        });

        offset.as_mutable().signal()
    });

    WindowOffsetSignal { signal }
}

```

- edit
```diff

#[cfg(test)]
mod tests {
    use super::{DomBuilder, text_signal, RefFn};
    use crate::{html, shadow_root, ShadowRootMode, with_cfg};
    use futures_signals::signal::{always, SignalExt};
    use once_cell::sync::Lazy;
    use web_sys::HtmlElement;
+   use wasm_bindgen_test::*;

+   // Run with..
+   // cargo install geckodriver
+   // cargo test --target wasm32-unknown-unknown
+   wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

+   #[wasm_bindgen_test]
-   #[test]
    fn apply() {
```
- edit add `#[test]` to `[wasm_bindgen_test]`
- edit `foo` and `bar` in `apply()` and `style_signal_types()` tests to `color` and `red`
- edit `foo` in `attribute_signal_types()` tests to `title`
