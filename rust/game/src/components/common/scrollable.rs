use crate::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ScrollableProps {
    children: Element,
    #[props(default = "".to_string())]
    height: String,
    #[props(default = "".to_string())]
    width: String,
    #[props(default = false)]
    horizontal: bool,
    #[props(default = "".to_string())]
    class: String,
}

#[component]
pub fn Scrollable(props: ScrollableProps) -> Element {
    let base_class = if props.horizontal {
        "custom-scrollbar custom-scrollbar-horizontal"
    } else {
        "custom-scrollbar"
    };

    let class_name = if props.class.is_empty() {
        base_class.to_string()
    } else {
        format!("{} {}", base_class, props.class)
    };

    let style = if !props.height.is_empty() || !props.width.is_empty() {
        let mut styles = Vec::new();
        if !props.height.is_empty() {
            styles.push(format!("height: {}", props.height));
        }
        if !props.width.is_empty() {
            styles.push(format!("width: {}", props.width));
        }
        if !props.horizontal {
            styles.push("overflow-y: auto".to_string());
        }
        Some(styles.join("; "))
    } else if props.horizontal {
        Some("overflow-x: auto".to_string())
    } else {
        Some("overflow-y: auto".to_string())
    };

    rsx! {
        div {
            class: class_name,
            style: style,
            {props.children}
        }
    }
}
