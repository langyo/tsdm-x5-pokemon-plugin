//! hikari 标准件的宿主适配层。
//!
//! hikari 的 Rust 镜像组件都是受控的（见各组件头注的 STATE CONTRACT）：开合、
//! 锚点、按下后做什么，全由宿主给，组件只负责渲染。页面代码不该为每个控件
//! 重复「造 props、包 EventHandler」的样板，所以这里收成几个小函数——页面
//! 只描述「放什么、点了做什么」：
//!
//! - [`HkAction`]：一个动作（标签 + 外观 + 回调），按钮、菜单行、弹窗页脚共用；
//! - [`button`]：hikari `Button`；
//! - [`modal`]：hikari `Modal`（标题、宽度、页脚动作）；
//! - [`anchored_menu`]：贴着锚点展开的动作菜单，行语法沿用 hikari `HkMenu`
//!   （表格的「管理」列触发后弹的就是它）。

use std::rc::Rc;

use hikari_components::feedback::popover::{
    PopoverAnchorRect, PopoverPlacement, popover_position_style,
};
use hikari_components::prelude::{Icon, IconProps, MdiIcon};
use hikari_components::{
    Button, ButtonProps, ButtonSize, ButtonVariant, Modal, ModalAction, ModalActionVariant,
    ModalProps,
};
use tairitsu_macros::rsx;
use tairitsu_vdom::{EventHandler, VNode, VText};

use crate::state::MenuAnchor;

/// 一个动作：标签 + 外观 + 按下后做什么（菜单行可带一个图标）。
#[derive(Clone)]
pub struct HkAction {
    pub label: String,
    pub variant: ButtonVariant,
    pub disabled: bool,
    pub icon: Option<MdiIcon>,
    pub run: Rc<dyn Fn()>,
}

impl HkAction {
    pub fn new(label: &str, variant: ButtonVariant, run: impl Fn() + 'static) -> Self {
        Self {
            label: label.to_string(),
            variant,
            disabled: false,
            icon: None,
            run: Rc::new(run),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon(mut self, icon: MdiIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

fn event_handler(run: Rc<dyn Fn()>) -> EventHandler<tairitsu_vdom::MouseEvent> {
    EventHandler::new(move |_event: tairitsu_vdom::MouseEvent| run())
}

/// hikari `Button`。
pub fn button(
    label: &str,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    run: impl Fn() + 'static,
) -> VNode {
    Button(ButtonProps {
        variant,
        size,
        disabled,
        children: VNode::Text(VText::new(label)),
        onclick: Some(event_handler(Rc::new(run))),
        ..Default::default()
    })
}

/// 由一个 [`HkAction`] 渲染的按钮。
pub fn action_button(action: &HkAction, size: ButtonSize) -> VNode {
    Button(ButtonProps {
        variant: action.variant,
        size,
        disabled: action.disabled,
        children: VNode::Text(VText::new(action.label.as_str())),
        onclick: Some(event_handler(action.run.clone())),
        ..Default::default()
    })
}

/// hikari `Modal`。调用方只在弹窗该开着的时候渲染它——这里固定 `open`，
/// 关闭走 `on_close`（✕、点遮罩、Esc 都汇到那里）。
pub fn modal(
    title: &str,
    width: &str,
    on_close: impl Fn() + 'static,
    body: VNode,
    footer: Vec<HkAction>,
) -> VNode {
    let on_close = Rc::new(on_close);
    let close = on_close.clone();
    Modal(ModalProps {
        open: true,
        title: Some(title.to_string()),
        width: width.to_string(),
        footer_actions: Some(
            footer
                .into_iter()
                .map(|action| {
                    let run = action.run.clone();
                    ModalAction {
                        label: action.label,
                        variant: match action.variant {
                            ButtonVariant::Primary => ModalActionVariant::Primary,
                            ButtonVariant::Danger => ModalActionVariant::Danger,
                            ButtonVariant::Ghost => ModalActionVariant::Ghost,
                            ButtonVariant::Outline => ModalActionVariant::Outline,
                            ButtonVariant::Secondary => ModalActionVariant::Secondary,
                        },
                        loading: false,
                        disabled: action.disabled,
                        on_click: Some(EventHandler::new(move |_event: ()| run())),
                    }
                })
                .collect(),
        ),
        children: body,
        on_close: Some(EventHandler::new(move |_event: tairitsu_vdom::MouseEvent| {
            close()
        })),
        ..Default::default()
    })
}

/// 量一个元素的视口矩形：从 `#app` 起按选择器找第一个命中。
pub fn measure(selector: &str) -> Option<MenuAnchor> {
    use tairitsu_vdom::platform::{DomOps, LayoutOps};
    let platform = tairitsu_web::WitPlatform::new().ok()?;
    let app = crate::element_handle("app")?;
    let app = tairitsu_web::WitElement::from_raw(app);
    let element = platform.query_selector_on(&app, selector)?;
    let rect = platform.get_bounding_client_rect(&element);
    Some(MenuAnchor {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    })
}

/// 贴着锚点展开的动作菜单（右缘对齐锚点右缘，向下）。
///
/// 行语法沿用 hikari `HkMenu`（`button.hk-menu-row > 图标 + 标签`），面板是
/// `HkSelect` 的浮层外观（`.hk-select-popout*`），位置由 hikari 的
/// `popover_position_style` 算。
pub fn anchored_menu(anchor: MenuAnchor, actions: Vec<HkAction>, class: &str) -> VNode {
    let style = popover_position_style(
        PopoverPlacement::BottomEnd,
        PopoverAnchorRect {
            x: anchor.x,
            y: anchor.y,
            width: anchor.width,
            height: anchor.height,
        },
        4.0,
    );
    let rows: Vec<VNode> = actions
        .into_iter()
        .map(|action| {
            let HkAction { label, variant, disabled, icon, run } = action;
            let danger = variant == ButtonVariant::Danger;
            let glyph = icon.map(|icon| {
                rsx! {
                    span {
                        class: "hk-menu-item-icon",
                        {Icon(IconProps {
                            icon,
                            size: 16,
                            ..Default::default()
                        })}
                    }
                }
            });
            rsx! {
                button {
                    "type": "button",
                    class: "hk-menu-row",
                    "data-danger": if danger { Some("true") } else { None },
                    disabled: disabled,
                    onclick: move |event: tairitsu_vdom::MouseEvent| {
                        event.stop_propagation();
                        run();
                    },
                    ..vec![glyph.unwrap_or_else(VNode::empty)]
                    span { class: "hk-menu-label", "{label}" }
                }
            }
        })
        .collect();
    let class = format!("hk-select-popout-host {class}");
    rsx! {
        div {
            class: class,
            "data-side": "bottom",
            "data-align": "end",
            style: style,
            "data-test-adm-row-menu": "1",
            div {
                class: "hk-select-popout",
                "role": "menu",
                div { class: "hk-menu-level", ..rows }
            }
        }
    }
}
