use axum::body::Bytes;
use reqwest::{Method, header};
use serde_json::Value;

use kphis_model::pacs::{PacsConfig, PacsXnData};
use kphis_util::error::{AppError, Source};

pub async fn get_xn_data_inner(xn: i32, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<PacsXnData, AppError> {
    BrokerClient::new(pacs_client).get_xn_data(xn, pacs_config).await
}

pub async fn get_thumbnail_inner(file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<Bytes, AppError> {
    BrokerClient::new(pacs_client).get_thumbnail(file_path, study_uid, series_uid, object_uid, pacs_config).await
}

pub async fn get_image_inner(file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, pacs_client: &reqwest::Client, pacs_config: &PacsConfig) -> Result<Bytes, AppError> {
    BrokerClient::new(pacs_client).get_image(file_path, study_uid, series_uid, object_uid, pacs_config).await
}

pub struct BrokerClient {
    pub client: reqwest::Client,
}

impl BrokerClient {
    fn new(pacs_client: &reqwest::Client) -> Self {
        Self { client: pacs_client.clone() }
    }

    async fn get_xn_data(&self, xn: i32, config: &PacsConfig) -> Result<PacsXnData, AppError> {
        let res = self
            .client
            .request(Method::GET, &[&config.pacs_host, "/xn/", &xn.to_string()].concat())
            .header(header::CONTENT_TYPE, "application/json;charset=utf-8")
            .send()
            .await
            .map_err(|e| Source::PacsClient.to_error(500, e, "GetXnData"))?;

        let status = res.status().as_u16();
        if status == 200 {
            res.json::<PacsXnData>().await.map_err(|e| Source::PacsClient.to_error(500, e, "GetXnData"))
        } else {
            let body = res.json::<Value>().await.map_err(|e| Source::PacsClient.to_error(500, e, "GetXnData"))?;
            Err(Source::PacsClient.to_error(500, ["PACsBroker return status:", &status.to_string(), ", message: ", &body.to_string()].concat(), "GetXnData"))
        }
    }

    async fn get_thumbnail(&self, file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, config: &PacsConfig) -> Result<Bytes, AppError> {
        let res = self
            .client
            .request(
                Method::GET,
                &[
                    &config.pacs_host,
                    "/thumbnail?file_path=",
                    file_path,
                    "&study_uid=",
                    study_uid,
                    "&series_uid=",
                    series_uid,
                    "&object_uid=",
                    object_uid,
                ]
                .concat(),
            )
            .header(header::CONTENT_TYPE, "image/jpeg")
            .send()
            .await
            .map_err(|e| Source::PacsClient.to_error(500, e, "GetPACsThumbnail"))?;

        let status = res.status().as_u16();
        if status == 200 {
            let body = res.bytes().await.unwrap_or(Bytes::new());
            if body.is_empty() {
                Err(Source::PacsClient.to_error(500, "PACsBroker cannot read all body", "GetPACsThumbnail"))
            } else {
                Ok(body)
            }
        } else {
            let body = res.json::<Value>().await.map_err(|e| Source::PacsClient.to_error(500, e, "GetPACsThumbnail"))?;
            Err(Source::PacsClient.to_error(500, ["PACsBroker return status:", &status.to_string(), ", message: ", &body.to_string()].concat(), "GetPACsThumbnail"))
        }
    }

    async fn get_image(&self, file_path: &str, study_uid: &str, series_uid: &str, object_uid: &str, config: &PacsConfig) -> Result<Bytes, AppError> {
        let res = self
            .client
            .request(
                Method::GET,
                &[
                    &config.pacs_host,
                    "/image?file_path=",
                    file_path,
                    "&study_uid=",
                    study_uid,
                    "&series_uid=",
                    series_uid,
                    "&object_uid=",
                    object_uid,
                ]
                .concat(),
            )
            .header(header::CONTENT_TYPE, "image/jpeg")
            .send()
            .await
            .map_err(|e| Source::PacsClient.to_error(500, e, "GetPACsImage"))?;

        let status = res.status().as_u16();
        if status == 200 {
            let body = res.bytes().await.unwrap_or(Bytes::new());
            if body.is_empty() {
                Err(Source::PacsClient.to_error(500, "PACsBroker cannot read all body", "GetPACsImage"))
            } else {
                Ok(body)
            }
        } else {
            let body = res.json::<Value>().await.map_err(|e| Source::PacsClient.to_error(500, e, "GetPACsImage"))?;
            Err(Source::PacsClient.to_error(500, ["PACsBroker return status:", &status.to_string(), ", message: ", &body.to_string()].concat(), "GetPACsImage"))
        }
    }
}
