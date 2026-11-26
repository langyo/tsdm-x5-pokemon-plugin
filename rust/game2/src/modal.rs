//! 通用弹窗外壳（对齐 Dioxus 版 `components/common/modal.rs` 的 Modal / ModalLg）。
//!
//! 结构、类名与旧版一致，样式走 game.css（.modal-overlay / .modal-container /
//! .modal-header / .modal-body）。点击遮罩或关闭按钮回调 `on_close`；点弹窗
//! 本体不关闭（容器上 stop_propagation，与旧版相同——委托时代靠属性向上
//! 匹配，点弹窗内部也会冒泡到遮罩而误关）。
//!
//! 这是旧版 Markup 的直接移植，事件用 tairitsu 原生绑定；换成 hikari 的
//! `Modal` 标准件是后续「页面逐个改用标准件」那一轮的事。

use std::rc::Rc;

use tairitsu_macros::rsx;
use tairitsu_vdom::VNode;

/// 弹窗宽度：`modal-container` 或 `modal-container modal-lg`。
pub fn modal(title: &str, large: bool, on_close: Rc<dyn Fn()>, body: Vec<VNode>) -> VNode {
    let title = title.to_string();
    let container_cls = if large {
        "modal-container modal-lg"
    } else {
        "modal-container"
    };
    let close_on_overlay = Rc::clone(&on_close);
    let close_on_button = Rc::clone(&on_close);
    rsx! {
        div { class: "modal-overlay",
            onclick: move |_| close_on_overlay(),
            div { class: container_cls,
                onclick: move |e: tairitsu_vdom::MouseEvent| e.stop_propagation(),
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close-btn", onclick: move |_| close_on_button(), "×" }
                }
                div { class: "modal-body custom-scrollbar", ..body }
            }
        }
    }
}
