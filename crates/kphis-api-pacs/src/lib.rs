pub mod broker;

use axum::body::Bytes;

use kphis_model::pacs::PacsXnData;
use kphis_util::error::AppError;

pub async fn get_xn_data(xn: i32, pacs_client: &reqwest::Client, pacs_host: &str) -> Result<PacsXnData, AppError> {
    broker::BrokerClient::new(pacs_client).get_xn_data(xn, pacs_host).await
}

pub async fn get_thumbnail(file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, pacs_client: &reqwest::Client, pacs_host: &str) -> Result<Bytes, AppError> {
    broker::BrokerClient::new(pacs_client).get_thumbnail(file_path, study_uid, series_uid, object_uid, pacs_host).await
}

pub async fn get_image(file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, pacs_client: &reqwest::Client, pacs_host: &str) -> Result<Bytes, AppError> {
    broker::BrokerClient::new(pacs_client).get_image(file_path, study_uid, series_uid, object_uid, pacs_host).await
}
