use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

/// 复制文本到剪贴板：优先走 navigator.clipboard，不可用时回退 execCommand。
/// 返回值只反映调用是否发出：writeText 的 Promise 不等待，异步失败不上报。
pub fn copy_to_clipboard(value: &str) -> Result<(), String> {
    let Some(window) = web_sys::window() else {
        return Err("剪贴板调用失败: 无法获取 window".to_string());
    };
    // 不等待 Promise，异步失败（如权限被拒）静默
    if let Some(clipboard) = navigator_clipboard(&window) {
        let promise = clipboard.write_text(value);
        wasm_bindgen_futures::spawn_local(async move {
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        });
        return Ok(());
    }
    copy_with_exec_command(&window, value).map_err(|error| format!("剪贴板调用失败: {:?}", error))
}

/// `navigator.clipboard`：非安全上下文（http 页面）下不存在，而 web-sys 的 getter
/// 不返回 Option，所以经 Reflect 读取再判类型。
fn navigator_clipboard(window: &web_sys::Window) -> Option<web_sys::Clipboard> {
    let clipboard = js_sys::Reflect::get(window.navigator().as_ref(), &"clipboard".into()).ok()?;
    clipboard.dyn_into::<web_sys::Clipboard>().ok()
}

/// 旧式回退：临时 textarea + execCommand("copy")，供没有 navigator.clipboard 的环境使用。
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
