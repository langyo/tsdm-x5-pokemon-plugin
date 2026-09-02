use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ErrorPopupData {
    pub message: String,
    pub debug: Option<String>,
}

pub static ERROR_POPUP: GlobalSignal<Option<ErrorPopupData>> = Signal::global(|| None);

#[allow(dead_code)]
pub fn show_error_popup(message: &str, debug: Option<String>) {
    let mut popup = ERROR_POPUP.write();
    *popup = Some(ErrorPopupData {
        message: message.to_string(),
        debug,
    });
}

#[component]
pub fn ErrorPopup() -> Element {
    let error_data = ERROR_POPUP.read();

    if let Some(ref data) = *error_data {
        let full_message = if let Some(ref debug) = data.debug {
            format!("{}\n\n调试信息: {}", data.message, debug)
        } else {
            data.message.clone()
        };

        let close_fn = move |_| {
            let mut popup = ERROR_POPUP.write();
            *popup = None;
        };

        let copy_fn = move |_| {
            let msg = full_message.clone();
            spawn(async move {
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
                    serde_json::to_string(&msg)
                        .unwrap_or_else(|_| format!("\"{}\"", msg.replace('"', "\\\"")))
                );
                let _ = js_sys::eval(&js_code);
            });
        };

        rsx! {
            div { class: "error-popup-overlay",
                div { class: "error-popup-container",
                    div { class: "error-popup-content",
                        div { class: "error-popup-icon", "⚠️" }
                        div { class: "error-popup-message", "{data.message}" }
                    }
                    div { class: "error-popup-actions",
                        button {
                            class: "error-popup-copy-btn",
                            onclick: copy_fn,
                            "复制"
                        }
                        button {
                            class: "error-popup-close-btn",
                            onclick: close_fn,
                            "关闭"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
