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
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-cfg") {
                    crate::pages::set_config_field(&st, &key, &ev.value, false);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-annf") {
                    crate::pages::set_announcement_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-pkf") {
                    // 编辑弹窗字段：整树重挂会把输入框重建，光标位置随之丢失
                    // —— 这是既有架构的取舍（与全局配置页一致），change 事件
                    // 在失焦/回车时才发，编辑中途不打断输入。
                    crate::pages::set_pokemon_editor_field(&st, &key, &ev.value, false);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-itf") {
                    crate::pages::set_item_editor_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-mpf") {
                    crate::pages::set_map_editor_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-skf") {
                    crate::pages::set_skill_editor_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-evf") {
                    crate::pages::set_evolution_editor_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-grantf") {
                    if key == "query" {
                        crate::pages::set_grant_query(&st, &ev.value);
                    } else {
                        crate::pages::set_grant_field(&st, &key, &ev.value);
                    }
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-efx-f") {
                    crate::pages::set_effect_editor_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-ipef") {
                    crate::pages::set_pm_instance_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-iief") {
                    crate::pages::set_item_instance_field(&st, &key, &ev.value);
                } else if let Some((_, key)) = find_with_attr(target, "data-pm-adm-umf") {
                    crate::pages::set_user_meta_field(&st, &key, &ev.value);
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
    // 技能编辑弹窗（sk 前缀）。
    if find_with_attr(target, "data-pm-adm-sk-save").is_some() {
        crate::pages::submit_skill_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-sk-close").is_some() {
        crate::pages::close_skill_editor(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-sk-overlay") {
        if hit == target {
            crate::pages::close_skill_editor(state);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-sk-new").is_some() {
        crate::pages::open_skill_new(state);
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-sk-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_skill_edit(state, id);
        }
        return;
    }
    // 进化链编辑弹窗（ev 前缀）。
    if find_with_attr(target, "data-pm-adm-ev-save").is_some() {
        crate::pages::submit_evolution_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-ev-close").is_some() {
        crate::pages::close_evolution_editor(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-ev-overlay") {
        if hit == target {
            crate::pages::close_evolution_editor(state);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-ev-new").is_some() {
        crate::pages::open_evolution_new(state);
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-ev-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_evolution_edit(state, id);
        }
        return;
    }
    // 用户发放弹窗（grant 前缀）。两个入口分别带 uid。
    // 顺序注意：遮罩判断必须放在本块最后——find_with_attr 会一路向上找到
    // 遮罩，若提前判定并 return，弹窗内部所有按钮都会被吞掉（曾经踩过）。
    if find_with_attr(target, "data-pm-adm-grant-close").is_some() {
        crate::pages::close_grant(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-grant-search").is_some() {
        crate::pages::search_grant_types(state);
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-grant-pick") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::pick_grant_type(state, id);
        }
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-adm-grant-step") {
        if let Ok(step) = v.trim().parse::<u8>() {
            crate::pages::grant_go_step(state, step);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-grant-shiny").is_some() {
        crate::pages::toggle_grant_shiny(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-grant-submit").is_some() {
        crate::pages::submit_grant(state);
        return;
    }
    if let Some((_, uid)) = find_with_attr(target, "data-pm-adm-grant-poke") {
        if let Ok(uid) = uid.trim().parse::<u64>() {
            crate::pages::open_grant(state, uid, "pokemon");
        }
        return;
    }
    if let Some((_, uid)) = find_with_attr(target, "data-pm-adm-grant-item") {
        if let Ok(uid) = uid.trim().parse::<u64>() {
            crate::pages::open_grant(state, uid, "item");
        }
        return;
    }
    // 遮罩关闭放在发放块最后：只有点在遮罩自身上才关，点弹窗内部空白不关。
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-grant-overlay") {
        if hit == target {
            crate::pages::close_grant(state);
        }
        return;
    }
    // 效果编辑弹窗（efx 前缀）。
    if find_with_attr(target, "data-pm-adm-efx-save").is_some() {
        crate::pages::submit_effect_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-efx-close").is_some() {
        crate::pages::close_effect_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-efx-new").is_some() {
        crate::pages::open_effect_new(state);
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-efx-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_effect_edit(state, id);
        }
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-efx-overlay") {
        if hit == target {
            crate::pages::close_effect_editor(state);
        }
        return;
    }
    // 行删除确认（六实体共用，值形如 "pk:25"）。确认/取消先于遮罩。
    if find_with_attr(target, "data-pm-adm-del-ok").is_some() {
        crate::pages::submit_delete(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-del-cancel").is_some() {
        crate::pages::close_delete_confirm(state);
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-adm-del") {
        if let Some((page, id)) = v.split_once(':') {
            if let Ok(id) = id.trim().parse::<u64>() {
                crate::pages::open_delete_confirm(state, page, id);
            }
        }
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-del-overlay") {
        if hit == target {
            crate::pages::close_delete_confirm(state);
        }
        return;
    }
    // 用户下钻（drill）：实例编辑器/删除确认的按钮必须先于下钻内部按钮与
    // 遮罩（三个弹窗同屏叠放，find_with_attr 会向上穿过遮罩）。
    if find_with_attr(target, "data-pm-adm-ipe-save").is_some() {
        crate::pages::submit_pm_instance_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-ipe-close").is_some() {
        crate::pages::close_pm_instance_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-ipe-shiny").is_some() {
        crate::pages::toggle_pm_instance_shiny(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-ipe-overlay") {
        if hit == target {
            crate::pages::close_pm_instance_editor(state);
        }
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-ipe-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_pm_instance_edit(state, id);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-iie-save").is_some() {
        crate::pages::submit_item_instance_editor(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-iie-close").is_some() {
        crate::pages::close_item_instance_editor(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-iie-overlay") {
        if hit == target {
            crate::pages::close_item_instance_editor(state);
        }
        return;
    }
    if let Some((_, id)) = find_with_attr(target, "data-pm-adm-iie-edit") {
        if let Ok(id) = id.trim().parse::<u64>() {
            crate::pages::open_item_instance_edit(state, id);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-idel-ok").is_some() {
        crate::pages::submit_instance_delete(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-idel-cancel").is_some() {
        crate::pages::close_instance_delete(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-idel-overlay") {
        if hit == target {
            crate::pages::close_instance_delete(state);
        }
        return;
    }
    if let Some((_, v)) = find_with_attr(target, "data-pm-adm-idel") {
        if let Some((kind, id)) = v.split_once(':') {
            if let Ok(id) = id.trim().parse::<u64>() {
                crate::pages::open_instance_delete(state, kind, id);
            }
        }
        return;
    }
    // 用户元数据编辑（下钻弹窗顶部）。
    if find_with_attr(target, "data-pm-adm-um-save").is_some() {
        crate::pages::submit_user_meta(state);
        return;
    }
    // 下钻弹窗内部（tab/刷新/行编辑引用由上面的编辑器分支兜住）。
    if let Some((_, v)) = find_with_attr(target, "data-pm-adm-drill-tab") {
        if let Some((kind, tab)) = v.split_once(':') {
            crate::pages::set_drill_tab(state, kind, tab);
        }
        return;
    }
    if find_with_attr(target, "data-pm-adm-drill-refresh").is_some() {
        crate::pages::refresh_drill(state, "all");
        return;
    }
    if find_with_attr(target, "data-pm-adm-drill-close").is_some() {
        crate::pages::close_drill(state);
        return;
    }
    if let Some((_, uid)) = find_with_attr(target, "data-pm-adm-drill") {
        if let Ok(uid) = uid.trim().parse::<u64>() {
            crate::pages::open_drill(state, uid);
        }
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-drill-overlay") {
        if hit == target {
            crate::pages::close_drill(state);
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
    // 全局配置表单（切片 14）：公告弹窗内部按钮必须先于遮罩判定。
    if find_with_attr(target, "data-pm-adm-ann-save").is_some() {
        crate::pages::save_announcement(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-ann-close").is_some() {
        crate::pages::close_announcement(state);
        return;
    }
    if find_with_attr(target, "data-pm-adm-ann-new").is_some() {
        crate::pages::open_announcement(state, None);
        return;
    }
    if let Some((_, idx)) = find_with_attr(target, "data-pm-adm-ann-edit") {
        if let Ok(idx) = idx.trim().parse::<usize>() {
            crate::pages::open_announcement(state, Some(idx));
        }
        return;
    }
    if let Some((_, idx)) = find_with_attr(target, "data-pm-adm-ann-del") {
        if let Ok(idx) = idx.trim().parse::<usize>() {
            crate::pages::remove_announcement(state, idx);
        }
        return;
    }
    if let Some((_, key)) = find_with_attr(target, "data-pm-adm-cfg-toggle") {
        crate::pages::toggle_config_bool(state, &key);
        return;
    }
    if find_with_attr(target, "data-pm-adm-config-rawmode").is_some() {
        crate::pages::toggle_config_raw_mode(state);
        return;
    }
    if let Some((hit, _)) = find_with_attr(target, "data-pm-adm-ann-overlay") {
        if hit == target {
            crate::pages::close_announcement(state);
        }
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
