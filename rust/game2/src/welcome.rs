//! 欢迎页（tairitsu 版，1:1 移植自 rust/game/src/pages/welcome.rs）。
//!
//! 新玩家首次进入时的初始化引导：文案/图标/按钮/成功态与 Dioxus 版
//! 同名同类（.welcome-page 系列，game.css 直用）。初始化成功后
//! 1.5s 刷新整页（对齐 Dioxus 版的 location.reload 行为）。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;

const IMG_PATH: &str = "source/plugin/pokemon/images";

/// 内联 lucide 图标路径（egg / gamepad-2 / sparkles / circle-x）。
const ICON_EGG: &str = r#"<path d="M12 22a8 8 0 0 0 8-7c0-5.5-4.5-10.5-8-11-3.5.5-8 5.5-8 11a8 8 0 0 0 8 7z"/><path d="M12 10c3.5 1 5 4 5 7"/><path d="M8.5 8.5c1 1.5 2 3 2 5"/>"#;
const ICON_GAMEPAD: &str = r#"<line x1="6" x2="10" y1="11" y2="11"/><line x1="8" x2="8" y1="9" y2="13"/><line x1="15" x2="15.01" y1="12" y2="12"/><line x1="18" x2="18.01" y1="10" y2="10"/><path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z"/>"#;
const ICON_SPARKLES: &str = r#"<path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"/><path d="M5 3v4"/><path d="M19 17v4"/><path d="M3 5h4"/><path d="M17 19h4"/>"#;
const ICON_CIRCLE_X: &str =
    r#"<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/>"#;

/// promise 线程统一收尾：内部值写入 + 请求重挂载（见 lib.rs 说明）。
fn commit(state: &AppState) {
    crate::request_remount();
    let _ = state;
}

/// 发起玩家初始化（对齐 Dioxus 版 handle_initialize）。
pub fn initialize(state: &AppState) {
    if state.welcome_initializing.get() {
        return;
    }
    state.welcome_initializing.set(true);
    state.welcome_error.set(None);
    api_initialize(state);
}

fn api_initialize(state: &AppState) {
    let state = state.clone();
    crate::api::initialize_player(Box::new(move |result| match result {
        Ok(out) => {
            if out.success {
                // 成功：显示成功态，1.5s 后整页刷新进游戏。
                state.welcome_success.inner().set(true);
                commit(&state);
                schedule_reload(1500);
            } else {
                state.welcome_error.inner().set(Some(out.message));
                state.welcome_initializing.inner().set(false);
                commit(&state);
            }
        }
        Err(e) => {
            state
                .welcome_error
                .inner()
                .set(Some(format!("初始化失败: {e}")));
            state.welcome_initializing.inner().set(false);
            commit(&state);
        }
    }));
}

/// 1.5s 后刷新整页：用 promise 桥的 setTimeout（时间到后 onTimeout
/// 回调线程里同步导入都可靠，但 location.reload 只在 JS 侧——
/// 经 set_attribute 打一个 data-reload 标记，boot.js 的观察器执行
/// location.reload()，与 pm-flush 同一模式。
fn schedule_reload(ms: i32) {
    use tairitsu_vdom::platform::TimerOps;
    let Ok(platform) = tairitsu_web::WitPlatform::new() else {
        return;
    };
    platform.set_timeout(
        Box::new(|| {
            use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::set_attribute;
            use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::non_element_parent_node::get_element_by_id;
            if let Some(app) = get_element_by_id(0, "app") {
                let _ = set_attribute(app, "data-reload", "1");
            }
        }),
        ms,
    );
}

#[component]
pub fn Welcome(state: Box<AppState>) -> VNode {
    let initializing = state.welcome_initializing.get();
    let is_success = state.welcome_success.get();
    let error = state.welcome_error.get();
    let error_text = error.unwrap_or_default();
    let has_error = !error_text.is_empty();
    // rsx 文本插值只支持整串 "{ident}"，按钮文案先取出。
    let button_text = if initializing {
        "初始化中..."
    } else {
        "开始冒险"
    }
    .to_string();
    let icon_egg = svg_lucide(ICON_EGG);
    let icon_gamepad = svg_lucide(ICON_GAMEPAD);
    let icon_sparkles = svg_lucide(ICON_SPARKLES);
    let icon_err = svg_lucide(ICON_CIRCLE_X);
    let button_class = if initializing {
        "welcome-button disabled"
    } else {
        "welcome-button"
    }
    .to_string();

    rsx! {
        div { class: "welcome-page",
            div { class: "welcome-container",
                div { class: "welcome-header",
                    img { class: "welcome-logo", src: "{IMG_PATH}/item/jlq.gif" }
                    h1 { "欢迎来到宝可梦世界" }
                }
                div { class: "welcome-content",
                    if is_success {
                        div { class: "success-message",
                            h2 { "欢迎加入！" }
                            p { "账户创建成功！正在进入游戏..." }
                        }
                    } else {
                        p { class: "welcome-message", "欢迎来到天使动漫的宝可梦世界" }
                        p { class: "welcome-description",
                            "您可以带着您的宝可梦到地图的任何地方旅行，开始精彩的冒险旅程！"
                        }
                        ul { class: "welcome-benefits",
                            li {
                                {icon_egg}
                                "获得一个宝可梦作为起始伙伴"
                            }
                            li {
                                {icon_gamepad}
                                "解锁所有游戏功能"
                            }
                            li {
                                {icon_sparkles}
                                "探索广阔的宝可梦世界"
                            }
                        }
                        if has_error {
                            div { class: "error-message",
                                {icon_err}
                                "{error_text}"
                            }
                        }
                        button {
                            class: button_class,
                            "data-pm-welcome-init": "1",
                            "{button_text}"
                        }
                    }
                }
                if !is_success {
                    div { class: "welcome-footer",
                        p { class: "welcome-hint",
                            "点击按钮后，系统将为您创建账户并赠送起始宝可梦"
                        }
                    }
                }
            }
        }
    }
}

/// 内联 lucide SVG（14px，与 Dioxus 版 Icon size=14 一致）。
fn svg_lucide(paths: &str) -> VNode {
    let class = "lucide-inline".to_string();
    rsx! {
        svg {
            class: class,
            xmlns: "http://www.w3.org/2000/svg",
            width: "14",
            height: "14",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            dangerous_inner_html: paths,
        }
    }
}
