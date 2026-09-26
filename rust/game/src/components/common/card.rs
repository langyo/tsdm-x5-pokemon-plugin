use crate::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default)]
    pub title: Option<String>,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let extra_class = props.class.unwrap_or_default();

    rsx! {
        div { class: "card {extra_class}",
            if let Some(title) = props.title {
                div { class: "card-header",
                    h3 { "{title}" }
                }
            }

            div { class: "card-body",
                {props.children}
            }
        }
    }
}
