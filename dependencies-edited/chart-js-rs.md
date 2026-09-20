[chart-js-rs](https://github.com/Billy-Sheppard/chart-js-rs)
> last check date = 2026-09-19

- Add zoom plugin
- Add destroy method
- Fix `pointercancel`, `pointermove` and `pointerup` event-listeners not destroy (stacking infinity every render)

## delete `rust-toolchain.toml`

## src/objects/chart_objects.rs

### add struct fields
```rust
pub struct ChartOptions {

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) aspectRatio: NumberString,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) locale: String,

pub struct ChartPlugins {

    // https://github.com/chartjs/chartjs-plugin-zoom
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) zoom: Option<PluginZoom>,

pub struct LineAnnotation {

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) mode: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) scaleID: String,
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) value: NumberString,

pub struct ScaleTime {

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) tooltipFormat: String,

pub struct ToolTipPlugin {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) usePointStyle: Option<bool>,

pub struct ChartScale {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) adapters: Option<ScaleAdapters>,

pub struct XYDataset {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hidden: Option<bool>,
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) rotation: NumberString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) showLine: Option<bool>,
```

### add new structs
```rust

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleAdapters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) date: Option<ScaleAdaptersDate>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScaleAdaptersDate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) outputCalendar: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct PluginZoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pan: Option<ZoomPan>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limits: Option<HashMap<String, ZoomLimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) zoom: Option<ZoomZoom>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomLimit {
    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) min: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) max: NumberString,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) minRange: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomPan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) mode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) modifierKey: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) overScaleMode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) scaleMode: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) threshold: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomZoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) mode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) overScaleMode: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) scaleMode: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) drag: Option<ZoomDragOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pinch: Option<ZoomPinchOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) wheel: Option<ZoomWheelOptions>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomDragOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) backgroundColor: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) borderColor: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) borderWidth: NumberString,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) drawTime: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) modifierKey: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) threshold: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomWheelOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) modifierKey: String,

    #[serde(skip_serializing_if = "NumberString::is_empty")]
    pub(crate) speed: NumberString,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZoomPinchOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) enabled: Option<bool>,
}
```

# src/export.rs

:10 add
```rs
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};
```

:16 add
```rs
static ACTIVE_CHART: LazyLock<Mutex<HashMap<String, JsValue>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

};
```

:130
```diff
    pub fn render_chart(v: JsValue, id: &str, mutate: bool, plugins: String, defaults: String) {
        register_chart_area_background();
        ensure_mt_dpr_watcher();

        if !defaults.is_empty() {
            // Side-effecting block (e.g. `Chart.defaults.*`), in global scope.
            let _ = js_sys::eval(&defaults);
        }
        if !plugins.is_empty() {
            if let Ok(p) = js_sys::eval(&plugins) {
                let _ = Reflect::set(&v, &"plugins".into(), &p);
            }
        }

        // Optional main-thread mutate hook (`window.mutate_chart_object`).
        let obj = if mutate {
            let g = js_sys::global();
            let window = Reflect::get(&g, &"window".into()).unwrap_or_else(|_| g.into());
            match Reflect::get(&window, &"mutate_chart_object".into())
                .ok()
                .and_then(|f| f.dyn_into::<Function>().ok())
            {
                Some(f) => f.call1(&window, &v).unwrap_or(v),
                None => v,
            }
        } else {
            v
        };

        let Some(el) = gloo_utils::document().get_element_by_id(id) else {
            return;
        };
        if let Ok(ctor) = chart_global().dyn_into::<Function>() {
-           // new Chart(el, obj)
-           let _ = Reflect::construct(&ctor, &Array::of2(&el.into(), &obj));
+           // live_chart(id) always return UNDEFINED, so store in HashMap instead
+           if let Ok(mut lock) = ACTIVE_CHART.lock() {
+               if let Some(old) = lock.get(id) {
+                   // chart.destroy()
+                   if let Ok(destroy) = Reflect::get(old, &"destroy".into()).and_then(|jsv| jsv.dyn_into::<Function>()) {
+                       destroy.call0(old).unwrap();
+                   }
+               }

+               // new Chart(el, obj)
+               if let Ok(new) = Reflect::construct(&ctor, &Array::of2(&el.into(), &obj)) {
+                   lock.entry(id.to_owned()).and_modify(|v| *v = new.clone()).or_insert(new);
+               }
+           }
+       }
+   }

+   pub fn destroy(id: &str) {
+       if let Ok(mut lock) = ACTIVE_CHART.lock() {
+           if let Some(old) = lock.remove(id) {
+               // chart.destroy()
+               if let Ok(destroy) = Reflect::get(&old, &"destroy".into()).and_then(|jsv| jsv.dyn_into::<Function>()) {
+                   destroy.call0(&old).unwrap();
+               }
+           }
        }
    }
```

# src/utils.rs
:126 add
```rs
    pub fn destroy(self) {
        destroy(&self.id);
    }
```

# src/objects/helper_object.rs
remove `all` gloo_console::debug!
:456
```diff
        Reflect::set(&js_window, &JsValue::from_str(&id), js_sys_fn).unwrap();
        js_closure.forget();

-       gloo_console::debug!(format!(
-           "Closure at {}:{}:{} set at window.['{id}'].",
-           file!(),
-           line!(),
-           column!()
-       ));
+       // gloo_console::debug!(format!(
+       //     "Closure at {}:{}:{} set at window.['{id}'].",
+       //     file!(),
+       //     line!(),
+       //     column!()
+       // ));
        self.closure_id = Some(id);
        self
```
