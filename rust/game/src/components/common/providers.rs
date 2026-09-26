use crate::prelude::*;

use crate::{components::common::ToastProvider, state::APP_STATE};

#[component]
pub fn AppProviders(children: Element) -> Element {
    rsx! {
        ToastProvider {}
        LoadingOverlay {}
        {children}
    }
}

#[component]
fn LoadingOverlay() -> Element {
    let app_state = APP_STATE.read();

    if app_state.loading {
        rsx! {
            div { class: "loading-overlay",
                div { class: "loading-spinner" }
                if !app_state.loading_message.is_empty() {
                    span { class: "loading-message", "{app_state.loading_message}" }
                }
            }
        }
    } else {
        rsx! {}
    }
}
