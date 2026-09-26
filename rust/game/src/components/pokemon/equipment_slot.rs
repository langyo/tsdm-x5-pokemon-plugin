use crate::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EquipmentSlotProps {
    #[props(default)]
    pub item_name: Option<String>,
    #[props(default)]
    pub item_icon: Option<String>,
    #[props(default)]
    pub bonus: Option<String>,
}

#[component]
pub fn EquipmentSlot(props: EquipmentSlotProps) -> Element {
    rsx! {
        div { class: "equipment-slot",
            if let Some(name) = props.item_name {
                div { class: "equipment-item",
                    if let Some(icon) = props.item_icon {
                        img { src: "{icon}", alt: "{name}" }
                    }
                    span { "{name}" }
                    if let Some(bonus) = props.bonus {
                        small { class: "equipment-bonus", "{bonus}" }
                    }
                }
            } else {
                div { class: "equipment-empty", "空" }
            }
        }
    }
}
