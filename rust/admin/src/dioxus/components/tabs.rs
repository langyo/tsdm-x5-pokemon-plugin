use crate::dioxus::prelude::*;

/// 通用的标签页组件 - 使用 modal tabs 样式
#[component]
pub fn Tabs<T: PartialEq + Clone + Copy + 'static>(
    /// 当前选中的标签页
    active: T,
    /// 标签页选项列表：(值, 标签文本)
    options: Vec<(T, String)>,
    /// 禁用状态
    #[props(default)]
    disabled: bool,
    /// 标签页切换回调
    on_change: EventHandler<T>,
) -> Element {
    rsx! {
        div { class: "admin-modal-tabs",
            for (value , label) in options {
                button {
                    class: if value == active { "admin-modal-tab is-active" } else { "admin-modal-tab" },
                    disabled,
                    r#type: "button",
                    onclick: move |_| on_change.call(value),
                    "{label}"
                }
            }
        }
    }
}
