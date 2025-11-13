use dioxus::prelude::*;
use crate::app_icons::FolderIcon;
use crate::components::home::DesktopIcons;

#[component]
pub fn MainHomeLayout() -> Element {
    rsx!{
        div {
            class:"flex flex-row m-4 gap-6",
            DesktopIcons {
                FolderIcon { class: "w-15 h-15 bg-(--primary) p-1 rounded-full text-(--on-primary)" }
            }
            DesktopIcons {
                FolderIcon { class: "w-15 h-15 bg-(--primary) p-1 rounded-full text-(--on-primary)" }
            }
        }
    }
}