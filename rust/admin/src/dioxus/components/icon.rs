use crate::dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconName {
    Plus,
    SquarePen,
    Search,
    RotateCcw,
    Check,
    ChevronsDown,
    LoaderCircle,
    Save,
    Copy,
    X,
    CircleCheck,
    CircleX,
    Trash2,
}

impl IconName {
    fn as_lucide_name(self) -> &'static str {
        match self {
            IconName::Plus => "plus",
            IconName::SquarePen => "square-pen",
            IconName::Search => "search",
            IconName::RotateCcw => "rotate-ccw",
            IconName::Check => "check",
            IconName::ChevronsDown => "chevrons-down",
            IconName::LoaderCircle => "loader-circle",
            IconName::Save => "save",
            IconName::Copy => "copy",
            IconName::X => "x",
            IconName::CircleCheck => "circle-check-big",
            IconName::CircleX => "circle-x",
            IconName::Trash2 => "trash-2",
        }
    }
}

#[component]
pub fn Icon(name: IconName, class: Option<String>) -> Element {
    let class_name = class.unwrap_or_else(|| "admin-icon".to_string());

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        render_lucide(16);
    });

    rsx! {
        i { class: "{class_name}", "data-lucide": "{name.as_lucide_name()}" }
    }
}

/// `window.lucide.createIcons({ attrs: { 'stroke-width': '2' } })`。
/// lucide.min.js 尚未加载（或没有 createIcons）时返回 false，由调用方稍后重试。
#[cfg(target_arch = "wasm32")]
fn try_render_lucide() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(lucide) = js_sys::Reflect::get(window.as_ref(), &"lucide".into()) else {
        return false;
    };
    let Ok(create_icons) = js_sys::Reflect::get(&lucide, &"createIcons".into()) else {
        return false;
    };
    let Ok(create_icons) = create_icons.dyn_into::<js_sys::Function>() else {
        return false;
    };
    let attrs = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&attrs, &"stroke-width".into(), &"2".into());
    let options = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&options, &"attrs".into(), &attrs);
    // 以 window.lucide 为 this 调用，不检查返回值
    let _ = create_icons.apply(&lucide, &js_sys::Array::of1(&options));
    true
}

/// lucide.min.js 是普通 `<script>`，可能晚于 Dioxus 渲染就绪：
/// 未就绪时每 160ms 重试一次，最多 `remaining` 次。
#[cfg(target_arch = "wasm32")]
fn render_lucide(remaining: u32) {
    if try_render_lucide() || remaining == 0 {
        return;
    }
    let callback = wasm_bindgen::closure::Closure::once(move || render_lucide(remaining - 1));
    if let Some(window) = web_sys::window() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            160,
        );
    }
    callback.forget();
}
