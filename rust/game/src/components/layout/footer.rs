use dioxus::prelude::*;

use crate::components::common::Modal;

const REPO_URL: &str = "https://github.com/langyo/tsdm-x5-pokemon-plugin";

/// 参与本项目开发的 AI 模型披露清单，按发布时间降序排列。
/// (模型名，提供方，发布日期)
const AI_MODELS: &[(&str, &str, &str)] = &[
    ("DeepSeek V4.1 Flash", "深度求索", "2026-09-10"),
    ("GLM 5.3 Flash", "智谱 AI", "2026-08-27"),
    ("GLM 5.3", "智谱 AI", "2026-08-14"),
    ("DeepSeek V4", "深度求索", "2026-04-24"),
    ("Claude Opus 4.7", "Anthropic", "2026-04-16"),
    ("Qwen 3.5 Plus", "阿里巴巴", "2026-02-16"),
    ("GLM 5", "智谱 AI", "2026-02-12"),
    ("MiniMax M2.5", "MiniMax", "2026-02-12"),
    ("Kimi K2.5", "月之暗面", "2026-01-27"),
];

fn info_icon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z" }
        }
    }
}

fn external_link_icon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z" }
        }
    }
}

#[component]
pub fn Footer() -> Element {
    let mut show_about = use_signal(|| false);

    rsx! {
        footer { class: "app-footer",
            div { class: "footer-inner",
                span { class: "footer-copyright", "© TSDM 天使动漫 宝可梦插件 V3" }
                button {
                    class: "footer-about-btn",
                    r#type: "button",
                    title: "关于本项目",
                    aria_label: "关于本项目",
                    onclick: move |_| show_about.set(true),
                    {info_icon()}
                }
            }
        }

        if show_about() {
            AboutModal { on_close: move |_| show_about.set(false) }
        }
    }
}

#[component]
fn AboutModal(on_close: EventHandler<()>) -> Element {
    rsx! {
        Modal {
            is_open: true,
            on_close,
            title: "关于本项目".to_string(),
            div { class: "about-dialog",
                div { class: "about-project",
                    div { class: "about-project-name", "天使动漫 · 宝可梦插件" }
                    p { class: "about-project-desc",
                        "天使动漫论坛的宝可梦养成游戏插件，基于 Discuz X5 构建，前端由 Rust + Dioxus 编译为 WebAssembly 运行。"
                    }
                    div { class: "about-meta",
                        div { class: "about-meta-row",
                            span { class: "about-meta-label", "作者" }
                            span { class: "about-meta-value", "langyo" }
                        }
                        div { class: "about-meta-row",
                            span { class: "about-meta-label", "仓库" }
                            a {
                                class: "about-repo-link",
                                href: REPO_URL,
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "langyo/tsdm-x5-pokemon-plugin"
                                {external_link_icon()}
                            }
                        }
                    }
                }

                div { class: "about-models",
                    div { class: "about-section-title", "AI 模型披露" }
                    p { class: "about-models-desc",
                        "本项目的开发过程中借助了以下 AI 模型，按发布时间降序排列："
                    }
                    ul { class: "about-model-list",
                        for (name, vendor, released) in AI_MODELS {
                            li { class: "about-model-item",
                                span { class: "about-model-name", "{name}" }
                                span { class: "about-model-vendor", "{vendor}" }
                                span { class: "about-model-date", "{released}" }
                            }
                        }
                    }
                    p { class: "about-models-footnote", "谨向以上模型及其背后的团队致谢。" }
                }
            }
        }
    }
}
