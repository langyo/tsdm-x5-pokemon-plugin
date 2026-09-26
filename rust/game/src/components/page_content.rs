use crate::prelude::*;

use crate::{
    components::layout::{Page, CURRENT_PAGE},
    pages::{Adventure, Home, Inventory, MyPokemon, PokemonCenter, PokemonStorage, Shop},
};

#[component]
pub fn PageContent() -> Element {
    let page = *CURRENT_PAGE.read();

    rsx! {
        match page {
            Page::Home => rsx! {
                Home {}
            },
            Page::MyPokemon => rsx! {
                MyPokemon {}
            },
            Page::Shop => rsx! {
                Shop {}
            },
            Page::PokemonCenter => rsx! {
                PokemonCenter {}
            },
            Page::Adventure => rsx! {
                Adventure {}
            },
            Page::Inventory => rsx! {
                Inventory {}
            },
            Page::PokemonStorage => rsx! {
                PokemonStorage {}
            },
        }
    }
}
