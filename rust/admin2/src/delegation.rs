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
                } else if attr_of(target, "data-pm-adm-pk-f-id").is_some() {
                    st.pokemon.f_id.inner().set(ev.value.clone());
                } else if attr_of(target, "data-pm-adm-pk-f-name").is_some() {
                    st.pokemon.f_name.inner().set(ev.value.clone());
                } else if attr_of(target, "data-pm-adm-pk-f-kind").is_some() {
                    st.pokemon.f_kind.inner().set(ev.value.clone());
                }
            }),
        );
    }
}

fn dispatch_click(state: &AppState, target: u64) {
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
    // 宝可梦数据页：筛选 / 重置 / 刷新 / 加载更多
    if find_with_attr(target, "data-pm-adm-pk-apply").is_some() {
        crate::pages::apply_pokemon_filters(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-pk-reset").is_some() {
        crate::pages::reset_pokemon_filters(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-pk-more").is_some() {
        crate::pages::load_more_pokemon(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-pk-reload").is_some() {
        state.pokemon.loaded.inner().set(None);
        state.pokemon.rows.inner().set(Vec::new());
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
