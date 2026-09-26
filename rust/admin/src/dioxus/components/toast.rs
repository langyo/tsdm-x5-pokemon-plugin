use crate::dioxus::prelude::*;
use std::collections::HashSet;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

use crate::dioxus::{
    components::icon::{Icon, IconName},
    state::{hide_toast, AdminNoticeLevel, ADMIN_TOASTS},
};

static COPIED_TOASTS: GlobalSignal<HashSet<u64>> = Signal::global(|| HashSet::new());

#[component]
pub fn AdminToastProvider() -> Element {
    let toasts = ADMIN_TOASTS.read().clone();
    let copied_ids = COPIED_TOASTS.read().clone();

    rsx! {
        div { class: "admin-toast-container",
            for toast in toasts {
                AdminToastItem {
                    key: "{toast.id}-{copied_ids.contains(&toast.id)}",
                    id: toast.id,
                    level: toast.level,
                    message: toast.message,
                    is_copied: copied_ids.contains(&toast.id),
                }
            }
        }
    }
}

#[component]
fn AdminToastItem(id: u64, level: AdminNoticeLevel, message: String, is_copied: bool) -> Element {
    let class_name = match level {
        AdminNoticeLevel::Info => "admin-toast admin-toast--info",
        AdminNoticeLevel::Success => "admin-toast admin-toast--success",
        AdminNoticeLevel::Error => "admin-toast admin-toast--error",
    };
    let is_error = matches!(level, AdminNoticeLevel::Error);
    let message_for_copy = message.clone();

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if is_error {
            return;
        }
        let callback = wasm_bindgen::closure::Closure::once(move || {
            hide_toast(id);
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                3000,
            );
        }
        callback.forget();
    });

    rsx! {
        div { class: "{class_name}", onclick: move |_| hide_toast(id),
            span { class: "admin-toast__message", "{message}" }
            if is_error {
                button {
                    class: "admin-toast__copy",
                    r#type: "button",
                    title: if is_copied { "已复制" } else { "复制错误信息" },
                    onclick: move |evt| {
                        evt.stop_propagation();
                        #[cfg(target_arch = "wasm32")]
                        {
                            let msg = message_for_copy.clone();
                            COPIED_TOASTS.write().insert(id);
                            if let Some(window) = web_sys::window() {
                                let navigator = window.navigator();
                                let promise = navigator.clipboard().write_text(&msg);
                                wasm_bindgen_futures::spawn_local(async move {
                                    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                });
                            }
                        }
                    },
                    if is_copied {
                        Icon {
                            name: IconName::Check,
                            class: "admin-icon admin-icon--sm".to_string(),
                        }
                    } else {
                        Icon {
                            name: IconName::Copy,
                            class: "admin-icon admin-icon--sm".to_string(),
                        }
                    }
                }
            }
            button {
                class: "admin-toast__close",
                r#type: "button",
                onclick: move |evt| {
                    evt.stop_propagation();
                    hide_toast(id);
                },
                Icon {
                    name: IconName::X,
                    class: "admin-icon admin-icon--sm".to_string(),
                }
            }
        }
    }
}
