use dioxus::prelude::*;

use crate::{components::layout::IMG_PATH, utils::api_client::NewApiClient};

#[component]
pub fn Welcome() -> Element {
    let mut initializing = use_signal(|| false);
    let mut error_message = use_signal(|| None::<String>);
    let mut is_success = use_signal(|| false);

    let handle_initialize = move |_| {
        spawn(async move {
            initializing.set(true);
            error_message.set(None);

            let api = NewApiClient::new();
            match api.initialize_player().await {
                Ok(response) => {
                    if response.success {
                        // 初始化成功，显示成功消息然后刷新
                        is_success.set(true);

                        // 延迟 1.5 秒后刷新页面，让用户看到成功消息
                        spawn(async move {
                            // 使用 wasm-bindgen 的 setTimeout
                            let promise = js_sys::Promise::new(&mut |resolve, _| {
                                let window = web_sys::window().expect("failed to get window");
                                let _ = window
                                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                                        &resolve, 1500,
                                    );
                            });

                            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

                            let window = web_sys::window().expect("no global `window` exists");
                            let _ = window.location().reload();
                        });
                    } else {
                        error_message.set(Some(response.message));
                        initializing.set(false);
                    }
                }
                Err(e) => {
                    error_message.set(Some(format!("初始化失败: {}", e)));
                    initializing.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "welcome-page",
            div { class: "welcome-container",
                div { class: "welcome-header",
                    img {
                        class: "welcome-logo",
                        src: "{IMG_PATH}/item/jlq.gif",
                        alt: "宝可梦",
                    }
                    h1 { "欢迎来到宝可梦世界" }
                }

                div { class: "welcome-content",
                    if is_success() {
                        // 成功消息
                        div { class: "success-message",
                            h2 { "欢迎加入！" }
                            p { "账户创建成功！正在进入游戏..." }
                        }
                    } else {
                        // 初始化表单（参考旧 PHP 模板文案）
                        p { class: "welcome-message", "欢迎来到天使动漫的宝可梦世界" }
                        p { class: "welcome-description",
                            "您可以带着您的宝可梦到地图的任何地方旅行，开始精彩的冒险旅程！"
                        }

                        ul { class: "welcome-benefits",
                            li { "🥚 获得一个宝可梦作为起始伙伴" }
                            li { "🎮 解锁所有游戏功能" }
                            li { "🌟 探索广阔的宝可梦世界" }
                        }

                        if let Some(error) = error_message.read().as_ref() {
                            div { class: "error-message", "❌ {error}" }
                        }

                        button {
                            class: "welcome-button",
                            disabled: initializing(),
                            onclick: handle_initialize,
                            if initializing() {
                                "初始化中..."
                            } else {
                                "开始冒险"
                            }
                        }
                    }
                }

                if !is_success() {
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
