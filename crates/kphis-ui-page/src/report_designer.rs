// model = kphis_model::report
// backend = handlers::pdf

use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use js_sys::JsString;
use std::rc::Rc;
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::{DomParser, HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, Node, Range, SupportedType};

use kphis_model::{
    A4_HEIGHT, A4_WIDTH,
    endpoint::EndPoint,
    fetch::Method,
    report::{CustomReport, ReportParam, ReportQuery, ReportTemplateParams, SystemReport, TypstRaw, TypstSvg, params_and_ids_to_json},
    timer::Timeout,
};
use kphis_ui_app::App;
use kphis_ui_component::modal::report::{param_editor::ReportParamEditor, param_input::ReportParamInput};
use kphis_ui_core::{
    binding::NiceSelect,
    class,
    highlight::{highlight_json, highlight_sql, highlight_stylesheet, highlight_stylesheet_typ, json_pretty},
    mixins,
    pannable::PanState,
    resizable::{Resizable, ResizeState},
};
use kphis_util::{
    error::CONTACT_ADMIN,
    util::{str_some, zoom_step},
};

#[derive(Clone, Default)]
enum DesignerMode {
    System,
    Custom,
    #[default]
    Demo,
}

/// - GET `EndPoint::ReportRawTemplateTypeId`
/// - GET `EndPoint::ReportCustom`
/// - POST `EndPoint::ReportRawQuery` (guarded, remove 'QUERY' btn)
/// - POST `EndPoint::ReportCustom` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::ReportCustom` (guarded, remove delete btn)
#[derive(Clone, Default)]
pub struct ReportDesignerPage {
    // Resizable
    resize_h1: Rc<Resizable>,
    resize_v1: Rc<Resizable>,
    resize_state: Rc<ResizeState>,

    // Pannable
    pan_state: Rc<PanState>,

    system_templates: MutableVec<SystemReport>,
    renew_system_templates_select_box: Mutable<bool>,
    selected_system_template: Mutable<Option<SystemReport>>,

    loaded_custom_templates_compact: Mutable<bool>,
    renew_custom_templates_select_box: Mutable<bool>,
    custom_templates_compact: MutableVec<CustomReport>,
    custom_template_disabled: Mutable<Option<bool>>,
    selected_custom_template_compact: Mutable<String>,
    selected_custom_template: Mutable<Option<CustomReport>>,
    custom_template_changed: Mutable<bool>,

    is_new_custom: Mutable<bool>,
    designer_mode: Mutable<DesignerMode>,
    editor_mode: Mutable<EditorMode>,

    changed: Mutable<bool>,
    render_svg: Mutable<bool>,
    render_pdf: Mutable<bool>,
    load_system_template: Mutable<bool>,
    load_json_from_query: Mutable<bool>,

    custom_template_name: Mutable<String>,
    custom_title: Mutable<String>,
    custom_disabled: Mutable<Option<bool>>,

    custom_params: Mutable<String>,
    custom_params_changed: Mutable<bool>,

    param_inputs: MutableVec<Rc<ReportParamInput>>,
    ids: Mutable<String>,
    ids_changed: Mutable<bool>,

    // typst editor
    typst_text: Mutable<String>,
    typst_html: Mutable<String>,
    set_typst_cursor_position: Mutable<bool>,
    typst_cursor_position: Mutable<Option<u32>>,
    typst_scroll_top: Mutable<i32>,

    // sql editor
    sql_text: Mutable<String>,
    sql_html: Mutable<String>,
    set_sql_cursor_position: Mutable<bool>,
    sql_cursor_position: Mutable<Option<u32>>,
    sql_scroll_top: Mutable<i32>,

    // info editor
    info_text: Mutable<String>,
    info_html: Mutable<String>,
    set_info_cursor_position: Mutable<bool>,
    info_cursor_position: Mutable<Option<u32>>,
    info_scroll_top: Mutable<i32>,

    // json editor
    json_text: Mutable<String>,
    json_html: Mutable<String>,
    set_json_cursor_position: Mutable<bool>,
    json_cursor_position: Mutable<Option<u32>>,
    json_scroll_top: Mutable<i32>,

    report_svg: MutableVec<Rc<TypstSvg>>,
    report_hidden: Mutable<bool>,

    report_width_percent: Mutable<f64>,
    viewer_position_percent: Mutable<f64>,
    viewer_top: Mutable<u32>,
    viewer_top_renew: Mutable<bool>,

    report_param_editor_modal: Mutable<Option<Rc<ReportParamEditor>>>,
    show_report_param_input_modal: Mutable<bool>,
}

impl ReportDesignerPage {
    pub fn new() -> Rc<Self> {
        let modal = Self {
            system_templates: MutableVec::new_with_values(SystemReport::iter().collect()),
            resize_h1: Resizable::new(60.0, false),
            resize_v1: Resizable::new(60.0, true),
            report_svg: MutableVec::new_with_values(vec![Rc::new(TypstSvg::default())]),
            report_width_percent: Mutable::new(100.0),
            ..Default::default()
        };
        modal.set_demo();
        Rc::new(modal)
    }

    fn set_demo(&self) {
        let typst_text = r##"== START TEST #datetime.today().display()
#import "@preview/cetz:0.5.2"
#place(dx: 127.5pt, dy: 320pt, cetz.canvas({
  import cetz.draw: *
  circle((0,0), radius: 6.2, fill: rgb("#FFFDDD"), stroke: none)
}))
#place(bottom + right, image(width: 300pt, "statics/picture/krut.svg"))
#let data = json("data.json")
#assert(data.hosp != none, message: "no 'hosp' data")
#let xhr = data.at("xhr", default: json("api/search/box/hosp-text/" + data.hosp))
#let hospital = text(24pt, if xhr.len() > 0 [#xhr.at(0).hospname] else [ไม่พบสถานบริการ])
#let weather = "windy"
#let forecast(w,hos) = block[
    #box(square(
        width: 2cm,
        inset: 8pt,
        fill: if w == "sunny" { yellow } else { aqua },
        align(bottom + right, strong(w)),
    ))
    #h(6pt)
    #set text(24pt, baseline: -8pt)
    #hos
]
#forecast(weather,hospital)
#set text(18pt)
== เจ้าหน้าที่ ที่มีวันคล้ายวันเกิดในเดือนนี้ ได้แก่
#if data.data.len() > 0 {
    for person in data.data [
        - #person.opduser_name อายุ #person.age ปี
    ]
} else [- ไม่มี]"##;
        let json_text = json_pretty(r#"{"hosp":"11059","data":[]}"#);
        let sql_text = r#"SELECT d.`code`,d.`name` AS doctor_name,d.birth_date,TIMESTAMPDIFF(YEAR, d.birth_date, CURDATE()) AS age,u.`name` AS opduser_name,u.entryposition,c.theme,c.wide_screen,
    (SELECT COUNT(loginname) FROM __HOSXP__.opduser WHERE doctorcode=d.code) AS count_user
FROM `__HOSXP__`.doctor d
    LEFT JOIN __HOSXP__.opduser u ON u.doctorcode = d.code
    LEFT JOIN __KPHIS_EXTRA__.user_config c ON c.loginname=u.loginname
-- Who has a birthday this month ?
WHERE d.active = 'Y' AND d.code <> ? AND MONTH(d.birth_date) = MONTH(CURDATE())
ORDER BY age DESC
LIMIT 50;"#;
        let custom_title = "Test report";
        let custom_params = "hosp^Hospital code^value|code^Doctor code^str";
        let ids = "11059|009";
        let custom_info = r#"วิธีการสร้าง Custom report

ชุดรายงานประกอบด้วย
    1. Template รายงาน เขียนด้วยภาษา Typst: นำข้อมูลจาก data.json มาสร้างรายงาน
    2. ชุดข้อมูล data.json ในรูปแบบ JSON string
    3. คำสั่งค้นฐานข้อมูล MySQL: ส่งไปยัง Server เพื่อค้นข้อมูลและส่งกลับมาเป็น data.json
    4. Parameters: ข้อมูลประกอบการค้นฐานข้อมูลและสร้าง data.json ในรูปแบบ 'label1^title1^type1|label2^title2^(type2,key1,value1,key2,value2,..)|..' โดยใช้ '|' คั่นระหว่าง parameter และใช้ '^' คั่นแยก label, title และ type ออกจากกัน
        4.1 Parameter type ที่มีได้ค่าเดียว (ใช้คู่กับ '?' ใน SQL statement) มี 3 รูปแบบ ได้แก่
            4.1.1 MySQL-type เช่น 'str' (TEXT), 'f32' (FLOAT) เป็นต้น
            4.1.2 List ที่ประกอบด้วยคู่ Value-Label สำหรับเป็นตัวเลือก ในรูปแบบ '(MySQL-type,value1,label1,value2,label2,..)'
            4.1.3 System List ที่มีในโปรแกรม เช่น ตัวเลือก Ward, Inscl เป็นต้น ในรูปแบบ '(system-list-name)'
        4.2 Parameter type ที่มีได้หลายค่า (ใช้คู่กับ 'IN (?)' ใน SQL statement) มี 3 รูปแบบ ได้แก่
            4.2.1 MySQL-type เช่น '[str]' (TEXT), '[f32]' (FLOAT) เป็นต้น
            4.2.2 List ที่ประกอบด้วยคู่ Value-Label สำหรับเป็นตัวเลือก ในรูปแบบ '[(MySQL-type,value1,label1,value2,label2,..)]'
            4.2.3 System List ที่มีในโปรแกรม เช่น ตัวเลือก Ward, Inscl เป็นต้น ในรูปแบบ '[(system-list-name)]'
    5. IDs: ค่าของ parameters ที่ต้องการค้นหา ในรูปแบบ 'id1|id2|..' สำหรับ parameter type ที่มีได้ค่าเดียว หรือ 'id1a,id1b|id2a,id2b,id2c|..' สำหรับ parameter type ที่มีได้หลายค่า
    6. Info: คู่มือการใช้งานรายงาน

ขั้นตอนการดำเนินการของระบบ
    1. แทนที่ค่า '__HOSXP__', '__KPHIS__', '__KPHIS_EXTRA__' และ '__KPHIS_LOG__' ในคำสั่ง MySQL ด้วยชื่อฐานข้อมูลตาม config file
    2. Parameterized queries '?' ด้วยค่า id ตามลำดับจากซ้ายไปขวา เช่น 'label1^title1^date|label2^title2^u32' + 'id1|id2' ระบบจะ parameterized queries '?' ตัวแรกด้วย DATE จาก 'id1' และ '?' ตัวที่ 2 ด้วย INT UNSIGNED จาก 'id2' (ยกเว้น type ชนิด 'value' ที่จะไม่เกี่ยวข้องกับการ query เลย)
       ส่วน Parameter type ที่มีได้หลายค่า ระบบจะเพิ่มจำนวน '?' ให้เท่ากับจำนวนค่าใน id เช่น id '1,2,3' จะได้ SQL statement จาก 'IN (?)' เป็น 'IN (?,?,?)'
    3. Execute MySQL query และได้ query_results ในรูปแบบ Array
    4. สร้าง data.json ชนิด Object ที่ประกอบด้วย
        - 'label: id' จากทุก parameters ทุก type (รวมถึง type 'value' ด้วย)
        - 'data: [query_results]'
        เช่น การค้นหาด้วย MySQL ด้วย parameters 'label1^title1^value|label2^title2^str + 'id1|id2' แล้วไม่พบข้อมูล จะได้ data.json เป็น
        { "label1":"id1", "label2":"id2", "data":[] }
    5. สร้างรายงาน จาก Template และ data.json
    
ขั้นตอนของผู้ใช้งาน
    1. เลือกรายงานที่สร้างไว้แล้ว
    2. กรอก IDs
    3. สั่งระบบให้สร้างรายงาน"#;
        let param_inputs = ReportParam::from_cap_pipe(custom_params)
            .iter()
            .zip(ids.split('|'))
            .map(|(p, i)| ReportParamInput::new_with_value(p, i))
            .collect::<Vec<Rc<ReportParamInput>>>();
        self.typst_html.set_neq(typst_highlight(typst_text).html());
        self.typst_text.set_neq(typst_text.to_owned());
        self.sql_html.set_neq(sql_highlight(sql_text).html());
        self.sql_text.set_neq(sql_text.to_owned());
        self.json_html.set_neq(json_highlight(&json_text).html());
        self.json_text.set_neq(json_text);
        self.info_html.set_neq(no_highlight(custom_info).html());
        self.info_text.set_neq(custom_info.to_owned());
        self.custom_title.set_neq(custom_title.to_owned());
        self.custom_params.set_neq(custom_params.to_owned());
        self.param_inputs.lock_mut().replace_cloned(param_inputs);
        self.ids.set_neq(ids.to_owned());
    }

    fn empty_system(&self) {
        self.selected_system_template.set(None);
        self.empty_inputs();
    }

    fn empty_custom(&self) {
        self.selected_custom_template.set(None);
        self.selected_custom_template_compact.set_neq(String::new());
        self.custom_template_name.set(String::new());
        self.custom_title.set_neq(String::new());
        self.custom_params.set_neq(String::new());
        self.custom_disabled.set_neq(Some(true));
        self.empty_inputs();
    }

    fn empty_inputs(&self) {
        self.typst_html.set_neq(String::from("<code></code>"));
        self.typst_text.set_neq(String::new());
        self.sql_html.set_neq(String::from("<code></code>"));
        self.sql_text.set_neq(String::new());
        self.json_html.set_neq(String::from("<code></code>"));
        self.json_text.set_neq(String::new());
        self.info_html.set_neq(String::from("<code></code>"));
        self.info_text.set_neq(String::new());
        self.param_inputs.lock_mut().clear();
        self.ids.set_neq(String::new());
        self.changed.set_neq(false);
    }

    fn is_old_unselected_custom(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_custom = self.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::Custom)),
            let is_new_custom = self.is_new_custom.signal(),
            let not_selected = self.selected_custom_template_compact.signal_ref(|s| s.is_empty()) =>
            *is_custom && !is_new_custom && *not_selected
        }
    }

    fn is_new_or_selected_custom(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_custom = self.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::Custom)),
            let is_new_custom = self.is_new_custom.signal(),
            let not_selected = self.selected_custom_template_compact.signal_ref(|s| s.is_empty()) =>
            *is_custom && (*is_new_custom || !not_selected)
        }
    }

    fn is_new_or_selected_custom_or_demo(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let designer_mode = self.designer_mode.signal_cloned(),
            let is_new_custom = self.is_new_custom.signal(),
            let not_selected = self.selected_custom_template_compact.signal_ref(|s| s.is_empty()) =>
            matches!(designer_mode, DesignerMode::Demo)
            || (matches!(designer_mode, DesignerMode::Custom) && (*is_new_custom || !not_selected))
        }
    }

    fn load_system_template(page: Rc<Self>, app: Rc<App>) {
        if let (Some(template), Some(ids)) = (page.selected_system_template.get_cloned(), str_some(page.ids.get_cloned())) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ReportRawTemplateTypeId`
                    match TypstRaw::call_api_get(template.template_name(), "system", &ids, app.state()).await {
                        Ok(response) => {
                            let typst_html = typst_highlight(&response.typ).html();
                            page.typst_text.set(response.typ.to_owned());
                            page.typst_html.set(typst_html);
                            let json_pretty = json_pretty(&response.data_json);
                            let json_html = json_highlight(&json_pretty).html();
                            page.json_text.set(json_pretty);
                            page.json_html.set(json_html);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.typst_text.set(e.message.clone());
                            page.typst_html.set(["<code>", &e.message, "</code>"].concat());
                            page.json_text.set(String::new());
                            page.json_html.set(String::from("<code></code>"));
                        }
                    }
                }),
            )
        }
    }

    fn renew_system_templates_select_box(page: Rc<Self>, app: Rc<App>) {
        if let Some(elm) = app.get_id("system_templates") {
            Timeout::new(
                0,
                clone!(page => move || {
                    if let Some(template) = page.selected_system_template.get_cloned() {
                        if page.system_templates.lock_ref().contains(&template) {
                            NiceSelect::new_default_with_value(&elm, template.template_name());
                        } else {
                            page.selected_system_template.set(None);
                            NiceSelect::new_default(&elm);
                        }
                    } else {
                        NiceSelect::new_default(&elm);
                    }
                }),
            )
            .forget();
        }
    }

    fn load_custom_templates_compact(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                let params = ReportTemplateParams {
                    disabled: page.custom_template_disabled.get(),
                    compact: Some(true),
                    ..Default::default()
                };
                // GET `EndPoint::ReportCustom`
                match CustomReport::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.custom_templates_compact.lock_mut();
                        lock.replace_cloned(responses);
                        page.renew_custom_templates_select_box.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                        page.custom_templates_compact.lock_mut().clear();
                    }
                }
            }),
        )
    }

    fn renew_custom_templates_select_box(page: Rc<Self>, app: Rc<App>) {
        if let Some(elm) = app.get_id("custom_templates") {
            Timeout::new(
                0,
                clone!(page => move || {
                    if let Some(template) = str_some(page.selected_custom_template_compact.get_cloned()) {
                        NiceSelect::new_default_with_value(&elm, &template);
                    } else {
                        NiceSelect::new_default(&elm);
                    }
                }),
            )
            .forget();
        }
    }

    fn load_custom_template(page: Rc<Self>, app: Rc<App>) {
        let template_id = page.selected_custom_template_compact.lock_ref().parse::<u32>().ok();
        if template_id.is_some() {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let params = ReportTemplateParams {
                        template_id,
                        ..Default::default()
                    };
                    // GET `EndPoint::ReportCustom`
                    match CustomReport::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            if let Some(template) = responses.first() {
                                page.selected_custom_template.set(Some(template.to_owned()));
                                let typst_html = typst_highlight(&template.content).html();
                                page.typst_text.set(template.content.to_owned());
                                page.typst_html.set(typst_html);
                                let sql_html = template.statement.as_ref().map(|s| sql_highlight(s).html()).unwrap_or(String::from("<code></code>"));
                                page.sql_text.set(template.statement.to_owned().unwrap_or_default());
                                page.sql_html.set(sql_html);
                                page.json_text.set(String::new());
                                page.json_html.set(String::from("<code></code>"));
                                let info_html = template.info.as_ref().map(|s| no_highlight(s).html()).unwrap_or(String::from("<code></code>"));
                                page.info_text.set(template.info.to_owned().unwrap_or_default());
                                page.info_html.set(info_html);
                                page.custom_template_name.set(template.template_name.to_owned());
                                page.custom_title.set(template.title.to_owned());
                                page.custom_disabled.set(template.disabled);
                                let custom_params = template.statement_params.to_owned().unwrap_or_default();
                                let param_inputs = ReportParam::from_cap_pipe(&custom_params);
                                {
                                    let mut lock = page.param_inputs.lock_mut();
                                    lock.clear();
                                    lock.extend(param_inputs.iter().map(ReportParamInput::new));
                                }
                                page.ids.set_neq(String::new());
                                page.custom_params.set(template.statement_params.to_owned().unwrap_or_default());
                            } else {
                                page.empty_custom();
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.empty_custom();
                        }
                    }
                }),
            )
        }
    }

    fn load_json_from_query(page: Rc<Self>, app: Rc<App>) {
        if let Some(statement) = str_some(page.sql_text.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let query = ReportQuery {
                        statement,
                        statement_params: page.custom_params.get_cloned(),
                        ids: page.ids.get_cloned(),
                    };
                    // POST `EndPoint::ReportRawQuery`
                    match query.call_api_post(app.state()).await {
                        Ok(json_string) => {
                            let json_pretty = json_pretty(&json_string);
                            let json_html = json_highlight(&json_pretty).html();
                            page.json_text.set(json_pretty);
                            page.json_html.set(json_html);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.json_text.set(["<code>", &e.message, "</code>"].concat());
                            page.json_html.set(String::from("<code></code>"));
                        }
                    }
                }),
            )
        }
    }

    fn post_custom_report(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let mut saver = CustomReport {
                    template_id: page.selected_custom_template_compact.lock_ref().parse::<u32>().unwrap_or_default(),
                    template_name: page.custom_template_name.get_cloned(),
                    title: page.custom_title.get_cloned(),
                    content: page.typst_text.get_cloned(),
                    statement: str_some(page.sql_text.get_cloned()),
                    statement_params: str_some(page.custom_params.get_cloned()),
                    info: str_some(page.info_text.get_cloned()),
                    disabled: page.custom_disabled.get(),
                    update_username: None,
                    update_datetime: None,
                };
                // POST `EndPoint::ReportCustom`
                match saver.call_api_post(app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, clone!(app => async move {
                            if response.last_insert_id > 0 {
                                saver.template_id = response.last_insert_id as u32;
                                page.is_new_custom.set(false);
                            }
                            page.selected_custom_template.set(Some(saver));
                            page.selected_custom_template_compact.set_neq(response.last_insert_id.to_string());
                            page.loaded_custom_templates_compact.set(false);
                            page.changed.set_neq(false);
                            app.alert("บันทึกสำเร็จ", "ระบบได้บันทึก Template รายงานเรียบร้อยแล้ว");
                        })).await;
                    }
                    Err(e) => {
                        if e.status == 400 {
                            app.alert("ข้อมูลที่ส่งไป ไม่ถูกต้อง", "ชื่อ Template นี้ถูกใช้แล้ว กรุณาเปลี่ยน แล้วลองใหม่อีกครั้ง");
                        } else {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    fn delete_custom_report(page: Rc<Self>, app: Rc<App>) {
        let template_id = page.selected_custom_template_compact.lock_ref().parse::<u32>().ok();
        if template_id.is_some() {
            app.async_load(
                true,
                clone!(app => async move {
                    if app.confirm("ยืนยันลบรายการ").await {
                        let params = ReportTemplateParams {
                            template_id,
                            ..Default::default()
                        };
                        // DELETE `EndPoint::ReportCustom`
                        match CustomReport::call_api_delete(&params, app.state()).await {
                            Ok(response) => {
                                app.alert_execute_response(&response, async move {
                                    page.empty_custom();
                                    page.loaded_custom_templates_compact.set(false);
                                }).await;
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            )
        }
    }

    fn render_svg(page: Rc<Self>, app: Rc<App>) {
        let template = page.typst_text.get_cloned();
        let data = page.json_text.get_cloned();
        if !template.trim_start().is_empty() && !data.trim_start().is_empty() {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let bytes = app.typst_worker().await.svg(
                        template,
                        data,
                        app.token().unwrap_or_default(),
                    ).await;
                    let reports = if bytes.is_empty() {
                        vec![TypstSvg::default()]
                    } else {
                        match bitcode::decode(&bytes) {
                            Ok(pages) => pages,
                            Err(e) => {
                                app.alert_error_with_clipboard(CONTACT_ADMIN, &["BitcodeError: ", &e.to_string()].concat()).await;
                                vec![TypstSvg::default()]
                            }
                        }
                    };
                    {
                        let mut lock = page.report_svg.lock_mut();
                        lock.clear();
                        lock.extend(reports.into_iter().map(Rc::new));
                    }
                    page.set_fit(app);
                    page.viewer_position_percent.set(0.0);
                    page.set_viewer_offset_and_inner_height();
                }),
            );
        }
    }

    fn render_pdf(page: Rc<Self>, app: Rc<App>) {
        let template = page.typst_text.get_cloned();
        let data = page.json_text.get_cloned();
        let author = app.app_status.lock_ref().as_ref().map(|status| status.hospital_name.clone()).unwrap_or_default();
        let user = app.user.lock_ref().as_ref().map(|user| user.user.name.get_cloned()).unwrap_or_default();
        let ids = page.ids.lock_ref();
        let (file_name, title) = match page.designer_mode.get_cloned() {
            DesignerMode::Custom => page
                .selected_custom_template
                .lock_ref()
                .as_ref()
                .map(|selected| (selected.download_file_name(&ids), selected.title_with_ids(&ids))),
            DesignerMode::System => page
                .selected_system_template
                .lock_ref()
                .as_ref()
                .map(|selected| (selected.download_file_name(&ids), selected.title_with_ids(&ids))),
            DesignerMode::Demo => None,
        }
        .unwrap_or((String::from("CUSTOM-REPORT"), String::from("Custom Report")));

        app.async_load(
            true,
            clone!(app => async move {
                let bytes = app.typst_worker().await.pdf(template, data, title, author, user, app.token().unwrap_or_default()).await;
                if !bytes.is_empty() {
                    app.open_file_with_mime(&bytes, &file_name, "application/pdf");
                }
            }),
        );
    }

    fn set_fit(&self, app: Rc<App>) {
        if let Some(viewer) = app.get_id("designer-viewer") {
            let viewer_width = viewer.client_width() as f64;
            let reports_width = self.report_svg.lock_ref().iter().max_by_key(|i| i.width as u64).map(|i| i.width).unwrap_or(A4_WIDTH);
            let percent = (viewer_width * 100.0) / reports_width;
            self.report_width_percent.set(percent);
        }
    }

    fn set_viewer_offset_and_inner_height(&self) {
        let percent = self.report_width_percent.get();
        let viewer_position_percent = self.viewer_position_percent.get();

        let report_count = self.report_svg.lock_ref().len();
        let reports_raw_height = self.report_svg.lock_ref().iter().map(|i| i.height).sum::<f64>();
        let reports_exact_height = if reports_raw_height > 0.0 { reports_raw_height } else { A4_HEIGHT };
        let gaps = ((report_count - 1) * 32) as f64;
        let reports_adjusted_height = (reports_exact_height * percent / 100.0) - gaps;

        self.viewer_top.set((reports_adjusted_height * viewer_position_percent / 100.0) as u32);
        self.viewer_top_renew.set(true);
    }

    fn set_viewer_position_percent(&self, app: Rc<App>) {
        if let Some(elm) = app.get_id("designer-viewer-gut") {
            let gut = elm.dyn_into::<HtmlElement>().unwrap();
            let content_height = gut.scroll_height() as u32;
            let scroll_top = gut.scroll_top() as u32;
            let content_position_percent = if content_height > 0 { scroll_top as f64 / content_height as f64 * 100.0 } else { 0.0 };
            self.viewer_position_percent.set(content_position_percent);
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Report Designer");

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::renew_system_templates_select_box(page.clone(), app.clone());
                }
                async{}
            })))
            .future(page.renew_system_templates_select_box.signal().for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::renew_system_templates_select_box(page.clone(), app.clone());
                    page.renew_system_templates_select_box.set(false);
                }
                async{}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let load = page.loaded_custom_templates_compact.signal() =>
                !busy && !load
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_custom_templates_compact(page.clone(), app.clone());
                    page.loaded_custom_templates_compact.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.custom_template_changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_custom_template(page.clone(), app.clone());
                    page.custom_template_changed.set(false);
                    page.report_param_editor_modal.set(None);
                    page.show_report_param_input_modal.set_neq(false);
                }
                async {}
            })))
            .future(page.renew_custom_templates_select_box.signal().for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::renew_custom_templates_select_box(page.clone(), app.clone());
                    page.renew_custom_templates_select_box.set(false);
                }
                async{}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let load = page.load_system_template.signal() =>
                !busy && *load
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_system_template(page.clone(), app.clone());
                    page.load_system_template.set(false);
                }
                async {}
            })))
            // generate param_inputs from page.custom_params and page.ids
            .future(page.custom_params_changed.signal().for_each(clone!(page => move |changed| {
                if changed {
                    let params = ReportParam::from_cap_pipe(&page.custom_params.lock_ref());
                    let raw_ids = page.ids.get_cloned();
                    let mut ids = raw_ids.split('|').collect::<Vec<&str>>();
                    if params.len() > ids.len() {
                        ids.extend(vec![""; params.len() - ids.len()]);
                    }
                    let inputs = params.iter().zip(ids.into_iter()).map(|(p, v)| ReportParamInput::new_with_value(p, v)).collect::<Vec<Rc<ReportParamInput>>>();
                    page.param_inputs.lock_mut().replace_cloned(inputs);
                    page.custom_params_changed.set(false);
                }
                async {}
            })))
            // regenerate page.ids from input form page.param_inputs changed
            .future(page.ids_changed.signal().for_each(clone!(page => move |changed| {
                if changed {
                    let ids = page.param_inputs.lock_ref().iter().map(|param| param.to_request_id()).collect::<Vec<String>>().join("|");
                    page.ids.set_neq(ids);
                    page.ids_changed.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let load = page.load_json_from_query.signal() =>
                !busy && *load
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_json_from_query(page.clone(), app.clone());
                    page.load_json_from_query.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let render = page.render_svg.signal() =>
                !busy && *render
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::render_svg(page.clone(), app.clone());
                    page.render_svg.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let render = page.render_pdf.signal() =>
                !busy && *render
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::render_pdf(page.clone(), app.clone());
                    page.render_pdf.set(false);
                }
                async {}
            })))
            .future(page.resize_h1.prev_percent.signal().for_each(clone!(page => move |prev_percent| {
                page.report_hidden.set_neq(prev_percent >= 100.0);
                async {}
            })))
            .global_event(clone!(page => move |e: events::MouseMove| {
                Resizable::on_mouse_move(&e, page.resize_state.clone());
                PanState::on_mouse_move(&e, page.pan_state.clone());
            }))
            .global_event(clone!(page => move |_: events::MouseUp| {
                Resizable::on_mouse_up(page.resize_state.clone());
                PanState::on_mouse_up(page.pan_state.clone());
            }))
            .style("width", "100%")
            .style("height", "calc(100vh - 105px)")
            .children([
                html!("style", {.text(&highlight_stylesheet_typ())}),
                html!("style", {.text(&highlight_stylesheet())}),
                html!("div", {
                    .class("m-2")
                    .child(html!("div", {
                        .class(class::ROW_AUTO_LG_G2_CT)
                        .child(html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_BLUEO)
                                        .class_signal("active", page.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::System)))
                                        .attr("data-bs-toggle", "button")
                                        .text("System")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.designer_mode.set(DesignerMode::System);
                                            page.editor_mode.set_neq(EditorMode::Typst);
                                            page.empty_system();
                                            page.report_param_editor_modal.set(None);
                                            page.show_report_param_input_modal.set_neq(false);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_BLUEO)
                                        .class_signal("active", page.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::Custom)))
                                        .attr("data-bs-toggle", "button")
                                        .text("Custom")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.designer_mode.set(DesignerMode::Custom);
                                            page.editor_mode.set_neq(EditorMode::Info);
                                            page.empty_custom();
                                            page.loaded_custom_templates_compact.set(false);
                                            page.report_param_editor_modal.set(None);
                                            page.show_report_param_input_modal.set_neq(false);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_BLUEO)
                                        .class_signal("active", page.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::Demo)))
                                        .attr("data-bs-toggle", "button")
                                        .text("Demo")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.designer_mode.set(DesignerMode::Demo);
                                            page.editor_mode.set_neq(EditorMode::Info);
                                            page.set_demo();
                                            page.report_param_editor_modal.set(None);
                                            page.show_report_param_input_modal.set_neq(false);
                                        }))
                                    }),
                                ])
                            }))
                        }))
                        .child_signal(page.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::Custom)).map(clone!(page => move |is_custom| {
                            is_custom.then(|| {
                                html!("div", {
                                    .class("col-12")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", page.is_new_custom.signal())
                                                .attr("data-bs-toggle", "button")
                                                .text("New")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.empty_custom();
                                                    page.custom_template_disabled.set_neq(Some(true));
                                                    page.is_new_custom.set(true);
                                                    page.report_param_editor_modal.set(None);
                                                    page.show_report_param_input_modal.set_neq(false);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_BLUEO)
                                                .class_signal("active", not(page.is_new_custom.signal()))
                                                .attr("data-bs-toggle", "button")
                                                .text("Edit")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.custom_template_disabled.set_neq(None);
                                                    page.loaded_custom_templates_compact.set(false);
                                                    page.is_new_custom.set(false);
                                                    page.report_param_editor_modal.set(None);
                                                    page.show_report_param_input_modal.set_neq(false);
                                                }))
                                            }),
                                        ])
                                    }))
                                })
                            })
                        })))
                        .child_signal(page.is_old_unselected_custom().map(clone!(page => move |is_old_unselected_custom| {
                            is_old_unselected_custom.then(|| {
                                html!("div", {
                                    .class("col-12")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_BLUEO)
                                                .class_signal("active", page.custom_template_disabled.signal().map(|opt| opt.is_none()))
                                                .attr("data-bs-toggle", "button")
                                                .text("All")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.custom_template_disabled.set(None);
                                                    page.loaded_custom_templates_compact.set(false);
                                                    page.report_param_editor_modal.set(None);
                                                    page.show_report_param_input_modal.set_neq(false);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_BLUEO)
                                                .class_signal("active", page.custom_template_disabled.signal().map(|opt| opt == Some(false)))
                                                .attr("data-bs-toggle", "button")
                                                .text("Enabled")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.custom_template_disabled.set(Some(false));
                                                    page.loaded_custom_templates_compact.set(false);
                                                    page.report_param_editor_modal.set(None);
                                                    page.show_report_param_input_modal.set_neq(false);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_BLUEO)
                                                .class_signal("active", page.custom_template_disabled.signal().map(|opt| opt == Some(true)))
                                                .attr("data-bs-toggle", "button")
                                                .text("Disabled")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.custom_template_disabled.set(Some(true));
                                                    page.loaded_custom_templates_compact.set(false);
                                                    page.report_param_editor_modal.set(None);
                                                    page.show_report_param_input_modal.set_neq(false);
                                                }))
                                            }),
                                        ])
                                    }))
                                })
                            })
                        })))
                        .child_signal(page.designer_mode.signal_cloned().map(clone!(app, page => move |designer_mode| {
                            match designer_mode {
                                DesignerMode::System => {
                                    page.renew_system_templates_select_box.set(true);
                                    Some(html!("div", {
                                        .class("col-12")
                                        .child(html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .style("min-width","350px")
                                            .children([
                                                html!("label", {
                                                    .class(class::INPUT_GROUP_TEXT_BG_GOLDS)
                                                    .text("Template")
                                                }),
                                                html!("div", {
                                                    .class(class::FLEX_W100)
                                                    .child(html!("select" => HtmlSelectElement, {
                                                        .class(class::FORM_CTRL_SM)
                                                        .attr("id", "system_templates")
                                                        .child(html!("option", {.attr("value","").text("เลือกรายงาน")}))
                                                        .children_signal_vec(page.system_templates.signal_vec_cloned().map(|template| {
                                                            html!("option", {
                                                                .attr("value", template.template_name())
                                                                .text(template.title())
                                                            })
                                                        }))
                                                        .prop_signal("value", page.selected_system_template.signal_cloned().map(|opt| opt.as_ref().map(|selected| selected.template_name().to_owned()).unwrap_or_default()))
                                                        .with_node!(element => {
                                                            .event(clone!(page => move |_: events::Change| {
                                                                let report_opt = SystemReport::new(&element.value());
                                                                if let Some(report) = report_opt.as_ref() {
                                                                    let params = ReportParam::from_cap_pipe(report.key_param()).iter().map(ReportParamInput::new).collect::<Vec<Rc<ReportParamInput>>>();
                                                                    page.param_inputs.lock_mut().replace_cloned(params);
                                                                }
                                                                page.ids.set_neq(String::new());
                                                                page.selected_system_template.set(report_opt);
                                                                page.report_param_editor_modal.set(None);
                                                                page.show_report_param_input_modal.set_neq(false);
                                                            }))
                                                        })
                                                    }))
                                                }),
                                            ])
                                        }))
                                    }))
                                }
                                DesignerMode::Custom => {
                                    Some(html!("div", {
                                        .class("col-12")
                                        .child(html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .style("min-width","350px")
                                            .child(html!("label", {
                                                .class("input-group-text")
                                                .class_signal("bg-warning-subtle", not(page.is_new_custom.signal()))
                                                .class_signal("bg-danger-subtle", map_ref!{
                                                    let is_new_custom = page.is_new_custom.signal(),
                                                    let designer_mode_name_empty = page.custom_template_name.signal_ref(|s| s.is_empty()) =>
                                                    *is_new_custom && *designer_mode_name_empty
                                                })
                                                .text("Template")
                                            }))
                                            .child_signal(page.is_new_custom.signal().map(clone!(app, page => move |is_new_custom| {
                                                if is_new_custom {
                                                    Some(html!("input" => HtmlInputElement, {
                                                        .class("form-control")
                                                        .attr("placeholder","ระบุ ชื่อ Template")
                                                        .prop_signal("value", page.custom_template_name.signal_cloned())
                                                        .with_node!(element => {
                                                            .event(clone!(app, page => move |_: events::Change| {
                                                                let value = element.value();
                                                                let is_neq = page.custom_template_name.lock_ref().as_str() != value.as_str();
                                                                if is_neq {
                                                                    if page.custom_templates_compact.lock_ref().iter().any(|template| template.template_name == value) {
                                                                        app.alert("เตือน", "ชื่อ Template นี้ ถูกใช้แล้ว กรุณาเปลี่ยนใหม่");
                                                                        page.custom_template_name.set_neq(String::new());
                                                                    } else {
                                                                        page.custom_template_name.set(value);
                                                                        page.changed.set_neq(true);
                                                                    }
                                                                }
                                                            }))
                                                        })
                                                    }))
                                                } else {
                                                    Some(html!("div", {
                                                        .class(class::FLEX_W100)
                                                        .child(html!("select" => HtmlSelectElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", "custom_templates")
                                                            .children_signal_vec(page.custom_templates_compact.signal_vec_cloned().map(|template| {
                                                                html!("option", {
                                                                    .attr("value", &template.template_id.to_string())
                                                                    .text(&template.template_name)
                                                                })
                                                            }))
                                                            .apply(mixins::string_value_select(page.selected_custom_template_compact.clone(), page.custom_template_changed.clone()))
                                                        }))
                                                    }))
                                                }
                                            })))
                                            .child_signal(page.selected_custom_template_compact.signal_ref(|s| !s.is_empty()).map(clone!(page => move |is_selected| {
                                                is_selected.then(|| {
                                                    html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_GRAY)
                                                        .child(html!("i", {.class(class::FA_X)}))
                                                        .event(clone!(page => move |_: events::Click| {
                                                            page.empty_custom();
                                                        }))
                                                    })
                                                })
                                            })))
                                        }))
                                    }))
                                }
                                DesignerMode::Demo => None,
                            }
                        })))
                        .child_signal(page.is_new_or_selected_custom_or_demo().map(clone!(page => move |is_new_or_selected_custom_or_demo| {
                            is_new_or_selected_custom_or_demo.then(|| {
                                html!("div", {
                                    .class("col-12")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .class("btn")
                                                .class_signal("btn-outline-primary", page.typst_text.signal_ref(|s| !s.trim_start().is_empty()))
                                                .class_signal("btn-outline-danger", page.typst_text.signal_ref(|s| s.trim_start().is_empty()))
                                                .class_signal("active", page.editor_mode.signal_cloned().map(|editor_mode| matches!(editor_mode, EditorMode::Typst)))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("Typst")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.editor_mode.set_neq(EditorMode::Typst);
                                                }))
                                            }),
                                            html!("button", {
                                                .class("btn")
                                                .class_signal("btn-outline-primary", page.sql_text.signal_ref(|s| !s.trim_start().is_empty()))
                                                .class_signal("btn-outline-danger", page.sql_text.signal_ref(|s| s.trim_start().is_empty()))
                                                .class_signal("active", page.editor_mode.signal_cloned().map(|editor_mode| matches!(editor_mode, EditorMode::MySql)))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("MySQL")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.editor_mode.set_neq(EditorMode::MySql);
                                                }))
                                            }),
                                            html!("button", {
                                                .class("btn")
                                                .class_signal("btn-outline-primary", page.info_text.signal_ref(|s| !s.trim_start().is_empty()))
                                                .class_signal("btn-outline-danger", page.info_text.signal_ref(|s| s.trim_start().is_empty()))
                                                .class_signal("active", page.editor_mode.signal_cloned().map(|editor_mode| matches!(editor_mode, EditorMode::Info)))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("Info")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.editor_mode.set_neq(EditorMode::Info);
                                                }))
                                            }),
                                        ])
                                    }))
                                })
                            })
                        })))
                        .child_signal(page.is_new_or_selected_custom().map(clone!(page => move |is_new_or_selected_custom| {
                            is_new_or_selected_custom.then(|| {
                                html!("div", {
                                    .class("col-12")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .style("min-width","300px")
                                        .children([
                                            html!("label", {
                                                .class("input-group-text")
                                                .class_signal("bg-danger-subtle", page.custom_title.signal_ref(|s| s.is_empty()))
                                                .text("Title")
                                            }),
                                            html!("input" => HtmlInputElement, {
                                                .class("form-control")
                                                .attr("placeholder","ระบุ ชื่อรายงาน")
                                                .apply(mixins::string_value(page.custom_title.clone(), page.changed.clone()))
                                            }),
                                        ])
                                    }))
                                })
                            })
                        })))
                        .child_signal(page.is_new_or_selected_custom().map(clone!(page => move |is_new_or_selected_custom| {
                            is_new_or_selected_custom.then(|| {
                                html!("div", {
                                    .class("col-12")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_GREENO)
                                                .class_signal("active", page.custom_disabled.signal().map(|opt| opt.is_none() || opt == Some(false)))
                                                .attr("data-bs-toggle", "button")
                                                .text("Enable")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.custom_disabled.set(Some(false));
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_REDO)
                                                .class_signal("active", page.custom_disabled.signal().map(|opt| opt == Some(true)))
                                                .attr("data-bs-toggle", "button")
                                                .text("Disable")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.custom_disabled.set(Some(true));
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                })
                            })
                        })))
                        .child_signal(page.is_new_or_selected_custom_or_demo().map(clone!(app, page => move |is_new_or_selected_custom_or_demo| {
                            is_new_or_selected_custom_or_demo.then(|| {
                                html!("div", {
                                    .class("col-12")
                                    .child(html!("button", {
                                        .attr("type","button")
                                        .class(class::BTN_SM_CYAN)
                                        .text("Params")
                                        .event(clone!(app, page => move |_:events::Click| {
                                            page.show_report_param_input_modal.set_neq(false);
                                            page.report_param_editor_modal.set(Some(ReportParamEditor::new(page.custom_params.clone(), page.custom_params_changed.clone(), app.clone())));
                                        }))
                                    }))
                                })
                            })
                        })))
                        .child(html!("div", {
                            .class("col-12")
                            .child(html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_GOLD)
                                .text("Values")
                                .event(clone!(app, page => move |_:events::Click| {
                                    page.report_param_editor_modal.set(None);
                                    page.show_report_param_input_modal.set_neq(true);
                                }))
                            }))
                        }))
                        .child_signal(page.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::System)).map(clone!(app, page => move |is_system| {
                            is_system.then(|| {
                                html!("div", {
                                    .class("col-12")
                                    .child(html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_BLUE)
                                        .text("LOAD")
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(page => move || {
                                            page.editor_mode.set_neq(EditorMode::Typst);
                                            // clean only-custom-template values
                                            page.selected_custom_template_compact.set_neq(String::new());
                                            page.sql_html.set_neq(String::from("<code></code>"));
                                            page.sql_text.set_neq(String::new());
                                            page.info_html.set_neq(String::from("<code></code>"));
                                            page.info_text.set_neq(String::new());
                                            page.custom_template_name.set(String::new());
                                            page.custom_title.set_neq(String::new());
                                            page.custom_params.set_neq(String::new());
                                            page.changed.set_neq(false);
                                            // signal to load
                                            page.load_system_template.set_neq(true);
                                        }), map_ref!{
                                            let is_selected = page.selected_system_template.signal_ref(|opt| opt.is_some()),
                                            let all_id = page.param_inputs.signal_vec_cloned().filter_signal_cloned(|s| s.is_empty_signal()).is_empty() =>
                                            !is_selected || !all_id
                                        }, app.state()))
                                    }))
                                })
                            })
                        })))
                        .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::ReportRawQuery, false), |dom| dom
                            .child_signal(page.editor_mode.signal_cloned().map(clone!(app, page => move |editor_mode| {
                                matches!(editor_mode, EditorMode::MySql).then(|| {
                                    html!("div", {
                                        .class("col-12")
                                        .child(html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_BLUE)
                                            .text("QUERY")
                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(page => move || {
                                                if page.sql_text.lock_ref().trim_start().is_empty() {
                                                    let ids = page.ids.lock_ref();
                                                    let json_text = params_and_ids_to_json(&page.custom_params.lock_ref(), &ids);
                                                    let json_pretty = json_pretty(&json_text);
                                                    let json_html = json_highlight(&json_pretty).html();
                                                    page.json_text.set(json_pretty);
                                                    page.json_html.set(json_html);
                                                } else {
                                                    page.load_json_from_query.set_neq(true);
                                                }
                                            }), map_ref!{
                                                let param_len = page.custom_params.signal_ref(|s| s.split("|").count()),
                                                let ids_len = page.param_inputs.signal_vec_cloned().filter_signal_cloned(|s| not(s.is_empty_signal())).len() =>
                                                param_len != ids_len
                                            }, app.state()))
                                        }))
                                    })
                                })
                            })))
                        )
                        .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::ReportCustom, false), |dom| dom
                            .child_signal(map_ref!{
                                let is_custom = page.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::Custom)),
                                let no_template_name = page.custom_template_name.signal_ref(|s| s.is_empty()),
                                let no_title = page.custom_title.signal_ref(|s| s.is_empty()),
                                let no_typst = page.typst_text.signal_ref(|s| s.trim_start().is_empty()) =>
                                *is_custom && !no_template_name && !no_title && !no_typst
                            }.map(clone!(app, page => move |ready| {
                                ready.then(|| {
                                    html!("div", {
                                        .class("col-12")
                                        .child(html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_BLUE)
                                            .child(html!("i", {.class(class::FA_SAVE)}))
                                            .text(" Save")
                                            .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                Self::post_custom_report(page.clone(), app.clone());
                                            }), not(page.changed.signal()), app.state()))
                                        }))
                                    })
                                })
                            })))
                        )
                        .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::ReportCustom, false), |dom| dom
                            .child_signal(map_ref!{
                                let is_custom = page.designer_mode.signal_ref(|dm| matches!(dm, DesignerMode::Custom)),
                                let not_selected = page.selected_custom_template_compact.signal_ref(|s| s.is_empty()) =>
                                *is_custom && !not_selected
                            }.map(clone!(app, page => move |ready| {
                                (ready).then(|| {
                                    html!("div", {
                                        .class("col-12")
                                        .child(html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_TRASH)}))
                                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                Self::delete_custom_report(page.clone(), app.clone());
                                            }), app.state()))
                                        }))
                                    })
                                })
                            })))
                        )
                    }))
                }),
                html!("div", {
                    .style("position", "fixed")
                    .style("bottom", "20px")
                    .style("right", "40px")
                    .style("z-index", "1")
                    .child(html!("div", {
                        .class("d-flex")
                        .children([
                            html!("div", {
                                .class("me-2")
                                .style("font-size","24px")
                                .visible_signal(app.loader_is_loading())
                                .child(html!("i", {.class(class::FA_SPIN)}))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_L_GRAY)
                                .child(html!("i", {.class(class::FA_ARROW_LR)}))
                                .event(clone!(app, page => move |_:events::Click| {
                                    page.set_fit(app.clone());
                                    page.set_viewer_position_percent(app.clone());
                                    page.set_viewer_offset_and_inner_height();
                                }))
                            }),
                            html!("div", {
                                .class(class::INPUT_GROUP)
                                .style("max-width","170px")
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_GRAY)
                                        .child(html!("i", {.class(class::FA_MINUS)}))
                                        .event(clone!(app, page => move |_:events::Click| {
                                            let zoom = page.report_width_percent.get();
                                            page.report_width_percent.set(zoom_step(zoom, false));
                                            page.set_viewer_position_percent(app.clone());
                                            page.set_viewer_offset_and_inner_height();
                                        }))
                                    }),
                                    html!("input" => HtmlInputElement, {
                                        .class(class::FORM_CTRL_C)
                                        .prop_signal("value", page.report_width_percent.signal_cloned().map(|u| [&u.round().to_string(), "%"].concat()))
                                        .with_node!(element => {
                                            .event(clone!(app, page => move |_:events::Change| {
                                                if let Ok(value) = element.value().trim_end_matches('%').parse::<f64>() {
                                                    page.report_width_percent.set(value);
                                                    page.set_viewer_position_percent(app.clone());
                                                    page.set_viewer_offset_and_inner_height();
                                                }
                                            }))
                                        })
                                        // .text_signal(page.report_width_percent.signal_cloned().map(|u| [&u.round().to_string(), "%"].concat()))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(app, page => move |_:events::Click| {
                                            let zoom = page.report_width_percent.get();
                                            page.report_width_percent.set(zoom_step(zoom, true));
                                            page.set_viewer_position_percent(app.clone());
                                            page.set_viewer_offset_and_inner_height();
                                        }))
                                    }),
                                ])
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_R_BLUE)
                                .text("RENDER")
                                .event(clone!(page => move |_:events::Click| {
                                    page.render_svg.set(true);
                                    page.report_param_editor_modal.set(None);
                                    page.show_report_param_input_modal.set_neq(false);
                                }))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_R_BLUE)
                                .text("PDF")
                                .event(clone!(page => move |_:events::Click| {
                                    page.render_pdf.set(true);
                                }))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_R_GRAY)
                                .child_signal(page.report_hidden.signal_cloned().map(|is_hidden| {
                                    if is_hidden {
                                        Some(html!("i", {.class(class::FA_COL2)}))
                                    } else {
                                        Some(html!("i", {.class(class::FA_COL1)}))
                                    }
                                }))
                                .event(clone!(page => move |_:events::Click| {
                                    page.report_hidden.set(!page.report_hidden.get());
                                    let new_percent = if page.report_hidden.get() {100.0} else {65.0};
                                    page.resize_h1.set_prev_percent(new_percent);
                                }))
                            }),
                        ])
                    }))
                }),
                html!("div", {
                    .class("resizer-container")
                    .children([
                        // left panel
                        html!("div", {
                            .class("resizer-container-vertical")
                            .apply(Resizable::prev_mixin(page.resize_h1.clone()))
                            .child_signal(page.editor_mode.signal_cloned().map(clone!(app, page => move |editor_mode| {
                                match editor_mode {
                                    EditorMode::Typst => {
                                        Some(html!("div", {
                                            .apply(Resizable::prev_mixin(page.resize_v1.clone()))
                                            .class("editor-container")
                                            .children([
                                                html!("div", {
                                                    .class("editor-label")
                                                    .style("pointer-events","none")
                                                    .text("Typst ")
                                                    .text(app.typst_version())
                                                }),
                                                loc(page.typst_text.signal_cloned(), page.typst_scroll_top.signal()),
                                                editor(
                                                    page.typst_text.clone(),
                                                    page.typst_html.clone(),
                                                    page.changed.clone(),
                                                    page.set_typst_cursor_position.clone(),
                                                    page.typst_cursor_position.clone(),
                                                    page.typst_scroll_top.clone(),
                                                    app.clone(),
                                                    typst_highlight,
                                                ),
                                            ])
                                        }))
                                    }
                                    EditorMode::MySql => {
                                        Some(html!("div", {
                                            .apply(Resizable::prev_mixin(page.resize_v1.clone()))
                                            .class("editor-container")
                                            .children([
                                                html!("div", {
                                                    .class("editor-label")
                                                    .style("pointer-events","none")
                                                    .text("MySQL")
                                                }),
                                                loc(page.sql_text.signal_cloned(), page.sql_scroll_top.signal()),
                                                editor(
                                                    page.sql_text.clone(),
                                                    page.sql_html.clone(),
                                                    page.changed.clone(),
                                                    page.set_sql_cursor_position.clone(),
                                                    page.sql_cursor_position.clone(),
                                                    page.sql_scroll_top.clone(),
                                                    app.clone(),
                                                    sql_highlight,
                                                ),
                                            ])
                                        }))
                                    }
                                    EditorMode::Info => {
                                        Some(html!("div", {
                                            .apply(Resizable::prev_mixin(page.resize_v1.clone()))
                                            .class("editor-container")
                                            .children([
                                                html!("div", {
                                                    .class("editor-label")
                                                    .style("pointer-events","none")
                                                    .text("Info")
                                                }),
                                                loc(page.info_text.signal_cloned(), page.info_scroll_top.signal()),
                                                editor(
                                                    page.info_text.clone(),
                                                    page.info_html.clone(),
                                                    page.changed.clone(),
                                                    page.set_info_cursor_position.clone(),
                                                    page.info_cursor_position.clone(),
                                                    page.info_scroll_top.clone(),
                                                    app.clone(),
                                                    no_highlight,
                                                ),
                                            ])
                                        }))
                                    }
                                }
                            })))
                            .children([
                                // vertical resizer
                                Resizable::render(page.resize_v1.clone(), page.resize_state.clone()),
                                // json editor
                                html!("div", {
                                    .apply(Resizable::next_mixin(page.resize_v1.clone()))
                                    .class("editor-container")
                                    .children([
                                        html!("div", {
                                            .class("editor-label")
                                            .text("data.json")
                                        }),
                                        loc(page.json_text.signal_cloned(), page.json_scroll_top.signal()),
                                        editor(
                                            page.json_text.clone(),
                                            page.json_html.clone(),
                                            Mutable::new(true),
                                            page.set_json_cursor_position.clone(),
                                            page.json_cursor_position.clone(),
                                            page.json_scroll_top.clone(),
                                            app.clone(),
                                            json_highlight,
                                        ),
                                    ])
                                }),
                            ])
                        }),
                        // horizontal resizer
                        Resizable::render(page.resize_h1.clone(), page.resize_state.clone()),
                        // right panel
                        html!("div", {
                            .attr("id", "designer-viewer")
                            .apply(Resizable::next_mixin(page.resize_h1.clone()))
                            .child(html!("div", {
                                .attr("id", "designer-viewer-gut")
                                .style("background-color","#eee")
                                .apply(PanState::pan_container_mixins(page.pan_state.clone()))
                                .child(html!("div", {
                                    .apply(mixins::typst_svg_mixins(page.report_width_percent.clone(), page.report_svg.clone()))
                                }))
                                .future(page.viewer_top_renew.signal().for_each(clone!(app, page => move |set_top| {
                                    if set_top {
                                        page.viewer_top_renew.set(false);
                                        if let Some(gut) = app.get_id("designer-viewer-gut") {
                                            gut.set_scroll_top(page.viewer_top.get() as i32);
                                        }
                                    }
                                    async {}
                                })))
                            }))
                        }),
                    ])
                }),
            ])
            .child_signal(page.report_param_editor_modal.signal_cloned().map(clone!(app, page => move |opt| {
                opt.map(|modal| ReportParamEditor::render(modal, page.report_param_editor_modal.clone(), app.clone()))
            })))
            .child_signal(page.show_report_param_input_modal.signal().map(clone!(app, page => move |show| {
                show.then(|| {
                    Self::render_param_input_modal(page.clone(), app.clone())
                })
            })))
        })
    }

    fn render_param_input_modal(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .style("position","fixed")
            .style("right","25px")
            .style("top","115px")
            .style("width","730px")
            .style("background-color","var(--bs-body-bg)")
            .class(class::BOX_ROUND)
            .children([
                html!("div", {
                    .children([
                        html!("span", {
                            .class(class::BOLD_FS5_R)
                            .text("Values")
                        }),
                        html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_FR_R)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(page => move |_:events::Click| {
                                page.show_report_param_input_modal.set(false);
                            }))
                        }),
                    ])
                }),
                html!("hr", {.class("my-2")}),
                html!("div", {
                    .class("mb-2")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class("form-control")
                        .style("font-family","Consolas, monospace, serif")
                        .style("color","var(--bs-body-bg)")
                        .style("background-color","var(--bs-body-color)")
                        .prop_signal("value", page.ids.signal_cloned())
                        .with_node!(element => {
                            .style_signal("height", page.ids.signal_ref(clone!(element => move |_| {
                                element.style().set_property("height", "auto").unwrap();
                                let min_height = 40 + ((element.rows() - 1) * 24) as i32;
                                let scroll_height = element.scroll_height();
                                let height = if scroll_height < min_height {min_height} else {scroll_height + 2};
                                [&height.to_string(), "px"].concat()
                            })))
                            .event(clone!(app, page => move |_: events::Input| {
                                let value = element.value();
                                let neq = page.ids.lock_ref().as_str() != value;
                                if neq {
                                    page.ids.set(value);
                                    page.custom_params_changed.set(true);
                                }
                            }))
                        })
                    }))
                }),
                html!("div", {
                    .class("mb-2")
                    .style("height","500px")
                    .style("overflow-y","auto")
                    .child(html!("div", {
                         .children_signal_vec(page.param_inputs.signal_vec_cloned().map(clone!(app, page => move |param| {
                            ReportParamInput::render(param.clone(), page.ids_changed.clone(), app.clone())
                        })))
                    }))
                }),
                html!("div", {
                    .child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_FR_GRAY)
                        .text("ปิด")
                        .event(clone!(page => move |_:events::Click| {
                            page.show_report_param_input_modal.set(false);
                        }))
                    }))
                }),
            ])
        })
    }
}

#[derive(Clone, Default, PartialEq)]
enum EditorMode {
    Typst,
    MySql,
    #[default]
    Info,
}

fn create_range(node: Node, target_position: u32, app: Rc<App>) -> Range {
    let range = app.create_range();
    match range.select_node(&node) {
        Err(e) => {
            app.show_jsvalue_message(&e);
            range
        }
        Ok(()) => {
            let mut pos = 0;
            match new_range(&node, target_position, &mut pos, &range) {
                Some(new_range) => new_range,
                None => {
                    let node_offset = node.child_nodes().length();
                    match range.set_start(&node, node_offset) {
                        Err(e) => {
                            app.show_jsvalue_message(&e);
                        }
                        Ok(()) => {
                            if let Err(e) = range.set_end(&node, node_offset) {
                                app.show_jsvalue_message(&e);
                            }
                        }
                    }
                    range
                }
            }
        }
    }
}

fn new_range(current: &Node, target_position: u32, pos: &mut u32, range: &Range) -> Option<Range> {
    if current.node_type() == 3 {
        // Node.TEXT_NODE (3) (https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)
        let text_content = current.text_content();
        let len = text_content.map(|text| text.chars().count()).unwrap_or_default() as u32;
        if *pos + len >= target_position {
            let offset = target_position.saturating_sub(*pos);
            match range.set_start(current, offset) {
                Err(e) => {
                    let message = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("cannot set_start"));
                    log::error!("{}", message);
                }
                Ok(()) => {
                    if let Err(e) = range.set_end(current, offset) {
                        let message = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("cannot set_end"));
                        log::error!("{}", message);
                    }
                }
            }
            Some(range.clone())
        } else {
            *pos += len;
            None
        }
    } else if current.has_child_nodes() {
        let children = current.child_nodes();
        for i in 0..children.length() {
            if *pos > target_position {
                break;
            } else {
                let result = children.get(i).and_then(|child| new_range(&child, target_position, pos, range));
                if result.is_some() {
                    return result;
                } else {
                    continue;
                }
            }
        }
        None
    } else {
        None
    }
}

fn loc<T, S>(text_signal: T, scroll_top_signal: S) -> Dom
where
    T: Signal<Item = String> + 'static,
    S: Signal<Item = i32> + 'static,
{
    html!("div", {
        .class("editor-loc-container")
        .child(html!("div", {
            .class("editor-loc")
            .children_signal_vec(text_signal.map(|text| {
                (1..(text.lines().count() + 1)).collect::<Vec<usize>>()
            }).to_signal_vec().map(|u| {
                html!("div", {
                    .class("text-end")
                    .text(&u.to_string())
                })
            }))
            .with_node!(element => {
                .future(scroll_top_signal.for_each(move |top| {
                    element.set_scroll_top(top);
                    async {}
                }))
            })
        }))
    })
}

fn editor<F: Fn(&str) -> HighLighted + 'static>(
    text: Mutable<String>,
    html: Mutable<String>,
    changed: Mutable<bool>,
    set_cursor_position: Mutable<bool>,
    cursor_position: Mutable<Option<u32>>,
    scroll_top: Mutable<i32>,
    app: Rc<App>,
    highlight_fn: F,
) -> Dom {
    html!("pre", {
        .class("editor-pre")
        .attr("autocapitalize","off")
        .attr("autocorrect","off")
        .attr("contenteditable","true")
        .attr("spellcheck","false")
        .attr("translate","no")
        .with_node!(element => {
            // after KeyUp #1 set innerHTML
            // we use DomParser instead of innerHTML to prevent injection
            .future(html.signal_cloned().for_each(clone!(element, set_cursor_position => move |html| {
                let doc = DomParser::new().unwrap().parse_from_string(&html, SupportedType::TextHtml).unwrap();
                if let Some(child) = doc.body().unwrap().first_element_child() {
                    element.replace_children_with_node_1(&child);
                    set_cursor_position.set(true);
                }
                async {}
            })))
            // after KeyUp #2 set cursor position
            .future(set_cursor_position.signal().for_each(clone!(app, element, cursor_position => move |set| {
                if set {
                    // set position
                    if let Some(pos) = cursor_position.get() {
                        let new_range = create_range(element.clone().into(), pos, app.clone());
                        if let Some(new_selection) = app.get_selection() {
                            match new_selection.remove_all_ranges() {
                                Err(e) => app.show_jsvalue_message(&e),
                                Ok(()) => if let Err(e) = new_selection.add_range(&new_range) {
                                    app.show_jsvalue_message(&e);
                                }
                            }
                        }
                    }
                    set_cursor_position.set(false);
                }
                async {}
            })))
            .event_with_options(&EventOptions::preventable(), clone!(app, element => move |event:events::KeyUp| {
                // if event.ctrl_key() && !event.shift_key() && &event.key() == "v" { // key a up after press ctrl + v
                //     app.alert_error("คำแนะนำ", "กรุณาใช้ (Ctrl + Shift + v) แทน (Ctrl + v) ในการ Paste ข้อมูลปริมาณมาก");
                // } else
                if event.ctrl_key() && &event.key() == "a" { // key a up after press ctrl + a
                    return
                } else if event.key() == "Control" { // when key ctrl up alone
                    return
                }
                // get position
                let pos = app.get_selection().and_then(|old_selection| {
                    if let Ok(range) = old_selection.get_range_at(0) {
                        let range_cloned = range.clone_range();
                        if let Err(e) = range_cloned.select_node_contents(&element) {
                            app.show_jsvalue_message(&e);
                            None
                        } else if let (Ok(end_node), Ok(end_offset)) = (range.end_container(), range.end_offset()) {
                            if let Err(e) = range_cloned.set_end(&end_node, end_offset) {
                                app.show_jsvalue_message(&e);
                                None
                            } else {
                                Some(range_cloned.to_string().length())
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });
                // render
                let inner_text = element.inner_text();
                let highlighted = highlight_fn(&inner_text);
                html.set(highlighted.html());
                text.set(inner_text);
                changed.set_neq(true);
                if event.key() == "Enter" {
                    // event.prevent_default();
                    cursor_position.set_neq(pos.map(|p| highlighted.pos_changed(p) + 1));
                } else {
                    cursor_position.set_neq(pos.map(|p| highlighted.pos_changed(p)));
                }
            }))
            .event_with_options(&EventOptions::preventable(), clone!(app => move |event:events::KeyDown| {
                match event.key().as_str() {
                    "Tab" => {
                        event.prevent_default();
                        add_text("    ", &app);
                    }
                    // "\"" => {
                    //     event.prevent_default();
                    //     add_scope("\"", "\"", &app);
                    // }
                    // "[" => {
                    //     event.prevent_default();
                    //     add_scope("[", "]", &app);
                    // }
                    // "{" => {
                    //     event.prevent_default();
                    //     add_scope("{", "}", &app);
                    // }
                    // "(" => {
                    //     event.prevent_default();
                    //     add_scope("(", ")", &app);
                    // }
                    _ => {}
                }
            }))
            .event(move |_:events::Scroll| {
                scroll_top.set(element.scroll_top());
            })
        })
    })
}

fn add_text(text: &str, app: &Rc<App>) {
    if let Some(selection) = app.get_selection() {
        if let Ok(range) = selection.get_range_at(0) {
            let tab_node = app.create_text_node(text);
            match range.insert_node(&tab_node) {
                Err(e) => {
                    app.show_jsvalue_message(&e);
                }
                Ok(()) => match range.set_start_after(&tab_node) {
                    Err(e) => {
                        app.show_jsvalue_message(&e);
                    }
                    Ok(()) => match range.set_end_after(&tab_node) {
                        Err(e) => {
                            app.show_jsvalue_message(&e);
                        }
                        Ok(()) => match selection.remove_all_ranges() {
                            Err(e) => {
                                app.show_jsvalue_message(&e);
                            }
                            Ok(()) => {
                                if let Err(e) = selection.add_range(&range) {
                                    app.show_jsvalue_message(&e);
                                }
                            }
                        },
                    },
                },
            }
        }
    }
}

// fn add_scope(open: &str, close: &str, app: &Rc<App>) {
//     if let Some(selection) = app.get_selection() {
//         if let Ok(range) = selection.get_range_at(0) {
//             let open_node = app.create_text_node(open);
//             let close_node = app.create_text_node(close);
//             match range.insert_node(&close_node) {
//                 Err(e) => {
//                     // insert at start of the range
//                     app.show_jsvalue_message(&e);
//                 }
//                 Ok(()) => {
//                     match range.insert_node(&open_node) {
//                         Err(e) => {
//                             // insert at start of the range
//                             app.show_jsvalue_message(&e);
//                         }
//                         Ok(()) => match range.set_start(&close_node, 0) {
//                             Err(e) => {
//                                 app.show_jsvalue_message(&e);
//                             }
//                             Ok(()) => match range.set_end(&close_node, 0) {
//                                 Err(e) => {
//                                     app.show_jsvalue_message(&e);
//                                 }
//                                 Ok(()) => match selection.remove_all_ranges() {
//                                     Err(e) => {
//                                         app.show_jsvalue_message(&e);
//                                     }
//                                     Ok(()) => {
//                                         if let Err(e) = selection.add_range(&range) {
//                                             app.show_jsvalue_message(&e);
//                                         }
//                                     }
//                                 },
//                             },
//                         },
//                     }
//                 }
//             }
//         }
//     }
// }

struct HighLighted {
    pub html: String,
    pub new_chars: usize,
    pub old_chars: usize,
    pub new_lines: usize,
    pub old_lines: usize,
}

impl HighLighted {
    fn html(&self) -> String {
        self.html.clone()
    }
    fn pos_changed(&self, pos: u32) -> u32 {
        // log::debug!("new_chars={}, old_chars={}, new_lines={}, old_lines={}", self.new_chars, self.old_chars, self.new_lines, self.old_lines);
        let chars_diff = self.new_chars.saturating_sub(self.old_chars);
        let lines_diff = self.new_lines.saturating_sub(self.old_lines);
        let diff = if chars_diff > 0 && lines_diff > 1 { (chars_diff / lines_diff).saturating_sub(1) } else { 0 };
        pos + (chars_diff.saturating_sub(diff) as u32)
    }
}

fn typst_highlight(code: &str) -> HighLighted {
    let original_chars = code.chars().count();
    let original_lines = code.lines().count();
    let root = typst_syntax::parse(code);
    HighLighted {
        html: typst_syntax::highlight_html(&root),
        new_chars: original_chars,
        old_chars: original_chars,
        new_lines: original_lines,
        old_lines: original_lines,
    }
}

fn sql_highlight(code: &str) -> HighLighted {
    let original_chars = code.chars().count();
    let original_lines = code.lines().count();
    HighLighted {
        html: highlight_sql(code),
        // html: ["<code>", &highlight_sql(code), "</code>"].concat(),
        new_chars: original_chars,
        old_chars: original_chars,
        new_lines: original_lines,
        old_lines: original_lines,
    }
}

fn json_highlight(code: &str) -> HighLighted {
    let original_chars = code.chars().count();
    let original_lines = code.lines().count();
    HighLighted {
        html: highlight_json(code),
        // html: ["<code>", &highlight_json(code), "</code>"].concat(),
        new_chars: original_chars,
        old_chars: original_chars,
        new_lines: original_lines,
        old_lines: original_lines,
    }
}

fn no_highlight(code: &str) -> HighLighted {
    let original_chars = code.chars().count();
    let original_lines = code.lines().count();
    HighLighted {
        html: ["<code>", code, "</code>"].concat(),
        new_chars: original_chars,
        old_chars: original_chars,
        new_lines: original_lines,
        old_lines: original_lines,
    }
}
