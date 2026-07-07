use std::sync::LazyLock;
use syntect::{dumps, highlighting::Theme, parsing::SyntaxSet};

pub static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(|| dumps::from_binary(include_bytes!("syntax_set.dump")));
pub static THEME: LazyLock<Theme> = LazyLock::new(|| dumps::from_binary(include_bytes!("theme.dump")));
