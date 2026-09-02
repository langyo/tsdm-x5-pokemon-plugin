use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::dioxus::state::FilterPackage;
use _utils::types::{
    api_map::WildPokemonInfo,
    evolution_info::EvolutionInfo,
    global_config::GlobalConfigType,
    item_info::ItemInfo,
    item_type::ItemType,
    map_boss::{MapBoss, MapBossConfig},
    map_info::MapInfo,
    pokemon_info::PokemonInfo,
    pokemon_type::PokemonType,
    skill_type::SkillType,
    user_info::UserInfo,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct RawString {
    raw: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct TaggedGlobalConfig {
    marker: (),
    config: GlobalConfigType,
}

impl<'de> Deserialize<'de> for TaggedGlobalConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct TaggedGlobalConfigVisitor;

        impl<'de> Visitor<'de> for TaggedGlobalConfigVisitor {
            type Value = TaggedGlobalConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TaggedGlobalConfig with _TYPE field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                use serde::de::Error;

                let mut found_type = false;
                let mut fields: Vec<(String, serde_json::Value)> = Vec::new();

                while let Some(key) = map.next_key::<String>()? {
                    let value = map.next_value::<serde_json::Value>()?;
                    if key == "_TYPE" {
                        if let Some(str_val) = value.as_str() {
                            if str_val != "global_config" {
                                return Err(A::Error::custom(format!(
                                    "_TYPE 不匹配，期望 global_config，实际为 {str_val}"
                                )));
                            }
                            found_type = true;
                        }
                    } else {
                        fields.push((key, value));
                    }
                }

                if !found_type {
                    return Err(A::Error::missing_field("_TYPE"));
                }

                let mut obj = serde_json::Map::new();
                for (key, value) in fields {
                    obj.insert(key, value);
                }

                let config = GlobalConfigType::deserialize(serde_json::Value::Object(obj))
                    .map_err(A::Error::custom)?;

                Ok(TaggedGlobalConfig { marker: (), config })
            }
        }

        deserializer.deserialize_map(TaggedGlobalConfigVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct TaggedWildPokemonInfo {
    marker: (),
    info: WildPokemonInfo,
}

impl<'de> Deserialize<'de> for TaggedWildPokemonInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct TaggedWildPokemonInfoVisitor;

        impl<'de> Visitor<'de> for TaggedWildPokemonInfoVisitor {
            type Value = TaggedWildPokemonInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TaggedWildPokemonInfo with _TYPE field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                use serde::de::Error;

                let mut found_type = false;
                let mut fields: Vec<(String, serde_json::Value)> = Vec::new();

                while let Some(key) = map.next_key::<String>()? {
                    let value = map.next_value::<serde_json::Value>()?;
                    if key == "_TYPE" {
                        if let Some(str_val) = value.as_str() {
                            if str_val != "wild_pokemon_info" {
                                return Err(A::Error::custom(format!(
                                    "_TYPE 不匹配，期望 wild_pokemon_info，实际为 {str_val}"
                                )));
                            }
                            found_type = true;
                        }
                    } else {
                        fields.push((key, value));
                    }
                }

                if !found_type {
                    return Err(A::Error::missing_field("_TYPE"));
                }

                let mut obj = serde_json::Map::new();
                for (key, value) in fields {
                    obj.insert(key, value);
                }

                let info = WildPokemonInfo::deserialize(serde_json::Value::Object(obj))
                    .map_err(A::Error::custom)?;

                Ok(TaggedWildPokemonInfo { marker: (), info })
            }
        }

        deserializer.deserialize_map(TaggedWildPokemonInfoVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct TaggedMapInfo {
    marker: (),
    info: MapInfo,
}

impl<'de> Deserialize<'de> for TaggedMapInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        struct TaggedMapInfoVisitor;

        impl<'de> Visitor<'de> for TaggedMapInfoVisitor {
            type Value = TaggedMapInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TaggedMapInfo with _TYPE field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                use serde::de::Error;

                // Collect all key-value pairs except _TYPE
                let mut found_type = false;
                let mut fields: Vec<(String, serde_json::Value)> = Vec::new();

                while let Some(key) = map.next_key::<String>()? {
                    let value = map.next_value::<serde_json::Value>()?;
                    if key == "_TYPE" {
                        if let Some(str_val) = value.as_str() {
                            if str_val != "map_info" {
                                return Err(A::Error::custom(format!(
                                    "_TYPE 不匹配，期望 map_info，实际为 {str_val}"
                                )));
                            }
                            found_type = true;
                        }
                    } else {
                        fields.push((key, value));
                    }
                }

                if !found_type {
                    return Err(A::Error::missing_field("_TYPE"));
                }

                // Create a serde_json::Value from the collected fields
                let mut obj = serde_json::Map::new();
                for (key, value) in fields {
                    obj.insert(key, value);
                }

                // Deserialize using the serde_json::Value
                let info = MapInfo::deserialize(serde_json::Value::Object(obj)).map_err(|e| {
                    A::Error::custom(format!("MapInfo deserialization failed: {}", e))
                })?;

                Ok(TaggedMapInfo { marker: (), info })
            }
        }

        deserializer.deserialize_map(TaggedMapInfoVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
enum RetStruct {
    GetGlobalConfig(TaggedGlobalConfig),
    WildPokemonInfo(TaggedWildPokemonInfo),
    GetMapInfo(TaggedMapInfo),
    GetEvolutionInfo(EvolutionInfo),
    GetItemType(ItemType),
    GetPokemonInfo(PokemonInfo),
    GetPokemonType(PokemonType),
    GetSkillType(SkillType),
    GetItemInfo(ItemInfo),
    GetUserInfo(UserInfo),
    Count(CountResult),
    RawResult(RawString),
}

impl<'de> Deserialize<'de> for RetStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};
        use std::fmt;

        // 首先尝试解析为通用 JSON 值来检查 _TYPE 字段
        struct RetStructVisitor;

        impl<'de> Visitor<'de> for RetStructVisitor {
            type Value = RetStruct;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any of the RetStruct variants")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                use serde::de::Error;

                // 收集所有数据
                let mut all_data = serde_json::Map::new();
                let mut type_value = None;

                while let Some(key) = map.next_key::<String>()? {
                    let value = map.next_value::<serde_json::Value>()?;
                    if key == "_TYPE" {
                        if let Some(str_val) = value.as_str() {
                            type_value = Some(str_val.to_string());
                        }
                    }
                    all_data.insert(key, value);
                }

                // 根据 _TYPE 字段选择变体
                if let Some(t) = type_value {
                    // Check for :: types first (before moving all_data)
                    if t.starts_with("::") {
                        // For untagged variants, remove _TYPE
                        all_data.remove("_TYPE");
                        let full_data = serde_json::Value::Object(all_data);
                        // Try count first (most common for :: prefixed types)
                        if let Ok(item) = CountResult::deserialize(full_data.clone()) {
                            return Ok(RetStruct::Count(item));
                        }
                        // Try other untagged variants
                        if let Ok(item) = RawString::deserialize(full_data.clone()) {
                            return Ok(RetStruct::RawResult(item));
                        }
                        return Err(A::Error::custom(format!("无法解析 :: 类型: {}", t)));
                    }

                    // Remove _TYPE for all other types
                    all_data.remove("_TYPE");
                    let data = serde_json::Value::Object(all_data);

                    // Handle each type
                    match t.as_str() {
                        "global_config" => {
                            let config =
                                GlobalConfigType::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetGlobalConfig(TaggedGlobalConfig {
                                marker: (),
                                config,
                            }));
                        }
                        "wild_pokemon_info" => {
                            let info =
                                WildPokemonInfo::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::WildPokemonInfo(TaggedWildPokemonInfo {
                                marker: (),
                                info,
                            }));
                        }
                        "map_info" => {
                            let info = MapInfo::deserialize(data).map_err(|e| {
                                A::Error::custom(format!("MapInfo deserialization failed: {}", e))
                            })?;
                            return Ok(RetStruct::GetMapInfo(TaggedMapInfo { marker: (), info }));
                        }
                        "pokemon_type" => {
                            let item = PokemonType::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetPokemonType(item));
                        }
                        "pokemon_info" => {
                            let item = PokemonInfo::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetPokemonInfo(item));
                        }
                        "pokemon" => {
                            let item = PokemonInfo::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetPokemonInfo(item));
                        }
                        "item_type" => {
                            let item = ItemType::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetItemType(item));
                        }
                        "item_info" => {
                            let item = ItemInfo::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetItemInfo(item));
                        }
                        "evolution_info" => {
                            let item =
                                EvolutionInfo::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetEvolutionInfo(item));
                        }
                        "skill_type" => {
                            let item = SkillType::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetSkillType(item));
                        }
                        "user_info" => {
                            let item = UserInfo::deserialize(data).map_err(A::Error::custom)?;
                            return Ok(RetStruct::GetUserInfo(item));
                        }
                        _ => {
                            return Err(A::Error::custom(format!("未知的 _TYPE 值: {}", t)));
                        }
                    }
                }

                // 没有 _TYPE 字段，尝试各个不带标签的变体
                let full_data = serde_json::to_value(all_data).map_err(A::Error::custom)?;

                // 尝试按顺序解析各个变体
                if let Ok(item) = EvolutionInfo::deserialize(full_data.clone()) {
                    return Ok(RetStruct::GetEvolutionInfo(item));
                }
                if let Ok(item) = ItemType::deserialize(full_data.clone()) {
                    return Ok(RetStruct::GetItemType(item));
                }
                if let Ok(item) = PokemonInfo::deserialize(full_data.clone()) {
                    return Ok(RetStruct::GetPokemonInfo(item));
                }
                if let Ok(item) = PokemonType::deserialize(full_data.clone()) {
                    return Ok(RetStruct::GetPokemonType(item));
                }
                if let Ok(item) = SkillType::deserialize(full_data.clone()) {
                    return Ok(RetStruct::GetSkillType(item));
                }
                if let Ok(item) = ItemInfo::deserialize(full_data.clone()) {
                    return Ok(RetStruct::GetItemInfo(item));
                }
                if let Ok(item) = UserInfo::deserialize(full_data.clone()) {
                    return Ok(RetStruct::GetUserInfo(item));
                }
                if let Ok(item) = CountResult::deserialize(full_data.clone()) {
                    return Ok(RetStruct::Count(item));
                }
                if let Ok(item) = RawString::deserialize(full_data.clone()) {
                    return Ok(RetStruct::RawResult(item));
                }

                Err(A::Error::custom("数据不匹配任何 RetStruct 变体"))
            }
        }

        deserializer.deserialize_map(RetStructVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct RetPackage {
    success: bool,
    data: Option<Vec<RetStruct>>,
    reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct CountResult {
    count: u64,
}

fn get_admin_endpoint() -> Result<String> {
    let js_code = r#"(() => {
        const url = new URL(window.location.href);
        url.hash = '';
        url.searchParams.set('id', 'pokemon:pokemon');
        url.searchParams.set('index', 'admin');
        return url.toString();
    })()"#
        .to_string();
    match js_sys::eval(js_code.as_str()) {
        Ok(value) => value
            .as_string()
            .ok_or_else(|| anyhow!("获取后台接口地址失败")),
        Err(error) => Err(anyhow!("获取后台接口地址失败")),
    }
}

fn strip_script_prefix(mut text: &str) -> &str {
    let script_end = "</script>";
    loop {
        let trimmed = text.trim_start();
        if !trimmed.starts_with("<script") {
            return trimmed;
        }
        if let Some(end) = trimmed.find(script_end) {
            text = &trimmed[end + script_end.len()..];
            continue;
        }
        return trimmed;
    }
}

fn extract_json_payload(text: &str) -> &str {
    let trimmed = strip_script_prefix(text).trim();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        return trimmed;
    }

    if let Some(idx) = trimmed.find('{').or_else(|| trimmed.find('[')) {
        return trimmed[idx..].trim();
    }

    trimmed
}

fn response_preview(text: &str) -> String {
    const MAX_CHARS: usize = 240;
    let mut preview = String::new();
    for ch in text.chars().take(MAX_CHARS) {
        preview.push(ch);
    }
    if text.chars().count() > MAX_CHARS {
        preview.push_str("...");
    }
    preview
}

async fn fetch(action: &str, val: HashMap<String, String>) -> Result<RetPackage> {
    let client = reqwest::Client::new();

    // 构建请求数据，直接使用 JSON 格式
    let mut request_data = serde_json::Map::new();
    request_data.insert(
        "action".to_string(),
        serde_json::Value::String(action.to_string()),
    );
    for (k, v) in val {
        request_data.insert(k, serde_json::Value::String(v));
    }

    let endpoint = get_admin_endpoint().context("无法获取后台接口地址")?;
    let response = client
        .post(endpoint)
        .json(&request_data)
        .send()
        .await
        .context("网络请求失败")?;
    let text = response.text().await.context("无法读取响应内容")?;
    let payload = extract_json_payload(&text);

    serde_json::from_str(payload).map_err(|error| {
        if text.contains("Database Error") || text.contains("数据库错误") {
            anyhow!("数据库错误：服务器返回了错误页面，可能是 SQL 语法错误或表名不存在")
        } else {
            anyhow!(
                "无法解析返回的数据包为 JSON：{}；解析错误：{}",
                response_preview(&text),
                error
            )
        }
    })
}

pub async fn get_global_config() -> Result<GlobalConfigType> {
    let data = fetch("list::global_config", HashMap::new()).await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetGlobalConfig(config)) => Ok(config.config),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_global_config(config: GlobalConfigType) -> Result<GlobalConfigType> {
    let data = fetch(
        "set::global_config",
        vec![("data".to_string(), serde_json::to_string(&config)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetGlobalConfig(saved)) => Ok(saved.config),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn run_sql(sql: String) -> Result<String> {
    let data = fetch(
        "run::sql_console",
        vec![("sql".to_string(), sql)].into_iter().collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::RawResult(result)) => Ok(result.raw),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn count_item_type() -> Result<u64> {
    let data = fetch("count::item_type", HashMap::new()).await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::Count(result)) => Ok(result.count),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_item_type(from: u64, count: u64) -> Result<Vec<ItemType>> {
    let data = fetch(
        "list::item_type",
        vec![
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetItemType(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn filter_item_type(filters: Vec<FilterPackage>) -> Result<Vec<ItemType>> {
    let data = fetch(
        "filter::item_type",
        vec![("filters".to_string(), serde_json::to_string(&filters)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetItemType(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_item_type(info: ItemType) -> Result<ItemType> {
    let data = fetch(
        "set::item_type",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetItemType(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_item_type(info: ItemType) -> Result<ItemType> {
    let data = fetch(
        "insert::item_type",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetItemType(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn delete_item_type(id: u64) -> Result<()> {
    let data = fetch(
        "delete::item_type",
        vec![("id".to_string(), id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn count_map_info() -> Result<u64> {
    let data = fetch("count::map_info", HashMap::new()).await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::Count(result)) => Ok(result.count),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_map_info(from: u64, count: u64) -> Result<Vec<MapInfo>> {
    let data = fetch(
        "list::map_info",
        vec![
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetMapInfo(tagged) => Ok(tagged.info),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn filter_map_info(filters: Vec<FilterPackage>) -> Result<Vec<MapInfo>> {
    let data = fetch(
        "filter::map_info",
        vec![("filters".to_string(), serde_json::to_string(&filters)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetMapInfo(tagged) => Ok(tagged.info),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_map_info(info: MapInfo) -> Result<MapInfo> {
    let json_string = serde_json::to_string(&info)?;

    let data = fetch(
        "set::map_info",
        vec![("data".to_string(), json_string)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetMapInfo(tagged)) => Ok(tagged.info),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_map_info(info: MapInfo) -> Result<MapInfo> {
    let data = fetch(
        "insert::map_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetMapInfo(tagged)) => Ok(tagged.info),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn delete_map_info(id: u64) -> Result<()> {
    let data = fetch(
        "delete::map_info",
        vec![("id".to_string(), id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn count_pokemon_type() -> Result<u64> {
    let data = fetch("count::pokemon_type", HashMap::new()).await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::Count(result)) => Ok(result.count),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_pokemon_type(from: u64, count: u64) -> Result<Vec<PokemonType>> {
    let data = fetch(
        "list::pokemon_type",
        vec![
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .filter_map(|item| match item {
                RetStruct::GetPokemonType(value) => Some(Ok(value)),
                RetStruct::GetMapInfo(_) => None,
                other => Some(Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other))),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn filter_pokemon_type(filters: Vec<FilterPackage>) -> Result<Vec<PokemonType>> {
    let data = fetch(
        "filter::pokemon_type",
        vec![("filters".to_string(), serde_json::to_string(&filters)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetPokemonType(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn get_pokemon_types_by_ids(ids: Vec<u64>) -> Result<Vec<WildPokemonInfo>> {
    let data = fetch(
        "get_pokemon_types_by_ids",
        vec![("ids".to_string(), serde_json::to_string(&ids)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => {
            let results: Vec<WildPokemonInfo> = items
                .into_iter()
                .filter_map(|item| match item {
                    RetStruct::WildPokemonInfo(tagged) => Some(tagged.info),
                    _ => None,
                })
                .collect();
            Ok(results)
        }
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_pokemon_type(info: PokemonType) -> Result<PokemonType> {
    let data = fetch(
        "set::pokemon_type",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetPokemonType(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_pokemon_type(info: PokemonType) -> Result<PokemonType> {
    let data = fetch(
        "insert::pokemon_type",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetPokemonType(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn delete_pokemon_type(id: u64) -> Result<()> {
    let data = fetch(
        "delete::pokemon_type",
        vec![("id".to_string(), id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn count_evolution_info() -> Result<u64> {
    let data = fetch("count::evolution_info", HashMap::new()).await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::Count(result)) => Ok(result.count),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_evolution_info(from: u64, count: u64) -> Result<Vec<EvolutionInfo>> {
    let data = fetch(
        "list::evolution_info",
        vec![
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetEvolutionInfo(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn filter_evolution_info(filters: Vec<FilterPackage>) -> Result<Vec<EvolutionInfo>> {
    let data = fetch(
        "filter::evolution_info",
        vec![("filters".to_string(), serde_json::to_string(&filters)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetEvolutionInfo(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_evolution_info(info: EvolutionInfo) -> Result<EvolutionInfo> {
    let data = fetch(
        "set::evolution_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetEvolutionInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_evolution_info(info: EvolutionInfo) -> Result<EvolutionInfo> {
    let data = fetch(
        "insert::evolution_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetEvolutionInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn delete_evolution_info(id: u64) -> Result<()> {
    let data = fetch(
        "delete::evolution_info",
        vec![("id".to_string(), id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn count_skill_type() -> Result<u64> {
    let data = fetch("count::skill_type", HashMap::new()).await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::Count(result)) => Ok(result.count),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_skill_type(from: u64, count: u64) -> Result<Vec<SkillType>> {
    let data = fetch(
        "list::skill_type",
        vec![
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetSkillType(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_skill_type(info: SkillType) -> Result<SkillType> {
    let data = fetch(
        "set::skill_type",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetSkillType(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_skill_type(info: SkillType) -> Result<SkillType> {
    let data = fetch(
        "insert::skill_type",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetSkillType(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn delete_skill_type(id: u64) -> Result<()> {
    let data = fetch(
        "delete::skill_type",
        vec![("id".to_string(), id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn filter_skill_type(filters: Vec<FilterPackage>) -> Result<Vec<SkillType>> {
    let data = fetch(
        "filter::skill_type",
        vec![("filters".to_string(), serde_json::to_string(&filters)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetSkillType(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn count_user_info() -> Result<u64> {
    let data = fetch("count::user_info", HashMap::new()).await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::Count(result)) => Ok(result.count),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_user_info(from: u64, count: u64) -> Result<Vec<UserInfo>> {
    let data = fetch(
        "list::user_info",
        vec![
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetUserInfo(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn filter_user_info(filters: Vec<FilterPackage>) -> Result<Vec<UserInfo>> {
    let data = fetch(
        "filter::user_info",
        vec![("filters".to_string(), serde_json::to_string(&filters)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetUserInfo(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_pokemon_info(uid: u64, from: u64, count: u64) -> Result<Vec<PokemonInfo>> {
    let data = fetch(
        "list::pokemon_info",
        vec![
            ("uid".to_string(), uid.to_string()),
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetPokemonInfo(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_pokemon_info(info: PokemonInfo) -> Result<PokemonInfo> {
    let data = fetch(
        "set::pokemon_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetPokemonInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_pokemon_info(info: PokemonInfo) -> Result<PokemonInfo> {
    let data = fetch(
        "insert::pokemon_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetPokemonInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn delete_pokemon_info(id: u64) -> Result<()> {
    let data = fetch(
        "delete::pokemon_info",
        vec![("id".to_string(), id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn list_item_info(uid: u64, from: u64, count: u64) -> Result<Vec<ItemInfo>> {
    let data = fetch(
        "list::item_info",
        vec![
            ("uid".to_string(), uid.to_string()),
            ("from".to_string(), from.to_string()),
            ("count".to_string(), count.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .map(|item| match item {
                RetStruct::GetItemInfo(value) => Ok(value),
                other => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_item_info(info: ItemInfo) -> Result<ItemInfo> {
    let data = fetch(
        "set::item_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetItemInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_item_info(info: ItemInfo) -> Result<ItemInfo> {
    let data = fetch(
        "insert::item_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetItemInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn delete_item_info(id: u64) -> Result<()> {
    let data = fetch(
        "delete::item_info",
        vec![("id".to_string(), id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn set_user_info(info: UserInfo) -> Result<UserInfo> {
    let data = fetch(
        "set::user_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetUserInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

pub async fn insert_user_info(info: UserInfo) -> Result<UserInfo> {
    let data = fetch(
        "insert::user_info",
        vec![("data".to_string(), serde_json::to_string(&info)?)]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data.and_then(|items| items.into_iter().next()) {
        Some(RetStruct::GetUserInfo(saved)) => Ok(saved),
        Some(other) => Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other)),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

// =============================================================================
// 地图-宠物关联管理
// =============================================================================

/// 将宠物类型添加到地图的野生宠物列表
pub async fn add_pokemon_to_map(map_id: u64, pokemon_type_id: u64) -> Result<()> {
    let data = fetch(
        "add_pokemon_to_map",
        vec![
            ("map_id".to_string(), map_id.to_string()),
            ("pokemon_type_id".to_string(), pokemon_type_id.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

/// 从地图的野生宠物列表中移除宠物类型
pub async fn remove_pokemon_from_map(map_id: u64, pokemon_type_id: u64) -> Result<()> {
    let data = fetch(
        "remove_pokemon_from_map",
        vec![
            ("map_id".to_string(), map_id.to_string()),
            ("pokemon_type_id".to_string(), pokemon_type_id.to_string()),
        ]
        .into_iter()
        .collect(),
    )
    .await?;

    match data.data {
        Some(_) => Ok(()),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

/// 获取地图的野生宠物列表
pub async fn get_wild_pokemons_for_map(map_id: u64) -> Result<Vec<WildPokemonInfo>> {
    let data = fetch(
        "get_wild_pokemons_for_map",
        vec![("map_id".to_string(), map_id.to_string())]
            .into_iter()
            .collect(),
    )
    .await?;

    match data.data {
        Some(items) => items
            .into_iter()
            .filter_map(|item| match item {
                RetStruct::WildPokemonInfo(tagged) => Some(Ok(tagged.info)),
                RetStruct::Count(_) => None,
                other => Some(Err(anyhow!("返回的数据包中包含了错误的信息 {:?}", other))),
            })
            .collect(),
        None => Err(anyhow!(data
            .reason
            .unwrap_or_else(|| "未知错误".to_string()))),
    }
}

// ==================== Boss API ====================

/// Boss API 响应结构
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct BossConfigResponse {
    success: bool,
    #[serde(default)]
    boss_config: Option<MapBossConfig>,
    #[serde(default)]
    message: Option<String>,
}

/// Boss 尝试刷新响应结构
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct BossSpawnResponse {
    success: bool,
    #[serde(default)]
    boss: Option<BossSpawnResult>,
    #[serde(default)]
    message: Option<String>,
}

/// Boss 刷新结果
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct BossSpawnResult {
    pokemon_type_id: u64,
    pokemon_name: String,
    level: u64,
    boss_multiplier: f32,
    #[serde(default)]
    pet: Option<BossPetInfo>,
}

/// Boss 宠物基本信息
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct BossPetInfo {
    id: u64,
    name: String,
    capture: u64,
}

/// 获取地图的 Boss 配置
pub async fn get_map_boss_config(map_id: u64) -> Result<MapBossConfig> {
    let endpoint = get_admin_endpoint()?;
    let client = reqwest::Client::new();

    let params = [("action", "get_config"), ("map_id", &map_id.to_string())];

    let url = format!("{}/boss.php", endpoint);

    let response = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .context("获取 Boss 配置请求失败")?;

    let text = response.text().await.context("读取响应失败")?;

    // 先尝试解析为 Boss API 响应
    if let Ok(resp) = serde_json::from_str::<BossConfigResponse>(&text) {
        if resp.success {
            return Ok(resp.boss_config.unwrap_or_default());
        }
        return Err(anyhow!(resp
            .message
            .unwrap_or_else(|| "获取 Boss 配置失败".to_string())));
    }

    // 兼容旧的 API 响应格式
    if let Ok(pkg) = serde_json::from_str::<RetPackage>(&text) {
        if pkg.success {
            return Ok(MapBossConfig::default());
        }
        return Err(anyhow!(pkg
            .reason
            .unwrap_or_else(|| "获取 Boss 配置失败".to_string())));
    }

    Err(anyhow!("无法解析响应: {}", text))
}

/// 尝试刷新 Boss
pub async fn try_spawn_map_boss(map_id: u64) -> Result<Option<MapBoss>> {
    let endpoint = get_admin_endpoint()?;
    let client = reqwest::Client::new();

    let params = [("action", "try_spawn"), ("map_id", &map_id.to_string())];

    let url = format!("{}/boss.php", endpoint);

    let response = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .context("Boss 刷新请求失败")?;

    let text = response.text().await.context("读取响应失败")?;

    // 尝试解析为 Boss 刷新响应
    if let Ok(resp) = serde_json::from_str::<BossSpawnResponse>(&text) {
        if resp.success {
            if let Some(boss_result) = resp.boss {
                return Ok(Some(MapBoss {
                    pokemon_type_id: boss_result.pokemon_type_id,
                    pokemon_name: boss_result.pokemon_name,
                    level: boss_result.level,
                    boss_multiplier: boss_result.boss_multiplier,
                    attributes: Default::default(),
                }));
            }
            return Ok(None);
        }
        return Err(anyhow!(resp
            .message
            .unwrap_or_else(|| "Boss 刷新失败".to_string())));
    }

    Err(anyhow!("无法解析响应: {}", text))
}
