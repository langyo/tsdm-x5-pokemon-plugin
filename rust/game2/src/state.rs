//! 页面共享状态（tairitsu 版）。
//!
//! tairitsu 的状态模式：`use_signal` 每次渲染创建新信号（受控组件设计），
//! 持久状态必须挂在渲染闭包外——由 `run_app` 在 `with_component` 上下文里
//! 构造一次，经 props 传入组件；事件回调捕获克隆共享同一组信号。

use tairitsu_hooks::{use_signal, ReactiveSignal};

/// 商店商品（与 rust/utils/src/types/api_shop.rs 字段一致）。
#[derive(Debug, Clone, Default)]
pub struct ShopItem {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub type_name: String,
    /// 物品图标文件名（tpname，不带扩展名）。
    pub image: String,
    pub price: i64,
    pub effect: ItemEffect,
    pub can_buy: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ItemEffect {
    pub addhp: i64,
    pub addexp: i64,
    pub addlv: i64,
}

/// 商店宠物（六维种族值）。
#[derive(Debug, Clone, Default)]
pub struct ShopPet {
    pub id: u64,
    pub name: String,
    pub type_1: String,
    pub type_2: Option<String>,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub spatk: u32,
    pub spdef: u32,
    pub speed: u32,
    pub price: i64,
    pub can_buy: bool,
}

/// 反馈提示（成功/失败共用一条 UI，颜色随 variant）。
#[derive(Clone, Copy, PartialEq)]
pub enum ToastKind {
    Success,
    Error,
}

/// 背包物品（与 rust/utils/src/types/api_user.rs InventoryItem 字段一致）。
#[derive(Debug, Clone, Default)]
pub struct InventoryItem {
    /// 背包行 ID（pm_myitem.id，仅标识用）。
    pub id: u64,
    /// 物品类型 ID（pm_itemdata.id）——使用/查询可用目标都传它，
    /// Dioxus 版同样以 type_id 作为使用键。
    pub type_id: u64,
    /// 物品大类（1=回复药 … 5=装备道具）。
    pub item_type: u64,
    /// 服务端给的使用目标：pokemon / battle / global。
    pub use_target: Option<String>,
    pub name: String,
    pub description: String,
    pub image: String,
    pub quantity: i64,
    pub type_name: String,
}

/// 弹窗里可选的目标宠物（get_usable_pokemon）。
#[derive(Debug, Clone, Default)]
pub struct UsablePokemon {
    pub id: u64,
    pub nickname: String,
    pub level: u64,
    pub hp: i64,
    pub max_hp: i64,
}

/// 背包页持久状态。
#[derive(Clone)]
pub struct InventoryState {
    pub component_id: std::cell::Cell<u32>,
    pub category: ReactiveSignal<Option<u32>>,
    pub page: ReactiveSignal<u32>,
    pub total_pages: ReactiveSignal<usize>,
    pub total: ReactiveSignal<usize>,
    pub search_input: ReactiveSignal<String>,
    pub search_query: ReactiveSignal<String>,

    pub items: ReactiveSignal<Option<Result<Vec<InventoryItem>, String>>>,
    /// 正在使用中的物品类型 ID（互斥守卫）。
    pub using: ReactiveSignal<Option<u64>>,
    /// 选中的物品（弹窗打开时的使用目标判定）。
    pub selected_item: ReactiveSignal<Option<u64>>,
    pub show_target_modal: ReactiveSignal<bool>,
    pub usable_pokemon: ReactiveSignal<Option<Result<Vec<UsablePokemon>, String>>>,

    pub toast_visible: ReactiveSignal<bool>,
    pub toast_message: ReactiveSignal<String>,
    pub toast_kind: ReactiveSignal<ToastKind>,
}

impl InventoryState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            category: use_signal(|| None),
            page: use_signal(|| 1u32),
            total_pages: use_signal(|| 1usize),
            total: use_signal(|| 0usize),
            search_input: use_signal(String::new),
            search_query: use_signal(String::new),
            items: use_signal(|| None),
            using: use_signal(|| None),
            selected_item: use_signal(|| None),
            show_target_modal: use_signal(|| false),
            usable_pokemon: use_signal(|| None),
            toast_visible: use_signal(|| false),
            toast_message: use_signal(String::new),
            toast_kind: use_signal(|| ToastKind::Success),
        }
    }
}

impl std::fmt::Debug for InventoryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InventoryState").finish()
    }
}

/// 背包页提示（与商店的 show_toast* 分开命名，避免重载——Rust 无函数重载）。
pub fn inv_show_toast(state: &InventoryState, kind: ToastKind, message: String) {
    state.toast_kind.set(kind);
    state.toast_message.set(message);
    state.toast_visible.set(true);
}

/// promise 线程版（与 ShopState 的同名函数同约定）。
pub fn inv_show_toast_silent(state: &InventoryState, kind: ToastKind, message: String) {
    state.toast_kind.inner().set(kind);
    state.toast_message.inner().set(message);
    state.toast_visible.inner().set(true);
}

/// 商店页持久状态。`PocState` 的继任者：POC 骨架已退役，
/// 这里承载第一个真实游戏页面。
#[derive(Clone)]
pub struct ShopState {
    /// 组件 id：promise 线程回调用 mark_dirty_deferred 入队时需要
    /// （不能走 mark_dirty——那会经调度回调发起异步导入并被 jco 吞掉）。
    pub component_id: std::cell::Cell<u32>,
    /// 当前分类：Some(1..=5) 为物品分类，None 为宠物页。
    pub category: ReactiveSignal<Option<u32>>,
    pub page: ReactiveSignal<u32>,
    pub keyword: ReactiveSignal<String>,
    pub filter_open: ReactiveSignal<bool>,

    /// 列表数据。None = 尚在首次加载；Some(Err) = 请求失败。
    pub items: ReactiveSignal<Option<Result<Vec<ShopItem>, String>>>,
    pub pets: ReactiveSignal<Option<Result<Vec<ShopPet>, String>>>,
    pub total_pages: ReactiveSignal<usize>,

    /// 悬浮详情：当前悬停的瓦片下标 + 视口定位。
    /// （tairitsu 的 use_signal 每次渲染新建，受控 hover 状态挂这里。）
    pub hover_key: ReactiveSignal<Option<usize>>,
    pub hover_pos: ReactiveSignal<(f64, f64)>,

    pub buying: ReactiveSignal<bool>,
    pub toast_visible: ReactiveSignal<bool>,
    pub toast_message: ReactiveSignal<String>,
    pub toast_kind: ReactiveSignal<ToastKind>,
}

impl ShopState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            category: use_signal(|| Some(1u32)),
            page: use_signal(|| 1u32),
            keyword: use_signal(String::new),
            filter_open: use_signal(|| false),
            items: use_signal(|| None),
            pets: use_signal(|| None),
            total_pages: use_signal(|| 1usize),
            hover_key: use_signal(|| None),
            hover_pos: use_signal(|| (0.0f64, 0.0f64)),
            buying: use_signal(|| false),
            toast_visible: use_signal(|| false),
            toast_message: use_signal(String::new),
            toast_kind: use_signal(|| ToastKind::Success),
        }
    }
}

pub fn show_toast(state: &ShopState, kind: ToastKind, message: String) {
    state.toast_kind.set(kind);
    state.toast_message.set(message);
    state.toast_visible.set(true);
}

/// promise 线程版：只写内部值（不触发 mark_dirty 的调度路径），
/// 渲染由 promise_commit → pm-flush 事件统一驱动。
pub fn show_toast_silent(state: &ShopState, kind: ToastKind, message: String) {
    state.toast_kind.inner().set(kind);
    state.toast_message.inner().set(message);
    state.toast_visible.inner().set(true);
}

impl std::fmt::Debug for ShopState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 组件宏会给 props 派生 Debug；信号句柄无意义，报个骨架即可。
        f.debug_struct("ShopState").finish()
    }
}

/// 宠物（对齐 rust/utils/src/types/api_pokemon.rs PokemonBasic 的显示子集）。
#[derive(Debug, Clone, Default)]
pub struct PokemonBasic {
    pub id: u64,
    /// 物种名（原始名）。
    pub name: String,
    pub nickname: Option<String>,
    pub type_id: u64,
    pub level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub is_shiny: bool,
    /// 位置：1=首位/战斗中，2=队伍，其余=箱子。
    pub site: u8,
    pub state: u8,
    pub state_text: String,
    pub state_class: String,
    /// 未满 PP 的技能列表（"pp/max"），用于治疗页提示。
    pub depleted_pp: Vec<String>,
    pub is_legendary: bool,
}

impl PokemonBasic {
    /// 显示名：优先昵称。
    pub fn display_name(&self) -> String {
        match self.nickname.as_deref() {
            Some(n) if !n.is_empty() => n.to_string(),
            _ => self.name.clone(),
        }
    }

    /// 该宠物身上的负面状态是否需要用治疗（对齐服务端负面码表）。
    pub fn needs_healing(&self) -> bool {
        self.hp <= 0
            || matches!(
                self.state,
                0 | 2 | 3 | 4 | 5 | 6 | 7 | 11 | 15 | 20 | 21 | 22
            )
    }
}

/// 治疗中心持久状态。
#[derive(Clone)]
pub struct CenterState {
    pub component_id: std::cell::Cell<u32>,
    /// None = 首次加载中；Some(Err) = 失败。
    pub pokemons: ReactiveSignal<Option<Result<Vec<PokemonBasic>, String>>>,
    /// 是否处于战斗（npcid > 0）：战斗中首位宠物只能「脱战并治疗」。
    pub in_battle: ReactiveSignal<bool>,
    /// 治疗请求进行中（互斥守卫）。
    pub healing: ReactiveSignal<bool>,
    pub toast_visible: ReactiveSignal<bool>,
    pub toast_message: ReactiveSignal<String>,
    pub toast_kind: ReactiveSignal<ToastKind>,
}

impl CenterState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            pokemons: use_signal(|| None),
            in_battle: use_signal(|| false),
            healing: use_signal(|| false),
            toast_visible: use_signal(|| false),
            toast_message: use_signal(String::new),
            toast_kind: use_signal(|| ToastKind::Success),
        }
    }
}

impl std::fmt::Debug for CenterState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CenterState").finish()
    }
}

pub fn center_show_toast(state: &CenterState, kind: ToastKind, message: String) {
    state.toast_kind.set(kind);
    state.toast_message.set(message);
    state.toast_visible.set(true);
}

pub fn center_show_toast_silent(state: &CenterState, kind: ToastKind, message: String) {
    state.toast_kind.inner().set(kind);
    state.toast_message.inner().set(message);
    state.toast_visible.inner().set(true);
}

/// 我的宝可梦页签名：当前只移植了「属性」标签页；装备与技能随后续阶段接入。
#[derive(Clone, Copy, PartialEq)]
pub enum MyPokemonTab {
    Stats,
}

/// 宠物详情（我的宝可梦「属性」页所需字段）。
///
/// 定义在 state 而非 api：`api` 模块只在 wasm 下编译，而持久状态要能在
/// 测试目标（host）下编译。`crate::api::pokemon_detail` 负责填充它。
#[derive(Debug, Clone, Default)]
pub struct PokemonDetail {
    pub id: u64,
    pub name: String,
    pub nickname: Option<String>,
    pub type_id: u64,
    pub level: u64,
    pub exp: u64,
    pub exp_for_current_level: u64,
    pub exp_for_next_level: u64,
    pub hp: i64,
    pub max_hp: i64,
    pub gender: u8,
    pub is_shiny: bool,
    pub state: u8,
    pub state_text: String,
    pub state_class: String,
    pub affection: u32,
    /// 六项个体值（旧端点可能无此字段）。
    pub ivs: Option<[u64; 6]>,
    pub quality_rank: Option<u8>,
    pub quality_grade: Option<String>,
    /// (攻击, 防御, 特攻, 特防, 速度)。
    pub stats: Option<[i64; 5]>,
    pub type_1: String,
    pub type_2: Option<String>,
    pub is_legendary: bool,
}

/// 我的宝可梦「属性」页持久状态。
#[derive(Clone)]
pub struct MyPokemonState {
    pub component_id: std::cell::Cell<u32>,
    pub tab: ReactiveSignal<MyPokemonTab>,
    /// 当前选中的宠物 ID；None = 列表第一只。
    pub selected_id: ReactiveSignal<Option<u64>>,
    /// 详情（属性面板数据）。
    pub detail: ReactiveSignal<Option<Result<PokemonDetail, String>>>,
    pub editing: ReactiveSignal<bool>,
    pub edit_value: ReactiveSignal<String>,
    /// 保存中（互斥守卫）。
    pub saving: ReactiveSignal<bool>,
    pub save_error: ReactiveSignal<Option<String>>,
    pub save_success: ReactiveSignal<bool>,
}

impl MyPokemonState {
    pub fn new() -> Self {
        Self {
            component_id: std::cell::Cell::new(0),
            tab: use_signal(|| MyPokemonTab::Stats),
            selected_id: use_signal(|| None),
            detail: use_signal(|| None),
            editing: use_signal(|| false),
            edit_value: use_signal(String::new),
            saving: use_signal(|| false),
            save_error: use_signal(|| None),
            save_success: use_signal(|| false),
        }
    }
}

impl std::fmt::Debug for MyPokemonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MyPokemonState").finish()
    }
}
