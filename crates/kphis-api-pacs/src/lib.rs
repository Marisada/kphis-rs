pub mod agent_api;
pub mod synapse;

use axum::body::Bytes;

use kphis_model::pacs::{PacsConfig, PacsXnData};
use kphis_util::error::AppError;

pub async fn get_xn_data(xn: i32, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<PacsXnData, AppError> {
    if pacs_config.pacs_host_is_kphis_broker {
        agent_api::get_xn_data_inner(xn, pacs_client, pacs_config).await
    } else {
        synapse::api::get_xn_data_inner(xn, pacs_client, pacs_config).await
    }
}

pub async fn get_thumbnail(file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<Bytes, AppError> {
    if pacs_config.pacs_host_is_kphis_broker {
        agent_api::get_thumbnail_inner(file_path, study_uid, series_uid, object_uid, pacs_client, pacs_config).await
    } else {
        synapse::api::get_thumbnail_inner(file_path, study_uid, pacs_client, pacs_config).await
    }
}

pub async fn get_image(file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<Bytes, AppError> {
    if pacs_config.pacs_host_is_kphis_broker {
        agent_api::get_image_inner(file_path, study_uid, series_uid, object_uid, pacs_client, pacs_config).await
    } else {
        synapse::api::get_image_inner(study_uid, series_uid, object_uid, pacs_client, pacs_config).await
    }
}
