use sqlx::{MySql, Pool};
use std::collections::HashMap;

use kphis_model::{fetch::ExecuteResponse, pre_order::order::PreOrderIntoCommand};
use kphis_util::{error::AppError, util::zero_none};

use super::{master, order, progress_note};

// ipd-dr-pre-order-to-order.php
// ipd-dr-template-to-opd-er-order.php
// ipd-dr-template-to-order.php
// ipd-dr-template-to-pre-order.php
// POST /ipd/dr-pre-order/into
pub async fn post_pre_order_into(command: PreOrderIntoCommand, loginname: &str, doctorcode: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let from = command.from.unwrap_or_default();
    let into = command.into.unwrap_or_default();
    let from_id = command.from_id.unwrap_or_default();
    let into_id = command.into_id.unwrap_or_default();

    match (from.as_str(), into.as_str()) {
        ("pre-order", "order") => pre_order_to_order(&from_id, &into_id, loginname, pool, kphis).await,
        ("pre-order", "opd-er-order") => pre_order_to_opd_er_order(&from_id, &into_id, loginname, doctorcode, pool, kphis).await,
        ("template", "order") => template_to_order(&from_id, &into_id, loginname, doctorcode, pool, kphis).await,
        ("template", "opd-er-order") => template_to_opd_er_order(&from_id, &into_id, loginname, doctorcode, pool, kphis).await,
        ("template", "pre-order") => template_to_pre_order(&from_id, &into_id, loginname, doctorcode, pool, kphis).await,
        _ => Ok(Vec::new()),
    }
}

// ipd-dr-pre-order-to-order.php
async fn pre_order_to_order(pre_order_master_id: &str, an: &str, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    if let Some(pre_order_master_id) = pre_order_master_id.parse::<u32>().ok().and_then(zero_none) {
        let used_opt = master::select_pre_order_used_by_master_id(pre_order_master_id, pool, kphis).await?;
        if used_opt.unwrap_or_default() {
            Ok(vec![ExecuteResponse {
                last_insert_id: 0,
                rows_affected: 0,
                error: Some(String::from("Used")),
                action: Some(String::from("Prevent re-use pre-order")),
            }])
        } else {
            let mut results = Vec::with_capacity(3);
            let pre_orders = order::select_pre_order_to(pre_order_master_id, pool, kphis).await?;

            if !pre_orders.is_empty() {
                let order_ids = pre_orders.iter().map(|order| order.order_id).collect::<Vec<u32>>();
                let new_order_ids = order::insert_many_ipd_orders_with_pre(&pre_orders, an, loginname, pool, kphis).await?;

                let pre_order_items = order::select_pre_order_item_to(&order_ids, pool, kphis).await?;
                if !pre_order_items.is_empty() {
                    let order_id_map: HashMap<u32, u64> = order_ids.into_iter().zip(new_order_ids).collect();
                    let insert_order_item_result = order::insert_ipd_order_items(&pre_order_items, &order_id_map, an, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_order_item_result, "Insert PreOrderItem to OrderItem"));
                }
            }

            let notes = progress_note::select_progress_note_to(pre_order_master_id, pool, kphis).await?;
            if !notes.is_empty() {
                let note_ids = notes.iter().map(|note| note.progress_note_id).collect::<Vec<u32>>();
                let new_note_ids = progress_note::insert_many_progress_notes_from_pre_order(&notes, an, loginname, pool, kphis).await?;

                let note_items = progress_note::select_progress_note_item_to(&note_ids, pool, kphis).await?;
                if !note_items.is_empty() {
                    let note_id_map: HashMap<u32, u64> = note_ids.into_iter().zip(new_note_ids).collect();
                    let insert_note_item_result = progress_note::insert_ipd_progress_note_items(&note_items, &note_id_map, an, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_note_item_result, "Insert PreProgressNoteItem to ProgressNoteItem"));
                }
            }

            // set used
            let update_used_result = master::update_pre_order_master_used(pre_order_master_id, loginname, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(update_used_result, "Update PreOrderMaster used"));

            Ok(results)
        }
    } else {
        Err(AppError::new_server(400, "Invalid pre_order_master_id", "PreOrderToOrder"))
    }
}

async fn pre_order_to_opd_er_order(
    pre_order_master_id: &str,
    opd_er_order_master_id: &str,
    loginname: &str,
    doctorcode: &Option<String>,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    if let (Some(pre_order_master_id), Some(opd_er_order_master_id)) = (
        pre_order_master_id.parse::<u32>().ok().and_then(zero_none),
        opd_er_order_master_id.parse::<u32>().ok().and_then(zero_none),
    ) {
        let used_opt = master::select_pre_order_used_by_master_id(pre_order_master_id, pool, kphis).await?;
        if used_opt.unwrap_or_default() {
            Ok(vec![ExecuteResponse {
                last_insert_id: 0,
                rows_affected: 0,
                error: Some(String::from("Used")),
                action: Some(String::from("Prevent re-use pre-order")),
            }])
        } else {
            let mut results = Vec::with_capacity(3);
            if let Some(doctor) = doctorcode {
                let pre_orders = order::select_pre_order_to(pre_order_master_id, pool, kphis).await?;
                if !pre_orders.is_empty() {
                    let order_ids = pre_orders.iter().map(|order| order.order_id).collect::<Vec<u32>>();
                    let new_order_ids = order::insert_many_opd_er_orders_with_pre(&pre_orders, opd_er_order_master_id, loginname, doctor, pool, kphis).await?;

                    let pre_order_items = order::select_pre_order_item_to(&order_ids, pool, kphis).await?;
                    if !pre_order_items.is_empty() {
                        let order_id_map: HashMap<u32, u64> = order_ids.into_iter().zip(new_order_ids).collect();
                        let insert_order_item_result = order::insert_opd_er_order_items(&pre_order_items, &order_id_map, opd_er_order_master_id, loginname, pool, kphis).await?;
                        results.push(ExecuteResponse::from_query_result(insert_order_item_result, "Insert PreOrderItem to OpdErOrderItem"));
                    }
                }

                let notes = progress_note::select_progress_note_to(pre_order_master_id, pool, kphis).await?;
                if !notes.is_empty() {
                    let note_ids = notes.iter().map(|note| note.progress_note_id).collect::<Vec<u32>>();
                    let new_note_ids = progress_note::insert_many_opd_er_progress_notes(&notes, opd_er_order_master_id, loginname, doctor, pool, kphis).await?;

                    let note_items = progress_note::select_progress_note_item_to(&note_ids, pool, kphis).await?;
                    if !note_items.is_empty() {
                        let note_id_map: HashMap<u32, u64> = note_ids.into_iter().zip(new_note_ids).collect();
                        let insert_note_item_result = progress_note::insert_opd_er_progress_note_items(&note_items, &note_id_map, opd_er_order_master_id, loginname, pool, kphis).await?;
                        results.push(ExecuteResponse::from_query_result(insert_note_item_result, "Insert PreProgressNoteItem to ProgressNoteItem"));
                    }
                }

                // set used
                let update_used_result = master::update_pre_order_master_used(pre_order_master_id, loginname, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(update_used_result, "Update PreOrderMaster used"));
            }

            Ok(results)
        }
    } else {
        Err(AppError::new_server(400, "Invalid pre_order_master_id or opd_er_order_master_id", "PreOrderToOpdErOrder"))
    }
}

// ipd-dr-template-to-opd-er-order.php
/// move continuous order to oneday order
async fn template_to_opd_er_order(
    template_master_id: &str,
    opd_er_order_master_id: &str,
    loginname: &str,
    doctorcode: &Option<String>,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    if let (Some(template_master_id), Some(opd_er_order_master_id)) = (
        template_master_id.parse::<u32>().ok().and_then(zero_none),
        opd_er_order_master_id.parse::<u32>().ok().and_then(zero_none),
    ) {
        let mut results = Vec::with_capacity(2);

        if let Some(doctor) = doctorcode {
            let pre_orders = order::select_pre_order_to(template_master_id, pool, kphis).await?;
            if !pre_orders.is_empty() {
                let order_ids = pre_orders.iter().map(|order| order.order_id).collect::<Vec<u32>>();
                let new_order_ids = order::insert_many_opd_er_orders_with_pre(&pre_orders, opd_er_order_master_id, loginname, doctor, pool, kphis).await?;

                let pre_order_items = order::select_pre_order_item_to(&order_ids, pool, kphis).await?;
                if !pre_order_items.is_empty() {
                    let order_id_map: HashMap<u32, u64> = order_ids.into_iter().zip(new_order_ids).collect();
                    let insert_order_item_result = order::insert_opd_er_order_items(&pre_order_items, &order_id_map, opd_er_order_master_id, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_order_item_result, "Insert TemplateItem to OpdErOrderItem"));
                }
            }

            let notes = progress_note::select_progress_note_to(template_master_id, pool, kphis).await?;
            if !notes.is_empty() {
                let note_ids = notes.iter().map(|note| note.progress_note_id).collect::<Vec<u32>>();
                let new_note_ids = progress_note::insert_many_opd_er_progress_notes(&notes, opd_er_order_master_id, loginname, doctor, pool, kphis).await?;

                let note_items = progress_note::select_progress_note_item_to(&note_ids, pool, kphis).await?;
                if !note_items.is_empty() {
                    let note_id_map: HashMap<u32, u64> = note_ids.into_iter().zip(new_note_ids).collect();
                    let insert_note_item_result = progress_note::insert_opd_er_progress_note_items(&note_items, &note_id_map, opd_er_order_master_id, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_note_item_result, "Insert TemplateProgressNoteItem to ProgresNoteItem"));
                }
            }
        }

        Ok(results)
    } else {
        Err(AppError::new_server(400, "Invalid template_master_id or opd_er_order_master_id", "TemplateToOpdErOrder"))
    }
}

// ipd-dr-template-to-order.php
async fn template_to_order(pre_order_master_id: &str, an: &str, loginname: &str, doctorcode: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    if let Some(pre_order_master_id) = pre_order_master_id.parse::<u32>().ok().and_then(zero_none) {
        let mut results = Vec::with_capacity(2);

        if let Some(doctor) = doctorcode {
            let pre_orders = order::select_pre_order_to(pre_order_master_id, pool, kphis).await?;
            if !pre_orders.is_empty() {
                let order_ids = pre_orders.iter().map(|order| order.order_id).collect::<Vec<u32>>();
                let new_order_ids = order::insert_many_ipd_orders(&pre_orders, an, loginname, doctor, pool, kphis).await?;
                let pre_order_items = order::select_pre_order_item_to(&order_ids, pool, kphis).await?;
                if !pre_order_items.is_empty() {
                    let order_id_map: HashMap<u32, u64> = order_ids.into_iter().zip(new_order_ids).collect();
                    let insert_order_item_result = order::insert_ipd_order_items(&pre_order_items, &order_id_map, an, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_order_item_result, "Insert TemplateItem to OrderItem"));
                }
            }

            let notes = progress_note::select_progress_note_to(pre_order_master_id, pool, kphis).await?;
            if !notes.is_empty() {
                let note_ids = notes.iter().map(|note| note.progress_note_id).collect::<Vec<u32>>();
                let new_note_ids = progress_note::insert_many_progress_notes_from_template(&notes, an, loginname, doctor, pool, kphis).await?;

                let note_items = progress_note::select_progress_note_item_to(&note_ids, pool, kphis).await?;
                if !note_items.is_empty() {
                    let note_id_map: HashMap<u32, u64> = note_ids.into_iter().zip(new_note_ids).collect();
                    let insert_note_item_result = progress_note::insert_ipd_progress_note_items(&note_items, &note_id_map, an, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_note_item_result, "Insert TemplateProgressNoteItem to ProgressNoteItem"));
                }
            }
        }

        Ok(results)
    } else {
        Err(AppError::new_server(400, "Invalid pre_order_master_id", "TemplateToOrder"))
    }
}

// ipd-dr-template-to-pre-order.php
async fn template_to_pre_order(
    template_master_id: &str,
    pre_order_master_id: &str,
    loginname: &str,
    doctorcode: &Option<String>,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    if let (Some(template_master_id), Some(pre_order_master_id)) = (template_master_id.parse::<u32>().ok().and_then(zero_none), pre_order_master_id.parse::<u32>().ok().and_then(zero_none)) {
        let mut results = Vec::with_capacity(2);

        if let Some(doctor) = doctorcode {
            let pre_orders = order::select_pre_order_to(template_master_id, pool, kphis).await?;
            if !pre_orders.is_empty() {
                let order_ids = pre_orders.iter().map(|order| order.order_id).collect::<Vec<u32>>();
                let new_order_ids = order::insert_many_pre_orders(&pre_orders, pre_order_master_id, loginname, doctor, pool, kphis).await?;
                let pre_order_items = order::select_pre_order_item_to(&order_ids, pool, kphis).await?;
                if !pre_order_items.is_empty() {
                    let order_id_map: HashMap<u32, u64> = order_ids.into_iter().zip(new_order_ids).collect();
                    let insert_order_item_result = order::insert_pre_order_items(&pre_order_items, &order_id_map, pre_order_master_id, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_order_item_result, "Insert TemplateItem to PreOrderItem"));
                }
            }

            let notes = progress_note::select_progress_note_to(template_master_id, pool, kphis).await?;
            if !notes.is_empty() {
                let note_ids = notes.iter().map(|note| note.progress_note_id).collect::<Vec<u32>>();
                let new_note_ids = progress_note::insert_many_pre_order_progress_notes(&notes, pre_order_master_id, loginname, doctor, pool, kphis).await?;

                let note_items = progress_note::select_progress_note_item_to(&note_ids, pool, kphis).await?;
                if !note_items.is_empty() {
                    let note_id_map: HashMap<u32, u64> = note_ids.into_iter().zip(new_note_ids).collect();
                    let insert_note_item_result = progress_note::insert_pre_order_progress_note_items(&note_items, &note_id_map, pre_order_master_id, loginname, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(
                        insert_note_item_result,
                        "Insert TemplateProgressNoteItem to PreOrderProgressNoteItem",
                    ));
                }
            }
        }

        Ok(results)
    } else {
        Err(AppError::new_server(400, "Invalid template_master_id or pre_order_master_id", "TemplateToPreOrder"))
    }
}
