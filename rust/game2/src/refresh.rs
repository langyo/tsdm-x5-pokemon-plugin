//! 操作后的全局刷新（对齐 Dioxus 版 `state/user.rs` 的
//! `refresh_user_profile_state` / `refresh_inventory_state` 与 `state/pokemon.rs`
//! 的 `refresh_pokemon_list`）：买入、用道具、战斗动作之后，侧栏的金币、背包
//! 小格的数量和宠物列表要跟着服务端走。各页自己的列表由各页刷新，这里只管
//! 跨页共享的那三份。刷新失败时保留上一次的值（旧版同此：资料出错只记
//! `profile_error`，不动 `profile`）。

use crate::app_state::AppState;

/// 侧栏资料（金币、在线天数等）。不动路由与起始页判定——那是启动时
/// `load_profile` 的事。
pub fn profile(state: &AppState) {
    let st = state.clone();
    crate::api::profile(Box::new(move |result| {
        if let Ok(summary) = result {
            st.sidebar.profile.inner().set(Some(Ok(summary)));
        }
        crate::request_remount();
    }));
}

/// 侧栏背包小格的数量。
pub fn bag_counts(state: &AppState) {
    crate::sidebar::refresh_bag_counts(state);
}

/// 宠物列表（个人中心、宠物中心、仓库、侧栏迷你格共用）。
pub fn pokemon_list(state: &AppState) {
    let st = state.clone();
    crate::api::pokemon_list(Box::new(move |result| {
        st.center.pokemons.inner().set(Some(result));
        crate::request_remount();
    }));
}
