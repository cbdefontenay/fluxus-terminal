use dioxus::prelude::*;
use crate::components::TerminalComponent;

#[component]
pub fn Terminal() -> Element {
    rsx! {
        TerminalComponent {}
    }
}
