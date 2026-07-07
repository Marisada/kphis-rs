pub mod claml;
pub mod index;

use std::sync::LazyLock;

pub static I10_DETAIL: LazyLock<claml::I10Claml> = LazyLock::new(|| claml::I10Claml::new());
pub static I10_INDEX: LazyLock<index::I10Index> = LazyLock::new(|| index::I10Index::new());
