use dioxus::prelude::*;

#[component]
pub fn DesktopIcons(children: Element) -> Element {
    rsx!{
        div { class: "rounded-full", {children} }
    }
}