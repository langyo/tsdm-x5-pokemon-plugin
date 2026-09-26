use crate::dioxus::prelude::*;

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
        let js = r#"
            (function retryRenderLucide(remaining) {
                if (window.lucide && typeof window.lucide.createIcons === 'function') {
                    window.lucide.createIcons({ attrs: { 'stroke-width': '2' } });
                    return;
                }
                if (remaining > 0) {
                    setTimeout(function () { retryRenderLucide(remaining - 1); }, 160);
                }
            })(16);
        "#;
        let _ = js_sys::eval(js);
    });

    rsx! {
        i { class: "{class_name}", "data-lucide": "{name.as_lucide_name()}" }
    }
}
