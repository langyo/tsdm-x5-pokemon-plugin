use wasm_bindgen::prelude::*;

use crate::dioxus::app::App;

// 导入 launch 函数
use dioxus::launch;

#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {}

#[wasm_bindgen]
impl WebHandle {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        if let Err(e) = console_log::init_with_level(log::Level::Debug) {}
        Self {}
    }

    #[wasm_bindgen]
    pub async fn start(&self) -> Result<(), wasm_bindgen::JsValue> {
        // 使用 dioxus::launch 启动应用
        launch(App);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn destroy(&self) {}

    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        false
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        None
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        None
    }
}
