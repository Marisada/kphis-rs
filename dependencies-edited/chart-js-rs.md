[chart-js-rs](https://github.com/Billy-Sheppard/chart-js-rs)
> last check date = 2026-06-01

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
