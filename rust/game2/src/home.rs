//! 首页（tairitsu 版，1:1 移植自 rust/game/src/pages/home.rs）。
//!
//! 左侧最新话题（新闻公告+论坛话题合并，各带图标/置顶态），右侧在线
//! 玩家（总数+头像墙）。同名同类直用 game.css。数据由 Home 页装载。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use crate::app_state::{AppState, TopicItem};

const IMG_PATH: &str = "source/plugin/pokemon/images";

/// 首次进入时装载话题与在线玩家（对齐 Dioxus 版 use_resource 首次求值）。
pub fn ensure_loaded(state: &AppState) {
    if state.topics.get().is_none() {
        load_topics(state);
    }
    if state.online.get().is_none() {
        load_online(state);
    }
}

fn load_topics(state: &AppState) {
    let state = state.clone();
    crate::api::topics(Box::new(move |result| {
        state.topics.inner().set(Some(result.map(|r| r.items)));
        crate::request_remount();
    }));
}

fn load_online(state: &AppState) {
    let state = state.clone();
    crate::api::online_players(Box::new(move |result| {
        state.online.inner().set(Some(result.map(|r| (r.total, r.players))));
        crate::request_remount();
    }));
}

#[component]
pub fn Home(state: Box<AppState>) -> VNode {
    let topics = state.topics.get();
    let online = state.online.get();

    // 话题列表体
    let list_body = match &topics {
        None => plain_p("加载中..."),
        Some(Err(_)) => plain_class("error", "话题加载失败"),
        Some(Ok(items)) if items.is_empty() => plain_p("暂无内容"),
        Some(Ok(items)) => {
            let rows: Vec<VNode> = items
                .iter()
                .map(|item| {
                    match item {
                        TopicItem::News { title, url } => {
                            let href = url.clone();
                            let has_url = !href.is_empty();
                            let icon_src = format!("{IMG_PATH}/item/gjq.gif");
                            rsx! {
                                img { class: "topic-icon-img", src: icon_src }
                                if has_url {
                                    a { class: "topic-link pinned", href: href, target: "_blank", "{title}" }
                                } else {
                                    span { class: "topic-link pinned", "{title}" }
                                }
                            }
                        }
                        TopicItem::Topic { id, title, is_pinned } => {
                            let href = format!("forum.php?mod=viewthread&tid={id}");
                            let icon = if *is_pinned { "gjq" } else { "jlq" };
                            let icon_src = format!("{IMG_PATH}/item/{icon}.gif");
                            let cls = if *is_pinned { "topic-link pinned" } else { "topic-link" };
                            rsx! {
                                img { class: "topic-icon-img", src: icon_src }
                                a { class: cls, href: href, target: "_blank", "{title}" }
                            }
                        }
                    }
                })
                .collect();
            rsx! {
                ul { class: "topic-list",
                    for i in 0..rows.len() {
                        li { class: "topic-item", ..vec![rows[i].clone()] }
                    }
                }
            }
        }
    };

    // 在线玩家体
    let online_body = match &online {
        None => count_p("加载中...", false),
        Some(Err(_)) => count_p("加载失败", true),
        Some(Ok((total, players))) => {
            let total_str = total.to_string();
            let avatars: Vec<VNode> = players
                .iter()
                .map(|p| {
                    let href = format!("home.php?mod=space&uid={}", p.uid);
                    let src = format!(
                        "plugin.php?id=pokemon:pokemon&endpoint=avatar&uid={}&size=small",
                        p.uid
                    );
                    let name = p.username.clone();
                    rsx! {
                        a { class: "player-avatar", href: href, title: name,
                            img { src: src }
                        }
                    }
                })
                .collect();
            rsx! {
                p { class: "player-count",
                    "当前在线 "
                    strong { "{total_str}" }
                    " 人"
                }
                div { class: "player-avatars",
                    for i in 0..avatars.len() {
                        ..vec![avatars[i].clone()]
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "page-home",
            div { class: "home-main",
                div { class: "portal-card",
                    div { class: "portal-card-header",
                        img { class: "header-icon-img", src: "{IMG_PATH}/item/jlq.gif" }
                        span { "最新话题" }
                    }
                    div { class: "portal-card-body", ..vec![list_body] }
                }
            }
            div { class: "home-sidebar",
                div { class: "portal-card",
                    div { class: "portal-card-header",
                        img { class: "header-icon-img", src: "{IMG_PATH}/item/jlq.gif" }
                        span { "在线玩家" }
                    }
                    div { class: "portal-card-body", ..vec![online_body] }
                }
            }
        }
    }
}

fn plain_p(text: &str) -> VNode {
    let t = text.to_string();
    rsx! { p { "{t}" } }
}

fn plain_class(class: &str, text: &str) -> VNode {
    let t = text.to_string();
    rsx! { p { class: class, "{t}" } }
}

fn count_p(text: &str, is_error: bool) -> VNode {
    let t = text.to_string();
    let cls = if is_error { "player-count error" } else { "player-count" };
    rsx! { p { class: cls, "{t}" } }
}
