use serde::{Deserialize, Serialize};

/// API - 进化系统类型
///
/// 对应 pokemon_system/api/evolution.php
/// 进化条件检查结果
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionCheckResponse {
    pub can_evolve: bool,
    pub pokemon_id: u64,
    pub current_form: u64,
    pub target_form: u64,
    pub conditions: EvolutionConditions,
    pub evolution_method: String,
    pub reason: String,
    /// 可选：具体的进化路线列表（物种无路线时可能缺失）
    #[serde(default)]
    pub available_evolutions: Vec<AvailableEvolution>,
}

/// 进化条件详情
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionConditions {
    pub can_evolve: bool,
    pub reason: String,
    pub details: EvolutionConditionDetails,
}

/// 进化条件详细信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionConditionDetails {
    #[serde(rename = "level_requirement")]
    pub level_requirement: Option<u64>,
    #[serde(rename = "item_requirement")]
    pub item_requirement: Option<u64>,
    #[serde(rename = "intimacy_requirement")]
    pub intimacy_requirement: Option<u64>,
}

/// 进化响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionResponse {
    pub message: String,
    pub previous_form: u64,
    pub new_form: u64,
    pub new_name: String,
    pub stats: EvolutionStats,
}

/// 进化后属性
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionStats {
    pub hp: i64,
    pub level: u64,
}

/// 可进化形态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvailableEvolution {
    pub from_id: u64,
    pub to_id: u64,
    pub to_name: String,
    pub evolution_type: String,
    pub can_evolve: bool,
    pub conditions: EvolutionConditions,
    /// 可选：后端拼好的条件文案（例 '亲密度达到 220'）
    #[serde(default)]
    pub condition_display: Option<String>,
}

/// 可进化列表响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvailableEvolutionsResponse {
    pub pokemon_id: u64,
    pub current_form: u64,
    pub available_evolutions: Vec<AvailableEvolution>,
}

/// 进化请求
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionRequest {
    pub pet_id: u64,
    /// 多条路线都满足时指定目标形态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_id: Option<u64>,
}

/// 进化路径节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionPathNode {
    pub id: u64,
    pub name: String,
    pub type_1: String,
    #[serde(default)]
    pub type_2: Option<String>,
    pub condition_type: String,
    pub condition_value: String,
    pub condition_display: String,
}

/// 进化路径响应
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionPathResponse {
    pub pokemon_id: u64,
    pub pokemon_name: String,
    pub pokemon_type1: String,
    #[serde(default)]
    pub pokemon_type2: Option<String>,
    pub forward: Vec<EvolutionPathNode>,
    pub backward: Vec<EvolutionPathNode>,
}
