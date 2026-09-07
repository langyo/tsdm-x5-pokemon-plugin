use serde::{Deserialize, Serialize};

/// API - 用户数据类型
///
/// 对应 pokemon_system/api/user.php
/// 用户资料响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub uid: u64,
    pub username: String,
    pub group_id: u64,
    #[serde(default)]
    pub is_admin: bool,
    #[serde(default)]
    pub is_new_player: bool,
    /// 金钱（新玩家为 None）
    pub money: Option<i64>,
    /// 冒险体力
    #[serde(rename = "adventure_strength")]
    pub adventure_strength: Option<u64>,
    /// 冒险强度等级 (pm_usersdata.strength)
    #[serde(rename = "strength_level", default)]
    pub strength_level: Option<u64>,
    /// 胜场
    #[serde(default)]
    pub wins: Option<u64>,
    /// 败场
    #[serde(default)]
    pub losses: Option<u64>,
    /// 总对战数
    #[serde(default)]
    pub total_battles: Option<u64>,
    #[serde(rename = "total_pokemons")]
    pub total_pokemons: Option<u64>,
    #[serde(rename = "total_items")]
    pub total_items: Option<u64>,
    /// 当前战斗中的宝可梦 ID（0 表示无战斗）
    #[serde(default)]
    pub npcid: u64,
}

/// 背包物品响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryResponse {
    pub items: Vec<InventoryItem>,
    pub total: usize,
    pub page: usize,
    #[serde(rename = "per_page")]
    pub per_page: usize,
    #[serde(rename = "total_pages")]
    pub total_pages: usize,
}

/// 背包物品
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: u64,
    #[serde(rename = "type_id")]
    pub type_id: u64,
    #[serde(rename = "item_type", default)]
    pub item_type: u64,
    pub name: String,
    pub description: String,
    /// 物品图标文件名 (tpname)
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub quantity: i64,
    /// 战斗中物品数量（API返回nums字段）
    #[serde(default)]
    pub nums: i64,
    #[serde(rename = "type_name")]
    pub type_name: String,
    #[serde(rename = "can_use")]
    pub can_use: bool,
    /// HP恢复量（仅HP药水有此字段）
    #[serde(default)]
    pub addhp: i64,
}

/// 游戏统计响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserStatsResponse {
    #[serde(rename = "pokemon_stats")]
    pub pokemon_stats: PokemonStats,
    #[serde(rename = "inventory_stats")]
    pub inventory_stats: InventoryStats,
    #[serde(rename = "battle_stats")]
    pub battle_stats: BattleStats,
    pub achievements: Achievements,
}

/// 宝可梦统计
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonStats {
    #[serde(rename = "total_owned")]
    pub total_owned: u64,
    #[serde(rename = "active_pokemon")]
    pub active_pokemon: u64,
    #[serde(rename = "total_levels")]
    pub total_levels: u64,
    #[serde(rename = "highest_level")]
    pub highest_level: u64,
}

/// 物品统计
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryStats {
    #[serde(rename = "total_items")]
    pub total_items: u64,
    #[serde(rename = "total_quantity")]
    pub total_quantity: i64,
}

/// 背包分类统计项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryCategoryStat {
    #[serde(rename = "type_id")]
    pub type_id: u64,
    pub count: i64,
}

/// 背包分类统计响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryStatsResponse {
    pub categories: Vec<InventoryCategoryStat>,
}

/// 战斗统计
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BattleStats {
    #[serde(rename = "total_battles")]
    pub total_battles: u64,
    pub victories: u64,
    pub defeats: u64,
}

/// 成就列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Achievements {
    #[serde(rename = "first_pokemon")]
    pub first_pokemon: bool,
    #[serde(rename = "pokemon_master")]
    pub pokemon_master: bool,
    #[serde(rename = "high_level")]
    pub high_level: bool,
    pub collector: bool,
}

/// 使用物品请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItemRequest {
    #[serde(rename = "item_id")]
    pub item_id: u64,
    #[serde(rename = "pokemon_id")]
    pub pokemon_id: Option<u64>,
}

/// 使用物品响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UseItemResponse {
    pub success: bool,
    pub message: String,
    #[serde(rename = "item_remaining")]
    pub item_remaining: Option<i64>,
    #[serde(rename = "pokemon_updated")]
    pub pokemon_updated: Option<PokemonUpdate>,
}

/// 宝可梦更新信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonUpdate {
    pub id: u64,
    #[serde(rename = "hp")]
    pub hp: Option<i64>,
    #[serde(rename = "max_hp")]
    pub max_hp: Option<i64>,
    #[serde(rename = "state")]
    pub state: Option<u64>,
}

/// 在线玩家响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnlinePlayersResponse {
    /// 在线玩家总数
    pub total: usize,
    /// 玩家列表（最多返回20个）
    pub players: Vec<OnlinePlayer>,
    /// 最大显示数量
    #[serde(rename = "max_display")]
    pub max_display: usize,
}

/// 在线玩家信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnlinePlayer {
    pub uid: u64,
    pub username: String,
    #[serde(default)]
    pub avatar: String,
    #[serde(rename = "last_activity")]
    pub last_activity: u64,
}

/// 初始化新玩家响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitializePlayerResponse {
    pub success: bool,
    pub message: String,
    pub money: i64,
    pub egg_received: bool,
}

/// 可使用物品的宠物信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsablePokemon {
    pub id: u64,
    #[serde(rename = "type_id")]
    pub type_id: u64,
    pub name: String,
    pub nickname: String,
    pub level: u64,
    pub hp: i64,
    #[serde(rename = "max_hp")]
    pub max_hp: i64,
    pub state: u64,
}

/// 获取可使用物品的宠物列表响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsablePokemonResponse {
    #[serde(rename = "item_id")]
    pub item_id: u64,
    #[serde(rename = "item_name")]
    pub item_name: String,
    #[serde(rename = "item_type")]
    pub item_type: u64,
    #[serde(rename = "usable_pokemon")]
    pub usable_pokemon: Vec<UsablePokemon>,
    #[serde(rename = "unusable_count")]
    pub unusable_count: usize,
}

/// 帖子宠物徽章可见状态（refresh_badge 与 badge_status 共用）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BadgeStatusResponse {
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub initialized: bool,
}
