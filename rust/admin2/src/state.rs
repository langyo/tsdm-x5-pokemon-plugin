//! 后台持久状态。

use tairitsu_hooks::{use_signal, ReactiveSignal};

/// 后台路由（对齐 Dioxus 版 AdminRoute；未移植页面在导航中占位）。
#[derive(Clone, Copy, PartialEq)]
pub enum AdminRoute {
    GlobalConfig,
    PokemonData,
    ItemData,
    MapData,
    UserData,
    EvolutionData,
    SkillType,
    EffectData,
}

impl AdminRoute {
    pub fn key(self) -> &'static str {
        match self {
            AdminRoute::GlobalConfig => "global-config",
            AdminRoute::PokemonData => "pokemon-data",
            AdminRoute::ItemData => "item-data",
            AdminRoute::MapData => "map-data",
            AdminRoute::UserData => "user-data",
            AdminRoute::EvolutionData => "evolution-data",
            AdminRoute::SkillType => "skill-type",
            AdminRoute::EffectData => "effect-data",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            AdminRoute::GlobalConfig => "全局配置",
            AdminRoute::PokemonData => "宝可梦数据",
            AdminRoute::ItemData => "物品数据",
            AdminRoute::MapData => "地图数据",
            AdminRoute::UserData => "用户数据",
            AdminRoute::EvolutionData => "进化链数据",
            AdminRoute::SkillType => "技能数据",
            AdminRoute::EffectData => "效果数据",
        }
    }

    /// 该路由是否已移植到 tairitsu。
    pub fn ported(self) -> bool {
        matches!(
            self,
            AdminRoute::GlobalConfig
                | AdminRoute::PokemonData
                | AdminRoute::ItemData
                | AdminRoute::MapData
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ToastKind {
    Success,
    Error,
}

/// 全局配置页状态：JSON 文本直接编辑（切片 1 形态；表单化随后续切片）。
#[derive(Clone)]
pub struct GlobalConfigState {
    pub component_id: std::cell::Cell<u32>,
    /// None = 加载中；Some(Err) = 加载失败。
    pub loaded: ReactiveSignal<Option<Result<(), String>>>,
    /// 服务端返回的原文（美化后）。
    pub saved_json: ReactiveSignal<String>,
    /// 编辑框当前内容。
    pub draft: ReactiveSignal<String>,
    pub saving: ReactiveSignal<bool>,
    pub toast_visible: ReactiveSignal<bool>,
    pub toast_message: ReactiveSignal<String>,
    pub toast_kind: ReactiveSignal<ToastKind>,
}

impl GlobalConfigState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            loaded: use_signal(|| None),
            saved_json: use_signal(String::new),
            draft: use_signal(String::new),
            saving: use_signal(|| false),
            toast_visible: use_signal(|| false),
            toast_message: use_signal(String::new),
            toast_kind: use_signal(|| ToastKind::Success),
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.draft.get() != self.saved_json.get()
    }
}

impl std::fmt::Debug for GlobalConfigState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlobalConfigState").finish()
    }
}

pub fn show_toast_silent(state: &GlobalConfigState, kind: ToastKind, message: String) {
    state.toast_kind.inner().set(kind);
    state.toast_message.inner().set(message);
    state.toast_visible.inner().set(true);
}

/// 通用数据列表页状态（切片 2 定型、切片 3 起复用）。
///
/// 与 Dioxus 版 ListPageState 对应：rows 保留原始 JSON（显示字段渲染时
/// 取，缺字段回退），装载守卫是 busy + loaded 组合（防重挂活锁）。
#[derive(Clone)]
pub struct ListState {
    pub component_id: std::cell::Cell<u32>,
    /// None = 首次加载中；Some(Err) = 加载失败。
    pub loaded: ReactiveSignal<Option<Result<(), String>>>,
    pub rows: ReactiveSignal<Vec<serde_json::Value>>,
    pub total: ReactiveSignal<u64>,
    pub loaded_count: ReactiveSignal<u64>,
    pub busy: ReactiveSignal<bool>,
    /// 筛选中（filter 结果不分页，隐藏加载更多）。
    pub filtered: ReactiveSignal<bool>,
    pub err: ReactiveSignal<Option<String>>,
    // 筛选输入：ID（equal）+ 名称（contains）+ 两个实体特定筛选。
    pub f_id: ReactiveSignal<String>,
    pub f_name: ReactiveSignal<String>,
    pub f_a: ReactiveSignal<String>,
    pub f_b: ReactiveSignal<String>,
}

impl ListState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            loaded: use_signal(|| None),
            rows: use_signal(Vec::new),
            total: use_signal(|| 0u64),
            loaded_count: use_signal(|| 0u64),
            busy: use_signal(|| false),
            filtered: use_signal(|| false),
            err: use_signal(|| None),
            f_id: use_signal(String::new),
            f_name: use_signal(String::new),
            f_a: use_signal(String::new),
            f_b: use_signal(String::new),
        }
    }
}

impl std::fmt::Debug for ListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListState").finish()
    }
}

/// 实体列表配置：动作名 + 实体特定筛选（f_a/f_b）的服务端 tag 与
/// operator + 输入框占位文案。切片 3 起每个数据页一份。
#[derive(Clone, Copy)]
pub struct EntitySpec {
    /// 如 "pokemon_type"（拼进 list::/count::/filter::）。
    pub entity: &'static str,
    pub title: &'static str,
    /// f_a 筛选；None 则该输入框不渲染。
    pub a: Option<FilterSpec>,
    pub b: Option<FilterSpec>,
}

#[derive(Clone, Copy)]
pub struct FilterSpec {
    /// 服务端 filter 的 tag（如「宠物类型」「类型」）。
    pub tag: &'static str,
    pub operator: &'static str,
    pub placeholder: &'static str,
}

/// 宝可梦数据（切片 2 实例化）。
pub const POKEMON_SPEC: EntitySpec = EntitySpec {
    entity: "pokemon_type",
    title: "宠物数据",
    a: Some(FilterSpec {
        tag: "宠物类型",
        operator: "contains",
        placeholder: "属性（如火）",
    }),
    b: None,
};

/// 物品数据（切片 3 实例化）。类型筛选对数字列（1药品/2球/3素材/4强化/5装备）。
pub const ITEM_SPEC: EntitySpec = EntitySpec {
    entity: "item_type",
    title: "物品数据",
    a: Some(FilterSpec {
        tag: "类型",
        operator: "contains",
        placeholder: "类型（1药品/2球/3素材/4强化/5装备）",
    }),
    b: Some(FilterSpec {
        tag: "价格",
        operator: "greater_or_equal",
        placeholder: "价格 ≥",
    }),
};

/// 地图数据（切片 4 实例化）。地形筛选传英文 snake_case 名
/// （服务端 translate_map_full_name_to_alpha 转单字符）；等级筛选走数字列。
pub const MAP_SPEC: EntitySpec = EntitySpec {
    entity: "map_info",
    title: "地图数据",
    a: Some(FilterSpec {
        tag: "地形类型",
        operator: "equal",
        placeholder: "地形（如 plain/grass/sea/cave）",
    }),
    b: Some(FilterSpec {
        tag: "野怪最低等级",
        operator: "greater_or_equal",
        placeholder: "最低等级 ≥",
    }),
};

/// 应用级状态。
#[derive(Clone)]
pub struct AppState {
    pub component_id: std::cell::Cell<u32>,
    pub route: ReactiveSignal<AdminRoute>,
    pub config: GlobalConfigState,
    pub pokemon: ListState,
    pub item: ListState,
    pub map: ListState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            route: use_signal(|| AdminRoute::GlobalConfig),
            config: GlobalConfigState::new(),
            pokemon: ListState::new(),
            item: ListState::new(),
            map: ListState::new(),
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}
