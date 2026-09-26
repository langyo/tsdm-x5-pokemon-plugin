use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use super::map_boss::MapBossConfig;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, EnumString, EnumIter, Display)]
pub enum MapAreaType {
    #[serde(rename = "l")]
    #[strum(to_string = "平原")]
    Plain,
    #[serde(rename = "g")]
    #[strum(to_string = "草丛")]
    Grass,
    #[serde(rename = "p")]
    #[strum(to_string = "水池")]
    Water,
    #[serde(rename = "s")]
    #[strum(to_string = "海洋")]
    Sea,
    #[serde(rename = "b")]
    #[strum(to_string = "海底")]
    SeaBottom,
    #[serde(rename = "m")]
    #[strum(to_string = "山谷")]
    Mountain,
    #[serde(rename = "c")]
    #[strum(to_string = "山洞")]
    Cave,
    #[serde(rename = "d")]
    #[strum(to_string = "沙漠")]
    Sand,
    #[serde(rename = "f")]
    #[strum(to_string = "工厂")]
    Factory,
    #[serde(rename = "t")]
    #[strum(to_string = "基地")]
    Base,
    #[serde(rename = "v")]
    #[strum(to_string = "市镇")]
    Town,
    #[serde(rename = "n")]
    #[strum(to_string = "道馆")]
    Gym,
    #[serde(rename = "h")]
    #[strum(to_string = "天空")]
    Sky,
    #[serde(rename = "o")]
    #[strum(to_string = "深海")]
    DeepSea,
    #[serde(rename = "k")]
    #[strum(to_string = "熔岩")]
    Lava,
}

/// 地图模式枚举：表示地图的冒险模式类型
///
/// 注意：这是 `pm_map` 表的 **管理端读写模型**；游戏前端的只读展示模型是
/// `_utils::types::api_map`（MapInfo/MapMode 带 Boss 展示信息与坐标）。
/// 两端通过 wire 上的 `mode` 标签（wild/boss/hybrid）耦合，标签值由
/// 双方的 `mode_tags_are_stable` 金丝雀测试共同钉死，改动任一侧必须同步另一侧。
/// 使用外部标记 (tag = "mode") 生成扁平化 JSON 格式：
/// - Wild: {"mode":"wild","experience":X,"experience_increase_times":Y}
/// - Boss: {"mode":"boss","bosses":[...]}
/// - Hybrid: {"mode":"hybrid","experience":X,"experience_increase_times":Y,"bosses":[...]}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum MapMode {
    /// 常规野生宠物模式 - 可以遇到该地图的野生宠物
    #[serde(rename = "wild")]
    Wild {
        experience: u64,
        experience_increase_times: u64,
    },
    /// Boss 挑战模式 - 可以挑战配置的 Boss（同时仍然可以遇到野生宠物）
    #[serde(rename = "boss")]
    Boss { bosses: MapBossConfig },
    /// 混合模式 - 同时支持野生宠物和 Boss 挑战
    #[serde(rename = "hybrid")]
    Hybrid {
        experience: u64,
        experience_increase_times: u64,
        bosses: MapBossConfig,
    },
}

impl MapMode {
    /// 获取基础经验值（如果适用）
    pub fn get_experience(&self) -> Option<u64> {
        match self {
            MapMode::Wild { experience, .. } => Some(*experience),
            MapMode::Hybrid { experience, .. } => Some(*experience),
            MapMode::Boss { .. } => None,
        }
    }

    /// 获取经验倍率（如果适用）
    pub fn get_experience_increase_times(&self) -> Option<u64> {
        match self {
            MapMode::Wild {
                experience_increase_times,
                ..
            } => Some(*experience_increase_times),
            MapMode::Hybrid {
                experience_increase_times,
                ..
            } => Some(*experience_increase_times),
            MapMode::Boss { .. } => None,
        }
    }

    /// 获取 Boss 配置（如果适用）
    pub fn get_boss_config(&self) -> Option<&MapBossConfig> {
        match self {
            MapMode::Wild { .. } => None,
            MapMode::Boss { bosses } => Some(bosses),
            MapMode::Hybrid { bosses, .. } => Some(bosses),
        }
    }

    /// 是否支持野生宠物
    pub fn has_wild_pokemon(&self) -> bool {
        matches!(self, MapMode::Wild { .. } | MapMode::Hybrid { .. })
    }

    /// 是否支持 Boss 挑战
    pub fn has_boss_challenge(&self) -> bool {
        matches!(self, MapMode::Boss { .. } | MapMode::Hybrid { .. })
    }

    /// 从数据库字段反序列化
    /// exp: -1 表示 Boss 模式，其他值表示经验值
    /// expn: JSON 字符串表示 Boss 配置，数字表示经验倍率
    pub fn from_db_fields(exp: i64, expn: String) -> Self {
        // 尝试解析 expn 为 Boss 配置
        let boss_config = MapBossConfig::from_expn(expn.clone());
        let has_boss = boss_config.has_boss();

        match (exp, has_boss) {
            // exp == -1 且有 boss 配置 -> Boss 模式
            (-1, true) => MapMode::Boss {
                bosses: boss_config,
            },
            // exp == -1 但无 boss 配置 -> 默认为 Wild 模式（数据异常情况）
            (-1, false) => MapMode::Wild {
                experience: 0,
                experience_increase_times: expn.parse().unwrap_or(0),
            },
            // exp != -1 且有 boss 配置 -> 混合模式
            (_, true) => MapMode::Hybrid {
                experience: exp.max(0) as u64,
                experience_increase_times: 0, // expn 已被用于解析 boss 配置
                bosses: boss_config,
            },
            // exp != -1 且无 boss 配置 -> 野生模式
            (_, false) => MapMode::Wild {
                experience: exp.max(0) as u64,
                experience_increase_times: expn.parse().unwrap_or(0),
            },
        }
    }

    /// 序列化为数据库字段
    pub fn to_db_fields(&self) -> (i64, String) {
        match self {
            MapMode::Wild {
                experience,
                experience_increase_times,
            } => (*experience as i64, experience_increase_times.to_string()),
            MapMode::Boss { bosses } => (-1, bosses.to_expn()),
            MapMode::Hybrid {
                experience, bosses, ..
            } => (*experience as i64, bosses.to_expn()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapInfo {
    pub id: u64,
    pub name: String,
    pub area_type: MapAreaType, // 对应列 site

    pub is_enabled: bool, // 对应列 kg
    pub min_level: u64,
    pub max_level: u64,
    /// 地图模式（包含经验值、Boss 配置等信息）
    pub mode: MapMode,
}

impl Serialize for MapInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        // 手动实现序列化以支持扁平化格式
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("area_type", &self.area_type)?;
        map.serialize_entry("is_enabled", &self.is_enabled)?;
        map.serialize_entry("min_level", &self.min_level)?;
        map.serialize_entry("max_level", &self.max_level)?;

        // 展开模式字段
        match &self.mode {
            MapMode::Wild {
                experience,
                experience_increase_times,
            } => {
                map.serialize_entry("mode", "wild")?;
                map.serialize_entry("experience", experience)?;
                map.serialize_entry("experience_increase_times", experience_increase_times)?;
            }
            MapMode::Boss { bosses } => {
                map.serialize_entry("mode", "boss")?;
                // 直接序列化 bosses 数组，而不是 MapBossConfig 对象
                map.serialize_entry("bosses", &bosses.bosses)?;
            }
            MapMode::Hybrid {
                experience,
                experience_increase_times,
                bosses,
            } => {
                map.serialize_entry("mode", "hybrid")?;
                map.serialize_entry("experience", experience)?;
                map.serialize_entry("experience_increase_times", experience_increase_times)?;
                // 直接序列化 bosses 数组，而不是 MapBossConfig 对象
                map.serialize_entry("bosses", &bosses.bosses)?;
            }
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for MapInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct MapInfoVisitor;

        impl<'de> Visitor<'de> for MapInfoVisitor {
            type Value = MapInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MapInfo")
            }

            fn visit_map<A>(self, mut map: A) -> Result<MapInfo, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut area_type = None;
                let mut is_enabled = None;
                let mut min_level = None;
                let mut max_level = None;
                let mut mode_str: Option<String> = None;
                let mut experience = None;
                let mut experience_increase_times = None;
                let mut bosses: Option<Vec<super::map_boss::MapBoss>> = None;

                while let Some(key) = map.next_key::<String>()? {
                    let key_str = key.as_str();
                    match key_str {
                        "id" => {
                            id = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "area_type" => {
                            area_type = Some(map.next_value()?);
                        }
                        "is_enabled" => {
                            is_enabled = Some(map.next_value()?);
                        }
                        "min_level" => {
                            min_level = Some(map.next_value()?);
                        }
                        "max_level" => {
                            max_level = Some(map.next_value()?);
                        }
                        "mode" => {
                            mode_str = Some(map.next_value()?);
                        }
                        "experience" => {
                            experience = Some(map.next_value()?);
                        }
                        "experience_increase_times" => {
                            experience_increase_times = Some(map.next_value()?);
                        }
                        "bosses" => {
                            // Handle potential malformed data - if bosses is an object instead of array, skip it
                            let value: serde_json::Value = map.next_value()?;
                            match value {
                                serde_json::Value::Array(arr) => {
                                    bosses = Some(
                                        arr.into_iter()
                                            .filter_map(|v| serde_json::from_value(v).ok())
                                            .collect(),
                                    );
                                }
                                serde_json::Value::Object(_obj) => {
                                    // Treat as empty array
                                    bosses = Some(Vec::new());
                                }
                                serde_json::Value::Null => {
                                    bosses = Some(Vec::new());
                                }
                                _ => {
                                    return Err(serde::de::Error::custom(format!(
                                        "bosses must be an array or null, got {:?}",
                                        value
                                    )));
                                }
                            }
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let area_type =
                    area_type.ok_or_else(|| serde::de::Error::missing_field("area_type"))?;
                let is_enabled =
                    is_enabled.ok_or_else(|| serde::de::Error::missing_field("is_enabled"))?;
                let min_level =
                    min_level.ok_or_else(|| serde::de::Error::missing_field("min_level"))?;
                let max_level =
                    max_level.ok_or_else(|| serde::de::Error::missing_field("max_level"))?;
                let mode_str = mode_str.ok_or_else(|| serde::de::Error::missing_field("mode"))?;

                // 根据 mode 字符串和可选字段构建 MapMode
                let mode = match mode_str.as_str() {
                    "wild" => {
                        let experience = experience.unwrap_or(0);
                        let experience_increase_times = experience_increase_times.unwrap_or(0);
                        MapMode::Wild {
                            experience,
                            experience_increase_times,
                        }
                    }
                    "boss" => {
                        // bosses 字段现在是扁平化的 Vec<MapBoss>
                        let boss_list = bosses.unwrap_or_default();
                        MapMode::Boss {
                            bosses: MapBossConfig { bosses: boss_list },
                        }
                    }
                    "hybrid" => {
                        let experience = experience.unwrap_or(0);
                        let experience_increase_times = experience_increase_times.unwrap_or(0);
                        // bosses 字段现在是扁平化的 Vec<MapBoss>
                        let boss_list = bosses.unwrap_or_default();
                        MapMode::Hybrid {
                            experience,
                            experience_increase_times,
                            bosses: MapBossConfig { bosses: boss_list },
                        }
                    }
                    _other => {
                        return Err(serde::de::Error::unknown_variant(
                            &mode_str,
                            &["wild", "boss", "hybrid"],
                        ));
                    }
                };

                Ok(MapInfo {
                    id,
                    name,
                    area_type,
                    is_enabled,
                    min_level,
                    max_level,
                    mode,
                })
            }
        }

        deserializer.deserialize_map(MapInfoVisitor)
    }
}

impl MapInfo {
    const _TYPE: &'static str = "map_info";
}

impl Default for MapInfo {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_owned(),
            area_type: MapAreaType::Plain,
            is_enabled: false,
            min_level: 0,
            max_level: 0,
            mode: MapMode::Wild {
                experience: 0,
                experience_increase_times: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 金丝雀：钉死管理端 MapMode 的 wire 标签。
    /// api_map.rs 中的同名测试钉死游戏端标签，两侧必须始终一致。
    #[test]
    fn mode_tags_are_stable() {
        let wild = serde_json::to_value(MapMode::Wild {
            experience: 5,
            experience_increase_times: 2,
        })
        .unwrap();
        assert_eq!(wild["mode"], "wild");

        let boss = serde_json::to_value(MapMode::Boss {
            bosses: MapBossConfig::default(),
        })
        .unwrap();
        assert_eq!(boss["mode"], "boss");

        let hybrid = serde_json::to_value(MapMode::Hybrid {
            experience: 5,
            experience_increase_times: 2,
            bosses: MapBossConfig::default(),
        })
        .unwrap();
        assert_eq!(hybrid["mode"], "hybrid");
    }
}
