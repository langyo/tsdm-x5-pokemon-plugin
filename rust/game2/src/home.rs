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
    if state.badge_hidden.get().is_none() {
        load_badge_status(state);
    }
}

/// 查询帖子徽章当前是否隐藏（对齐 Dioxus 版侧栏的 badge_status）。
fn load_badge_status(state: &AppState) {
    let state = state.clone();
    crate::api::badge_status(Box::new(move |result| {
        state
            .badge_hidden
            .inner()
            .set(Some(result.map(|s| s.hidden)));
        crate::request_remount();
    }));
}

/// 刷新 / 隐藏帖子徽章。`hide = Some(true)` 隐藏，`None` 或 `Some(false)` 刷新显示。
pub fn apply_badge(state: &AppState, hide: Option<bool>) {
    if state.badge_busy.get() {
        return;
    }
    state.badge_busy.set(true);
    let state_result = state.clone();
    let state_done = state.clone();
    let target = hide.unwrap_or(false);
    crate::api::refresh_forum_badge(
        target,
        Box::new(move |result| {
            match result {
                Ok(status) => state_result
                    .badge_hidden
                    .inner()
                    .set(Some(Ok(status.hidden))),
                Err(e) => state_result.badge_hidden.inner().set(Some(Err(e))),
            }
            state_done.badge_busy.set(false);
            crate::request_remount();
        }),
    );
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
        state
            .online
            .inner()
            .set(Some(result.map(|r| (r.total, r.players))));
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
    // 外层 .online-players 是旧版就有的包裹层（text-align:center + strong 大号
    // 蓝字），移植时漏掉会让人数不居中、字号也不对。头像区 240px 固定高度是
    // 为了容纳 4 行滚动，但没人时不该留一大片空白。
    let online_body = match &online {
        None => wrap_online(count_p("加载中...", false)),
        Some(Err(_)) => wrap_online(count_p("加载失败", true)),
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
                div { class: "online-players",
                    p { class: "player-count",
                        "当前在线 "
                        strong { "{total_str}" }
                        " 人"
                    }
                    // 无人时不给空容器撑 240px 高度
                    if !avatars.is_empty() {
                        div { class: "player-avatars",
                            for i in 0..avatars.len() {
                                ..vec![avatars[i].clone()]
                            }
                        }
                    }
                }
            }
        }
    };

    // 帖子徽章控制区：对齐 Dioxus 版侧栏的「刷新徽章 / 隐藏宠物」两个动作。
    let busy = state.badge_busy.get();
    let (badge_state_text, toggle_label, toggle_hide) = match &state.badge_hidden.get() {
        None => (
            "徽章状态加载中...".to_string(),
            "隐藏宠物".to_string(),
            true,
        ),
        Some(Err(e)) => (
            format!("徽章状态加载失败：{e}"),
            "隐藏宠物".to_string(),
            true,
        ),
        Some(Ok(true)) => ("当前：已隐藏".to_string(), "显示宠物".to_string(), false),
        Some(Ok(false)) => ("当前：显示中".to_string(), "隐藏宠物".to_string(), true),
    };
    let toggle_attr = if toggle_hide { "1" } else { "0" };
    let refresh_cls = if busy {
        "admin-btn badge-refresh-btn disabled"
    } else {
        "admin-btn badge-refresh-btn"
    };
    let toggle_cls = if busy {
        "admin-btn badge-toggle-btn disabled"
    } else {
        "admin-btn badge-toggle-btn"
    };
    let badge_body = rsx! {
        div { class: "online-players",
            p { class: "player-count", "{badge_state_text}" }
            div { class: "badge-actions",
                button {
                    class: refresh_cls,
                    r#type: "button",
                    disabled: busy,
                    "data-pm-badge-refresh": "1",
                    if busy { "刷新中..." } else { "刷新徽章" }
                }
                button {
                    class: toggle_cls,
                    r#type: "button",
                    disabled: busy,
                    "data-pm-badge-toggle": toggle_attr,
                    "{toggle_label}"
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
                div { class: "portal-card",
                    div { class: "portal-card-header",
                        img { class: "header-icon-img", src: "{IMG_PATH}/item/jlq.gif" }
                        span { "帖子徽章" }
                    }
                    div { class: "portal-card-body", ..vec![badge_body] }
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
    let cls = if is_error {
        "player-count error"
    } else {
        "player-count"
    };
    rsx! { p { class: cls, "{t}" } }
}

/// 包上旧版的 .online-players 容器（居中对齐 + 人数大字样式都挂在它下面）。
fn wrap_online(inner: VNode) -> VNode {
    rsx! {
        div { class: "online-players",
            ..vec![inner]
        }
    }
}
