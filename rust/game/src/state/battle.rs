use crate::prelude::*;
use web_sys::window;

use _utils::types::api_battle::BattleScene;

#[derive(Clone, PartialEq, Default)]
pub struct BattleState {
    pub scene: Option<BattleScene>,
    pub loading: bool,
}

/// 切换宠物请求信号（用于 sidebar 触发切换宠物）
pub static SWITCH_POKEMON_REQUEST: GlobalSignal<bool> = Signal::global(|| false);

/// 替换宠物请求状态（持久化，用于宠物被打死后需要替换）
pub static REPLACE_POKEMON_REQUEST: GlobalSignal<bool> = Signal::global(|| false);

const STORAGE_KEY_REPLACE: &str = "pokemon_replace_pending";

/// 最近一次开战的地图 ID（持久化）。
/// 战斗结束响应里服务端不返回 map_id（build_battle_response 仅在开战时带地图），
/// "继续战斗"必须靠这里记住的值，否则永远拿 0 而静默失效。
const STORAGE_KEY_LAST_MAP: &str = "pokemon_last_map_id";

pub fn remember_map_id(map_id: u64) {
    if map_id == 0 {
        return;
    }
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(STORAGE_KEY_LAST_MAP, &map_id.to_string());
        }
    }
}

pub fn get_last_map_id() -> u64 {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(value)) = storage.get_item(STORAGE_KEY_LAST_MAP) {
                if let Ok(parsed) = value.parse::<u64>() {
                    return parsed;
                }
            }
        }
    }
    0
}

pub fn set_battle_scene(scene: Option<BattleScene>) {
    crate::state::BATTLE_STATE.write().scene = scene.clone();

    // 如果战斗中有宠物倒下且需要替换，持久化状态
    // 依据服务端的 battle_over：宠物倒下但还有替补时 status 会是 defeat 而
    // battle_over=false（战斗继续，应弹换宠）；只有 battle_over=true 才算终局
    if let Some(scene) = &scene {
        // 开战响应带 map_id，此时记下来供"继续战斗"使用
        if scene.status == _utils::types::api_battle::BattleStatus::Active {
            remember_map_id(scene.map_id);
        }

        let needs_replace = scene.my_pokemon.hp <= 0 && !scene.battle_over;

        // 检查是否有可用替换宠物
        let has_replacements = crate::state::POKEMON_STATE
            .read()
            .list
            .iter()
            .filter(|p| p.hp > 0 && (p.site == 1 || p.site == 2))
            .count()
            > 0;

        if needs_replace && has_replacements {
            save_replace_state(true);
        } else {
            save_replace_state(false);
        }
    } else {
        save_replace_state(false);
    }
}

pub fn clear_battle_scene() {
    crate::state::BATTLE_STATE.write().scene = None;
    save_replace_state(false);
}

pub fn use_battle_state() -> bool {
    crate::state::BATTLE_STATE
        .read()
        .scene
        .as_ref()
        // 宠物倒下但还有替补时战斗并未结束（status 可能是 defeat），
        // 以 battle_over 为准
        .map(|scene| !scene.battle_over)
        .unwrap_or(false)
}

/// 触发切换宠物请求
pub fn request_switch_pokemon() {
    *SWITCH_POKEMON_REQUEST.write() = true;
}

/// 检查是否有切换宠物请求
pub fn take_switch_pokemon_request() -> bool {
    let has_request = *SWITCH_POKEMON_REQUEST.read();
    if has_request {
        *SWITCH_POKEMON_REQUEST.write() = false;
    }
    has_request
}

/// 保存替换状态到 localStorage
fn save_replace_state(value: bool) {
    *REPLACE_POKEMON_REQUEST.write() = value;
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(STORAGE_KEY_REPLACE, &value.to_string());
        }
    }
}

/// 从 localStorage 加载替换状态
pub fn load_replace_state() -> bool {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(value)) = storage.get_item(STORAGE_KEY_REPLACE) {
                if let Ok(parsed) = value.parse::<bool>() {
                    *REPLACE_POKEMON_REQUEST.write() = parsed;
                    return parsed;
                }
            }
        }
    }
    false
}

/// 清除替换状态
pub fn clear_replace_state() {
    save_replace_state(false);
}

/// 检查是否有替换宠物请求
pub fn take_replace_pokemon_request() -> bool {
    let has_request = *REPLACE_POKEMON_REQUEST.read();
    if has_request {
        *REPLACE_POKEMON_REQUEST.write() = false;
        save_replace_state(false);
    }
    has_request
}
