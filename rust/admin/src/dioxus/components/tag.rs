use crate::dioxus::prelude::*;

use crate::dioxus::components::icon::{Icon, IconName};

#[component]
pub fn Tag(
    label: String,
    disabled: bool,
    on_close: EventHandler<()>,
    class: Option<String>,
) -> Element {
    let mut class_name = "admin-tag".to_string();

    if let Some(extra) = class {
        if !extra.trim().is_empty() {
            class_name.push(' ');
            class_name.push_str(extra.trim());
        }
    }

    rsx! {
        div { class: "{class_name}",
            span { class: "admin-tag__label", "{label}" }
            button {
                class: "admin-tag__close",
                r#type: "button",
                disabled,
                onclick: move |_| on_close.call(()),
                Icon {
                    name: IconName::X,
                    class: None,
                }
            }
        }
    }
}
