//! 技能属性 / 分类徽章（与 Dioxus 版 `components/common/type_badge.rs` 同一张
//! 对照表）。个人中心的技能卡、战斗页的技能按钮、宠物 tooltip 共用这一处。
//!
//! 对照表是纯数据（可在 host 上测）；标记是 hikari `Badge` 带着旧类名
//! （`crate::badge`）——和 Dioxus 的 `TypeBadge` 组件一样：`div.skill-type-badges`
//! 里属性、分类各一枚，空串的那枚不渲染。

use tairitsu_macros::rsx;
use tairitsu_vdom::VNode;

/// 技能属性 → 徽章类（表里没有的算「未知」）。
pub fn skill_type_badge_class(skill_type: &str) -> String {
    let name = match skill_type.to_lowercase().as_str() {
        "火" | "fire" => "火",
        "水" | "water" => "水",
        "草" | "grass" => "草",
        "电" | "electric" => "电",
        "超能" | "psychic" => "超能",
        "冰" | "ice" => "冰",
        "龙" | "dragon" => "龙",
        "恶" | "dark" => "恶",
        "妖精" | "fairy" => "妖精",
        "格斗" | "fighting" => "格斗",
        "飞行" | "flying" => "飞行",
        "地面" | "ground" => "地面",
        "岩石" | "rock" => "岩石",
        "虫" | "bug" => "虫",
        "幽灵" | "ghost" => "幽灵",
        "钢" | "steel" => "钢",
        "毒" | "poison" => "毒",
        "普通" | "normal" => "普通",
        _ => "未知",
    };
    format!("badge-type-{name}")
}

/// 技能分类 → 徽章类（其余算「其他」）。
pub fn skill_category_badge_class(category: &str) -> String {
    let name = match category.to_lowercase().as_str() {
        "物攻" | "物理" | "physical" => "物攻",
        "特攻" | "特殊" | "special" => "特攻",
        "变化" | "status" => "变化",
        _ => "其他",
    };
    format!("badge-cat-{name}")
}

/// `div.skill-type-badges[{extra_class}]`：属性、分类各一枚。`extra_class`
/// 是 Dioxus 版 `TypeBadge` 的 `class` 属性（tooltip 用 `type-badge-sm`）。
pub fn type_badge(skill_type: &str, category: &str, extra_class: &str) -> VNode {
    let mut cls = String::from("skill-type-badges");
    if !extra_class.is_empty() {
        cls.push(' ');
        cls.push_str(extra_class);
    }
    let show_type = !skill_type.is_empty();
    let show_category = !category.is_empty();
    let type_cls = format!(
        "badge badge-skill-type {}",
        skill_type_badge_class(skill_type)
    );
    let cat_cls = format!(
        "badge badge-category {}",
        skill_category_badge_class(category)
    );
    let skill_type = skill_type.to_string();
    let category = category.to_string();
    // 徽章是 hikari `Badge` 带着旧类名（`crate::badge`）；空串的那枚不出节点。
    let mut badges: Vec<VNode> = Vec::with_capacity(2);
    if show_type {
        badges.push(crate::badge::legacy(type_cls, skill_type));
    }
    if show_category {
        badges.push(crate::badge::legacy(cat_cls, category));
    }
    rsx! {
        div { class: cls, ..badges }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_table_accepts_both_languages_and_falls_back_to_unknown() {
        assert_eq!(skill_type_badge_class("火"), "badge-type-火");
        assert_eq!(skill_type_badge_class("Ground"), "badge-type-地面");
        assert_eq!(skill_type_badge_class(""), "badge-type-未知");
        assert_eq!(skill_type_badge_class(" ??? "), "badge-type-未知");
    }

    #[test]
    fn category_table_maps_the_aliases() {
        assert_eq!(skill_category_badge_class("物理"), "badge-cat-物攻");
        assert_eq!(skill_category_badge_class("特殊"), "badge-cat-特攻");
        assert_eq!(skill_category_badge_class("status"), "badge-cat-变化");
        assert_eq!(skill_category_badge_class("Heal"), "badge-cat-其他");
    }
}
