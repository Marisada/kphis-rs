use js_sys::Promise;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, closure::Closure, prelude::wasm_bindgen};
use web_sys::Element;

// https://github.com/bluzky/nice-select2/
// NiceSelect will create new Element for render NiceSelect
// old select element will set height, width, pading, border, opacity to 0
// new() will auto-remove old NiceSelect element
#[wasm_bindgen(raw_module = "/statics/js/nice-select2.js")]
extern "C" {

    // #[wasm_bindgen(js_name = NiceSelect)]
    pub type NiceSelect;

    // // Using as import in webpack
    // // = new NiceSelect(document.getElementById("a-select"), { .. });
    // #[wasm_bindgen(constructor)]
    // pub fn new(_: &Element, _: &JsValue) -> NiceSelect;

    // - NiceSelect.bind(document.getElementById("a-select"), { .. });
    #[wasm_bindgen]
    pub fn bind(_: &Element, _: &JsValue) -> NiceSelect;

    #[wasm_bindgen(method, js_name = renderValue)]
    pub fn render_value(this: &NiceSelect, value: &JsValue);

    #[wasm_bindgen(method)]
    pub fn update(this: &NiceSelect);

    #[wasm_bindgen(method)]
    pub fn focus(this: &NiceSelect);

    #[wasm_bindgen(method)]
    pub fn disable(this: &NiceSelect);

    #[wasm_bindgen(method)]
    pub fn enable(this: &NiceSelect);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &NiceSelect);

    #[wasm_bindgen(method)]
    pub fn clear(this: &NiceSelect);
}

// impl NiceSelect {
//     pub fn new_default(elm: &Element) -> NiceSelect {
//         NiceSelect::new(elm, &NiceSelectOption::default().to_value())
//     }
//     pub fn new_default_with_value(elm: &Element, value: &str) -> NiceSelect {
//         let select = NiceSelect::new(elm, &NiceSelectOption::default().to_value());
//         select.render_value(&JsValue::from_str(value));
//         select
//     }
// }

impl NiceSelect {
    pub fn new_default(elm: &Element) -> NiceSelect {
        bind(elm, &NiceSelectOption::default().to_value())
    }
    pub fn new_default_with_value(elm: &Element, value: &str) -> NiceSelect {
        let select = bind(elm, &NiceSelectOption::default().to_value());
        select.render_value(&JsValue::from_str(value));
        select
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NiceSelectOption {
    pub searchable: bool,
    pub placeholder: String,
    pub searchtext: String,
    pub show_selected_items: bool,
}

impl NiceSelectOption {
    pub fn to_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}

impl Default for NiceSelectOption {
    fn default() -> Self {
        Self {
            searchable: true,
            placeholder: String::from("เลือก"),
            searchtext: String::from("ค้นหา"),
            show_selected_items: true,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type Modal;
    // new Modal [modal = new bootstrap.Modal('id')]
    #[wasm_bindgen(constructor, js_namespace = bootstrap, js_name = "Modal")]
    pub fn new(_: &str) -> Modal;

    // hide Modal [modal.hide()]
    #[wasm_bindgen(method, js_name = hide)]
    pub fn hide(this: &Modal);

    // show Modal [modal.show()]
    #[wasm_bindgen(method, js_name = show)]
    pub fn show(this: &Modal);

    // toggle Modal [modal.toggle()]
    #[wasm_bindgen(method, js_name = toggle)]
    pub fn toggle(this: &Modal);
}

// https://github.com/fabricjs/fabric.js
#[wasm_bindgen]
extern "C" {

    pub type Canvas;
    pub type FabricObject;
    pub type PencilBrush;

    pub type SVGParsingOutput;

    // using the CDN at html file
    // new canvas [canvas = new fabric.Canvas('id', { option })]
    #[wasm_bindgen(constructor, js_namespace = fabric, js_name = "Canvas")]
    pub fn new(_: &str, _: &JsValue) -> Canvas;

    // add object to canvas [canvas.add(object)]
    #[wasm_bindgen(method, js_name = add)]
    pub fn add(this: &Canvas, _: &JsValue);

    // remove object from canvas [canvas.remove(object)]
    #[wasm_bindgen(method)]
    pub fn remove(this: &Canvas, _: &JsValue);

    // clear all objects in canvas [canvas.clear()]
    #[wasm_bindgen(method)]
    pub fn clear(this: &Canvas);

    // render canvas [canvas.renderAll()]
    #[wasm_bindgen(method, js_name = renderAll)]
    pub fn render_all(this: &Canvas);

    // get all objects from canvas [objects = canvas.getObjects()]
    #[wasm_bindgen(method, js_name = getObjects)]
    pub fn get_objects(this: &Canvas) -> Vec<JsValue>;

    // parse from svg string to canvas (in closure)
    // [fabric.loadSVGFromString(svgTag.value, function(objects, options) { })]
    #[wasm_bindgen(js_namespace = fabric, js_name = loadSVGFromString)]
    pub fn load_svg_from_string(_: &str) -> Promise<SVGParsingOutput>;

    #[wasm_bindgen(method, getter, js_name = objects)]
    pub fn get_objects(_: &SVGParsingOutput) -> Vec<JsValue>;

    // parse canvas to svg string [canvas.toSVG()]
    #[wasm_bindgen(method, js_name = toSVG)]
    pub fn to_svg(this: &Canvas) -> String;

    // parse canvas to svg string [canvas.toSVG(option)]
    #[wasm_bindgen(method, js_name = toSVG)]
    pub fn to_svg_with_option(this: &Canvas, _: &JsValue) -> String;

    // add event-listener to canvas
    // [canvas.on('object:added', function(){ })]
    #[wasm_bindgen(method)]
    pub fn on(this: &Canvas, _: &str, _: &Closure<dyn FnMut()>);

    // get canvas object from svg elements
    // [loadedObjects = fabric.util.groupSVGElements(objects)]
    #[wasm_bindgen(js_namespace = ["fabric","util"], js_name = groupSVGElements)]
    pub fn group_svg_elements(_: &[JsValue]) -> FabricObject;

    // set parameter to canvas object [loadedObjects.set('selectable', false)]
    #[wasm_bindgen(method)]
    pub fn set(this: &FabricObject, _: &str, _: bool);

    // `canvas.freeDrawingBrush = new fabric.PencilBrush(canvas);`
    #[wasm_bindgen(constructor, js_namespace = fabric, js_name = "PencilBrush")]
    pub fn new(_: &Canvas) -> PencilBrush;

    // get all objects from canvas [brush = canvas.freeDrawingBrush]
    #[wasm_bindgen(method, getter, js_name = freeDrawingBrush)]
    pub fn get_brush(this: &Canvas) -> PencilBrush;

    // `canvas.freeDrawingBrush = new fabric.PencilBrush(canvas);`
    #[wasm_bindgen(method, setter, js_name = freeDrawingBrush)]
    pub fn set_free_drawing_brush(this: &Canvas, _: &PencilBrush);

    // set width to pencilBrush [brush.width = 2]
    #[wasm_bindgen(method, setter)]
    pub fn set_width(this: &PencilBrush, _: u32);

    // set color to pencilBrush [brush.color = '#000']
    #[wasm_bindgen(method, setter)]
    pub fn set_color(this: &PencilBrush, _: &str);
}

impl Canvas {
    pub fn to_svg_suppress_preamble(&self) -> String {
        Self::to_svg_with_option(self, &TSVGExportOptions::default().to_value())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FabricOption {
    pub is_drawing_mode: bool,
}

impl FabricOption {
    pub fn to_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TSVGExportOptions {
    pub suppress_preamble: bool,
}

impl TSVGExportOptions {
    pub fn to_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}

impl Default for TSVGExportOptions {
    fn default() -> Self {
        Self { suppress_preamble: true }
    }
}

// https://github.com/fengyuanchen/viewerjs
#[wasm_bindgen]
extern "C" {

    pub type Viewer;

    // = new Viewer(document.getElementById("a-select"));
    #[wasm_bindgen(constructor)]
    pub fn new(_: &Element) -> Viewer;

    // = new Viewer(document.getElementById("a-select"), {option}"});
    #[wasm_bindgen(constructor)]
    pub fn new_with_option(_: &Element, _: &JsValue) -> Viewer;

    #[wasm_bindgen(method)]
    pub fn update(this: &Viewer);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Viewer);
}

impl Viewer {
    pub fn new_with_data_url(element: &Element) -> Self {
        Self::new_with_option(element, &ViewerOption::default().to_value())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewerOption {
    pub url: String,
}

impl ViewerOption {
    pub fn to_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}

impl Default for ViewerOption {
    fn default() -> Self {
        Self { url: String::from("data-original") }
    }
}

// https://www.jsdelivr.com/package/npm/zoomist
// https://github.com/cotton123236/zoomist/
#[wasm_bindgen]
extern "C" {

    pub type Zoomist;

    // = new Zoomist("#id");
    #[wasm_bindgen(constructor)]
    pub fn new(_: &str) -> Zoomist;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Zoomist, _: &JsValue);

    // // = new Zoomist("#id", {option}"});
    // #[wasm_bindgen(constructor)]
    // pub fn new_with_option(_: &str, _: &JsValue) -> Zoomist;

    // #[wasm_bindgen(method)]
    // pub fn reset(this: &Zoomist);

    // #[wasm_bindgen(method, js_name = zoomTo)]
    // pub fn zoom_to(this: &Zoomist, _: f32);
}

// #[derive(Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ZoomistOptions {
//     pub min_scale: u32,
//     pub max_scale: u32,
//     pub init_scale: u32,
// }

// impl ZoomistOptions {
//     pub fn to_value(&self) -> JsValue {
//         serde_wasm_bindgen::to_value(self).unwrap()
//     }
// }

// impl Default for ZoomistOptions {
//     fn default() -> Self {
//         Self {
//             min_scale: 1,
//             max_scale: 5,
//             init_scale: 1,
//         }
//     }
// }
