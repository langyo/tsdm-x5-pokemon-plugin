//! 页脚 + 关于本项目弹窗（tairitsu 版）。
//!
//! 结构、类名、文案与 Dioxus 版 `components/layout/footer.rs` 保持一致：
//! 样式表（.app-footer / .about-*）两版共用同一份 game.css。
//! 其中 AI 模型披露是 #86 引入的项目约定，迁移时不应丢失。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::AppState;

const REPO_URL: &str = "https://github.com/langyo/tsdm-x5-pokemon-plugin";

/// 参与本项目开发的 AI 模型披露清单，按发布时间降序排列。
/// (模型名，提供方，发布日期)
const AI_MODELS: &[(&str, &str, &str)] = &[
    ("GLM 5.4", "智谱 AI", "2026-11-20"),
    ("Claude Opus 5.5", "Anthropic", "2026-09-22"),
    ("DeepSeek V4.1 Flash", "深度求索", "2026-09-10"),
    ("Claude Fable 5.1", "Anthropic", "2026-09-01"),
    ("GLM 5.3 Flash", "智谱 AI", "2026-08-27"),
    ("GLM 5.3", "智谱 AI", "2026-08-14"),
    ("DeepSeek V4", "深度求索", "2026-04-24"),
    ("Claude Opus 4.7", "Anthropic", "2026-04-16"),
    ("Qwen 3.5 Plus", "阿里巴巴", "2026-02-16"),
    ("GLM 5", "智谱 AI", "2026-02-12"),
    ("MiniMax M2.5", "MiniMax", "2026-02-12"),
    ("Kimi K2.5", "月之暗面", "2026-01-27"),
];

/// 技术栈标签。
const TECH_TAGS: &[&str] = &["Discuz X5", "Rust", "tairitsu", "WebAssembly"];

const INFO_ICON: &str = r#"<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z" />"#;
const EXTERNAL_ICON: &str = r#"<path d="M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z" />"#;

/// 页脚：版权 + “关于本项目”入口。单根节点，便于外壳内联放置。
#[component]
pub fn Footer(state: Box<AppState>) -> VNode {
    let open_state = state.clone();
    // 图标是纯装饰，dangerous_inner_html 只放本文件内的常量路径。
    rsx! {
        footer { class: "app-footer",
            div { class: "footer-inner",
                span { class: "footer-copyright", "© TSDM 天使动漫 宝可梦插件 V3" }
                button {
                    class: "footer-about-btn",
                    r#type: "button",
                    title: "关于本项目",
                    "aria-label": "关于本项目",
                    onclick: move |_| open_state.show_about.set(true),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 24 24",
                        width: "18",
                        height: "18",
                        fill: "currentColor",
                        dangerous_inner_html: INFO_ICON,
                    }
                }
            }
        }
    }
}

/// 关于本项目弹窗：信息展示 + AI 模型披露（只读，无表单）。
pub fn about_modal(state: &AppState) -> VNode {
    let tech_tags: Vec<VNode> = TECH_TAGS
        .iter()
        .map(|tag| rsx! { span { class: "about-tech-tag", "{*tag}" } })
        .collect();
    let model_items: Vec<VNode> = AI_MODELS
        .iter()
        .map(|(name, vendor, released)| {
            rsx! {
                li { class: "about-model-item",
                    span { class: "about-model-name", "{*name}" }
                    span { class: "about-model-vendor", "{*vendor}" }
                    span { class: "about-model-date", "{*released}" }
                }
            }
        })
        .collect();
    let close_state = state.clone();
    let on_close: std::rc::Rc<dyn Fn()> =
        std::rc::Rc::new(move || close_state.show_about.set(false));
    let dialog = rsx! {
                        div { class: "about-dialog",
                        div { class: "about-hero",
                            img {
                                class: "about-hero-logo",
                                src: "source/plugin/pokemon/images/item/jlq.gif",
                                alt: "宝可梦插件",
                            }
                            div { class: "about-hero-name", "天使动漫 · 宝可梦插件" }
                            span { class: "about-hero-version", "tairitsu" }
                            p { class: "about-hero-desc",
                                "天使动漫论坛的宝可梦养成游戏插件，基于 Discuz X5 构建，前端由 Rust + tairitsu 编译为 WebAssembly 运行。"
                            }
                            div { class: "about-tech",
                                for i in 0..tech_tags.len() {
                                    ..vec![tech_tags[i].clone()]
                                }
                            }
                        }
                        div { class: "about-meta",
                            span { class: "about-meta-item",
                                span { class: "about-meta-label", "作者" }
                                span { class: "about-meta-value", "langyo" }
                            }
                            span { class: "about-meta-dot", "·" }
                            a {
                                class: "about-repo-link",
                                href: REPO_URL,
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "langyo/tsdm-x5-pokemon-plugin"
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    "viewBox": "0 0 24 24",
                                    width: "12",
                                    height: "12",
                                    fill: "currentColor",
                                    dangerous_inner_html: EXTERNAL_ICON,
                                }
                            }
                        }
                        div { class: "about-models",
                            div { class: "about-section-title", "AI 模型披露" }
                            p { class: "about-models-desc",
                                "本项目的开发过程借助了以下 AI 模型，按发布时间降序排列："
                            }
                            ul { class: "about-model-list",
                                for i in 0..model_items.len() {
                                    ..vec![model_items[i].clone()]
                                }
                            }
                            p { class: "about-models-footnote", "谨向以上模型及其背后的团队致谢。" }
                        }
                    }
    };
    crate::modal::modal("关于本项目", true, on_close, vec![dialog])
}
