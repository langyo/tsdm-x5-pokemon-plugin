use crate::prelude::*;

/// 技能属性标签组件
/// 显示技能的属性（如火、水、草等）和分类（物攻、特攻、变化）
#[component]
pub fn TypeBadge(
    /// 技能属性（火、水、草、电、超能等）
    #[props(default = "".to_string())]
    skill_type: String,
    /// 技能分类（物攻、特攻、变化）
    #[props(default = "".to_string())]
    category: String,
    /// 额外的 class
    #[props(default = None)]
    class: Option<String>,
) -> Element {
    let type_class = match skill_type.to_lowercase().as_str() {
        "火" | "fire" => "badge-type-火",
        "水" | "water" => "badge-type-水",
        "草" | "grass" => "badge-type-草",
        "电" | "electric" => "badge-type-电",
        "超能" | "psychic" => "badge-type-超能",
        "冰" | "ice" => "badge-type-冰",
        "龙" | "dragon" => "badge-type-龙",
        "恶" | "dark" => "badge-type-恶",
        "妖精" | "fairy" => "badge-type-妖精",
        "格斗" | "fighting" => "badge-type-格斗",
        "飞行" | "flying" => "badge-type-飞行",
        "地面" | "ground" => "badge-type-地面",
        "岩石" | "rock" => "badge-type-岩石",
        "虫" | "bug" => "badge-type-虫",
        "幽灵" | "ghost" => "badge-type-幽灵",
        "钢" | "steel" => "badge-type-钢",
        "毒" | "poison" => "badge-type-毒",
        "普通" | "normal" => "badge-type-普通",
        _ => "badge-type-未知",
    };

    let category_class = match category.to_lowercase().as_str() {
        "物攻" | "物理" | "physical" => "badge-cat-物攻",
        "特攻" | "特殊" | "special" => "badge-cat-特攻",
        "变化" | "status" => "badge-cat-变化",
        _ => "badge-cat-其他",
    };

    rsx! {
        div {
            class: "skill-type-badges {class.clone().unwrap_or_default()}",
            if !skill_type.is_empty() {
                span {
                    class: "badge badge-skill-type {type_class}",
                    "{skill_type}"
                }
            }
            if !category.is_empty() {
                span {
                    class: "badge badge-category {category_class}",
                    "{category}"
                }
            }
        }
    }
}
