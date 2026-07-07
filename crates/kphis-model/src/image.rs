pub mod file_path;
pub mod scan_his;

use base64::Engine;
use derive_demo::Demo;
use hayro::vello_cpu::color::palette::css::WHITE;
use image::{DynamicImage, ImageBuffer, ImageFormat, ImageReader, Rgba, imageops::FilterType};
use serde_derive::{Deserialize, Serialize};
use std::{cmp::Ordering, io::Cursor, sync::Arc};
use utoipa::ToSchema;

use kphis_util::error::{AppError, Source};

use crate::{DEFAULT_USER_IMAGE, DEFAULT_WARN_IMAGE, IMAGE_MAX_SIZE_SQUARE, THUMB_MAX_SIZE_SQUARE};

/// Image base64-encoded with size<br>
/// inner image is string that can use in `<img src="xxx">`
#[derive(Clone, Debug, Demo, Deserialize, Serialize, PartialEq, ToSchema)]
#[schema(example = json!(ImageBase64::demo()))]
pub struct ImageBase64 {
    #[Demo(value = r#"String::from("statics/picture/warn.svg")"#)]
    pub image: String,
    #[Demo(value = "128")]
    pub width: u32,
    #[Demo(value = "128")]
    pub height: u32,
}

impl Default for ImageBase64 {
    fn default() -> Self {
        Self::new_user()
    }
}

impl ImageBase64 {
    pub fn new_user() -> Self {
        Self {
            image: String::from(DEFAULT_USER_IMAGE),
            width: 128,
            height: 128,
        }
    }

    pub fn new_warn() -> Self {
        Self {
            image: String::from(DEFAULT_WARN_IMAGE),
            width: 128,
            height: 128,
        }
    }

    pub fn from_bytes(image_bytes: &[u8]) -> Result<Option<Self>, AppError> {
        let result = if image_bytes.is_empty() {
            None
        } else {
            let image_info = get_image_info(image_bytes)?;
            let image_base64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(image_bytes);
            Some(ImageBase64 {
                image: ["data:", image_info.mime_type, ";base64,", &image_base64].concat(),
                width: image_info.width,
                height: image_info.height,
            })
        };

        Ok(result)
    }
}

pub struct ImageInfo {
    pub mime_type: &'static str,
    pub width: u32,
    pub height: u32,
}

pub fn get_image_info(image_bytes: &[u8]) -> Result<ImageInfo, AppError> {
    let dyn_image = image::load_from_memory(image_bytes).map_err(|e| Source::Image.to_teapot_error(e, "IntoDynamicImage"))?;
    let image_format = image::guess_format(image_bytes).map_err(|e| Source::Image.to_teapot_error(e, "GuessImageFormat"))?;
    let info = ImageInfo {
        mime_type: image_format.to_mime_type(),
        width: dyn_image.width(),
        height: dyn_image.height(),
    };

    Ok(info)
}

pub fn pdf_to_image(pdf_bytes: Vec<u8>) -> Result<DynamicImage, AppError> {
    let pdf = hayro_syntax::Pdf::new(Arc::new(pdf_bytes)).map_err(|_e| Source::Hayro.to_teapot_error("Failed to read PDF file", "Parse PDF"))?;
    let cache = hayro::RenderCache::new();
    let pixmap = pdf
        .pages()
        .first()
        .map(|page| {
            hayro::render(
                page,
                &cache,
                &hayro_interpret::InterpreterSettings::default(),
                &hayro::RenderSettings {
                    x_scale: 2.0,
                    y_scale: 2.0,
                    width: None,
                    height: None,
                    bg_color: WHITE,
                },
            )
        })
        .ok_or(Source::Hayro.to_teapot_error("PDF file without any page", "Parse PDF"))?;
    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(pixmap.width() as u32, pixmap.height() as u32, pixmap.data_as_u8_slice().to_vec()).ok_or(Source::Image.to_teapot_error("Failed create ImageBuffer", "Parse PDF"))?;

    Ok(DynamicImage::ImageRgba8(image_buffer))
}

pub fn image_from_raw_image(raw_data: &[u8], file_name: &str) -> Result<DynamicImage, AppError> {
    // 1. guess image format
    let mut image_reader = ImageReader::new(Cursor::new(raw_data))
        .with_guessed_format()
        .map_err(|e| Source::Image.to_teapot_error(&["Error read image: ", &e.to_string()].concat(), "Parse Image"))?;
    if image_reader.format().is_none() {
        // 2. find format from file name
        if let Some(format) = file_name.split(".").last().and_then(|ext| ImageFormat::from_extension(ext)) {
            image_reader.set_format(format);
        } else {
            // 3. try with png
            if image_reader.format().is_none() {
                image_reader.set_format(ImageFormat::Png);
            }
        };
    }

    image_reader
        .decode()
        .map_err(|e| Source::Image.to_teapot_error(&["Error decode image: ", &e.to_string()].concat(), "Parse Image"))
}

/// return (full, thumbnail) bytes of WebP file<br>
/// with limited image size (except image from PDF)
pub fn webp_creator(any_image: DynamicImage, is_from_pdf: bool) -> Result<(Vec<u8>, Vec<u8>), AppError> {
    let raw_w = any_image.width();
    let raw_h = any_image.height();
    let image = if !is_from_pdf && (raw_w > IMAGE_MAX_SIZE_SQUARE || raw_h > IMAGE_MAX_SIZE_SQUARE) {
        // see filters detail at https://docs.rs/image/latest/image/imageops/enum.FilterType.html
        any_image.resize(IMAGE_MAX_SIZE_SQUARE, IMAGE_MAX_SIZE_SQUARE, FilterType::Triangle)
    } else {
        any_image
    };
    let mut res_image = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut res_image), ImageFormat::WebP)
        .map_err(|e| Source::Image.to_teapot_error(&["Error create image: ", &e.to_string()].concat(), "Create Image"))?;

    let img_w = image.width();
    let img_h = image.height();
    let cubic = match img_h.cmp(&img_w) {
        Ordering::Equal => image,
        Ordering::Greater => image.crop_imm(0, (img_h - img_w) / 2, img_w, img_w),
        Ordering::Less => image.crop_imm((img_w - img_h) / 2, 0, img_h, img_h),
    };
    let thumb = cubic.thumbnail(THUMB_MAX_SIZE_SQUARE, THUMB_MAX_SIZE_SQUARE);
    let mut res_thumb = Vec::new();
    thumb
        .write_to(&mut Cursor::new(&mut res_thumb), ImageFormat::WebP)
        .map_err(|e| Source::Image.to_teapot_error(&["Error create thumbnail: ", &e.to_string()].concat(), "Create Image"))?;

    Ok((res_image, res_thumb))
}
