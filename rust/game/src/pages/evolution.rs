use crate::prelude::*;

use crate::{
    components::common::Card,
    state::{refresh_pokemon_list, show_error, show_success, use_pokemon_state, POKEMON_STATE},
    utils::api_client::NewApiClient,
};
use _utils::types::api_evolution::EvolutionCheckResponse;

#[component]
pub fn Evolution() -> Element {
    use_pokemon_state();

    let mut checking = use_signal(|| None::<u64>);
    let mut evolving = use_signal(|| false);
    let mut evolution_check_result = use_signal(|| None::<(u64, EvolutionCheckResponse)>);

    let mut check_evolution = move |pet_id: u64| {
        checking.set(Some(pet_id));
        evolution_check_result.set(None);
        let api = NewApiClient::new();

        spawn(async move {
            match api.check_evolution(pet_id).await {
                Ok(result) => {
                    if result.can_evolve {
                        evolution_check_result.set(Some((pet_id, result)));
                    } else {
                        show_error(format!("无法进化: {}", result.reason));
                    }
                }

                Err(e) => {
                    show_error(format!("检查失败: {}", e));
                }
            }

            checking.set(None);
        });
    };

    let mut evolve = move |pet_id: u64| {
        evolving.set(true);
        let api = NewApiClient::new();

        spawn(async move {
            match api.evolve(pet_id).await {
                Ok(result) => {
                    show_success(format!("进化成功！已进化为 {}", result.new_name));
                    evolution_check_result.set(None);
                    refresh_pokemon_list();
                }

                Err(e) => {
                    show_error(format!("进化失败: {}", e));
                }
            }

            evolving.set(false);
        });
    };

    let (list_data, pokemon_loading) = {
        let pokemon_state = POKEMON_STATE.read();
        let data = if pokemon_state.loaded {
            Some(pokemon_state.list.clone())
        } else {
            None
        };
        (data, pokemon_state.loading)
    };

    rsx! {
        div {

            class: "page-evolution",
            Card {

                title: "宝可梦进化".to_string(),
                div {

                    class: "evolution-intro",
                    h3 {
                        "宝可梦进化"
                    }

                    p {
                        "当宝可梦达到特定等级或满足特定条件时，它们可以进化成更强的形态！"
                    }
                }

                if let Some(pokemons)=list_data {
                    if pokemons.is_empty() {
                        p {
                            "没有可以进化的宝可梦"
                        }
                    }

                    else {
                        div {

                            class: "pokemon-list",
                                {
                                pokemons.into_iter().enumerate().map(|(idx, pokemon)| {
                                        let pet_id=pokemon.id;
                                        let is_checking=*checking.read()==Some(pet_id);
                                        let check_result=evolution_check_result.read();
                                        let can_evolve=check_result.as_ref() .map(|(id, result)| *id==pet_id && result.can_evolve) .unwrap_or(false);

                                        let evolution_info=check_result.as_ref() .and_then(|(id, result)| {
                                                if *id==pet_id {
                                                    Some(result.clone())
                                                }

                                                else {
                                                    None
                                                }
                                            }

                                        );

                                        rsx ! {
                                            div {

                                                key: "{idx}", class: "pokemon-card",
                                                div {

                                                    class: "pokemon-info",
                                                    h4 {
                                                        "{pokemon.name}"
                                                    }

                                                    p {
                                                        "Lv.{pokemon.level}"
                                                    }

                                                    p {
                                                        "ID: {pokemon.id}"
                                                    }
                                                }

                                                div {

                                                    class: "pokemon-actions",
                                                    button {

                                                        onclick: move |_| check_evolution(pet_id),
                                                        disabled: *evolving.read() || is_checking,
                                                        class: "check-button",
                                                            {
                                                            if is_checking {
                                                                "检查中..."
                                                            }

                                                            else {
                                                                "检查进化"
                                                            }
                                                        }
                                                    }

                                                    if can_evolve {
                                                        if let Some(info)=evolution_info {
                                                            div {

                                                                class: "evolution-info",
                                                                p {
                                                                    "可进化为: {info.target_form}"
                                                                }

                                                                p {
                                                                    "方式: {info.evolution_method}"
                                                                }

                                                                button {

                                                                    onclick: move |_| evolve(pet_id),
                                                                    disabled: *evolving.read(),
                                                                    class: "evolve-button",
                                                                        {
                                                                        if *evolving.read() {
                                                                            "进化中..."
                                                                        }

                                                                        else {
                                                                            "确认进化"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                )
                            }
                        }
                    }
                }

                if pokemon_loading {
                    div {
                        class: "loading", "加载中..."
                    }
                }
            }
        }
    }
}
