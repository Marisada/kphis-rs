use bitcode::{Decode, Encode};
use derive_demo::Demo;
use serde_derive::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

/// HTML select element option with color
#[derive(Clone, Debug, Demo, Decode, Encode, Hash, PartialEq, Serialize, ToSchema)]
#[schema(example = json!(ColorSelectOption::demo()))]
pub struct ColorSelectOption {
    #[Demo(value = r#"String::from("1")"#)]
    pub key: String,
    #[Demo(value = r#"String::from("Item1")"#)]
    pub value: String,
    #[Demo(value = r##"String::from("#888888")"##)]
    pub color: String,
}

/// HTML select element option without color
#[derive(Clone, Debug, Demo, Decode, Encode, FromRow, Hash, PartialEq, Serialize, ToSchema)]
#[schema(example = json!(SelectOption::demo()))]
pub struct SelectOption {
    #[Demo(value = r#"String::from("1")"#)]
    pub key: String,
    #[Demo(value = r#"String::from("Item1")"#)]
    pub value: String,
}

impl std::convert::From<&ColorSelectOption> for SelectOption {
    fn from(item: &ColorSelectOption) -> Self {
        Self {
            key: item.key.to_owned(),
            value: item.value.to_owned(),
        }
    }
}
