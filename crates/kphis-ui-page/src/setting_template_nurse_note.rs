use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::SignalVecExt,
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    ipd::tmp::{TmpDlc, TmpFocus, TmpGoal, TmpGroup, TmpIntvt, TmpParams, TmpSubGroup},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::util::str_some;

/// - GET `EndPoint::IpdTmpGroup`
/// - GET `EndPoint::IpdTmpSubgroup`
/// - GET `EndPoint::IpdTmpFocus`
/// - GET `EndPoint::IpdTmpGoal`
/// - GET `EndPoint::IpdTmpIntvt`
/// - GET `EndPoint::IpdTmpDlc`
/// - POST `EndPoint::IpdTmpGroup` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdTmpGroup` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdTmpSubgroup` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdTmpSubgroup` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdTmpFocus` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdTmpFocus` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdTmpGoal` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdTmpGoal` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdTmpIntvt` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdTmpIntvt` (guarded, remove 'Delete' btn)
/// - POST `EndPoint::IpdTmpDlc` (guarded, remove 'Save' btn)
/// - DELETE `EndPoint::IpdTmpDlc` (guarded, remove 'Delete' btn)
#[derive(Clone, Default)]
pub struct SettingTemplateNurseNotePage {
    loaded_groups: Mutable<bool>,
    groups: Mutable<Vec<TmpGroup>>,
    group_changed: Mutable<bool>,
    group_select_redraw: Mutable<bool>,
    smp_id: Mutable<String>,
    smp_name: Mutable<String>,
    // smp_group: Mutable<Option<i16>>,
    // smp_order: Mutable<Option<i16>>,
    smp_status: Mutable<String>,

    loaded_subgroups: Mutable<bool>,
    subgroups: Mutable<Vec<TmpSubGroup>>,
    subgroup_changed: Mutable<bool>,
    subgroup_select_redraw: Mutable<bool>,
    // subgroup_smp_id: Mutable<String>,
    subgroup: Mutable<String>,
    subgroup_name: Mutable<String>,
    subgroup_order: Mutable<Option<u32>>,
    subgroup_status: Mutable<String>,

    loaded_focus_goal_intvt: Mutable<bool>,

    loaded_focuses: Mutable<bool>,
    focuses: Mutable<Vec<TmpFocus>>,
    focus_changed: Mutable<bool>,
    focus_select_redraw: Mutable<bool>,
    focus_id: Mutable<String>,
    // focus_smp_id: Mutable<String>,
    // focus_subgroup: Mutable<String>,
    focus_name: Mutable<String>,
    // focus_order: Mutable<Option<u32>>,
    focus_status: Mutable<String>,

    loaded_goals: Mutable<bool>,
    goals: Mutable<Vec<TmpGoal>>,
    goal_changed: Mutable<bool>,
    // goal_select_redraw: Mutable<bool>,
    goal_id: Mutable<String>,
    // goal_smp_id: Mutable<String>,
    // goal_subgroup: Mutable<String>,
    goal_name: Mutable<String>,
    // goal_order: Mutable<Option<u32>>,
    goal_status: Mutable<String>,

    loaded_intvts: Mutable<bool>,
    intvts: Mutable<Vec<TmpIntvt>>,
    intvt_changed: Mutable<bool>,
    // intvt_select_redraw: Mutable<bool>,
    intvt_id: Mutable<String>,
    // intvt_smp_id: Mutable<String>,
    // intvt_subgroup: Mutable<String>,
    intvt_name: Mutable<String>,
    // intvt_order: Mutable<Option<u32>>,
    intvt_status: Mutable<String>,

    loaded_dlcs: Mutable<bool>,
    dlcs: Mutable<Vec<TmpDlc>>,
    dlc_changed: Mutable<bool>,
    // dlc_select_redraw: Mutable<bool>,
    dlc_id: Mutable<String>,
    dlc_name: Mutable<String>,
    // dlc_order: Mutable<Option<u32>>,
}

impl SettingTemplateNurseNotePage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            loaded_subgroups: Mutable::new(true),
            loaded_focus_goal_intvt: Mutable::new(true),
            loaded_focuses: Mutable::new(true),
            loaded_goals: Mutable::new(true),
            loaded_intvts: Mutable::new(true),
            smp_status: Mutable::new(String::from("Y")),
            subgroup_status: Mutable::new(String::from("Y")),
            focus_status: Mutable::new(String::from("Y")),
            goal_status: Mutable::new(String::from("Y")),
            intvt_status: Mutable::new(String::from("Y")),
            ..Default::default()
        })
    }

    fn has_group_and_subgroup(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let smp_id = self.smp_id.signal_cloned(),
            let subgroup = self.subgroup.signal_cloned() =>
            !smp_id.is_empty() && !subgroup.is_empty()
        }
    }

    fn load_group(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdTmpGroup`
                match TmpGroup::call_api_get(&TmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        page.groups.set(responses);
                        page.group_select_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn save_group(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let group = TmpGroup {
                    smp_id: page.smp_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    smp_name: str_some(page.smp_name.lock_ref().trim().to_owned()),
                    smp_group: None,
                    smp_order: None,
                    smp_status: str_some(page.smp_status.get_cloned()),
                };
                // POST `EndPoint::IpdTmpGroup`
                match group.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.smp_id.set(last_insert_id.to_string());
                            }
                            page.loaded_groups.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_group(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams {
                    smp_id: page.smp_id.lock_ref().parse::<u32>().ok(),
                    ..Default::default()
                };
                // DELETE `EndPoint::IpdTmpGroup`
                match TmpGroup::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.smp_id.set(String::new());
                            page.loaded_groups.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_subgroup(page: Rc<Self>, app: Rc<App>) {
        let res = page.smp_id.lock_ref().parse::<u32>();
        if let Ok(smp_id) = res {
            app.async_load(
                true,
                clone!(app => async move {
                    let params = TmpParams {
                        smp_id: Some(smp_id),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpSubgroup`
                    match TmpSubGroup::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.subgroups.set(responses);
                            page.subgroup_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }
    fn save_subgroup(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let subgroup = TmpSubGroup {
                    smp_id: page.smp_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    subgroup: page.subgroup.lock_ref().parse::<u32>().unwrap_or_default(),
                    subgroup_name: str_some(page.subgroup_name.lock_ref().trim().to_owned()),
                    subgroup_order: None,
                    subgroup_status: str_some(page.subgroup_status.get_cloned()),
                };
                // POST `EndPoint::IpdTmpSubgroup`
                match subgroup.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.subgroup.set(last_insert_id.to_string());
                            }
                            page.loaded_subgroups.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_subgroup(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams {
                    smp_id: page.smp_id.lock_ref().parse::<u32>().ok(),
                    subgroup: page.subgroup.lock_ref().parse::<u32>().ok(),
                    ..Default::default()
                };
                // DELETE `EndPoint::IpdTmpSubgroup`
                match TmpSubGroup::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.subgroup.set(String::new());
                            page.loaded_subgroups.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_focus_goal_intvt(page: Rc<Self>, app: Rc<App>) {
        let res = page.subgroup.lock_ref().parse::<u32>();
        if let Ok(subgroup) = res {
            app.async_load(
                true,
                clone!(app => async move {
                    let params = TmpParams {
                        smp_id: page.smp_id.lock_ref().parse::<u32>().ok(),
                        subgroup: Some(subgroup),
                        strict: Some(true),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpFocus`
                    match TmpFocus::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.focuses.set(responses);
                            // page.focus_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                    // GET `EndPoint::IpdTmpGoal`
                    match TmpGoal::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.goals.set(responses);
                            // page.goal_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                    // GET `EndPoint::IpdTmpIntvt`
                    match TmpIntvt::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.intvts.set(responses);
                            // page.intvt_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_focus(page: Rc<Self>, app: Rc<App>) {
        let res = page.smp_id.lock_ref().parse::<u32>();
        if let Ok(smp_id) = res {
            app.async_load(
                true,
                clone!(app => async move {
                    let params = TmpParams {
                        smp_id: Some(smp_id),
                        subgroup: page.subgroup.lock_ref().parse::<u32>().ok(),
                        strict: Some(true),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpFocus`
                    match TmpFocus::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.focuses.set(responses);
                            page.focus_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }
    fn save_focus(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let focus = TmpFocus {
                    focus_id: page.focus_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    smp_id: page.smp_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    subgroup: page.subgroup.lock_ref().parse::<u32>().unwrap_or_default(),
                    focus_name: str_some(page.focus_name.lock_ref().trim().to_owned()),
                    focus_order: None,
                    focus_status: str_some(page.focus_status.get_cloned()),
                };
                // POST `EndPoint::IpdTmpFocus`
                match focus.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.focus_id.set(last_insert_id.to_string());
                            }
                            page.loaded_focuses.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_focus(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams {
                    id: page.focus_id.lock_ref().parse::<u32>().ok(),
                    ..Default::default()
                };
                // DELETE `EndPoint::IpdTmpFocus`
                match TmpFocus::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.focus_id.set(String::new());
                            page.loaded_focuses.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_goal(page: Rc<Self>, app: Rc<App>) {
        let res = page.smp_id.lock_ref().parse::<u32>();
        if let Ok(smp_id) = res {
            app.async_load(
                true,
                clone!(app => async move {
                    let params = TmpParams {
                        smp_id: Some(smp_id),
                        subgroup: page.subgroup.lock_ref().parse::<u32>().ok(),
                        strict: Some(true),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpGoal`
                    match TmpGoal::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.goals.set(responses);
                            // page.goal_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }
    fn save_goal(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let goal = TmpGoal {
                    goal_id: page.goal_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    smp_id: page.smp_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    subgroup: page.subgroup.lock_ref().parse::<u32>().unwrap_or_default(),
                    goal_name: str_some(page.goal_name.lock_ref().trim().to_owned()),
                    goal_order: None,
                    goal_status: str_some(page.goal_status.get_cloned()),
                };
                // POST `EndPoint::IpdTmpGoal`
                match goal.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.goal_id.set(last_insert_id.to_string());
                            }
                            page.loaded_goals.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_goal(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams {
                    id: page.goal_id.lock_ref().parse::<u32>().ok(),
                    ..Default::default()
                };
                // DELETE `EndPoint::IpdTmpGoal`
                match TmpGoal::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.goal_id.set(String::new());
                            page.loaded_goals.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_intvt(page: Rc<Self>, app: Rc<App>) {
        let res = page.smp_id.lock_ref().parse::<u32>();
        if let Ok(smp_id) = res {
            app.async_load(
                true,
                clone!(app => async move {
                    let params = TmpParams {
                        smp_id: Some(smp_id),
                        subgroup: page.subgroup.lock_ref().parse::<u32>().ok(),
                        strict: Some(true),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpIntvt`
                    match TmpIntvt::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.intvts.set(responses);
                            // page.intvt_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }
    fn save_intvt(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let intvt = TmpIntvt {
                    intvt_id: page.intvt_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    smp_id: page.smp_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    subgroup: page.subgroup.lock_ref().parse::<u32>().unwrap_or_default(),
                    intvt_name: str_some(page.intvt_name.lock_ref().trim().to_owned()),
                    intvt_order: None,
                    intvt_status: str_some(page.intvt_status.get_cloned()),
                };
                // POST `EndPoint::IpdTmpIntvt`
                match intvt.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.intvt_id.set(last_insert_id.to_string());
                            }
                            page.loaded_intvts.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_intvt(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams {
                    id: page.intvt_id.lock_ref().parse::<u32>().ok(),
                    ..Default::default()
                };
                // DELETE `EndPoint::IpdTmpIntvt`
                match TmpIntvt::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.intvt_id.set(String::new());
                            page.loaded_intvts.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_dlc(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams::default();
                // GET `EndPoint::IpdTmpDlc`
                match TmpDlc::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        page.dlcs.set(responses);
                        // page.dlc_select_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn save_dlc(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let dlc = TmpDlc {
                    dlc_id: page.dlc_id.lock_ref().parse::<u32>().unwrap_or_default(),
                    dlc_name: page.dlc_name.lock_ref().trim().to_owned(),
                    dlc_order: None,
                };
                // POST `EndPoint::IpdTmpDlc`
                match dlc.call_api_post(app.state()).await {
                    Ok(response) => {
                        let last_insert_id = response.last_insert_id;
                        app.alert_execute_response(&response, async move {
                            if last_insert_id > 0 {
                                page.dlc_id.set(last_insert_id.to_string());
                            }
                            page.loaded_dlcs.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }
    fn delete_dlc(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams {
                    id: page.dlc_id.lock_ref().parse::<u32>().ok(),
                    ..Default::default()
                };
                // DELETE `EndPoint::IpdTmpDlc`
                match TmpDlc::call_api_delete(&params, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, async move {
                            page.dlc_id.set(String::new());
                            page.loaded_dlcs.set(false);
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
        app.set_title("KPHIS - Setting Template Nurse Note");

        html!("div", {
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_groups.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_group(page.clone(), app.clone());
                    page.loaded_groups.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_subgroups.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_subgroup(page.clone(), app.clone());
                    page.loaded_subgroups.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_focus_goal_intvt.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_focus_goal_intvt(page.clone(), app.clone());
                    page.loaded_focus_goal_intvt.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_focuses.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_focus(page.clone(), app.clone());
                    page.loaded_focuses.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_goals.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_goal(page.clone(), app.clone());
                    page.loaded_goals.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_intvts.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_intvt(page.clone(), app.clone());
                    page.loaded_intvts.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_dlcs.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_dlc(page.clone(), app.clone());
                    page.loaded_dlcs.set(true);
                }
                async {}
            })))
            .future(page.group_select_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    if let Some(elm) = app.get_id("search_temp_smp") {
                        NiceSelect::new_default_with_value(&elm, &page.smp_id.lock_ref());
                    }
                    page.group_select_redraw.set(false);
                }
                async {}
            })))
            .future(page.subgroup_select_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    if let Some(elm) = app.get_id("search_temp_subgroup") {
                        NiceSelect::new_default_with_value(&elm, &page.subgroup.lock_ref());
                    }
                    page.subgroup_select_redraw.set(false);
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
                                .text(" Template Nursing Progress Note บันทึก/แก้ไข")
                            }))
                        }))
                    }),
                    Self::render_group(page.clone(), app.clone()),
                    Self::render_subgroup(page.clone(), app.clone()),
                    Self::render_focus(page.clone(), app.clone()),
                    Self::render_goal(page.clone(), app.clone()),
                    Self::render_intvt(page.clone(), app.clone()),
                    Self::render_dlc(page.clone(), app.clone()),
                ])
            }))
        })
    }

    fn render_group(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_CYANS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" กลุ่มอาการ (Group)")
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("col-sm-12")
                            .children([
                                html!("div", {
                                    .class(class::FLEX_T2)
                                    .child(html!("div", {
                                        .class("col-4")
                                        .child(html!("div", {
                                            //.attr("id", "dropdown_data_smp")
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class("form-control")
                                                .attr("id", "search_temp_smp")
                                                .child(html!("option", {.attr("value","").text("เลือก")}))
                                                .children_signal_vec(page.groups.signal_cloned().to_signal_vec().map(|group| {
                                                    html!("option", {
                                                        .attr("value", &group.smp_id.to_string())
                                                        .text(&group.smp_name.unwrap_or_default())
                                                    })
                                                }))
                                                .prop_signal("value", page.smp_id.signal_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Change| {
                                                        page.smp_id.set_neq(element.value());
                                                        page.subgroup.set_neq(String::new());
                                                        page.focuses.lock_mut().clear();
                                                        page.goals.lock_mut().clear();
                                                        page.intvts.lock_mut().clear();
                                                        page.loaded_subgroups.set(false);
                                                    }))
                                                })
                                                // onchange="table_data_seting_temp_smp_search()
                                            }))
                                        }))
                                    }))
                                    // SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_R_BLUE)
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#AddTempSmpModal")
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .text(" เพิ่ม")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.smp_id.set_neq(String::new());
                                                page.smp_name.set_neq(String::new());
                                                // page.smp_group.set_neq(None);
                                                // page.smp_order.set_neq(None);
                                                page.smp_status.set_neq(String::from("Y"));
                                                page.group_changed.set_neq(false);
                                                page.group_select_redraw.set(true);
                                            }))
                                            // .attr("onclick", "add_setting_template_nurse_note_smp()")
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempSmpModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempSmpModal")
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
                                                            .text(" จัดการ Template กลุ่มอาการ (Group)")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        //.attr("id", "form_temp_setting_smp")
                                                        .children([
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    html!("label", {
                                                                        .attr("for", "smp_name")
                                                                        .text("กลุ่มอาการ")
                                                                    }),
                                                                    html!("input" => HtmlInputElement, {
                                                                        .attr("type", "text")
                                                                        .class(class::FORM_CTRL_T)
                                                                        .attr("id", "smp_name")
                                                                        .apply(mixins::string_value_not_empty(page.smp_name.clone(), page.group_changed.clone()))
                                                                    }),
                                                                ])
                                                            }),
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    radio_input("smp_status1", page.smp_status.clone(), page.group_changed.clone(), "Y", "ยังใช้งานอยู่"),
                                                                    radio_input("smp_status2", page.smp_status.clone(), page.group_changed.clone(), "N", "ยกเลิกการใช้งาน"),
                                                                ])
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.smp_id.signal_cloned().map(clone!(app, page => move |smp_id| {
                                                        // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','REMOVE'))
                                                        (!smp_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdTmpGroup, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                //.attr("id", "btn_delete_template_smp")
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_group(page.clone(), app.clone());
                                                                }), app.state()))
                                                                // .attr("onclick", "delete_setting_template_nurse_note_smp();")
                                                            })
                                                        )
                                                    })))
                                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                                    .child_signal(page.smp_id.signal_cloned().map(clone!(app, page => move |smp_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdTmpGroup, false) &&
                                                        if smp_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.group_changed.signal())
                                                                .class_signal("btn-secondary", not(page.group_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_group(page.clone(), app.clone());
                                                                }), not(page.group_changed.signal()), app.state()))
                                                                // .attr("onclick", "save_setting_template_nurse_note_smp();")
                                                            })
                                                        })
                                                    })))
                                                    .child(doms::close_modal_btn())
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                            ])
                            .child_signal(map_ref!{
                                let smp_id = page.smp_id.signal_cloned(),
                                let groups =  page.groups.signal_cloned() =>
                                (smp_id.clone(), groups.clone())
                            }.map(clone!(app, page => move |(smp_id, groups)| {
                                groups.into_iter().find(|item| item.smp_id == smp_id.parse::<u32>().unwrap_or_default()).map(clone!(app, page => move |group| {
                                    html!("table", {
                                        //.attr("id", "table_temp_setting_smp")
                                        .class(class::TABLE_SM)
                                        .children([
                                            html!("thead", {
                                                .child(html!("tr", {
                                                    .children([
                                                        html!("th", {.attr("scope", "col").style("display","none").text("#")}),
                                                        html!("th", {.attr("scope", "col").text("ชื่อกลุ่มอาการ")}),
                                                        html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่ม")}),
                                                        html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("สถานะ")}),
                                                        html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                    ])
                                                }))
                                            }),
                                            html!("tbody", {
                                                //.attr("id", "table_temp_smp_body")
                                                .child(html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&group.smp_name.clone().unwrap_or_default())}),
                                                        html!("td", {.class("text-center").text(&group.smp_group.unwrap_or_default().to_string())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply(|dom| {
                                                                let child = if &group.smp_status.clone().unwrap_or_default() == "Y" {
                                                                    html!("i", {
                                                                        .class(class::FA_CHECK_GREEN)
                                                                        .attr("data-bs-toggle","tooltip")
                                                                        .attr("data-bs-placement","top")
                                                                        .attr("title","ยังใช้งานอยู่")
                                                                    })
                                                                } else {
                                                                    html!("i", {.class(class::FA_CHECK_GREEN)})
                                                                };
                                                                dom.child(child)
                                                            })
                                                        }),
                                                        html!("td", {
                                                            .class("text-center")
                                                            // NURSING_PROGRESSNOTE_TEMPLATE_EDIT
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempSmpModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.smp_id.set_neq(group.smp_id.to_string());
                                                                        page.smp_name.set_neq(group.smp_name.clone().unwrap_or_default());
                                                                        // page.smp_group.set_neq(group.smp_group);
                                                                        // page.smp_order.set_neq(group.smp_order);
                                                                        page.smp_status.set_neq(group.smp_status.clone().unwrap_or_default());
                                                                        page.group_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                }))
                                            }),
                                        ])
                                    })
                                }))
                            })))
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_subgroup(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            //.attr("id", "card_subgroup")
            .visible_signal(page.smp_id.signal_cloned().map(|smp_id| !smp_id.is_empty()))
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_GRAYS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" กลุ่มอาการย่อย (Subgroup)")
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("col-sm-12")
                            .children([
                                html!("div", {
                                    .class(class::FLEX_T2)
                                    .child(html!("div", {
                                        .class("col-4")
                                        .child(html!("div", {
                                            //.attr("id", "dropdown_data_subgroup")
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class("form-control")
                                                .attr("id", "search_temp_subgroup")
                                                .child(html!("option", {.attr("value","").text("เลือก")}))
                                                .children_signal_vec(page.subgroups.signal_cloned().to_signal_vec().map(|subgroup| {
                                                    html!("option", {
                                                        .attr("value", &subgroup.subgroup.to_string())
                                                        .text(&subgroup.subgroup_name.unwrap_or_default())
                                                    })
                                                }))
                                                .child(html!("option", {.attr("value","0").class("fw-bold").text("**ไม่ระบุ(แสดงเสมอ)**")}))
                                                .prop_signal("value", page.subgroup.signal_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Change| {
                                                        page.subgroup.set_neq(element.value());
                                                        page.loaded_focus_goal_intvt.set(false);
                                                    }))
                                                })
                                                // table_data_seting_temp_subgroup_search()
                                            }))
                                        }))
                                    }))
                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_R_BLUE)
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#AddTempSubgroupModal")
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .text(" เพิ่ม")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.subgroup.set_neq(String::new());
                                                page.subgroup_name.set_neq(String::new());
                                                page.subgroup_order.set_neq(None);
                                                page.subgroup_status.set_neq(String::from("Y"));
                                                page.subgroup_changed.set_neq(false);
                                                page.subgroup_select_redraw.set(true);
                                            }))
                                            // .attr("onclick", "add_setting_template_nurse_note_subgroup()")
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempSubgroupModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempSubgroupModal")
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
                                                            .text(" จัดการ Template กลุ่มอาการย่อย (Subgroup)")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        //.attr("id", "form_temp_setting_subgroup")
                                                        .children([
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    html!("label", {
                                                                        .attr("for", "subgroup_name")
                                                                        .text("กลุ่มอาการ")
                                                                    }),
                                                                    html!("input" => HtmlInputElement, {
                                                                        .attr("type", "text")
                                                                        .class(class::FORM_CTRL_T)
                                                                        .attr("id", "subgroup_name")
                                                                        .apply(mixins::string_value_not_empty(page.subgroup_name.clone(), page.subgroup_changed.clone()))
                                                                    }),
                                                                ])
                                                            }),
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    radio_input("subgroup_status1", page.subgroup_status.clone(), page.subgroup_changed.clone(), "Y", "ยังใช้งานอยู่"),
                                                                    radio_input("subgroup_status2", page.subgroup_status.clone(), page.subgroup_changed.clone(), "N", "ยกเลิกการใช้งาน"),
                                                                ])
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.subgroup.signal_cloned().map(clone!(app, page => move |subgroup| {
                                                        // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','REMOVE'))
                                                        (!subgroup.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdTmpSubgroup, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                //.attr("id", "btn_delete_template_subgroup")
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_subgroup(page.clone(), app.clone());
                                                                }), app.state()))
                                                                // .attr("onclick", "delete_setting_template_nurse_note_subgroup();")
                                                            })
                                                        )
                                                    })))
                                                    // .attr("onclick", "reset_setting_template_nurse_note_subgroup()")
                                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                                    .child_signal(page.subgroup.signal_cloned().map(clone!(app, page => move |subgroup| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdTmpSubgroup, false) &&
                                                        if subgroup.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.subgroup_changed.signal())
                                                                .class_signal("btn-secondary", not(page.subgroup_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_subgroup(page.clone(), app.clone());
                                                                }), not(page.subgroup_changed.signal()), app.state()))
                                                                // .attr("onclick", "save_setting_template_nurse_note_subgroup();")
                                                            })
                                                        })
                                                    })))
                                                    .child(doms::close_modal_btn())
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                            ])
                            .child_signal(map_ref!{
                                let subgroup_id = page.subgroup.signal_cloned(),
                                let subgroups =  page.subgroups.signal_cloned() =>
                                (subgroup_id.clone(), subgroups.clone())
                            }.map(clone!(app, page => move |(subgroup_id, subgroups)| {
                                subgroups.into_iter().find(|item| item.subgroup == subgroup_id.parse::<u32>().unwrap_or_default()).map(clone!(app, page => move |subgroup| {
                                    html!("table", {
                                        //.attr("id", "table_temp_setting_subgroup")
                                        .class(class::TABLE_SM)
                                        .children([
                                            html!("thead", {
                                                .child(html!("tr", {
                                                    .children([
                                                        // html!("th", {.attr("scope", "col").style("display","none").text("#")}),
                                                        html!("th", {.attr("scope", "col").text("กลุ่มอาการย่อย (Subgroup)")}),
                                                        html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่ม")}),
                                                        html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่มย่อย")}),
                                                        html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("สถานะ")}),
                                                        html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                    ])
                                                }))
                                            }),
                                            html!("tbody", {
                                                //.attr("id", "table_temp_subgroup_body")
                                                .child(html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&subgroup.subgroup_name.clone().unwrap_or_default())}),
                                                        html!("td", {.class("text-center").text(&subgroup.smp_id.to_string())}),
                                                        html!("td", {.class("text-center").text(&subgroup.subgroup.to_string())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply(|dom| {
                                                                let child = if &subgroup.subgroup_status.clone().unwrap_or_default() == "Y" {
                                                                    html!("i", {
                                                                        .class(class::FA_CHECK_GREEN)
                                                                        .attr("data-bs-toggle","tooltip")
                                                                        .attr("data-bs-placement","top")
                                                                        .attr("title","ยังใช้งานอยู่")
                                                                    })
                                                                } else {
                                                                    html!("i", {.class(class::FA_X_RED)})
                                                                };
                                                                dom.child(child)
                                                            })
                                                        }),
                                                        html!("td", {
                                                            .class("text-center")
                                                            // NURSING_PROGRESSNOTE_TEMPLATE_EDIT
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempSubgroupModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.subgroup.set_neq(subgroup.subgroup.to_string());
                                                                        page.subgroup_name.set_neq(subgroup.subgroup_name.clone().unwrap_or_default());
                                                                        page.subgroup_order.set_neq(subgroup.subgroup_order);
                                                                        page.subgroup_status.set_neq(subgroup.subgroup_status.clone().unwrap_or_default());
                                                                        page.subgroup_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                }))
                                            }),
                                        ])
                                    })
                                }))
                            })))
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_focus(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            //.attr("id", "card_focus")
            .visible_signal(page.has_group_and_subgroup())
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_REDS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" ปัญหา (Focus)")
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
                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempFocusModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.focus_id.set_neq(String::new());
                                                    page.focus_name.set_neq(String::new());
                                                    // page.focus_order.set_neq(None);
                                                    page.focus_status.set_neq(String::from("Y"));
                                                    page.focus_changed.set_neq(false);
                                                    // page.focus_select_redraw.set(true);
                                                }))
                                                // .attr("onclick", "add_setting_template_nurse_note_focus()")
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempFocusModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempFocusModal")
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
                                                            .text(" จัดการ Template ปัญหา (Focus)")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        //.attr("id", "form_temp_setting_focus")
                                                        .children([
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    html!("label", {
                                                                        .attr("for", "focus_name")
                                                                        .text("ปัญหา (Focus)")
                                                                    }),
                                                                    html!("input" => HtmlInputElement, {
                                                                        .attr("type", "text")
                                                                        .class(class::FORM_CTRL_T)
                                                                        .attr("id", "focus_name")
                                                                        .apply(mixins::string_value_not_empty(page.focus_name.clone(), page.focus_changed.clone()))
                                                                    }),
                                                                ])
                                                            }),
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    radio_input("focus_status1", page.focus_status.clone(), page.focus_changed.clone(), "Y", "ยังใช้งานอยู่"),
                                                                    radio_input("focus_status2", page.focus_status.clone(), page.focus_changed.clone(), "N", "ยกเลิกการใช้งาน"),
                                                                ])
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.focus_id.signal_cloned().map(clone!(app, page => move |focus_id| {
                                                        // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','REMOVE'))
                                                        (!focus_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdTmpFocus, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                //.attr("id", "btn_delete_template_focus")
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_focus(page.clone(), app.clone());
                                                                }), app.state()))
                                                                // .attr("onclick", "delete_setting_template_nurse_note_focus();")
                                                            })
                                                        )
                                                    })))
                                                    // .attr("onclick", "reset_setting_template_nurse_note_focus()")
                                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                                    .child_signal(page.focus_id.signal_cloned().map(clone!(app, page => move |focus_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdTmpFocus, false) &&
                                                        if focus_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.focus_changed.signal())
                                                                .class_signal("btn-secondary", not(page.focus_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_focus(page.clone(), app.clone());
                                                                }), not(page.focus_changed.signal()), app.state()))
                                                                // .attr("onclick", "save_setting_template_nurse_note_focus();")
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
                                    //.attr("id", "table_temp_setting_focus")
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    // html!("th", {.attr("scope", "col").style("display","none").text("#")}),
                                                    html!("th", {.attr("scope", "col").text("ชื่อปัญหา (Focus)")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่ม")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่มย่อย")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("สถานะ")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            //.attr("id", "table_temp_focus_body")
                                            .children_signal_vec(page.focuses.signal_cloned().to_signal_vec().map(clone!(page => move |focus| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&focus.focus_name.clone().unwrap_or_default())}),
                                                        html!("td", {.class("text-center").text(&focus.smp_id.to_string())}),
                                                        html!("td", {.class("text-center").text(&focus.subgroup.to_string())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply(|dom| {
                                                                let child = if &focus.focus_status.clone().unwrap_or_default() == "Y" {
                                                                    html!("i", {
                                                                        .class(class::FA_CHECK_GREEN)
                                                                        .attr("data-bs-toggle","tooltip")
                                                                        .attr("data-bs-placement","top")
                                                                        .attr("title","ยังใช้งานอยู่")
                                                                    })
                                                                } else {
                                                                    html!("i", {.class(class::FA_X_RED)})
                                                                };
                                                                dom.child(child)
                                                            })
                                                        }),
                                                        html!("td", {
                                                            .class("text-center")
                                                            // NURSING_PROGRESSNOTE_TEMPLATE_EDIT
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempFocusModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.focus_id.set_neq(focus.focus_id.to_string());
                                                                        page.focus_name.set_neq(focus.focus_name.clone().unwrap_or_default());
                                                                        // page.focus_order.set_neq(focus.focus_order);
                                                                        page.focus_status.set_neq(focus.focus_status.clone().unwrap_or_default());
                                                                        page.focus_changed.set_neq(false);
                                                                    }))
                                                                }))
                                                            })
                                                        }),
                                                    ])
                                                })
                                            })))
                                        }),
                                    ])
                                })
                            ])
                        }))
                    }))
                }),
            ])
        })
    }

    fn render_goal(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            //.attr("id", "card_goal")
            .visible_signal(page.has_group_and_subgroup())
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_GREENS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" เป้าหมาย (Goal)")
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
                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempGoalModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.goal_id.set_neq(String::new());
                                                    page.goal_name.set_neq(String::new());
                                                    // page.goal_order.set_neq(None);
                                                    page.goal_status.set_neq(String::from("Y"));
                                                    page.goal_changed.set_neq(false);
                                                    // page.goal_select_redraw.set(true);
                                                }))
                                                // .attr("onclick", "add_setting_template_nurse_note_goal()")
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempGoalModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempGoalModal")
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
                                                            .text(" จัดการ Template เป้าหมาย (Goal)")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        //.attr("id", "form_temp_setting_goal")
                                                        .children([
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    html!("label", {
                                                                        .attr("for", "goal_name")
                                                                        .text("เป้าหมาย (Goal)")
                                                                    }),
                                                                    html!("input" => HtmlInputElement, {
                                                                        .attr("type", "text")
                                                                        .class(class::FORM_CTRL_T)
                                                                        .attr("id", "goal_name")
                                                                        .apply(mixins::string_value_not_empty(page.goal_name.clone(), page.goal_changed.clone()))
                                                                    }),
                                                                ])
                                                            }),
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    radio_input("goal_status1", page.goal_status.clone(), page.goal_changed.clone(), "Y", "ยังใช้งานอยู่"),
                                                                    radio_input("goal_status2", page.goal_status.clone(), page.goal_changed.clone(), "N", "ยกเลิกการใช้งาน"),
                                                                ])
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.goal_id.signal_cloned().map(clone!(app, page => move |goal_id| {
                                                        // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','REMOVE'))
                                                        (!goal_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdTmpGoal, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                //.attr("id", "btn_delete_template_goal")
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_goal(page.clone(), app.clone());
                                                                }), app.state()))
                                                                // .attr("onclick", "delete_setting_template_nurse_note_goal();")
                                                            })
                                                        )
                                                    })))
                                                    // .attr("onclick", "reset_setting_template_nurse_note_goal()")
                                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                                    .child_signal(page.goal_id.signal_cloned().map(clone!(app, page => move |goal_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdTmpGoal, false) &&
                                                        if goal_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.goal_changed.signal())
                                                                .class_signal("btn-secondary", not(page.goal_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_goal(page.clone(), app.clone());
                                                                }), not(page.goal_changed.signal()), app.state()))
                                                                // .attr("onclick", "save_setting_template_nurse_note_goal();")
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
                                    //.attr("id", "table_temp_setting_goal")
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    // html!("th", {.attr("scope", "col").style("display","none").text("#")}),
                                                    html!("th", {.attr("scope", "col").text("ชื่อเป้าหมาย (Goal)")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่ม")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่มย่อย")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("สถานะ")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            //.attr("id", "table_temp_goal_body")
                                            .children_signal_vec(page.goals.signal_cloned().to_signal_vec().map(clone!(page => move |goal| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&goal.goal_name.clone().unwrap_or_default())}),
                                                        html!("td", {.class("text-center").text(&goal.smp_id.to_string())}),
                                                        html!("td", {.class("text-center").text(&goal.subgroup.to_string())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply(|dom| {
                                                                let child = if &goal.goal_status.clone().unwrap_or_default() == "Y" {
                                                                    html!("i", {
                                                                        .class(class::FA_CHECK_GREEN)
                                                                        .attr("data-bs-toggle","tooltip")
                                                                        .attr("data-bs-placement","top")
                                                                        .attr("title","ยังใช้งานอยู่")
                                                                    })
                                                                } else {
                                                                    html!("i", {.class(class::FA_X_RED)})
                                                                };
                                                                dom.child(child)
                                                            })
                                                        }),
                                                        html!("td", {
                                                            .class("text-center")
                                                            // NURSING_PROGRESSNOTE_TEMPLATE_EDIT
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempGoalModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.goal_id.set_neq(goal.goal_id.to_string());
                                                                        page.goal_name.set_neq(goal.goal_name.clone().unwrap_or_default());
                                                                        // page.goal_order.set_neq(goal.goal_order);
                                                                        page.goal_status.set_neq(goal.goal_status.clone().unwrap_or_default());
                                                                        page.goal_changed.set_neq(false);
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

    fn render_intvt(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            //.attr("id", "card_intvt")
            .visible_signal(page.has_group_and_subgroup())
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_CYANS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" Intervention")
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
                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempIntvtModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.intvt_id.set_neq(String::new());
                                                    page.intvt_name.set_neq(String::new());
                                                    // page.intvt_order.set_neq(None);
                                                    page.intvt_status.set_neq(String::from("Y"));
                                                    page.intvt_changed.set_neq(false);
                                                    // page.intvt_select_redraw.set(true);
                                                }))
                                                // .attr("onclick", "add_setting_template_nurse_note_intvt()")
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempIntvtModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempIntvtModal")
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
                                                            .text(" จัดการ Template Intervention")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        //.attr("id", "form_temp_setting_intvt")
                                                        .children([
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    html!("label", {
                                                                        .attr("for", "intvt_name")
                                                                        .text("Intervention")
                                                                    }),
                                                                    html!("input" => HtmlInputElement, {
                                                                        .attr("type", "text")
                                                                        .class(class::FORM_CTRL_T)
                                                                        .attr("id", "intvt_name")
                                                                        .apply(mixins::string_value_not_empty(page.intvt_name.clone(), page.intvt_changed.clone()))
                                                                    }),
                                                                ])
                                                            }),
                                                            html!("div", {
                                                                .class("mb-3")
                                                                .children([
                                                                    radio_input("intvt_status1", page.intvt_status.clone(), page.intvt_changed.clone(), "Y", "ยังใช้งานอยู่"),
                                                                    radio_input("intvt_status2", page.intvt_status.clone(), page.intvt_changed.clone(), "N", "ยกเลิกการใช้งาน"),
                                                                ])
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.intvt_id.signal_cloned().map(clone!(app, page => move |intvt_id| {
                                                        // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','REMOVE'))
                                                        (!intvt_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdTmpIntvt, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                //.attr("id", "btn_delete_template_intvt")
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_intvt(page.clone(), app.clone());
                                                                }), app.state()))
                                                                // .attr("onclick", "delete_setting_template_nurse_note_intvt();")
                                                            })
                                                        )
                                                    })))
                                                    // .attr("onclick", "reset_setting_template_nurse_note_intvt()")
                                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                                    .child_signal(page.intvt_id.signal_cloned().map(clone!(app, page => move |intvt_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdTmpIntvt, false) &&
                                                        if intvt_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.intvt_changed.signal())
                                                                .class_signal("btn-secondary", not(page.intvt_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_intvt(page.clone(), app.clone());
                                                                }), not(page.intvt_changed.signal()), app.state()))
                                                                // .attr("onclick", "save_setting_template_nurse_note_intvt();")
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
                                    //.attr("id", "table_temp_setting_intvt")
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    // html!("th", {.attr("scope", "col").style("display","none").text("#")}),
                                                    html!("th", {.attr("scope", "col").text("Intervention")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่ม")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("กลุ่มย่อย")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("สถานะ")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            //.attr("id", "table_temp_intvt_body")
                                            .children_signal_vec(page.intvts.signal_cloned().to_signal_vec().map(clone!(page => move |intvt| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {
                                                            .children(doms::square_bracket_to_span(&intvt.intvt_name.clone().unwrap_or_default()))
                                                            // .text(&intvt.intvt_name.clone().unwrap_or_default())
                                                        }),
                                                        html!("td", {.class("text-center").text(&intvt.smp_id.to_string())}),
                                                        html!("td", {.class("text-center").text(&intvt.subgroup.to_string())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            .apply(|dom| {
                                                                let child = if &intvt.intvt_status.clone().unwrap_or_default() == "Y" {
                                                                    html!("i", {
                                                                        .class(class::FA_CHECK_GREEN)
                                                                        .attr("data-bs-toggle","tooltip")
                                                                        .attr("data-bs-placement","top")
                                                                        .attr("title","ยังใช้งานอยู่")
                                                                    })
                                                                } else {
                                                                    html!("i", {.class(class::FA_X_RED)})
                                                                };
                                                                dom.child(child)
                                                            })
                                                        }),
                                                        html!("td", {
                                                            .class("text-center")
                                                            // NURSING_PROGRESSNOTE_TEMPLATE_EDIT
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempIntvtModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.intvt_id.set_neq(intvt.intvt_id.to_string());
                                                                        page.intvt_name.set_neq(intvt.intvt_name.clone().unwrap_or_default());
                                                                        // page.intvt_order.set_neq(intvt.intvt_order);
                                                                        page.intvt_status.set_neq(intvt.intvt_status.clone().unwrap_or_default());
                                                                        page.intvt_changed.set_neq(false);
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

    fn render_dlc(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD_BDARKS)
            //.attr("id", "card_dlc")
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_BDARKS_LIGHTS)
                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                    .text(" Daily Care")
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
                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                    .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateAdd), |dom| {
                                        dom.child(html!("div", {
                                            .class(class::COL12_T)
                                            .child(html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_R_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#AddTempDlcModal")
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .text(" เพิ่ม")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.dlc_id.set_neq(String::new());
                                                    page.dlc_name.set_neq(String::new());
                                                    // page.intvt_order.set_neq(None);
                                                    page.dlc_changed.set_neq(false);
                                                }))
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "AddTempDlcModal")
                                    .attr("tabindex", "-1")
                                    .attr("aria-labelledby", "AddTempDlcModal")
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
                                                            .text(" จัดการ Daily Care")
                                                        }),
                                                        doms::close_modal_x_btn(),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class("modal-body")
                                                    .child(html!("div", {
                                                        //.attr("id", "form_temp_setting_dlc")
                                                        .child(html!("div", {
                                                            .class("mb-3")
                                                            .children([
                                                                html!("label", {
                                                                    .attr("for", "dlc_name")
                                                                    .text("Daily Care")
                                                                }),
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "text")
                                                                    .class(class::FORM_CTRL_T)
                                                                    .attr("id", "dlc_name")
                                                                    .apply(mixins::string_value_not_empty(page.dlc_name.clone(), page.dlc_changed.clone()))
                                                                }),
                                                            ])
                                                        }))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("modal-footer")
                                                    .child_signal(page.dlc_id.signal_cloned().map(clone!(app, page => move |dlc_id| {
                                                        // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','REMOVE'))
                                                        (!dlc_id.is_empty() && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdTmpDlc, false)).then(||
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_LX_RED)
                                                                //.attr("id", "btn_delete_template_dlc")
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Delete")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                    Self::delete_dlc(page.clone(), app.clone());
                                                                }), app.state()))
                                                            })
                                                        )
                                                    })))
                                                    // if(SessionManager::checkPermission('NURSING_PROGRESSNOTE_TEMPLATE','ADD'))
                                                    .child_signal(page.dlc_id.signal_cloned().map(clone!(app, page => move |dlc_id| {
                                                        (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdTmpDlc, false) &&
                                                        if dlc_id.parse::<u32>().map(|id| id > 0).unwrap_or_default() {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateEdit)
                                                        } else {
                                                            app.has_permission(Permission::NursingProgressnoteTemplateAdd)
                                                        }).then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class("btn")
                                                                .class_signal("btn-primary", page.dlc_changed.signal())
                                                                .class_signal("btn-secondary", not(page.dlc_changed.signal()))
                                                                .attr("data-bs-dismiss", "modal")
                                                                .text("Save")
                                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                    Self::save_dlc(page.clone(), app.clone());
                                                                }), map_ref!{
                                                                    let changed = page.dlc_changed.signal(),
                                                                    let name = page.dlc_name.signal_cloned() =>
                                                                    !changed || name.is_empty()
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
                                    //.attr("id", "table_temp_setting_dlc")
                                    .class(class::TABLE_SM)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    // html!("th", {.attr("scope", "col").style("display","none").text("#")}),
                                                    html!("th", {.attr("scope", "col").text("Daily Care")}),
                                                    html!("th", {.attr("scope", "col").class("text-center").style("width","120px").text("แก้ไข")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            //.attr("id", "table_temp_dlc_body")
                                            .children_signal_vec(page.dlcs.signal_cloned().to_signal_vec().map(clone!(page => move |dlc| {
                                                html!("tr", {
                                                    .children([
                                                        html!("td", {.text(&dlc.dlc_name.clone())}),
                                                        html!("td", {
                                                            .class("text-center")
                                                            // NURSING_PROGRESSNOTE_TEMPLATE_EDIT
                                                            .apply_if(app.has_permission(Permission::NursingProgressnoteTemplateEdit), |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#AddTempDlcModal")
                                                                    .child(html!("i",{.class(class::FA_EDIT)}))
                                                                    .event(clone!(page => move |_: events::Click| {
                                                                        page.dlc_id.set_neq(dlc.dlc_id.to_string());
                                                                        page.dlc_name.set_neq(dlc.dlc_name.clone());
                                                                        // page.dlc_order.set_neq(dlc.dlc_order);
                                                                        page.dlc_changed.set_neq(false);
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

fn radio_input(id: &str, mutable: Mutable<String>, changed: Mutable<bool>, value: &'static str, label: &'static str) -> Dom {
    html!("div", {
        .class("form-check")
        .children([
            html!("input" => HtmlInputElement, {
                .class("form-check-input")
                .attr("type", "radio")
                //.attr("id", id)
                .attr("value", value)
                .apply(mixins::radio_match(mutable, changed, value))
            }),
            doms::label_check_for(id, label),
        ])
    })
}
