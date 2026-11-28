use crate::prelude::*;

use crate::state::{ToastInfo, ToastType, UI_STATE};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};

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

/// 复制文本到剪贴板：优先走 navigator.clipboard，不可用时回退 execCommand。
#[cfg(target_arch = "wasm32")]
fn copy_text_to_clipboard(text: String) {
    let Some(window) = web_sys::window() else {
        return;
    };
    // 不等待 Promise，异步失败（如权限被拒）静默
    if let Some(clipboard) = navigator_clipboard(&window) {
        let promise = clipboard.write_text(&text);
        wasm_bindgen_futures::spawn_local(async move {
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        });
        return;
    }
    let _ = copy_with_exec_command(&window, &text);
}

/// `navigator.clipboard`：非安全上下文（http 页面）下不存在，而 web-sys 的 getter
/// 不返回 Option，所以经 Reflect 读取再判类型。
#[cfg(target_arch = "wasm32")]
fn navigator_clipboard(window: &web_sys::Window) -> Option<web_sys::Clipboard> {
    let clipboard = js_sys::Reflect::get(window.navigator().as_ref(), &"clipboard".into()).ok()?;
    clipboard.dyn_into::<web_sys::Clipboard>().ok()
}

/// 旧式回退：临时 textarea + execCommand("copy")，供没有 navigator.clipboard 的环境使用。
#[cfg(target_arch = "wasm32")]
fn copy_with_exec_command(window: &web_sys::Window, value: &str) -> Result<(), JsValue> {
    let document = window.document().ok_or("no document")?;
    let textarea = document
        .create_element("textarea")?
        .dyn_into::<web_sys::HtmlTextAreaElement>()?;
    textarea.set_value(value);
    // 移出可视区域，避免页面滚动位置跳动
    let style = textarea.style();
    style.set_property("position", "fixed")?;
    style.set_property("left", "-9999px")?;
    let body = document.body().ok_or("no body")?;
    let _ = body.append_child(&textarea)?;
    textarea.select();
    let html_document = document
        .dyn_ref::<web_sys::HtmlDocument>()
        .ok_or("no HTML document")?;
    // 不检查 execCommand 的返回值
    let _ = html_document.exec_command("copy");
    let _ = body.remove_child(&textarea)?;
    Ok(())
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
