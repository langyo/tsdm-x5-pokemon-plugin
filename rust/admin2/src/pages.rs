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
                reload_list(crate::state::POKEMON_SPEC, &state.pokemon);
            }
        }
        AdminRoute::ItemData => {
            if state.item.loaded.get().is_none() && !state.item.busy.get() {
                reload_list(crate::state::ITEM_SPEC, &state.item);
            }
        }
        AdminRoute::MapData => {
            if state.map.loaded.get().is_none() && !state.map.busy.get() {
                reload_list(crate::state::MAP_SPEC, &state.map);
            }
        }
        AdminRoute::UserData => {
            if state.user.loaded.get().is_none() && !state.user.busy.get() {
                reload_list(crate::state::USER_SPEC, &state.user);
            }
        }
        _ => {}
    }
}

// ===========================================================================
// 宝可梦数据页（切片 2：列表 + 筛选 + 加载更多）
// ===========================================================================

const LIST_PAGE_SIZE: u64 = 100;

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

pub fn apply_list_filters_pub(
    spec: crate::state::EntitySpec,
    state: &AppState,
    ls: &crate::state::ListState,
) {
    apply_list_filters(spec, state, ls);
}

pub fn reset_list_filters_pub(spec: crate::state::EntitySpec, ls: &crate::state::ListState) {
    reset_list_filters(spec, ls);
}

pub fn load_more_list_pub(spec: crate::state::EntitySpec, ls: &crate::state::ListState) {
    load_more_list(spec, ls);
}

/// 全量重载：count + 首页（对齐 Dioxus 版 reload_*；切片 3 起泛型化）。
fn reload_list(spec: crate::state::EntitySpec, ls: &crate::state::ListState) {
    let ls = ls.clone();
    ls.busy.inner().set(true);
    ls.filtered.inner().set(false);
    ls.err.inner().set(None);
    let entity = spec.entity;
    crate::api::count_entity(
        entity,
        Box::new(move |total| match total {
            Ok(total) => {
                ls.total.inner().set(total);
                let ls2 = ls.clone();
                crate::api::list_entity(
                    entity,
                    0,
                    LIST_PAGE_SIZE,
                    Box::new(move |result| {
                        match result {
                            Ok(rows) => {
                                let n = rows.len() as u64;
                                ls2.rows.inner().set(rows);
                                ls2.loaded_count.inner().set(n);
                                ls2.loaded.inner().set(Some(Ok(())));
                            }
                            Err(e) => {
                                ls2.loaded.inner().set(Some(Err(e)));
                            }
                        }
                        ls2.busy.inner().set(false);
                        crate::request_remount();
                    }),
                );
            }
            Err(e) => {
                ls.loaded.inner().set(Some(Err(e)));
                ls.busy.inner().set(false);
                crate::request_remount();
            }
        }),
    );
}

/// 加载更多：filter 态不分页（对齐 Dioxus 版）。
fn load_more_list(spec: crate::state::EntitySpec, ls: &crate::state::ListState) {
    if ls.busy.get() || ls.filtered.get() {
        return;
    }
    let from = ls.loaded_count.get();
    if from >= ls.total.get() {
        return;
    }
    let ls2 = ls.clone();
    ls2.busy.inner().set(true);
    crate::request_remount();
    crate::api::list_entity(
        spec.entity,
        from,
        LIST_PAGE_SIZE,
        Box::new(move |result| {
            if let Ok(rows) = result {
                let n = rows.len() as u64;
                let mut all = ls2.rows.get();
                all.extend(rows);
                ls2.rows.inner().set(all);
                ls2.loaded_count.inner().set(from + n);
            }
            ls2.busy.inner().set(false);
            crate::request_remount();
        }),
    );
}

/// 应用筛选：任一输入非空即发 filter；全空回到全量列表。
fn apply_list_filters(
    spec: crate::state::EntitySpec,
    state: &AppState,
    ls: &crate::state::ListState,
) {
    if ls.busy.get() {
        return;
    }
    let id = ls.f_id.get().trim().to_string();
    let name = ls.f_name.get().trim().to_string();
    let a = ls.f_a.get().trim().to_string();
    let b = ls.f_b.get().trim().to_string();
    if id.is_empty() && name.is_empty() && a.is_empty() && b.is_empty() {
        ls.loaded.inner().set(None);
        crate::request_remount();
        reload_list(spec, ls);
        return;
    }
    let mut packages: Vec<serde_json::Value> = Vec::new();
    if !id.is_empty() {
        packages.push(crate::api::filter_package(spec.id_tag, "equal", &id));
    }
    if !name.is_empty() {
        packages.push(crate::api::filter_package(spec.name_tag, spec.name_operator, &name));
    }
    if !a.is_empty() {
        if let Some(sa) = spec.a {
            packages.push(crate::api::filter_package(sa.tag, sa.operator, &a));
        }
    }
    if !b.is_empty() {
        if let Some(sb) = spec.b {
            packages.push(crate::api::filter_package(sb.tag, sb.operator, &b));
        }
    }
    let ls2 = ls.clone();
    ls2.busy.inner().set(true);
    ls2.err.inner().set(None);
    let entity = spec.entity;
    crate::request_remount();
    crate::api::filter_entity(
        entity,
        serde_json::to_string(&packages).unwrap_or_default(),
        Box::new(move |result| {
            match result {
                Ok(rows) => {
                    let n = rows.len() as u64;
                    ls2.rows.inner().set(rows);
                    ls2.loaded_count.inner().set(n);
                    ls2.total.inner().set(n);
                    ls2.filtered.inner().set(true);
                    ls2.loaded.inner().set(Some(Ok(())));
                }
                Err(e) => {
                    ls2.err.inner().set(Some(e));
                }
            }
            ls2.busy.inner().set(false);
            crate::request_remount();
        }),
    );
}

/// 重置筛选：清输入并回全量列表。
fn reset_list_filters(spec: crate::state::EntitySpec, ls: &crate::state::ListState) {
    ls.f_id.inner().set(String::new());
    ls.f_name.inner().set(String::new());
    ls.f_a.inner().set(String::new());
    ls.f_b.inner().set(String::new());
    ls.loaded.inner().set(None);
    crate::request_remount();
    reload_list(spec, ls);
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
        AdminRoute::ItemData => item_data_page(&st),
        AdminRoute::MapData => map_data_page(&st),
        AdminRoute::UserData => user_data_page(&st),
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
    let f_kind = pk.f_a.get();
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
                        "data-pm-adm-l-reload": "pk",
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
                            "data-pm-adm-l-more": "pk",
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
                    "data-pm-adm-f-id": "pk"
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "名称（包含）",
                    value: f_name,
                    "data-pm-adm-f-name": "pk"
                }
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: "属性（如火）",
                    value: f_kind,
                    "data-pm-adm-f-a": "pk"
                }
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-l-apply": "pk",
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reset": "pk",
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reload": "pk",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}

/// 物品数据页（切片 3）：复用列表页模板 + 泛型动作/属性（it 页键）。
fn item_data_page(st: &AppState) -> VNode {
    let ls = st.item.clone();
    let spec = crate::state::ITEM_SPEC;
    let load_state = ls.loaded.get();
    let busy = ls.busy.get();
    let filtered = ls.filtered.get();
    let err = ls.err.get();
    let rows = ls.rows.get();
    let total = ls.total.get();
    let loaded_count = ls.loaded_count.get();
    let has_more = !filtered && loaded_count < total;
    let f_id = ls.f_id.get();
    let f_name = ls.f_name.get();
    let f_a = ls.f_a.get();
    let f_b = ls.f_b.get();
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载物品数据..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-l-reload": "it",
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
                rsx! { p { class: "admin-empty-hint", "暂无物品数据。" } }
            } else {
                let mut trs: Vec<VNode> = Vec::new();
                for item in rows.iter() {
                    let id = json_u64(item, "id").to_string();
                    let name = json_str(item, "name");
                    // tag: {"drug":null} / {"ball":5} / {"evolution":3} / {"armor":…} / {"special":"…"}
                    let tag_label = item
                        .get("tag")
                        .and_then(|t| t.as_object())
                        .and_then(|obj| obj.keys().next())
                        .map(|k| match k.as_str() {
                            "drug" => "药品".to_string(),
                            "ball" => "宠物球".to_string(),
                            "evolution" => "升级素材".to_string(),
                            "enhance" => "强化道具".to_string(),
                            "armor" => "装备".to_string(),
                            "special" => "特殊物品".to_string(),
                            other => other.to_string(),
                        })
                        .unwrap_or_else(|| "—".to_string());
                    let price = json_u64(item, "price").to_string();
                    let selling = if json_bool(item, "is_selling") {
                        "是"
                    } else {
                        "否"
                    };
                    let desc = json_str(item, "description");
                    let desc_short = if desc.chars().count() > 16 {
                        let cut: String = desc.chars().take(16).collect();
                        format!("{cut}…")
                    } else {
                        desc
                    };
                    trs.push(rsx! {
                        tr {
                            td { "{id}" }
                            td { "{name}" }
                            td { "{tag_label}" }
                            td { "{price}" }
                            td { "{selling}" }
                            td { class: "adm-pk-stats", "{desc_short}" }
                        }
                    });
                }
                rsx! {
                    table { class: "adm-table",
                        thead {
                            tr {
                                th { "ID" }
                                th { "名称" }
                                th { "类型" }
                                th { "价格" }
                                th { "出售" }
                                th { "描述" }
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
                            "data-pm-adm-l-more": "it",
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

    let _ = spec;
    rsx! {
        section { class: "admin-page admin-config-page",
            div { class: "admin-page-header",
                h2 { "物品数据" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "ID（精确）",
                    value: f_id,
                    "data-pm-adm-f-id": "it"
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "名称（包含）",
                    value: f_name,
                    "data-pm-adm-f-name": "it"
                }
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: "类型（1药品/2球/3素材/4强化/5装备）",
                    value: f_a,
                    "data-pm-adm-f-a": "it"
                }
                input {
                    class: "adm-filter-input adm-filter-num",
                    r#type: "text",
                    placeholder: "价格 ≥",
                    value: f_b,
                    "data-pm-adm-f-b": "it"
                }
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-l-apply": "it",
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reset": "it",
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reload": "it",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}

/// 地形单字符 → 中文名（载荷 area_type 为 "l"/"g"/… 单字符）。
fn area_name(code: &str) -> &'static str {
    match code {
        "l" => "平原",
        "g" => "草丛",
        "p" => "水池",
        "s" => "海洋",
        "b" => "海底",
        "m" => "山谷",
        "c" => "山洞",
        "d" => "沙漠",
        "f" => "工厂",
        "t" => "基地",
        "v" => "市镇",
        "n" => "道馆",
        "h" => "天空",
        "o" => "深海",
        "k" => "熔岩",
        _ => "？",
    }
}

/// 地图数据页（切片 4）：列表页模板 + 泛型动作/属性（mp 页键）。
fn map_data_page(st: &AppState) -> VNode {
    let ls = st.map.clone();
    let load_state = ls.loaded.get();
    let busy = ls.busy.get();
    let filtered = ls.filtered.get();
    let err = ls.err.get();
    let rows = ls.rows.get();
    let total = ls.total.get();
    let loaded_count = ls.loaded_count.get();
    let has_more = !filtered && loaded_count < total;
    let f_id = ls.f_id.get();
    let f_name = ls.f_name.get();
    let f_a = ls.f_a.get();
    let f_b = ls.f_b.get();
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载地图数据..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-l-reload": "mp",
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
                rsx! { p { class: "admin-empty-hint", "暂无地图数据。" } }
            } else {
                let mut trs: Vec<VNode> = Vec::new();
                for item in rows.iter() {
                    let id = json_u64(item, "id").to_string();
                    let name = json_str(item, "name");
                    let area = area_name(&json_str(item, "area_type")).to_string();
                    let enabled_text = if json_bool(item, "is_enabled") {
                        "启用"
                    } else {
                        "停用"
                    };
                    let min_lv = json_u64(item, "min_level");
                    let max_lv = json_u64(item, "max_level");
                    let range_text = format!("Lv.{min_lv} ~ Lv.{max_lv}");
                    let min_quality = json_u64(item, "min_quality");
                    let quality_text = if min_quality > 0 {
                        min_quality.to_string()
                    } else {
                        "—".to_string()
                    };
                    // mode 是扁平字符串："wild"/"boss"/"hybrid"
                    let mode_label = match json_str(item, "mode").as_str() {
                        "wild" => "野生".to_string(),
                        "boss" => "Boss".to_string(),
                        "hybrid" => "混合".to_string(),
                        _ => "？".to_string(),
                    };
                    trs.push(rsx! {
                        tr {
                            td { "{id}" }
                            td { "{name}" }
                            td { "{area}" }
                            td { "{enabled_text}" }
                            td { "{range_text}" }
                            td { "{mode_label}" }
                            td { "{quality_text}" }
                        }
                    });
                }
                rsx! {
                    table { class: "adm-table",
                        thead {
                            tr {
                                th { "ID" }
                                th { "名称" }
                                th { "地形" }
                                th { "状态" }
                                th { "等级范围" }
                                th { "模式" }
                                th { "品质门槛" }
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
                            "data-pm-adm-l-more": "mp",
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
                h2 { "地图数据" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "ID（精确）",
                    value: f_id,
                    "data-pm-adm-f-id": "mp"
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "名称（包含）",
                    value: f_name,
                    "data-pm-adm-f-name": "mp"
                }
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: "地形（plain/grass/sea/cave…）",
                    value: f_a,
                    "data-pm-adm-f-a": "mp"
                }
                input {
                    class: "adm-filter-input adm-filter-num",
                    r#type: "text",
                    placeholder: "最低等级 ≥",
                    value: f_b,
                    "data-pm-adm-f-b": "mp"
                }
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-l-apply": "mp",
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reset": "mp",
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reload": "mp",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}

/// 用户数据页（切片 5）：列表页模板 + 泛型动作/属性（us 页键）。
/// ID 筛选 tag 是 UID、名称是 昵称（服务端反查 common_member.username）。
fn user_data_page(st: &AppState) -> VNode {
    let ls = st.user.clone();
    let load_state = ls.loaded.get();
    let busy = ls.busy.get();
    let filtered = ls.filtered.get();
    let err = ls.err.get();
    let rows = ls.rows.get();
    let total = ls.total.get();
    let loaded_count = ls.loaded_count.get();
    let has_more = !filtered && loaded_count < total;
    let f_id = ls.f_id.get();
    let f_name = ls.f_name.get();
    let f_a = ls.f_a.get();
    let f_b = ls.f_b.get();
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载用户数据..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-l-reload": "us",
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
                rsx! { p { class: "admin-empty-hint", "暂无用户数据。" } }
            } else {
                let mut trs: Vec<VNode> = Vec::new();
                for item in rows.iter() {
                    let id = json_u64(item, "id").to_string();
                    let name = json_str(item, "name");
                    let win = json_u64(item, "win_count").to_string();
                    let lose = json_u64(item, "lose_count").to_string();
                    let money = json_u64(item, "money").to_string();
                    let exp = json_u64(item, "experience").to_string();
                    let pm_count = item
                        .get("pokemon_list")
                        .and_then(|l| l.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0)
                        .to_string();
                    let item_count = item
                        .get("item_list")
                        .and_then(|l| l.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0)
                        .to_string();
                    trs.push(rsx! {
                        tr {
                            td { "{id}" }
                            td { "{name}" }
                            td { "{win}" }
                            td { "{lose}" }
                            td { "{money}" }
                            td { "{exp}" }
                            td { "{pm_count}" }
                            td { "{item_count}" }
                        }
                    });
                }
                rsx! {
                    table { class: "adm-table",
                        thead {
                            tr {
                                th { "UID" }
                                th { "用户名" }
                                th { "胜" }
                                th { "输" }
                                th { "金钱" }
                                th { "经验" }
                                th { "宠物" }
                                th { "物品" }
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
                            "data-pm-adm-l-more": "us",
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
                h2 { "用户数据" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "UID（精确）",
                    value: f_id,
                    "data-pm-adm-f-id": "us"
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "用户名（包含）",
                    value: f_name,
                    "data-pm-adm-f-name": "us"
                }
                input {
                    class: "adm-filter-input adm-filter-num",
                    r#type: "text",
                    placeholder: "胜场 ≥",
                    value: f_a,
                    "data-pm-adm-f-a": "us"
                }
                input {
                    class: "adm-filter-input adm-filter-num",
                    r#type: "text",
                    placeholder: "金钱 ≥",
                    value: f_b,
                    "data-pm-adm-f-b": "us"
                }
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-l-apply": "us",
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reset": "us",
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reload": "us",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}
