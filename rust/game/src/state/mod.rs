mod app;
mod battle;
mod pokemon;
mod ui;
mod user;

use crate::prelude::*;
pub use app::*;
pub use battle::*;
pub use pokemon::*;
pub use ui::*;
pub use user::*;

// 重新导出 modal 相关函数
pub use ui::{close_modal, is_modal_open, open_modal};

pub static APP_STATE: GlobalSignal<AppState> = Signal::global(AppState::default);
pub static USER_STATE: GlobalSignal<UserState> = Signal::global(UserState::default);
pub static POKEMON_STATE: GlobalSignal<PokemonState> = Signal::global(PokemonState::default);
pub static BATTLE_STATE: GlobalSignal<BattleState> = Signal::global(BattleState::default);
pub static UI_STATE: GlobalSignal<UIState> = Signal::global(UIState::default);

pub fn reset_all_states() {
    *APP_STATE.write() = AppState::default();
    *USER_STATE.write() = UserState::default();
    *POKEMON_STATE.write() = PokemonState::default();
    *BATTLE_STATE.write() = BattleState::default();
    *UI_STATE.write() = UIState::default();
}
