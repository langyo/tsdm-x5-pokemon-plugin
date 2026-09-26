use crate::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconButtonProps {
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub tooltip: Option<String>,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    let mut class_name = "icon-button".to_string();
    if let Some(extra) = &props.class {
        if !extra.trim().is_empty() {
            class_name.push(' ');
            class_name.push_str(extra.trim());
        }
    }
    let tooltip = props.tooltip.unwrap_or_default();

    rsx! {
        button {
            class: "{class_name}",
            disabled: props.disabled,
            title: "{tooltip}",
            aria_label: "{tooltip}",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}
