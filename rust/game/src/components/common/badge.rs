use crate::prelude::*;

#[derive(Clone, PartialEq, Default, Debug)]
pub enum BadgeType {
    #[default]
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub badge_type: BadgeType,
    pub label: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let type_class = format!("badge-{:?}", props.badge_type).to_lowercase();

    rsx! {
        span { class: "badge {type_class}", "{props.label}" }
    }
}
