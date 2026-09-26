use crate::prelude::*;

use crate::components::common::Badge;

#[derive(Props, Clone, PartialEq)]
pub struct SkillCardProps {
    pub name: String,
    pub skill_type: String,
    pub category: String,
    pub power: Option<u32>,
    pub pp: u32,
    pub level: u32,
}

#[component]
pub fn SkillCard(props: SkillCardProps) -> Element {
    rsx! {
        div { class: "skill-card",
            div { class: "skill-header",
                h4 { "{props.name}" }
                Badge {
                    badge_type: Default::default(),
                    label: props.skill_type.clone(),
                }
            }

            div { class: "skill-info",
                span { "类别: {props.category}" }
                if let Some(power) = props.power {
                    span { "威力: {power}" }
                }
                span { "PP: {props.pp}" }
                span { "等级: {props.level}" }
            }
        }
    }
}
