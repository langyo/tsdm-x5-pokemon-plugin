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
            ..vec![toast, delete_confirm_modal(&st)]
        }
    }
}

fn global_config_page(st: &AppState) -> VNode {
    let cfg = st.config.clone();
    let cf = st.config_form.clone();
    let load_state = cfg.loaded.get();
    let draft = cfg.draft.get();
    let dirty = cfg.is_dirty();
    let saving = cfg.saving.get();
    let raw_mode = cf.raw_mode.get();

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
            let mode_label = if raw_mode {
                "切换到表单"
            } else {
                "切换到原始 JSON"
            };
            let content = if raw_mode {
                let rows = draft.lines().count().max(4).min(30);
                rsx! {
                    div { class: "admin-card admin-config-editor",
                        textarea {
                            class: "admin-config-textarea",
                            rows: rows,
                            spellcheck: "false",
                            "data-pm-adm-config-input": "1",
                            "{draft}"
                        }
                    }
                }
            } else {
                config_form_body(st)
            };
            rsx! {
                div { class: "admin-card admin-config-editor",
                    div { class: "admin-config-toolbar",
                        span { class: "admin-config-hint",
                            if dirty { "有未保存修改" } else { "与已保存版本一致" }
                        }
                        button {
                            class: "admin-btn",
                            disabled: saving,
                            "data-pm-adm-config-rawmode": "1",
                            "{mode_label}"
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
                    ..vec![content]
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
                    let del_attr_pk = format!("pk:{id}");
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
                                div { class: "adm-row-actions",
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-pk-edit": id_for_edit,
                                        "编辑"
                                    }
                                    button {
                                        class: "admin-btn adm-row-del",
                                        disabled: busy,
                                        "data-pm-adm-del": del_attr_pk,
                                        "删除"
                                    }
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
                    let del_attr_it = format!("it:{id}");
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
                                div { class: "adm-row-actions",
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-it-edit": id_for_edit,
                                        "编辑"
                                    }
                                    button {
                                        class: "admin-btn adm-row-del",
                                        disabled: busy,
                                        "data-pm-adm-del": del_attr_it,
                                        "删除"
                                    }
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
                    let del_attr_mp = format!("mp:{id}");
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
                                div { class: "adm-row-actions",
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-mp-edit": id_for_edit,
                                        "编辑"
                                    }
                                    button {
                                        class: "admin-btn adm-row-del",
                                        disabled: busy,
                                        "data-pm-adm-del": del_attr_mp,
                                        "删除"
                                    }
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
                    let uid = id.clone();
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
                            td {
                                div { class: "adm-row-actions",
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-grant-poke": uid.clone(),
                                        "给宠物"
                                    }
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-grant-item": uid.clone(),
                                        "给物品"
                                    }
                                    button {
                                        class: "admin-btn",
                                        disabled: busy,
                                        "data-pm-adm-drill": uid,
                                        "管理"
                                    }
                                }
                            }
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
            ..vec![body, grant_modal(st), user_drill_modal(st)]
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
                    let id_for_edit = id.clone();
                    // 历史数据存在 from_id/to_id 为 0 的孤儿行，名字为空——显示占位符。
                    let del_attr_ev = format!("ev:{id}");
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
                            td {
                                div { class: "adm-row-actions",
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-ev-edit": id_for_edit,
                                        "编辑"
                                    }
                                    button {
                                        class: "admin-btn adm-row-del",
                                        disabled: busy,
                                        "data-pm-adm-del": del_attr_ev,
                                        "删除"
                                    }
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
                                th { "进化来源" }
                                th { "进化目标" }
                                th { "条件" }
                                th { "道具" }
                                th { "优先级" }
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
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-ev-new": "1",
                    "新增进化链"
                }
            }
            ..vec![body, evolution_editor_modal(st)]
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
                    let id_for_edit = id.clone();
                    let name = json_str(item, "name");
                    let desc = json_str(item, "description");
                    let del_attr_sk = format!("sk:{id}");
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
                            td {
                                div { class: "adm-row-actions",
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-sk-edit": id_for_edit,
                                        "编辑"
                                    }
                                    button {
                                        class: "admin-btn adm-row-del",
                                        disabled: busy,
                                        "data-pm-adm-del": del_attr_sk,
                                        "删除"
                                    }
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
                                th { "描述" }
                                th { "等级限制" }
                                th { "次数" }
                                th { "可学种族数" }
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
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-sk-new": "1",
                    "新增技能"
                }
            }
            ..vec![body, skill_editor_modal(st)]
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
                    let id_for_edit = id.clone();
                    let code = json_str(item, "code");
                    let kind = json_str(item, "kind");
                    let del_attr_ef = format!("ef:{id}");
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
                            td {
                                div { class: "adm-row-actions",
                                    button {
                                        class: "admin-btn adm-row-edit",
                                        disabled: busy,
                                        "data-pm-adm-efx-edit": id_for_edit,
                                        "编辑"
                                    }
                                    button {
                                        class: "admin-btn adm-row-del",
                                        disabled: busy,
                                        "data-pm-adm-del": del_attr_ef,
                                        "删除"
                                    }
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
                                th { "效果码" }
                                th { "类型" }
                                th { "钩子" }
                                th { "版本" }
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
                button {
                    class: "admin-btn admin-btn-primary",
                    disabled: busy,
                    "data-pm-adm-efx-new": "1",
                    "新增效果"
                }
            }
            ..vec![body, effect_editor_modal(st)]
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

// ---------------------------------------------------------------------------
// 技能编辑弹窗（切片 11）
// ---------------------------------------------------------------------------

/// 服务端唯一接受的三种技能类别（translate_skill_type_obj_to_raw）。
const SKILL_CATEGORIES: [(&str, &str); 3] = [
    ("physical_damage", "物攻"),
    ("special_damage", "特攻"),
    ("others", "其他"),
];

/// 从载荷里读出 (category, element 中文, power)。
/// effect 形态是 {"physical_damage": ["fire", 40]}。
fn skill_effect_parts(item: &serde_json::Value) -> (String, String, u64) {
    let Some(obj) = item.get("effect").and_then(|e| e.as_object()) else {
        return ("others".to_string(), String::new(), 0);
    };
    let Some((cat, value)) = obj.iter().next() else {
        return ("others".to_string(), String::new(), 0);
    };
    let Some(arr) = value.as_array() else {
        return (cat.clone(), String::new(), 0);
    };
    let elem_code = arr.first().and_then(|v| v.as_str()).unwrap_or("");
    let cn = kind_name(elem_code);
    let element = if cn == "？" { String::new() } else { cn.to_string() };
    let power = arr.get(1).and_then(|v| v.as_u64()).unwrap_or(0);
    (cat.clone(), element, power)
}

pub fn open_skill_new(state: &AppState) {
    let ed = state.skill_editor.clone();
    ed.editing_id.inner().set(None);
    ed.name.inner().set(String::new());
    ed.description.inner().set(String::new());
    ed.min_level_limit.inner().set("0".to_string());
    ed.use_times_limit.inner().set("0".to_string());
    ed.category.inner().set("physical_damage".to_string());
    ed.element.inner().set("普通".to_string());
    ed.power.inner().set("0".to_string());
    ed.available.inner().set(String::new());
    ed.effect_id.inner().set("0".to_string());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn open_skill_edit(state: &AppState, id: u64) {
    let ed = state.skill_editor.clone();
    let row = state
        .skill
        .rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    ed.editing_id.inner().set(Some(id));
    ed.name.inner().set(json_str(&row, "name"));
    ed.description.inner().set(json_str(&row, "description"));
    ed.min_level_limit
        .inner()
        .set(json_u64(&row, "min_level_limit").to_string());
    ed.use_times_limit
        .inner()
        .set(json_u64(&row, "use_times_limit").to_string());
    let (cat, element, power) = skill_effect_parts(&row);
    ed.category.inner().set(cat);
    ed.element.inner().set(element);
    ed.power.inner().set(power.to_string());
    let ids: Vec<String> = row
        .get("available_pokemons")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_u64())
                .map(|n| n.to_string())
                .collect()
        })
        .unwrap_or_default();
    ed.available.inner().set(ids.join(","));
    ed.effect_id
        .inner()
        .set(json_u64(&row, "effect_id").to_string());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_skill_editor(state: &AppState) {
    state.skill_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn set_skill_editor_field(state: &AppState, key: &str, value: &str) {
    let ed = state.skill_editor.clone();
    match key {
        "name" => ed.name.inner().set(value.to_string()),
        "description" => ed.description.inner().set(value.to_string()),
        "min_level_limit" => ed.min_level_limit.inner().set(value.to_string()),
        "use_times_limit" => ed.use_times_limit.inner().set(value.to_string()),
        "category" => ed.category.inner().set(value.to_string()),
        "element" => ed.element.inner().set(value.to_string()),
        "power" => ed.power.inner().set(value.to_string()),
        "available" => ed.available.inner().set(value.to_string()),
        "effect_id" => ed.effect_id.inner().set(value.to_string()),
        _ => {}
    }
    crate::request_remount();
}

pub fn submit_skill_editor(state: &AppState) {
    let ed = state.skill_editor.clone();
    if ed.saving.get() {
        return;
    }
    let name = ed.name.get().trim().to_string();
    if name.is_empty() {
        ed.err.inner().set(Some("名称不能为空".to_string()));
        crate::request_remount();
        return;
    }
    // 元素走中文 → 英文 → 服务端再翻回中文；非法值会导致服务端整条拒绝，
    // 所以先自己拦下来。
    let element_cn = ed.element.get();
    let element_cn = element_cn.trim();
    let Some(element_code) = reverse_kind_name(element_cn) else {
        ed.err
            .inner()
            .set(Some(format!("属性「{element_cn}」不是合法属性名")));
        crate::request_remount();
        return;
    };
    let category = ed.category.get();
    if !SKILL_CATEGORIES.iter().any(|(c, _)| *c == category) {
        ed.err.inner().set(Some("技能类别无效".to_string()));
        crate::request_remount();
        return;
    }
    // 可学种族：逗号分隔 ID，服务端会排序去重后写 "k,1,2,k"。
    let mut ids: Vec<u64> = Vec::new();
    for part in ed.available.get().split([',', '，', ' ', '\n']) {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
        match t.parse::<u64>() {
            Ok(n) => ids.push(n),
            Err(_) => {
                ed.err.inner().set(Some(format!("种族 ID「{t}」不是数字")));
                crate::request_remount();
                return;
            }
        }
    }
    let parse = |s: &str| s.trim().parse::<u64>().unwrap_or(0);
    let effect = serde_json::json!({ category: [element_code, parse(&ed.power.get())] });
    let mut payload = serde_json::json!({
        "name": name,
        "description": ed.description.get(),
        "min_level_limit": parse(&ed.min_level_limit.get()),
        "use_times_limit": parse(&ed.use_times_limit.get()),
        "available_pokemons": ids,
        "effect": effect,
        "effect_id": parse(&ed.effect_id.get()),
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
        let ed = st.skill_editor.clone();
        match result {
            Ok(_) => {
                ed.open.inner().set(false);
                ed.saving.inner().set(false);
                st.skill.loaded.inner().set(None);
                st.skill.rows.inner().set(Vec::new());
                crate::request_remount();
                reload_list(crate::state::SKILL_SPEC, &st.skill);
            }
            Err(e) => {
                ed.saving.inner().set(false);
                ed.err.inner().set(Some(format!("保存失败：{e}")));
                crate::request_remount();
            }
        }
    });
    if editing.is_some() {
        crate::api::set_entity("skill_type", body, on_done);
    } else {
        crate::api::insert_entity("skill_type", body, on_done);
    }
}

fn skill_editor_modal(st: &AppState) -> VNode {
    let ed = st.skill_editor.clone();
    if !ed.open.get() {
        return VNode::empty();
    }
    let editing = ed.editing_id.get();
    let title = match editing {
        Some(id) => format!("编辑技能 #{id}"),
        None => "新增技能".to_string(),
    };
    let saving = ed.saving.get();
    let err = ed.err.get();
    let category = ed.category.get();
    let save_label = if saving { "保存中..." } else { "保存" };

    let mut cat_options: Vec<VNode> = Vec::new();
    for (code, label) in SKILL_CATEGORIES {
        cat_options.push(rsx! {
            option {
                value: code,
                selected: code == category,
                "{label}"
            }
        });
    }

    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-sk-overlay": "1",
            div { class: "adm-editor-modal",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-sk-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    div { class: "adm-editor-grid",
                        label { class: "adm-editor-field",
                            span { "名称" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.name.get(),
                                "data-pm-adm-skf": "name"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "类别" }
                            select {
                                class: "adm-filter-input",
                                "data-pm-adm-skf": "category",
                                for i in 0..cat_options.len() {
                                    ..vec![cat_options[i].clone()]
                                }
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "属性（中文，如 火）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.element.get(),
                                "data-pm-adm-skf": "element"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "威力" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                value: ed.power.get(),
                                "data-pm-adm-skf": "power"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "等级限制" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                value: ed.min_level_limit.get(),
                                "data-pm-adm-skf": "min_level_limit"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "可用次数" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                value: ed.use_times_limit.get(),
                                "data-pm-adm-skf": "use_times_limit"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "效果模板 ID（0 = 不绑定）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "0",
                                value: ed.effect_id.get(),
                                "data-pm-adm-skf": "effect_id"
                            }
                        }
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "描述" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 2,
                            "data-pm-adm-skf": "description",
                            "{ed.description.get()}"
                        }
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "可学种族 ID（逗号分隔，如 25,26,27）" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 3,
                            "data-pm-adm-skf": "available",
                            "{ed.available.get()}"
                        }
                    }
                    ..vec![err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-sk-close": "1",
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn-primary",
                        disabled: saving,
                        "data-pm-adm-sk-save": "1",
                        "{save_label}"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 进化链编辑弹窗（切片 12）
// ---------------------------------------------------------------------------

/// 条件类型 → (码, 中文, 值输入提示, 值域种类)。
/// 值域种类决定提交时的类型转换：
/// - "num"：整数
/// - "float"：小数（random）
/// - "enum"：只能取固定取值（comp_atk_def / sex / chairs）
const EVO_COND_KINDS: [(&str, &str, &str, &str); 7] = [
    ("level", "等级达到", "等级（整数）", "num"),
    ("item", "使用道具", "道具 ID（整数）", "num"),
    ("good", "亲密度达到", "亲密度（整数）", "num"),
    (
        "comp_atk_def",
        "攻防比较",
        "greater / less / equal",
        "enum",
    ),
    ("random", "随机概率", "概率（0-1 小数，如 0.25）", "float"),
    ("sex", "性别", "male / female / unknown", "enum"),
    ("chairs", "背包有空位", "true / false", "enum"),
];

/// 从载荷的 condition 单键对象还原 (cond_kind, cond_value)。
fn evo_condition_parts(item: &serde_json::Value) -> (String, String) {
    let Some(cond) = item.get("condition").and_then(|c| c.as_object()) else {
        return ("level".to_string(), "0".to_string());
    };
    // 服务端 translator 的判定顺序：哪个键在就用哪个。
    if let Some(v) = cond.get("min_level") {
        return ("level".to_string(), v.to_string());
    }
    if let Some(v) = cond.get("use_item") {
        return ("item".to_string(), v.to_string());
    }
    if let Some(v) = cond.get("min_intimacy") {
        return ("good".to_string(), v.to_string());
    }
    if let Some(v) = cond.get("compare_attack_and_defense").and_then(|v| v.as_str()) {
        return ("comp_atk_def".to_string(), v.to_string());
    }
    if let Some(v) = cond.get("random") {
        return ("random".to_string(), v.to_string());
    }
    if let Some(v) = cond.get("sex").and_then(|v| v.as_str()) {
        return ("sex".to_string(), v.to_string());
    }
    if let Some(v) = cond.get("is_bag_have_chairs").and_then(|v| v.as_bool()) {
        return ("chairs".to_string(), if v { "true" } else { "false" }.to_string());
    }
    ("level".to_string(), "0".to_string())
}

pub fn open_evolution_new(state: &AppState) {
    let ed = state.evolution_editor.clone();
    ed.editing_id.inner().set(None);
    ed.source_id.inner().set(String::new());
    ed.target_id.inner().set(String::new());
    ed.priority.inner().set("0".to_string());
    ed.cond_kind.inner().set("level".to_string());
    ed.cond_value.inner().set("0".to_string());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn open_evolution_edit(state: &AppState, id: u64) {
    let ed = state.evolution_editor.clone();
    let row = state
        .evolution
        .rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    ed.editing_id.inner().set(Some(id));
    ed.source_id
        .inner()
        .set(json_u64(&row, "source_id").to_string());
    ed.target_id
        .inner()
        .set(json_u64(&row, "target_id").to_string());
    ed.priority
        .inner()
        .set(json_u64(&row, "priority").to_string());
    let (kind, value) = evo_condition_parts(&row);
    ed.cond_kind.inner().set(kind);
    ed.cond_value.inner().set(value);
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_evolution_editor(state: &AppState) {
    state.evolution_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn set_evolution_editor_field(state: &AppState, key: &str, value: &str) {
    let ed = state.evolution_editor.clone();
    match key {
        "source_id" => ed.source_id.inner().set(value.to_string()),
        "target_id" => ed.target_id.inner().set(value.to_string()),
        "priority" => ed.priority.inner().set(value.to_string()),
        "cond_kind" => ed.cond_kind.inner().set(value.to_string()),
        "cond_value" => ed.cond_value.inner().set(value.to_string()),
        _ => {}
    }
    crate::request_remount();
}

pub fn submit_evolution_editor(state: &AppState) {
    let ed = state.evolution_editor.clone();
    if ed.saving.get() {
        return;
    }
    let parse = |s: &str| s.trim().parse::<u64>().unwrap_or(0);
    let source_id = parse(&ed.source_id.get());
    let target_id = parse(&ed.target_id.get());
    // 孤儿行（from/to 为 0）是历史遗留，新建/编辑都必须给真实宠物 ID：
    // 服务端不校验，写 0 只会造出又一条查不到名字的记录。
    if source_id == 0 || target_id == 0 {
        ed.err
            .inner()
            .set(Some("进化来源与目标必须是有效的宠物 ID".to_string()));
        crate::request_remount();
        return;
    }
    if source_id == target_id {
        ed.err
            .inner()
            .set(Some("进化来源与目标不能是同一只宠物".to_string()));
        crate::request_remount();
        return;
    }
    let kind = ed.cond_kind.get();
    let value = ed.cond_value.get();
    let value = value.trim();
    let Some((_, _, _, value_kind)) = EVO_COND_KINDS.iter().find(|(c, _, _, _)| *c == kind) else {
        ed.err.inner().set(Some("条件类型无效".to_string()));
        crate::request_remount();
        return;
    };
    let condition = match *value_kind {
        "enum" => {
            let allowed: &[&str] = match kind.as_str() {
                "comp_atk_def" => &["greater", "less", "equal"],
                "sex" => &["male", "female", "unknown"],
                _ => &["true", "false"],
            };
            if !allowed.contains(&value) {
                ed.err.inner().set(Some(format!(
                    "该条件的取值必须是 {} 之一",
                    allowed.join(" / ")
                )));
                crate::request_remount();
                return;
            }
            // 服务端的键名与「条件类型」不是同名：compare_attack_and_defense
            // 与 is_bag_have_chairs 才是它认的键。
            match kind.as_str() {
                "comp_atk_def" => serde_json::json!({ "compare_attack_and_defense": value }),
                "sex" => serde_json::json!({ "sex": value }),
                _ => serde_json::json!({ "is_bag_have_chairs": value == "true" }),
            }
        }
        "float" => match value.parse::<f64>() {
            Ok(n) => serde_json::json!({ "random": n }),
            Err(_) => {
                ed.err.inner().set(Some("概率必须是小数（如 0.25）".to_string()));
                crate::request_remount();
                return;
            }
        },
        _ => {
            // 数值条件的服务端键名同样不同：level→min_level、
            // item→use_item、good→min_intimacy。
            let key = match kind.as_str() {
                "item" => "use_item",
                "good" => "min_intimacy",
                _ => "min_level",
            };
            match value.parse::<u64>() {
                Ok(n) => serde_json::json!({ key: n }),
                Err(_) => {
                    ed.err.inner().set(Some("该条件的值必须是整数".to_string()));
                    crate::request_remount();
                    return;
                }
            }
        }
    };
    let mut payload = serde_json::json!({
        "source_id": source_id,
        "target_id": target_id,
        "priority": parse(&ed.priority.get()),
        "condition": condition,
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
        let ed = st.evolution_editor.clone();
        match result {
            Ok(_) => {
                ed.open.inner().set(false);
                ed.saving.inner().set(false);
                st.evolution.loaded.inner().set(None);
                st.evolution.rows.inner().set(Vec::new());
                crate::request_remount();
                reload_list(crate::state::EVOLUTION_SPEC, &st.evolution);
            }
            Err(e) => {
                ed.saving.inner().set(false);
                ed.err.inner().set(Some(format!("保存失败：{e}")));
                crate::request_remount();
            }
        }
    });
    if editing.is_some() {
        crate::api::set_entity("evolution_info", body, on_done);
    } else {
        crate::api::insert_entity("evolution_info", body, on_done);
    }
}

fn evolution_editor_modal(st: &AppState) -> VNode {
    let ed = st.evolution_editor.clone();
    if !ed.open.get() {
        return VNode::empty();
    }
    let editing = ed.editing_id.get();
    let title = match editing {
        Some(id) => format!("编辑进化链 #{id}"),
        None => "新增进化链".to_string(),
    };
    let saving = ed.saving.get();
    let err = ed.err.get();
    let cond_kind = ed.cond_kind.get();
    let save_label = if saving { "保存中..." } else { "保存" };

    let mut kind_options: Vec<VNode> = Vec::new();
    for (code, label, _, _) in EVO_COND_KINDS {
        kind_options.push(rsx! {
            option {
                value: code,
                selected: code == cond_kind,
                "{label}"
            }
        });
    }
    let placeholder = EVO_COND_KINDS
        .iter()
        .find(|(c, _, _, _)| *c == cond_kind)
        .map(|(_, _, hint, _)| *hint)
        .unwrap_or("");

    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-ev-overlay": "1",
            div { class: "adm-editor-modal",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-ev-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    div { class: "adm-editor-grid",
                        label { class: "adm-editor-field",
                            span { "进化来源（宠物 ID）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "1",
                                value: ed.source_id.get(),
                                "data-pm-adm-evf": "source_id"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "进化目标（宠物 ID）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "1",
                                value: ed.target_id.get(),
                                "data-pm-adm-evf": "target_id"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "优先级" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                value: ed.priority.get(),
                                "data-pm-adm-evf": "priority"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "条件类型" }
                            select {
                                class: "adm-filter-input",
                                "data-pm-adm-evf": "cond_kind",
                                for i in 0..kind_options.len() {
                                    ..vec![kind_options[i].clone()]
                                }
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "{placeholder}" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.cond_value.get(),
                                "data-pm-adm-evf": "cond_value"
                            }
                        }
                    }
                    ..vec![err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-ev-close": "1",
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn-primary",
                        disabled: saving,
                        "data-pm-adm-ev-save": "1",
                        "{save_label}"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 用户发放弹窗（切片 13）：发放宠物 / 发放物品
// ---------------------------------------------------------------------------

/// 品质档位阈值（与 `_utils::functions::quality` 及 PHP `pm_quality_thresholds`
/// 同一套阶梯：E 0-9 ... SS 170-185、SS+ 恰好 186）。
const GRANT_QUALITY_THRESHOLDS: [u64; 8] = [0, 10, 70, 100, 120, 150, 170, 186];
const GRANT_QUALITY_GRADES: [&str; 8] = ["E", "D", "C", "B", "A", "S", "SS", "SS+"];
const GRANT_IV_TOTAL_MAX: u64 = 186;

/// 伪随机数：没有导入 crypto 接口（glue 未提供该域），用时钟低位做种子。
/// 这里只用于「发放宠物时掷个体值」，不承担任何安全职责。
fn grant_random_below(bound: u64) -> u64 {
    if bound <= 1 {
        return 0;
    }
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<u64> = const { Cell::new(0) };
    }
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9e3779b97f4a7c15);
    let next = SEED.with(|s| {
        let mut x = s.get() ^ nanos ^ 0x9e3779b97f4a7c15;
        // xorshift64*
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        s.set(x);
        x.wrapping_mul(0x2545F4914F6CDD1D)
    });
    next % bound
}

/// 掷出一组落在指定品质档位内的六项个体值（0-31）。
///
/// 与 `_utils::functions::quality::roll_for_rank` / PHP `pm_roll_ivs_for_rank`
/// 同一策略：档位区间内取目标总和 → 均分六项 → 保持总和随机搬移。
fn grant_roll_ivs_for_rank(rank: u8) -> [u64; 6] {
    let rank = rank.clamp(1, 8);
    let min = GRANT_QUALITY_THRESHOLDS[(rank - 1) as usize];
    let max = if rank == 8 {
        GRANT_IV_TOTAL_MAX
    } else {
        GRANT_QUALITY_THRESHOLDS[rank as usize] - 1
    };
    let target = min + grant_random_below(max - min + 1);
    let mut values = [target / 6; 6];
    for v in values.iter_mut().take((target % 6) as usize) {
        *v += 1;
    }
    for _ in 0..24 {
        let from = grant_random_below(6) as usize;
        let to = grant_random_below(6) as usize;
        if from == to {
            continue;
        }
        let delta = 1 + grant_random_below(4);
        if values[from] < delta || values[to] + delta > 31 {
            continue;
        }
        values[from] -= delta;
        values[to] += delta;
    }
    values
}

/// 随机 0-31（与野怪捕捉一致）。
fn grant_random_ivs() -> [u64; 6] {
    std::array::from_fn(|_| grant_random_below(32))
}

pub fn open_grant(state: &AppState, uid: u64, mode: &str) {
    let g = state.grant.clone();
    g.open.inner().set(true);
    g.mode.inner().set(mode.to_string());
    g.owner_uid.inner().set(uid);
    g.step.inner().set(1);
    g.query.inner().set(String::new());
    g.results.inner().set(Vec::new());
    g.searching.inner().set(false);
    g.picked_id.inner().set(None);
    g.picked_name.inner().set(String::new());
    g.poke.inner().set(crate::state::GrantPokeDraft::blank());
    g.item_count.inner().set("1".to_string());
    g.err.inner().set(None);
    crate::request_remount();
}

pub fn close_grant(state: &AppState) {
    let g = state.grant.clone();
    if g.saving.get() {
        return;
    }
    g.open.inner().set(false);
    crate::request_remount();
}

pub fn set_grant_query(state: &AppState, query: &str) {
    let g = state.grant.clone();
    g.query.inner().set(query.to_string());
    crate::request_remount();
}

/// 按名称搜类型（宠物类型 / 物品类型）。
///
/// 复用 `filter::<entity>` 的「名称 contains」筛选——Dioxus 版用的是
/// 名称 equal，但 contains 更适合边输边搜，且服务端语义一致。
pub fn search_grant_types(state: &AppState) {
    let g = state.grant.clone();
    if g.searching.get() {
        return;
    }
    let query = g.query.get().trim().to_string();
    if query.is_empty() {
        g.results.inner().set(Vec::new());
        crate::request_remount();
        return;
    }
    let entity = if g.mode.get() == "item" {
        "item_type"
    } else {
        "pokemon_type"
    };
    let packages = vec![crate::api::filter_package("名称", "contains", &query)];
    let filters = serde_json::Value::Array(packages).to_string();
    g.searching.inner().set(true);
    crate::request_remount();
    let st = state.clone();
    crate::api::filter_entity(
        entity,
        filters,
        Box::new(move |result: Result<Vec<serde_json::Value>, String>| {
            let g = st.grant.clone();
            match result {
                Ok(items) => {
                    let rows: Vec<(u64, String)> = items
                        .iter()
                        .map(|it| (json_u64(it, "id"), json_str(it, "name")))
                        .collect();
                    g.results.inner().set(rows);
                    g.err.inner().set(None);
                }
                Err(e) => {
                    g.results.inner().set(Vec::new());
                    g.err.inner().set(Some(format!("搜索失败：{e}")));
                }
            }
            g.searching.inner().set(false);
            crate::request_remount();
        }),
    );
}

pub fn pick_grant_type(state: &AppState, id: u64) {
    let g = state.grant.clone();
    let name = g
        .results
        .get()
        .into_iter()
        .find(|(rid, _)| *rid == id)
        .map(|(_, name)| name)
        .unwrap_or_default();
    g.picked_id.inner().set(Some(id));
    g.picked_name.inner().set(name);
    g.err.inner().set(None);
    crate::request_remount();
}

pub fn grant_go_step(state: &AppState, step: u8) {
    let g = state.grant.clone();
    if step == 2 && g.picked_id.get().is_none() {
        g.err.inner().set(Some("请先选择一个类型".to_string()));
        crate::request_remount();
        return;
    }
    g.step.inner().set(step);
    g.err.inner().set(None);
    crate::request_remount();
}

/// 宠物表单字段变更。`key` 形如 `site` / `iv:3` / `bp:0`。
pub fn set_grant_field(state: &AppState, key: &str, value: &str) {
    let g = state.grant.clone();
    let mut d = g.poke.get();
    match key {
        "name" => d.name = value.to_string(),
        "level" => d.level = value.to_string(),
        "site" => d.site = value.to_string(),
        "sex" => d.sex = value.to_string(),
        "intimacy" => d.intimacy = value.to_string(),
        "iv_mode" => d.iv_mode = value.to_string(),
        "iv_quality" => d.iv_quality = value.to_string(),
        "item_count" => g.item_count.inner().set(value.to_string()),
        _ => {
            if let Some((group, idx)) = key.split_once(':') {
                if let Ok(i) = idx.parse::<usize>() {
                    if i < 6 {
                        if group == "iv" {
                            d.ivs[i] = value.to_string();
                        } else if group == "bp" {
                            d.base_points[i] = value.to_string();
                        }
                    }
                }
            }
        }
    }
    g.poke.inner().set(d);
    crate::request_remount();
}

pub fn toggle_grant_shiny(state: &AppState) {
    let g = state.grant.clone();
    let mut d = g.poke.get();
    d.is_shiny = !d.is_shiny;
    g.poke.inner().set(d);
    crate::request_remount();
}

/// 提交发放。
pub fn submit_grant(state: &AppState) {
    let g = state.grant.clone();
    if g.saving.get() {
        return;
    }
    let Some(type_id) = g.picked_id.get() else {
        g.err.inner().set(Some("请先选择一个类型".to_string()));
        crate::request_remount();
        return;
    };
    let uid = g.owner_uid.get();
    if uid == 0 {
        g.err.inner().set(Some("用户 UID 无效".to_string()));
        crate::request_remount();
        return;
    }
    let parse = |s: &str| s.trim().parse::<u64>().unwrap_or(0);
    if g.mode.get() == "item" {
        // 服务端把 nums 当 SMALLINT，1..=32767；提前拦比让它报错更清楚。
        let count = parse(&g.item_count.get());
        if count < 1 || count > 32767 {
            g.err
                .inner()
                .set(Some("数量必须是 1 至 32767 的整数".to_string()));
            crate::request_remount();
            return;
        }
        let body = serde_json::json!({
            "owner": uid,
            "type_id": type_id,
            "count": count,
        })
        .to_string();
        g.saving.inner().set(true);
        g.err.inner().set(None);
        crate::request_remount();
        let st = state.clone();
        crate::api::insert_entity(
            "item_info",
            body,
            Box::new(move |result: Result<serde_json::Value, String>| {
                let g = st.grant.clone();
                g.saving.inner().set(false);
                match result {
                    Ok(saved) => {
                        g.open.inner().set(false);
                        let id = json_u64(&saved, "id");
                        crate::state::show_toast_silent(
                            &st.config,
                            ToastKind::Success,
                            format!("已为用户 #{uid} 发放物品 #{id}"),
                        );
                        st.user.loaded.inner().set(None);
                        st.user.rows.inner().set(Vec::new());
                        crate::request_remount();
                        reload_list(crate::state::USER_SPEC, &st.user);
                    }
                    Err(e) => {
                        g.err.inner().set(Some(format!("发放失败：{e}")));
                        crate::request_remount();
                    }
                }
            }),
        );
        return;
    }

    let d = g.poke.get();
    let level = parse(&d.level);
    if level < 1 || level > 100 {
        g.err
            .inner()
            .set(Some("等级必须在 1 至 100 之间".to_string()));
        crate::request_remount();
        return;
    }
    let intimacy = parse(&d.intimacy);
    if intimacy > 100 {
        g.err.inner().set(Some("亲密度必须在 0 至 100".to_string()));
        crate::request_remount();
        return;
    }
    let ivs: [u64; 6] = match d.iv_mode.as_str() {
        "quality" => {
            let rank = parse(&d.iv_quality).clamp(1, 8) as u8;
            grant_roll_ivs_for_rank(rank)
        }
        "manual" => std::array::from_fn(|i| parse(&d.ivs[i]).min(31)),
        _ => grant_random_ivs(),
    };
    let base: [u64; 6] = std::array::from_fn(|i| parse(&d.base_points[i]).min(3));
    // 昵称留空时服务端会退化成类型名（insert_pokemon_info 里 nowname 直接用），
    // 所以这里显式带上类型名，避免出现空名字的宠物。
    let name = {
        let typed = d.name.trim();
        if typed.is_empty() {
            g.picked_name.get()
        } else {
            typed.to_string()
        }
    };
    let stat_obj = |keys: [&str; 6], values: [u64; 6]| {
        let mut map = serde_json::Map::new();
        for i in 0..6 {
            map.insert(keys[i].to_string(), serde_json::json!(values[i]));
        }
        serde_json::Value::Object(map)
    };
    let body = serde_json::json!({
        "id": 0,
        "type_id": type_id,
        "owner": uid,
        "name": name,
        "site": d.site,
        "level": level,
        "experience": 0,
        "intimacy": intimacy,
        "using_ball_id": 1,
        "is_shiny": d.is_shiny,
        "status": "normal",
        "sex": d.sex,
        "statistic": stat_obj(STAT_KEYS, ivs),
        "base_points": stat_obj(STAT_KEYS, base),
        "skills": [],
        "armor_slots_id": [serde_json::Value::Null, serde_json::Value::Null, serde_json::Value::Null, serde_json::Value::Null],
    })
    .to_string();
    g.saving.inner().set(true);
    g.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::insert_entity(
        "pokemon_info",
        body,
        Box::new(move |result: Result<serde_json::Value, String>| {
            let g = st.grant.clone();
            g.saving.inner().set(false);
            match result {
                Ok(saved) => {
                    g.open.inner().set(false);
                    let id = json_u64(&saved, "id");
                    crate::state::show_toast_silent(
                        &st.config,
                        ToastKind::Success,
                        format!("已为用户 #{uid} 发放宠物 #{id}"),
                    );
                    st.user.loaded.inner().set(None);
                    st.user.rows.inner().set(Vec::new());
                    crate::request_remount();
                    reload_list(crate::state::USER_SPEC, &st.user);
                }
                Err(e) => {
                    g.err.inner().set(Some(format!("发放失败：{e}")));
                    crate::request_remount();
                }
            }
        }),
    );
}

fn grant_modal(st: &AppState) -> VNode {
    let g = st.grant.clone();
    if !g.open.get() {
        return VNode::empty();
    }
    let mode = g.mode.get();
    let is_item = mode == "item";
    let uid = g.owner_uid.get();
    let step = g.step.get();
    let saving = g.saving.get();
    let err = g.err.get();
    let picked = g.picked_id.get();
    let picked_name = g.picked_name.get();
    let query = g.query.get();
    let searching = g.searching.get();
    let title = if is_item {
        format!("发放物品给用户 #{uid}")
    } else {
        format!("发放宠物给用户 #{uid}")
    };
    let noun = if is_item { "物品" } else { "宠物" };
    let save_label = if saving { "发放中..." } else { "发放" };

    // 第一步：搜索 + 结果列表
    let step1 = {
        let mut result_rows: Vec<VNode> = Vec::new();
        for (rid, rname) in g.results.get().into_iter() {
            let rid_text = rid.to_string();
            let cls = if picked == Some(rid) {
                "admin-btn admin-btn-primary adm-grant-result"
            } else {
                "admin-btn adm-grant-result"
            };
            result_rows.push(rsx! {
                button {
                    class: cls,
                    disabled: saving,
                    "data-pm-adm-grant-pick": rid_text,
                    "{rname}"
                }
            });
        }
        let results_block = if searching {
            rsx! { p { class: "adm-editor-hint", "搜索中..." } }
        } else if result_rows.is_empty() {
            let hint = format!("输入名称后点「搜索」查找{noun}类型");
            rsx! { p { class: "adm-editor-hint", "{hint}" } }
        } else {
            rsx! {
                div { class: "adm-grant-results",
                    for i in 0..result_rows.len() {
                        ..vec![result_rows[i].clone()]
                    }
                }
            }
        };
        let step1_parts: Vec<VNode> = vec![
            rsx! {
                div { class: "adm-editor-grid",
                    label { class: "adm-editor-field",
                        span { "名称（包含）" }
                        input {
                            class: "adm-filter-input",
                            r#type: "text",
                            value: query,
                            "data-pm-adm-grantf": "query"
                        }
                    }
                    button {
                        class: "admin-btn adm-editor-toggle",
                        disabled: saving || searching,
                        "data-pm-adm-grant-search": "1",
                        "搜索"
                    }
                }
            },
            results_block,
        ];
        rsx! {
            for i in 0..step1_parts.len() {
                ..vec![step1_parts[i].clone()]
            }
        }
    };

    // 第二步：物品只要数量；宠物要一整套属性
    let step2 = if is_item {
        rsx! {
            div { class: "adm-editor-grid",
                label { class: "adm-editor-field",
                    span { "数量（1-32767）" }
                    input {
                        class: "adm-filter-input",
                        r#type: "number",
                        min: "1",
                        max: "32767",
                        value: g.item_count.get(),
                        "data-pm-adm-grantf": "item_count"
                    }
                }
            }
        }
    } else {
        let d = g.poke.get();
        let mut iv_inputs: Vec<VNode> = Vec::new();
        for i in 0..6 {
            let label = STAT_LABELS[i];
            let attr = format!("iv:{i}");
            let v = d.ivs[i].clone();
            iv_inputs.push(rsx! {
                label { class: "adm-editor-field",
                    span { "{label}" }
                    input {
                        class: "adm-filter-input",
                        r#type: "number",
                        min: "0",
                        max: "31",
                        value: v,
                        "data-pm-adm-grantf": attr
                    }
                }
            });
        }
        let iv_block = if d.iv_mode == "manual" {
            rsx! {
                div { class: "adm-editor-section-title", "个体值（0-31）" }
                div { class: "adm-editor-grid",
                    for i in 0..iv_inputs.len() {
                        ..vec![iv_inputs[i].clone()]
                    }
                }
            }
        } else {
            VNode::empty()
        };
        let quality_block = if d.iv_mode == "quality" {
            let mut opts: Vec<VNode> = Vec::new();
            for (i, grade) in GRANT_QUALITY_GRADES.iter().enumerate() {
                let code = (i + 1).to_string();
                let selected = d.iv_quality == code;
                let label = *grade;
                opts.push(rsx! {
                    option { value: code, selected: selected, "{label}" }
                });
            }
            rsx! {
                label { class: "adm-editor-field",
                    span { "品质档位（结果落在该档内）" }
                    select {
                        class: "adm-filter-input",
                        "data-pm-adm-grantf": "iv_quality",
                        for i in 0..opts.len() {
                            ..vec![opts[i].clone()]
                        }
                    }
                }
            }
        } else {
            VNode::empty()
        };
        let mut mode_opts: Vec<VNode> = Vec::new();
        for (code, label) in [
            ("random", "随机 0-31（同野怪捕捉）"),
            ("quality", "按品质档位随机"),
            ("manual", "手动指定六项"),
        ] {
            let selected = d.iv_mode == code;
            mode_opts.push(rsx! {
                option { value: code, selected: selected, "{label}" }
            });
        }
        let mut site_opts: Vec<VNode> = Vec::new();
        for (code, label) in [
            ("store", "仓库"),
            ("bag", "背包"),
            ("header", "跟随玩家"),
        ] {
            let selected = d.site == code;
            site_opts.push(rsx! {
                option { value: code, selected: selected, "{label}" }
            });
        }
        let mut sex_opts: Vec<VNode> = Vec::new();
        for (code, label) in [("unknown", "未知"), ("male", "雄性"), ("female", "雌性")] {
            let selected = d.sex == code;
            sex_opts.push(rsx! {
                option { value: code, selected: selected, "{label}" }
            });
        }
        let shiny_label = if d.is_shiny { "闪光：是" } else { "闪光：否" };
        let shiny_cls = if d.is_shiny {
            "admin-btn admin-btn-primary adm-editor-toggle"
        } else {
            "admin-btn adm-editor-toggle"
        };
        let step2_parts: Vec<VNode> = vec![
            rsx! {
                div { class: "adm-editor-grid",
                    label { class: "adm-editor-field",
                        span { "昵称（留空用类型名）" }
                        input {
                            class: "adm-filter-input",
                            r#type: "text",
                            value: d.name.clone(),
                            "data-pm-adm-grantf": "name"
                        }
                    }
                    label { class: "adm-editor-field",
                        span { "等级（1-100）" }
                        input {
                            class: "adm-filter-input",
                            r#type: "number",
                            min: "1",
                            max: "100",
                            value: d.level.clone(),
                            "data-pm-adm-grantf": "level"
                        }
                    }
                    label { class: "adm-editor-field",
                        span { "位置" }
                        select {
                            class: "adm-filter-input",
                            "data-pm-adm-grantf": "site",
                            for i in 0..site_opts.len() {
                                ..vec![site_opts[i].clone()]
                            }
                        }
                    }
                    label { class: "adm-editor-field",
                        span { "性别" }
                        select {
                            class: "adm-filter-input",
                            "data-pm-adm-grantf": "sex",
                            for i in 0..sex_opts.len() {
                                ..vec![sex_opts[i].clone()]
                            }
                        }
                    }
                    label { class: "adm-editor-field",
                        span { "亲密度（0-100）" }
                        input {
                            class: "adm-filter-input",
                            r#type: "number",
                            min: "0",
                            max: "100",
                            value: d.intimacy.clone(),
                            "data-pm-adm-grantf": "intimacy"
                        }
                    }
                    button {
                        class: shiny_cls,
                        disabled: saving,
                        "data-pm-adm-grant-shiny": "1",
                        "{shiny_label}"
                    }
                    label { class: "adm-editor-field",
                        span { "个体值模式" }
                        select {
                            class: "adm-filter-input",
                            "data-pm-adm-grantf": "iv_mode",
                            for i in 0..mode_opts.len() {
                                ..vec![mode_opts[i].clone()]
                            }
                        }
                    }
                    ..vec![quality_block]
                }
            },
            iv_block,
        ];
        rsx! {
            for i in 0..step2_parts.len() {
                ..vec![step2_parts[i].clone()]
            }
        }
    };

    let picked_bar = match picked {
        Some(id) => {
            let text = format!("已选：#{id} {picked_name}");
            rsx! {
                div { class: "adm-grant-picked",
                    span { class: "adm-editor-hint", "{text}" }
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-grant-step": "1",
                        "重新选择"
                    }
                }
            }
        }
        None => VNode::empty(),
    };

    let body_parts: Vec<VNode> = if step == 1 {
        vec![step1]
    } else {
        vec![picked_bar, step2]
    };
    let body = rsx! {
        for i in 0..body_parts.len() {
            ..vec![body_parts[i].clone()]
        }
    };

    let next_btn = if step == 1 {
        rsx! {
            button {
                class: "admin-btn admin-btn-primary",
                disabled: saving,
                "data-pm-adm-grant-step": "2",
                "下一步"
            }
        }
    } else {
        rsx! {
            button {
                class: "admin-btn admin-btn-primary",
                disabled: saving,
                "data-pm-adm-grant-submit": "1",
                "{save_label}"
            }
        }
    };

    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };
    let step_hint = if step == 1 {
        format!("第 1 步 / 共 2 步：选择{noun}类型")
    } else {
        format!("第 2 步 / 共 2 步：设置发放内容")
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-grant-overlay": "1",
            div { class: "adm-editor-modal",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-grant-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    p { class: "adm-editor-hint", "{step_hint}" }
                    ..vec![body, err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-grant-close": "1",
                        "取消"
                    }
                    ..vec![next_btn]
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 全局配置分节表单（切片 14）
// ---------------------------------------------------------------------------

/// 配置字段的类型。表单按类型渲染对应控件。
#[derive(Clone, Copy, PartialEq)]
enum CfgKind {
    Bool,
    Num,
    Text,
    /// 分节标题（不是字段）。
    Section(&'static str),
}

/// (键, 标签, 类型, 说明)。键就是 `pm_config.key`；顺序即渲染顺序。
///
/// 只列表单建模的键——不在表里的键在保存时原样保留（Dioxus 版用强类型
/// `GlobalConfigType` 反序列化，`is_enable_pvp` / `pvp_xp_multiple` /
/// `cancel_pvp_price` / `skill_rules_json` 这些 DB 里真实存在的键会被
/// 悄悄丢掉；按实际键读写避开了这个问题）。
const CONFIG_FIELDS: &[(&str, &str, CfgKind, &str)] = &[
    ("", "基础设定", CfgKind::Section("基础设定"), ""),
    ("is_open", "是否开放", CfgKind::Bool, "插件全局开关"),
    ("ann_title", "页眉公告标题", CfgKind::Text, ""),
    ("ann_url", "页眉公告链接", CfgKind::Text, "https://..."),
    ("", "亲密度设定", CfgKind::Section("亲密度设定"), ""),
    (
        "intimacy_increase_per_hour",
        "每小时亲密度增加量",
        CfgKind::Num,
        "0 时随机增加 1-10",
    ),
    (
        "intimacy_increase_per_earn_xp",
        "每次获得经验亲密度增加量",
        CfgKind::Num,
        "关闭宠物蛋模式时也不应设置为 0",
    ),
    (
        "intimacy_increase_multiple",
        "亲密度增加倍率",
        CfgKind::Num,
        "填 0 时宠物蛋无法正常孵化",
    ),
    ("", "治疗设定", CfgKind::Section("治疗设定"), ""),
    ("revive_time", "宠物复活所需时间", CfgKind::Num, "单位：分钟"),
    ("medical_price", "宠物单次治疗费用", CfgKind::Num, ""),
    (
        "posts_count_for_wakeup",
        "宠物复活所需回帖数量",
        CfgKind::Num,
        "",
    ),
    ("", "捕捉设定", CfgKind::Section("捕捉设定"), ""),
    ("is_enable_catch", "是否开放捕捉", CfgKind::Bool, ""),
    (
        "catch_price_in_catch_area",
        "捕捉区域进入费用",
        CfgKind::Num,
        "",
    ),
    (
        "pve_catch_xp_multiple",
        "捕捉经验倍率",
        CfgKind::Num,
        "填 0 采用地图经验倍数设置",
    ),
    (
        "pve_catch_level",
        "可捕捉等级",
        CfgKind::Num,
        "填 0 采用各宠物自己的设置",
    ),
    ("", "PVE 设定", CfgKind::Section("PVE 设定"), ""),
    (
        "is_enable_earn_xp_on_pve",
        "是否开放战斗获得经验",
        CfgKind::Bool,
        "关闭后无法获得战斗经验",
    ),
    ("drop_money_on_pve", "是否掉落金钱", CfgKind::Bool, ""),
    (
        "drop_money_config_by_global",
        "金钱掉落配置来源",
        CfgKind::Bool,
        "否则以宠物各自设置为准",
    ),
    (
        "drop_money_percent_min_on_pve",
        "金钱掉落下限（全局）",
        CfgKind::Num,
        "以全局配置为准时生效",
    ),
    (
        "drop_money_percent_max_on_pve",
        "金钱掉落上限（全局）",
        CfgKind::Num,
        "",
    ),
    (
        "drop_money_level_gap",
        "等级差距上限",
        CfgKind::Num,
        "高出超过此级数不掉落宠物币；0 = 不限制",
    ),
    (
        "drop_money_daily_limit",
        "每日宠物币上限",
        CfgKind::Num,
        "0 = 不限制",
    ),
    ("", "PVP 设定", CfgKind::Section("PVP 设定"), ""),
    ("is_enable_pvp", "是否开放 PVP", CfgKind::Bool, ""),
    (
        "is_enable_earn_xp_on_pvp",
        "PVP 是否获得经验",
        CfgKind::Bool,
        "",
    ),
    ("pvp_xp_multiple", "PVP 经验倍率", CfgKind::Num, ""),
    ("cancel_pvp_price", "取消 PVP 费用", CfgKind::Num, ""),
    ("", "宠物蛋设定", CfgKind::Section("宠物蛋设定"), ""),
    (
        "is_enable_egg",
        "是否开放宠物蛋直接领取",
        CfgKind::Bool,
        "",
    ),
    ("is_enable_buy_egg", "是否允许购买宠物蛋", CfgKind::Bool, ""),
    (
        "is_enable_buy_pokemon",
        "是否允许购买宠物",
        CfgKind::Bool,
        "",
    ),
    (
        "is_enable_multiple_eggs",
        "是否允许持有多个宠物蛋",
        CfgKind::Bool,
        "仅对免费领取有效",
    ),
    ("egg_price", "宠物蛋购买费用", CfgKind::Num, ""),
    (
        "hatch_egg_intimacy",
        "孵化所需最低亲密度",
        CfgKind::Num,
        "最大 255；小于 50 则直接孵化",
    ),
];

/// 单个配置值的显示文本（布尔用 "1"/"" 与 DB 一致）。
fn config_value_text(config: &serde_json::Value, key: &str) -> String {
    match config.get(key) {
        Some(serde_json::Value::Bool(b)) => {
            if *b {
                "1".to_string()
            } else {
                String::new()
            }
        }
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::Null) | None => String::new(),
        Some(other) => other.to_string(),
    }
}

fn config_value_bool(config: &serde_json::Value, key: &str) -> bool {
    match config.get(key) {
        Some(serde_json::Value::Bool(b)) => *b,
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        Some(serde_json::Value::String(s)) => {
            matches!(s.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on")
        }
        _ => false,
    }
}

/// 改一个配置值并重建 draft 文本（dirty 判定依赖 draft 与 saved 的差异）。
pub fn set_config_field(state: &AppState, key: &str, value: &str, checked: bool) {
    let cfg = state.config.clone();
    let Ok(mut parsed) = serde_json::from_str::<serde_json::Value>(&cfg.draft.get()) else {
        return;
    };
    let Some(obj) = parsed.as_object_mut() else {
        return;
    };
    let is_bool = matches!(obj.get(key), Some(serde_json::Value::Bool(_)));
    if is_bool || CONFIG_FIELDS
        .iter()
        .any(|(k, _, kind, _)| *k == key && *kind == CfgKind::Bool)
    {
        // 布尔沿用 DB 的 "1"/"" 文本形态：服务端 data_type 是 string 的
        // 键也只认这种写法。
        obj.insert(
            key.to_string(),
            serde_json::Value::String(if checked { "1" } else { "" }.to_string()),
        );
    } else {
        obj.insert(
            key.to_string(),
            serde_json::Value::String(value.to_string()),
        );
    }
    cfg.draft
        .inner()
        .set(serde_json::to_string_pretty(&parsed).unwrap_or_default());
    crate::request_remount();
}

pub fn toggle_config_bool(state: &AppState, key: &str) {
    let cfg = state.config.clone();
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&cfg.draft.get()) else {
        return;
    };
    let next = !config_value_bool(&parsed, key);
    set_config_field(state, key, if next { "1" } else { "" }, next);
}

pub fn toggle_config_raw_mode(state: &AppState) {
    let cf = state.config_form.clone();
    cf.raw_mode.inner().set(!cf.raw_mode.get());
    crate::request_remount();
}

// -- 新闻公告列表编辑器 ------------------------------------------------------

fn announcements_of(config: &serde_json::Value) -> Vec<serde_json::Value> {
    match config.get("news_announcements") {
        Some(serde_json::Value::Array(items)) => items.clone(),
        // 服务端可能把 JSON 存成字符串（DB 里就是 "[]"）。
        Some(serde_json::Value::String(s)) => serde_json::from_str::<Vec<serde_json::Value>>(s)
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn write_announcements(state: &AppState, items: Vec<serde_json::Value>) {
    let cfg = state.config.clone();
    let Ok(mut parsed) = serde_json::from_str::<serde_json::Value>(&cfg.draft.get()) else {
        return;
    };
    if let Some(obj) = parsed.as_object_mut() {
        // 写成紧凑 JSON 字符串：DB 里这一列是 string，服务端按字符串接收。
        obj.insert(
            "news_announcements".to_string(),
            serde_json::Value::String(serde_json::to_string(&items).unwrap_or_else(|_| "[]".into())),
        );
    }
    cfg.draft
        .inner()
        .set(serde_json::to_string_pretty(&parsed).unwrap_or_default());
    crate::request_remount();
}

pub fn open_announcement(state: &AppState, index: Option<usize>) {
    let cf = state.config_form.clone();
    let cfg = state.config.clone();
    let items = match serde_json::from_str::<serde_json::Value>(&cfg.draft.get()) {
        Ok(v) => announcements_of(&v),
        Err(_) => Vec::new(),
    };
    match index {
        Some(i) => {
            let item = items.get(i).cloned().unwrap_or_default();
            cf.ann_title.inner().set(json_str(&item, "title"));
            cf.ann_url.inner().set(json_str(&item, "url"));
        }
        None => {
            cf.ann_title.inner().set(String::new());
            cf.ann_url.inner().set(String::new());
        }
    }
    cf.ann_editing.inner().set(index);
    cf.ann_open.inner().set(true);
    crate::request_remount();
}

pub fn close_announcement(state: &AppState) {
    let cf = state.config_form.clone();
    cf.ann_open.inner().set(false);
    crate::request_remount();
}

pub fn set_announcement_field(state: &AppState, key: &str, value: &str) {
    let cf = state.config_form.clone();
    match key {
        "title" => cf.ann_title.inner().set(value.to_string()),
        "url" => cf.ann_url.inner().set(value.to_string()),
        _ => {}
    }
    crate::request_remount();
}

pub fn save_announcement(state: &AppState) {
    let cf = state.config_form.clone();
    let cfg = state.config.clone();
    let title = cf.ann_title.get().trim().to_string();
    if title.is_empty() {
        crate::state::show_toast_silent(&cfg, ToastKind::Error, "公告标题不能为空".to_string());
        crate::request_remount();
        return;
    }
    let Ok(draft) = serde_json::from_str::<serde_json::Value>(&cfg.draft.get()) else {
        return;
    };
    let mut items = announcements_of(&draft);
    let entry = serde_json::json!({ "title": title, "url": cf.ann_url.get().trim() });
    match cf.ann_editing.get() {
        Some(i) if i < items.len() => items[i] = entry,
        Some(_) => items.push(entry),
        None => {
            // Dioxus 版上限 6 条，这里保持一致。
            if items.len() >= 6 {
                crate::state::show_toast_silent(
                    &cfg,
                    ToastKind::Error,
                    "最多只能配置 6 条公告".to_string(),
                );
                crate::request_remount();
                return;
            }
            items.push(entry);
        }
    }
    write_announcements(state, items);
    cf.ann_open.inner().set(false);
    crate::request_remount();
}

pub fn remove_announcement(state: &AppState, index: usize) {
    let cfg = state.config.clone();
    let Ok(draft) = serde_json::from_str::<serde_json::Value>(&cfg.draft.get()) else {
        return;
    };
    let mut items = announcements_of(&draft);
    if index < items.len() {
        items.remove(index);
        write_announcements(state, items);
    }
}

fn config_form_body(st: &AppState) -> VNode {
    let cfg = st.config.clone();
    let cf = st.config_form.clone();
    let Ok(config) = serde_json::from_str::<serde_json::Value>(&cfg.draft.get()) else {
        return rsx! {
            p { class: "admin-empty-hint", "配置内容不是合法 JSON，请切到原始 JSON 模式修正。" }
        };
    };

    let mut blocks: Vec<VNode> = Vec::new();
    let mut section: Vec<VNode> = Vec::new();
    let mut current_title: Option<&'static str> = None;

    let flush = |section: &mut Vec<VNode>, title: Option<&'static str>, blocks: &mut Vec<VNode>| {
        if let Some(t) = title {
            let rows: Vec<VNode> = section.drain(..).collect();
            blocks.push(rsx! {
                div { class: "adm-cfg-section",
                    div { class: "adm-cfg-section-title", "{t}" }
                    div { class: "adm-cfg-grid",
                        for i in 0..rows.len() {
                            ..vec![rows[i].clone()]
                        }
                    }
                }
            });
        }
    };

    for (key, label, kind, hint) in CONFIG_FIELDS {
        let label: &str = label;
        let hint: &str = hint;
        if let CfgKind::Section(title) = kind {
            flush(&mut section, current_title, &mut blocks);
            current_title = Some(title);
            continue;
        }
        let k = key.to_string();
        match kind {
            CfgKind::Bool => {
                let on = config_value_bool(&config, key);
                let text = if on { "是" } else { "否" };
                let cls = if on {
                    "admin-btn admin-btn-primary adm-cfg-toggle"
                } else {
                    "admin-btn adm-cfg-toggle"
                };
                section.push(rsx! {
                    div { class: "adm-cfg-field",
                        span { class: "adm-cfg-label", "{label}" }
                        button {
                            class: cls,
                            "data-pm-adm-cfg-toggle": k.clone(),
                            "{text}"
                        }
                        if !hint.is_empty() {
                            span { class: "adm-cfg-hint", "{hint}" }
                        }
                    }
                });
            }
            CfgKind::Num | CfgKind::Text => {
                let value = config_value_text(&config, key);
                let is_num = *kind == CfgKind::Num;
                section.push(rsx! {
                    div { class: "adm-cfg-field",
                        span { class: "adm-cfg-label", "{label}" }
                        input {
                            class: "adm-filter-input adm-cfg-input",
                            r#type: if is_num { "number" } else { "text" },
                            value: value,
                            "data-pm-adm-cfg": k.clone()
                        }
                        if !hint.is_empty() {
                            span { class: "adm-cfg-hint", "{hint}" }
                        }
                    }
                });
            }
            CfgKind::Section(_) => {}
        }
    }
    flush(&mut section, current_title, &mut blocks);

    // 新闻公告列表
    let items = announcements_of(&config);
    let mut ann_rows: Vec<VNode> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let title = json_str(item, "title");
        let url = json_str(item, "url");
        let idx = i.to_string();
        let idx_edit = idx.clone();
        ann_rows.push(rsx! {
            div { class: "adm-cfg-ann-row",
                span { class: "adm-cfg-ann-title", "{title}" }
                span { class: "adm-cfg-ann-url", "{url}" }
                div { class: "adm-cfg-ann-actions",
                    button {
                        class: "admin-btn adm-row-edit",
                        "data-pm-adm-ann-edit": idx_edit,
                        "编辑"
                    }
                    button {
                        class: "admin-btn adm-row-edit",
                        "data-pm-adm-ann-del": idx,
                        "删除"
                    }
                }
            }
        });
    }
    let can_add = items.len() < 6;
    let ann_body = if ann_rows.is_empty() {
        rsx! { p { class: "adm-editor-hint", "暂无公告" } }
    } else {
        rsx! {
            div { class: "adm-cfg-ann-list",
                for i in 0..ann_rows.len() {
                    ..vec![ann_rows[i].clone()]
                }
            }
        }
    };
    blocks.push(rsx! {
        div { class: "adm-cfg-section",
            div { class: "adm-cfg-section-title",
                "首页新闻公告"
                span { class: "adm-cfg-hint", "（最多 6 条）" }
            }
            ..vec![ann_body]
            button {
                class: "admin-btn adm-cfg-add",
                disabled: !can_add,
                "data-pm-adm-ann-new": "1",
                "新增公告"
            }
        }
    });

    // 未建模的键原样列出，避免用户以为它们不存在。
    let mut extra_rows: Vec<VNode> = Vec::new();
    let modeled: Vec<&str> = CONFIG_FIELDS.iter().map(|(k, _, _, _)| *k).collect();
    if let Some(obj) = config.as_object() {
        for (k, v) in obj {
            if modeled.contains(&k.as_str()) || k == "news_announcements" {
                continue;
            }
            let text = match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            extra_rows.push(rsx! {
                div { class: "adm-cfg-field",
                    span { class: "adm-cfg-label", "{k}" }
                    span { class: "adm-cfg-hint", "{text}" }
                }
            });
        }
    }
    if !extra_rows.is_empty() {
        blocks.push(rsx! {
            div { class: "adm-cfg-section",
                div { class: "adm-cfg-section-title", "其他键（只读，原样保留）" }
                div { class: "adm-cfg-grid",
                    for i in 0..extra_rows.len() {
                        ..vec![extra_rows[i].clone()]
                    }
                }
            }
        });
    }

    // 公告编辑弹窗
    let ann_modal = if cf.ann_open.get() {
        let editing = cf.ann_editing.get();
        let title = match editing {
            Some(i) => format!("编辑公告 #{}", i + 1),
            None => "新增公告".to_string(),
        };
        rsx! {
            div { class: "modal-overlay", "data-pm-adm-ann-overlay": "1",
                div { class: "adm-editor-modal adm-editor-modal--narrow",
                    div { class: "modal-header",
                        h3 { "{title}" }
                        button { class: "modal-close", "data-pm-adm-ann-close": "1", "×" }
                    }
                    div { class: "modal-body adm-editor-body",
                        div { class: "adm-editor-grid",
                            label { class: "adm-editor-field",
                                span { "标题" }
                                input {
                                    class: "adm-filter-input",
                                    r#type: "text",
                                    value: cf.ann_title.get(),
                                    "data-pm-adm-annf": "title"
                                }
                            }
                            label { class: "adm-editor-field",
                                span { "链接（可空）" }
                                input {
                                    class: "adm-filter-input",
                                    r#type: "text",
                                    value: cf.ann_url.get(),
                                    "data-pm-adm-annf": "url"
                                }
                            }
                        }
                    }
                    div { class: "adm-editor-footer",
                        button {
                            class: "admin-btn",
                            "data-pm-adm-ann-close": "1",
                            "取消"
                        }
                        button {
                            class: "admin-btn admin-btn-primary",
                            "data-pm-adm-ann-save": "1",
                            "确定"
                        }
                    }
                }
            }
        }
    } else {
        VNode::empty()
    };

    blocks.push(ann_modal);
    rsx! {
        for i in 0..blocks.len() {
            ..vec![blocks[i].clone()]
        }
    }
}

// ---------------------------------------------------------------------------
// 行删除确认弹窗（切片 15）
// ---------------------------------------------------------------------------

/// 页键 → (实体名, delete 端点实体, 列表状态, 列表 SPEC)。us 不在列：用户行没有删除。
fn delete_target(
    st: &AppState,
    page: &str,
) -> Option<(&'static str, &'static str, crate::state::ListState, crate::state::EntitySpec)> {
    match page {
        "pk" => Some((
            "宠物",
            "pokemon_type",
            st.pokemon.clone(),
            crate::state::POKEMON_SPEC,
        )),
        "it" => Some((
            "物品",
            "item_type",
            st.item.clone(),
            crate::state::ITEM_SPEC,
        )),
        "mp" => Some(("地图", "map_info", st.map.clone(), crate::state::MAP_SPEC)),
        "ev" => Some((
            "进化链",
            "evolution_info",
            st.evolution.clone(),
            crate::state::EVOLUTION_SPEC,
        )),
        "sk" => Some((
            "技能",
            "skill_type",
            st.skill.clone(),
            crate::state::SKILL_SPEC,
        )),
        "ef" => Some((
            "效果",
            "effect_data",
            st.effect.clone(),
            crate::state::EFFECT_SPEC,
        )),
        _ => None,
    }
}

/// 行名展示键：各实体列表里最像「名字」的字段。
fn delete_row_label(item: &serde_json::Value, page: &str) -> String {
    match page {
        "pk" | "it" | "sk" => json_str(item, "name"),
        "mp" => json_str(item, "name"),
        "ef" => json_str(item, "code"),
        "ev" => {
            let s = json_str(item, "source_name");
            let t = json_str(item, "target_name");
            if s.is_empty() && t.is_empty() {
                String::new()
            } else {
                format!("{s} → {t}")
            }
        }
        _ => String::new(),
    }
}

pub fn open_delete_confirm(state: &AppState, page: &str, id: u64) {
    if delete_target(state, page).is_none() {
        return;
    }
    let label = {
        let ls = match page {
            "pk" => state.pokemon.clone(),
            "it" => state.item.clone(),
            "mp" => state.map.clone(),
            "ev" => state.evolution.clone(),
            "sk" => state.skill.clone(),
            "ef" => state.effect.clone(),
            _ => state.pokemon.clone(),
        };
        ls.rows
            .get()
            .into_iter()
            .find(|r| json_u64(r, "id") == id)
            .map(|r| delete_row_label(&r, page))
            .unwrap_or_default()
    };
    let d = state.delete.clone();
    d.page.inner().set(page.to_string());
    d.id.inner().set(Some(id));
    d.label.inner().set(label);
    d.err.inner().set(None);
    d.open.inner().set(true);
    crate::request_remount();
}

pub fn close_delete_confirm(state: &AppState) {
    let d = state.delete.clone();
    if d.busy.get() {
        return;
    }
    d.open.inner().set(false);
    crate::request_remount();
}

pub fn submit_delete(state: &AppState) {
    let d = state.delete.clone();
    if d.busy.get() {
        return;
    }
    let Some(id) = d.id.get() else { return };
    let page = d.page.get();
    let Some((_, entity, ls, spec)) = delete_target(state, &page) else {
        return;
    };
    d.busy.inner().set(true);
    d.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::delete_entity(
        entity,
        id,
        Box::new(move |result: Result<(), String>| {
            let d = st.delete.clone();
            match result {
                Ok(()) => {
                    d.open.inner().set(false);
                    d.busy.inner().set(false);
                    ls.loaded.inner().set(None);
                    ls.rows.inner().set(Vec::new());
                    crate::request_remount();
                    reload_list(spec, &ls);
                }
                Err(e) => {
                    d.busy.inner().set(false);
                    d.err.inner().set(Some(format!("删除失败：{e}")));
                    crate::request_remount();
                }
            }
        }),
    );
}

/// 删除确认弹窗（挂载在 Admin 壳层，全页共用）。
fn delete_confirm_modal(st: &AppState) -> VNode {
    let d = st.delete.clone();
    if !d.open.get() {
        return VNode::empty();
    }
    let page = d.page.get();
    let Some((entity_title, _, _, _)) = delete_target(st, &page) else {
        return VNode::empty();
    };
    let id = d.id.get();
    let id_text = id.map(|n| n.to_string()).unwrap_or_default();
    let label = d.label.get();
    let display = if label.is_empty() {
        format!("#{id_text}")
    } else {
        format!("{label}（#{id_text}）")
    };
    let busy = d.busy.get();
    let err = d.err.get();
    let title = format!("删除{}", entity_title);
    let confirm_label = if busy { "删除中..." } else { "确认删除" };
    let del_message = format!("确定要删除 {display} 吗？此操作不可撤销。");
    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };
    rsx! {
        div { class: "modal-overlay", "data-pm-adm-del-overlay": "1",
            div { class: "adm-editor-modal adm-editor-modal--narrow",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-del-cancel": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    p { "{del_message}" }
                    ..vec![err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: busy,
                        "data-pm-adm-del-cancel": "1",
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn-danger",
                        disabled: busy,
                        "data-pm-adm-del-ok": "1",
                        "{confirm_label}"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 效果数据编辑弹窗（切片 15）
// ---------------------------------------------------------------------------

const EFFECT_KIND_OPTIONS: [(&str, &str); 6] = [
    ("move", "招式"),
    ("ability", "特性"),
    ("item", "道具"),
    ("weather", "天气"),
    ("field", "场地"),
    ("status", "状态"),
];

pub fn open_effect_new(state: &AppState) {
    let ed = state.effect_editor.clone();
    ed.editing_id.inner().set(None);
    ed.code.inner().set(String::new());
    ed.kind.inner().set("move".to_string());
    ed.hooks_json.inner().set("[\n  \"on_after_move\"\n]".to_string());
    ed.params_json
        .inner()
        .set("{\n  \"code\": \"\"\n}".to_string());
    ed.description.inner().set(String::new());
    ed.version.inner().set("1".to_string());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn open_effect_edit(state: &AppState, id: u64) {
    let ed = state.effect_editor.clone();
    let row = state
        .effect
        .rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    ed.editing_id.inner().set(Some(id));
    ed.code.inner().set(json_str(&row, "code"));
    ed.kind.inner().set(json_str(&row, "kind"));
    let hooks = row.get("hooks").cloned().unwrap_or(serde_json::json!([]));
    let params = row.get("params").cloned().unwrap_or(serde_json::json!({}));
    ed.hooks_json
        .inner()
        .set(serde_json::to_string_pretty(&hooks).unwrap_or_else(|_| "[]".into()));
    ed.params_json
        .inner()
        .set(serde_json::to_string_pretty(&params).unwrap_or_else(|_| "{}".into()));
    ed.description.inner().set(json_str(&row, "description"));
    ed.version
        .inner()
        .set(json_u64(&row, "version").to_string());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_effect_editor(state: &AppState) {
    state.effect_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn set_effect_editor_field(state: &AppState, key: &str, value: &str) {
    let ed = state.effect_editor.clone();
    match key {
        "code" => ed.code.inner().set(value.to_string()),
        "kind" => ed.kind.inner().set(value.to_string()),
        "hooks" => ed.hooks_json.inner().set(value.to_string()),
        "params" => ed.params_json.inner().set(value.to_string()),
        "description" => ed.description.inner().set(value.to_string()),
        "version" => ed.version.inner().set(value.to_string()),
        _ => {}
    }
    crate::request_remount();
}

pub fn submit_effect_editor(state: &AppState) {
    let ed = state.effect_editor.clone();
    if ed.saving.get() {
        return;
    }
    let code = ed.code.get().trim().to_string();
    if code.is_empty() {
        ed.err.inner().set(Some("效果码不能为空".to_string()));
        crate::request_remount();
        return;
    }
    // hooks 必须是非空数组；params 必须是含 code 键的对象——服务端
    // build_effect_declaration 同样硬校验，这里提前拦解析错误。
    let hooks: serde_json::Value = match serde_json::from_str(&ed.hooks_json.get()) {
        Ok(v) => v,
        Err(e) => {
            ed.err.inner().set(Some(format!("钩子 JSON 无效：{e}")));
            crate::request_remount();
            return;
        }
    };
    if !hooks.as_array().is_some_and(|a| !a.is_empty()) {
        ed.err
            .inner()
            .set(Some("钩子必须是非空数组（如 [\"on_after_move\"]）".to_string()));
        crate::request_remount();
        return;
    }
    let params: serde_json::Value = match serde_json::from_str(&ed.params_json.get()) {
        Ok(v) => v,
        Err(e) => {
            ed.err.inner().set(Some(format!("参数 JSON 无效：{e}")));
            crate::request_remount();
            return;
        }
    };
    if !params.get("code").and_then(|c| c.as_str()).is_some_and(|c| !c.is_empty()) {
        ed.err.inner().set(Some(
            "参数必须是对象且包含非空 code 键（核心效果码）".to_string(),
        ));
        crate::request_remount();
        return;
    }
    let version = ed.version.get().trim().parse::<u64>().unwrap_or(1).max(1);
    let mut payload = serde_json::json!({
        "code": code,
        "kind": ed.kind.get(),
        "hooks": hooks,
        "params": params,
        "description": ed.description.get(),
        "version": version,
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
    let on_done: Box<dyn FnOnce(Result<serde_json::Value, String>)> =
        Box::new(move |result: Result<serde_json::Value, String>| {
            let ed = st.effect_editor.clone();
            match result {
                Ok(_) => {
                    ed.open.inner().set(false);
                    ed.saving.inner().set(false);
                    st.effect.loaded.inner().set(None);
                    st.effect.rows.inner().set(Vec::new());
                    crate::request_remount();
                    reload_list(crate::state::EFFECT_SPEC, &st.effect);
                }
                Err(e) => {
                    ed.saving.inner().set(false);
                    ed.err.inner().set(Some(format!("保存失败：{e}")));
                    crate::request_remount();
                }
            }
        });
    if editing.is_some() {
        crate::api::set_entity("effect_data", body, on_done);
    } else {
        crate::api::insert_entity("effect_data", body, on_done);
    }
}

fn effect_editor_modal(st: &AppState) -> VNode {
    let ed = st.effect_editor.clone();
    if !ed.open.get() {
        return VNode::empty();
    }
    let editing = ed.editing_id.get();
    let title = match editing {
        Some(id) => format!("编辑效果 #{id}"),
        None => "新增效果".to_string(),
    };
    let saving = ed.saving.get();
    let err = ed.err.get();
    let kind = ed.kind.get();
    let save_label = if saving { "保存中..." } else { "保存" };

    let mut kind_opts: Vec<VNode> = Vec::new();
    for (code, label) in EFFECT_KIND_OPTIONS {
        kind_opts.push(rsx! {
            option { value: code, selected: code == kind, "{label}" }
        });
    }

    let err_block = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-efx-overlay": "1",
            div { class: "adm-editor-modal",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-efx-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    div { class: "adm-editor-grid",
                        label { class: "adm-editor-field",
                            span { "效果码（唯一标识）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "text",
                                value: ed.code.get(),
                                "data-pm-adm-efx-f": "code"
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "类型" }
                            select {
                                class: "adm-filter-input",
                                "data-pm-adm-efx-f": "kind",
                                for i in 0..kind_opts.len() {
                                    ..vec![kind_opts[i].clone()]
                                }
                            }
                        }
                        label { class: "adm-editor-field",
                            span { "版本（≥1）" }
                            input {
                                class: "adm-filter-input",
                                r#type: "number",
                                min: "1",
                                value: ed.version.get(),
                                "data-pm-adm-efx-f": "version"
                            }
                        }
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "描述" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 2,
                            "data-pm-adm-efx-f": "description",
                            "{ed.description.get()}"
                        }
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "钩子 JSON（非空数组，如 on_before_move / on_hit / on_after_move / on_turn_end …）" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 4,
                            "data-pm-adm-efx-f": "hooks",
                            "{ed.hooks_json.get()}"
                        }
                    }
                    label { class: "adm-editor-field adm-editor-wide",
                        span { "参数 JSON（对象，必须含非空 code 键 = 核心效果码）" }
                        textarea {
                            class: "adm-editor-textarea",
                            rows: 8,
                            "data-pm-adm-efx-f": "params",
                            "{ed.params_json.get()}"
                        }
                    }
                    ..vec![err_block]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: saving,
                        "data-pm-adm-efx-close": "1",
                        "取消"
                    }
                    button {
                        class: "admin-btn admin-btn-primary",
                        disabled: saving,
                        "data-pm-adm-efx-save": "1",
                        "{save_label}"
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 用户下钻：宠物/物品实例管理（切片 16）
// ---------------------------------------------------------------------------

/// 宠物状态标签（与 Dioxus 的 status_options 同表；服务端同时认 0-22 数字）。
const PM_STATUS_OPTIONS: [(&str, &str); 23] = [
    ("critical", "濒危"),
    ("normal", "正常"),
    ("sick1", "生病一"),
    ("sick2", "生病二"),
    ("sick3", "生病三"),
    ("hungry1", "饥饿一"),
    ("hungry2", "饥饿二"),
    ("tired", "疲惫"),
    ("excited1", "兴奋一"),
    ("excited2", "兴奋二"),
    ("excited3", "兴奋三"),
    ("hurt", "受伤"),
    ("happy1", "快乐一"),
    ("happy2", "快乐二"),
    ("happy3", "快乐三"),
    ("shock", "惊慌"),
    ("self_love1", "自恋一"),
    ("self_love2", "自恋二"),
    ("angry1", "愤怒一"),
    ("angry2", "愤怒二"),
    ("dead", "虚弱一"),
    ("weak2", "虚弱二"),
    ("weak3", "虚弱三"),
];

/// 宠物位置（site）载荷值 → 中文名。
fn pm_site_name(site: &str) -> &'static str {
    match site {
        "header" => "跟随",
        "bag" => "背包",
        "hospital" => "医院",
        _ => "仓库",
    }
}

/// 物品 tag_kind → tab 键（物品 tab 过滤用，与 Dioxus 的 ItemTab 对齐）。
fn item_tag_kind(tag: &serde_json::Value) -> String {
    tag.as_object()
        .and_then(|obj| obj.keys().next())
        .cloned()
        .unwrap_or_default()
}

/// 拉某用户的两份实例列表。
fn drill_load_lists(state: &AppState) {
    let dr = state.drill.clone();
    if dr.loading.get() {
        return;
    }
    let uid = dr.uid.get();
    dr.loading.inner().set(true);
    dr.err.inner().set(None);
    crate::request_remount();
    let st1 = state.clone();
    crate::api::list_user_instances(
        "pokemon_info",
        uid,
        Box::new(move |result: Result<Vec<serde_json::Value>, String>| {
            let dr = st1.drill.clone();
            match result {
                Ok(rows) => dr.pm_rows.inner().set(rows),
                Err(e) => dr.err.inner().set(Some(format!("宠物列表加载失败：{e}"))),
            }
            dr.loading.inner().set(false);
            crate::request_remount();
        }),
    );
    let st2 = state.clone();
    crate::api::list_user_instances(
        "item_info",
        uid,
        Box::new(move |result: Result<Vec<serde_json::Value>, String>| {
            let dr = st2.drill.clone();
            match result {
                Ok(rows) => dr.item_rows.inner().set(rows),
                Err(e) => dr.err.inner().set(Some(format!("物品列表加载失败：{e}"))),
            }
            crate::request_remount();
        }),
    );
}

/// 拉类型目录（种族名 / 物品名+tag），只拉一次缓存。
fn drill_load_catalogs(state: &AppState) {
    let dr = state.drill.clone();
    if dr.catalog_loaded.get() {
        return;
    }
    let st = state.clone();
    crate::api::list_entity(
        "pokemon_type",
        0,
        1000,
        Box::new(move |result: Result<Vec<serde_json::Value>, String>| {
            let dr = st.drill.clone();
            if let Ok(rows) = result {
                let catalog: Vec<(u64, String)> = rows
                    .iter()
                    .map(|r| (json_u64(r, "id"), json_str(r, "name")))
                    .collect();
                dr.pm_catalog.inner().set(catalog);
            }
            crate::request_remount();
        }),
    );
    let st2 = state.clone();
    crate::api::list_entity(
        "item_type",
        0,
        1000,
        Box::new(move |result: Result<Vec<serde_json::Value>, String>| {
            let dr = st2.drill.clone();
            if let Ok(rows) = result {
                let catalog: Vec<(u64, String, String)> = rows
                    .iter()
                    .map(|r| {
                        (
                            json_u64(r, "id"),
                            json_str(r, "name"),
                            item_tag_kind(r.get("tag").unwrap_or(&serde_json::Value::Null)),
                        )
                    })
                    .collect();
                dr.item_catalog.inner().set(catalog);
            }
            dr.catalog_loaded.inner().set(true);
            crate::request_remount();
        }),
    );
}

pub fn open_drill(state: &AppState, uid: u64) {
    let dr = state.drill.clone();
    dr.uid.inner().set(uid);
    dr.open.inner().set(true);
    dr.pm_rows.inner().set(Vec::new());
    dr.item_rows.inner().set(Vec::new());
    dr.pm_tab.inner().set("all".to_string());
    dr.item_tab.inner().set("all".to_string());
    dr.err.inner().set(None);
    drill_load_catalogs(state);
    drill_load_lists(state);
    crate::request_remount();
}

pub fn close_drill(state: &AppState) {
    let dr = state.drill.clone();
    if dr.loading.get() || dr.pm_editor.saving.get() || dr.item_editor.saving.get() {
        return;
    }
    dr.open.inner().set(false);
    dr.del_open.inner().set(false);
    dr.pm_editor.open.inner().set(false);
    dr.item_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn refresh_drill(state: &AppState, which: &str) {
    // 两份列表共用一个 loading 守卫，刷新任一就重拉两份（数据量小，简单优先）。
    let _ = which;
    drill_load_lists(state);
}

pub fn set_drill_tab(state: &AppState, kind: &str, tab: &str) {
    let dr = state.drill.clone();
    if kind == "pm" {
        dr.pm_tab.inner().set(tab.to_string());
    } else {
        dr.item_tab.inner().set(tab.to_string());
    }
    crate::request_remount();
}

// -- 宠物实例编辑器 ----------------------------------------------------------

pub fn open_pm_instance_edit(state: &AppState, id: u64) {
    let dr = state.drill.clone();
    let row = dr
        .pm_rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    let ed = dr.pm_editor.clone();
    ed.editing_id.inner().set(Some(id));
    ed.type_id.inner().set(json_u64(&row, "type_id"));
    ed.owner.inner().set(json_u64(&row, "owner"));
    ed.name.inner().set(json_str(&row, "name"));
    ed.site.inner().set(json_str(&row, "site"));
    ed.sex.inner().set(json_str(&row, "sex"));
    ed.status.inner().set(json_str(&row, "status"));
    ed.is_shiny.inner().set(json_bool(&row, "is_shiny"));
    ed.level.inner().set(json_u64(&row, "level").to_string());
    ed.experience
        .inner()
        .set(json_u64(&row, "experience").to_string());
    ed.intimacy
        .inner()
        .set(json_u64(&row, "intimacy").to_string());
    ed.using_ball_id
        .inner()
        .set(json_u64(&row, "using_ball_id").to_string());
    let read6 = |key: &str| -> [String; 6] {
        let node = row.get(key).cloned().unwrap_or_default();
        std::array::from_fn(|i| {
            node.get(STAT_KEYS[i])
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
                .to_string()
        })
    };
    ed.stat.inner().set(read6("statistic"));
    ed.base.inner().set(read6("base_points"));
    let armor_node = row.get("armor_slots_id").cloned().unwrap_or_default();
    ed.armor.inner().set(std::array::from_fn(|i| {
        armor_node
            .get(i)
            .and_then(|v| v.as_u64())
            .unwrap_or(0)
            .to_string()
    }));
    let skills = row.get("skills").cloned().unwrap_or(serde_json::json!([]));
    ed.skills_json
        .inner()
        .set(serde_json::to_string_pretty(&skills).unwrap_or_else(|_| "[]".into()));
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_pm_instance_editor(state: &AppState) {
    state.drill.pm_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn toggle_pm_instance_shiny(state: &AppState) {
    let ed = state.drill.pm_editor.clone();
    ed.is_shiny.inner().set(!ed.is_shiny.get());
    crate::request_remount();
}

/// 宠物实例字段变更。数组字段用 `stat:i` / `base:i` / `armor:i`。
pub fn set_pm_instance_field(state: &AppState, key: &str, value: &str) {
    let ed = state.drill.pm_editor.clone();
    match key {
        "name" => ed.name.inner().set(value.to_string()),
        "site" => ed.site.inner().set(value.to_string()),
        "sex" => ed.sex.inner().set(value.to_string()),
        "status" => ed.status.inner().set(value.to_string()),
        "level" => ed.level.inner().set(value.to_string()),
        "experience" => ed.experience.inner().set(value.to_string()),
        "intimacy" => ed.intimacy.inner().set(value.to_string()),
        "using_ball_id" => ed.using_ball_id.inner().set(value.to_string()),
        "skills" => ed.skills_json.inner().set(value.to_string()),
        _ => {
            if let Some((group, idx)) = key.split_once(':') {
                if let Ok(i) = idx.parse::<usize>() {
                    match group {
                        "stat" if i < 6 => {
                            let mut a = ed.stat.get();
                            a[i] = value.to_string();
                            ed.stat.inner().set(a);
                        }
                        "base" if i < 6 => {
                            let mut a = ed.base.get();
                            a[i] = value.to_string();
                            ed.base.inner().set(a);
                        }
                        "armor" if i < 4 => {
                            let mut a = ed.armor.get();
                            a[i] = value.to_string();
                            ed.armor.inner().set(a);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    crate::request_remount();
}

pub fn submit_pm_instance_editor(state: &AppState) {
    let ed = state.drill.pm_editor.clone();
    if ed.saving.get() {
        return;
    }
    let parse = |s: &str| s.trim().parse::<u64>().unwrap_or(0);
    let level = parse(&ed.level.get());
    if level < 1 {
        ed.err.inner().set(Some("等级必须 ≥ 1".to_string()));
        crate::request_remount();
        return;
    }
    let skills: serde_json::Value = match serde_json::from_str(&ed.skills_json.get()) {
        Ok(v) => v,
        Err(e) => {
            ed.err.inner().set(Some(format!("技能 JSON 无效：{e}")));
            crate::request_remount();
            return;
        }
    };
    if !skills.is_array() {
        ed.err
            .inner()
            .set(Some("技能必须是数组（[{type_id, count}]）".to_string()));
        crate::request_remount();
        return;
    }
    let stat = ed.stat.get();
    let base = ed.base.get();
    let armor = ed.armor.get();
    let stat_obj = |values: &[String; 6]| {
        let mut m = serde_json::Map::new();
        for i in 0..6 {
            m.insert(
                STAT_KEYS[i].to_string(),
                serde_json::json!(parse(&values[i])),
            );
        }
        serde_json::Value::Object(m)
    };
    let payload = serde_json::json!({
        "id": ed.editing_id.get(),
        "type_id": ed.type_id.get(),
        "owner": ed.owner.get(),
        "name": ed.name.get(),
        "site": ed.site.get(),
        "level": level,
        "experience": parse(&ed.experience.get()),
        "intimacy": parse(&ed.intimacy.get()),
        "using_ball_id": parse(&ed.using_ball_id.get()),
        "is_shiny": ed.is_shiny.get(),
        "status": ed.status.get(),
        "sex": ed.sex.get(),
        "statistic": stat_obj(&stat),
        "base_points": stat_obj(&base),
        "skills": skills,
        "armor_slots_id": [
            parse(&armor[0]), parse(&armor[1]), parse(&armor[2]), parse(&armor[3])
        ],
    });
    let body = payload.to_string();
    ed.saving.inner().set(true);
    ed.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::set_entity(
        "pokemon_info",
        body,
        Box::new(move |result: Result<serde_json::Value, String>| {
            let dr = st.drill.clone();
            let ed = dr.pm_editor.clone();
            match result {
                Ok(_) => {
                    ed.open.inner().set(false);
                    ed.saving.inner().set(false);
                    drill_load_lists(&st);
                }
                Err(e) => {
                    ed.saving.inner().set(false);
                    ed.err.inner().set(Some(format!("保存失败：{e}")));
                    crate::request_remount();
                }
            }
        }),
    );
}

// -- 物品实例编辑器 ----------------------------------------------------------

pub fn open_item_instance_edit(state: &AppState, id: u64) {
    let dr = state.drill.clone();
    let row = dr
        .item_rows
        .get()
        .into_iter()
        .find(|r| json_u64(r, "id") == id);
    let Some(row) = row else { return };
    let ed = dr.item_editor.clone();
    ed.editing_id.inner().set(Some(id));
    ed.owner.inner().set(json_u64(&row, "owner"));
    ed.type_id
        .inner()
        .set(json_u64(&row, "type_id").to_string());
    ed.count.inner().set(json_u64(&row, "count").to_string());
    ed.err.inner().set(None);
    ed.open.inner().set(true);
    crate::request_remount();
}

pub fn close_item_instance_editor(state: &AppState) {
    state.drill.item_editor.open.inner().set(false);
    crate::request_remount();
}

pub fn set_item_instance_field(state: &AppState, key: &str, value: &str) {
    let ed = state.drill.item_editor.clone();
    match key {
        "type_id" => ed.type_id.inner().set(value.to_string()),
        "count" => ed.count.inner().set(value.to_string()),
        _ => {}
    }
    crate::request_remount();
}

pub fn submit_item_instance_editor(state: &AppState) {
    let ed = state.drill.item_editor.clone();
    if ed.saving.get() {
        return;
    }
    let parse = |s: &str| s.trim().parse::<u64>().unwrap_or(0);
    let type_id = parse(&ed.type_id.get());
    if type_id == 0 {
        ed.err.inner().set(Some("物品类型 ID 必须为正整数".to_string()));
        crate::request_remount();
        return;
    }
    let count = parse(&ed.count.get());
    if count < 1 || count > 32767 {
        ed.err
            .inner()
            .set(Some("数量必须是 1 至 32767 的整数".to_string()));
        crate::request_remount();
        return;
    }
    let payload = serde_json::json!({
        "id": ed.editing_id.get(),
        "owner": ed.owner.get(),
        "type_id": type_id,
        "count": count,
    });
    let body = payload.to_string();
    ed.saving.inner().set(true);
    ed.err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::set_entity(
        "item_info",
        body,
        Box::new(move |result: Result<serde_json::Value, String>| {
            let dr = st.drill.clone();
            let ed = dr.item_editor.clone();
            match result {
                Ok(_) => {
                    ed.open.inner().set(false);
                    ed.saving.inner().set(false);
                    drill_load_lists(&st);
                }
                Err(e) => {
                    ed.saving.inner().set(false);
                    ed.err.inner().set(Some(format!("保存失败：{e}")));
                    crate::request_remount();
                }
            }
        }),
    );
}

// -- 实例删除确认 ------------------------------------------------------------

pub fn open_instance_delete(state: &AppState, kind: &str, id: u64) {
    let dr = state.drill.clone();
    let label = if kind == "pm" {
        dr.pm_rows
            .get()
            .into_iter()
            .find(|r| json_u64(r, "id") == id)
            .map(|r| json_str(&r, "name"))
            .unwrap_or_default()
    } else {
        let type_id = dr
            .item_rows
            .get()
            .into_iter()
            .find(|r| json_u64(r, "id") == id)
            .map(|r| json_u64(&r, "type_id"))
            .unwrap_or(0);
        dr.item_catalog
            .get()
            .into_iter()
            .find(|(tid, _, _)| *tid == type_id)
            .map(|(_, name, _)| name)
            .unwrap_or_else(|| format!("#{}", type_id))
    };
    dr.del_kind.inner().set(kind.to_string());
    dr.del_id.inner().set(Some(id));
    dr.del_label.inner().set(label);
    dr.del_err.inner().set(None);
    dr.del_open.inner().set(true);
    crate::request_remount();
}

pub fn close_instance_delete(state: &AppState) {
    let dr = state.drill.clone();
    if dr.del_busy.get() {
        return;
    }
    dr.del_open.inner().set(false);
    crate::request_remount();
}

pub fn submit_instance_delete(state: &AppState) {
    let dr = state.drill.clone();
    if dr.del_busy.get() {
        return;
    }
    let Some(id) = dr.del_id.get() else { return };
    let entity = if dr.del_kind.get() == "pm" {
        "pokemon_info"
    } else {
        "item_info"
    };
    dr.del_busy.inner().set(true);
    dr.del_err.inner().set(None);
    crate::request_remount();
    let st = state.clone();
    crate::api::delete_entity(
        entity,
        id,
        Box::new(move |result: Result<(), String>| {
            let dr = st.drill.clone();
            match result {
                Ok(()) => {
                    dr.del_open.inner().set(false);
                    dr.del_busy.inner().set(false);
                    drill_load_lists(&st);
                }
                Err(e) => {
                    dr.del_busy.inner().set(false);
                    dr.del_err.inner().set(Some(format!("删除失败：{e}")));
                    crate::request_remount();
                }
            }
        }),
    );
}

/// 下钻弹窗 + 两个实例编辑器 + 删除确认（三个弹窗互斥层级：编辑器/确认在
/// 下钻之上，由委托顺序保证内部按钮优先于各自遮罩）。
fn user_drill_modal(st: &AppState) -> VNode {
    let dr = st.drill.clone();
    if !dr.open.get() {
        return VNode::empty();
    }
    let uid = dr.uid.get();
    let loading = dr.loading.get();
    let err = dr.err.get();
    let pm_tab = dr.pm_tab.get();
    let item_tab = dr.item_tab.get();
    let title = format!("用户 #{} 的宠物与物品", uid);

    // -- 宠物表 --
    let mut pm_trs: Vec<VNode> = Vec::new();
    for row in dr.pm_rows.get().iter() {
        let site = json_str(row, "site");
        if pm_tab == "bag" && site != "bag" {
            continue;
        }
        if pm_tab == "store" && site != "store" {
            continue;
        }
        let id = json_u64(row, "id").to_string();
        let id_for_edit = id.clone();
        let del_attr = format!("pm:{}", id);
        let type_id = json_u64(row, "type_id");
        let species = dr
            .pm_catalog
            .get()
            .into_iter()
            .find(|(tid, _)| *tid == type_id)
            .map(|(_, n)| n)
            .unwrap_or_else(|| format!("#{}", type_id));
        let name = json_str(row, "name");
        let level = json_u64(row, "level").to_string();
        let site_text = pm_site_name(&site).to_string();
        pm_trs.push(rsx! {
            tr {
                td { "{id}" }
                td { "{species}" }
                td { "{name}" }
                td { "{level}" }
                td { "{site_text}" }
                td {
                    div { class: "adm-row-actions",
                        button {
                            class: "admin-btn adm-row-edit",
                            disabled: loading,
                            "data-pm-adm-ipe-edit": id_for_edit,
                            "编辑"
                        }
                        button {
                            class: "admin-btn adm-row-del",
                            disabled: loading,
                            "data-pm-adm-idel": del_attr,
                            "放生"
                        }
                    }
                }
            }
        });
    }

    // -- 物品表 --
    let mut item_trs: Vec<VNode> = Vec::new();
    for row in dr.item_rows.get().iter() {
        let type_id = json_u64(row, "type_id");
        let entry = dr
            .item_catalog
            .get()
            .into_iter()
            .find(|(tid, _, _)| *tid == type_id);
        let (name, tag_kind) = entry
            .clone()
            .map(|(_, n, t)| (n, t))
            .unwrap_or_else(|| (format!("#{}", type_id), String::new()));
        if item_tab != "all" && item_tab != tag_kind {
            continue;
        }
        let id = json_u64(row, "id").to_string();
        let id_for_edit = id.clone();
        let del_attr = format!("item:{}", id);
        let count = json_u64(row, "count").to_string();
        item_trs.push(rsx! {
            tr {
                td { "{id}" }
                td { "{name}" }
                td { "{count}" }
                td {
                    div { class: "adm-row-actions",
                        button {
                            class: "admin-btn adm-row-edit",
                            disabled: loading,
                            "data-pm-adm-iie-edit": id_for_edit,
                            "编辑"
                        }
                        button {
                            class: "admin-btn adm-row-del",
                            disabled: loading,
                            "data-pm-adm-idel": del_attr,
                            "删除"
                        }
                    }
                }
            }
        });
    }

    let catalog_ready = dr.catalog_loaded.get();
    let pm_table = if !catalog_ready {
        rsx! { p { class: "adm-editor-hint", "类型目录加载中..." } }
    } else if pm_trs.is_empty() {
        rsx! { p { class: "admin-empty-hint", "该用户暂无宠物。" } }
    } else {
        rsx! {
            table { class: "adm-table",
                thead {
                    tr {
                        th { "ID" }
                        th { "种族" }
                        th { "昵称" }
                        th { "等级" }
                        th { "位置" }
                        th { "操作" }
                    }
                }
                tbody {
                    for i in 0..pm_trs.len() {
                        ..vec![pm_trs[i].clone()]
                    }
                }
            }
        }
    };
    let item_table = if !catalog_ready {
        rsx! { p { class: "adm-editor-hint", "类型目录加载中..." } }
    } else if item_trs.is_empty() {
        rsx! { p { class: "admin-empty-hint", "该用户暂无物品。" } }
    } else {
        rsx! {
            table { class: "adm-table",
                thead {
                    tr {
                        th { "ID" }
                        th { "类型" }
                        th { "数量" }
                        th { "操作" }
                    }
                }
                tbody {
                    for i in 0..item_trs.len() {
                        ..vec![item_trs[i].clone()]
                    }
                }
            }
        }
    };

    let err_bar = match err {
        Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
        None => VNode::empty(),
    };
    let pm_tab_btn = |code: &str, label: &str, active: &str| -> VNode {
        let cls = if active == code {
            "admin-btn admin-btn-primary adm-tab-btn"
        } else {
            "admin-btn adm-tab-btn"
        };
        let attr = format!("pm:{}", code);
        rsx! {
            button {
                class: cls,
                disabled: loading,
                "data-pm-adm-drill-tab": attr,
                "{label}"
            }
        }
    };
    let item_tab_btn = |code: &str, label: &str, active: &str| -> VNode {
        let cls = if active == code {
            "admin-btn admin-btn-primary adm-tab-btn"
        } else {
            "admin-btn adm-tab-btn"
        };
        let attr = format!("item:{}", code);
        rsx! {
            button {
                class: cls,
                disabled: loading,
                "data-pm-adm-drill-tab": attr,
                "{label}"
            }
        }
    };

    // -- 宠物实例编辑器 --
    let pm_editor = {
        let ed = dr.pm_editor.clone();
        if !ed.open.get() {
            VNode::empty()
        } else {
            let saving = ed.saving.get();
            let err = ed.err.get();
            let title = format!("编辑宠物 #{}（种族 #{}）", ed.editing_id.get().unwrap_or(0), ed.type_id.get());
            let save_label = if saving { "保存中..." } else { "保存" };
            let shiny_label = if ed.is_shiny.get() { "闪光：是" } else { "闪光：否" };
            let shiny_cls = if ed.is_shiny.get() {
                "admin-btn admin-btn-primary adm-editor-toggle"
            } else {
                "admin-btn adm-editor-toggle"
            };
            let site = ed.site.get();
            let sex = ed.sex.get();
            let status = ed.status.get();
            let stat = ed.stat.get();
            let base = ed.base.get();
            let armor = ed.armor.get();

            fn sel(opts: &[(&str, &str)], current: &str) -> Vec<VNode> {
                opts.iter()
                    .map(|(code, label)| {
                        let selected: bool = *code == current;
                        let code: &str = code;
                        let label: &str = label;
                        rsx! {
                            option { value: code, selected: selected, "{label}" }
                        }
                    })
                    .collect()
            }
            let site_opts = sel(
                &[
                    ("header", "跟随"),
                    ("bag", "背包"),
                    ("store", "仓库"),
                    ("hospital", "医院"),
                ],
                &site,
            );
            let sex_opts = sel(
                &[("male", "雄性"), ("female", "雌性"), ("unknown", "未知")],
                &sex,
            );
            let status_opts = sel(&PM_STATUS_OPTIONS, &status);

            let mut stat_inputs: Vec<VNode> = Vec::new();
            let mut base_inputs: Vec<VNode> = Vec::new();
            for i in 0..6 {
                let sl = STAT_LABELS[i];
                let sv = stat[i].clone();
                let sa = format!("stat:{i}");
                stat_inputs.push(rsx! {
                    label { class: "adm-editor-field",
                        span { "个体 {sl}" }
                        input {
                            class: "adm-filter-input",
                            r#type: "number",
                            value: sv,
                            "data-pm-adm-ipef": sa
                        }
                    }
                });
                let bv = base[i].clone();
                let ba = format!("base:{i}");
                base_inputs.push(rsx! {
                    label { class: "adm-editor-field",
                        span { "努力 {sl}" }
                        input {
                            class: "adm-filter-input",
                            r#type: "number",
                            value: bv,
                            "data-pm-adm-ipef": ba
                        }
                    }
                });
            }
            let mut armor_inputs: Vec<VNode> = Vec::new();
            for i in 0..4 {
                let label = format!("装备槽 {}", i + 1);
                let av = armor[i].clone();
                let aa = format!("armor:{i}");
                armor_inputs.push(rsx! {
                    label { class: "adm-editor-field",
                        span { "{label}" }
                        input {
                            class: "adm-filter-input",
                            r#type: "number",
                            value: av,
                            "data-pm-adm-ipef": aa
                        }
                    }
                });
            }

            let err_block = match err {
                Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
                None => VNode::empty(),
            };

            rsx! {
                div { class: "modal-overlay", "data-pm-adm-ipe-overlay": "1",
                    div { class: "adm-editor-modal",
                        div { class: "modal-header",
                            h3 { "{title}" }
                            button { class: "modal-close", "data-pm-adm-ipe-close": "1", "×" }
                        }
                        div { class: "modal-body adm-editor-body",
                            div { class: "adm-editor-grid",
                                label { class: "adm-editor-field",
                                    span { "昵称" }
                                    input {
                                        class: "adm-filter-input",
                                        r#type: "text",
                                        value: ed.name.get(),
                                        "data-pm-adm-ipef": "name"
                                    }
                                }
                                label { class: "adm-editor-field",
                                    span { "位置" }
                                    select {
                                        class: "adm-filter-input",
                                        "data-pm-adm-ipef": "site",
                                        for i in 0..site_opts.len() {
                                            ..vec![site_opts[i].clone()]
                                        }
                                    }
                                }
                                label { class: "adm-editor-field",
                                    span { "性别" }
                                    select {
                                        class: "adm-filter-input",
                                        "data-pm-adm-ipef": "sex",
                                        for i in 0..sex_opts.len() {
                                            ..vec![sex_opts[i].clone()]
                                        }
                                    }
                                }
                                label { class: "adm-editor-field",
                                    span { "状态" }
                                    select {
                                        class: "adm-filter-input",
                                        "data-pm-adm-ipef": "status",
                                        for i in 0..status_opts.len() {
                                            ..vec![status_opts[i].clone()]
                                        }
                                    }
                                }
                                button {
                                    class: shiny_cls,
                                    disabled: saving,
                                    "data-pm-adm-ipe-shiny": "1",
                                    "{shiny_label}"
                                }
                                label { class: "adm-editor-field",
                                    span { "等级" }
                                    input {
                                        class: "adm-filter-input",
                                        r#type: "number",
                                        min: "1",
                                        value: ed.level.get(),
                                        "data-pm-adm-ipef": "level"
                                    }
                                }
                                label { class: "adm-editor-field",
                                    span { "经验" }
                                    input {
                                        class: "adm-filter-input",
                                        r#type: "number",
                                        value: ed.experience.get(),
                                        "data-pm-adm-ipef": "experience"
                                    }
                                }
                                label { class: "adm-editor-field",
                                    span { "亲密度" }
                                    input {
                                        class: "adm-filter-input",
                                        r#type: "number",
                                        value: ed.intimacy.get(),
                                        "data-pm-adm-ipef": "intimacy"
                                    }
                                }
                                label { class: "adm-editor-field",
                                    span { "精灵球 ID" }
                                    input {
                                        class: "adm-filter-input",
                                        r#type: "number",
                                        value: ed.using_ball_id.get(),
                                        "data-pm-adm-ipef": "using_ball_id"
                                    }
                                }
                            }
                            div { class: "adm-editor-section-title", "个体值" }
                            div { class: "adm-editor-grid",
                                for i in 0..stat_inputs.len() {
                                    ..vec![stat_inputs[i].clone()]
                                }
                            }
                            div { class: "adm-editor-section-title", "努力值" }
                            div { class: "adm-editor-grid",
                                for i in 0..base_inputs.len() {
                                    ..vec![base_inputs[i].clone()]
                                }
                            }
                            div { class: "adm-editor-section-title", "装备槽（0 = 空）" }
                            div { class: "adm-editor-grid",
                                for i in 0..armor_inputs.len() {
                                    ..vec![armor_inputs[i].clone()]
                                }
                            }
                            label { class: "adm-editor-field adm-editor-wide",
                                span { "技能 JSON（[{type_id, count}]，服务端逐条校验）" }
                                textarea {
                                    class: "adm-editor-textarea",
                                    rows: 4,
                                    "data-pm-adm-ipef": "skills",
                                    "{ed.skills_json.get()}"
                                }
                            }
                            ..vec![err_block]
                        }
                        div { class: "adm-editor-footer",
                            button {
                                class: "admin-btn",
                                disabled: saving,
                                "data-pm-adm-ipe-close": "1",
                                "取消"
                            }
                            button {
                                class: "admin-btn admin-btn-primary",
                                disabled: saving,
                                "data-pm-adm-ipe-save": "1",
                                "{save_label}"
                            }
                        }
                    }
                }
            }
        }
    };

    // -- 物品实例编辑器 --
    let item_editor = {
        let ed = dr.item_editor.clone();
        if !ed.open.get() {
            VNode::empty()
        } else {
            let saving = ed.saving.get();
            let err = ed.err.get();
            let title = format!("编辑物品实例 #{}", ed.editing_id.get().unwrap_or(0));
            let save_label = if saving { "保存中..." } else { "保存" };
            let err_block = match err {
                Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
                None => VNode::empty(),
            };
            rsx! {
                div { class: "modal-overlay", "data-pm-adm-iie-overlay": "1",
                    div { class: "adm-editor-modal adm-editor-modal--narrow",
                        div { class: "modal-header",
                            h3 { "{title}" }
                            button { class: "modal-close", "data-pm-adm-iie-close": "1", "×" }
                        }
                        div { class: "modal-body adm-editor-body",
                            div { class: "adm-editor-grid",
                                label { class: "adm-editor-field",
                                    span { "物品类型 ID" }
                                    input {
                                        class: "adm-filter-input",
                                        r#type: "number",
                                        min: "1",
                                        value: ed.type_id.get(),
                                        "data-pm-adm-iief": "type_id"
                                    }
                                }
                                label { class: "adm-editor-field",
                                    span { "数量（1-32767）" }
                                    input {
                                        class: "adm-filter-input",
                                        r#type: "number",
                                        min: "1",
                                        max: "32767",
                                        value: ed.count.get(),
                                        "data-pm-adm-iief": "count"
                                    }
                                }
                            }
                            ..vec![err_block]
                        }
                        div { class: "adm-editor-footer",
                            button {
                                class: "admin-btn",
                                disabled: saving,
                                "data-pm-adm-iie-close": "1",
                                "取消"
                            }
                            button {
                                class: "admin-btn admin-btn-primary",
                                disabled: saving,
                                "data-pm-adm-iie-save": "1",
                                "{save_label}"
                            }
                        }
                    }
                }
            }
        }
    };

    // -- 实例删除确认 --
    let del_confirm = {
        if !dr.del_open.get() {
            VNode::empty()
        } else {
            let busy = dr.del_busy.get();
            let err = dr.del_err.get();
            let kind = dr.del_kind.get();
            let noun = if kind == "pm" { "宠物" } else { "物品" };
            let title = format!("删除{}", noun);
            let label = dr.del_label.get();
            let id = dr.del_id.get();
            let id_text = id.map(|n| n.to_string()).unwrap_or_default();
            let display = if label.is_empty() {
                format!("#{id_text}")
            } else {
                format!("{label}（#{id_text}）")
            };
            // 放生有额外护栏：最后一只不可放生、战斗中首位不可放生。
            let extra = if kind == "pm" { "（放生有护栏：最后一只、战斗中的首位都会被服务端拒绝）" } else { "" };
            let message = format!("确定要删除 {display} 吗？{extra}");
            let confirm_label = if busy { "删除中..." } else { "确认删除" };
            let err_block = match err {
                Some(e) => rsx! { p { class: "adm-editor-error", "{e}" } },
                None => VNode::empty(),
            };
            rsx! {
                div { class: "modal-overlay", "data-pm-adm-idel-overlay": "1",
                    div { class: "adm-editor-modal adm-editor-modal--narrow",
                        div { class: "modal-header",
                            h3 { "{title}" }
                            button { class: "modal-close", "data-pm-adm-idel-cancel": "1", "×" }
                        }
                        div { class: "modal-body adm-editor-body",
                            p { "{message}" }
                            ..vec![err_block]
                        }
                        div { class: "adm-editor-footer",
                            button {
                                class: "admin-btn",
                                disabled: busy,
                                "data-pm-adm-idel-cancel": "1",
                                "取消"
                            }
                            button {
                                class: "admin-btn admin-btn-danger",
                                disabled: busy,
                                "data-pm-adm-idel-ok": "1",
                                "{confirm_label}"
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "modal-overlay", "data-pm-adm-drill-overlay": "1",
            div { class: "adm-editor-modal",
                div { class: "modal-header",
                    h3 { "{title}" }
                    button { class: "modal-close", "data-pm-adm-drill-close": "1", "×" }
                }
                div { class: "modal-body adm-editor-body",
                    ..vec![err_bar]
                    div { class: "adm-editor-section-title",
                        if loading { "宠物（加载中...）" } else { "宠物" }
                        ..vec![pm_tab_btn("all", "全部", &pm_tab), pm_tab_btn("bag", "背包", &pm_tab), pm_tab_btn("store", "仓库", &pm_tab)]
                        button {
                            class: "admin-btn adm-tab-btn",
                            disabled: loading,
                            "data-pm-adm-drill-refresh": "pm",
                            "刷新"
                        }
                    }
                    ..vec![pm_table]
                    div { class: "adm-editor-section-title",
                        if loading { "物品（加载中...）" } else { "物品" }
                        ..vec![
                            item_tab_btn("all", "全部", &item_tab),
                            item_tab_btn("drug", "药品", &item_tab),
                            item_tab_btn("ball", "宠物球", &item_tab),
                            item_tab_btn("evolution", "素材", &item_tab),
                            item_tab_btn("armor", "装备", &item_tab),
                            item_tab_btn("special", "特殊", &item_tab),
                        ]
                        button {
                            class: "admin-btn adm-tab-btn",
                            disabled: loading,
                            "data-pm-adm-drill-refresh": "item",
                            "刷新"
                        }
                    }
                    ..vec![item_table]
                }
                div { class: "adm-editor-footer",
                    button {
                        class: "admin-btn",
                        disabled: loading,
                        "data-pm-adm-drill-close": "1",
                        "关闭"
                    }
                }
            }
            ..vec![pm_editor, item_editor, del_confirm]
        }
    }
}
