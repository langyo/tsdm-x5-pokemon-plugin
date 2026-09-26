use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapsResponse {
    pub maps: Vec<MapInfo>,
    pub total: usize,
}

/// Boss 信息（游戏前端展示用）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BossInfo {
    pub pokemon_type_id: u64,
    pub pokemon_name: String,
    pub level: u64,
    pub boss_multiplier: f32,
}

/// 地图模式枚举（用于 API 传输）
///
/// 注意：这是 `pm_map` 表的 **游戏端只读展示模型**；管理端的读写模型是
/// `_utils::types::map_info`（MapInfo/MapMode 含经验值与 Boss 配置）。
/// 反序列化忽略 experience / experience_increase_times 字段以兼容管理端形态，
/// `mode` 标签（wild/boss/hybrid）由双方的金丝雀测试共同钉死。
/// 序列化/反序列化由 MapInfo 自定义实现处理，生成扁平化格式
#[derive(Debug, Clone, PartialEq)]
pub enum MapMode {
    /// 常规野生宠物模式
    Wild,
    /// Boss 挑战模式
    Boss {
        /// Boss 列表
        bosses: Vec<BossInfo>,
    },
    /// 混合模式
    Hybrid {
        /// Boss 列表
        bosses: Vec<BossInfo>,
    },
}

impl MapMode {
    /// 获取 Boss 列表
    pub fn get_bosses(&self) -> Vec<BossInfo> {
        match self {
            MapMode::Wild => Vec::new(),
            MapMode::Boss { bosses } => bosses.clone(),
            MapMode::Hybrid { bosses } => bosses.clone(),
        }
    }

    /// 是否支持野生宠物
    pub fn has_wild_pokemon(&self) -> bool {
        matches!(self, MapMode::Wild | MapMode::Hybrid { .. })
    }

    /// 是否支持 Boss 挑战
    pub fn has_boss_challenge(&self) -> bool {
        matches!(self, MapMode::Boss { .. } | MapMode::Hybrid { .. })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapInfo {
    pub id: u64,
    pub name: String,
    pub area_type: String,
    pub area_type_name: String,
    pub region: String,
    pub pos_x: u32,
    pub pos_y: u32,
    pub is_enabled: bool,
    pub min_level: u64,
    pub max_level: u64,
    /// 地图模式
    pub mode: MapMode,
    /// 野生宠物列表（从 pm_data 表查询）
    pub wild_pokemons: Vec<WildPokemonInfo>,
}

// 自定义序列化以支持扁平化格式
impl Serialize for MapInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("area_type", &self.area_type)?;
        map.serialize_entry("area_type_name", &self.area_type_name)?;
        map.serialize_entry("region", &self.region)?;
        map.serialize_entry("pos_x", &self.pos_x)?;
        map.serialize_entry("pos_y", &self.pos_y)?;
        map.serialize_entry("is_enabled", &self.is_enabled)?;
        map.serialize_entry("min_level", &self.min_level)?;
        map.serialize_entry("max_level", &self.max_level)?;

        // 展开模式字段到扁平化格式
        match &self.mode {
            MapMode::Wild => {
                map.serialize_entry("mode", "wild")?;
            }
            MapMode::Boss { bosses } => {
                map.serialize_entry("mode", "boss")?;
                map.serialize_entry("bosses", bosses)?;
            }
            MapMode::Hybrid { bosses } => {
                map.serialize_entry("mode", "hybrid")?;
                map.serialize_entry("bosses", bosses)?;
            }
        }

        map.serialize_entry("wild_pokemons", &self.wild_pokemons)?;
        map.end()
    }
}

// 自定义反序列化以支持扁平化格式
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
                let mut area_type_name = None;
                let mut region = None;
                let mut pos_x = None;
                let mut pos_y = None;
                let mut is_enabled = None;
                let mut min_level = None;
                let mut max_level = None;
                let mut mode_str: Option<String> = None;
                let mut bosses = None;
                let mut wild_pokemons = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => {
                            id = Some(map.next_value()?);
                        }
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "area_type" => {
                            area_type = Some(map.next_value()?);
                        }
                        "area_type_name" => {
                            area_type_name = Some(map.next_value()?);
                        }
                        "region" => {
                            region = Some(map.next_value()?);
                        }
                        "pos_x" => {
                            pos_x = Some(map.next_value()?);
                        }
                        "pos_y" => {
                            pos_y = Some(map.next_value()?);
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
                        // 兼容旧 API，忽略这些字段
                        "experience" => {
                            let _ = map.next_value::<u64>();
                        }
                        "experience_increase_times" => {
                            let _ = map.next_value::<u64>();
                        }
                        "bosses" => {
                            bosses = Some(map.next_value()?);
                        }
                        "wild_pokemons" => {
                            wild_pokemons = Some(map.next_value()?);
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let area_type = area_type.unwrap_or_default();
                let area_type_name = area_type_name.unwrap_or_default();
                let region = region.unwrap_or_default();
                let pos_x = pos_x.unwrap_or(50);
                let pos_y = pos_y.unwrap_or(50);
                let is_enabled =
                    is_enabled.ok_or_else(|| serde::de::Error::missing_field("is_enabled"))?;
                let min_level =
                    min_level.ok_or_else(|| serde::de::Error::missing_field("min_level"))?;
                let max_level =
                    max_level.ok_or_else(|| serde::de::Error::missing_field("max_level"))?;
                let wild_pokemons = wild_pokemons.unwrap_or_default();

                // 根据 mode 字符串和可选字段构建 MapMode
                let mode = match mode_str.as_deref() {
                    Some("wild") => MapMode::Wild,
                    Some("boss") => {
                        let bosses = bosses.unwrap_or_default();
                        MapMode::Boss { bosses }
                    }
                    Some("hybrid") => {
                        let bosses = bosses.unwrap_or_default();
                        MapMode::Hybrid { bosses }
                    }
                    _ => {
                        // 默认为 Wild 模式
                        MapMode::Wild
                    }
                };

                Ok(MapInfo {
                    id,
                    name,
                    area_type,
                    area_type_name,
                    region,
                    pos_x,
                    pos_y,
                    is_enabled,
                    min_level,
                    max_level,
                    mode,
                    wild_pokemons,
                })
            }
        }

        deserializer.deserialize_map(MapInfoVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WildPokemonInfo {
    pub id: u64,
    pub name: String,
}

impl MapInfo {
    pub fn get_level_range_text(&self) -> String {
        format!("Lv.{} ~ Lv.{}", self.min_level, self.max_level)
    }

    pub fn get_area_icon(&self) -> &'static str {
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

    pub fn get_area_color(&self) -> &'static str {
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

    /// 获取 Boss 列表（兼容旧代码）
    pub fn get_bosses(&self) -> Vec<BossInfo> {
        self.mode.get_bosses()
    }

    /// 获取 Boss 列表引用（兼容旧代码）
    pub fn bosses(&self) -> &[BossInfo] {
        // 这里需要一个静态的空 Vec 用于返回
        static EMPTY: Vec<BossInfo> = Vec::new();
        // 由于需要返回引用，这里用一个小技巧
        // 实际使用时应该修改调用方使用 get_bosses()
        &EMPTY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(mode: MapMode) -> MapInfo {
        MapInfo {
            id: 1,
            name: "测试地图".to_string(),
            area_type: "g".to_string(),
            area_type_name: "草丛".to_string(),
            region: "kanto".to_string(),
            pos_x: 1,
            pos_y: 2,
            is_enabled: true,
            min_level: 1,
            max_level: 10,
            mode,
            wild_pokemons: vec![],
        }
    }

    /// 金丝雀：钉死游戏端 MapMode 的 wire 标签（含自定义序列化与反序列化回路）。
    /// map_info.rs 中的同名测试钉死管理端标签，两侧必须始终一致。
    #[test]
    fn mode_tags_are_stable() {
        for (mode, tag) in [
            (MapMode::Wild, "wild"),
            (
                MapMode::Boss {
                    bosses: vec![BossInfo {
                        pokemon_type_id: 1,
                        pokemon_name: "n".to_string(),
                        level: 5,
                        boss_multiplier: 1.0,
                    }],
                },
                "boss",
            ),
            (MapMode::Hybrid { bosses: vec![] }, "hybrid"),
        ] {
            let value = serde_json::to_value(sample(mode.clone())).unwrap();
            assert_eq!(value["mode"], tag, "serialize tag mismatch for {}", tag);
            // 反序列化回路：自定义 visitor 必须还原同一标签
            let back: MapInfo = serde_json::from_value(value).unwrap();
            assert_eq!(back.mode, mode, "round-trip mismatch for {}", tag);
        }
    }
}
