use crate::app_icons::FolderIcon;
use crate::components::home::DesktopIcons;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn MainHomeLayout() -> Element {
    rsx! {
        div { class: "flex flex-row m-4 gap-6",
            Link {
                class: "bg-(--primary)",
                to: Route::Home {},
                DesktopIcons {
                    FolderIcon { class: "w-15 h-15 p-1 rounded-full text-(--on-primary)" }
                }
            }
        }
    }
}
