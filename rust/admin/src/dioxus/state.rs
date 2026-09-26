use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::dioxus::prelude::*;

use _utils::types::{
    evolution_info::EvolutionInfo, global_config::GlobalConfigType, item_type::ItemType,
    map_info::MapInfo, pokemon_type::PokemonType, skill_type::SkillType, user_info::UserInfo,
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdminRoute {
    GlobalConfig,
    SqlConsole,
    PokemonData,
    ItemData,
    MapData,
    UserData,
    EvolutionData,
    SkillType,
}

impl Default for AdminRoute {
    fn default() -> Self {
        Self::GlobalConfig
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdminNoticeLevel {
    Info,
    Success,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdminNotice {
    pub level: AdminNoticeLevel,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdminToast {
    pub id: u64,
    pub level: AdminNoticeLevel,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalConfigState {
    pub current: GlobalConfigType,
    pub saved: GlobalConfigType,
    pub initialized: bool,
    pub loading: bool,
}

impl Default for GlobalConfigState {
    fn default() -> Self {
        let config = GlobalConfigType::default();
        Self {
            current: config.clone(),
            saved: config,
            initialized: false,
            loading: false,
        }
    }
}

impl GlobalConfigState {
    pub fn is_dirty(&self) -> bool {
        self.current != self.saved
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SqlHistoryEntry {
    pub executed_at: DateTime<Utc>,
    pub sql: String,
    pub result: Result<String, String>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SqlConsoleState {
    pub console: String,
    pub history: Vec<SqlHistoryEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberOperation {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogicalOperation {
    Equal,
    NotEqual,
    Contains,
}

/// 过滤条件类型 - 直接序列化为字符串供后端使用
/// 后端期望 operator 是字符串，如 "equal", "not_equal", "contains", "greater", 等
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilterConditionType {
    Id,
    Text(LogicalOperation),
    Number(NumberOperation),
    Boolean,
}

impl FilterConditionType {
    /// 获取后端期望的 operator 字符串值
    pub fn as_str(&self) -> &'static str {
        match self {
            FilterConditionType::Id => "equal",
            FilterConditionType::Text(LogicalOperation::Equal) => "equal",
            FilterConditionType::Text(LogicalOperation::NotEqual) => "not_equal",
            FilterConditionType::Text(LogicalOperation::Contains) => "contains",
            FilterConditionType::Number(NumberOperation::Equal) => "equal",
            FilterConditionType::Number(NumberOperation::NotEqual) => "not_equal",
            FilterConditionType::Number(NumberOperation::Greater) => "greater",
            FilterConditionType::Number(NumberOperation::GreaterOrEqual) => "greater_or_equal",
            FilterConditionType::Number(NumberOperation::Less) => "less",
            FilterConditionType::Number(NumberOperation::LessOrEqual) => "less_or_equal",
            FilterConditionType::Boolean => "equal",
        }
    }
}

// 自定义序列化为字符串
impl Serialize for FilterConditionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

// 自定义反序列化从字符串
impl<'de> Deserialize<'de> for FilterConditionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};
        use std::fmt;

        struct FilterConditionTypeVisitor;

        impl<'de> Visitor<'de> for FilterConditionTypeVisitor {
            type Value = FilterConditionType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing filter operation")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match value {
                    "equal" => Ok(FilterConditionType::Text(LogicalOperation::Equal)),
                    "not_equal" => Ok(FilterConditionType::Text(LogicalOperation::NotEqual)),
                    "contains" => Ok(FilterConditionType::Text(LogicalOperation::Contains)),
                    "greater" => Ok(FilterConditionType::Number(NumberOperation::Greater)),
                    "greater_or_equal" => {
                        Ok(FilterConditionType::Number(NumberOperation::GreaterOrEqual))
                    }
                    "less" => Ok(FilterConditionType::Number(NumberOperation::Less)),
                    "less_or_equal" => {
                        Ok(FilterConditionType::Number(NumberOperation::LessOrEqual))
                    }
                    _ => Err(Error::unknown_variant(
                        value,
                        &[
                            "equal",
                            "not_equal",
                            "contains",
                            "greater",
                            "greater_or_equal",
                            "less",
                            "less_or_equal",
                        ],
                    )),
                }
            }
        }

        deserializer.deserialize_str(FilterConditionTypeVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FilterPackage {
    pub tag: String,
    pub operator: FilterConditionType,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterFieldState {
    pub tag: String,
    pub operator: FilterConditionType,
    pub value: String,
    pub enum_options: Vec<(String, String)>, // (database_value, display_label)
    pub enabled: bool,                       // 是否启用此过滤条件
}

impl FilterFieldState {
    pub fn to_package(&self) -> Option<FilterPackage> {
        // 只有启用且值不为空时才生成过滤包
        if !self.enabled || self.value.trim().is_empty() {
            return None;
        }
        // 对于有枚举选项的字段（无论使用 Enum 还是 Text 类型），value 存储的是显示标签，需要找到对应的数据库值
        let db_value = if !self.enum_options.is_empty() {
            self.enum_options
                .iter()
                .find(|(_, label)| label == &self.value)
                .map(|(db_val, _)| db_val.clone())
                .unwrap_or_else(|| self.value.clone())
        } else {
            self.value.clone()
        };
        Some(FilterPackage {
            tag: self.tag.clone(),
            operator: self.operator,
            value: db_value,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListPageState<T> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub loaded_count: u64,
    pub initialized: bool,
    pub loading: bool,
    pub is_filtered: bool,
    pub filters: Vec<FilterFieldState>,
}

impl<T> ListPageState<T> {
    pub fn with_filters(filters: Vec<FilterFieldState>) -> Self {
        Self {
            items: Vec::new(),
            total_count: 0,
            loaded_count: 0,
            initialized: false,
            loading: false,
            is_filtered: false,
            filters,
        }
    }

    pub fn has_active_filters(&self) -> bool {
        self.filters
            .iter()
            .any(|filter| !filter.value.trim().is_empty())
    }
}

fn item_filters() -> Vec<FilterFieldState> {
    vec![
        FilterFieldState {
            tag: "ID".to_string(),
            operator: FilterConditionType::Id,
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "名称".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "类型".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "价格".to_string(),
            operator: FilterConditionType::Number(NumberOperation::GreaterOrEqual),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "是否出售".to_string(),
            operator: FilterConditionType::Boolean,
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "描述".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
    ]
}

fn map_filters() -> Vec<FilterFieldState> {
    // 地形类型：后端使用 snake_case，通过 translate_map_full_name_to_alpha 转换为数据库单字符值
    let area_type_options: Vec<(String, String)> = vec![
        ("plain".to_string(), "平原".to_string()),
        ("grass".to_string(), "草丛".to_string()),
        ("water".to_string(), "水池".to_string()),
        ("sea".to_string(), "海洋".to_string()),
        ("sea_bottom".to_string(), "海底".to_string()),
        ("mountain".to_string(), "山谷".to_string()),
        ("cave".to_string(), "山洞".to_string()),
        ("sand".to_string(), "沙漠".to_string()),
        ("factory".to_string(), "工厂".to_string()),
        ("base".to_string(), "基地".to_string()),
        ("town".to_string(), "市镇".to_string()),
        ("gym".to_string(), "道馆".to_string()),
        ("sky".to_string(), "天空".to_string()),
        ("deep_sea".to_string(), "深海".to_string()),
        ("lava".to_string(), "熔岩".to_string()),
    ];

    vec![
        FilterFieldState {
            tag: "ID".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "名称".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "地形类型".to_string(),
            // 使用 Text 类型而不是 Enum，确保与后端兼容
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: area_type_options,
            enabled: false,
        },
        FilterFieldState {
            tag: "野怪最低等级".to_string(),
            operator: FilterConditionType::Number(NumberOperation::GreaterOrEqual),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "野怪最高等级".to_string(),
            operator: FilterConditionType::Number(NumberOperation::GreaterOrEqual),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
    ]
}

fn pokemon_filters() -> Vec<FilterFieldState> {
    // 宠物类型：后端数据库存储中文名称，使用 LIKE '%$value%' 查询
    let kind_options: Vec<(String, String)> = vec![
        ("普通".to_string(), "普通".to_string()),
        ("火".to_string(), "火".to_string()),
        ("水".to_string(), "水".to_string()),
        ("草".to_string(), "草".to_string()),
        ("电".to_string(), "电".to_string()),
        ("冰".to_string(), "冰".to_string()),
        ("格斗".to_string(), "格斗".to_string()),
        ("毒".to_string(), "毒".to_string()),
        ("地面".to_string(), "地面".to_string()),
        ("飞行".to_string(), "飞行".to_string()),
        ("超能".to_string(), "超能".to_string()),
        ("虫".to_string(), "虫".to_string()),
        ("岩石".to_string(), "岩石".to_string()),
        ("幽灵".to_string(), "幽灵".to_string()),
        ("龙".to_string(), "龙".to_string()),
        ("恶".to_string(), "恶".to_string()),
        ("钢".to_string(), "钢".to_string()),
        ("妖精".to_string(), "妖精".to_string()),
    ];

    vec![
        FilterFieldState {
            tag: "ID".to_string(),
            operator: FilterConditionType::Id,
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "名称".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "宠物类型".to_string(),
            // 使用 Text 类型而不是 Enum
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: kind_options,
            enabled: false,
        },
    ]
}

fn evolution_filters() -> Vec<FilterFieldState> {
    // 注意：后端不支持按"规则"字段筛选，已移除该选项
    vec![
        FilterFieldState {
            tag: "ID".to_string(),
            operator: FilterConditionType::Id,
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "进化来源".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "进化目标".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
    ]
}

fn skill_filters() -> Vec<FilterFieldState> {
    // 注意：后端不支持按"效果"字段筛选，已移除该选项
    vec![
        FilterFieldState {
            tag: "ID".to_string(),
            operator: FilterConditionType::Id,
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "名称".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "可用此的种族".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
    ]
}

fn user_filters() -> Vec<FilterFieldState> {
    vec![
        FilterFieldState {
            tag: "UID".to_string(),
            operator: FilterConditionType::Id,
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "昵称".to_string(),
            operator: FilterConditionType::Text(LogicalOperation::Equal),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "胜场".to_string(),
            operator: FilterConditionType::Number(NumberOperation::GreaterOrEqual),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "输场".to_string(),
            operator: FilterConditionType::Number(NumberOperation::GreaterOrEqual),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "金钱".to_string(),
            operator: FilterConditionType::Number(NumberOperation::GreaterOrEqual),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
        FilterFieldState {
            tag: "经验值".to_string(),
            operator: FilterConditionType::Number(NumberOperation::GreaterOrEqual),
            value: String::new(),
            enum_options: Vec::new(),
            enabled: false,
        },
    ]
}

pub static ADMIN_BUSY: GlobalSignal<bool> = Signal::global(|| false);
pub static ADMIN_NOTICE: GlobalSignal<Option<AdminNotice>> = Signal::global(|| None);
pub static ADMIN_TOASTS: GlobalSignal<Vec<AdminToast>> = Signal::global(Vec::new);
pub static ADMIN_TOAST_COUNTER: GlobalSignal<u64> = Signal::global(|| 0);
pub static ADMIN_GLOBAL_CONFIG: GlobalSignal<GlobalConfigState> =
    Signal::global(GlobalConfigState::default);
pub static ADMIN_SQL_CONSOLE: GlobalSignal<SqlConsoleState> =
    Signal::global(SqlConsoleState::default);
pub static ADMIN_ITEM_DATA: GlobalSignal<ListPageState<ItemType>> =
    Signal::global(|| ListPageState::with_filters(item_filters()));
pub static ADMIN_MAP_DATA: GlobalSignal<ListPageState<MapInfo>> =
    Signal::global(|| ListPageState::with_filters(map_filters()));
pub static ADMIN_POKEMON_DATA: GlobalSignal<ListPageState<PokemonType>> =
    Signal::global(|| ListPageState::with_filters(pokemon_filters()));
pub static ADMIN_EVOLUTION_DATA: GlobalSignal<ListPageState<EvolutionInfo>> =
    Signal::global(|| ListPageState::with_filters(evolution_filters()));
pub static ADMIN_SKILL_DATA: GlobalSignal<ListPageState<SkillType>> =
    Signal::global(|| ListPageState::with_filters(skill_filters()));
pub static ADMIN_USER_DATA: GlobalSignal<ListPageState<UserInfo>> =
    Signal::global(|| ListPageState::with_filters(user_filters()));

// 全局Tooltip状态
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TooltipState {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub visible: bool,
}

pub static ADMIN_TOOLTIP: GlobalSignal<TooltipState> = Signal::global(TooltipState::default);

pub fn show_tooltip(text: String, x: f64, y: f64) {
    let mut state = ADMIN_TOOLTIP.write();
    state.text = text;
    state.x = x;
    state.y = y;
    state.visible = true;
}

pub fn hide_tooltip() {
    ADMIN_TOOLTIP.write().visible = false;
}

pub fn set_busy(is_busy: bool) {
    *ADMIN_BUSY.write() = is_busy;
}

pub fn clear_notice() {
    *ADMIN_NOTICE.write() = None;
}

pub fn set_notice(level: AdminNoticeLevel, message: impl Into<String>) {
    let message = message.into();
    *ADMIN_NOTICE.write() = Some(AdminNotice {
        level,
        message: message.clone(),
    });
    push_toast(level, message);
}

pub fn push_toast(level: AdminNoticeLevel, message: impl Into<String>) {
    let mut toasts = ADMIN_TOASTS.write();
    if matches!(level, AdminNoticeLevel::Error) {
        toasts.retain(|toast| toast.level != AdminNoticeLevel::Error);
    }

    // 如果 toast 数量达到 3 个，移除最旧的
    if toasts.len() >= 3 {
        toasts.remove(0);
    }

    let mut counter = ADMIN_TOAST_COUNTER.write();
    let id = *counter;
    *counter += 1;
    toasts.push(AdminToast {
        id,
        level,
        message: message.into(),
    });
}

pub fn hide_toast(id: u64) {
    ADMIN_TOASTS.write().retain(|toast| toast.id != id);
}

pub fn update_global_config(update: impl FnOnce(&mut GlobalConfigType)) {
    let mut state = ADMIN_GLOBAL_CONFIG.write();
    update(&mut state.current);
}

pub fn begin_global_config_request() {
    let mut state = ADMIN_GLOBAL_CONFIG.write();
    state.loading = true;
}

pub fn finish_global_config_load(config: GlobalConfigType) {
    let mut state = ADMIN_GLOBAL_CONFIG.write();
    state.current = config.clone();
    state.saved = config;
    state.initialized = true;
    state.loading = false;
}

pub fn finish_global_config_attempt() {
    let mut state = ADMIN_GLOBAL_CONFIG.write();
    state.initialized = true;
    state.loading = false;
}

pub fn reset_global_config_to_saved() {
    let saved = ADMIN_GLOBAL_CONFIG.read().saved.clone();
    let mut state = ADMIN_GLOBAL_CONFIG.write();
    state.current = saved;
}

pub fn set_sql_console_input(value: String) {
    ADMIN_SQL_CONSOLE.write().console = value;
}

pub fn clear_sql_console_input() {
    ADMIN_SQL_CONSOLE.write().console.clear();
}

pub fn push_sql_history(entry: SqlHistoryEntry) {
    ADMIN_SQL_CONSOLE.write().history.push(entry);
}
