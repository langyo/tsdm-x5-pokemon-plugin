//! 战斗与地图数据类型（纯数据，host 可测）。
//!
//! 移植自 `rust/utils/src/types/api_battle.rs` / `api_map.rs` 的显示子集。
//! 与 Dioxus 版的差异：
//! - `BattleScene` 不用手写 Visitor——`battle_over`/`can_continue_switch`
//!   走 `#[serde(default)]`，语义回退放在 `effective_battle_over()`。
//! - `MapInfo` 按服务端的扁平载荷（`mode` 为字符串 + `bosses` 数组）直接
//!   反序列化，不用 Dioxus 的 tag 展开。
//! - 请求幂等 ID 用「时间戳+计数器」，不引入 uuid 依赖。

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// 战斗
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattleSkill {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub power: u64,
    #[serde(default)]
    pub pp: u64,
    #[serde(default)]
    pub max_pp: u64,
    #[serde(default)]
    pub skill_type: String,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattlePokemon {
    #[serde(default)]
    pub id: u64,
    /// 数据库唯一 ID（pm_mypm.id），换宠/替换判定用。
    #[serde(default)]
    pub instance_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub level: u64,
    #[serde(default)]
    pub hp: i64,
    #[serde(default)]
    pub max_hp: i64,
    #[serde(default)]
    pub is_shiny: bool,
    #[serde(default)]
    pub skills: Vec<BattleSkill>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WildPokemon {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub level: u64,
    #[serde(default)]
    pub hp: i64,
    #[serde(default)]
    pub max_hp: i64,
    #[serde(default)]
    pub gender: u8,
    #[serde(default)]
    pub is_shiny: bool,
    #[serde(default)]
    pub is_boss: bool,
    #[serde(default)]
    pub boss_multiplier: f32,
}

/// 战斗状态（服务端小写字符串）。
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BattleStatus {
    #[default]
    Active,
    Victory,
    Defeat,
    Fled,
    Captured,
}

impl BattleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            BattleStatus::Active => "active",
            BattleStatus::Victory => "victory",
            BattleStatus::Defeat => "defeat",
            BattleStatus::Fled => "fled",
            BattleStatus::Captured => "captured",
        }
    }

    pub fn result_title(self) -> &'static str {
        match self {
            BattleStatus::Victory => "战斗胜利！",
            BattleStatus::Captured => "捕捉成功！",
            BattleStatus::Defeat => "战斗失败...",
            BattleStatus::Fled => "成功逃脱！",
            BattleStatus::Active => "战斗结束",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattleRewards {
    #[serde(default)]
    pub exp: u64,
    #[serde(default)]
    pub money: u64,
    #[serde(default)]
    pub money_today: u64,
    #[serde(default)]
    pub money_daily_limit: u64,
    #[serde(default)]
    pub money_notes: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LevelUpInfo {
    #[serde(default)]
    pub level_up: bool,
    #[serde(default)]
    pub old_level: u64,
    #[serde(default)]
    pub new_level: u64,
    #[serde(default)]
    pub new_exp: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattleScene {
    #[serde(default)]
    pub battle_id: String,
    #[serde(default)]
    pub engine_battle_id: u64,
    #[serde(default)]
    pub revision: u64,
    #[serde(default)]
    pub phase: String,
    #[serde(default)]
    pub events: Vec<serde_json::Value>,
    #[serde(default)]
    pub map_id: u64,
    #[serde(default)]
    pub map_name: String,
    #[serde(default)]
    pub turn: u64,
    #[serde(default)]
    pub my_pokemon: BattlePokemon,
    #[serde(default)]
    pub wild_pokemon: WildPokemon,
    #[serde(default)]
    pub status: BattleStatus,
    #[serde(default)]
    pub message: String,
    /// 服务端显式给出的结束标记；旧服务端缺省（None）时按 status 推断。
    #[serde(default)]
    pub battle_over: Option<bool>,
    #[serde(default)]
    pub can_continue_switch: bool,
    #[serde(default)]
    pub rewards: Option<BattleRewards>,
    #[serde(default)]
    pub level_up: Option<LevelUpInfo>,
}

impl BattleScene {
    /// 旧服务端不回 battle_over 时按 status 推断（Active 之外视为已结束）；
    /// 显式 false 优先（我方倒下但有替补时 status=defeat 而 over=false）。
    pub fn effective_battle_over(&self) -> bool {
        match self.battle_over {
            Some(v) => v,
            None => self.status != BattleStatus::Active,
        }
    }

    /// 宠物倒下但战斗继续（有替补）：应弹换宠而非终局。
    pub fn needs_replacement(&self) -> bool {
        self.can_continue_switch && !self.effective_battle_over()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PpRestoreSkill {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub skill_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub current_pp: u64,
    #[serde(default)]
    pub max_pp: u64,
}

/// PP 恢复道具的技能选择响应。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SkillSelection {
    #[serde(default)]
    pub item_id: u64,
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub available_skills: Vec<PpRestoreSkill>,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub engine_battle_id: u64,
    #[serde(default)]
    pub revision: u64,
}

/// 战斗道具（物品 id 是种类 ID，非背包行 ID）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattleItem {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub img: String,
    #[serde(default)]
    pub nums: i64,
    #[serde(default)]
    pub item_type: u64,
    #[serde(default)]
    pub module: String,
    #[serde(default)]
    pub addhp: i64,
}

impl BattleItem {
    pub fn is_pp_restore(&self) -> bool {
        matches!(self.module.as_str(), "pp5" | "pp10" | "pp15" | "pp99")
    }

    pub fn can_use_on(&self, pokemon: &BattlePokemon) -> bool {
        if self.nums <= 0 || pokemon.hp <= 0 {
            return false;
        }
        if self.is_pp_restore() {
            pokemon.skills.iter().any(|skill| skill.pp < skill.max_pp)
        } else {
            pokemon.hp < pokemon.max_hp
        }
    }

    /// 显示描述（对齐 Dioxus 版 battle_item_for_card）。
    pub fn display_description(&self) -> String {
        if self.module == "pp99" {
            "完全恢复一个技能的PP".to_string()
        } else if self.is_pp_restore() {
            format!(
                "恢复一个技能的 {} 点PP",
                self.module.trim_start_matches("pp")
            )
        } else {
            format!("恢复 {} HP", self.addhp)
        }
    }

    pub fn type_label(&self) -> &'static str {
        if self.is_pp_restore() {
            "PP恢复"
        } else {
            "HP恢复"
        }
    }
}

// ---------------------------------------------------------------------------
// 地图
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BossInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boss_index: Option<u64>,
    #[serde(default)]
    pub pokemon_type_id: u64,
    #[serde(default)]
    pub pokemon_name: String,
    #[serde(default)]
    pub level: u64,
    #[serde(default)]
    pub boss_multiplier: f32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WildPokemonInfo {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MapInfo {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub area_type: String,
    #[serde(default)]
    pub area_type_name: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub pos_x: u32,
    #[serde(default)]
    pub pos_y: u32,
    #[serde(default)]
    pub is_enabled: bool,
    #[serde(default)]
    pub min_level: u64,
    #[serde(default)]
    pub max_level: u64,
    #[serde(default)]
    pub min_quality: u8,
    #[serde(default)]
    pub min_quality_name: String,
    /// "wild" | "boss" | "hybrid"。
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub bosses: Vec<BossInfo>,
    #[serde(default)]
    pub wild_pokemons: Vec<WildPokemonInfo>,
}

impl MapInfo {
    pub fn has_wild(&self) -> bool {
        self.mode != "boss"
    }

    pub fn level_range_text(&self) -> String {
        format!("Lv.{} ~ Lv.{}", self.min_level, self.max_level)
    }

    pub fn mode_label(&self) -> &'static str {
        match self.mode.as_str() {
            "boss" => "Boss 挑战",
            "hybrid" => "混合模式",
            _ => "野生模式",
        }
    }

    pub fn area_icon(&self) -> &'static str {
        match self.area_type.as_str() {
            "l" => "🌿",
            "g" => "🌱",
            "p" => "💧",
            "s" => "🌊",
            "b" => "🐚",
            "m" => "⛰️",
            "c" => "🕳️",
            "d" => "🏜️",
            "f" => "🏭",
            "t" => "🏗️",
            "v" => "🏘️",
            "n" => "🏟️",
            "h" => "☁️",
            "o" => "🦑",
            "k" => "🌋",
            _ => "❓",
        }
    }

    pub fn area_color(&self) -> &'static str {
        match self.area_type.as_str() {
            "l" => "plain",
            "g" => "grass",
            "p" | "s" | "b" | "o" => "water",
            "m" | "c" => "mountain",
            "d" => "sand",
            "f" | "t" => "factory",
            "v" | "n" => "town",
            "h" => "sky",
            "k" => "lava",
            _ => "unknown",
        }
    }
}

/// 地图区域。`id` 是服务端给的 region（英文或中文名），`name` 是地图上显示的
/// 名字（「山脉地区」），由 [`region_display_name`] 按 id 查出来——旧版
/// `MapRegion::from_maps` 就是这样，并不是把 id 直接当标签。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapRegion {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub maps: Vec<MapInfo>,
}

pub fn regions_from_maps(maps: &[MapInfo]) -> Vec<MapRegion> {
    let mut order: Vec<String> = Vec::new();
    let mut grouped: std::collections::HashMap<String, Vec<MapInfo>> =
        std::collections::HashMap::new();
    for m in maps {
        // 空 region 归到 unknown（旧版如此）。
        let id = if m.region.is_empty() {
            "unknown".to_string()
        } else {
            m.region.clone()
        };
        if !grouped.contains_key(&id) {
            order.push(id.clone());
        }
        grouped.entry(id).or_default().push(m.clone());
    }
    let mut regions: Vec<MapRegion> = order
        .into_iter()
        .map(|id| {
            let name = region_display_name(&id).to_string();
            let maps = grouped.remove(&id).unwrap_or_default();
            let (x, y) = maps
                .first()
                .map(|m| (m.pos_x as f32, m.pos_y as f32))
                .unwrap_or((50.0, 50.0));
            MapRegion {
                id,
                name,
                x,
                y,
                maps,
            }
        })
        .collect();
    // 旧版按地区次序排（中央、北、东、南、西……，未知最后）；次序相同的保持
    // 首次出现的先后。
    regions.sort_by_key(|r| region_order(&r.id));
    regions
}

/// 地图上显示的地区名（旧版 `get_region_names` 那张表）。表里没有的叫「未知地区」。
pub fn region_display_name(region_id: &str) -> &'static str {
    match region_id {
        // 英文 region id
        "central" => "中央地区",
        "north" => "北部地区",
        "east" => "东部地区",
        "south" => "南部地区",
        "west" => "西部地区",
        "ocean" => "海洋地区",
        "cave" => "洞窟地区",
        "mountain" => "山脉地区",
        "sky" => "天空地区",
        "desert" => "沙漠地区",
        "city" => "城市地区",
        "volcano" => "火山地区",
        "wild" => "野外地区",
        "special" => "特殊地区",
        "unknown" => "未知地区",
        // 中文 region（兼容旧数据库）
        "中央" => "中央地区",
        "北部" => "北部地区",
        "东部" => "东部地区",
        "南部" => "南部地区",
        "西部" => "西部地区",
        "西北" => "西北地区",
        "东北" => "东北地区",
        "西南" => "西南地区",
        "东南" => "东南地区",
        "海洋" => "海洋地区",
        "洞窟" => "洞窟地区",
        "山洞" => "洞窟地区",
        "山脉" => "山脉地区",
        "山谷" => "山脉地区",
        "天空" => "天空地区",
        "沙漠" => "沙漠地区",
        "城市" => "城市地区",
        "火山" => "火山地区",
        "野外" => "野外地区",
        "特殊" => "特殊地区",
        "未知" => "未知地区",
        _ => "未知地区",
    }
}

/// 地区的排列次序（旧版 `get_region_order`）。
fn region_order(region_id: &str) -> u8 {
    match region_id {
        "central" | "中央" => 1,
        "north" | "北部" | "东北" => 2,
        "east" | "东部" | "东南" => 3,
        "south" | "南部" | "西南" => 4,
        "west" | "西部" | "西北" => 5,
        "ocean" | "海洋" => 6,
        "cave" | "洞窟" | "山洞" => 7,
        "mountain" | "山脉" | "山谷" => 8,
        "sky" | "天空" => 9,
        "desert" | "沙漠" => 10,
        "city" | "城市" => 11,
        "volcano" | "火山" => 12,
        "wild" | "野外" => 13,
        "special" | "特殊" => 14,
        _ => 99,
    }
}

/// 战报行（battle_log 端点）。
///
/// 服务端同时返回 `turns`（按回合分组）与 `bbcode`（可直接贴到论坛的整段
/// 引用块）；只解析 `lines` 会让「复制战报」与逐回合视图无法实现。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattleLogResponse {
    #[serde(default)]
    pub lines: Vec<String>,
    #[serde(default)]
    pub turns: Vec<BattleLogTurn>,
    #[serde(default)]
    pub bbcode: String,
}

/// 单个回合的战报行。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattleLogTurn {
    #[serde(default)]
    pub turn: u32,
    #[serde(default)]
    pub lines: Vec<String>,
}

// ---------------------------------------------------------------------------
// 冒险建议与品质门槛（对齐 Dioxus 版 adventure.rs 的闭包）。
// ---------------------------------------------------------------------------

/// 活动地图品质门槛：出战宠物的品质档位是否低于地图要求（对齐 Dioxus 版
/// quality_gate_notice）。None = 无门槛或已达要求；旧端点未返回个体值时
/// 不做客户端预判（服务端仍会拦截）。
pub fn quality_gate_notice(
    map: &MapInfo,
    first: Option<&crate::state::PokemonBasic>,
) -> Option<String> {
    if map.min_quality == 0 {
        return None;
    }
    let rank = first
        .and_then(|p| p.iv_total)
        .map(crate::quality::quality_rank)?;
    if u32::from(rank) < u32::from(map.min_quality) {
        Some(format!(
            "需要出战宠物品质达到 {}（当前 {}）",
            map.min_quality_name,
            crate::quality::quality_grade(rank)
        ))
    } else {
        None
    }
}

/// 挑战建议文案（对齐 Dioxus 版 get_recommendation）：门槛优先，其次等级。
pub fn recommendation(map: &MapInfo, first: Option<&crate::state::PokemonBasic>) -> String {
    let Some(pm) = first else {
        return "请先获得一只宝可梦".to_string();
    };
    if let Some(notice) = quality_gate_notice(map, first) {
        return notice;
    }
    let level = pm.level;
    if level < map.min_level {
        format!("等级不足，建议等级 Lv.{} 以上", map.min_level)
    } else if level > map.max_level + 10 {
        "等级过高，经验收益较低".to_string()
    } else if level >= map.min_level && level <= map.max_level {
        "等级适中，推荐挑战".to_string()
    } else {
        "可以挑战".to_string()
    }
}

/// 卡片难度配色（对齐 Dioxus 版 get_difficulty_class）。
pub fn difficulty_class(map: &MapInfo, first: Option<&crate::state::PokemonBasic>) -> &'static str {
    let Some(pm) = first else {
        return "difficulty-unknown";
    };
    let level = pm.level;
    if level < map.min_level {
        "difficulty-hard"
    } else if level > map.max_level + 10 {
        "difficulty-easy"
    } else if level >= map.min_level && level <= map.max_level {
        "difficulty-medium"
    } else {
        "difficulty-normal"
    }
}

#[cfg(test)]
mod parity_tests {
    use super::*;
    use crate::state::PokemonBasic;

    fn map(min_level: u64, max_level: u64, min_quality: u8) -> MapInfo {
        MapInfo {
            min_level,
            max_level,
            min_quality,
            min_quality_name: if min_quality == 0 {
                String::new()
            } else {
                "C".to_string()
            },
            ..Default::default()
        }
    }

    fn pet(level: u64, iv_total: Option<u64>) -> PokemonBasic {
        PokemonBasic {
            level,
            iv_total,
            ..Default::default()
        }
    }

    #[test]
    fn recommendation_prefers_the_quality_gate_and_names_the_current_grade() {
        let m = map(2, 5, 4); // 门槛 C（rank 4）
                              // 无个体值（旧载荷）不做客户端预判。
        assert_eq!(
            recommendation(&m, Some(&pet(3, None))),
            "等级适中，推荐挑战"
        );
        // rank 3（C）< 4：门槛文案覆盖等级建议。
        assert_eq!(
            recommendation(&m, Some(&pet(3, Some(80)))),
            "需要出战宠物品质达到 C（当前 C）"
        );
        // 无门槛时恢复等级建议。
        assert_eq!(
            recommendation(&map(2, 5, 0), Some(&pet(1, Some(80)))),
            "等级不足，建议等级 Lv.2 以上"
        );
    }

    #[test]
    fn recommendation_follows_the_dioxus_level_branches() {
        let m = map(10, 20, 0);
        assert_eq!(recommendation(&m, None), "请先获得一只宝可梦");
        assert_eq!(
            recommendation(&m, Some(&pet(9, None))),
            "等级不足，建议等级 Lv.10 以上"
        );
        assert_eq!(
            recommendation(&m, Some(&pet(15, None))),
            "等级适中，推荐挑战"
        );
        // 高于上限但在 +10 容差内：可以挑战。
        assert_eq!(recommendation(&m, Some(&pet(28, None))), "可以挑战");
        assert_eq!(
            recommendation(&m, Some(&pet(31, None))),
            "等级过高，经验收益较低"
        );
    }

    #[test]
    fn quality_gate_notice_ignores_maps_without_a_gate() {
        assert_eq!(
            quality_gate_notice(&map(2, 5, 0), Some(&pet(3, Some(10)))),
            None
        );
        assert_eq!(quality_gate_notice(&map(2, 5, 2), None), None);
        assert!(quality_gate_notice(&map(2, 5, 6), Some(&pet(3, Some(120)))).is_some());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battle_over_falls_back_to_status_for_legacy_payloads() {
        let mut scene = BattleScene {
            status: BattleStatus::Victory,
            ..Default::default()
        };
        assert!(scene.effective_battle_over());
        scene.status = BattleStatus::Active;
        assert!(!scene.effective_battle_over());
        // 显式 battle_over 优先（我方倒下但有替补时 status=defeat、over=false）。
        scene.status = BattleStatus::Defeat;
        scene.battle_over = Some(false);
        scene.can_continue_switch = true;
        assert!(!scene.effective_battle_over());
        assert!(scene.needs_replacement());
    }

    #[test]
    fn battle_item_gates_mirror_the_dioxus_rules() {
        let mut pokemon = BattlePokemon {
            hp: 10,
            max_hp: 20,
            ..Default::default()
        };
        let mut potion = BattleItem {
            nums: 1,
            addhp: 20,
            ..Default::default()
        };
        let mut pp_item = BattleItem {
            nums: 1,
            module: "pp10".to_string(),
            ..Default::default()
        };
        assert!(potion.can_use_on(&pokemon));
        assert!(!pp_item.can_use_on(&pokemon)); // 无技能可恢复
        pokemon.skills = vec![BattleSkill {
            pp: 1,
            max_pp: 10,
            ..Default::default()
        }];
        assert!(pp_item.can_use_on(&pokemon));
        pokemon.hp = 0;
        assert!(!potion.can_use_on(&pokemon));
        potion.nums = 0;
        assert!(!potion.can_use_on(&pokemon));
        assert_eq!(pp_item.display_description(), "恢复一个技能的 10 点PP");
    }

    #[test]
    fn maps_payload_parses_flat_modes_and_regions_group_in_order() {
        let payload = r#"{
            "maps": [
                {"id":1,"name":"道路103","region":"北部","pos_x":50,"pos_y":20,
                 "min_level":2,"max_level":5,"mode":"wild",
                 "wild_pokemons":[{"id":25,"name":"皮卡丘"}]},
                {"id":2,"name":"洞窟","region":"洞窟","pos_x":70,"pos_y":70,
                 "min_level":10,"max_level":20,"mode":"boss",
                 "bosses":[{"boss_index":0,"pokemon_type_id":51,"pokemon_name":"三地鼠","level":30,"boss_multiplier":1.5}]}
            ],
            "total":2
        }"#;
        #[derive(Deserialize)]
        struct MapsDto {
            #[serde(default)]
            maps: Vec<MapInfo>,
        }
        let dto: MapsDto = serde_json::from_str(payload).unwrap();
        assert!(dto.maps[0].has_wild());
        assert!(!dto.maps[1].has_wild());
        assert_eq!(dto.maps[1].bosses.len(), 1);
        let regions = regions_from_maps(&dto.maps);
        assert_eq!(regions.len(), 2);
        assert_eq!(regions[0].id, "北部");
        assert_eq!(regions[1].maps.len(), 1);
    }

    #[test]
    fn region_labels_and_order_follow_the_dioxus_table() {
        let map = |n: u32, region: &str| MapInfo {
            id: u64::from(n),
            region: region.to_string(),
            pos_x: n * 10,
            pos_y: 20,
            ..Default::default()
        };
        let regions = regions_from_maps(&[
            map(1, "野外"),
            map(2, "山洞"),
            map(3, ""),
            map(4, "ocean"),
            map(5, "山脉"),
        ]);
        // 排列按旧版的地区次序（海洋 6、洞窟 7、山脉 8、野外 13、未知 99），
        // 不是首次出现的次序；空 region 归到 unknown。
        let ids: Vec<&str> = regions.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(ids, ["ocean", "山洞", "山脉", "野外", "unknown"]);
        // 标签是显示名，不是 id。
        let names: Vec<&str> = regions.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names,
            ["海洋地区", "洞窟地区", "山脉地区", "野外地区", "未知地区"]
        );
        // 标记位置取该地区第一张地图的坐标。
        assert_eq!((regions[1].x, regions[1].y), (20.0, 20.0));
        assert_eq!(region_display_name("东北"), "东北地区");
        assert_eq!(region_display_name("no-such-region"), "未知地区");
    }
}
