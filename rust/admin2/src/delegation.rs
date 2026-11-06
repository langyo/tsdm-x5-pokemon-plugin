//! 事件委托层（与 _game2 同模式：#app 上注册一次，data-pm-adm-* 命中）。

use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::element::get_attribute;
use tairitsu_web::wit_platform::wasm_impl::bindings::tairitsu_browser::full::node::get_parent_element;
use tairitsu_web::WitElement;
use tairitsu_web::WitPlatform;

use crate::app_state::AppState;
use crate::state::AdminRoute;

fn attr_of(handle: u64, name: &str) -> Option<String> {
    get_attribute(handle, name)
}

fn find_with_attr(target: u64, name: &str) -> Option<(u64, String)> {
    let mut cur = target;
    for _ in 0..12 {
        if let Some(v) = attr_of(cur, name) {
            return Some((cur, v));
        }
        match get_parent_element(cur) {
            Some(parent) => cur = parent,
            None => return None,
        }
    }
    None
}

pub fn install(app: &WitElement, state: &AppState) {
    use tairitsu_vdom::platform::DomOps;
    let platform = WitPlatform::new().expect("platform");

    {
        let st = state.clone();
        platform.add_event_listener(
            app,
            "click",
            Box::new(move |e| {
                let Some(ev) = e.as_any().downcast_ref::<tairitsu_vdom::MouseEvent>() else {
                    return;
                };
                let Some(target) = ev.target else { return };
                dispatch_click(&st, target);
            }),
        );
    }

    // 配置编辑框：change 时同步草稿（无需重挂——但 dirty 徽章需要，仍请求重挂）。
    {
        let st = state.clone();
        platform.add_event_listener(
            app,
            "change",
            Box::new(move |e| {
                let Some(ev) = e.as_any().downcast_ref::<tairitsu_vdom::ChangeEvent>() else {
                    return;
                };
                let Some(target) = ev.target else { return };
                if attr_of(target, "data-pm-adm-config-input").is_some() {
                    st.config.draft.inner().set(ev.value.clone());
                    crate::request_remount();
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-pkf") {
                    // 编辑弹窗字段：整树重挂会把输入框重建，光标位置随之丢失
                    // —— 这是既有架构的取舍（与全局配置页一致），change 事件
                    // 在失焦/回车时才发，编辑中途不打断输入。
                    crate::pages::set_pokemon_editor_field(&st, &key, &ev.value, false);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-itf") {
                    crate::pages::set_item_editor_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-mpf") {
                    crate::pages::set_map_editor_field(&st, &key, &ev.value);
                } else if let Some((_, page)) = find_with_attr(target, "data-pm-adm-f-id") {
                    let (ls, _) = list_of(&st, &page);
                    ls.f_id.inner().set(ev.value.clone());
                } else if let Some((_, page)) = find_with_attr(target, "data-pm-adm-f-name") {
                    let (ls, _) = list_of(&st, &page);
                    ls.f_name.inner().set(ev.value.clone());
                } else if let Some((_, page)) = find_with_attr(target, "data-pm-adm-f-a") {
                    let (ls, _) = list_of(&st, &page);
                    ls.f_a.inner().set(ev.value.clone());
                } else if let Some((_, page)) = find_with_attr(target, "data-pm-adm-f-b") {
                    let (ls, _) = list_of(&st, &page);
                    ls.f_b.inner().set(ev.value.clone());
                }
            }),
        );
    }
}

/// 页面键（pk=宝可梦 / it=物品）→ 列表状态与实体配置。
fn list_of(st: &AppState, page: &str) -> (crate::state::ListState, crate::state::EntitySpec) {
    match page {
        "pk" => (st.pokemon.clone(), crate::state::POKEMON_SPEC),
        "it" => (st.item.clone(), crate::state::ITEM_SPEC),
        "mp" => (st.map.clone(), crate::state::MAP_SPEC),
        "us" => (st.user.clone(), crate::state::USER_SPEC),
        "ev" => (st.evolution.clone(), crate::state::EVOLUTION_SPEC),
        "sk" => (st.skill.clone(), crate::state::SKILL_SPEC),
        "ef" => (st.effect.clone(), crate::state::EFFECT_SPEC),
        _ => (st.pokemon.clone(), crate::state::POKEMON_SPEC),
    }
}

fn dispatch_click(state: &AppState, target: u64) {
    // 宝可梦编辑弹窗：内部按钮必须先于遮罩关闭属性判定（否则保存/取消
    // 会连带命中 data-pm-adm-pk-close 只关不存）。
    if let Some((_, key)) = find_with_attr(target, "data-pm-adm-pk-toggle") {
        let ed = state.pokemon_editor.clone();
        match key.as_str() {
            "selling" => ed.is_selling.inner().set(!ed.is_selling.get()),
            "legendary" => ed.is_legendary.inner().set(!ed.is_legendary.get()),
            _ => {}
        }
        crate::request_remount();
        return;
    }
    if find_with_attr(target, "data-pm-adm-pk-save").is_some() {
        crate::pages::submit_pokemon_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-pk-close").is_some() {
        crate::pages::close_pokemon_editor(state);
        return;
    }
    // 遮罩只有点在遮罩自身上才关（点弹窗内部空白不关）。find_with_attr
    // 会一路向上找到遮罩，所以这里要比对命中的句柄是不是点击目标本身。
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-pk-overlay") {
        if hit == target {
            crate::pages::close_pokemon_editor(state);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-pk-new").is_some() {
        crate::pages::open_pokemon_new(state);
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-pk-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_pokemon_edit(state, id);
        }
        return;
    }
    // 物品编辑弹窗（it 前缀）。与宝可梦弹窗同样的顺序要求：内部按钮先于
    // 遮罩判定。
    if let Some((_, key)) = find_with_attr(target, "data-pm-adm-it-toggle") {
        if key == "selling" {
            crate::pages::toggle_item_editor_selling(state);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-it-save").is_some() {
        crate::pages::submit_item_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-it-close").is_some() {
        crate::pages::close_item_editor(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-it-overlay") {
        if hit == target {
            crate::pages::close_item_editor(state);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-it-new").is_some() {
        crate::pages::open_item_new(state);
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-it-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_item_edit(state, id);
        }
        return;
    }
    // 地图编辑弹窗（mp 前缀）。
    if let Some((_, key)) = find_with_attr(target, "data-pm-adm-mp-toggle") {
        if key == "enabled" {
            crate::pages::toggle_map_editor_enabled(state);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-mp-save").is_some() {
        crate::pages::submit_map_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-mp-close").is_some() {
        crate::pages::close_map_editor(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-mp-overlay") {
        if hit == target {
            crate::pages::close_map_editor(state);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-mp-new").is_some() {
        crate::pages::open_map_new(state);
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-mp-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_map_edit(state, id);
        }
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-adm-nav") {
        let route = match v.as_str() {
            "pokemon-data" => AdminRoute::PokemonData,
            "item-data" => AdminRoute::ItemData,
            "map-data" => AdminRoute::MapData,
            "user-data" => AdminRoute::UserData,
            "evolution-data" => AdminRoute::EvolutionData,
            "skill-type" => AdminRoute::SkillType,
            "effect-data" => AdminRoute::EffectData,
            _ => AdminRoute::GlobalConfig,
        };
        crate::pages::switch_route(state, route);
        return;
    }
    if find_with_attr(target, "data-pm-adm-config-save").is_some() {
        crate::pages::save_config(state);
        return;
    }
    // 数据列表页（pk/it 前缀路由到对应实体）：筛选 / 重置 / 刷新 / 加载更多
    if let Some((_, page)) = find_with_attr(target, "data-pm-adm-l-apply") {
        let (ls, spec) = list_of(state, &page);
        crate::pages::apply_list_filters_pub(spec, state, &ls);
        return;
    }
    if let Some((_, page)) = find_with_attr(target, "data-pm-adm-l-reset") {
        let (ls, spec) = list_of(state, &page);
        crate::pages::reset_list_filters_pub(spec, &ls);
        return;
    }
    if let Some((_, page)) = find_with_attr(target, "data-pm-adm-l-more") {
        let (ls, spec) = list_of(state, &page);
        crate::pages::load_more_list_pub(spec, &ls);
        return;
    }
    if let Some((_, page)) = find_with_attr(target, "data-pm-adm-l-reload") {
        let (ls, _) = list_of(state, &page);
        ls.loaded.inner().set(None);
        ls.rows.inner().set(Vec::new());
        crate::request_remount();
        crate::pages::ensure_loaded(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-config-reset").is_some() {
        crate::pages::reset_config(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-config-reload").is_some() {
        state.config.loaded.inner().set(None);
        crate::request_remount();
        crate::pages::ensure_loaded(state);
    }
}
