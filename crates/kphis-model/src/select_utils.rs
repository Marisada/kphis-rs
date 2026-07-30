use bitcode::{Decode, Encode};
use derive_demo::Demo;
use serde_derive::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

/// HTML select element option with color
#[derive(Clone, Debug, Demo, Decode, Encode, Hash, PartialEq, Serialize, ToSchema)]
#[schema(example = json!(ColorSelectOption::demo(String::from("1"), String::from("Item1"))))]
pub struct ColorSelectOption {
    pub key: String,
    pub value: String,
    #[Demo(value = r##"String::from("#888888")"##)]
    pub color: String,
}

/// HTML select element option without color
#[derive(Clone, Debug, Demo, Decode, Encode, FromRow, Hash, PartialEq, Serialize, ToSchema)]
#[schema(example = json!(SelectOption::demo(String::from("1"), String::from("Item1"))))]
pub struct SelectOption {
    pub key: String,
    pub value: String,
}
