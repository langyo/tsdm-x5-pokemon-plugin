//! 后台页面：壳（侧边导航 + 忙指示 + Toast）与全局配置页。

use tairitsu_macros::{component, rsx};
use tairitsu_vdom::VNode;

use hikari_components::{Toast, ToastPosition, ToastProps, ToastVariant};

use crate::app_state::AppState;
use crate::state::{AdminRoute, ToastKind};

/// 首次进入装载当前页数据。
pub fn ensure_loaded(state: &AppState) {
    match state.route.get() {
        AdminRoute::GlobalConfig => {
            if state.config.loaded.get().is_none() {
                reload_config(state);
            }
        }
        _ => {}
    }
}

fn reload_config(state: &AppState) {
    let cfg = state.config.clone();
    crate::api::get_global_config(Box::new(move |result| {
        // 回调跑在 promise 线程：信号只写内部值，重挂必须在这里请求——
        // 发请求后同步请求重挂是没有意义的（数据还没回来）。
        match result {
            Ok(config) => {
                let pretty = serde_json::to_string_pretty(&config).unwrap_or_default();
                cfg.saved_json.inner().set(pretty.clone());
                cfg.draft.inner().set(pretty);
                cfg.loaded.inner().set(Some(Ok(())));
            }
            Err(e) => {
                cfg.loaded.inner().set(Some(Err(e)));
            }
        }
        crate::request_remount();
    }));
}

/// 保存：本地先校验 JSON 合法性。
pub fn save_config(state: &AppState) {
    let cfg = state.config.clone();
    if cfg.saving.get() {
        return;
    }
    let draft = cfg.draft.get();
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&draft);
    let Ok(value) = parsed else {
        let err = parsed.err().map(|e| e.to_string()).unwrap_or_default();
        crate::state::show_toast_silent(&cfg, ToastKind::Error, format!("JSON 无效：{err}"));
        crate::request_remount();
        return;
    };
    let serialized = value.to_string();
    cfg.saving.inner().set(true);
    crate::request_remount();
    let st = state.clone();
    crate::api::set_global_config(
        serialized,
        Box::new(move |result| {
            match result {
                Ok(saved) => {
                    let pretty = serde_json::to_string_pretty(&saved).unwrap_or_default();
                    let cfg = st.config.clone();
                    cfg.saved_json.inner().set(pretty.clone());
                    cfg.draft.inner().set(pretty);
                    crate::state::show_toast_silent(&cfg, ToastKind::Success, "配置已保存".to_string());
                }
                Err(e) => {
                    crate::state::show_toast_silent(
                        &st.config,
                        ToastKind::Error,
                        format!("保存失败：{e}"),
                    );
                }
            }
            // promise 线程收尾：复位守卫并请求重挂（成功/失败都到这里）。
            st.config.saving.inner().set(false);
            crate::request_remount();
        }),
    );
}

pub fn reset_config(state: &AppState) {
    let cfg = state.config.clone();
    let saved = cfg.saved_json.get();
    cfg.draft.inner().set(saved);
    crate::request_remount();
}

pub fn switch_route(state: &AppState, route: AdminRoute) {
    if state.route.get() == route {
        return;
    }
    state.route.inner().set(route);
    crate::request_remount();
    ensure_loaded(state);
}

// ---------------------------------------------------------------------------
// 渲染
// ---------------------------------------------------------------------------

pub fn render_route(state: &AppState) -> VNode {
    ensure_loaded(state);
    Admin(AdminProps {
        state: Some(Box::new(state.clone())),
        ..Default::default()
    })
}

#[component]
pub fn Admin(state: Box<AppState>) -> VNode {
    let st = state;
    let route = st.route.get();
    let busy = st.config.saving.get();
    let dirty = st.config.is_dirty();

    let nav_items: Vec<(AdminRoute, VNode)> = [
        AdminRoute::GlobalConfig,
        AdminRoute::PokemonData,
        AdminRoute::ItemData,
        AdminRoute::MapData,
        AdminRoute::UserData,
        AdminRoute::EvolutionData,
        AdminRoute::SkillType,
        AdminRoute::EffectData,
    ]
    .iter()
    .map(|r| {
        let key = r.key();
        let title = r.title();
        let ported = r.ported();
        let cls = if *r == route {
            "admin-nav-item active"
        } else {
            "admin-nav-item"
        };
        let badge = if ported {
            VNode::empty()
        } else {
            rsx! { span { class: "admin-nav-badge", "移植中" } }
        };
        (*r, rsx! {
            button {
                class: cls,
                "data-pm-adm-nav": key,
                span { "{title}" }
                ..vec![badge]
            }
        })
    })
    .collect();

    let content = match route {
        AdminRoute::GlobalConfig => global_config_page(&st),
        _ => {
            let title = route.title();
            rsx! {
                section { class: "admin-page",
                    div { class: "admin-page-header", h2 { "{title}" } }
                    div { class: "admin-card admin-placeholder",
                        p { "该页面尚未移植到 tairitsu 版后台，正在按切片推进。" }
                        p { class: "admin-placeholder-hint", "可切回原后台操作：plugin.php?id=pokemon:game&index=admin" }
                    }
                }
            }
        }
    };

    let toast = if st.config.toast_visible.get() {
        let cfg = st.config.clone();
        let variant = match st.config.toast_kind.get() {
            ToastKind::Success => ToastVariant::Success,
            ToastKind::Error => ToastVariant::Error,
        };
        let message = st.config.toast_message.get();
        rsx! {
            Toast {
                variant: variant,
                message: message,
                position: ToastPosition::BottomRight,
                on_close: Some(tairitsu_vdom::Callback::new(move |_| {
                    cfg.toast_visible.set(false)
                })),
            }
        }
    } else {
        VNode::empty()
    };

    let busy_hint = if busy {
        rsx! { span { class: "admin-busy", "处理中..." } }
    } else {
        VNode::empty()
    };
    let dirty_hint = if dirty {
        rsx! { span { class: "admin-dirty", "有未保存修改" } }
    } else {
        VNode::empty()
    };

    rsx! {
        div { class: "admin-layout",
            aside { class: "admin-sidebar",
                div { class: "admin-sidebar-header",
                    span { class: "admin-sidebar-title", "宠物管理后台" }
                    span { class: "admin-sidebar-sub", "tairitsu v2" }
                }
                nav { class: "admin-nav",
                    for i in 0..nav_items.len() {
                        ..vec![nav_items[i].1.clone()]
                    }
                }
            }
            main { class: "admin-main",
                header { class: "admin-topbar",
                    span { class: "admin-topbar-title", "Online Void Pokemon 管理后台" }
                    ..vec![dirty_hint, busy_hint]
                }
                ..vec![content]
            }
            ..vec![toast]
        }
    }
}

fn global_config_page(st: &AppState) -> VNode {
    let cfg = st.config.clone();
    let load_state = cfg.loaded.get();
    let draft = cfg.draft.get();
    let dirty = cfg.is_dirty();
    let saving = cfg.saving.get();

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载全局配置..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        "data-pm-adm-config-reload": "1",
                        "重新加载"
                    }
                }
            }
        }
        Some(Ok(())) => {
            let save_cls = if dirty && !saving {
                "admin-btn admin-btn-primary"
            } else {
                "admin-btn admin-btn-primary disabled"
            };
            let reset_cls = if dirty && !saving {
                "admin-btn"
            } else {
                "admin-btn disabled"
            };
            let rows = draft.lines().count().max(4).min(30);
            rsx! {
                div { class: "admin-card admin-config-editor",
                    div { class: "admin-config-toolbar",
                        span { class: "admin-config-hint",
                            "直接编辑配置 JSON（切片 1 形态；表单化随后续切片）"
                        }
                        button {
                            class: save_cls,
                            disabled: !dirty || saving,
                            "data-pm-adm-config-save": "1",
                            "保存配置"
                        }
                        button {
                            class: reset_cls,
                            disabled: !dirty || saving,
                            "data-pm-adm-config-reset": "1",
                            "重置修改"
                        }
                    }
                    textarea {
                        class: "admin-config-textarea",
                        rows: rows,
                        spellcheck: "false",
                        "data-pm-adm-config-input": "1",
                        "{draft}"
                    }
                }
            }
        }
    };

    rsx! {
        section { class: "admin-page admin-config-page",
            div { class: "admin-page-header",
                h2 { "全局配置" }
            }
            ..vec![body]
        }
    }
}
