use serde::{Deserialize, Serialize};

/// API - 战斗系统类型
///
/// 对应 pokemon_system/api/battle.php
/// 战斗场景
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BattleScene {
    pub battle_id: String,
    pub map_id: u64,
    pub map_name: String,
    pub turn: u64,
    pub my_pokemon: BattlePokemon,
    pub wild_pokemon: WildPokemon,
    pub status: BattleStatus,
    #[serde(default)]
    pub message: String,
    /// 战斗是否已在服务端真正结束（状态已清空）。
    /// status 在「我方宠物倒下但还有替补」时仍为 defeat（兼容旧客户端），
    /// 此时 battle_over=false，客户端应依据 can_continue_switch 弹出换宠。
    pub battle_over: bool,
    /// 宠物倒下但战斗继续，客户端应弹出换宠选择。
    pub can_continue_switch: bool,
    pub rewards: Option<BattleRewards>,
    pub level_up: Option<LevelUpInfo>,
}

// Custom deserialization to add logging
impl<'de> Deserialize<'de> for BattleScene {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct BattleSceneVisitor;

        impl<'de> Visitor<'de> for BattleSceneVisitor {
            type Value = BattleScene;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BattleScene")
            }

            fn visit_map<A>(self, mut map: A) -> Result<BattleScene, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut battle_id: Option<String> = None;
                let mut map_id: Option<u64> = None;
                let mut map_name: Option<String> = None;
                let mut turn: Option<u64> = None;
                let mut my_pokemon: Option<BattlePokemon> = None;
                let mut wild_pokemon: Option<WildPokemon> = None;
                let mut status: Option<BattleStatus> = None;
                let mut message: Option<String> = None;
                let mut battle_over: Option<bool> = None;
                let mut can_continue_switch: Option<bool> = None;
                let mut rewards: Option<BattleRewards> = None;
                let mut level_up: Option<LevelUpInfo> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "battle_id" => {
                            battle_id = Some(map.next_value()?);
                        }
                        "map_id" => {
                            map_id = Some(map.next_value()?);
                        }
                        "map_name" => {
                            map_name = Some(map.next_value()?);
                        }
                        "turn" => {
                            turn = Some(map.next_value()?);
                        }
                        "my_pokemon" => {
                            my_pokemon = Some(map.next_value()?);
                        }
                        "wild_pokemon" => {
                            wild_pokemon = Some(map.next_value()?);
                        }
                        "status" => {
                            status = Some(map.next_value()?);
                        }
                        "message" => {
                            message = Some(map.next_value()?);
                        }
                        "battle_over" => {
                            battle_over = Some(map.next_value()?);
                        }
                        "can_continue_switch" => {
                            can_continue_switch = Some(map.next_value()?);
                        }
                        "rewards" => {
                            rewards = Some(map.next_value()?);
                        }
                        "level_up" => {
                            level_up = Some(map.next_value()?);
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let battle_id =
                    battle_id.ok_or_else(|| serde::de::Error::missing_field("battle_id"))?;
                let map_id = map_id.ok_or_else(|| serde::de::Error::missing_field("map_id"))?;
                let map_name =
                    map_name.ok_or_else(|| serde::de::Error::missing_field("map_name"))?;
                let turn = turn.ok_or_else(|| serde::de::Error::missing_field("turn"))?;
                let my_pokemon =
                    my_pokemon.ok_or_else(|| serde::de::Error::missing_field("my_pokemon"))?;
                let wild_pokemon =
                    wild_pokemon.ok_or_else(|| serde::de::Error::missing_field("wild_pokemon"))?;
                let status = status.ok_or_else(|| serde::de::Error::missing_field("status"))?;
                let message = message.unwrap_or_default();
                // 旧服务端不回这两个字段：battle_over 按 status 推断
                // （Active 之外一律视为已结束），can_continue_switch 默认 false，
                // 与旧客户端行为完全一致
                let battle_over = battle_over.unwrap_or(status != BattleStatus::Active);
                let can_continue_switch = can_continue_switch.unwrap_or(false);

                Ok(BattleScene {
                    battle_id,
                    map_id,
                    map_name,
                    turn,
                    my_pokemon,
                    wild_pokemon,
                    status,
                    message,
                    battle_over,
                    can_continue_switch,
                    rewards,
                    level_up,
                })
            }
        }

        deserializer.deserialize_map(BattleSceneVisitor)
    }
}

/// 战斗中的宠物
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattlePokemon {
    pub id: u64,
    #[serde(default)]
    pub instance_id: u64, // 数据库唯一 ID (pm_mypm.id)
    pub name: String,
    pub level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub skills: Vec<BattleSkill>,
}

/// 野怪/Boss
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WildPokemon {
    pub id: u64,
    pub name: String,
    pub level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub gender: u8,
    pub is_shiny: bool,
    /// Boss 标记
    #[serde(default)]
    pub is_boss: bool,
    /// Boss 属性倍率
    #[serde(default)]
    pub boss_multiplier: f32,
}

// Custom deserialization to add logging
impl<'de> Deserialize<'de> for WildPokemon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct WildPokemonVisitor;

        impl<'de> Visitor<'de> for WildPokemonVisitor {
            type Value = WildPokemon;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct WildPokemon")
            }

            fn visit_map<A>(self, mut map: A) -> Result<WildPokemon, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut level = None;
                let mut hp = None;
                let mut max_hp = None;
                let mut gender = None;
                let mut is_shiny = None;
                let mut is_boss = None;
                let mut boss_multiplier = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => {
                            id = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "level" => {
                            level = Some(map.next_value()?);
                        }
                        "hp" => {
                            hp = Some(map.next_value()?);
                        }
                        "max_hp" => {
                            max_hp = Some(map.next_value()?);
                        }
                        "gender" => {
                            gender = Some(map.next_value()?);
                        }
                        "is_shiny" => {
                            is_shiny = Some(map.next_value()?);
                        }
                        "is_boss" => {
                            is_boss = Some(map.next_value()?);
                        }
                        "boss_multiplier" => {
                            boss_multiplier = Some(map.next_value()?);
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let level = level.ok_or_else(|| serde::de::Error::missing_field("level"))?;
                let hp = hp.ok_or_else(|| serde::de::Error::missing_field("hp"))?;
                let max_hp = max_hp.ok_or_else(|| serde::de::Error::missing_field("max_hp"))?;
                let gender = gender.unwrap_or(0);
                let is_shiny = is_shiny.unwrap_or(false);
                let is_boss = is_boss.unwrap_or(false);
                let boss_multiplier = boss_multiplier.unwrap_or(1.0);

                Ok(WildPokemon {
                    id,
                    name,
                    level,
                    hp,
                    max_hp,
                    gender,
                    is_shiny,
                    is_boss,
                    boss_multiplier,
                })
            }
        }

        deserializer.deserialize_map(WildPokemonVisitor)
    }
}

/// 战斗技能
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleSkill {
    pub id: u64,
    pub name: String,
    pub power: u64,
    pub pp: u64,
    pub max_pp: u64,
    /// 技能属性 (电，超能，普通，etc.)
    #[serde(default)]
    pub skill_type: String,
    /// 招式分类 (物攻，特攻，etc.)
    #[serde(default)]
    pub category: String,
}

/// 战斗状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BattleStatus {
    Active,
    Victory,
    Defeat,
    Fled,
    Captured,
}

/// 战斗奖励
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleRewards {
    pub exp: u64,
    pub money: u64,
}

/// 升级信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelUpInfo {
    pub level_up: bool,
    pub old_level: u64,
    pub new_level: u64,
    pub new_exp: u64,
}

/// 开始战斗请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartBattleRequest {
    pub map_id: u64,
    /// 指定 Boss 寺孠类型 ID（仅 Boss 地图使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boss_pokemon_type_id: Option<u64>,
}

/// 使用技能请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseSkillRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battle_id: Option<String>,
    pub skill_id: u64,
}

/// 逃跑请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battle_id: Option<String>,
}

/// 技能选择响应（用于PP恢复道具）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillSelectionResponse {
    pub requires_skill_selection: bool,
    pub item_id: u64,
    pub item_name: String,
    pub available_skills: Vec<BattleSkill>,
    pub message: String,
}

/// 对技能使用道具请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItemOnSkillRequest {
    pub item_id: u64,
    pub skill_id: u64,
}
