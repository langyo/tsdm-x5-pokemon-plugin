use crate::prelude::*;

use crate::{components::common::Card, utils::api_client::NewApiClient};

#[component]
pub fn PokemonDetail(id: u64) -> Element {
    let mut pokemon_detail = use_resource(move || async move {
        let api = NewApiClient::new();
        api.get_pokemon_detail(id).await
    });

    let mut renaming = use_signal(|| false);
    let mut new_name = use_signal(String::new);
    let mut message = use_signal(|| None::<String>);
    let mut error = use_signal(|| None::<String>);
    let mut confirmation = use_signal(|| None::<String>);
    let mut released = use_signal(|| false); // 追踪是否已放生

    // 读取数据避免生命周期问题
    let detail_to_display = pokemon_detail
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    rsx! {
        div { class: "page-pokemon-detail",
            Card { title: "宠物详情".to_string(),
                // 消息提示
                if let Some(msg) = message.read().clone() {
                    div { class: "success-message",
                        "{msg}"
                        if *released.read() {
                            a { class: "back-link", "返回宝可梦列表" }
                        }
                    }
                }

                // 错误提示
                if let Some(err) = error.read().clone() {
                    div { class: "error-message", "{err}" }
                }

                // 确认提示
                if let Some(conf) = confirmation.read().clone() {
                    div { class: "confirmation-dialog",
                        p { "{conf}" }
                        div { class: "confirmation-actions",
                            button { onclick: move |_| confirmation.set(None), "取消" }
                        }
                    }
                }

                // 加载中
                if pokemon_detail.read().is_none() {
                    div { class: "loading", "加载中..." }
                } else if let Some(detail) = detail_to_display {
                    div { class: "detail-content",
                        // 基本信息
                        div { class: "basic-info",
                            h2 { "{detail.nickname.clone().unwrap_or(detail.name.clone())}" }
                            p { "No. {detail.id}" }
                            p { "等级: Lv.{detail.level}" }
                            p { "经验值: {detail.exp}" }
                            p { "HP: {detail.hp}/{detail.max_hp}" }
                            p { "性别: {detail.gender}" }
                            if detail.is_shiny {
                                p { class: "shiny-badge", "★ 稀有" }
                            }
                        }

                        // 属性显示
                        div { class: "stats-display",
                            h3 { "能力值" }
                            p { "HP: {detail.stats.hp}" }
                            p { "攻击: {detail.stats.attack}" }
                            p { "防御: {detail.stats.defense}" }
                            p { "特攻: {detail.stats.sp_attack}" }
                            p { "特防: {detail.stats.sp_defense}" }
                            p { "速度: {detail.stats.speed}" }
                        }

                        // 技能列表
                        div { class: "skills-section",
                            h3 { "技能" }
                            {
                                detail
                                    .skills
                                    .into_iter()
                                    .enumerate()
                                    .map(|(idx, skill)| {
                                        rsx! {
                                            div { key: "{idx}", class: "skill-card",
                                                p { "技能ID: {skill.type_id}" }
                                                p { "PP: {skill.pp}" }
                                            }
                                        }
                                    })
                            }
                        }

                        // 基础信息
                        div { class: "base-info",
                            h3 { "种族值信息" }
                            p { "名称: {detail.base_info.name}" }
                            p { "属性1: {detail.base_info.type_1}" }
                            if let Some(type2) = &detail.base_info.type_2 {
                                p { "属性2: {type2}" }
                            }
                            if let Some(desc) = &detail.base_info.description {
                                p { "描述: {desc}" }
                            }
                        }

                        // 操作按钮
                        div { class: "actions",
                            button {
                                onclick: move |_| {
                                    new_name.set(String::new());
                                    renaming.set(true);
                                },
                                disabled: *renaming.read(),
                                "重命名"
                            }
                            button {
                                onclick: move |_| {
                                    let pokemon_id = id;
                                    spawn(async move {
                                        let api = NewApiClient::new();
                                        match api.release_pokemon(pokemon_id).await {
                                            Ok(_) => {
                                                message.set(Some("已放生".to_string()));
                                                released.set(true); // 标记已放生
                                            }
                                            Err(e) => {
                                                error.set(Some(format!("放生失败: {}", e)));
                                            }
                                        }
                                    });
                                },
                                class: "danger-button",
                                "放生"
                            }
                        }

                        // 重命名表单
                        if *renaming.read() {
                            div { class: "rename-form",
                                input {
                                    value: "{new_name.read()}",
                                    oninput: move |evt| new_name.set(evt.value()),
                                    placeholder: "新名称",
                                }
                                button {
                                    onclick: move |_| {
                                        let name = new_name.read().clone();
                                        if name.is_empty() {
                                            error.set(Some("名称不能为空".to_string()));
                                            return;
                                        }
                                        let pokemon_id = id;
                                        spawn(async move {
                                            let api = NewApiClient::new();
                                            match api.rename_pokemon(pokemon_id, &name).await {
                                                Ok(_) => {
                                                    message.set(Some("重命名成功".to_string()));
                                                    // 重新加载详情
                                                    pokemon_detail.restart();
                                                }
                                                Err(e) => {
                                                    error.set(Some(format!("重命名失败: {}", e)));
                                                }
                                            }
                                            renaming.set(false);
                                        });
                                    },
                                    "确认"
                                }
                                button { onclick: move |_| renaming.set(false), "取消" }
                            }
                        }
                    }
                }
            }
        }
    }
}
