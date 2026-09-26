use crate::dioxus::prelude::*;
use dioxus_web::WebEventExt;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::dioxus::{
    components::icon::{Icon, IconName},
    state::{hide_tooltip, show_tooltip},
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Small,
    ExtraSmall,
}

#[component]
pub fn IconButton(
    icon: IconName,
    tooltip: String,
    disabled: bool,
    onclick: EventHandler<()>,
    class: Option<String>,
    size: Option<ButtonSize>,
) -> Element {
    let size = size.unwrap_or_default();
    let mut class_name =
        "admin-btn admin-config-bottom-bar__icon-btn admin-filter-action-btn admin-icon-button"
            .to_string();

    match size {
        ButtonSize::Small => {
            class_name.push_str(" admin-icon-button--small");
        }
        ButtonSize::ExtraSmall => {
            class_name.push_str(" admin-icon-button--xs");
        }
        ButtonSize::Default => {}
    }

    if let Some(extra) = class {
        if !extra.trim().is_empty() {
            class_name.push(' ');
            class_name.push_str(extra.trim());
        }
    }

    let tooltip_text = tooltip.clone();

    rsx! {
        button {
            class: "{class_name}",
            title: "{tooltip}",
            aria_label: "{tooltip}",
            disabled,
            onclick: move |_| onclick.call(()),
            onmouseenter: move |evt: Event<MouseData>| {
                // 获取触发事件的元素
                if let Some(web_evt) = evt.try_as_web_event() {
                    if let Some(target) = web_evt.current_target() {
                        let element: &HtmlElement = target.unchecked_ref();
                        let rect = element.get_bounding_client_rect();
                        // 计算tooltip位置：按钮底部居中
                        let x = rect.left() + rect.width() / 2.0;
                        let y = rect.bottom();
                        show_tooltip(tooltip_text.clone(), x, y);
                    }
                }
            },
            onmouseleave: move |_| {
                hide_tooltip();
            },
            Icon {
                name: icon,
                class: "admin-config-bottom-bar__icon".to_string(),
            }
        }
    }
}
