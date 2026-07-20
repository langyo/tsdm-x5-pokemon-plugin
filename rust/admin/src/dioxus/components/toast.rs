use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

use crate::dioxus::{
    components::icon::{Icon, IconName},
    state::{hide_toast, AdminNoticeLevel, ADMIN_TOASTS},
};

#[component]
pub fn AdminToastProvider() -> Element {
    let toasts = ADMIN_TOASTS.read().clone();

    rsx! {
        div { class: "admin-toast-container",
            for toast in toasts {
                AdminToastItem {
                    id: toast.id,
                    level: toast.level,
                    message: toast.message,
                }
            }
        }
    }
}

#[component]
fn AdminToastItem(id: u64, level: AdminNoticeLevel, message: String) -> Element {
    let class_name = match level {
        AdminNoticeLevel::Info => "admin-toast admin-toast--info",
        AdminNoticeLevel::Success => "admin-toast admin-toast--success",
        AdminNoticeLevel::Error => "admin-toast admin-toast--error",
    };
    let message_for_copy = message.clone();
    let mut copied = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if matches!(level, AdminNoticeLevel::Error) {
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
            button {
                class: "admin-toast__copy",
                r#type: "button",
                title: if copied() { "复制成功" } else { "复制" },
                onclick: move |evt| {
                    evt.stop_propagation();
                    #[cfg(target_arch = "wasm32")]
                    {
                        let msg = message_for_copy.clone();
                        let mut c = copied;
                        let js = format!("navigator.clipboard.writeText('{}').then(function(){{}})", msg.replace('\'', "\\'"));
                        let _ = js_sys::eval(&js);
                        c.set(true);
                    }
                },
                if copied() {
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
