use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
    signal_vec::SignalVecExt,
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    ipd::dc_plan_tmp::{DcPlanTmpDiet, DcPlanTmpDx, DcPlanTmpEnv, DcPlanTmpMed, DcPlanTmpParams, DcPlanTmpTx},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::util::str_some;

/// - GET `EndPoint::IpdDcPlanTmpDx`
/// - GET `EndPoint::IpdDcPlanTmpMed`
/// - GET `EndPoint::IpdDcPlanTmpEnv`
/// - GET `EndPoint::IpdDcPlanTmpTx`
/// - GET `EndPoint::IpdDcPlanTmpDiet`
/// - POST `EndPoint::IpdDcPlanTmpDx` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdDcPlanTmpDx` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdDcPlanTmpMed` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdDcPlanTmpMed` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdDcPlanTmpEnv` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdDcPlanTmpEnv` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdDcPlanTmpTx` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdDcPlanTmpTx` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdDcPlanTmpDiet` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdDcPlanTmpDiet` (guarded, remove 'Delete' btn)
#[derive(Clone, Default)]
pub struct SettingTemplateDcPlanPage {
    loaded_dxs: Mutable<bool>,
    dxs: Mutable<Vec<DcPlanTmpDx>>,
    dx_changed: Mutable<bool>,
    dx_select_redraw: Mutable<bool>,
    dx_id: Mutable<String>,
    dx_name: Mutable<String>,
    dx_knowledge: Mutable<String>,
    dx_revisit: Mutable<String>,
    dx_prevention: Mutable<String>,

    loaded_meds: Mutable<bool>,
    meds: Mutable<Vec<DcPlanTmpMed>>,
    med_changed: Mutable<bool>,
    med_id: Mutable<String>,
    med_text: Mutable<String>,

    loaded_envs: Mutable<bool>,
    envs: Mutable<Vec<DcPlanTmpEnv>>,
    env_changed: Mutable<bool>,
    env_id: Mutable<String>,
    env_text: Mutable<String>,

    loaded_txs: Mutable<bool>,
    txs: Mutable<Vec<DcPlanTmpTx>>,
    tx_changed: Mutable<bool>,
    tx_id: Mutable<String>,
    tx_text: Mutable<String>,

    loaded_diets: Mutable<bool>,
    diets: Mutable<Vec<DcPlanTmpDiet>>,
    diet_changed: Mutable<bool>,
    diet_id: Mutable<String>,
    diet_text: Mutable<String>,
}

impl SettingTemplateDcPlanPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn load_dx(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdDcPlanTmpDx`
                match DcPlanTmpDx::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        page.dxs.set(responses);
                        page.dx_select_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn save_dx(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let dx = DcPlanTmpDx {
                    dx_id: page.dx_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    dx_name: str_some(page.dx_name.lock_ref().trim().to_owned()),
                    dx_knowledge: str_some(page.dx_knowledge.lock_ref().trim().to_owned()),
                    dx_revisit: str_some(page.dx_revisit.lock_ref().trim().to_owned()),
                    dx_prevention: str_some(page.dx_prevention.lock_ref().trim().to_owned()),
                };
                // POST `EndPoint::IpdDcPlanTmpDx`
                match dx.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.dx_id.set(last_insert_id.to_string());
                            }
                            page.loaded_dxs.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_dx(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = DcPlanTmpParams {
                    id: page.dx_id.lock_ref().parse::<u32>().ok(),
                };
                // DELETE `EndPoint::IpdDcPlanTmpDx`
                match DcPlanTmpDx::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.dx_id.set(String::new());
                            page.loaded_dxs.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_med(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdDcPlanTmpMed`
                match DcPlanTmpMed::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        page.meds.set(responses);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn save_med(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let med = DcPlanTmpMed {
                    med_id: page.med_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    med_text: str_some(page.med_text.lock_ref().trim().to_owned()),
                };
                // POST `EndPoint::IpdDcPlanTmpMed`
                match med.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.med_id.set(last_insert_id.to_string());
                            }
                            page.loaded_meds.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_med(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = DcPlanTmpParams {
                    id: page.med_id.lock_ref().parse::<u32>().ok(),
                };
                // DELETE `EndPoint::IpdDcPlanTmpMed`
                match DcPlanTmpMed::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.med_id.set(String::new());
                            page.loaded_meds.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_env(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdDcPlanTmpEnv`
                match DcPlanTmpEnv::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        page.envs.set(responses);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn save_env(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let env = DcPlanTmpEnv {
                    env_id: page.env_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    env_text: str_some(page.env_text.lock_ref().trim().to_owned()),
                };
                // POST `EndPoint::IpdDcPlanTmpEnv`
                match env.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.env_id.set(last_insert_id.to_string());
                            }
                            page.loaded_envs.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_env(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = DcPlanTmpParams {
                    id: page.env_id.lock_ref().parse::<u32>().ok(),
                };
                // DELETE `EndPoint::IpdDcPlanTmpEnv`
                match DcPlanTmpEnv::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.env_id.set(String::new());
                            page.loaded_envs.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_tx(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdDcPlanTmpTx`
                match DcPlanTmpTx::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        page.txs.set(responses);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn save_tx(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let tx = DcPlanTmpTx {
                    tx_id: page.tx_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    tx_text: str_some(page.tx_text.lock_ref().trim().to_owned()),
                };
                // POST `EndPoint::IpdDcPlanTmpTx`
                match tx.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.tx_id.set(last_insert_id.to_string());
                            }
                            page.loaded_txs.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_tx(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = DcPlanTmpParams {
                    id: page.tx_id.lock_ref().parse::<u32>().ok(),
                };
                // DELETE `EndPoint::IpdDcPlanTmpTx`
                match DcPlanTmpTx::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.tx_id.set(String::new());
                            page.loaded_txs.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_diet(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdDcPlanTmpDiet`
                match DcPlanTmpDiet::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        page.diets.set(responses);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn save_diet(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let diet = DcPlanTmpDiet {
                    diet_id: page.diet_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    diet_text: str_some(page.diet_text.lock_ref().trim().to_owned()),
                };
                // POST `EndPoint::IpdDcPlanTmpDiet`
                match diet.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.diet_id.set(last_insert_id.to_string());
                            }
                            page.loaded_diets.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_diet(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = DcPlanTmpParams {
                    id: page.diet_id.lock_ref().parse::<u32>().ok(),
                };
                // DELETE `EndPoint::IpdDcPlanTmpDiet`
                match DcPlanTmpDiet::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.diet_id.set(String::new());
                            page.loaded_diets.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Setting Template Discharge Plan");

        html!("div", {
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_dxs.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_dx(page.clone(), app.clone());
                    page.loaded_dxs.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_meds.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_med(page.clone(), app.clone());
                    page.loaded_meds.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_envs.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_env(page.clone(), app.clone());
                    page.loaded_envs.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_txs.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_tx(page.clone(), app.clone());
                    page.loaded_txs.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_diets.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_diet(page.clone(), app.clone());
                    page.loaded_diets.set(true);
                }
                async {}
            })))
            .future(page.dx_select_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    if let Some(elm) = app.get_id("search_temp_dx") {
                        NiceSelect::new_default_with_value(&elm, &page.dx_id.lock_ref());
                    }
                    page.dx_select_redraw.set(false);
                }
                async {}
            })))
            .class("container-fluid")
            .child(html!("div", {
                .children([
                    html!("div", {
                        .class("row")
                        .child(html!("h4", {
                            .child(html!("label", {
                                .class(class::FORM_COL_LBL_AUTO)
                                .child(html!("i", {.class(class::FA_USER_COG)}))
                                .text(" Template Nursing Discharge Plan บันทึก/แก้ไข")
                            }))
                        }))
                    }),
                    Self::render_dx(page.clone(), app.clone()),
                    Self::render_med(page.clone(), app.clone()),
                    Self::render_env(page.clone(), app.clone()),
                    Self::render_tx(page.clone(), app.clone()),
                    Self::render_diet(page.clone(), app.clone()),
                ])
            }))
        })
    }

    fn render_dx(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_CYANS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" D: Diagnosis")
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("col-sm-12")
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempDxModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.dx_id.set_neq(String::new());
                                                    page.dx_name.set_neq(String::new());
                                                    page.dx_knowledge.set_neq(String::new());
                                                    page.dx_revisit.set_neq(String::new());
                                                    page.dx_prevention.set_neq(String::new());
                                                    page.dx_changed.set_neq(false);
                                                    page.dx_select_redraw.set(true);
                                                }))
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempDxModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempDxModal")
                                    .child(html!("div", {
                                        .class(class::MODAL_DIALOG_C)
                                        .child(html!("div", {
                                            .class("modal-content")
                                            .children([
                                                html!("div", {
                                                    .class("modal-header")
                                                    .children([
                                                        html!("h5", {
                                                            .class("modal-title")
                                                            .child(html!("i", {.class(class::FA_USER_COG)}))
                                                            .text(" จัดการ Template D: Diagnosis")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .children([
                                                        html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "dx_name")
                                                                    .text("ชื่อโรคที่เจ็บป่วย")
                                                                }),
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "text")
                                                                    .attr("maxlength", "250")
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "dx_name")
                                                                    .apply(mixins::string_value_not_empty(page.dx_name.clone(), page.dx_changed.clone()))
                                                                }),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "dx_knowledge")
                                                                    .text("ความรู้เกี่ยวกับโรค")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "dx_knowledge")
                                                                    .apply(mixins::textarea_value_auto_expand(page.dx_knowledge.clone(), page.dx_changed.clone()))
                                                                }),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "dx_revisit")
                                                                    .text("T Treatment: อาการเร่งด่วนที่ต้องมา รพ.")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "dx_revisit")
                                                                    .apply(mixins::textarea_value_auto_expand(page.dx_revisit.clone(), page.dx_changed.clone()))
                                                                }),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "dx_prevention")
                                                                    .text("H Health: การฟื้นฟูสภาพร่างกาย, การป้องกันภาวะแทรกซ้อน")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "dx_prevention")
                                                                    .apply(mixins::textarea_value_auto_expand(page.dx_prevention.clone(), page.dx_changed.clone()))
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.dx_id.signal_cloned().map(clone!(app, page => move |dx_id| {
                                                        (!dx_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdDcPlanTmpDx, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_dx(page.clone(), app.clone());
                                                                }), app.state()))
                                                            })
                                                        )
                                                    })))
                                                    .child_signal(page.dx_id.signal_cloned().map(clone!(app, page => move |dx_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdDcPlanTmpDx, false) &&
                                                        if dx_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.dx_changed.signal())
                                                                .class_signal("btn-secondary", not(page.dx_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_dx(page.clone(), app.clone());
                                                                }), not(page.dx_changed.signal()), app.state()))
                                                            })
                                                        })
                                                    })))
                                                    .child(doms::close_modal_btn())
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                                html!("table", {
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("ชื่อโรคที่เจ็บป่วย")}),
                                                    html!("th", {.attr("scope", "col").text("ความรู้เกี่ยวกับโรค")}),
                                                    html!("th", {.attr("scope", "col").text("T: อาการเร่งด่วนที่ต้องมา รพ.")}),
                                                    html!("th", {.attr("scope", "col").text("H: การฟื้นฟู/ป้องกันภาวะแทรกซ้อน")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(page.dxs.signal_cloned().to_signal_vec().map(clone!(page => move |dx| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.class("text-truncate").style("max-width","230px").text(&dx.dx_name.clone().unwrap_or_default())}),
                                                        html!("td", {.class("text-truncate").style("max-width","calc((100vw - 440px) / 3").text(&dx.dx_knowledge.clone().unwrap_or_default())}),
                                                        html!("td", {.class("text-truncate").style("max-width","calc((100vw - 440px) / 3").text(&dx.dx_revisit.clone().unwrap_or_default())}),
                                                        html!("td", {.class("text-truncate").style("max-width","calc((100vw - 440px) / 3").text(&dx.dx_prevention.clone().unwrap_or_default())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempDxModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.dx_id.set_neq(dx.dx_id.to_string());
                                                                        page.dx_name.set_neq(dx.dx_name.clone().unwrap_or_default());
                                                                        page.dx_knowledge.set_neq(dx.dx_knowledge.clone().unwrap_or_default());
                                                                        page.dx_revisit.set_neq(dx.dx_revisit.clone().unwrap_or_default());
                                                                        page.dx_prevention.set_neq(dx.dx_prevention.clone().unwrap_or_default());
                                                                        page.dx_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                })
                                            })))
                                        }),
                                    ])
                                }),
                            ])
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_med(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_LIGHTS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" M: Medication")
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("col-sm-12")
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempMedModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.med_id.set_neq(String::new());
                                                    page.med_text.set_neq(String::new());
                                                    page.med_changed.set_neq(false);
                                                }))
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempMedModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempMedModal")
                                    .child(html!("div", {
                                        .class(class::MODAL_DIALOG_C)
                                        .child(html!("div", {
                                            .class("modal-content")
                                            .children([
                                                html!("div", {
                                                    .class("modal-header")
                                                    .children([
                                                        html!("h5", {
                                                            .class("modal-title")
                                                            .child(html!("i", {.class(class::FA_USER_COG)}))
                                                            .text(" จัดการ M: Medication")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "med_text")
                                                                    .text("ยาและวิธีใช้ยาที่ได้รับกลับบ้าน")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "med_text")
                                                                    .apply(mixins::textarea_value_auto_expand(page.med_text.clone(), page.med_changed.clone()))
                                                                }),
                                                            ])
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.med_id.signal_cloned().map(clone!(app, page => move |med_id| {
                                                        (!med_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdDcPlanTmpMed, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_med(page.clone(), app.clone());
                                                                }), app.state()))
                                                            })
                                                        )
                                                    })))
                                                    .child_signal(page.med_id.signal_cloned().map(clone!(app, page => move |med_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdDcPlanTmpMed, false) &&
                                                        if med_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.med_changed.signal())
                                                                .class_signal("btn-secondary", not(page.med_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_med(page.clone(), app.clone());
                                                                }), map_ref!{
                                                                    let changed = page.med_changed.signal(),
                                                                    let med_text = page.med_text.signal_cloned() =>
                                                                    !changed || med_text.is_empty()
                                                                }, app.state()))
                                                            })
                                                        })
                                                    })))
                                                    .child(doms::close_modal_btn())
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                                html!("table", {
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("ยาและวิธีใช้ยาที่ได้รับกลับบ้าน")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(page.meds.signal_cloned().to_signal_vec().map(clone!(page => move |med| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&med.med_text.clone().unwrap_or_default())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempMedModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.med_id.set_neq(med.med_id.to_string());
                                                                        page.med_text.set_neq(med.med_text.clone().unwrap_or_default());
                                                                        page.med_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                })
                                            })))
                                        }),
                                    ])
                                }),
                            ])
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_env(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_LIGHTS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" E: Environment")
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("col-sm-12")
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempEnvModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.env_id.set_neq(String::new());
                                                    page.env_text.set_neq(String::new());
                                                    page.env_changed.set_neq(false);
                                                }))
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempEnvModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempEnvModal")
                                    .child(html!("div", {
                                        .class(class::MODAL_DIALOG_C)
                                        .child(html!("div", {
                                            .class("modal-content")
                                            .children([
                                                html!("div", {
                                                    .class("modal-header")
                                                    .children([
                                                        html!("h5", {
                                                            .class("modal-title")
                                                            .child(html!("i", {.class(class::FA_USER_COG)}))
                                                            .text(" จัดการ E: Environment")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "env_text")
                                                                    .text("สภาพแวดล้อมที่เหมาะสม")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "env_text")
                                                                    .apply(mixins::textarea_value_auto_expand(page.env_text.clone(), page.env_changed.clone()))
                                                                }),
                                                            ])
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.env_id.signal_cloned().map(clone!(app, page => move |env_id| {
                                                        (!env_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdDcPlanTmpEnv, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_env(page.clone(), app.clone());
                                                                }), app.state()))
                                                            })
                                                        )
                                                    })))
                                                    .child_signal(page.env_id.signal_cloned().map(clone!(app, page => move |env_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdDcPlanTmpEnv, false) &&
                                                        if env_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.env_changed.signal())
                                                                .class_signal("btn-secondary", not(page.env_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_env(page.clone(), app.clone());
                                                                }), map_ref!{
                                                                    let changed = page.env_changed.signal(),
                                                                    let env_text = page.env_text.signal_cloned() =>
                                                                    !changed || env_text.is_empty()
                                                                }, app.state()))
                                                            })
                                                        })
                                                    })))
                                                    .child(doms::close_modal_btn())
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                                html!("table", {
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("สภาพแวดล้อมที่เหมาะสม")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(page.envs.signal_cloned().to_signal_vec().map(clone!(page => move |env| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&env.env_text.clone().unwrap_or_default())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempEnvModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.env_id.set_neq(env.env_id.to_string());
                                                                        page.env_text.set_neq(env.env_text.clone().unwrap_or_default());
                                                                        page.env_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                })
                                            })))
                                        }),
                                    ])
                                }),
                            ])
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_tx(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_LIGHTS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" T: Treatment")
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("col-sm-12")
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempTxModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.tx_id.set_neq(String::new());
                                                    page.tx_text.set_neq(String::new());
                                                    page.tx_changed.set_neq(false);
                                                }))
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempTxModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempTxModal")
                                    .child(html!("div", {
                                        .class(class::MODAL_DIALOG_C)
                                        .child(html!("div", {
                                            .class("modal-content")
                                            .children([
                                                html!("div", {
                                                    .class("modal-header")
                                                    .children([
                                                        html!("h5", {
                                                            .class("modal-title")
                                                            .child(html!("i", {.class(class::FA_USER_COG)}))
                                                            .text(" จัดการ T: Treatment")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "tx_text")
                                                                    .text("ข้อควรปฏิบัติ")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "tx_text")
                                                                    .apply(mixins::textarea_value_auto_expand(page.tx_text.clone(), page.tx_changed.clone()))
                                                                }),
                                                            ])
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.tx_id.signal_cloned().map(clone!(app, page => move |tx_id| {
                                                        (!tx_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdDcPlanTmpTx, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_tx(page.clone(), app.clone());
                                                                }), app.state()))
                                                            })
                                                        )
                                                    })))
                                                    .child_signal(page.tx_id.signal_cloned().map(clone!(app, page => move |tx_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdDcPlanTmpTx, false) &&
                                                        if tx_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.tx_changed.signal())
                                                                .class_signal("btn-secondary", not(page.tx_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_tx(page.clone(), app.clone());
                                                                }), map_ref!{
                                                                    let changed = page.tx_changed.signal(),
                                                                    let tx_text = page.tx_text.signal_cloned() =>
                                                                    !changed || tx_text.is_empty()
                                                                }, app.state()))
                                                            })
                                                        })
                                                    })))
                                                    .child(doms::close_modal_btn())
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                                html!("table", {
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("ข้อควรปฏิบัติ")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(page.txs.signal_cloned().to_signal_vec().map(clone!(page => move |tx| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&tx.tx_text.clone().unwrap_or_default())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempTxModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.tx_id.set_neq(tx.tx_id.to_string());
                                                                        page.tx_text.set_neq(tx.tx_text.clone().unwrap_or_default());
                                                                        page.tx_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                })
                                            })))
                                        }),
                                    ])
                                }),
                            ])
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_diet(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_LIGHTS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" D: Diet")
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("col-sm-12")
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempDietModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.diet_id.set_neq(String::new());
                                                    page.diet_text.set_neq(String::new());
                                                    page.diet_changed.set_neq(false);
                                                }))
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempDietModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempDietModal")
                                    .child(html!("div", {
                                        .class(class::MODAL_DIALOG_C)
                                        .child(html!("div", {
                                            .class("modal-content")
                                            .children([
                                                html!("div", {
                                                    .class("modal-header")
                                                    .children([
                                                        html!("h5", {
                                                            .class("modal-title")
                                                            .child(html!("i", {.class(class::FA_USER_COG)}))
                                                            .text(" จัดการ D: Diet")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .class("fw-bold")
                                                                    .attr("for", "diet_text")
                                                                    .text("อาหารที่ควรงดหรือควรรับประทาน")
                                                                }),
                                                                html!("textarea" => HtmlTextAreaElement, {
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "diet_text")
                                                                    .apply(mixins::textarea_value_auto_expand(page.diet_text.clone(), page.diet_changed.clone()))
                                                                }),
                                                            ])
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.diet_id.signal_cloned().map(clone!(app, page => move |diet_id| {
                                                        (!diet_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdDcPlanTmpDiet, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_diet(page.clone(), app.clone());
                                                                }), app.state()))
                                                            })
                                                        )
                                                    })))
                                                    .child_signal(page.diet_id.signal_cloned().map(clone!(app, page => move |diet_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdDcPlanTmpDiet, false) &&
                                                        if diet_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.diet_changed.signal())
                                                                .class_signal("btn-secondary", not(page.diet_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_diet(page.clone(), app.clone());
                                                                }), map_ref!{
                                                                    let changed = page.diet_changed.signal(),
                                                                    let diet_text = page.diet_text.signal_cloned() =>
                                                                    !changed || diet_text.is_empty()
                                                                }, app.state()))
                                                            })
                                                        })
                                                    })))
                                                    .child(doms::close_modal_btn())
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                                html!("table", {
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("อาหารที่ควรงดหรือควรรับประทาน")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(page.diets.signal_cloned().to_signal_vec().map(clone!(page => move |diet| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&diet.diet_text.clone().unwrap_or_default())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempDietModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.diet_id.set_neq(diet.diet_id.to_string());
                                                                        page.diet_text.set_neq(diet.diet_text.clone().unwrap_or_default());
                                                                        page.diet_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                })
                                            })))
                                        }),
                                    ])
                                }),
                            ])
                        }))
                    }))
                }),
            ])
        })
    }
}
