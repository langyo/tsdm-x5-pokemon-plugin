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
        AdminRoute::PokemonData => {
            // busy 守卫与 maps 哨兵同因：loaded 要等 list 回调才落位，
            // 重挂期间反复进来会无限重复发起（活锁）。
            if state.pokemon.loaded.get().is_none() && !state.pokemon.busy.get() {
                reload_pokemon(state);
            }
        }
        _ => {}
    }
}

// ===========================================================================
// 宝可梦数据页（切片 2：列表 + 筛选 + 加载更多）
// ===========================================================================

const POKEMON_PAGE_SIZE: u64 = 100;

/// 属性 ID → 中文名（服务端载荷的 kind 是英文 snake_case ID）。
fn kind_name(id: &str) -> &'static str {
    match id {
        "normal" => "普通",
        "fire" => "火",
        "water" => "水",
        "grass" => "草",
        "electric" => "电",
        "ice" => "冰",
        "fighting" => "格斗",
        "poison" => "毒",
        "ground" => "地面",
        "flying" => "飞行",
        "psychic" => "超能",
        "bug" => "虫",
        "rock" => "岩石",
        "ghost" => "幽灵",
        "dragon" => "龙",
        "dark" => "恶",
        "steel" => "钢",
        "fairy" => "妖精",
        _ => "？",
    }
}

/// 全量重载：count + 首页（对齐 Dioxus 版 reload_pokemons）。
fn reload_pokemon(state: &AppState) {
    let pk = state.pokemon.clone();
    pk.busy.inner().set(true);
    pk.filtered.inner().set(false);
    pk.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::count_pokemon_type(Box::new(move |total| {
        let pk = st.pokemon.clone();
        match total {
            Ok(total) => {
                pk.total.inner().set(total);
                let st2 = st.clone();
                crate::api::list_pokemon_type(
                    0,
                    POKEMON_PAGE_SIZE,
                    Box::new(move |result| {
                        let pk = st2.pokemon.clone();
                        match result {
                            Ok(rows) => {
                                let n = rows.len() as u64;
                                pk.rows.inner().set(rows);
                                pk.loaded_count.inner().set(n);
                                pk.loaded.inner().set(Some(Ok(())));
                            }
                            Err(e) => {
                                pk.loaded.inner().set(Some(Err(e)));
                            }
                        }
                        pk.busy.inner().set(false);
                        crate::request_remount();
                    }),
                );
            }
            Err(e) => {
                pk.loaded.inner().set(Some(Err(e)));
                pk.busy.inner().set(false);
                crate::request_remount();
            }
        }
    }));
}

/// 加载更多：filter 态不分页（对齐 Dioxus 版）。
pub fn load_more_pokemon(state: &AppState) {
    let pk = state.pokemon.clone();
    if pk.busy.get() || pk.filtered.get() {
        return;
    }
    let from = pk.loaded_count.get();
    let total = pk.total.get();
    if from >= total {
        return;
    }
    pk.busy.inner().set(true);
    crate::request_remount();
    let st = state.clone();
    crate::api::list_pokemon_type(
        from,
        POKEMON_PAGE_SIZE,
        Box::new(move |result| {
            let pk = st.pokemon.clone();
            if let Ok(rows) = result {
                let n = rows.len() as u64;
                let mut all = pk.rows.get();
                all.extend(rows);
                pk.rows.inner().set(all);
                pk.loaded_count.inner().set(from + n);
            }
            pk.busy.inner().set(false);
            crate::request_remount();
        }),
    );
}

/// 应用筛选：任一输入非空即发 filter；全空回到全量列表。
pub fn apply_pokemon_filters(state: &AppState) {
    let pk = state.pokemon.clone();
    if pk.busy.get() {
        return;
    }
    let id = pk.f_id.get().trim().to_string();
    let name = pk.f_name.get().trim().to_string();
    let kind = pk.f_kind.get().trim().to_string();
    if id.is_empty() && name.is_empty() && kind.is_empty() {
        pk.loaded.inner().set(None);
        crate::request_remount();
        reload_pokemon(state);
        return;
    }
    let mut packages: Vec<serde_json::Value> = Vec::new();
    if !id.is_empty() {
        packages.push(crate::api::filter_package("ID", "equal", &id));
    }
    if !name.is_empty() {
        packages.push(crate::api::filter_package("名称", "contains", &name));
    }
    if !kind.is_empty() {
        packages.push(crate::api::filter_package("宠物类型", "contains", &kind));
    }
    pk.busy.inner().set(true);
    pk.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::filter_pokemon_type(
        serde_json::to_string(&packages).unwrap_or_default(),
        Box::new(move |result| {
            let pk = st.pokemon.clone();
            match result {
                Ok(rows) => {
                    let n = rows.len() as u64;
                    pk.rows.inner().set(rows);
                    pk.loaded_count.inner().set(n);
                    pk.total.inner().set(n);
                    pk.filtered.inner().set(true);
                    pk.loaded.inner().set(Some(Ok(())));
                }
                Err(e) => {
                    pk.err.inner().set(Some(e));
                }
            }
            pk.busy.inner().set(false);
            crate::request_remount();
        }),
    );
}

/// 重置筛选：清输入并回全量列表。
pub fn reset_pokemon_filters(state: &AppState) {
    let pk = state.pokemon.clone();
    pk.f_id.inner().set(String::new());
    pk.f_name.inner().set(String::new());
    pk.f_kind.inner().set(String::new());
    pk.loaded.inner().set(None);
    crate::request_remount();
    reload_pokemon(state);
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
                    crate::state::show_toast_silent(
                        &cfg,
                        ToastKind::Success,
                        "配置已保存".to_string(),
                    );
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
        (
            *r,
            rsx! {
                button {
                    class: cls,
                    "data-pm-adm-nav": key,
                    span { "{title}" }
                    ..vec![badge]
                }
            },
        )
    })
    .collect();

    let content = match route {
        AdminRoute::GlobalConfig => global_config_page(&st),
        AdminRoute::PokemonData => pokemon_data_page(&st),
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

// ---------------------------------------------------------------------------
// 宝可梦数据页
// ---------------------------------------------------------------------------

fn json_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn json_u64(v: &serde_json::Value, key: &str) -> u64 {
    v.get(key).and_then(|x| x.as_u64()).unwrap_or(0)
}

fn json_bool(v: &serde_json::Value, key: &str) -> bool {
    v.get(key).and_then(|x| x.as_bool()).unwrap_or(false)
}

fn pokemon_data_page(st: &AppState) -> VNode {
    let pk = st.pokemon.clone();
    let load_state = pk.loaded.get();
    let busy = pk.busy.get();
    let filtered = pk.filtered.get();
    let err = pk.err.get();
    let rows = pk.rows.get();
    let total = pk.total.get();
    let loaded_count = pk.loaded_count.get();
    let has_more = !filtered && loaded_count < total;
    let f_id = pk.f_id.get();
    let f_name = pk.f_name.get();
    let f_kind = pk.f_kind.get();
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载宠物数据..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-pk-reload": "1",
                        "重新加载"
                    }
                }
            }
        }
        Some(Ok(())) => {
            let err_bar = match &err {
                Some(e) => {
                    let msg = format!("筛选失败：{e}");
                    rsx! { div { class: "admin-card admin-error", p { "{msg}" } } }
                }
                None => VNode::empty(),
            };
            let table = if rows.is_empty() {
                rsx! { p { class: "admin-empty-hint", "暂无宠物数据。" } }
            } else {
                let mut trs: Vec<VNode> = Vec::new();
                for item in rows.iter() {
                    let id = json_u64(item, "id").to_string();
                    let name = json_str(item, "name");
                    // kind: ["fire", "water"] / ["normal", null]
                    let kinds: Vec<&str> = item
                        .get("kind")
                        .and_then(|k| k.as_array())
                        .map(|arr| arr.iter().filter_map(|k| k.as_str()).collect::<Vec<&str>>())
                        .unwrap_or_default();
                    let kind_text = if kinds.is_empty() {
                        "—".to_string()
                    } else {
                        kinds
                            .iter()
                            .map(|k| kind_name(k))
                            .collect::<Vec<_>>()
                            .join("/")
                    };
                    let is_selling = json_bool(item, "is_selling");
                    let cost = json_u64(item, "cost");
                    let cost_text = if is_selling {
                        cost.to_string()
                    } else {
                        "不可出售".to_string()
                    };
                    let is_legendary = json_bool(item, "is_legendary");
                    let legendary_tag = if is_legendary {
                        rsx! { span { class: "adm-pk-legendary", "神兽" } }
                    } else {
                        VNode::empty()
                    };
                    // 六维（initial_statistic 或展平字段，都尝试）。
                    let stat = item.get("initial_statistic").cloned().unwrap_or_default();
                    let sv = |key: &str, alias: &str| -> u64 {
                        stat.get(key)
                            .or_else(|| item.get(key))
                            .and_then(|x| x.as_u64())
                            .or_else(|| {
                                stat.get(alias)
                                    .or_else(|| item.get(alias))
                                    .and_then(|x| x.as_u64())
                            })
                            .unwrap_or(0)
                    };
                    let stats_text = format!(
                        "HP {} · 攻 {} · 防 {} · 特攻 {} · 特防 {} · 速 {}",
                        sv("hit_points", "hp"),
                        sv("attack", "atk"),
                        sv("defense", "def"),
                        sv("special_attack", "spatk"),
                        sv("special_defense", "spdef"),
                        sv("speed", "spd"),
                    );
                    trs.push(rsx! {
                        tr {
                            td { "{id}" }
                            td {
                                "{name}"
                                ..vec![legendary_tag]
                            }
                            td { "{kind_text}" }
                            td { "{cost_text}" }
                            td { class: "adm-pk-stats", "{stats_text}" }
                        }
                    });
                }
                rsx! {
                    table { class: "adm-table",
                        thead {
                            tr {
                                th { "ID" }
                                th { "名称" }
                                th { "属性" }
                                th { "售价" }
                                th { "六维" }
                            }
                        }
                        tbody {
                            for i in 0..trs.len() {
                                ..vec![trs[i].clone()]
                            }
                        }
                    }
                }
            };
            let more_btn = if has_more {
                let label = format!("加载更多（已加载 {loaded_count}/{total}）");
                rsx! {
                    div { class: "adm-load-more",
                        button {
                            class: "admin-btn",
                            disabled: busy,
                            "data-pm-adm-pk-more": "1",
                            "{label}"
                        }
                    }
                }
            } else {
                VNode::empty()
            };
            let parts: Vec<VNode> = vec![err_bar, table, more_btn];
            rsx! {
                for i in 0..parts.len() {
                    ..vec![parts[i].clone()]
                }
            }
        }
    };

    rsx! {
        section { class: "admin-page admin-config-page",
            div { class: "admin-page-header",
                h2 { "宠物数据" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "ID（精确）",
                    value: f_id,
                    "data-pm-adm-pk-f-id": "1"
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "名称（包含）",
                    value: f_name,
                    "data-pm-adm-pk-f-name": "1"
                }
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: "属性（如火）",
                    value: f_kind,
                    "data-pm-adm-pk-f-kind": "1"
                }
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-pk-apply": "1",
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-pk-reset": "1",
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-pk-reload": "1",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}
