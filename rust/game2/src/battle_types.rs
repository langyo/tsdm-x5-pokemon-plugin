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
            BattleStatus::Victory | BattleStatus::Captured => "战斗胜利！",
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

/// 地图区域（服务端 region 即中文名，无需再映射）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapRegion {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub maps: Vec<MapInfo>,
}

pub fn regions_from_maps(maps: &[MapInfo]) -> Vec<MapRegion> {
    let mut order: Vec<String> = Vec::new();
    let mut grouped: std::collections::HashMap<String, Vec<MapInfo>> =
        std::collections::HashMap::new();
    for m in maps {
        if !grouped.contains_key(&m.region) {
            order.push(m.region.clone());
        }
        grouped.entry(m.region.clone()).or_default().push(m.clone());
    }
    order
        .into_iter()
        .map(|id| {
            let maps = grouped.remove(&id).unwrap_or_default();
            let (x, y) = maps
                .first()
                .map(|m| (m.pos_x as f32, m.pos_y as f32))
                .unwrap_or((50.0, 50.0));
            MapRegion { id, x, y, maps }
        })
        .collect()
}

/// 战报行（battle_log 端点）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BattleLogResponse {
    #[serde(default)]
    pub lines: Vec<String>,
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
}
