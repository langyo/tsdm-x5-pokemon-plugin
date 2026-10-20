use serde::{Deserialize, Serialize};

/// API - 战斗系统类型
///
/// 对应 pokemon_system/api/battle.php
/// 战斗场景
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BattleScene {
    pub battle_id: String,
    pub engine_battle_id: u64,
    pub revision: u64,
    pub phase: String,
    pub events: Vec<BattleLogEvent>,
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

impl BattleScene {
    /// 以服务端判断为准，宠物列表可能尚未加载或仍是上一回合的缓存。
    pub fn needs_replacement(&self) -> bool {
        self.can_continue_switch && !self.battle_over
    }
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
                let mut engine_battle_id = 0;
                let mut revision = 0;
                let mut phase = String::new();
                let mut events = Vec::new();
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
                        "engine_battle_id" => engine_battle_id = map.next_value()?,
                        "revision" => revision = map.next_value()?,
                        "phase" => phase = map.next_value()?,
                        "events" => events = map.next_value()?,
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
                            rewards = map.next_value()?;
                        }
                        "level_up" => {
                            level_up = map.next_value()?;
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
                    engine_battle_id,
                    revision,
                    phase,
                    events,
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
    /// 结算后今日（站点时区）已获得的 PVE 宠物币；旧服务端不回此字段时为 0。
    #[serde(default)]
    pub money_today: u64,
    /// 每日 PVE 宠物币上限，0 表示不限制；旧服务端不回此字段时为 0。
    #[serde(default)]
    pub money_daily_limit: u64,
    /// 宠物币管控提示（等级差距过大 / 已达每日上限），没有时为空。
    #[serde(default)]
    pub money_notes: Vec<String>,
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
    /// 指定 Boss 宠物类型 ID（仅 Boss 地图使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boss_pokemon_type_id: Option<u64>,
    /// 地图配置的原始索引；缺省时兼容按物种选择的旧请求。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boss_index: Option<u64>,
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
    pub available_skills: Vec<PpRestoreSkill>,
    pub message: String,
    #[serde(default)]
    pub engine_battle_id: u64,
    #[serde(default)]
    pub revision: u64,
}

/// PP 恢复选择的是已学技能记录，不是技能种类。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PpRestoreSkill {
    pub id: u64,
    pub skill_id: u64,
    pub name: String,
    pub current_pp: u64,
    pub max_pp: u64,
}

/// 对技能使用道具请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItemOnSkillRequest {
    pub item_id: u64,
    pub skill_record_id: u64,
}

/// 战斗道具接口不分页，物品 id 是种类 ID（普通背包接口的 id 则是持有记录 ID）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleItemsResponse {
    pub items: Vec<BattleItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleLogEvent {
    pub turn: u64,
    pub seq: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleLogTurn {
    pub turn: u64,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleLogResponse {
    pub battle_id: u64,
    pub turn: u64,
    pub phase: String,
    pub result: String,
    pub lines: Vec<String>,
    #[serde(default)]
    pub turns: Vec<BattleLogTurn>,
    pub events: Vec<BattleLogEvent>,
    pub bbcode: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleMutationRequest {
    pub request_id: String,
    pub engine_battle_id: u64,
    pub expected_revision: u64,
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingBattleAction {
    pub action: String,
    pub request: BattleMutationRequest,
}

impl PendingBattleAction {
    pub fn new(action: &str, scene: Option<&BattleScene>, fields: serde_json::Value) -> Self {
        Self {
            action: action.to_string(),
            request: BattleMutationRequest {
                request_id: uuid::Uuid::new_v4().to_string(),
                engine_battle_id: scene.map(|scene| scene.engine_battle_id).unwrap_or(0),
                expected_revision: scene.map(|scene| scene.revision).unwrap_or(0),
                fields,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum BattleMutationResponse {
    Scene(Box<BattleScene>),
    SkillSelection(SkillSelectionResponse),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleItem {
    pub id: u64,
    pub name: String,
    pub img: String,
    pub nums: i64,
    pub item_type: u64,
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
            self.item_type == 1 && self.addhp > 0 && pokemon.hp < pokemon.max_hp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::api_pokemon::ApiResponse;

    #[test]
    fn indexed_boss_selection_survives_pending_request_round_trip() {
        let legacy: StartBattleRequest =
            serde_json::from_value(serde_json::json!({"map_id":7,"boss_pokemon_type_id":25}))
                .unwrap();
        assert_eq!(legacy.boss_index, None);
        assert_eq!(
            serde_json::to_value(&legacy).unwrap(),
            serde_json::json!({"map_id":7,"boss_pokemon_type_id":25})
        );

        for index in [0, 1] {
            let request = StartBattleRequest {
                boss_index: Some(index),
                ..legacy.clone()
            };
            let pending =
                PendingBattleAction::new("start", None, serde_json::to_value(request).unwrap());
            let encoded = serde_json::to_string(&pending).unwrap();
            let restored: PendingBattleAction = serde_json::from_str(&encoded).unwrap();
            assert_eq!(restored, pending);
            assert_eq!(restored.request.fields["boss_index"], index);
            assert_eq!(restored.request.engine_battle_id, 0);
            assert_eq!(restored.request.expected_revision, 0);
        }
    }

    // Shapes captured from the PHP battle endpoints, including migrated type=1 PP items.
    const BATTLE_ITEMS: &str = r#"{"success":true,"data":{"items":[
        {"id":17,"name":"Potion","img":"hp20","nums":3,"item_type":1,"module":"hp20","addhp":20},
        {"id":18,"name":"Ether","img":"pp5","nums":2,"item_type":1,"module":"pp5"}
    ]}}"#;
    const PP_SELECTION: &str = r#"{"success":true,"data":{
        "requires_skill_selection":true,"item_id":18,"item_name":"Ether",
        "available_skills":[{"id":701,"skill_id":12,"name":"Tackle","current_pp":2,"max_pp":20}],
        "message":"请选择要恢复PP的技能"
    }}"#;

    fn pokemon() -> BattlePokemon {
        BattlePokemon {
            id: 25,
            instance_id: 501,
            name: "Pikachu".into(),
            level: 5,
            hp: 50,
            max_hp: 50,
            skills: vec![BattleSkill {
                id: 12,
                name: "Tackle".into(),
                power: 40,
                pp: 2,
                max_pp: 20,
                skill_type: String::new(),
                category: String::new(),
            }],
        }
    }

    #[test]
    fn battle_rewards_accept_daily_money_fields_and_legacy_shape() {
        let legacy: BattleRewards = serde_json::from_str(r#"{"exp":27,"money":40}"#).unwrap();
        assert_eq!(legacy.money_today, 0);
        assert_eq!(legacy.money_daily_limit, 0);
        assert!(legacy.money_notes.is_empty());
        let capped: BattleRewards = serde_json::from_str(
            r#"{"exp":27,"money":10,"money_today":100,"money_daily_limit":100,
                "money_notes":["今日宠物币已达上限，本场只获得 10"]}"#,
        )
        .unwrap();
        assert_eq!(capped.money, 10);
        assert_eq!(capped.money_today, 100);
        assert_eq!(capped.money_daily_limit, 100);
        assert_eq!(
            capped.money_notes,
            vec!["今日宠物币已达上限，本场只获得 10"]
        );
    }

    #[test]
    fn battle_items_accept_php_shape_without_inventory_pagination() {
        let response: ApiResponse<BattleItemsResponse> =
            serde_json::from_str(BATTLE_ITEMS).unwrap();
        let items = response.data.unwrap().items;
        assert_eq!(items[0].id, 17);
        assert_eq!(items[0].img, "hp20");
        assert_eq!(items[0].nums, 3);
        assert_eq!(items[1].addhp, 0);
        let empty: ApiResponse<BattleItemsResponse> =
            serde_json::from_str(r#"{"success":true,"data":{"items":[]}}"#).unwrap();
        assert!(empty.data.unwrap().items.is_empty());
    }

    #[test]
    fn pp_restore_uses_record_id_and_server_request_field() {
        let response: ApiResponse<SkillSelectionResponse> =
            serde_json::from_str(PP_SELECTION).unwrap();
        let selection = response.data.unwrap();
        let skill = &selection.available_skills[0];
        assert_eq!(skill.current_pp, 2);
        assert_eq!(skill.skill_id, 12);
        let request = UseItemOnSkillRequest {
            item_id: selection.item_id,
            skill_record_id: skill.id,
        };
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            serde_json::json!({"item_id":18,"skill_record_id":701})
        );
    }

    #[test]
    fn migrated_pp_items_use_pp_not_hp_to_determine_availability() {
        let response: ApiResponse<BattleItemsResponse> =
            serde_json::from_str(BATTLE_ITEMS).unwrap();
        let items = response.data.unwrap().items;
        let mut pokemon = pokemon();
        assert!(!items[0].can_use_on(&pokemon));
        assert!(items[1].can_use_on(&pokemon));
        pokemon.hp = 20;
        pokemon.skills[0].pp = 20;
        assert!(items[0].can_use_on(&pokemon));
        assert!(!items[1].can_use_on(&pokemon));
        pokemon.hp = 0;
        assert!(!items[0].can_use_on(&pokemon));
        assert!(!items[1].can_use_on(&pokemon));
    }

    #[test]
    fn replacement_follows_server_flags_without_a_cached_pokemon_list() {
        let mut scene: BattleScene = serde_json::from_value(serde_json::json!({
            "battle_id":"battle_1", "map_id":0, "map_name":"", "turn":3,
            "my_pokemon":{"id":25,"instance_id":501,"name":"Pikachu","level":5,"hp":0,"max_hp":50,"skills":[]},
            "wild_pokemon":{"id":19,"name":"Rattata","level":5,"hp":20,"max_hp":30,"gender":0,"is_shiny":false},
            "status":"defeat", "battle_over":false, "can_continue_switch":true
        })).unwrap();
        assert_eq!(scene.my_pokemon.instance_id, 501);
        assert!(scene.needs_replacement());
        scene.battle_over = true;
        assert!(!scene.needs_replacement());
    }
}
