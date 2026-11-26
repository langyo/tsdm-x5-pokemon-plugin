//! 应用级轻提示（对齐 Dioxus 版 `state/ui.rs` 的 toast 部分与
//! `components/common/toast.rs`）。
//!
//! - 列表放 tairitsu 的全局 [`Store`]：任何回调里直接 `toast::show`，
//!   不用把应用状态层层传下去。
//! - 根组件启动时 `toast::init` 订阅一次，列表一变就标脏重渲染。
//! - 展示：hikari `Toast` 标准件承载消息本体；外层沿用旧版 `.toast-item`
//!   的结构与类名（复制 / 关闭是旧版就有的操作，hikari Toast 没有动作槽），
//!   观感由 `rust/game2/styles/main.scss` 还原。
//!
//! `store!` 宏目前不可用（同名的 static 与 fn 冲突，且 `Store` 不是 `Sync`，
//! 进不了 `static OnceLock`），所以这里手写等价的登记逻辑。

use hikari_components::{Toast, ToastPosition, ToastProps, ToastVariant};
use tairitsu_hooks::{register_store, Store};
use tairitsu_macros::rsx;
use tairitsu_vdom::VNode;

use crate::state::ToastKind;

impl ToastKind {
    /// 旧版 `.toast-item` 的修饰类（game.css）。
    fn legacy_class(self) -> &'static str {
        match self {
            ToastKind::Success => "toast-item success",
            ToastKind::Error => "toast-item error",
            ToastKind::Info => "toast-item info",
            ToastKind::Warning => "toast-item warning",
        }
    }

    fn variant(self) -> ToastVariant {
        match self {
            ToastKind::Success => ToastVariant::Success,
            ToastKind::Error => ToastVariant::Error,
            ToastKind::Info => ToastVariant::Info,
            ToastKind::Warning => ToastVariant::Warning,
        }
    }

    /// 错误/警告停留更久便于阅读和复制，但也要自动消失（旧版数值）。
    fn timeout_ms(self) -> i32 {
        match self {
            ToastKind::Error => 8000,
            ToastKind::Warning => 6000,
            ToastKind::Success | ToastKind::Info => 3000,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ToastInfo {
    pub id: u64,
    pub message: String,
    pub kind: ToastKind,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ToastState {
    pub toasts: Vec<ToastInfo>,
    counter: u64,
}

/// 最多同时显示的条数（旧版上限），超出时挤掉最旧的一条。
pub const MAX_TOASTS: usize = 3;

thread_local! {
    static TOAST_STORE: Store<ToastState> =
        register_store(Store::new(ToastState::default()));
}

pub fn store() -> Store<ToastState> {
    TOAST_STORE.with(|s| s.clone())
}

/// 根组件启动时调用一次：列表变更 → 标脏根组件。
pub fn init() {
    store().subscribe(|_| crate::request_remount());
}

pub fn show(kind: ToastKind, message: impl Into<String>) {
    let store = store();
    let id = store.get().counter;
    store.update(|s| {
        if s.toasts.len() >= MAX_TOASTS {
            s.toasts.remove(0);
        }
        s.counter += 1;
        s.toasts.push(ToastInfo {
            id,
            message: message.into(),
            kind,
        });
    });
    schedule_hide(id, kind.timeout_ms());
}

/// 错误提示只保留最新一条（旧版 `show_error`）。
pub fn show_error(message: impl Into<String>) {
    store().update(|s| s.toasts.retain(|t| t.kind != ToastKind::Error));
    show(ToastKind::Error, message);
}

pub fn hide(id: u64) {
    store().update(|s| s.toasts.retain(|t| t.id != id));
}

fn schedule_hide(id: u64, ms: i32) {
    use tairitsu_vdom::platform::TimerOps;
    let Ok(platform) = tairitsu_web::WitPlatform::new() else {
        return;
    };
    platform.set_timeout(Box::new(move || hide(id)), ms);
}

fn copy_to_clipboard(text: &str) -> bool {
    tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::platform_helpers::copy_to_clipboard(text)
}

/// 顶部提示容器，挂在应用根部（对齐 Dioxus 版 `ToastProvider`）。
/// 紧随其后的 `div.toast-spacer` 由调用方一并渲染。
pub fn container() -> VNode {
    let items: Vec<VNode> = store().get().toasts.iter().map(render_item).collect();
    rsx! {
        div { class: "toast-container", ..items }
    }
}

fn render_item(toast: &ToastInfo) -> VNode {
    let id = toast.id;
    let message = toast.message.clone();
    let copy_message = message.clone();
    rsx! {
        div {
            class: toast.kind.legacy_class(),
            onclick: move |_| hide(id),
            Toast {
                variant: toast.kind.variant(),
                message: message,
                position: ToastPosition::TopCenter,
                closable: false,
                class: "toast-item-body".to_string(),
            }
            div { class: "toast-actions",
                button {
                    class: "toast-copy",
                    onclick: move |e: tairitsu_vdom::MouseEvent| {
                        e.stop_propagation();
                        copy_to_clipboard(&copy_message);
                    },
                    "⧉"
                }
                button { class: "toast-close", "×" }
            }
        }
    }
}
