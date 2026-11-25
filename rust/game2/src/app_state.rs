//! 应用壳（tairitsu 版）：页面路由 + 各页面的持久状态。
//!
//! Dioxus 版由 Layout + CURRENT_PAGE 全局信号驱动；tairitsu 版
//! 把路由放进 AppState.route 信号，重挂载时按路由渲染对应页面。
//! 页面切换走委托（data-pm-nav），request_remount 整树重建。

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

/// 应用级持久状态（含各页面子状态）。
#[derive(Clone)]
pub struct AppState {
    pub component_id: std::cell::Cell<u32>,
    /// 当前页面。
    pub route: ReactiveSignal<Route>,
    /// 新玩家（is_new_player 或 total_pokemons==0）时先看欢迎页。
    pub needs_welcome: ReactiveSignal<bool>,
    /// 管理员（决定是否显示「后台管理」入口）。
    pub is_admin: ReactiveSignal<bool>,

    // ---- 欢迎页 ----
    pub welcome_initializing: ReactiveSignal<bool>,
    pub welcome_error: ReactiveSignal<Option<String>>,
    pub welcome_success: ReactiveSignal<bool>,

    // ---- 页脚“关于本项目”弹窗 ----
    pub show_about: ReactiveSignal<bool>,

    // ---- 帖子徽章（首页右侧控制区）----
    /// None = 尚未查询；Some(Err) = 查询失败。
    pub badge_hidden: ReactiveSignal<Option<Result<bool, String>>>,
    /// 正在请求（按钮禁用，防重复提交）。
    pub badge_busy: ReactiveSignal<bool>,

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
            is_admin: use_signal(|| false),
            welcome_initializing: use_signal(|| false),
            welcome_error: use_signal(|| None),
            welcome_success: use_signal(|| false),
            show_about: use_signal(|| false),
            badge_hidden: use_signal(|| None),
            badge_busy: use_signal(|| false),
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
