use serde::{Deserialize, Serialize};

use super::pokemon_type::PokemonAttributes;

/// 地图 Boss 配置
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MapBossConfig {
    pub bosses: Vec<MapBoss>,
}

/// 单个 Boss 配置
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapBoss {
    /// 宠物类型 ID
    #[serde(default)]
    pub pokemon_type_id: u64,
    /// 宠物名称（用于显示）
    #[serde(default)]
    pub pokemon_name: String,
    /// 固定等级
    #[serde(default = "default_boss_level")]
    pub level: u64,
    /// 个体值
    #[serde(default)]
    pub attributes: PokemonAttributes,
    /// Boss 属性倍率
    #[serde(default = "default_boss_multiplier")]
    pub boss_multiplier: f32,
}

fn default_boss_level() -> u64 {
    50
}
fn default_boss_multiplier() -> f32 {
    1.5
}

impl MapBoss {
    /// 创建一个新的 Boss 配置
    pub fn new(pokemon_type_id: u64, pokemon_name: String) -> Self {
        Self {
            pokemon_type_id,
            pokemon_name,
            level: default_boss_level(),
            attributes: PokemonAttributes::default(),
            boss_multiplier: default_boss_multiplier(),
        }
    }
}

/// Boss 配置的 JSON 编码/解码
impl MapBossConfig {
    /// 从 expn 字段解析配置
    /// 如果 expn 是数字，返回默认配置（兼容旧数据）
    /// 如果 expn 是 JSON，解析为 Boss 配置
    pub fn from_expn(expn: String) -> Self {
        // 尝试解析为 JSON
        if let Ok(config) = serde_json::from_str::<MapBossConfig>(&expn) {
            return config;
        }

        // 如果不是 JSON，尝试解析为数字（兼容旧数据）
        if expn.parse::<u64>().is_ok() {
            return Self::default();
        }

        // 无效数据，返回默认配置
        Self::default()
    }

    /// 将配置编码为 expn 字段
    pub fn to_expn(&self) -> String {
        if self.bosses.is_empty() {
            // 没有 boss 配置，返回 "0" 表示默认经验倍率
            return "0".to_string();
        }
        serde_json::to_string(self).unwrap_or_else(|_| "0".to_string())
    }

    /// 是否有 boss 配置
    pub fn has_boss(&self) -> bool {
        !self.bosses.is_empty()
    }
}
