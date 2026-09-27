use crate::prelude::*;

use _utils::types::api_pokemon::PokemonBasic;

#[derive(Clone, PartialEq, Default)]
pub struct PokemonState {
    pub list: Vec<PokemonBasic>,
    pub loading: bool,
    pub error: Option<String>,
    pub loaded: bool,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum MyPokemonTab {
    #[default]
    Stats,
    Equipment,
    Skills,
}

#[derive(Clone, PartialEq, Default)]
pub struct SelectedItem {
    pub source_type: String,
    pub myitem_id: u64,
    pub name: String,
    pub image: String,
    pub slot_index: Option<u32>,
}

impl PokemonState {
    pub fn get_by_id(&self, id: u64) -> Option<&PokemonBasic> {
        self.list.iter().find(|p| p.id == id)
    }

    pub fn get_bag_pokemons(&self) -> Vec<PokemonBasic> {
        self.list
            .iter()
            .filter(|p| p.site == 1 || p.site == 2)
            .cloned()
            .collect()
    }

    pub fn get_storage_pokemons(&self) -> Vec<PokemonBasic> {
        // 旧版数据用 site 3..N 表示多个箱子（boxnum 决定 N），不能只认 site == 3
        self.list.iter().filter(|p| p.site >= 3).cloned().collect()
    }

    pub fn get_first_pokemon(&self) -> Option<&PokemonBasic> {
        self.list.iter().find(|p| p.site == 1)
    }

    pub fn get_injured_pokemons(&self) -> Vec<PokemonBasic> {
        // 仅限身上携带的宠物（战斗前后自动治疗只针对它们）
        self.list
            .iter()
            .filter(|p| {
                (p.site == 1 || p.site == 2) && (p.hp < p.max_hp || is_negative_state(p.state))
            })
            .cloned()
            .collect()
    }
}

fn is_negative_state(state: u8) -> bool {
    // 与 utils::is_negative_state 保持一致：20-22 为虚弱状态，同样需要治疗
    matches!(state, 0 | 2 | 3 | 4 | 5 | 6 | 7 | 11 | 15 | 20 | 21 | 22)
}

pub fn use_pokemon_state() {
    let state = crate::state::POKEMON_STATE.read();
    let loaded = state.loaded;
    let loading = state.loading;
    drop(state);

    use_effect(move || {
        if loaded || loading {
            return;
        }

        spawn(async move {
            let mut state = crate::state::POKEMON_STATE.write();
            state.loading = true;
            state.error = None;
            drop(state);

            let api = crate::utils::api_client::NewApiClient::new();
            match api.get_pokemon_list().await {
                Ok(data) => {
                    let mut state = crate::state::POKEMON_STATE.write();
                    state.list = data.pokemons;
                    state.loaded = true;
                    state.loading = false;
                }
                Err(e) => {
                    let mut state = crate::state::POKEMON_STATE.write();
                    state.error = Some(format!("加载宠物列表失败: {}", e));
                    state.loading = false;
                }
            }
        });
    });
}

pub fn refresh_pokemon_list() {
    spawn(async move {
        let mut state = crate::state::POKEMON_STATE.write();
        state.loading = true;
        state.error = None;
        drop(state);

        let api = crate::utils::api_client::NewApiClient::new();
        match api.get_pokemon_list().await {
            Ok(data) => {
                let mut state = crate::state::POKEMON_STATE.write();
                state.list = data.pokemons;
                state.loading = false;
            }
            Err(e) => {
                let mut state = crate::state::POKEMON_STATE.write();
                state.error = Some(format!("刷新宠物列表失败: {}", e));
                state.loading = false;
            }
        }
    });
}

pub fn update_pokemon_hp(pokemon_id: u64, new_hp: i64, new_max_hp: i64) {
    let mut state = crate::state::POKEMON_STATE.write();
    for pm in state.list.iter_mut() {
        if pm.id == pokemon_id {
            pm.hp = new_hp;
            pm.max_hp = new_max_hp;
            break;
        }
    }
}

pub static MY_POKEMON_TAB: GlobalSignal<MyPokemonTab> = Signal::global(MyPokemonTab::default);
pub static EQUIPMENT_BONUSES: GlobalSignal<Vec<(String, i64)>> = Signal::global(Vec::new);
pub static SELECTED_ITEM: GlobalSignal<Option<SelectedItem>> = Signal::global(|| None);
pub static SELECTED_POKEMON_INDEX: GlobalSignal<Option<u64>> = Signal::global(|| None);

#[cfg(test)]
mod tests {
    use super::is_negative_state;

    #[test]
    fn weak_states_count_as_negative() {
        assert!(is_negative_state(20));
        assert!(is_negative_state(21));
        assert!(is_negative_state(22));
    }

    #[test]
    fn healthy_and_positive_states_are_not_negative() {
        assert!(!is_negative_state(1));
        assert!(!is_negative_state(8));
        assert!(!is_negative_state(12));
        assert!(!is_negative_state(16));
        assert!(!is_negative_state(18));
    }
}
