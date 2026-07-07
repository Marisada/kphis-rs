pub mod file_path;
pub mod patient;
pub mod scan_his;

use sqlx::{Row, mysql::MySqlRow};

use kphis_model::image::ImageBase64;
use kphis_util::error::{AppError, Source};

pub fn image_from_row(row: &MySqlRow) -> Result<Option<ImageBase64>, AppError> {
    let image_bytes: Option<Vec<u8>> = row.try_get("image").map_err(|e| Source::SQLx.to_error(500, e, "ParseImage"))?;
    if let Some(bytes) = image_bytes { ImageBase64::from_bytes(&bytes) } else { Ok(None) }
}
