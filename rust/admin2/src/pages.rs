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
        AdminRoute::EvolutionData => {
            if state.evolution.loaded.get().is_none() && !state.evolution.busy.get() {
                reload_list(crate::state::EVOLUTION_SPEC, &state.evolution);
            }
        }
        AdminRoute::SkillType => {
            if state.skill.loaded.get().is_none() && !state.skill.busy.get() {
                reload_list(crate::state::SKILL_SPEC, &state.skill);
            }
        }
        AdminRoute::EffectData => {
            if state.effect.loaded.get().is_none() && !state.effect.busy.get() {
                reload_list(crate::state::EFFECT_SPEC, &state.effect);
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
        packages.push(crate::api::filter_package(
            spec.name_tag,
            spec.name_operator,
            &name,
        ));
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
        AdminRoute::EvolutionData => evolution_data_page(&st),
        AdminRoute::SkillType => skill_data_page(&st),
        AdminRoute::EffectData => effect_data_page(&st),
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
                    let id_for_edit = id.clone();
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
                            td {
                                button {
                                    class: "admin-btn adm-row-edit",
                                    disabled: busy,
                                    "data-pm-adm-pk-edit": id_for_edit,
                                    "编辑"
                                }
                            }
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
                                th { "操作" }
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
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-pk-new": "1",
                    "新增宠物"
                }
            }
            ..vec![body, pokemon_editor_modal(st)]
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
                    let id_for_edit = id.clone();
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
                            td {
                                button {
                                    class: "admin-btn adm-row-edit",
                                    disabled: busy,
                                    "data-pm-adm-it-edit": id_for_edit,
                                    "编辑"
                                }
                            }
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
                                th { "操作" }
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
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-it-new": "1",
                    "新增物品"
                }
            }
            ..vec![body, item_editor_modal(st)]
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
                    let id_for_edit = id.clone();
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
                            td {
                                button {
                                    class: "admin-btn adm-row-edit",
                                    disabled: busy,
                                    "data-pm-adm-mp-edit": id_for_edit,
                                    "编辑"
                                }
                            }
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
                                th { "操作" }
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
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-mp-new": "1",
                    "新增地图"
                }
            }
            ..vec![body, map_editor_modal(st)]
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

/// 进化链数据页（切片 6）：列表页模板；name 槽位=进化来源、a 槽位=进化目标。
fn evolution_data_page(st: &AppState) -> VNode {
    let ls = st.evolution.clone();
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
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载进化链数据..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-l-reload": "ev",
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
                rsx! { p { class: "admin-empty-hint", "暂无进化链数据。" } }
            } else {
                let mut trs: Vec<VNode> = Vec::new();
                for item in rows.iter() {
                    let id = json_u64(item, "id").to_string();
                    // 历史数据存在 from_id/to_id 为 0 的孤儿行，名字为空——显示占位符。
                    let source = {
                        let s = json_str(item, "source_name");
                        if s.is_empty() {
                            format!("（未知 #{}）", json_u64(item, "source_id"))
                        } else {
                            s
                        }
                    };
                    let target = {
                        let s = json_str(item, "target_name");
                        if s.is_empty() {
                            format!("（未知 #{}）", json_u64(item, "target_id"))
                        } else {
                            s
                        }
                    };
                    let condition = json_str(item, "condition");
                    let priority = json_u64(item, "priority").to_string();
                    let item_name = json_str(item, "item_name");
                    let item_hint = if item_name.is_empty() {
                        "—".to_string()
                    } else {
                        item_name
                    };
                    trs.push(rsx! {
                        tr {
                            td { "{id}" }
                            td { "{source}" }
                            td { "{target}" }
                            td { class: "adm-pk-stats", "{condition}" }
                            td { "{item_hint}" }
                            td { "{priority}" }
                        }
                    });
                }
                rsx! {
                    table { class: "adm-table",
                        thead {
                            tr {
                                th { "ID" }
                                th { "进化来源" }
                                th { "进化目标" }
                                th { "条件" }
                                th { "道具" }
                                th { "优先级" }
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
                            "data-pm-adm-l-more": "ev",
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
                h2 { "进化链数据" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "ID（精确）",
                    value: f_id,
                    "data-pm-adm-f-id": "ev"
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "进化来源（宠物名）",
                    value: f_name,
                    "data-pm-adm-f-name": "ev"
                }
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: "进化目标（宠物名）",
                    value: f_a,
                    "data-pm-adm-f-a": "ev"
                }
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-l-apply": "ev",
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reset": "ev",
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reload": "ev",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}

/// 技能数据页（切片 7a）：列表页模板 + 泛型动作/属性（sk 页键）。
fn skill_data_page(st: &AppState) -> VNode {
    let ls = st.skill.clone();
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
    let count_text = if filtered {
        format!("筛选结果 {loaded_count} 条")
    } else {
        format!("已加载 {loaded_count} / {total} 条")
    };

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载技能数据..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-l-reload": "sk",
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
                rsx! { p { class: "admin-empty-hint", "暂无技能数据。" } }
            } else {
                let mut trs: Vec<VNode> = Vec::new();
                for item in rows.iter() {
                    let id = json_u64(item, "id").to_string();
                    let name = json_str(item, "name");
                    let desc = json_str(item, "description");
                    let desc_short = if desc.chars().count() > 18 {
                        let cut: String = desc.chars().take(18).collect();
                        format!("{cut}…")
                    } else {
                        desc
                    };
                    let level = json_u64(item, "min_level_limit").to_string();
                    let uses = json_u64(item, "use_times_limit").to_string();
                    // available_pokemons 是种族 ID 数组，只显示数量。
                    let species_count = item
                        .get("available_pokemons")
                        .and_then(|l| l.as_array())
                        .map(|a| a.len())
                        .unwrap_or(0)
                        .to_string();
                    trs.push(rsx! {
                        tr {
                            td { "{id}" }
                            td { "{name}" }
                            td { class: "adm-pk-stats", "{desc_short}" }
                            td { "{level}" }
                            td { "{uses}" }
                            td { "{species_count}" }
                        }
                    });
                }
                rsx! {
                    table { class: "adm-table",
                        thead {
                            tr {
                                th { "ID" }
                                th { "名称" }
                                th { "描述" }
                                th { "等级限制" }
                                th { "次数" }
                                th { "可学种族数" }
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
                            "data-pm-adm-l-more": "sk",
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
                h2 { "技能数据" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "ID（精确）",
                    value: f_id,
                    "data-pm-adm-f-id": "sk"
                }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    placeholder: "名称（包含）",
                    value: f_name,
                    "data-pm-adm-f-name": "sk"
                }
                input {
                    class: "adm-filter-input adm-filter-kind",
                    r#type: "text",
                    placeholder: "种族 ID（如 25 皮卡丘）",
                    value: f_a,
                    "data-pm-adm-f-a": "sk"
                }
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-l-apply": "sk",
                    "筛选"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reset": "sk",
                    "重置"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reload": "sk",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}

/// 效果数据页（切片 7b）：服务端无 filter 端点，只 list/count + 分页。
fn effect_data_page(st: &AppState) -> VNode {
    let ls = st.effect.clone();
    let load_state = ls.loaded.get();
    let busy = ls.busy.get();
    let rows = ls.rows.get();
    let total = ls.total.get();
    let loaded_count = ls.loaded_count.get();
    let has_more = loaded_count < total;
    let count_text = format!("已加载 {loaded_count} / {total} 条");

    let body = match &load_state {
        None => rsx! { div { class: "admin-loading", "正在加载效果数据..." } },
        Some(Err(e)) => {
            let msg = format!("加载失败：{e}");
            rsx! {
                div { class: "admin-card admin-error",
                    p { "{msg}" }
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-l-reload": "ef",
                        "重新加载"
                    }
                }
            }
        }
        Some(Ok(())) => {
            let table = if rows.is_empty() {
                rsx! { p { class: "admin-empty-hint", "暂无效果数据。" } }
            } else {
                let mut trs: Vec<VNode> = Vec::new();
                for item in rows.iter() {
                    let id = json_u64(item, "id").to_string();
                    let code = json_str(item, "code");
                    let kind = json_str(item, "kind");
                    let version = json_u64(item, "version").to_string();
                    let hooks = item
                        .get("hooks")
                        .and_then(|h| h.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|x| x.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default();
                    let desc = json_str(item, "description");
                    let desc_short = if desc.chars().count() > 20 {
                        let cut: String = desc.chars().take(20).collect();
                        format!("{cut}…")
                    } else {
                        desc
                    };
                    trs.push(rsx! {
                        tr {
                            td { "{id}" }
                            td { "{code}" }
                            td { "{kind}" }
                            td { class: "adm-pk-stats", "{hooks}" }
                            td { "{version}" }
                            td { class: "adm-pk-stats", "{desc_short}" }
                        }
                    });
                }
                rsx! {
                    table { class: "adm-table",
                        thead {
                            tr {
                                th { "ID" }
                                th { "效果码" }
                                th { "类型" }
                                th { "钩子" }
                                th { "版本" }
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
                            "data-pm-adm-l-more": "ef",
                            "{label}"
                        }
                    }
                }
            } else {
                VNode::empty()
            };
            let parts: Vec<VNode> = vec![table, more_btn];
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
                h2 { "效果数据" }
                span { class: "adm-count", "{count_text}" }
            }
            div { class: "admin-card adm-filter-bar",
                span { class: "admin-config-hint",
                    "战斗引擎 2.0 效果模板。服务端未提供筛选端点，此页仅支持刷新与分页。"
                }
                button {
                    class: "admin-btn",
                    disabled: busy,
                    "data-pm-adm-l-reload": "ef",
                    "刷新"
                }
            }
            ..vec![body]
        }
    }
}

// ---------------------------------------------------------------------------
// 宝可梦编辑弹窗（切片 8）
// ---------------------------------------------------------------------------

const STAT_KEYS: [&str; 6] = [
    "hit_points",
    "attack",
    "defense",
    "special_attack",
    "special_defense",
    "speed",
];
const STAT_LABELS: [&str; 6] = ["HP", "攻击", "防御", "特攻", "特防", "速度"];

/// 打开新增弹窗（默认骨架）。
pub fn open_pokemon_new(state: &AppState) {
    let ed = state.pokemon_editor.clone();
    ed.editing_id.inner().set(None);
    ed.name.inner().set(String::new());
    ed.description.inner().set(String::new());
    ed.cost.inner().set("0".to_string());
    ed.is_selling.inner().set(false);
    ed.stat
        .inner()
        .set(std::array::from_fn(|_| "50".to_string()));
    ed.base
        .inner()
        .set(std::array::from_fn(|_| "0".to_string()));
    ed.kind1.inner().set("普通".to_string());
    ed.kind2.inner().set(String::new());
    ed.is_legendary.inner().set(false);
    // 表单未覆盖字段的默认值：服务端要求这些键存在。
    ed.extra_json.inner().set(
        serde_json::json!({
            "sex_weight": 0.5,
            "map_ids": [],
            "evolution_info_ids": [],
            "capture_weight": 0,
            "meet_weight": 0,
            "birth_order": 0,
            "strength_weight": 0,
            "drop_money_range": [0, 0]
        })
        .to_string(),
    );
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

/// 打开编辑弹窗（从当前行数据填充表单）。
pub fn open_pokemon_edit(state: &AppState, id: u64) {
    let ed = state.pokemon_editor.clone();
    let row = state
        .pokemon
        .rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    ed.editing_id.inner().set(Some(id));
    ed.name.inner().set(json_str(&row, "name"));
    ed.description.inner().set(json_str(&row, "description"));
    ed.cost.inner().set(json_u64(&row, "cost").to_string());
    ed.is_selling.inner().set(json_bool(&row, "is_selling"));
    let stat = row.get("initial_statistic").cloned().unwrap_or_default();
    let base = row.get("initial_base_points").cloned().unwrap_or_default();
    ed.stat.inner().set(std::array::from_fn(|i| {
        stat.get(STAT_KEYS[i])
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            .to_string()
    }));
    ed.base.inner().set(std::array::from_fn(|i| {
        base.get(STAT_KEYS[i])
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            .to_string()
    }));
    // kind: ["fire", null] → 中文名（kind_name 反向查表）
    let kinds: Vec<&str> = row
        .get("kind")
        .and_then(|k| k.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str()).collect())
        .unwrap_or_default();
    let to_cn = |code: &str| -> String {
        let name = kind_name(code);
        if name == "？" {
            String::new()
        } else {
            name.to_string()
        }
    };
    ed.kind1
        .inner()
        .set(kinds.first().map(|k| to_cn(k)).unwrap_or_default());
    ed.kind2
        .inner()
        .set(kinds.get(1).map(|k| to_cn(k)).unwrap_or_default());
    ed.is_legendary
        .inner()
        .set(json_bool(&row, "is_legendary"));
    // 未覆盖字段原样保留（避免保存时丢数据）。
    let extra = serde_json::json!({
        "sex_weight": row.get("sex_weight").cloned().unwrap_or(serde_json::Value::Null),
        "map_ids": row.get("map_ids").cloned().unwrap_or(serde_json::json!([])),
        "evolution_info_ids": row.get("evolution_info_ids").cloned().unwrap_or(serde_json::json!([])),
        "capture_weight": json_u64(&row, "capture_weight"),
        "meet_weight": json_u64(&row, "meet_weight"),
        "birth_order": json_u64(&row, "birth_order"),
        "strength_weight": json_u64(&row, "strength_weight"),
        "drop_money_range": row.get("drop_money_range").cloned().unwrap_or(serde_json::json!([0, 0])),
    });
    ed.extra_json
        .inner()
        .set(serde_json::to_string_pretty(&extra).unwrap_or_default());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_pokemon_editor(state: &AppState) {
    state.pokemon_editor.open.inner().set(false);
    crate::request_remount();
}

/// 表单字段变更（key 见 delegation 的 data-pm-adm-pkf）。数组字段用
/// `stat:i` / `base:i` 形式。
pub fn set_pokemon_editor_field(state: &AppState, key: &str, value: &str, checked: bool) {
    let ed = state.pokemon_editor.clone();
    match key {
        "name" => ed.name.inner().set(value.to_string()),
        "description" => ed.description.inner().set(value.to_string()),
        "cost" => ed.cost.inner().set(value.to_string()),
        "selling" => ed.is_selling.inner().set(checked),
        "kind1" => ed.kind1.inner().set(value.to_string()),
        "kind2" => ed.kind2.inner().set(value.to_string()),
        "legendary" => ed.is_legendary.inner().set(checked),
        "extra" => ed.extra_json.inner().set(value.to_string()),
        _ => {
            if let Some((group, idx)) = key.split_once(':') {
                if let Ok(i) = idx.parse::<usize>() {
                    if i < 6 {
                        if group == "stat" {
                            let mut arr = ed.stat.get();
                            arr[i] = value.to_string();
                            ed.stat.inner().set(arr);
                        } else if group == "base" {
                            let mut arr = ed.base.get();
                            arr[i] = value.to_string();
                            ed.base.inner().set(arr);
                        }
                    }
                }
            }
        }
    }
    crate::request_remount();
}

/// 提交：组装完整 PokemonType JSON → set/insert。
pub fn submit_pokemon_editor(state: &AppState) {
    let ed = state.pokemon_editor.clone();
    if ed.saving.get() {
        return;
    }
    let name = ed.name.get().trim().to_string();
    if name.is_empty() {
        ed.err.inner().set(Some("名称不能为空".to_string()));
        crate::request_remount();
        return;
    }
    // extra JSON 必须合法（承载表单未覆盖字段）。
    let extra: serde_json::Value = match serde_json::from_str(&ed.extra_json.get()) {
        Ok(v) => v,
        Err(e) => {
            ed.err
                .inner()
                .set(Some(format!("扩展字段 JSON 无效：{e}")));
            crate::request_remount();
            return;
        }
    };
    let stat = ed.stat.get();
    let base = ed.base.get();
    let parse_num = |s: &str| s.trim().parse::<u64>().unwrap_or(0);
    // 个体值 0-255、努力值 0-3（服务端硬校验）。
    if stat.iter().any(|s| parse_num(s) > 255) {
        ed.err
            .inner()
            .set(Some("初始个体值必须在 0-255".to_string()));
        crate::request_remount();
        return;
    }
    if base.iter().any(|s| parse_num(s) > 3) {
        ed.err
            .inner()
            .set(Some("初始努力值必须在 0-3".to_string()));
        crate::request_remount();
        return;
    }
    let kind_code = |cn: &str| -> serde_json::Value {
        match reverse_kind_name(cn.trim()) {
            Some(code) => serde_json::json!(code),
            None => serde_json::Value::Null,
        }
    };
    let kind2 = kind_code(&ed.kind2.get());
    // 属性 1 必填：服务端会把 null 写进 xs 列（translate_kind_id_to_chinese_kind
    // 的 default 分支返回 null），落库后该宠物无法按属性筛选。
    if kind_code(&ed.kind1.get()).is_null() {
        ed.err
            .inner()
            .set(Some("属性 1 必须是合法中文属性名（如 火/水/电）".to_string()));
        crate::request_remount();
        return;
    }
    let mut payload = serde_json::json!({
        "name": name,
        "description": ed.description.get(),
        "cost": parse_num(&ed.cost.get()),
        "is_selling": ed.is_selling.get(),
        "initial_statistic": {
            STAT_KEYS[0]: parse_num(&stat[0]),
            STAT_KEYS[1]: parse_num(&stat[1]),
            STAT_KEYS[2]: parse_num(&stat[2]),
            STAT_KEYS[3]: parse_num(&stat[3]),
            STAT_KEYS[4]: parse_num(&stat[4]),
            STAT_KEYS[5]: parse_num(&stat[5]),
        },
        "initial_base_points": {
            STAT_KEYS[0]: parse_num(&base[0]),
            STAT_KEYS[1]: parse_num(&base[1]),
            STAT_KEYS[2]: parse_num(&base[2]),
            STAT_KEYS[3]: parse_num(&base[3]),
            STAT_KEYS[4]: parse_num(&base[4]),
            STAT_KEYS[5]: parse_num(&base[5]),
        },
        "kind": [kind_code(&ed.kind1.get()), kind2],
        "is_legendary": ed.is_legendary.get(),
    });
    let Some(extra_obj) = extra.as_object() else {
        ed.err
            .inner()
            .set(Some("扩展字段必须是一个 JSON 对象".to_string()));
        crate::request_remount();
        return;
    };
    if let Some(dst) = payload.as_object_mut() {
        for (k, v) in extra_obj {
            dst.insert(k.clone(), v.clone());
        }
        // 服务端会无条件读这些键（foreach / 下标），缺一个就落 PHP 警告。
        for (k, default) in [
            ("sex_weight", serde_json::Value::Null),
            ("map_ids", serde_json::json!([])),
            ("evolution_info_ids", serde_json::json!([])),
            ("capture_weight", serde_json::json!(0)),
            ("meet_weight", serde_json::json!(0)),
            ("birth_order", serde_json::json!(0)),
            ("strength_weight", serde_json::json!(0)),
            ("drop_money_range", serde_json::json!([0, 0])),
        ] {
            dst.entry(k.to_string()).or_insert(default);
        }
    }
    let editing = ed.editing_id.get();
    if let Some(id) = editing {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(id));
        }
    }
    let body = payload.to_string();
    ed.saving.inner().set(true);
    ed.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    let on_done: Box<dyn FnOnce(Result<serde_json::Value, String>)> = Box::new(move |result| {
        let ed = st.pokemon_editor.clone();
        match result {
            Ok(_) => {
                ed.open.inner().set(false);
                ed.saving.inner().set(false);
                // 刷新列表（mock 信号复位后由 ensure_loaded 重新拉取）。
                st.pokemon.loaded.inner().set(None);
                st.pokemon.rows.inner().set(Vec::new());
                crate::request_remount();
                reload_list(crate::state::POKEMON_SPEC, &st.pokemon);
            }
            Err(e) => {
                ed.saving.inner().set(false);
                ed.err.inner().set(Some(format!("保存失败：{e}")));
                crate::request_remount();
            }
        }
    });
    if editing.is_some() {
        crate::api::set_entity("pokemon_type", body, on_done);
    } else {
        crate::api::insert_entity("pokemon_type", body, on_done);
    }
}

/// 编辑弹窗渲染（未打开时为空 VNode）。
fn pokemon_editor_modal(st: &AppState) -> VNode {
    let ed = st.pokemon_editor.clone();
    if !ed.open.get() {
        return VNode::empty();
    }
    let editing = ed.editing_id.get();
    let title = match editing {
        Some(id) => format!("编辑宠物 #{id}"),
        None => "新增宠物".to_string(),
    };
    let saving = ed.saving.get();
    let err = ed.err.get();
    let stat = ed.stat.get();
    let base = ed.base.get();
    // 布尔字段用双态按钮而不是 checkbox：jco 下 ChangeEvent 只带 value，
    // 读不到 checkbox 的 checked（要额外接 html-input-element.get_checked
    // 与 shim 导出），按钮 + 重挂是同一份架构里最省的一条路。
    let selling_label = if ed.is_selling.get() {
        "商店出售：是"
    } else {
        "商店出售：否"
    };
    let legendary_label = if ed.is_legendary.get() {
        "神兽：是"
    } else {
        "神兽：否"
    };
    let save_label = if saving { "保存中..." } else { "保存" };
    let selling_cls = if ed.is_selling.get() {
        "admin-btn admin-btn-primary adm-editor-toggle"
    } else {
        "admin-btn adm-editor-toggle"
    };
    let legendary_cls = if ed.is_legendary.get() {
        "admin-btn admin-btn-primary adm-editor-toggle"
    } else {
        "admin-btn adm-editor-toggle"
    };

    let mut stat_inputs: Vec<VNode> = Vec::new();
    let mut base_inputs: Vec<VNode> = Vec::new();
    for i in 0..6 {
        let sv = stat[i].clone();
        let sa = format!("stat:{i}");
        let sl = STAT_LABELS[i];
        stat_inputs.push(rsx! {
            label { class: "adm-editor-field",
                span { "{sl}" }
                input {
                    class: "adm-filter-input",
                    r#type: "number",
                    min: "0",
                    max: "255",
                    value: sv,
                    "data-pm-adm-pkf": sa
                }
            }
        });
        let bv = base[i].clone();
        let ba = format!("base:{i}");
        base_inputs.push(rsx! {
            label { class: "adm-editor-field",
                span { "{sl}" }
                input {
                    class: "adm-filter-input",
                    r#type: "number",
                    min: "0",
                    max: "3",
                    value: bv,
                    "data-pm-adm-pkf": ba
                }
            }
        });
    }

    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-pk-overlay": "1",
            div { class: "adm-editor-modal", "data-pm-adm-pk-modal": "1",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-pk-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    div { class: "adm-editor-grid",
                        label { class: "adm-editor-field",
                            span { "名称" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.name.get(),
                                "data-pm-adm-pkf": "name"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "售价" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                value: ed.cost.get(),
                                "data-pm-adm-pkf": "cost"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "属性 1（中文，如 火）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.kind1.get(),
                                "data-pm-adm-pkf": "kind1"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "属性 2（可空）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.kind2.get(),
                                "data-pm-adm-pkf": "kind2"
                            }
                        }
                        button {
                            class: selling_cls,
                            disabled: saving,
                            "data-pm-adm-pk-toggle": "selling",
                            "{selling_label}"
                        }
                        button {
                            class: legendary_cls,
                            disabled: saving,
                            "data-pm-adm-pk-toggle": "legendary",
                            "{legendary_label}"
                        }
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "描述" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 2,
                            "data-pm-adm-pkf": "description",
                            "{ed.description.get()}"
                        }
                    }
                    div { class: "adm-editor-section-title", "初始个体值（0-255）" }
                    div { class: "adm-editor-grid",
                        for i in 0..stat_inputs.len() {
                            ..vec![stat_inputs[i].clone()]
                        }
                    }
                    div { class: "adm-editor-section-title", "初始努力值（0-3）" }
                    div { class: "adm-editor-grid",
                        for i in 0..base_inputs.len() {
                            ..vec![base_inputs[i].clone()]
                        }
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "扩展字段 JSON（sex_weight / map_ids / evolution_info_ids / capture_weight / meet_weight / birth_order / strength_weight / drop_money_range）" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 8,
                            "data-pm-adm-pkf": "extra",
                            "{ed.extra_json.get()}"
                        }
                    }
                    ..vec![err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-pk-close": "1",
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn-primary",
                        disabled: saving,
                        "data-pm-adm-pk-save": "1",
                        "{save_label}"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 物品编辑弹窗（切片 9）
// ---------------------------------------------------------------------------

/// effects 的 11 个字段，顺序与表单渲染/提交一致。
const ITEM_EFFECT_KEYS: [&str; 11] = [
    "add_hit_points",
    "add_experience",
    "add_level",
    "add_intimacy",
    "attribute_add_hit_points",
    "attribute_add_attack",
    "attribute_add_defense",
    "attribute_add_special_attack",
    "attribute_add_special_defense",
    "attribute_add_speed",
    "capture",
];

const ITEM_EFFECT_LABELS: [&str; 11] = [
    "恢复 HP",
    "增加经验",
    "增加等级",
    "增加好感",
    "HP+",
    "攻击+",
    "防御+",
    "特攻+",
    "特防+",
    "速度+",
    "捕获率修正",
];

/// 类型 → (中文名, 附加值输入框提示)。药品无附加值，其余四种都要。
fn item_tag_kinds() -> [(&'static str, &'static str, Option<&'static str>); 5] {
    [
        ("drug", "药品", None),
        ("ball", "宠物球", Some("宠物球 ID")),
        ("evolution", "升级素材", Some("进化素材 ID")),
        ("armor", "装备", Some("部位：head/necklace/weapon/armor")),
        ("special", "特殊物品", Some("特殊物品标识文本")),
    ]
}

/// 从当前行载荷还原 (tag_kind, tag_value)。
fn item_tag_parts(item: &serde_json::Value) -> (String, String) {
    let Some(obj) = item.get("tag").and_then(|t| t.as_object()) else {
        return ("drug".to_string(), String::new());
    };
    let Some((key, value)) = obj.iter().next() else {
        return ("drug".to_string(), String::new());
    };
    let text = match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        _ => String::new(),
    };
    (key.clone(), text)
}

/// 把表单里的 (tag_kind, tag_value) 还原成服务端要的 tag 对象。
/// 药品发 `{"drug":null}`（translator 认这个形态，也认裸 "drug"）。
fn item_tag_payload(kind: &str, value: &str) -> serde_json::Value {
    match kind {
        "ball" => serde_json::json!({ "ball": value.trim().parse::<u64>().unwrap_or(0) }),
        "evolution" => serde_json::json!({ "evolution": value.trim().parse::<u64>().unwrap_or(0) }),
        "special" => serde_json::json!({ "special": value }),
        "armor" => {
            let site = match value.trim() {
                "head" | "necklace" | "weapon" | "armor" => value.trim().to_string(),
                _ => "head".to_string(),
            };
            serde_json::json!({ "armor": site })
        }
        _ => serde_json::json!({ "drug": serde_json::Value::Null }),
    }
}

pub fn open_item_new(state: &AppState) {
    let ed = state.item_editor.clone();
    ed.editing_id.inner().set(None);
    ed.name.inner().set(String::new());
    ed.img_name.inner().set(String::new());
    ed.description.inner().set(String::new());
    ed.price.inner().set("0".to_string());
    ed.is_selling.inner().set(false);
    ed.tag_kind.inner().set("drug".to_string());
    ed.tag_value.inner().set(String::new());
    ed.min_level.inner().set("0".to_string());
    ed.kind_require.inner().set(String::new());
    ed.effects
        .inner()
        .set(std::array::from_fn(|_| "0".to_string()));
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn open_item_edit(state: &AppState, id: u64) {
    let ed = state.item_editor.clone();
    let row = state
        .item
        .rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    ed.editing_id.inner().set(Some(id));
    ed.name.inner().set(json_str(&row, "name"));
    ed.img_name.inner().set(json_str(&row, "img_name"));
    ed.description.inner().set(json_str(&row, "description"));
    ed.price.inner().set(json_u64(&row, "price").to_string());
    ed.is_selling.inner().set(json_bool(&row, "is_selling"));
    let (kind, value) = item_tag_parts(&row);
    ed.tag_kind.inner().set(kind);
    ed.tag_value.inner().set(value);
    let limits = row.get("limits").cloned().unwrap_or_default();
    ed.min_level
        .inner()
        .set(limits.get("min_level").and_then(|v| v.as_u64()).unwrap_or(0).to_string());
    // kind_require 是英文 id（服务端 translate_chinese_kind_to_kind_id 的产物）
    let kind_req = limits
        .get("kind_require")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let cn = kind_name(kind_req);
    ed.kind_require
        .inner()
        .set(if cn == "？" { String::new() } else { cn.to_string() });
    let effects = row.get("effects").cloned().unwrap_or_default();
    ed.effects.inner().set(std::array::from_fn(|i| {
        effects
            .get(ITEM_EFFECT_KEYS[i])
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
            .to_string()
    }));
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_item_editor(state: &AppState) {
    state.item_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn set_item_editor_field(state: &AppState, key: &str, value: &str) {
    let ed = state.item_editor.clone();
    match key {
        "name" => ed.name.inner().set(value.to_string()),
        "img_name" => ed.img_name.inner().set(value.to_string()),
        "description" => ed.description.inner().set(value.to_string()),
        "price" => ed.price.inner().set(value.to_string()),
        "tag_kind" => ed.tag_kind.inner().set(value.to_string()),
        "tag_value" => ed.tag_value.inner().set(value.to_string()),
        "min_level" => ed.min_level.inner().set(value.to_string()),
        "kind_require" => ed.kind_require.inner().set(value.to_string()),
        _ => {
            if let Some(idx) = key.strip_prefix("effect:") {
                if let Ok(i) = idx.parse::<usize>() {
                    if i < 11 {
                        let mut arr = ed.effects.get();
                        arr[i] = value.to_string();
                        ed.effects.inner().set(arr);
                    }
                }
            }
        }
    }
    crate::request_remount();
}

pub fn toggle_item_editor_selling(state: &AppState) {
    let ed = state.item_editor.clone();
    ed.is_selling.inner().set(!ed.is_selling.get());
    crate::request_remount();
}

pub fn submit_item_editor(state: &AppState) {
    let ed = state.item_editor.clone();
    if ed.saving.get() {
        return;
    }
    let name = ed.name.get().trim().to_string();
    if name.is_empty() {
        ed.err.inner().set(Some("名称不能为空".to_string()));
        crate::request_remount();
        return;
    }
    // 非药品类型必须有可解析的附加值：服务端 translator 对缺键直接报错退出，
    // 提前拦住能给出更清楚的原因。
    let tag_kind = ed.tag_kind.get();
    let tag_value = ed.tag_value.get();
    if tag_kind == "ball" || tag_kind == "evolution" {
        if tag_value.trim().parse::<u64>().is_err() {
            ed.err
                .inner()
                .set(Some("该类型需要一个非负整数 ID".to_string()));
            crate::request_remount();
            return;
        }
    }
    if tag_kind == "armor"
        && !matches!(tag_value.trim(), "head" | "necklace" | "weapon" | "armor")
    {
        ed.err
            .inner()
            .set(Some("装备部位必须是 head/necklace/weapon/armor".to_string()));
        crate::request_remount();
        return;
    }
    // 种族限制：服务端拿英文 id 写库，非法中文名会被翻成 null 再写成空串。
    let kind_req = ed.kind_require.get();
    let kind_req_trim = kind_req.trim();
    let kind_payload = if kind_req_trim.is_empty() {
        serde_json::Value::Null
    } else {
        let code = reverse_kind_name(kind_req_trim);
        if code.is_none() {
            ed.err
                .inner()
                .set(Some(format!("种族限制「{kind_req_trim}」不是合法属性名")));
            crate::request_remount();
            return;
        }
        serde_json::json!(code.unwrap())
    };
    let effects = ed.effects.get();
    let parse_i64 = |s: &str| s.trim().parse::<i64>().unwrap_or(0);
    let mut effects_payload = serde_json::Map::new();
    for i in 0..11 {
        effects_payload.insert(
            ITEM_EFFECT_KEYS[i].to_string(),
            serde_json::json!(parse_i64(&effects[i])),
        );
    }
    let mut payload = serde_json::json!({
        "name": name,
        "img_name": ed.img_name.get(),
        "description": ed.description.get(),
        "price": ed.price.get().trim().parse::<u64>().unwrap_or(0),
        "is_selling": ed.is_selling.get(),
        "tag": item_tag_payload(&tag_kind, &tag_value),
        "limits": {
            "min_level": ed.min_level.get().trim().parse::<u64>().unwrap_or(0),
            "kind_require": kind_payload,
        },
        "effects": serde_json::Value::Object(effects_payload),
    });
    let editing = ed.editing_id.get();
    if let Some(id) = editing {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(id));
        }
    }
    let body = payload.to_string();
    ed.saving.inner().set(true);
    ed.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    let on_done: Box<dyn FnOnce(Result<serde_json::Value, String>)> = Box::new(move |result| {
        let ed = st.item_editor.clone();
        match result {
            Ok(_) => {
                ed.open.inner().set(false);
                ed.saving.inner().set(false);
                st.item.loaded.inner().set(None);
                st.item.rows.inner().set(Vec::new());
                crate::request_remount();
                reload_list(crate::state::ITEM_SPEC, &st.item);
            }
            Err(e) => {
                ed.saving.inner().set(false);
                ed.err.inner().set(Some(format!("保存失败：{e}")));
                crate::request_remount();
            }
        }
    });
    if editing.is_some() {
        crate::api::set_entity("item_type", body, on_done);
    } else {
        crate::api::insert_entity("item_type", body, on_done);
    }
}

/// 中文属性名 → 英文 id（kind_name 的反向；供筛选/表单复用）。
fn reverse_kind_name(cn: &str) -> Option<&'static str> {
    const CODES: [&str; 18] = [
        "normal", "fire", "water", "grass", "electric", "ice", "fighting", "poison", "ground",
        "flying", "psychic", "bug", "rock", "ghost", "dragon", "dark", "steel", "fairy",
    ];
    CODES.into_iter().find(|c| kind_name(c) == cn)
}

fn item_editor_modal(st: &AppState) -> VNode {
    let ed = st.item_editor.clone();
    if !ed.open.get() {
        return VNode::empty();
    }
    let editing = ed.editing_id.get();
    let title = match editing {
        Some(id) => format!("编辑物品 #{id}"),
        None => "新增物品".to_string(),
    };
    let saving = ed.saving.get();
    let err = ed.err.get();
    let effects = ed.effects.get();
    let tag_kind = ed.tag_kind.get();
    let tag_value = ed.tag_value.get();
    let selling_label = if ed.is_selling.get() {
        "可购买：是"
    } else {
        "可购买：否"
    };
    let selling_cls = if ed.is_selling.get() {
        "admin-btn admin-btn-primary adm-editor-toggle"
    } else {
        "admin-btn adm-editor-toggle"
    };
    let save_label = if saving { "保存中..." } else { "保存" };

    // 类型选择：下拉在 jco 下 change 会带 value，可直接用。
    let mut kind_options: Vec<VNode> = Vec::new();
    for (code, label, _) in item_tag_kinds() {
        kind_options.push(rsx! {
            option {
                value: code,
                selected: code == tag_kind,
                "{label}"
            }
        });
    }
    // 附加值输入框（药品没有）。
    let extra_placeholder = item_tag_kinds()
        .into_iter()
        .find(|(code, _, _)| *code == tag_kind)
        .and_then(|(_, _, hint)| hint)
        .unwrap_or("");
    let tag_extra = if tag_kind == "drug" {
        rsx! { p { class: "adm-editor-hint", "药品无附加配置" } }
    } else {
        rsx! {
            label { class: "adm-editor-field",
                span { "{extra_placeholder}" }
                input {
                    class: "adm-filter-input",
                    r#type: "text",
                    value: tag_value,
                    "data-pm-adm-itf": "tag_value"
                }
            }
        }
    };

    let mut effect_inputs: Vec<VNode> = Vec::new();
    for i in 0..11 {
        let label = ITEM_EFFECT_LABELS[i];
        let attr = format!("effect:{i}");
        effect_inputs.push(rsx! {
            label { class: "adm-editor-field",
                span { "{label}" }
                input {
                    class: "adm-filter-input",
                    r#type: "number",
                    value: effects[i].clone(),
                    "data-pm-adm-itf": attr
                }
            }
        });
    }

    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };
    let img_name = ed.img_name.get();
    let img_src = format!("source/plugin/pokemon/pokemon_system/images/item/{img_name}.gif");
    let preview = if img_name.trim().is_empty() {
        VNode::empty()
    } else {
        let alt = img_name.clone();
        rsx! {
            img {
                class: "adm-editor-sprite",
                src: img_src,
                alt: alt
            }
        }
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-it-overlay": "1",
            div { class: "adm-editor-modal",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-it-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    div { class: "adm-editor-grid",
                        label { class: "adm-editor-field",
                            span { "名称" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.name.get(),
                                "data-pm-adm-itf": "name"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "图片文件名（不含扩展名）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: img_name,
                                "data-pm-adm-itf": "img_name"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "价格" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                value: ed.price.get(),
                                "data-pm-adm-itf": "price"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "最低等级" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                value: ed.min_level.get(),
                                "data-pm-adm-itf": "min_level"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "种族限制（中文名，可空）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.kind_require.get(),
                                "data-pm-adm-itf": "kind_require"
                            }
                        }
                        button {
                            class: selling_cls,
                            disabled: saving,
                            "data-pm-adm-it-toggle": "selling",
                            "{selling_label}"
                        }
                    }
                    div { class: "adm-editor-grid",
                        label { class: "adm-editor-field",
                            span { "类型" }
                            select {
                                class: "adm-filter-input",
                                "data-pm-adm-itf": "tag_kind",
                                for i in 0..kind_options.len() {
                                    ..vec![kind_options[i].clone()]
                                }
                            }
                        }
                        ..vec![tag_extra]
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "描述" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 2,
                            "data-pm-adm-itf": "description",
                            "{ed.description.get()}"
                        }
                    }
                    div { class: "adm-editor-section-title", "效果" }
                    div { class: "adm-editor-grid",
                        for i in 0..effect_inputs.len() {
                            ..vec![effect_inputs[i].clone()]
                        }
                    }
                    ..vec![preview, err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-it-close": "1",
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn-primary",
                        disabled: saving,
                        "data-pm-adm-it-save": "1",
                        "{save_label}"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 地图编辑弹窗（切片 10）
// ---------------------------------------------------------------------------

/// 地形码 → (英文全名, 中文名)。英文全名是服务端 translate_map_alpha_to_full_name
/// 的输出，反过来也是筛选接受的输入。
const AREA_TYPES: [(&str, &str, &str); 15] = [
    ("l", "plain", "平原"),
    ("g", "grass", "草丛"),
    ("p", "water", "水池"),
    ("s", "sea", "海洋"),
    ("b", "sea_bottom", "海底"),
    ("m", "mountain", "山谷"),
    ("c", "cave", "山洞"),
    ("d", "sand", "沙漠"),
    ("f", "factory", "工厂"),
    ("t", "base", "基地"),
    ("v", "town", "市镇"),
    ("n", "gym", "道馆"),
    ("h", "sky", "天空"),
    ("o", "deep_sea", "深海"),
    ("k", "lava", "熔岩"),
];

pub fn open_map_new(state: &AppState) {
    let ed = state.map_editor.clone();
    ed.editing_id.inner().set(None);
    ed.name.inner().set(String::new());
    ed.area_type.inner().set("l".to_string());
    ed.is_enabled.inner().set(true);
    ed.min_level.inner().set("1".to_string());
    ed.max_level.inner().set("10".to_string());
    ed.min_quality.inner().set("0".to_string());
    ed.mode.inner().set("wild".to_string());
    ed.experience.inner().set("0".to_string());
    ed.experience_increase_times
        .inner()
        .set("0".to_string());
    ed.bosses_json.inner().set("[]".to_string());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn open_map_edit(state: &AppState, id: u64) {
    let ed = state.map_editor.clone();
    let row = state
        .map
        .rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    ed.editing_id.inner().set(Some(id));
    ed.name.inner().set(json_str(&row, "name"));
    ed.area_type.inner().set(json_str(&row, "area_type"));
    ed.is_enabled.inner().set(json_bool(&row, "is_enabled"));
    ed.min_level
        .inner()
        .set(json_u64(&row, "min_level").to_string());
    ed.max_level
        .inner()
        .set(json_u64(&row, "max_level").to_string());
    ed.min_quality
        .inner()
        .set(json_u64(&row, "min_quality").to_string());
    let mode = json_str(&row, "mode");
    ed.mode
        .inner()
        .set(if mode.is_empty() { "wild".to_string() } else { mode });
    // boss 模式的 experience 在库里是 -1，载荷里已经归零，这里直接用载荷值。
    ed.experience
        .inner()
        .set(json_u64(&row, "experience").to_string());
    ed.experience_increase_times
        .inner()
        .set(json_u64(&row, "experience_increase_times").to_string());
    let bosses = row.get("bosses").cloned().unwrap_or(serde_json::json!([]));
    ed.bosses_json
        .inner()
        .set(serde_json::to_string_pretty(&bosses).unwrap_or_else(|_| "[]".to_string()));
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_map_editor(state: &AppState) {
    state.map_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn set_map_editor_field(state: &AppState, key: &str, value: &str) {
    let ed = state.map_editor.clone();
    match key {
        "name" => ed.name.inner().set(value.to_string()),
        "area_type" => ed.area_type.inner().set(value.to_string()),
        "min_level" => ed.min_level.inner().set(value.to_string()),
        "max_level" => ed.max_level.inner().set(value.to_string()),
        "min_quality" => ed.min_quality.inner().set(value.to_string()),
        "mode" => ed.mode.inner().set(value.to_string()),
        "experience" => ed.experience.inner().set(value.to_string()),
        "increase_times" => ed.experience_increase_times.inner().set(value.to_string()),
        "bosses" => ed.bosses_json.inner().set(value.to_string()),
        _ => {}
    }
    crate::request_remount();
}

pub fn toggle_map_editor_enabled(state: &AppState) {
    let ed = state.map_editor.clone();
    ed.is_enabled.inner().set(!ed.is_enabled.get());
    crate::request_remount();
}

pub fn submit_map_editor(state: &AppState) {
    let ed = state.map_editor.clone();
    if ed.saving.get() {
        return;
    }
    let name = ed.name.get().trim().to_string();
    if name.is_empty() {
        ed.err.inner().set(Some("名称不能为空".to_string()));
        crate::request_remount();
        return;
    }
    // 服务端硬校验：地形码必须已知、1 ≤ min ≤ max ≤ 255、品质门槛 0-8。
    let code = ed.area_type.get();
    if !AREA_TYPES.iter().any(|(c, _, _)| *c == code) {
        ed.err.inner().set(Some("地形类型无效".to_string()));
        crate::request_remount();
        return;
    }
    let parse = |s: &str| s.trim().parse::<u64>().unwrap_or(0);
    let min_level = parse(&ed.min_level.get());
    let max_level = parse(&ed.max_level.get());
    if min_level < 1 {
        ed.err
            .inner()
            .set(Some("最低等级必须高于或等于 1 级".to_string()));
        crate::request_remount();
        return;
    }
    if max_level > 255 {
        ed.err
            .inner()
            .set(Some("最高等级必须低于或等于 255 级".to_string()));
        crate::request_remount();
        return;
    }
    if min_level > max_level {
        ed.err
            .inner()
            .set(Some("最低等级不得高于最高等级".to_string()));
        crate::request_remount();
        return;
    }
    let min_quality = parse(&ed.min_quality.get());
    if min_quality > 8 {
        ed.err
            .inner()
            .set(Some("品质门槛必须在 0-8 之间".to_string()));
        crate::request_remount();
        return;
    }
    let mode = ed.mode.get();
    // bosses 只在 boss/hybrid 下参与提交：野生模式不读它，也就不能因为
    // 用户先前在 Boss 模式里留下的半截 JSON 拦住一次合法的野生保存。
    let needs_bosses = mode == "boss" || mode == "hybrid";
    let bosses: serde_json::Value = if needs_bosses {
        match serde_json::from_str(&ed.bosses_json.get()) {
            Ok(v) => v,
            Err(e) => {
                ed.err.inner().set(Some(format!("Boss 配置 JSON 无效：{e}")));
                crate::request_remount();
                return;
            }
        }
    } else {
        serde_json::json!([])
    };
    if needs_bosses && !bosses.is_array() {
        ed.err
            .inner()
            .set(Some("Boss 配置必须是一个数组".to_string()));
        crate::request_remount();
        return;
    }
    if needs_bosses && bosses.as_array().is_some_and(|a| a.is_empty()) {
        ed.err.inner().set(Some(format!(
            "{mode} 模式至少需要一个 Boss（否则服务端会把它当成野生模式）"
        )));
        crate::request_remount();
        return;
    }
    let mut payload = serde_json::json!({
        "name": name,
        "area_type": code,
        "is_enabled": ed.is_enabled.get(),
        "min_level": min_level,
        "max_level": max_level,
        "min_quality": min_quality,
        "mode": mode,
    });
    if let Some(obj) = payload.as_object_mut() {
        // 服务端按 mode 决定读哪些键：wild 读 experience/倍率，boss/hybrid
        // 读 bosses（hybrid 还读 experience）。全给上最省事，服务端只取
        // 它要的那几个。
        obj.insert(
            "experience".to_string(),
            serde_json::json!(parse(&ed.experience.get())),
        );
        obj.insert(
            "experience_increase_times".to_string(),
            serde_json::json!(parse(&ed.experience_increase_times.get())),
        );
        obj.insert("bosses".to_string(), bosses);
    }
    let editing = ed.editing_id.get();
    if let Some(id) = editing {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("id".to_string(), serde_json::json!(id));
        }
    }
    let body = payload.to_string();
    ed.saving.inner().set(true);
    ed.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    let on_done: Box<dyn FnOnce(Result<serde_json::Value, String>)> = Box::new(move |result| {
        let ed = st.map_editor.clone();
        match result {
            Ok(_) => {
                ed.open.inner().set(false);
                ed.saving.inner().set(false);
                st.map.loaded.inner().set(None);
                st.map.rows.inner().set(Vec::new());
                crate::request_remount();
                reload_list(crate::state::MAP_SPEC, &st.map);
            }
            Err(e) => {
                ed.saving.inner().set(false);
                ed.err.inner().set(Some(format!("保存失败：{e}")));
                crate::request_remount();
            }
        }
    });
    if editing.is_some() {
        crate::api::set_entity("map_info", body, on_done);
    } else {
        crate::api::insert_entity("map_info", body, on_done);
    }
}

fn map_editor_modal(st: &AppState) -> VNode {
    let ed = st.map_editor.clone();
    if !ed.open.get() {
        return VNode::empty();
    }
    let editing = ed.editing_id.get();
    let title = match editing {
        Some(id) => format!("编辑地图 #{id}"),
        None => "新增地图".to_string(),
    };
    let saving = ed.saving.get();
    let err = ed.err.get();
    let mode = ed.mode.get();
    let area_type = ed.area_type.get();
    let enabled_label = if ed.is_enabled.get() {
        "启用：是"
    } else {
        "启用：否"
    };
    let enabled_cls = if ed.is_enabled.get() {
        "admin-btn admin-btn-primary adm-editor-toggle"
    } else {
        "admin-btn adm-editor-toggle"
    };
    let save_label = if saving { "保存中..." } else { "保存" };

    let mut area_options: Vec<VNode> = Vec::new();
    for (code, full, cn) in AREA_TYPES {
        let label = format!("{cn}（{full}）");
        area_options.push(rsx! {
            option {
                value: code,
                selected: code == area_type,
                "{label}"
            }
        });
    }
    let mut mode_options: Vec<VNode> = Vec::new();
    for (code, label) in [("wild", "野生"), ("boss", "Boss"), ("hybrid", "混合")] {
        mode_options.push(rsx! {
            option {
                value: code,
                selected: code == mode,
                "{label}"
            }
        });
    }

    // 野生模式只显示经验倍率；boss/hybrid 显示 Boss JSON。
    let mode_extras: Vec<VNode> = if mode == "wild" {
        vec![rsx! {
            label { class: "adm-editor-field",
                span { "经验值" }
                input {
                    class: "adm-filter-input",
                    r#type: "number",
                    min: "0",
                    value: ed.experience.get(),
                    "data-pm-adm-mpf": "experience"
                }
            }
            label { class: "adm-editor-field",
                span { "经验倍率" }
                input {
                    class: "adm-filter-input",
                    r#type: "number",
                    min: "0",
                    value: ed.experience_increase_times.get(),
                    "data-pm-adm-mpf": "increase_times"
                }
            }
        }]
    } else {
        let mut v = Vec::new();
        if mode == "hybrid" {
            v.push(rsx! {
                label { class: "adm-editor-field",
                    span { "经验值" }
                    input {
                        class: "adm-filter-input",
                        r#type: "number",
                        min: "0",
                        value: ed.experience.get(),
                        "data-pm-adm-mpf": "experience"
                    }
                }
            });
        }
        v.push(rsx! {
            label { class: "adm-editor-field adm-editor-wide",
                span { "Boss 配置 JSON（数组，元素含 pokemon_type_id / pokemon_name / level / attributes / boss_multiplier）" }
                textarea {
                    class: "adm-editor-textarea",
                    rows: 8,
                    "data-pm-adm-mpf": "bosses",
                    "{ed.bosses_json.get()}"
                }
            }
        });
        v
    };

    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-mp-overlay": "1",
            div { class: "adm-editor-modal",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-mp-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    div { class: "adm-editor-grid",
                        label { class: "adm-editor-field",
                            span { "名称" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.name.get(),
                                "data-pm-adm-mpf": "name"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "地形类型" }
                            select {
                                class: "adm-filter-input",
                                "data-pm-adm-mpf": "area_type",
                                for i in 0..area_options.len() {
                                    ..vec![area_options[i].clone()]
                                }
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "模式" }
                            select {
                                class: "adm-filter-input",
                                "data-pm-adm-mpf": "mode",
                                for i in 0..mode_options.len() {
                                    ..vec![mode_options[i].clone()]
                                }
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "最低等级" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "1",
                                max: "255",
                                value: ed.min_level.get(),
                                "data-pm-adm-mpf": "min_level"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "最高等级" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "1",
                                max: "255",
                                value: ed.max_level.get(),
                                "data-pm-adm-mpf": "max_level"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "品质门槛（0-8，0=不限制）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                max: "8",
                                value: ed.min_quality.get(),
                                "data-pm-adm-mpf": "min_quality"
                            }
                        }
                        button {
                            class: enabled_cls,
                            disabled: saving,
                            "data-pm-adm-mp-toggle": "enabled",
                            "{enabled_label}"
                        }
                    }
                    div { class: "adm-editor-grid",
                        for i in 0..mode_extras.len() {
                            ..vec![mode_extras[i].clone()]
                        }
                    }
                    ..vec![err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-mp-close": "1",
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn-primary",
                        disabled: saving,
                        "data-pm-adm-mp-save": "1",
                        "{save_label}"
                    }
                }
            }
        }
    }
}
