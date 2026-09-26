use crate::dioxus::prelude::*;

use crate::dioxus::{components::user_grant::StepIndicator, pages::shared::ActionModal};

/// 通用的两步向导 Modal
///
/// 用于需要两个步骤的操作流程，提供统一的 UI 模式：
/// - 顶部：步骤指示器
/// - 中间：根据步骤显示不同内容
/// - 底部：操作按钮（取消/下一步 或 上一步/确认）
///
/// # 使用示例
/// ```rust,ignore
/// TwoStepWizardModal {
///     title: "给予宠物".to_string(),
///     step: *step.read(),
///     steps: vec!["搜索宠物".to_string(), "设置属性".to_string()],
///     can_proceed: can_go_to_step_2,
///     disabled: is_busy,
///     on_close: move |_| { /* 关闭逻辑 */ },
///     on_confirm: move |_| { /* 确认逻辑 */ },
///     on_step_change: move |new_step| step.set(new_step),
///     // 根据步骤渲染不同内容
///     children: if *step.read() == 0 {
///         rsx! { /* 第一步内容 */ }
///     } else {
///         rsx! { /* 第二步内容 */ }
///     },
/// }
/// ```

#[component]
pub fn TwoStepWizardModal(
    /// Modal 标题
    title: String,
    /// 当前步骤 (0 或 1)
    step: usize,
    /// 步骤标签列表
    steps: Vec<String>,
    /// 是否可以进入下一步（用于第一步的"下一步"按钮）
    can_proceed: bool,
    /// 禁用状态
    #[props(default)]
    disabled: bool,
    /// 关闭回调
    on_close: EventHandler<()>,
    /// 确认回调（第二步的"确认"按钮）
    on_confirm: EventHandler<()>,
    /// 步骤变化回调
    on_step_change: EventHandler<usize>,
    /// 子内容（根据步骤显示不同的内容）
    children: Element,
) -> Element {
    rsx! {
        ActionModal {
            title: title.clone(),
            on_close: move |_| {
                on_close.call(());
            },
            div { class: "admin-form-editor admin-form-editor--compact",
                // 步骤指示器
                StepIndicator { steps: steps.clone(), current_step: step }

                // 内容区域
                {children}

                // 操作按钮
                div { class: "admin-form-actions",
                    if step == 0 {
                        // 第一步按钮
                        button {
                            class: "admin-btn",
                            disabled,
                            r#type: "button",
                            onclick: move |_| on_close.call(()),
                            "取消"
                        }
                        button {
                            class: "admin-btn admin-btn--primary",
                            disabled: disabled || !can_proceed,
                            r#type: "button",
                            onclick: move |_| on_step_change.call(1),
                            "下一步"
                        }
                    } else {
                        // 第二步按钮
                        button {
                            class: "admin-btn",
                            disabled,
                            r#type: "button",
                            onclick: move |_| on_step_change.call(0),
                            "← 返回"
                        }
                        button {
                            class: "admin-btn admin-btn--primary",
                            disabled,
                            r#type: "button",
                            onclick: move |_| on_confirm.call(()),
                            "确认"
                        }
                    }
                }
            }
        }
    }
}
