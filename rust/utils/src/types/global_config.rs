use crate::types::api_config::NewsAnnouncement;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum BoolishValue {
    Bool(bool),
    Int(i64),
    UInt(u64),
    String(String),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum U64ishValue {
    UInt(u64),
    Int(i64),
    String(String),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringishValue {
    String(String),
    Int(i64),
    UInt(u64),
    Bool(bool),
}

fn deserialize_boolish<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match BoolishValue::deserialize(deserializer)? {
        BoolishValue::Bool(value) => Ok(value),
        BoolishValue::Int(value) => Ok(value != 0),
        BoolishValue::UInt(value) => Ok(value != 0),
        BoolishValue::String(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            match normalized.as_str() {
                "1" | "true" | "yes" | "on" => Ok(true),
                "0" | "false" | "no" | "off" | "" => Ok(false),
                _ => Err(D::Error::custom(format!("无法将值 {value:?} 解析为布尔值"))),
            }
        }
    }
}

fn deserialize_u64ish<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    match U64ishValue::deserialize(deserializer)? {
        U64ishValue::UInt(value) => Ok(value),
        U64ishValue::Int(value) => value
            .try_into()
            .map_err(|_| D::Error::custom(format!("无法将负数 {value} 解析为 u64"))),
        U64ishValue::String(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                Ok(0)
            } else {
                trimmed.parse::<u64>().map_err(|err| {
                    D::Error::custom(format!("无法将值 {value:?} 解析为 u64: {err}"))
                })
            }
        }
    }
}

fn deserialize_stringish<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match StringishValue::deserialize(deserializer)? {
        StringishValue::String(value) => Ok(value),
        StringishValue::Int(value) => Ok(value.to_string()),
        StringishValue::UInt(value) => Ok(value.to_string()),
        StringishValue::Bool(value) => Ok(value.to_string()),
    }
}

/// 反序列化 news_announcements - 支持数组或 JSON 字符串
fn deserialize_news_announcements<'de, D>(
    deserializer: D,
) -> Result<Vec<NewsAnnouncement>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde_json::Value;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NewsAnnouncementsValue {
        Array(Vec<NewsAnnouncement>),
        String(String),
    }

    match NewsAnnouncementsValue::deserialize(deserializer)? {
        NewsAnnouncementsValue::Array(arr) => Ok(arr),
        NewsAnnouncementsValue::String(s) => {
            // 尝试解析 JSON 字符串
            if let Ok(Value::Array(arr)) = serde_json::from_str::<Value>(&s) {
                let result: Vec<NewsAnnouncement> = arr
                    .into_iter()
                    .filter_map(|v| serde_json::from_value::<NewsAnnouncement>(v).ok())
                    .collect();
                Ok(result)
            } else {
                // 解析失败，返回空数组
                Ok(Vec::new())
            }
        }
    }
}

/// 序列化 news_announcements - 直接序列化为数组
fn serialize_news_announcements<S>(
    value: &Vec<NewsAnnouncement>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.serialize(serializer)
}

fn default_news_announcements() -> Vec<NewsAnnouncement> {
    Vec::new()
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct GlobalConfigType {
    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub is_open: bool,
    #[serde(default, deserialize_with = "deserialize_stringish")]
    pub version: String,
    #[serde(default, deserialize_with = "deserialize_stringish")]
    pub ann_title: String,
    #[serde(default, deserialize_with = "deserialize_stringish")]
    pub ann_url: String,
    #[serde(
        default = "default_news_announcements",
        deserialize_with = "deserialize_news_announcements",
        serialize_with = "serialize_news_announcements"
    )]
    pub news_announcements: Vec<NewsAnnouncement>,

    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub intimacy_increase_per_hour: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub intimacy_increase_per_earn_xp: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub intimacy_increase_multiple: u64,

    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub revive_time: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub medical_price: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub posts_count_for_wakeup: u64,

    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub is_enable_catch: bool,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub catch_price_in_catch_area: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub pve_catch_xp_multiple: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub pve_catch_level: u64,

    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub is_enable_earn_xp_on_pve: bool,
    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub drop_money_on_pve: bool,
    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub drop_money_config_by_global: bool,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub drop_money_percent_min_on_pve: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub drop_money_percent_max_on_pve: u64,

    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub is_enable_egg: bool,
    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub is_enable_buy_egg: bool,
    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub is_enable_buy_pokemon: bool,
    #[serde(default, deserialize_with = "deserialize_boolish")]
    pub is_enable_multiple_eggs: bool,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub egg_price: u64,
    #[serde(default, deserialize_with = "deserialize_u64ish")]
    pub hatch_egg_intimacy: u64,
}

impl GlobalConfigType {
    const _TYPE: &'static str = "global_config";
}

impl Default for GlobalConfigType {
    fn default() -> Self {
        Self {
            is_open: true,
            version: "Unknown".into(),
            ann_title: "Test".into(),
            ann_url: "https://www.tsdm39.com".into(),
            news_announcements: Vec::new(),

            intimacy_increase_per_hour: 1,
            intimacy_increase_per_earn_xp: 1,
            intimacy_increase_multiple: 1,

            revive_time: 10,
            medical_price: 10,
            posts_count_for_wakeup: 10,

            is_enable_catch: true,
            catch_price_in_catch_area: 10,
            pve_catch_xp_multiple: 1,
            pve_catch_level: 1,

            is_enable_earn_xp_on_pve: true,
            drop_money_on_pve: true,
            drop_money_config_by_global: false,
            drop_money_percent_min_on_pve: 0,
            drop_money_percent_max_on_pve: 0,

            is_enable_egg: true,
            is_enable_buy_egg: true,
            is_enable_buy_pokemon: true,
            is_enable_multiple_eggs: true,
            egg_price: 10,
            hatch_egg_intimacy: 10,
        }
    }
}
