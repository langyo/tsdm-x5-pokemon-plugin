use dioxus::prelude::*;

use crate::{components::layout::IMG_PATH, utils::api_client::NewApiClient};
use _utils::types::api_config::NewsAnnouncement;

/// 话题/新闻项（统一显示格式）
#[derive(Clone, Debug, PartialEq)]
enum DisplayItem {
    News(NewsAnnouncement),
    Topic {
        id: u64,
        title: String,
        is_pinned: bool,
    },
}

#[component]
pub fn Home() -> Element {
    // 加载话题数据（包含新闻公告）
    let topics_data = use_resource(|| async move {
        let api = NewApiClient::new();
        api.get_topics(Some(6)).await
    });

    // 加载在线玩家数据
    let online_data = use_resource(|| async move {
        let api = NewApiClient::new();
        api.get_online_players().await
    });

    // 使用 use_memo 响应式计算显示列表
    let display_items = use_memo(move || {
        let mut items = vec![];

        // 先添加新闻公告（如果有）
        if let Some(Ok(data)) = topics_data.read().as_ref() {
            for news in &data.news_announcements {
                if !news.title.is_empty() {
                    items.push(DisplayItem::News(news.clone()));
                }
            }

            // 再添加话题，直到达到 6 条
            let remaining = 6usize.saturating_sub(items.len());
            for topic in data.topics.iter().take(remaining) {
                items.push(DisplayItem::Topic {
                    id: topic.id,
                    title: topic.title.clone(),
                    is_pinned: topic.is_pinned,
                });
            }
        }

        items
    });

    rsx! {
        div { class: "page-home",
            // ===== 主内容区（左侧）- 最新话题 =====
            div { class: "home-main",
                div { class: "portal-card",
                    div { class: "portal-card-header",
                        img {
                            class: "header-icon-img",
                            src: "{IMG_PATH}/item/jlq.gif",
                            alt: "",
                        }
                        span { "最新话题" }
                    }

                    div { class: "portal-card-body",
                        // 渲染合并后的列表
                        if display_items().is_empty() {
                            match topics_data.read().as_ref() {
                                Some(Err(_)) => rsx! {
                                    p { class: "error", "话题加载失败" }
                                },
                                None => rsx! {
                                    p { "加载中..." }
                                },
                                _ => rsx! {
                                    p { "暂无内容" }
                                }
                            }
                        } else {
                            ul { class: "topic-list",
                                for (idx , item) in display_items().iter().enumerate() {
                                    li { key: "item-{idx}", class: "topic-item",
                                        match item {
                                            DisplayItem::News(news) => rsx! {
                                                // 新闻公告：使用金色图标
                                                img {
                                                    class: "topic-icon-img",
                                                    src: "{IMG_PATH}/item/gjq.gif",
                                                    alt: "",
                                                }
                                                if !news.url.is_empty() {
                                                    a {
                                                        class: "topic-link pinned",
                                                        href: "{news.url}",
                                                        target: "_blank",
                                                        "{news.title}"
                                                    }
                                                } else {
                                                    span { class: "topic-link pinned", "{news.title}" }
                                                }
                                            },
                                            DisplayItem::Topic { id, title, is_pinned } => rsx! {
                                                img {
                                                    class: "topic-icon-img",
                                                    src: if *is_pinned { "{IMG_PATH}/item/gjq.gif" } else { "{IMG_PATH}/item/jlq.gif" },
                                                    alt: "",
                                                }
                                                a {
                                                    class: if *is_pinned { "topic-link pinned" } else { "topic-link" },
                                                    href: "forum.php?mod=viewthread&tid={id}",
                                                    target: "_blank",
                                                    "{title}"
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ===== 侧边栏（右侧）- 在线玩家 =====
            div { class: "home-sidebar",
                div { class: "portal-card",
                    div { class: "portal-card-header",
                        img {
                            class: "header-icon-img",
                            src: "{IMG_PATH}/item/jlq.gif",
                            alt: "",
                        }
                        span { "在线玩家" }
                    }

                    div { class: "portal-card-body",
                        div { class: "online-players",
                            // 显示在线人数
                            match online_data.read().as_ref() {
                                Some(Ok(data)) => rsx! {
                                    p { class: "player-count",
                                        "当前在线 "
                                        strong { "{data.total}" }
                                        " 人"
                                    }

                                    // 显示玩家头像（固定高度，滚动显示所有）
                                    div { class: "player-avatars",
                                        // 显示所有真实玩家头像
                                        for (idx , player) in data.players.iter().enumerate() {
                                            a {
                                                key: "{idx}",
                                                class: "player-avatar",
                                                href: "home.php?mod=space&uid={player.uid}",
                                                title: "{player.username}",
                                                target: "_blank",
                                                img {
                                                    src: "plugin.php?id=pokemon:pokemon&endpoint=avatar&uid={player.uid}&size=small",
                                                    alt: "{player.username}",
                                                }
                                            }
                                        }
                                    }
                                },
                                Some(Err(_)) => rsx! {
                                    p { class: "player-count error", "加载失败" }
                                },
                                None => rsx! {
                                    p { class: "player-count", "加载中..." }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
