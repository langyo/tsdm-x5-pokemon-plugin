//! 应用级轻提示：上游 hikari `ToastStore` 的薄适配。
//!
//! 早先这里自己养了一套（`Store<ToastState>` 存列表、`set_timeout` 定时移除、
//! 手画的 `.toast-item` 容器与复制/关闭按钮）。hikari 的 `ToastStore` 把这些
//! 全做了：`success/error/info/warning` 入栈、按类型给默认时长、自动消失、
//! 容器 `Toast` 自带关闭与复制（`on_remove` / `on_copy`），同类型的后续消息
//! 还会合并进同一个槽并显示计数。
//!
//! 页面代码的调用形态保持不变（`toast::show(ToastKind::Error, "...")` /
//! `toast::show_error(...)`），二十来处调用点不用动；差异只在这个文件里。

use hikari_components::{use_toast, Toast, ToastProps};
use tairitsu_vdom::VNode;

use crate::state::ToastKind;

impl ToastKind {
    /// 入栈到 hikari 的哪一个类型槽。
    fn store_push(self, message: &str) {
        let store = use_toast();
        match self {
            ToastKind::Success => {
                store.success(message);
            }
            ToastKind::Error => {
                store.error(message);
            }
            ToastKind::Info => {
                store.info(message);
            }
            ToastKind::Warning => {
                store.warning(message);
            }
        }
    }
}

/// 根组件启动时调用一次。hikari 的 store 自己在渲染时订阅并管定时器，这里
/// 不需要转发标脏；保留这个空函数是为了让 `lib.rs` 的启动顺序读起来完整。
pub fn init() {}

/// 顶部提示容器，挂在应用根部。`Toast` 自带 `.hk-toast-container`。
pub fn container() -> VNode {
    Toast(ToastProps::default())
}

pub fn show(kind: ToastKind, message: impl Into<String>) {
    kind.store_push(&message.into());
}

/// 错误提示只保留最新一条（旧版 `show_error` 的语义）。
///
/// hikari 的 store 按类型聚合成一个槽，同类型的后续消息落在同一槽里，所以
/// 「清掉旧错误」由它天然满足；这里只负责入栈。
pub fn show_error(message: impl Into<String>) {
    let message = message.into();
    let store = use_toast();
    store.error(&message);
}

pub fn store() -> hikari_components::ToastStore {
    use_toast()
}
