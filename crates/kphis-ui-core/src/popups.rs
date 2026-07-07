pub mod confirm;
pub mod dom_with_close;
pub mod prompt_password;
pub mod with_close;

use dominator::class;
use std::sync::LazyLock;

pub static MODAL: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("position","fixed")
        .style("padding-top","100px")
        .style("left","0")
        .style("top","0")
        .style("width", "100%")
        .style("height","100%")
        .style("z-index","1088")
        .style("overflow","auto")
        .style("background-color","rgba(0,0,0,0.5)")
    }
});

pub static MODAL_CONTENT: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("position","relative")
        .style("background-color","var(--bs-body-bg)")
        .style("margin","auto")
        .style("padding","0")
        .style("border","3px solid steelblue")
        .style("border-radius","9px")
        .style("max-width","1024px")
    }
});

pub static MODAL_CONTENT_ALERT: LazyLock<String> = LazyLock::new(|| {
    class! {
        .style("position","relative")
        .style("background-color","var(--bs-body-bg)")
        .style("margin","auto")
        .style("padding","0")
        .style("border","3px solid red")
        .style("border-radius","9px")
        .style("max-width","1024px")
        .style("color","red")
    }
});

#[derive(Clone, PartialEq)]
pub enum PopupAuth {
    /// pwd, totp
    Ok(String, String),
    Cancel,
}

#[derive(Clone, PartialEq)]
pub enum PopupOkCancel {
    Ok,
    Cancel,
}
