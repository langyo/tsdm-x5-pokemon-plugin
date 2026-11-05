//! POC 页面：hikari 组件 + 信号列表 + 异步 fetch 回调桥 + 内联 SVG + scss!。
//!
//! tairitsu 的状态模式：use_signal 每次渲染都会创建新信号（受控组件设计），
//! 持久状态必须放在渲染闭包外（run_app 创建 PocState），经 props 传入组件；
//! 事件回调捕获同一组信号的克隆进行修改。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use hikari_components::{
    Button, ButtonProps, ButtonVariant, ThemeProvider, Toast, ToastPosition, ToastProps,
    ToastVariant,
};
use hikari_components::theme::ThemeProviderProps;
use tairitsu_hooks::{use_signal, ReactiveSignal};

/// 内联 lucide coins 图标（与 game 端 Icon 组件同一份路径数据）。
const ICON_COINS: &str = r#"<circle cx="8" cy="8" r="6"/><path d="M18.09 10.37A6 6 0 1 1 10.34 18"/><path d="M7 6h1v4"/><path d="m16.71 13.88.7.71-2.82 2.82"/>"#;

/// POC 持久状态：在 run_app 创建一次，mount 渲染与重渲染闭包共享同一组信号。
#[derive(Clone)]
pub struct PocState {
    pub items: ReactiveSignal<Vec<String>>,
    pub status: ReactiveSignal<String>,
    pub toast_visible: ReactiveSignal<bool>,
    pub toast_message: ReactiveSignal<String>,
    pub counter: std::rc::Rc<std::cell::Cell<usize>>,
}

impl PocState {
    pub fn new() -> Self {
        Self {
            items: use_signal(|| vec!["药水 ×1".to_string(), "精灵球 ×2".to_string()]),
            status: use_signal(|| "尚未请求".to_string()),
            toast_visible: use_signal(|| false),
            toast_message: use_signal(|| String::new()),
            counter: std::rc::Rc::new(std::cell::Cell::new(0usize)),
        }
    }
}

impl Default for PocState {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for PocState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PocState").finish()
    }
}

/// scss! file 形态：编译期编译 src/styles/poc.scss 并哈希类名（CSS Modules）。
/// 宏产物为 'static 常量，OnceLock 缓存解析结果，避免每次渲染重建。
fn shell_styles() -> (&'static str, Vec<&'static str>) {
    static STYLES: std::sync::OnceLock<(&'static str, Vec<&'static str>)> =
        std::sync::OnceLock::new();
    STYLES.get_or_init(|| {
        let (css, map) = tairitsu_macros::scss! { file: "src/styles/poc.scss" };
        let keys = [
            "shell",
            "shell-header",
            "shell-icon",
            "shell-actions",
            "shell-list",
            "shell-row",
            "shell-status",
        ];
        let resolved: Vec<&'static str> = keys
            .iter()
            .map(|k| map.get(*k).copied().unwrap_or(k))
            .collect();
        (css, resolved)
    })
    .clone()
}

#[component]
pub fn App(state: PocState) -> VNode {
    // 二分层：true = 走 hikari ThemeProvider 完整版；false = 原生元素精简版。
    const FULL_UI: bool = true;
    if FULL_UI {
        // 品牌：游戏蓝主色 + 橙次色（对齐现有 $color-primary/$color-secondary）。
        rsx! {
            ThemeProvider {
                palette: "hikari".to_string(),
                primary: Some("#5cb8e6".to_string()),
                secondary: Some("#f5a623".to_string()),
                PocPanel { state: Some(state.clone()) }
            }
        }
    } else {
        // 二分：单信号 + 显式 VNode::Text（完全复刻 MiniCounter 形态）。
        let items_c = state.items.clone();
        let count = state.counter.clone();
        rsx! {
            div {
                id: "poc-plain",
                span {
                    ..vec![tairitsu_vdom::VNode::Text(tairitsu_vdom::VText {
                        text: format!("items={} clicks={}", items_c.get().len(), count.get()),
                    })]
                },
                button {
                    onclick: move |_| {
                        let n = count.get() + 1;
                        count.set(n);
                    },
                    "clicks++"
                }
                button {
                    onclick: move |_| {
                        let mut cur = items_c.get();
                        cur.push(format!("POC 物品 #{}", cur.len() + 1));
                        items_c.set(cur);
                    },
                    "添加一项"
                }
                ul {
                    ..state.items.get().iter().map(|item| {
                        rsx! {
                            li {
                                ..vec![tairitsu_vdom::VNode::Text(tairitsu_vdom::VText {
                                    text: item.clone(),
                                })]
                            }
                        }
                    }).collect()
                }
            }
        }
    }
}

/// 二分诊断用最小计数器：无 hikari、无 style 元素、最小树。
/// 信号由 run_app 在 with_component 上下文创建后经 props 传入。
#[component]
pub fn MiniCounter(count: ReactiveSignal<i32>, renders: usize) -> VNode {
    let count2 = count.clone();
    // 诊断信息直接进 DOM（wasm 的 eprintln 在 jco 下会被截断）：
    //   count  —— 信号值，变了说明 set 成功
    //   cid    —— 信号绑定的组件 id
    //   render —— 渲染闭包被调用次数，不涨说明 mark_dirty/flush 没把渲染跑起来
    let label = format!("count={} cid={} render={}", count.get(), count.component_id(), renders);
    rsx! {
        div {
            id: "mini",
            span {
                ..vec![tairitsu_vdom::VNode::Text(tairitsu_vdom::VText {
                    text: label,
                })]
            },
            button {
                onclick: move |_| {
                    let c = count2.get();
                    count2.set(c + 1);
                },
                "inc"
            }
        }
    }
}

#[component]
pub fn PocPanel(state: PocState) -> VNode {
    let (css, classes) = shell_styles();
    let shell = classes[0].to_string();
    let header = classes[1].to_string();
    let icon = classes[2].to_string();
    let actions = classes[3].to_string();
    let list = classes[4].to_string();
    let row = classes[5].to_string();
    let status = classes[6].to_string();
    // tairitsu rsx 文本子节点仅支持 "{ident}" 整串占位符简写，格式化放组件体内。
    let statusline = format!("fetch 状态：{}", state.status.get());

    let add_item = {
        let state = state.clone();
        tairitsu_vdom::Callback::new(move |_| {
            let n = state.counter.get() + 1;
            state.counter.set(n);
            let mut cur = state.items.get();
            cur.push(format!("POC 物品 #{n}"));
            state.items.set(cur);
        })
    };

    let start_fetch = {
        let state = state.clone();
        tairitsu_vdom::Callback::new(move |_| {
            state.status.set("请求中…".to_string());
            // 异步回调桥：fetch_promise + PROMISE_CALLBACKS，回调里 set 信号驱动重渲染。
            #[cfg(target_family = "wasm")]
            {
                let state = state.clone();
                tairitsu_web::fetch_text(
                    "/source/plugin/pokemon/wasm2/poc.json",
                    Box::new(move |result| match result {
                        Ok(body) => {
                            state.status.set(format!("OK（{} 字节）", body.len()));
                            state.toast_message.set("poc.json 加载成功".to_string());
                            state.toast_visible.set(true);
                        }
                        Err(e) => {
                            state.status.set(format!("失败：{e}"));
                        }
                    }),
                );
            }
            #[cfg(not(target_family = "wasm"))]
            state.status.set("native 环境不发起请求".to_string());
        })
    };

    let dismiss_toast = {
        let state = state.clone();
        tairitsu_vdom::Callback::new(move |_| state.toast_visible.set(false))
    };

    rsx! {
        style { dangerous_inner_html: css }
        div {
            class: shell,
            div {
                class: header,
                svg {
                    class: icon,
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    dangerous_inner_html: ICON_COINS,
                }
                span { "POC：tairitsu + hikari" }
            }
            div {
                class: actions,
                Button {
                    variant: ButtonVariant::Primary,
                    onclick: Some(add_item),
                    "添加一项"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: Some(start_fetch),
                    "fetch poc.json"
                }
            }
            div {
                class: list,
                for item in state.items.get() {
                    {
                        let row = row.clone();
                        rsx! {
                            div {
                                class: row,
                                span { "{item}" }
                            }
                        }
                    }
                }
            }
            div {
                class: status,
                "{statusline}"
            }
            if state.toast_visible.get() {
                Toast {
                    variant: ToastVariant::Success,
                    message: state.toast_message.get().clone(),
                    position: ToastPosition::BottomRight,
                    on_close: Some(dismiss_toast),
                }
            }
        }
    }
}
