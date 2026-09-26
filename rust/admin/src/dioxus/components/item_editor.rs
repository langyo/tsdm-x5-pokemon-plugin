use strum::IntoEnumIterator;

use crate::dioxus::prelude::*;

use super::form_fields::{
    BoolField, Col, FormSection, NumberField, Row, SelectField, TextField, UnsignedNumberField,
};
use crate::dioxus::pages::shared::ActionModal;
use _utils::types::{
    item_type::{ItemArmorSite, ItemEffect, ItemTag, ItemType},
    pokemon_type::PokemonKind,
};

/// 道具图片路径前缀
const IMG_PATH: &str = "source/plugin/pokemon/pokemon_system/images";

/// 获取所有 PokemonKind 选项
fn pokemon_kind_options() -> Vec<(String, String)> {
    let mut options = vec![("无".to_string(), "无".to_string())];
    options.extend(PokemonKind::iter().map(|k| (k.to_string(), k.to_string())));
    options
}

/// 道具类型精确编辑器 Modal
#[component]
pub fn ItemTypeEditorModal(
    title: String,
    data: ItemType,
    save_text: String,
    disabled: bool,
    on_save: EventHandler<ItemType>,
    on_close: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| data.clone());
    let img_name = draft.read().img_name.clone();
    let img_src = format!("{}/item/{}.gif", IMG_PATH, img_name);

    rsx! {
        ActionModal { title, on_close,
            div { class: "admin-form-editor",
                // 基本信息
                FormSection { title: "基本信息".to_string(),
                    // 图片预览
                    if !img_name.is_empty() {
                        div { class: "admin-form-pokemon-preview",
                            img {
                                class: "admin-form-item-sprite",
                                src: "{img_src}",
                                alt: "{img_name}",
                            }
                        }
                    }
                    Row {
                        Col { span: 4,
                            UnsignedNumberField {
                                label: "ID".to_string(),
                                value: draft.read().id,
                                min: Some(0),
                                disabled: true,
                                on_change: move |_| {},
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 8,
                            TextField {
                                label: "名称".to_string(),
                                value: draft.read().name.clone(),
                                placeholder: Some("道具名称".to_string()),
                                disabled,
                                on_change: move |v| draft.write().name = v,
                                help: None,
                            }
                        }
                    }
                    Row {
                        Col { span: 6,
                            TextField {
                                label: "图片文件名".to_string(),
                                value: draft.read().img_name.clone(),
                                placeholder: Some("图片文件名（不含扩展名）".to_string()),
                                disabled,
                                on_change: move |v| draft.write().img_name = v,
                                help: None,
                            }
                        }
                        Col { span: 6,
                            TextField {
                                label: "描述".to_string(),
                                value: draft.read().description.clone(),
                                placeholder: Some("道具描述".to_string()),
                                disabled,
                                on_change: move |v| draft.write().description = v,
                                help: None,
                            }
                        }
                    }
                }

                // 商店设置
                FormSection { title: "商店设置".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "价格".to_string(),
                                value: draft.read().price,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().price = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            BoolField {
                                label: "可购买".to_string(),
                                value: draft.read().is_selling,
                                disabled,
                                on_change: move |v| draft.write().is_selling = v,
                                help: None,
                            }
                        }
                    }
                }

                // 道具类型（两列）
                FormSection { title: "道具类型".to_string(),
                    Row {
                        Col { span: 6,
                            ItemTagTypeSelect {
                                value: draft.read().tag.clone(),
                                disabled,
                                on_change: move |v| draft.write().tag = v,
                            }
                        }
                        Col { span: 6,
                            ItemTagExtraField {
                                value: draft.read().tag.clone(),
                                disabled,
                                on_change: move |v| draft.write().tag = v,
                            }
                        }
                    }
                }

                // 使用限制
                FormSection { title: "使用限制".to_string(),
                    Row {
                        Col { span: 6,
                            UnsignedNumberField {
                                label: "最低等级".to_string(),
                                value: draft.read().limits.min_level,
                                min: Some(0),
                                disabled,
                                on_change: move |v| draft.write().limits.min_level = v,
                                help: None,
                                step: None,
                                max: None,
                            }
                        }
                        Col { span: 6,
                            SelectField {
                                label: "种族限制".to_string(),
                                value: draft.read().limits.kind_require
                                    .map(|k| k.to_string())
                                    .unwrap_or_else(|| "无".to_string()),
                                options: pokemon_kind_options(),
                                disabled,
                                on_change: move |v: String| {
                                    if v == "无" {
                                        draft.write().limits.kind_require = None;
                                    } else if let Ok(parsed) = PokemonKind::try_from(v.as_str()) {
                                        draft.write().limits.kind_require = Some(parsed);
                                    }
                                },
                                help: None,
                            }
                        }
                    }
                }

                // 道具效果
                FormSection { title: "道具效果".to_string(),
                    ItemEffectEditor {
                        value: draft.read().effects,
                        disabled,
                        on_change: move |v| draft.write().effects = v,
                    }
                }

                // 操作按钮
                div { class: "admin-form-actions",
                    button {
                        class: "admin-btn",
                        disabled,
                        onclick: move |_| on_close.call(()),
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn--primary",
                        disabled,
                        onclick: move |_| on_save.call(draft.read().clone()),
                        "{save_text}"
                    }
                }
            }
        }
    }
}

/// 道具类型选择器
#[component]
fn ItemTagTypeSelect(value: ItemTag, disabled: bool, on_change: EventHandler<ItemTag>) -> Element {
    let tag_type = match &value {
        ItemTag::Drug => "drug",
        ItemTag::Ball(_) => "ball",
        ItemTag::Evolution(_) => "evolution",
        ItemTag::Enhance => "enhance",
        ItemTag::Armor(_) => "armor",
        ItemTag::Special(_) => "special",
    };

    let tag_options: Vec<(String, String)> = vec![
        ("drug".to_string(), "药品".to_string()),
        ("ball".to_string(), "宠物球".to_string()),
        ("evolution".to_string(), "升级素材".to_string()),
        ("enhance".to_string(), "强化道具".to_string()),
        ("armor".to_string(), "装备".to_string()),
        ("special".to_string(), "特殊物品".to_string()),
    ];

    rsx! {
        SelectField {
            label: "类型".to_string(),
            value: tag_type.to_string(),
            options: tag_options,
            disabled,
            on_change: move |v: String| {
                let new_tag = match v.as_str() {
                    "drug" => ItemTag::Drug,
                    "ball" => ItemTag::Ball(0),
                    "evolution" => ItemTag::Evolution(0),
                    "enhance" => ItemTag::Enhance,
                    "armor" => ItemTag::Armor(ItemArmorSite::Head),
                    "special" => ItemTag::Special(String::new()),
                    _ => ItemTag::Drug,
                };
                on_change.call(new_tag);
            },
            help: None,
        }
    }
}

/// 道具类型额外字段
#[component]
fn ItemTagExtraField(value: ItemTag, disabled: bool, on_change: EventHandler<ItemTag>) -> Element {
    let armor_site_options: Vec<(String, String)> = vec![
        ("head".to_string(), "头部".to_string()),
        ("necklace".to_string(), "饰品".to_string()),
        ("weapon".to_string(), "武器".to_string()),
        ("armor".to_string(), "衣服".to_string()),
    ];

    match &value {
        ItemTag::Ball(id) => rsx! {
            UnsignedNumberField {
                label: "宠物球 ID".to_string(),
                value: *id,
                min: Some(0),
                disabled,
                on_change: move |v| on_change.call(ItemTag::Ball(v)),
                help: None,
                step: None,
                max: None,
            }
        },
        ItemTag::Evolution(id) => rsx! {
            UnsignedNumberField {
                label: "进化素材 ID".to_string(),
                value: *id,
                min: Some(0),
                disabled,
                on_change: move |v| on_change.call(ItemTag::Evolution(v)),
                help: None,
                step: None,
                max: None,
            }
        },
        ItemTag::Armor(site) => rsx! {
            SelectField {
                label: "装备部位".to_string(),
                value: match site {
                    ItemArmorSite::Head => "head",
                    ItemArmorSite::Necklace => "necklace",
                    ItemArmorSite::Weapon => "weapon",
                    ItemArmorSite::Armor => "armor",
                }.to_string(),
                options: armor_site_options,
                disabled,
                on_change: move |v: String| {
                    let new_site = match v.as_str() {
                        "head" => ItemArmorSite::Head,
                        "necklace" => ItemArmorSite::Necklace,
                        "weapon" => ItemArmorSite::Weapon,
                        "armor" => ItemArmorSite::Armor,
                        _ => ItemArmorSite::Head,
                    };
                    on_change.call(ItemTag::Armor(new_site));
                },
                help: None,
            }
        },
        ItemTag::Special(text) => rsx! {
            TextField {
                label: "特殊物品标识".to_string(),
                value: text.clone(),
                placeholder: Some("特殊物品文本标识".to_string()),
                disabled,
                on_change: move |v| on_change.call(ItemTag::Special(v)),
                help: None,
            }
        },
        _ => rsx! {
            div { class: "admin-field",
                div { class: "admin-field__body",
                    span { class: "admin-text-muted", "无额外配置" }
                }
            }
        },
    }
}

/// 道具效果编辑器
#[component]
fn ItemEffectEditor(
    value: ItemEffect,
    disabled: bool,
    on_change: EventHandler<ItemEffect>,
) -> Element {
    let mut effect = value;

    rsx! {
        div { class: "admin-form-effects",
            // 基础效果（两列两行）
            div { class: "admin-form-effects-group",
                h5 { class: "admin-form-effects-group__title", "基础效果" }
                Row {
                    Col { span: 6,
                        NumberField {
                            label: "恢复 HP".to_string(),
                            value: effect.add_hit_points,
                            disabled,
                            on_change: move |v| {
                                effect.add_hit_points = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                    Col { span: 6,
                        NumberField {
                            label: "增加经验".to_string(),
                            value: effect.add_experience,
                            disabled,
                            on_change: move |v| {
                                effect.add_experience = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                }
                Row {
                    Col { span: 6,
                        NumberField {
                            label: "增加等级".to_string(),
                            value: effect.add_level,
                            disabled,
                            on_change: move |v| {
                                effect.add_level = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                    Col { span: 6,
                        NumberField {
                            label: "增加好感".to_string(),
                            value: effect.add_intimacy,
                            disabled,
                            on_change: move |v| {
                                effect.add_intimacy = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                }
            }

            // 属性加成（三列两行）
            div { class: "admin-form-effects-group",
                h5 { class: "admin-form-effects-group__title", "属性加成" }
                Row {
                    Col { span: 4,
                        NumberField {
                            label: "HP+".to_string(),
                            value: effect.attribute_add_hit_points,
                            disabled,
                            on_change: move |v| {
                                effect.attribute_add_hit_points = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                    Col { span: 4,
                        NumberField {
                            label: "攻击+".to_string(),
                            value: effect.attribute_add_attack,
                            disabled,
                            on_change: move |v| {
                                effect.attribute_add_attack = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                    Col { span: 4,
                        NumberField {
                            label: "防御+".to_string(),
                            value: effect.attribute_add_defense,
                            disabled,
                            on_change: move |v| {
                                effect.attribute_add_defense = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                }
                Row {
                    Col { span: 4,
                        NumberField {
                            label: "特攻+".to_string(),
                            value: effect.attribute_add_special_attack,
                            disabled,
                            on_change: move |v| {
                                effect.attribute_add_special_attack = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                    Col { span: 4,
                        NumberField {
                            label: "特防+".to_string(),
                            value: effect.attribute_add_special_defense,
                            disabled,
                            on_change: move |v| {
                                effect.attribute_add_special_defense = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                    Col { span: 4,
                        NumberField {
                            label: "速度+".to_string(),
                            value: effect.attribute_add_speed,
                            disabled,
                            on_change: move |v| {
                                effect.attribute_add_speed = v;
                                on_change.call(effect);
                            },
                            help: None,
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                }
            }

            // 其他效果（三列一行）
            div { class: "admin-form-effects-group",
                h5 { class: "admin-form-effects-group__title", "其他效果" }
                Row {
                    Col { span: 4,
                        NumberField {
                            label: "捕获率修正".to_string(),
                            value: effect.capture,
                            disabled,
                            on_change: move |v| {
                                effect.capture = v;
                                on_change.call(effect);
                            },
                            help: Some("用于精灵球".to_string()),
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                    Col { span: 4,
                        // 恢复技能槽 PP 用 switch 控制
                        BoolField {
                            label: "恢复技能 PP".to_string(),
                            value: effect.restore_pp_skill_slot.is_some(),
                            disabled,
                            on_change: move |v| {
                                effect.restore_pp_skill_slot = if v { Some(0) } else { None };
                                on_change.call(effect);
                            },
                            help: Some("是否恢复技能 PP".to_string()),
                        }
                    }
                    Col { span: 4,
                        NumberField {
                            label: "恢复 PP 数量".to_string(),
                            value: effect.restore_pp_amount,
                            disabled: disabled || effect.restore_pp_skill_slot.is_none(),
                            on_change: move |v| {
                                effect.restore_pp_amount = v;
                                on_change.call(effect);
                            },
                            help: Some("负数为百分比".to_string()),
                            max: None,
                            min: None,
                            step: None,
                        }
                    }
                }
            }
        }
    }
}
