use crate::prelude::*;

use crate::state::{get_loading_message, use_global_loading};

#[component]
pub fn LoadingOverlay() -> Element {
    let is_loading = use_global_loading();
    let message = get_loading_message();

    if !is_loading {
        return rsx! {};
    }

    rsx! {
        div {
            class: "global-loading-overlay",
            div { class: "loading-spinner" },
            if !message.is_empty() {
                div { class: "loading-message", "{message}" }
            } else {
                div { class: "loading-message", "处理中..." }
            }
        }
    }
}
