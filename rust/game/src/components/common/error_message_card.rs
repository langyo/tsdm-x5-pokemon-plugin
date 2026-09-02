use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorMessageCardProps {
    pub message: String,
    #[props(default = None)]
    pub debug: Option<String>,
}

#[component]
pub fn ErrorMessageCard(props: ErrorMessageCardProps) -> Element {
    let mut copied = use_signal(|| false);

    let full_message = if let Some(ref debug) = props.debug {
        format!("{}\n\n调试信息: {}", props.message, debug)
    } else {
        props.message.clone()
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
            if js_sys::eval(&js_code).is_ok() {
                copied.set(true);
                TimeoutFuture::new(2000).await;
                copied.set(false);
            }
        });
    };

    rsx! {
        div { class: "error-message-card",
            div { class: "error-message-content",
                span { "⚠️" }
                span { "{props.message}" }
            }
            button {
                class: "error-copy-btn",
                onclick: copy_fn,
                if *copied.read() { "已复制" } else { "复制" }
            }
        }
    }
}
