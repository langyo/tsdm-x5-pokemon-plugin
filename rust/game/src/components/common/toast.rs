use crate::prelude::*;

use crate::state::{ToastInfo, ToastType, UI_STATE};

#[component]
pub fn ToastProvider() -> Element {
    let toasts = UI_STATE.read().toasts.clone();

    rsx! {
        div { class: "toast-container",
            for toast in toasts {
                ToastItem { toast: toast.clone() }
            }
        }
        div { class: "toast-spacer" }
    }
}

#[cfg(target_arch = "wasm32")]
fn copy_text_to_clipboard(text: String) {
    let js_code = format!(
        r#"(function() {{
            var msg = {};
            if (navigator.clipboard && navigator.clipboard.writeText) {{
                navigator.clipboard.writeText(msg);
            }} else {{
                var ta = document.createElement('textarea');
                ta.value = msg;
                ta.style.position = 'fixed';
                ta.style.left = '-9999px';
                document.body.appendChild(ta);
                ta.select();
                document.execCommand('copy');
                document.body.removeChild(ta);
            }}
        }})();"#,
        serde_json::to_string(&text)
            .unwrap_or_else(|_| format!("\"{}\"", text.replace('"', "\\\"")))
    );
    let _ = js_sys::eval(&js_code);
}

#[component]
fn ToastItem(toast: ToastInfo) -> Element {
    let class_name = match toast.toast_type {
        ToastType::Success => "toast-item success",
        ToastType::Error => "toast-item error",
        ToastType::Info => "toast-item info",
        ToastType::Warning => "toast-item warning",
    };

    let toast_id = toast.id;
    let message = toast.message.clone();

    let copy_to_clipboard = {
        #[cfg(target_arch = "wasm32")]
        let message_for_copy = message.clone();

        move |_| {
            #[cfg(target_arch = "wasm32")]
            {
                copy_text_to_clipboard(message_for_copy.clone());
            }
        }
    };

    rsx! {
        div {
            class: "{class_name}",
            onclick: move |_| crate::state::hide_toast(toast_id),
            span { class: "toast-message", "{message}" }
            div { class: "toast-actions",
                button {
                    class: "toast-copy",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        copy_to_clipboard(evt);
                    },
                    "⧉"
                }
                button { class: "toast-close", "×" }
            }
        }
    }
}
