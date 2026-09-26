use crate::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatsRadarProps {
    pub attack: u32,
    pub defense: u32,
    pub sp_attack: u32,
    pub sp_defense: u32,
    pub speed: u32,
    pub hp: u32,
    pub level: u32,
}

#[component]
pub fn StatsRadar(props: StatsRadarProps) -> Element {
    // Simplified stats display (radar chart could be implemented with SVG or Canvas)
    rsx! {
        div { class: "stats-radar",
            h4 { "能力值 (Lv.{props.level})" }

            div { class: "stat-bars",
                div { class: "stat-bar",
                    span { class: "stat-label", "HP:" }
                    div { class: "stat-progress",
                        div { class: "stat-fill", style: "width: {props.hp}%" }
                    }
                    span { class: "stat-value", "{props.hp}" }
                }

                div { class: "stat-bar",
                    span { class: "stat-label", "攻击:" }
                    div { class: "stat-progress",
                        div {
                            class: "stat-fill",
                            style: "width: {props.attack}%",
                        }
                    }
                    span { class: "stat-value", "{props.attack}" }
                }

                div { class: "stat-bar",
                    span { class: "stat-label", "防御:" }
                    div { class: "stat-progress",
                        div {
                            class: "stat-fill",
                            style: "width: {props.defense}%",
                        }
                    }
                    span { class: "stat-value", "{props.defense}" }
                }

                div { class: "stat-bar",
                    span { class: "stat-label", "特攻:" }
                    div { class: "stat-progress",
                        div {
                            class: "stat-fill",
                            style: "width: {props.sp_attack}%",
                        }
                    }
                    span { class: "stat-value", "{props.sp_attack}" }
                }

                div { class: "stat-bar",
                    span { class: "stat-label", "特防:" }
                    div { class: "stat-progress",
                        div {
                            class: "stat-fill",
                            style: "width: {props.sp_defense}%",
                        }
                    }
                    span { class: "stat-value", "{props.sp_defense}" }
                }

                div { class: "stat-bar",
                    span { class: "stat-label", "速度:" }
                    div { class: "stat-progress",
                        div {
                            class: "stat-fill",
                            style: "width: {props.speed}%",
                        }
                    }
                    span { class: "stat-value", "{props.speed}" }
                }
            }
        }
    }
}
