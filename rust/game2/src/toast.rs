//! 应用级轻提示：上游 hikari `ToastStore` 的薄适配。
//!
//! 早先这里自己养了一套（`Store<ToastState>` 存列表、`set_timeout` 定时移除、
//! 手画的 `.toast-item` 容器与复制/关闭按钮）。hikari 的 `ToastStore` 把这些
//! 全做了：`push` 按类型入槽、自动消失、容器 `Toast` 自带关闭与复制（槽位
//! `copyable` + 容器的 `on_copy`），同类型的后续消息还会合并进同一个槽并显示
//! 计数。
//!
//! 旧版 `show_toast` 的时长与可复制性经 `ToastOptions` 传下去：失败 8 秒、
//! 警告 6 秒、其余 3 秒（hikari 默认失败/警告 30 秒），每条都带「复制」（hikari
//! 默认只有失败/警告有）。观感由皮肤（`styles/main.scss` 的 Toast 段）压回旧版
//! 的按类型着色药丸。
//!
//! 页面代码的调用形态保持不变（`toast::show(ToastKind::Error, "...")` /
//! `toast::show_error(...)`），二十来处调用点不用动；差异只在这个文件里。

use hikari_components::{use_toast, Toast, ToastOptions, ToastProps, ToastType};
use tairitsu_vdom::{Callback, VNode};

use crate::state::ToastKind;

impl ToastKind {
    /// 入栈到 hikari 的哪一个类型槽，以及旧版给这个类型的停留时长（毫秒）。
    fn slot(self) -> (ToastType, u64) {
        match self {
            ToastKind::Success => (ToastType::Success, 3_000),
            ToastKind::Error => (ToastType::Error, 8_000),
            ToastKind::Info => (ToastType::Info, 3_000),
            ToastKind::Warning => (ToastType::Warning, 6_000),
        }
    }

    /// 入栈。时长按旧版 `show_toast`，并打开「复制」（旧版每条提示都有）。
    fn store_push(self, message: &str) {
        let (toast_type, duration_ms) = self.slot();
        use_toast().push(
            toast_type,
            message,
            ToastOptions {
                duration: Some(duration_ms),
                copyable: Some(true),
                action: None,
            },
        );
    }
}

/// 根组件启动时调用一次。hikari 的 store 自己在渲染时订阅并管定时器，这里
/// 不需要转发标脏；保留这个空函数是为了让 `lib.rs` 的启动顺序读起来完整。
pub fn init() {}

/// 顶部提示容器，挂在应用根部。`Toast` 自带 `.hk-toast-container`；复制按钮
/// 的落点由这里给（hikari 不自己碰剪贴板）。
pub fn container() -> VNode {
    Toast(ToastProps {
        on_copy: Some(Callback::new(|text: String| {
            copy_to_clipboard(&text);
        })),
        ..Default::default()
    })
}

/// 把提示文字写进剪贴板（与战报的「复制战报」同一个平台接口）。
fn copy_to_clipboard(text: &str) {
    tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::platform_helpers::copy_to_clipboard(text);
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
    ToastKind::Error.store_push(&message);
}

pub fn store() -> hikari_components::ToastStore {
    use_toast()
}
