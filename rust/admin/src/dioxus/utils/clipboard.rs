/// 复制文本到剪贴板：优先走 navigator.clipboard，不可用时回退 execCommand。
/// 文本经 serde_json 转义成 JS 字面量再注入，CRLF、U+2028 等特殊字符
/// 不会破坏脚本（此前手写转义漏掉 \r，复制多行 SQL 会静默失败）。
pub fn copy_to_clipboard(value: &str) -> Result<(), String> {
    let literal = serde_json::to_string(value).map_err(|e| format!("文本序列化失败: {}", e))?;
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
        literal
    );
    js_sys::eval(&js_code)
        .map(|_| ())
        .map_err(|e| format!("剪贴板调用失败: {:?}", e))
}
