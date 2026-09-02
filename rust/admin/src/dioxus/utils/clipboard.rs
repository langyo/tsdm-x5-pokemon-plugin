pub fn copy_to_clipboard(value: &str) {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    let js_code = format!(r#"navigator.clipboard.writeText(\"{}\");"#, escaped);
    if let Err(error) = js_sys::eval(js_code.as_str()) {}
}
