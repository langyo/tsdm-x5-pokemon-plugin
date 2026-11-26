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
//!   （表格的「管理」列触发后弹的就是它）；
//! - [`section_card`] / [`page_header`] / [`alert`] / [`spinner`]：版面与状态；
//! - [`form_field`]：标签 + 控件 + 说明的一格表单；
//! - [`text_field`] / [`textarea_field`] / [`switch`] / [`select_field`]：表单字段。

use std::rc::Rc;

use hikari_components::feedback::popover::{
    PopoverAnchorRect, PopoverPlacement, popover_position_style,
};
use hikari_components::prelude::{Icon, IconProps, MdiIcon};
use hikari_components::{
    Alert, AlertProps, AlertVariant, Button, ButtonProps, ButtonSize, ButtonVariant, FormField,
    FormFieldProps, Input, InputProps, Modal, ModalAction, ModalActionVariant, ModalProps,
    PageHeader, PageHeaderProps, SectionCard, SectionCardProps, Select, SelectOption, SelectProps,
    Spinner, SpinnerProps, Switch, SwitchProps, Textarea, TextareaProps,
};
use tairitsu_macros::rsx;
use tairitsu_vdom::{EventHandler, VNode, VText};

use crate::app_state::AppState;
use crate::state::{MenuAnchor, RowMenu};

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

/// 在 `position: fixed` 的坐标系里量一个元素。
///
/// hikari 的 Select / Popover 面板是 fixed 定位，而弹窗面板
/// （`.hk-modal-content`）靠 transform 居中——按 CSS 规范它是 fixed 后代的
/// 包含块，面板的 left / top 从弹窗面板的左上角算起。把触发钮的矩形换算到
/// 这个坐标系里，面板才贴得住触发钮；不在弹窗里时就是视口坐标。
pub fn measure_fixed(selector: &str) -> Option<MenuAnchor> {
    use tairitsu_vdom::platform::{DomOps, LayoutOps};
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element;
    let platform = tairitsu_web::WitPlatform::new().ok()?;
    let app = crate::element_handle("app")?;
    let app = tairitsu_web::WitElement::from_raw(app);
    let trigger = platform.query_selector_on(&app, selector)?;
    let rect = platform.get_bounding_client_rect(&trigger);
    let mut anchor = MenuAnchor {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    };
    if let Some(block) = element::closest(trigger.as_raw(), ".hk-modal-content") {
        let origin = element::get_bounding_client_rect(block);
        anchor.x -= origin.x;
        anchor.y -= origin.y;
    }
    Some(anchor)
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

// ---------------------------------------------------------------------------
// 版面
// ---------------------------------------------------------------------------

/// hikari `PageHeader`：页头——标题、副标题、右侧动作。列表页由
/// `AdminTablePage` 自带页头，这一给没有表格的页面用。
pub fn page_header(title: &str, subtitle: Option<String>, actions: Vec<VNode>) -> VNode {
    let action_slot = if actions.is_empty() {
        None
    } else {
        Some(rsx! {
            div { class: "hk-button-row", ..actions }
        })
    };
    PageHeader(PageHeaderProps {
        title: title.to_string(),
        subtitle,
        actions: action_slot,
        ..Default::default()
    })
}

/// hikari `Alert`：一条横幅（错误 / 提醒 / 信息 / 成功）。
pub fn alert(variant: AlertVariant, message: &str) -> VNode {
    Alert(AlertProps {
        variant,
        message: message.to_string(),
        ..Default::default()
    })
}

/// hikari `Spinner`：居中的加载指示。
pub fn spinner() -> VNode {
    Spinner(SpinnerProps {
        center: true,
        ..Default::default()
    })
}

/// hikari `FormField`：标签 + 控件 + 说明。标签留空则只渲染控件与说明。
///
/// 这个组件的样式来自 Rust 镜像自己的 `StyledComponent`，Vue 侧没有对应表，
/// 所以由 `scripts/publish/dump_hikari_mirror_css.py` 单独补一份
/// `admin-mirror.css`（见该脚本头注）。
pub fn form_field(label: &str, help: Option<String>, control: VNode) -> VNode {
    FormField(FormFieldProps {
        label: label.to_string(),
        help_text: help,
        children: control,
        ..Default::default()
    })
}

/// hikari `SectionCard`：一张带标题的分区卡片。`actions` 放在标题右侧，
/// `body` 是卡片正文（按 `adm-cfg-grid` 这类宿主网格自行排布）。
pub fn section_card(title: &str, actions: Vec<VNode>, body: Vec<VNode>) -> VNode {
    let action_slot = if actions.is_empty() {
        None
    } else {
        Some(rsx! {
            div { class: "hk-button-row", ..actions }
        })
    };
    SectionCard(SectionCardProps {
        title: title.to_string(),
        actions: action_slot,
        children: rsx! {
            div { class: "adm-section-body", ..body }
        },
        ..Default::default()
    })
}

// ---------------------------------------------------------------------------
// 表单字段
// ---------------------------------------------------------------------------

/// 一个字段怎么写回状态：`(页面状态, 字段键, 新值)`。各编辑器的
/// `set_*_field` 都是这个形状，写完会自己请求重挂。
pub type Setter = fn(&AppState, &str, &str);

/// 文本字段在 DOM 里的 id 前缀：`{前缀}{key}`，整页唯一。
const INPUT_ID_PREFIX: &str = "hkf-";

/// 按选择器读一个输入框此刻的值（从 `#app` 起找第一个命中）。
///
/// hikari 的 `Input` / `Textarea` 镜像在 `oninput` 里回传的是 `event.data`
/// ——本次输入插入的那几个字符，不是整个字段的新值（删除、粘贴、输入法
/// 组合时它更对不上）。宿主于是不收它给的值，事件一到就按选择器找到
/// 输入框直接读 DOM。tairitsu-web 给 `ChangeEvent.value` / `InputEvent.value`
/// 取值用的是同一个读法（`html-input-element` 的 getter 对 textarea 同样
/// 有效）。上游改传 `event.value`、且带它的 tairitsu 版本进了 hikari 的
/// 依赖下限之后，这里连同各 `oninput` 里的读值一起删掉。
pub fn dom_value(selector: &str) -> Option<String> {
    use tairitsu_vdom::platform::DomOps;
    use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::html_input_element;
    let platform = tairitsu_web::WitPlatform::new().ok()?;
    let app = crate::element_handle("app")?;
    let app = tairitsu_web::WitElement::from_raw(app);
    let element = platform.query_selector_on(&app, selector)?;
    Some(html_input_element::get_value(element.as_raw()))
}

/// 文本 / 数字字段：hikari Input。输入即写状态；重挂时输入框的值与 DOM
/// 一致，diff 不出补丁，光标不动。
///
/// `key` 是这个输入框在页面里的 DOM 键（整页唯一，落在 id 上），`field`
/// 才是写回状态的字段名。
pub fn text_field(
    st: &AppState,
    set: Setter,
    key: &'static str,
    field: &'static str,
    label: &str,
    value: String,
    input_type: &'static str,
) -> VNode {
    let field_state = st.clone();
    let selector = format!("#{INPUT_ID_PREFIX}{key}");
    Input(InputProps {
        id: Some(format!("{INPUT_ID_PREFIX}{key}")),
        // 标签留空 = 不渲染标签行（搜索框这类自带占位文字的格子）。
        label: (!label.is_empty()).then(|| label.to_string()),
        model_value: value,
        input_type: Some(input_type.to_string()),
        oninput: Some(EventHandler::new(move |_inserted: String| {
            // 找不到输入框时保持原状态，别把空值写回去冲掉用户敲的字。
            if let Some(text) = dom_value(&selector) {
                set(&field_state, field, &text);
            }
        })),
        ..Default::default()
    })
}

/// 多行字段：标签 + hikari Textarea（它自己不带标签）。标签留空则不渲染
/// 标签行。hikari 的 Textarea 不收 id，`key` 落在包着它的 label 的
/// `data-hk-field` 上。
pub fn textarea_field(
    st: &AppState,
    set: Setter,
    key: &'static str,
    field: &'static str,
    label: &str,
    value: String,
    rows: u32,
) -> VNode {
    let field_state = st.clone();
    let selector = format!("[data-hk-field=\"{key}\"] textarea");
    let area = Textarea(TextareaProps {
        model_value: value,
        rows,
        oninput: Some(EventHandler::new(move |_inserted: String| {
            if let Some(text) = dom_value(&selector) {
                set(&field_state, field, &text);
            }
        })),
        ..Default::default()
    });
    let label_node = if label.is_empty() {
        VNode::empty()
    } else {
        rsx! {
            span { "{label}" }
        }
    };
    let kids: Vec<VNode> = vec![label_node, area];
    rsx! {
        label {
            class: "adm-editor-field adm-editor-wide",
            "data-hk-field": key,
            ..kids
        }
    }
}

/// 开关：hikari Switch。镜像不存状态，翻转后的动作由宿主给。
pub fn switch(checked: bool, label: &str, toggle: impl Fn() + 'static) -> VNode {
    Switch(SwitchProps {
        checked,
        label: Some(label.to_string()),
        on_change: Some(EventHandler::new(move |_next: bool| toggle())),
        children: VNode::empty(),
        ..Default::default()
    })
}

/// 下拉：hikari Select。
///
/// 镜像不存状态：开合与锚点由宿主记在 `select_ui` 上（一次只开一个），
/// 桌面上的「点外部关闭」也归宿主——打开时铺一层透明遮罩，层级压在
/// 弹出面板之下、同一层叠上下文的其余内容之上。
pub fn select_field(
    st: &AppState,
    set: Setter,
    key: &'static str,
    field: &'static str,
    label: &str,
    options: Vec<(String, String)>,
    value: String,
) -> VNode {
    let open = st.select_ui.get().filter(|ui| ui.key == key);
    let is_open = open.is_some();
    let toggle_state = st.clone();
    let pick_state = st.clone();
    let close_state = st.clone();
    let key_for_toggle = key.to_string();
    let select = Select(SelectProps {
        label: Some(label.to_string()),
        options: options
            .into_iter()
            .map(|(value, label)| SelectOption {
                value,
                label,
                disabled: false,
            })
            .collect(),
        model_value: value,
        open: is_open,
        anchor: open.map(|ui| PopoverAnchorRect {
            x: ui.anchor.x,
            y: ui.anchor.y,
            width: ui.anchor.width,
            height: ui.anchor.height,
        }),
        on_open_change: Some(EventHandler::new(move |open: bool| {
            let ui = if open {
                measure_fixed(&format!(
                    "[data-hk-select=\"{key_for_toggle}\"] .hk-select-trigger"
                ))
                .map(|anchor| RowMenu {
                    key: key_for_toggle.clone(),
                    anchor,
                })
            } else {
                None
            };
            toggle_state.select_ui.inner().set(ui);
            crate::request_remount();
        })),
        on_value_change: Some(EventHandler::new(move |value: String| {
            set(&pick_state, field, &value);
            pick_state.select_ui.inner().set(None);
            crate::request_remount();
        })),
        ..Default::default()
    });
    let backdrop = if is_open {
        rsx! {
            div {
                class: "admin-select-backdrop",
                onclick: move |_| {
                    close_state.select_ui.inner().set(None);
                    crate::request_remount();
                },
            }
        }
    } else {
        VNode::empty()
    };
    let parts: Vec<VNode> = vec![select, backdrop];
    rsx! {
        div {
            class: "admin-select",
            "data-hk-select": key,
            ..parts
        }
    }
}
