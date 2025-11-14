use dioxus::prelude::*;
use crate::components::MainHomeLayout;

#[component]
pub fn Home() -> Element {
    rsx! {
        MainHomeLayout {}
    }
}
