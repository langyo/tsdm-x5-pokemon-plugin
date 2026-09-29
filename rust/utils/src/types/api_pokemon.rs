use serde::{Deserialize, Serialize};

/// API - 宝可梦相关类型
///
/// 对应 pokemon_system/api/pokemon.php
/// API统一响应格式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub code: Option<u16>,
    pub timestamp: Option<i64>,
}

/// 宠物列表响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonListResponse {
    pub pokemons: Vec<PokemonBasic>,
    pub total: usize,
}

/// 宠物基础信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonBasic {
    pub id: u64,
    /// 物种名（原始名称）
    pub name: String,
    /// 昵称（用户自定义名称）
    #[serde(default)]
    pub nickname: Option<String>,
    pub type_id: u64,
    pub level: u64,
    pub exp: u64,
    pub exp_to_next_level: u64,
    pub exp_for_current_level: u64,
    pub exp_for_next_level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub gender: u8,
    pub is_shiny: bool,
    /// 位置 (1=首位/正在战斗中)
    #[serde(default)]
    pub site: u8,
    /// 状态码
    #[serde(default)]
    pub state: u8,
    /// 状态文本
    #[serde(default)]
    pub state_text: String,
    /// 状态 CSS 类名
    #[serde(default)]
    pub state_class: String,
    /// 好感度/亲密度
    #[serde(default)]
    pub affection: u32,
    pub skills: Vec<PokemonSkillSlot>,
    pub base_info: PokemonBaseInfo,
}

/// 宠物详细信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonDetail {
    pub id: u64,
    /// 物种名（原始名称）
    pub name: String,
    /// 昵称（用户自定义名称）
    #[serde(default)]
    pub nickname: Option<String>,
    pub type_id: u64,
    pub level: u64,
    pub exp: u64,
    pub exp_to_next_level: u64,
    pub exp_for_current_level: u64,
    pub exp_for_next_level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub gender: u8,
    pub is_shiny: bool,
    /// 状态码
    #[serde(default)]
    pub state: u8,
    /// 状态文本
    #[serde(default)]
    pub state_text: String,
    /// 状态 CSS 类名
    #[serde(default)]
    pub state_class: String,
    /// 好感度/亲密度
    #[serde(default)]
    pub affection: u32,
    pub stats: PokemonStats,
    pub skills: Vec<PokemonSkillSlot>,
    pub base_info: PokemonBaseInfo,
}

/// 技能槽位
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonSkillSlot {
    pub type_id: u64,
    pub pp: u64,
    /// 技能名称
    #[serde(default)]
    pub name: String,
    /// 技能属性 (电, 超能, 普通, etc.)
    #[serde(default)]
    pub skill_type: String,
    /// 招式分类 (物攻, 特攻, etc.)
    #[serde(default)]
    pub category: String,
    /// 学习等级
    #[serde(default)]
    pub level: u32,
    /// 威力
    #[serde(default)]
    pub power: u32,
    /// 最大PP
    #[serde(default)]
    pub max_pp: u64,
    /// 是否满足遗忘条件（遗忘要求PP为满；max_uses=0 恒可遗忘）
    #[serde(default)]
    pub can_forget: bool,
}

/// 宠物基础信息(从pm_data表)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonBaseInfo {
    pub id: u64,
    pub name: String,
    #[serde(alias = "type1", alias = "type_1")]
    pub type_1: String,
    #[serde(alias = "type2", alias = "type_2")]
    pub type_2: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// 宠物能力值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PokemonStats {
    pub hp: i64,
    pub attack: i64,
    pub defense: i64,
    #[serde(rename = "sp_attack")]
    pub sp_attack: i64,
    #[serde(rename = "sp_defense")]
    pub sp_defense: i64,
    pub speed: i64,
}

/// 重命名请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenameRequest {
    pub id: u64,
    pub name: String,
}

/// 放生请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseRequest {
    pub id: u64,
}

/// 可学习的技能信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearnableSkill {
    pub id: u64,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub skill_type: String,
    pub category: String,
    pub power: u32,
    pub max_pp: u32,
    pub required_level: u32,
    pub is_available: bool,
}

/// 技能学习响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearnableSkillsResponse {
    pub pokemon_id: u64,
    pub pokemon_level: u32,
    pub available_skills: Vec<LearnableSkill>,
    pub unlocked_skills: Vec<LearnableSkill>,
}

/// 遗忘技能请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgetSkillRequest {
    pub pokemon_id: u64,
    pub skill_id: u64,
}

/// 学习技能请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearnSkillRequest {
    pub pokemon_id: u64,
    pub skill_id: u64,
}

/// 学习技能响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearnSkillResponse {
    pub message: String,
    pub pokemon_id: u64,
    pub skill: PokemonSkillSlot,
}

/// 装备槽位信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentSlot {
    pub slot_index: u32,
    pub equipment_id: u64,
    pub item: Option<EquipmentItem>,
}

/// 装备物品信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentItem {
    #[serde(default)]
    pub myitem_id: u64,
    pub type_id: u64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub image: String,
    /// 装备类型 (1-4 对应不同槽位类型)
    #[serde(default)]
    pub zbtype: u32,
    #[serde(default)]
    pub equipment_hp: i32,
    #[serde(default)]
    pub equipment_atk: i32,
    #[serde(default)]
    pub equipment_def: i32,
    #[serde(default)]
    pub equipment_spatk: i32,
    #[serde(default)]
    pub equipment_spdef: i32,
    #[serde(default)]
    pub equipment_sd: i32,
    /// 总数量 (仅 owned_items 有)
    #[serde(default)]
    pub quantity: u32,
    /// 已装备数量 (仅 owned_items 有)
    #[serde(default)]
    pub equipped_count: u32,
    /// 可用数量 = 总数量 - 已装备数量 (仅 owned_items 有)
    #[serde(default)]
    pub available_count: u32,
    /// 是否已被当前宝可梦装备 (仅 owned_items 有)
    #[serde(default)]
    pub is_equipped: bool,
    /// 价格 (仅 shop_items 有)
    #[serde(default)]
    pub price: i64,
    /// 是否已拥有 (仅 shop_items 有)
    #[serde(default)]
    pub is_owned: bool,
    /// 是否可购买 (仅 shop_items 有)
    #[serde(default)]
    pub can_buy: bool,
}

/// 装备信息响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentResponse {
    pub pokemon_id: u64,
    pub equipment_slots: Vec<EquipmentSlot>,
    pub owned_items: Vec<EquipmentItem>,
    pub shop_items: Vec<EquipmentItem>,
    pub user_money: i64,
}

/// 装备物品请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipItemRequest {
    pub pokemon_id: u64,
    pub myitem_id: u64,
    #[serde(default)]
    pub slot_index: Option<i32>,
}

/// 装备物品响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipItemResponse {
    pub message: String,
    pub pokemon_id: u64,
    pub slot_index: u32,
    pub item_name: String,
    #[serde(default)]
    pub base_hp: i32,
    #[serde(default)]
    pub equipment_hp: i32,
    #[serde(default)]
    pub total_hp: i32,
    #[serde(default)]
    pub equipment_atk: i32,
    #[serde(default)]
    pub equipment_def: i32,
    #[serde(default)]
    pub equipment_spatk: i32,
    #[serde(default)]
    pub equipment_spdef: i32,
    #[serde(default)]
    pub equipment_speed: i32,
    #[serde(default)]
    pub base_atk: i32,
    #[serde(default)]
    pub base_def: i32,
    #[serde(default)]
    pub base_spatk: i32,
    #[serde(default)]
    pub base_spdef: i32,
    #[serde(default)]
    pub base_speed: i32,
    #[serde(default)]
    pub total_atk: i32,
    #[serde(default)]
    pub total_def: i32,
    #[serde(default)]
    pub total_spatk: i32,
    #[serde(default)]
    pub total_spdef: i32,
    #[serde(default)]
    pub total_speed: i32,
    #[serde(default)]
    pub new_hp: u32,
    #[serde(default)]
    pub new_maxhp: u32,
}

/// 状态属性修正系数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateMultipliers {
    #[serde(default)]
    pub hp: f64,
    #[serde(default)]
    pub atk: f64,
    #[serde(default)]
    pub def: f64,
    #[serde(default)]
    pub spatk: f64,
    #[serde(default)]
    pub spdef: f64,
    #[serde(default)]
    pub speed: f64,
}

/// 更新状态响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateStateResponse {
    pub pokemon_id: u64,
    pub old_state: u8,
    pub new_state: u8,
    pub state_changed: bool,
    pub state_text: String,
    pub state_class: String,
    pub state_multipliers: StateMultipliers,
    #[serde(default)]
    pub exp_change: i32,
}

/// 卸下装备请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnequipItemRequest {
    pub pokemon_id: u64,
    pub slot_index: u32,
}

/// 卸下装备响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnequipItemResponse {
    pub message: String,
    pub pokemon_id: u64,
    pub slot_index: u32,
    pub item_name: String,
    #[serde(default)]
    pub base_hp: i32,
    #[serde(default)]
    pub equipment_hp: i32,
    #[serde(default)]
    pub total_hp: i32,
    #[serde(default)]
    pub equipment_atk: i32,
    #[serde(default)]
    pub equipment_def: i32,
    #[serde(default)]
    pub equipment_spatk: i32,
    #[serde(default)]
    pub equipment_spdef: i32,
    #[serde(default)]
    pub equipment_speed: i32,
    #[serde(default)]
    pub base_atk: i32,
    #[serde(default)]
    pub base_def: i32,
    #[serde(default)]
    pub base_spatk: i32,
    #[serde(default)]
    pub base_spdef: i32,
    #[serde(default)]
    pub base_speed: i32,
    #[serde(default)]
    pub total_atk: i32,
    #[serde(default)]
    pub total_def: i32,
    #[serde(default)]
    pub total_spatk: i32,
    #[serde(default)]
    pub total_spdef: i32,
    #[serde(default)]
    pub total_speed: i32,
    #[serde(default)]
    pub new_hp: u32,
    #[serde(default)]
    pub new_maxhp: u32,
}
