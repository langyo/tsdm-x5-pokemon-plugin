use crate::dioxus::prelude::*;

use crate::dioxus::{components::tooltip::TooltipContainer, state::AdminRoute};

#[component]
pub fn AdminLayout(
    current: AdminRoute,
    on_navigate: EventHandler<AdminRoute>,
    busy: bool,
    children: Element,
) -> Element {
    let main_class = match current {
        AdminRoute::GlobalConfig | AdminRoute::SqlConsole => "admin-scrollable",
        _ => "",
    };

    let main_routes: &[(AdminRoute, &str)] = &[
        (AdminRoute::GlobalConfig, "全局配置"),
        (AdminRoute::PokemonData, "宠物数据"),
        (AdminRoute::ItemData, "道具数据"),
        (AdminRoute::MapData, "地图设定"),
        (AdminRoute::UserData, "用户数据"),
        (AdminRoute::EvolutionData, "进化路线"),
        (AdminRoute::SkillType, "技能数据"),
    ];

    rsx! {
        div {
            class: "admin-shell",
            style: "--admin-sidebar-width:192px;display:flex;flex-direction:column;height:80vh;font-family:'Microsoft YaHei',sans-serif;",
            header { style: "padding:10px 16px;background:#2f6aa0;color:#fff;font-size:20px;font-weight:700;text-align:center;user-select:none;",
                div { style: "display:flex;align-items:center;justify-content:center;gap:12px;user-select:none;",
                    span { style: "user-select:none;", "天使动漫 · 宠物插件后台" }
                }
            }
            div { style: "display:flex;flex:1;min-height:0;min-width:0;",
                aside { style: "flex:0 0 192px;width:192px;background:#f4f7fb;border-right:1px solid #d8e0ea;padding:12px 0;overflow:auto;display:flex;flex-direction:column;justify-content:space-between;align-items:stretch;",
                    div { style: "width:100%;display:flex;flex-direction:column;align-items:stretch;padding:0 14px;box-sizing:border-box;",
                        for (route , label) in main_routes.iter() {
                            button {
                                style: if *route == current { "display:block;width:100%;margin:4px 0;padding:8px 6px;border:1px solid #2f6aa0;background:#2f6aa0;color:#fff;text-align:center;cursor:pointer;font-size:13px;" } else { "display:block;width:100%;margin:4px 0;padding:8px 6px;border:1px solid #c7d4e6;background:#fff;color:#234;text-align:center;cursor:pointer;font-size:13px;" },
                                onclick: move |_| on_navigate.call(*route),
                                "{label}"
                            }
                        }
                    }
                }
                main {
                    class: "{main_class}",
                    style: "flex:1 1 auto;padding:16px;min-height:0;min-width:0;",
                    {children}
                }
            }
            // 全屏加载覆盖层
            if busy {
                div {
                    class: "admin-fullscreen-overlay",
                    div {
                        class: "admin-fullscreen-overlay__spinner"
                    }
                }
            }
            // 全局Tooltip容器 - 渲染在根级别，不受层叠上下文影响
            TooltipContainer {}
        }
    }
}
