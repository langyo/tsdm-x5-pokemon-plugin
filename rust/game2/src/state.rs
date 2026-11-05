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
