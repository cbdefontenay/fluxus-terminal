use crate::app_icons::{FolderIcon, TerminalIcon};
use crate::components::home::DesktopIcons;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn MainHomeLayout() -> Element {
    rsx! {
        div { class: "flex flex-row m-4 gap-6",
            Link {
                class: "bg-(--tertiary) rounded-full p-4",
                to: Route::Terminal {},
                DesktopIcons {
                    FolderIcon { class: "w-15 h-15 p-1 rounded-full text-(--on-tertiary)" }
                }
            }
            Link {
                class: "bg-(--error) rounded-full p-4",
                to: Route::Terminal {},
                DesktopIcons {
                    TerminalIcon { class: "w-15 h-15 p-1 rounded-full text-(--on-error)" }
                }
            }
        }
    }
}
