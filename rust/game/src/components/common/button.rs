use crate::prelude::*;

#[derive(Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Danger => "btn-danger",
        ButtonVariant::Ghost => "btn-ghost",
    };

    rsx! {
        button {
            class: "btn {variant_class}",
            disabled: props.disabled,
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}
