use dioxus::prelude::*;
use std::env::{current_dir, var};

#[component]
pub fn TerminalComponent() -> Element {
    let path = current_dir()?;
    let user = if cfg!(windows) {
        var("USERNAME")
    } else {
        var("USER")
    }
    .unwrap_or_else(|_| "user".to_string());

    rsx! {
        div {
            class: "flex m-5",
        }
    }
}
