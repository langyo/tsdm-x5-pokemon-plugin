//! 应用壳（tairitsu 版）：页面路由 + 各页面的持久状态。
//!
//! Dioxus 版由 Layout + CURRENT_PAGE 全局信号驱动；tairitsu 版把路由放进
//! AppState.route 信号，各页面自己用 tairitsu 原生事件绑定发起切换。

use tairitsu_hooks::{use_signal, ReactiveSignal};

use crate::state::ShopState;

/// 页面路由（对齐 Dioxus 版 Page 枚举；Adventure/Battle 等待移植）。
#[derive(Clone, Copy, PartialEq)]
pub enum Route {
    Welcome,
    Home,
    Shop,
    Inventory,
    PokemonCenter,
    MyPokemon,
    PokemonStorage,
    Adventure,
}

/// 首页话题项（新闻公告与论坛话题的统一显示格式）。
#[derive(Clone, Debug)]
pub enum TopicItem {
    News {
        title: String,
        url: String,
    },
    Topic {
        id: u64,
        title: String,
        is_pinned: bool,
    },
}

/// 在线玩家。
#[derive(Clone, Debug, Default)]
pub struct OnlinePlayer {
    pub uid: u64,
    pub username: String,
}

/// 侧栏状态（对齐 Dioxus 版 Sidebar 读的全局信号：USER_STATE /
/// SELECTED_POKEMON_INDEX + 组件内的徽章信号）。携带宠物列表不在这里：
/// `center.pokemons` 就是全应用共享的那份宠物列表（各页写操作后都会重拉它）。
#[derive(Clone)]
pub struct SidebarState {
    /// None = 尚未载入；Some(Err) = 失败。
    pub profile: ReactiveSignal<Option<Result<crate::state::UserProfile, String>>>,
    /// 背包各分类件数 `(type_id, count)`。
    pub bag_counts: ReactiveSignal<Option<Result<Vec<(u64, i64)>, String>>>,
    /// 迷你格里当前选中的宠物 id（非战斗时的高亮）。
    pub selected_id: ReactiveSignal<Option<u64>>,
    /// 悬浮中的迷你格下标（tooltip 显隐）。
    pub hover_slot: ReactiveSignal<Option<usize>>,
    /// 帖子徽章是否隐藏。None = 尚未查询；Some(Err) = 查询失败。
    pub badge_hidden: ReactiveSignal<Option<Result<bool, String>>>,
    /// 徽章请求进行中（按钮禁用，防重复提交）。
    pub badge_busy: ReactiveSignal<bool>,
}

impl SidebarState {
    pub fn new() -> Self {
        Self {
            profile: use_signal(|| None),
            bag_counts: use_signal(|| None),
            selected_id: use_signal(|| None),
            hover_slot: use_signal(|| None),
            badge_hidden: use_signal(|| None),
            badge_busy: use_signal(|| false),
        }
    }
}

/// 应用级持久状态（含各页面子状态）。
#[derive(Clone)]
pub struct AppState {
    pub component_id: std::cell::Cell<u32>,
    /// 当前页面。
    pub route: ReactiveSignal<Route>,
    /// 新玩家（is_new_player 或 total_pokemons==0）时先看欢迎页。
    pub needs_welcome: ReactiveSignal<bool>,
    /// 侧栏。
    pub sidebar: SidebarState,

    // ---- 欢迎页 ----
    pub welcome_initializing: ReactiveSignal<bool>,
    pub welcome_error: ReactiveSignal<Option<String>>,
    pub welcome_success: ReactiveSignal<bool>,

    // ---- 页脚“关于本项目”弹窗 ----
    pub show_about: ReactiveSignal<bool>,

    // ---- 应用级弹出菜单（技能卡 / 仓库格）与悬浮提示（装备 / 商店 / 战斗道具卡）----
    /// None = 没开着。旧版放在 `UI_STATE.popup`，由根部的 `PopupProvider` 渲染；
    /// 这里对应 `crate::popup::provider`。
    pub popup: ReactiveSignal<Option<crate::popup_types::OpenPopup>>,
    /// None = 没有卡片被悬浮。旧版每处提示各是卡片组件内的状态；这里收成一席，
    /// 由 `crate::hover_tip::provider` 渲染。
    pub hover_tip: ReactiveSignal<Option<crate::state::HoverTip>>,

    // ---- 首页 ----
    /// 话题+新闻合并列表。None = 加载中；Some(Err) = 失败。
    pub topics: ReactiveSignal<Option<Result<Vec<TopicItem>, String>>>,
    pub online: ReactiveSignal<Option<Result<(usize, Vec<OnlinePlayer>), String>>>,

    // ---- 商店页 ----
    pub shop: ShopState,

    // ---- 背包页 ----
    pub inv: crate::state::InventoryState,

    // ---- 宠物中心（治疗）页 ----
    pub center: crate::state::CenterState,

    // ---- 我的宝可梦页 ----
    pub my_pm: crate::state::MyPokemonState,

    // ---- 仓库页 ----
    pub storage: crate::state::StorageState,

    // ---- 冒险/战斗页 ----
    pub battle: crate::state::BattleState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            route: use_signal(|| Route::Welcome),
            needs_welcome: use_signal(|| true),
            sidebar: SidebarState::new(),
            welcome_initializing: use_signal(|| false),
            welcome_error: use_signal(|| None),
            welcome_success: use_signal(|| false),
            show_about: use_signal(|| false),
            popup: use_signal(|| None),
            hover_tip: use_signal(|| None),
            topics: use_signal(|| None),
            online: use_signal(|| None),
            shop: ShopState::new(),
            inv: crate::state::InventoryState::new(),
            center: crate::state::CenterState::new(),
            my_pm: crate::state::MyPokemonState::new(),
            storage: crate::state::StorageState::new(),
            battle: crate::state::BattleState::new(),
        }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}
