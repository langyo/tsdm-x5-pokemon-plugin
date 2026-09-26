use crate::dioxus::prelude::*;

use crate::dioxus::{
    components::{layout::AdminLayout, toast::AdminToastProvider},
    pages::{
        EvolutionDataPage, GlobalConfigPage, ItemDataPage, MapDataPage, PokemonDataPage,
        SkillDataPage, SqlConsolePage, UserDataPage,
    },
    state::{AdminRoute, ADMIN_BUSY},
};

#[component]
pub fn App() -> Element {
    let mut route = use_signal(AdminRoute::default);
    let busy = *ADMIN_BUSY.read();

    let content = match route() {
        AdminRoute::GlobalConfig => rsx! {
            GlobalConfigPage {}
        },
        AdminRoute::SqlConsole => rsx! {
            SqlConsolePage {}
        },
        AdminRoute::PokemonData => rsx! {
            PokemonDataPage {}
        },
        AdminRoute::ItemData => rsx! {
            ItemDataPage {}
        },
        AdminRoute::MapData => rsx! {
            MapDataPage {}
        },
        AdminRoute::UserData => rsx! {
            UserDataPage {}
        },
        AdminRoute::EvolutionData => rsx! {
            EvolutionDataPage {}
        },
        AdminRoute::SkillType => rsx! {
            SkillDataPage {}
        },
    };

    rsx! {
        AdminToastProvider {}
        AdminLayout {
            current: route(),
            on_navigate: move |next| route.set(next),
            busy,
            {content}
        }
    }
}
