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
        matches!(self, AdminRoute::GlobalConfig)
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

/// 应用级状态。
#[derive(Clone)]
pub struct AppState {
    pub component_id: std::cell::Cell<u32>,
    pub route: ReactiveSignal<AdminRoute>,
    pub config: GlobalConfigState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            route: use_signal(|| AdminRoute::GlobalConfig),
            config: GlobalConfigState::new(),
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}
