use base64::{Engine, engine::general_purpose};
use rust_decimal::prelude::{Decimal, ToPrimitive};
use serde_json::{Number, Value};
use sqlx::{AssertSqlSafe, Column, FromRow, MySql, Pool, Row, TypeInfo, mysql::MySqlRow};
use std::sync::Arc;
use time::{Date, PrimitiveDateTime, Time, format_description::well_known::Iso8601};
use tokio::sync::Mutex;

use kphis_model::{
    app::AppAsset,
    fetch::ExecuteResponse,
    report::{BasicType, CustomReport, ReportParam, ReportTemplateParams},
};
use kphis_sql::report;
use kphis_util::{
    datetime::JsTime,
    error::{AppError, Source},
};

// GET /template/customs/{template_name}.typ
pub async fn select_report_template_content(template_name: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Option<String>, AppError> {
    let sql = report::get_report_template_content(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(template_name)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReportTemplateContent"))?
        .as_ref()
        .map(|row| row.try_get("content"))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReportTemplateContent"))
}

pub async fn select_report_template(params: &ReportTemplateParams, pool: &Pool<MySql>, hosxp: &str, kphis_extra: &str) -> Result<Vec<CustomReport>, AppError> {
    let sql = report::get_report_template(params, hosxp, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(template_id) = params.template_id {
        query = query.bind(template_id);
    }
    if let Some(template_name) = params.template_name.as_ref() {
        query = query.bind(template_name);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReportTemplate"))?
        .iter()
        .map(CustomReport::from_row)
        .collect::<sqlx::Result<Vec<CustomReport>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReportTemplate"))
}

pub async fn select_report_template_compact(params: &ReportTemplateParams, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<CustomReport>, AppError> {
    let sql = report::get_report_template_compact(params, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(template_id) = params.template_id {
        query = query.bind(template_id);
    }
    if let Some(template_name) = params.template_name.as_ref() {
        query = query.bind(template_name);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReportTemplateCompact"))?
        .iter()
        .map(custom_report_compact_from_row)
        .collect::<sqlx::Result<Vec<CustomReport>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReportTemplateCompact"))
}
fn custom_report_compact_from_row(row: &MySqlRow) -> sqlx::Result<CustomReport> {
    Ok(CustomReport {
        template_id: row.try_get("template_id")?,
        template_name: row.try_get("template_name")?,
        title: row.try_get("title")?,
        disabled: row.try_get("disabled")?,
        ..Default::default()
    })
}

// /// `params` format is `param1^__P1__|param2^__P2__`<br>
// /// `ids` format is `id1|id2`<br>
// /// Replace `__P1__` and `__P2__` in sql statement with `id1` and `id2`.<br>
// /// Return JSON string as<br>
// /// `{ "param1": "id1", "param2": "id2", .., "data": [sql results]}`
// pub async fn select_raw_query_to_json_string(statement: &str, params: &str, ids: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str, kphis_log: &str) -> Result<String, AppError> {
//     if statement.starts_with("SELECT ") {
//         let mut obj = serde_json::Map::new();
//         // get iter of ([label, __XX__], id)
//         let params = explode_cap_pipe(params, 2).into_iter().zip(ids.split("|"));
//         let mut sql = statement
//             .replace("__HOSXP__", hosxp)
//             .replace("__KPHIS__", kphis)
//             .replace("__KPHIS_EXTRA__", kphis_extra)
//             .replace("__KPHIS_LOG__", kphis_log);
//         // param always has 2 items
//         for (param, id) in params {
//             sql = sql.replace(&param[1], id);
//             obj.insert(param[0].to_owned(), Value::String(id.to_owned()));
//         }
//         let values = sqlx::query(AssertSqlSafe(sql))
//             .fetch_all(pool)
//             .await
//             .map_err(|e| Source::SQLx.to_error(500, e, "Select RawQuery"))?
//             .iter()
//             .map(row_to_json)
//             .collect::<sqlx::Result<Vec<Value>>>()
//             .map_err(|e| Source::SQLx.to_error(500, e, "Select ReportTemplateContent"))?;
//         obj.insert(String::from("data"), Value::Array(values));

//         Ok(Value::Object(obj).to_string())
//     } else {
//         Err(AppError::new_server(500, "Not a SELECT statement", "Select RawQuery"))
//     }
// }

/// `params` (ex: 2 params, single and array) format is `param1^title1^type1|param2^title2^[type2]`<br>
/// `ids` (ex: 2 params, single and array) format is `id1|id2a,id2b`<br>
/// 3 type of `type`
/// - Basic ex.`str`
/// - List ex. `(str,I,Internal,E,External)`
/// - System list ex. `(str)`
///
/// Array type ex. `[str]`, `[(str,I,Internal,E,External)]` and `[(str)]`<br>
/// Amount of `?` in SQL must equal to params and ids<br>
/// Array type will duplicate `?` amount as in `ids` separated by `,` ex. `IN (?)` to `IN (?,?)`<br>
/// Return JSON string as<br>
/// `{ "param1": "id1", "param2": ["id2a","id2b"], .., "data": [sql results]}`
pub async fn select_raw_query_to_json_string(
    statement: &str,
    params: &str,
    ids: &str,
    assets: &Arc<Mutex<AppAsset>>,
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
    kphis_extra: &str,
    kphis_log: &str,
) -> Result<String, AppError> {
    if statement.starts_with("SELECT ") {
        let mut obj = serde_json::Map::new();
        let raw_sql = statement
            .replace("__HOSXP__", hosxp)
            .replace("__KPHIS__", kphis)
            .replace("__KPHIS_EXTRA__", kphis_extra)
            .replace("__KPHIS_LOG__", kphis_log);
        let sql_split = raw_sql.split('?').collect::<Vec<&str>>();
        // sql_split.len() always not below 1
        let (sql_split_body, sql_split_tail) = sql_split.split_at(sql_split.len() - 1);
        let sql_tail = sql_split_tail.concat();
        let params_vec = ReportParam::from_cap_pipe(params);
        let ids_vec = ids.split("|").collect::<Vec<&str>>();

        if (sql_split_body.len() != params_vec.len()) || (params_vec.len() != ids_vec.len()) {
            return Err(Source::App.to_error(400, "Params and IDs size not equal", "Select RawQuery"));
        }
        // process `?` in array type
        let ids_split_inner = ids_vec.iter().map(|s| s.split(',').map(str::trim).collect::<Vec<&str>>());
        let question_marks = ids_split_inner.clone().map(|values| vec!["?"; values.len()].join(","));
        let sql = sql_split_body
            .into_iter()
            .zip(question_marks)
            .flat_map(|(s, q)| [s.to_string(), q])
            .chain([sql_tail])
            .collect::<Vec<String>>()
            .concat();
        // process query
        let mut query = sqlx::query(AssertSqlSafe(sql));
        // NOTE: `id_vec.len()` always not below 1, so `Some(v) = values.first()` below always get v
        for (param, id_vec) in params_vec.into_iter().zip(ids_split_inner) {
            let key = &param.id;
            let is_list = param.ty.is_list();
            let key_labels = if is_list {
                let assets_lock = assets.lock().await;
                param.ty.get_items(&assets_lock)
            } else {
                Vec::new()
            };
            match param.ty.get_basic_type() {
                BasicType::Bool => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<bool>())
                        .collect::<Result<Vec<bool>, _>>()
                        .map_err(|e| Source::ParseBool.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Bool(v)).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Bool(*v)
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Int8 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<i8>())
                        .collect::<Result<Vec<i8>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Int16 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<i16>())
                        .collect::<Result<Vec<i16>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Int32 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<i32>())
                        .collect::<Result<Vec<i32>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Int64 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<i64>())
                        .collect::<Result<Vec<i64>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Uint8 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<u8>())
                        .collect::<Result<Vec<u8>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Uint16 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<u16>())
                        .collect::<Result<Vec<u16>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Uint32 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<u32>())
                        .collect::<Result<Vec<u32>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Uint64 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<u64>())
                        .collect::<Result<Vec<u64>, _>>()
                        .map_err(|e| Source::ParseInt.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::Number((v).into())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::Number((*v).into())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Float32 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<f32>())
                        .collect::<Result<Vec<f32>, _>>()
                        .map_err(|e| Source::ParseFloat.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values
                                .into_iter()
                                .map(|v| Number::from_f64(v as f64).map(|n| Value::Number(n)))
                                .collect::<Option<Vec<Value>>>()
                                .ok_or(Source::App.to_error(400, "Infenite or NAN float", "Select RawQuery"))?
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            let json_values = Number::from_f64(*v as f64).ok_or(Source::App.to_error(400, "Infenite or NAN float", "Select RawQuery"))?;
                            Value::Number(json_values)
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Float64 => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<f64>())
                        .collect::<Result<Vec<f64>, _>>()
                        .map_err(|e| Source::ParseFloat.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values
                                .into_iter()
                                .map(|v| Number::from_f64(v as f64).map(|n| Value::Number(n)))
                                .collect::<Option<Vec<Value>>>()
                                .ok_or(Source::App.to_error(400, "Infenite or NAN float", "Select RawQuery"))?
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            let json_values = Number::from_f64(*v as f64).ok_or(Source::App.to_error(400, "Infenite or NAN float", "Select RawQuery"))?;
                            Value::Number(json_values)
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Decimal => {
                    let values = id_vec
                        .iter()
                        .map(|id| id.parse::<Decimal>())
                        .collect::<Result<Vec<Decimal>, _>>()
                        .map_err(|e| Source::Decimal.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values
                                .into_iter()
                                .map(|v| Number::from_f64(v.as_f64()).map(|n| Value::Number(n)))
                                .collect::<Option<Vec<Value>>>()
                                .ok_or(Source::App.to_error(400, "Infenite or NAN Decimal", "Select RawQuery"))?
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            let json_values = Number::from_f64(v.as_f64()).ok_or(Source::App.to_error(400, "Infenite or NAN Decimal", "Select RawQuery"))?;
                            Value::Number(json_values)
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Date => {
                    let values = id_vec
                        .iter()
                        .map(|id| Date::parse(id, &Iso8601::DEFAULT))
                        .collect::<Result<Vec<Date>, _>>()
                        .map_err(|e| Source::Time.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::String(v.to_string())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::String(v.to_string())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Time => {
                    let values = id_vec
                        .iter()
                        .map(|id| Time::parse(id, &Iso8601::DEFAULT))
                        .collect::<Result<Vec<Time>, _>>()
                        .map_err(|e| Source::Time.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::String(v.js_string())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.js_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::String(v.js_string())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::DateTime => {
                    let values = id_vec
                        .iter()
                        .map(|id| PrimitiveDateTime::parse(id, &Iso8601::DEFAULT))
                        .collect::<Result<Vec<PrimitiveDateTime>, _>>()
                        .map_err(|e| Source::Time.to_error(400, e, "Select RawQuery"))?;
                    if param.is_array() {
                        for v in values.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            values.into_iter().map(|v| Value::String(v.js_string())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = values.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.js_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::String(v.js_string())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Str => {
                    if param.is_array() {
                        for v in id_vec.iter() {
                            query = query.bind(v);
                        }
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            id_vec.into_iter().map(|v| Value::String(v.to_string())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = id_vec.first() {
                        query = query.bind(v);
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::String(v.to_string())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
                BasicType::Value => {
                    if param.is_array() {
                        let json_values = if is_list {
                            key_labels
                                .iter()
                                .filter_map(|kl| id_vec.contains(&kl.key.as_str()).then(|| Value::String(kl.label.to_owned())))
                                .collect::<Vec<Value>>()
                        } else {
                            id_vec.into_iter().map(|v| Value::String(v.to_string())).collect()
                        };
                        obj.insert(key.to_owned(), Value::Array(json_values));
                    } else if let Some(v) = id_vec.first() {
                        let json_value = if is_list && let Some(key_label) = key_labels.iter().find(|kl| kl.key == v.to_string()) {
                            Value::String(key_label.label.to_owned())
                        } else {
                            Value::String(v.to_string())
                        };
                        obj.insert(key.to_owned(), json_value);
                    }
                }
            }
        }
        let values = query
            .fetch_all(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Select RawQuery"))?
            .iter()
            .map(row_to_json)
            .collect::<sqlx::Result<Vec<Value>>>()
            .map_err(|e| Source::SQLx.to_error(500, e, "Select RawQuery"))?;
        obj.insert(String::from("data"), Value::Array(values));

        Ok(Value::Object(obj).to_string())
    } else {
        Err(AppError::new_server(500, "Not a SELECT statement", "Select RawQuery"))
    }
}

/// Convert a MySqlRow into a JSON value
fn row_to_json(row: &MySqlRow) -> sqlx::Result<Value> {
    let mut obj = serde_json::Map::new();

    for col in row.columns() {
        let value = match col.type_info().name() {
            // List from `fn name()`` in https://github.com/launchbadge/sqlx/blob/main/sqlx-mysql/src/protocol/text/column.rs
            "NULL" => Some(Value::Null),
            "BOOLEAN" => row.try_get::<Option<bool>, usize>(col.ordinal())?.map(|b| Value::Bool(b)),
            "TINYINT UNSIGNED" | "SMALLINT UNSIGNED" | "INT UNSIGNED" | "MEDIUMINT UNSIGNED" | "BIGINT UNSIGNED" => {
                row.try_get::<Option<u64>, usize>(col.ordinal())?.map(|u| Value::Number(Number::from(u)))
            }
            "TINYINT" | "SMALLINT" | "INT" | "MEDIUMINT" | "BIGINT" | "TIMESTAMP" => row.try_get::<Option<i64>, usize>(col.ordinal())?.map(|i| Value::Number(Number::from(i))),
            "FLOAT" | "DOUBLE" => row.try_get::<Option<f64>, usize>(col.ordinal())?.and_then(|f| Number::from_f64(f)).map(|n| Value::Number(n)),
            "DECIMAL" => row
                .try_get::<Option<Decimal>, usize>(col.ordinal())?
                .and_then(|d| d.to_f64())
                .and_then(|f| Number::from_f64(f))
                .map(|n| Value::Number(n)),
            "BINARY" | "VARBINARY" | "TINYBLOB" | "BLOB" | "MEDIUMBLOB" | "LONGBLOB" => {
                row.try_get::<Option<Vec<u8>>, usize>(col.ordinal())?.map(|b| Value::String(general_purpose::URL_SAFE_NO_PAD.encode(b)))
            }
            "DATE" => row.try_get::<Option<Date>, usize>(col.ordinal())?.map(|d| Value::String(d.to_string())),
            "TIME" => row.try_get::<Option<Time>, usize>(col.ordinal())?.map(|t| Value::String(t.to_string())),
            "DATETIME" => row.try_get::<Option<PrimitiveDateTime>, usize>(col.ordinal())?.map(|dt| Value::String(dt.to_string())),
            // "YEAR","BIT","ENUM","SET",
            // "GEOMETRY","JSON","ENUM",
            // "CHAR","VARCHAR","TINYTEXT","TEXT","MEDIUMTEXT","LONGTEXT",
            _ => row.try_get::<Option<String>, usize>(col.ordinal())?.map(|s| Value::String(s)),
        };
        obj.insert(col.name().to_string(), value.unwrap_or(Value::Null));
    }

    Ok(Value::Object(obj))
}

pub async fn select_report_template_exists(template_name: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<bool, AppError> {
    let sql = report::get_report_template_exists(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(template_name)
        .fetch_one(pool)
        .await
        .map(|row| row.try_get::<bool, usize>(0))
        .map_err(|e| Source::SQLx.to_error(500, e, "Select CustomReportExists"))?
        .map_err(|e| Source::SQLx.to_error(500, e, "Select CustomReportExists"))
}

pub async fn insert_report_template(save: &CustomReport, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let insert_sql = report::insert_report_template(kphis_extra);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.template_name)
        .bind(&save.title)
        .bind(&save.content)
        .bind(&save.statement)
        .bind(&save.statement_params)
        .bind(&save.info)
        .bind(save.disabled)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Insert CustomReport"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert CustomReport"))
}

pub async fn update_report_template(save: &CustomReport, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let update_sql = report::update_report_template(kphis_extra);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.template_name)
        .bind(&save.title)
        .bind(&save.content)
        .bind(&save.statement)
        .bind(&save.statement_params)
        .bind(&save.info)
        .bind(save.disabled)
        .bind(user)
        .bind(save.template_id)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Update CustomReport"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update CustomReport"))
}
pub async fn delete_report_template(template_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = report::delete_report_template(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(template_id)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Delete CustomReport"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete CustomReport"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_report_template_content() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_report_template_content("lab-v4",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(found.is_some());
        let disabled_false = select_report_template_content("lab-v3",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(disabled_false.is_some());
        let disabled_true = select_report_template_content("lab-v2",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(disabled_true.is_none());
        let not_found = select_report_template_content("lab-v8",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_report_template() {
        let tester = MySqlTester::new_hosxp_and_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        let default = select_report_template(&ReportTemplateParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 5);
        let found_template_id = select_report_template(&ReportTemplateParams {template_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_template_id.len(), 1);
        let found_template_name = select_report_template(&ReportTemplateParams {template_name: Some(String::from("lab-v2")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_template_name.len(), 1);
        let found_disabled = select_report_template(&ReportTemplateParams {disabled: Some(true),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_disabled.len(), 1);
        let found_disabled_false = select_report_template(&ReportTemplateParams {disabled: Some(false),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_disabled_false.len(), 4);
        let not_found = select_report_template(&ReportTemplateParams {template_id: Some(9),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_report_template_compact() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        let default = select_report_template_compact(&ReportTemplateParams::default(),&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 5);
        let found_template_id = select_report_template_compact(&ReportTemplateParams {template_id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_template_id.len(), 1);
        let found_template_name = select_report_template_compact(&ReportTemplateParams {template_name: Some(String::from("lab-v2")),..Default::default()},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_template_name.len(), 1);
        let found_disabled = select_report_template_compact(&ReportTemplateParams {disabled: Some(true),..Default::default()},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_disabled.len(), 1);
        let found_disabled_false = select_report_template_compact(&ReportTemplateParams {disabled: Some(false),..Default::default()},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_disabled_false.len(), 4);
        let not_found = select_report_template_compact(&ReportTemplateParams {template_id: Some(9),..Default::default()},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_raw_query_to_json_string() {
        let tester = MySqlTester::new_all().await;
        let app_assets = crate::assets::load_app_asset_from_file("../../volume/app_assets.bin").await.map(|(assets, _)| Arc::new(Mutex::new(assets))).unwrap();

        let not_select = select_raw_query_to_json_string("DELETE * FROM hos","id^ID^str","1", &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log).await;
        assert!(not_select.is_err());

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient_image.sql")).execute(&tester.db_pool).await.unwrap();
        let hosxp_blob = select_raw_query_to_json_string(
            "SELECT * FROM __HOSXP__.patient_image WHERE hn=?;",
            "hn^HN^str", "0001234",
            &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log,
        ).await.unwrap();
        assert_eq!(
            hosxp_blob,
            "{\"data\":[\
                {\"capture_date\":null,\"height\":null,\"hn\":\"0001234\",\"hos_guid\":null,\"hos_guid_ext\":null,\"image\":\"_9j_4AAQSkZJRgABAQEAYABgAAD_2wBDAAIBAQEBAQIBAQECAgICAgQDAgICAgUEBAMEBgUGBgYFBgYGBwkIBgcJBwYGCAsICQoKCgoKBggLDAsKDAkKCgr_2wBDAQICAgICAgUDAwUKBwYHCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgr_wAARCACyAJQDASIAAhEBAxEB_8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL_8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4-Tl5ufo6erx8vP09fb3-Pn6_8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL_8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3-Pn6_9oADAMBAAIRAxEAPwD9_KKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiiigAooooAKKKKACiory9tNPtXvb64SKKNcvI5wAK888U_Gy5kdrTwrbhEHH2qdcsfdV6D8c_QUAekUV4Pf8AirxLqjl7_XLqTP8ACZiF_IcCqsWpajbtvgv5kbOcpKQf0NAH0FRXjOifFLxhozqH1E3cQ6xXXzZ_4F1H516N4N-Imi-L1-zx5t7sDLW0jZz7qf4h-vtQB0FFFFABRRRQAUUUUAFFFFABRRRQAUUUUAFIzKil3YAAZJJ4Apa5f4ua6-j-E3t4H2y3r-SCOoXGW_QY_GgDhfiP47n8V6k1paSlbCB8QoD_AKwj-M_09BXM0UUAFFFFABT7e4ntJ0ubaZo5I2DI6HBUjoRTKKAPZvhx41XxfpBF0QLy2wtwo_i9HH1_nXRV4n8O9dfQPFlrcb8RTOIZx2KscfocH8K9soAKKKKACiiigAooooAKKKKACiiigArzj48zsbnTbbPyhJW_ElR_SvR68--PFi7W-nakq_KjyRufcgEfyNAHnFFFFABRRRQAUUUUAKrMrBlOCDkEV9B2kpntIpz1eNWP4ivALK1lvryKyhGXmkVEHqScCvoGKNYo1iXoqgCgB1FFFABRRRQAUUUUAFFFFABRRRQAVk-N_Dw8T-G7jS0A80rvtyezjkfn0_GtaigD55likhkaGZCroxVlYYII6im16b8TvhpJqsj-IvD8ObjGbm3Uf6z_AGl_2vbv9evmbo8bmORCrKcMrDBBoASiiigAoorX8JeDdW8X3wt7GIrCp_f3LD5Yx_U-1AGz8HfDMmreIP7anj_0ex5BI4aQ_dH4dfwHrXrFU9B0Ow8OaXFpOmx7Y4xyT1c92PuauUAFFFFABRRRQAUUUUAFFFFABRRRQAUUVV1DW9G0r_kJ6rb257CaZVJ_AmgC1WJ4m-H_AIa8UkzX1n5c5H_HzAdr_j2b8RSt8RPBKnB8R2_4En-lH_CxvBH_AEMdv-v-FAHH6h8CdQRydK12GRewuIyhH4jOarRfA3xOz4m1KxVc8kO5P5ba7n_hY3gj_oY7f9f8KP8AhY3gj_oY7f8AX_CgDC0T4IaLaOJtb1CS7I58pF8tPx5JP5iuysrGz062WzsLVIYkGFjjUACsr_hY3gj_AKGO3_X_AAo_4WN4I_6GO3_X_CgDaorKtvHPg-8bZB4jtMk8BpgufzxWpHJHKgkicMrDIZTkGgBaKKKACiiigAooooAKKKKACor29tNOtJL6-nWKGJd0kjHgCpa8r-L_AIwk1TVT4cspSLa0bE20_wCsl759h0-uaAE8Y_F7VtWkey8Ou1pag480cSyD1z_CPpz71x0kkk0hlmkZmY5ZmOSTTaKACiiigAooooAKKKKACtDQfFWveG5xNpGovGM5aInKN9VPFZ9FAHsfgT4jWHi-P7HcILe-RctDnhx6r_h1HvXS18-Wd5c6fdR3tnM0csTho3U8givbvBniWLxX4fh1VQBIfkuEH8Ljr_iPY0AatFFFABRRRQAUUUUAVtYvxpek3OpMM_Z7d5MeuFJrwKWWSeVp5nLO7FmY9yepr27x-xXwXqRU4_0VhXh9ABRRRQAUUUUAFFFFABRRRQAUUUUAFd78CtUdNRvdGZvlkhEyj0KnB_PcPyrgq6z4MMV8aAA9bWQH9KAPXKKKKACiiigAooooAx_iD_yJWpf9erV4fXuHxB_5ErUv-vVq8PoAKKKKACiiigAooooAKKKKACiiigArrPgz_wAjqv8A16yf0rk66z4M_wDI6r_16yf0oA9cooooAKKKKACiiigDH-IP_Ilal_16tXh9e4fEH_kStS_69Wrw-gAooooAKKKKACiiigAooooAKKKKACus-DP_ACOq_wDXrJ_SuTrrPgz_AMjqv_XrJ_SgD1yiiigAooooAKKKKAMf4g_8iVqX_Xq1eH17h8Qf-RK1L_r1avD6ACiiigAooooAKKKKACiiigAooooAK6z4M_8AI6r_ANesn9K5Ous-DP8AyOq_9esn9KAPXKKKKACiiigAooooAx_H_wDyJmpf9erV4ngegoooAMD0FGB6CiigAwPQUYHoKKKADA9BRgegoooAMD0FGB6CiigAwPQUYHoKKKADA9BXVfBwAeM1wP8Al2k_pRRQB61RRRQAUUUUAf_Z\",\"image_name\":\"OPD\",\"width\":null}\
            ],\"hn\":\"0001234\"}",
        );

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_o2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_o2.sql")).execute(&tester.db_pool).await.unwrap();
        let kphis_system_list = select_raw_query_to_json_string(
            "SELECT * FROM __KPHIS__.ipd_vs_o2 WHERE o2_id=?;",
            "o2^O2^(o2)", "1",
            &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log,
        ).await.unwrap();
        assert_eq!(
            kphis_system_list,
            "{\"data\":[{\"o2_id\":1,\"o2_name\":\"Canular\"}],\"o2\":\"Canular\"}",
        );
        let kphis_system_list_many = select_raw_query_to_json_string(
            "SELECT * FROM __KPHIS__.ipd_vs_o2 WHERE o2_id IN (?);",
            "o2^O2^[(o2)]", "1,2,3",
            &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log,
        ).await.unwrap();
        assert_eq!(
            kphis_system_list_many,
            "{\"data\":[\
                {\"o2_id\":1,\"o2_name\":\"Canular\"},\
                {\"o2_id\":2,\"o2_name\":\"Mask c bag\"},\
                {\"o2_id\":3,\"o2_name\":\"Collar\"}\
            ],\"o2\":[\"Canular\",\"Mask c bag\",\"Collar\"]}",
        );

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap();
        let kphis_array_one = select_raw_query_to_json_string(
            "SELECT * FROM __KPHIS__.ipd_io WHERE an IN (?) AND io_date > ?;",
            "an^AN^[str]|io_date^IO date^date", "660001234|2024-01-10",
            &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log,
        ).await.unwrap();
        assert_eq!(
            kphis_array_one,
            "{\"an\":[\"660001234\"],\"data\":[\
                {\"an\":\"660001234\",\"create_datetime\":\"2024-01-01 11:11:11.0\",\"create_user\":\"admin\",\"io_date\":\"2024-01-11\",\"io_id\":2,\"io_oral_absorb\":77,\"io_oral_amount\":77,\"io_oral_carry_forward\":77,\"io_oral_name\":\"Water\",\"io_oral_remark\":null,\"io_output_amount\":77,\"io_output_remark\":\"Remark\",\"io_output_type\":\"urine\",\"io_parenteral_absorb\":77.0,\"io_parenteral_amount\":222.0,\"io_parenteral_carry_forward\":77.0,\"io_parenteral_name\":\"0.9% NSS\",\"io_parenteral_remark\":null,\"io_parenteral_type\":\"iv\",\"io_time\":\"11:11:11.0\",\"update_datetime\":\"2024-01-01 11:11:11.0\",\"update_user\":\"admin\",\"version\":1},\
                {\"an\":\"660001234\",\"create_datetime\":\"2024-01-01 11:11:11.0\",\"create_user\":\"admin\",\"io_date\":\"2024-01-21\",\"io_id\":3,\"io_oral_absorb\":77,\"io_oral_amount\":77,\"io_oral_carry_forward\":77,\"io_oral_name\":\"Water\",\"io_oral_remark\":null,\"io_output_amount\":77,\"io_output_remark\":\"Remark\",\"io_output_type\":\"urine\",\"io_parenteral_absorb\":77.0,\"io_parenteral_amount\":222.0,\"io_parenteral_carry_forward\":77.0,\"io_parenteral_name\":\"0.9% NSS\",\"io_parenteral_remark\":null,\"io_parenteral_type\":\"iv\",\"io_time\":\"11:11:11.0\",\"update_datetime\":\"2024-01-01 11:11:11.0\",\"update_user\":\"admin\",\"version\":1}\
            ],\"io_date\":\"2024-01-10\"}",
        );

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        let kphis_extra_list = select_raw_query_to_json_string(
            "SELECT * FROM __KPHIS_EXTRA__.ipd_dc_plan WHERE an=?;",
            "an^AN^(str,660001234,Mr.A)", "660001234",
            &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log,
        ).await.unwrap();
        assert_eq!(
            kphis_extra_list,
            "{\"an\":\"Mr.A\",\"data\":[\
                {\"an\":\"660001234\",\"appoint_date\":\"2024-01-01\",\"appoint_for\":\"F/U\",\"appoint_place\":\"OPD\",\"appoint_time\":\"11:11:11.0\",\"create_datetime\":\"2024-01-01 11:11:11.0\",\"create_user\":\"user\",\"dc_datetime\":\"2024-01-01 11:11:11.0\",\"dc_plan_id\":1,\"dc_symptom\":\"Good\",\"dc_type_ok\":\"Y\",\"dc_type_other\":null,\"dc_type_refer\":null,\"diet_datetime\":null,\"diet_doctor\":\"007\",\"diet_other\":null,\"diet_patient_ok\":\"Y\",\"diet_relatives_ok\":\"Y\",\"diet_text\":\"diet-text\",\"dx_datetime\":null,\"dx_doctor\":\"007\",\"dx_id\":1,\"dx_other\":null,\"dx_patient_ok\":\"Y\",\"dx_relatives_ok\":\"Y\",\"dx_text\":\"dx-text\",\"env_datetime\":null,\"env_doctor\":\"007\",\"env_other\":null,\"env_patient_ok\":\"Y\",\"env_relatives_ok\":\"Y\",\"env_text\":\"env-text\",\"health_datetime\":null,\"health_doctor\":\"007\",\"health_other\":null,\"health_patient_ok\":\"Y\",\"health_relatives_ok\":\"Y\",\"health_text\":\"health-text\",\"inst_ett\":null,\"inst_foley\":null,\"inst_ng\":null,\"inst_none\":\"Y\",\"inst_other\":null,\"inst_tt\":null,\"med_datetime\":null,\"med_doctor\":\"007\",\"med_other\":null,\"med_patient_ok\":\"Y\",\"med_relatives_ok\":\"Y\",\"med_text\":\"med-text\",\"out_datetime\":null,\"out_doctor\":\"007\",\"out_other\":null,\"out_patient_ok\":\"Y\",\"out_relatives_ok\":\"Y\",\"out_text\":\"out-text\",\"refer_to\":null,\"tx_datetime\":null,\"tx_doctor\":\"007\",\"tx_other\":null,\"tx_patient_ok\":\"Y\",\"tx_relatives_ok\":\"Y\",\"tx_text\":\"tx-text\",\"update_datetime\":\"2024-01-01 11:11:11.0\",\"update_user\":\"user\",\"version\":1,\"with_appoint\":\"Y\",\"with_cert\":\"Y\",\"with_drug\":\"Y\",\"with_other\":null}\
            ]}",
        );

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();
        let kphis_log_array_many = select_raw_query_to_json_string(
            "SELECT * FROM __KPHIS_LOG__.ipt_log WHERE an IN (?) AND ipt_log_type=?;",
            "an^AN^[str]|log_type^Log Type^str", "660001234,990001234|D",
            &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log,
        ).await.unwrap();
        assert_eq!(
            kphis_log_array_many,
            "{\"an\":[\"660001234\",\"990001234\"],\"data\":[\
                {\"an\":\"660001234\",\"create_datetime\":\"2023-12-31 23:59:59.0\",\"hn\":\"0001234\",\"ipt_log_id\":1,\"ipt_log_type\":\"D\",\"vn\":\"661231235959\",\"ward\":\"01\"}\
            ],\"log_type\":\"D\"}",
        );
        let kphis_log_array_list_many = select_raw_query_to_json_string(
            "SELECT * FROM __KPHIS_LOG__.ipt_log WHERE an IN (?) AND ipt_log_type IN (?);",
            "an^AN^[str]|log_type^Log Type^[(str,D,Delete,I,Insert)]", "660001234,990001234|D,I",
            &app_assets,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra,&tester.kphis_log,
        ).await.unwrap();
        assert_eq!(
            kphis_log_array_list_many,
            "{\"an\":[\"660001234\",\"990001234\"],\"data\":[\
                {\"an\":\"660001234\",\"create_datetime\":\"2023-12-31 23:59:59.0\",\"hn\":\"0001234\",\"ipt_log_id\":1,\"ipt_log_type\":\"D\",\"vn\":\"661231235959\",\"ward\":\"01\"},\
                {\"an\":\"660001234\",\"create_datetime\":\"2023-12-31 23:59:59.0\",\"hn\":\"0001234\",\"ipt_log_id\":2,\"ipt_log_type\":\"I\",\"vn\":\"661231235959\",\"ward\":\"01\"}\
            ],\"log_type\":[\"Delete\",\"Insert\"]}",
        );
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_report_template_exists() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_report_template_exists("lab-v3",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(found);
        let not_found = select_report_template_exists("nothing",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(!not_found);
    }


    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_report_template() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_report_template(&CustomReport::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_error_duplicate = insert_report_template(&CustomReport::demo(),"user",&tester.db_pool,&tester.kphis_extra).await;
        assert!(again_error_duplicate.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_report_template() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_report_template(&CustomReport::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_report_template(&CustomReport::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_report_template() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/report_template.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_report_template(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_report_template(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
