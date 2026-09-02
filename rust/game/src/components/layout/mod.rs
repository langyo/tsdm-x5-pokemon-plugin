mod footer;
mod header;
mod page_container;
mod sidebar;

use dioxus::prelude::*;
pub use footer::Footer;
pub use header::{Header, IMG_PATH};
pub use page_container::{LayoutMode, PageContainer};
pub use sidebar::Sidebar;

use crate::{
    components::{common::LoadingOverlay, page_content::PageContent},
    pages::Welcome,
    state::{use_inventory_state, use_pokemon_state, use_user_profile_state, USER_STATE},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Page {
    Home,
    MyPokemon,
    Shop,
    PokemonCenter,
    Adventure,
    Inventory,
    PokemonStorage,
}

impl Page {
    pub fn label(&self) -> &'static str {
        match self {
            Page::Home => "首页",
            Page::MyPokemon => "个人中心",
            Page::Shop => "商店",
            Page::PokemonCenter => "宠物中心",
            Page::Adventure => "野外冒险",
            Page::Inventory => "背包",
            Page::PokemonStorage => "宠物仓库",
        }
    }
}

pub static CURRENT_PAGE: GlobalSignal<Page> = Signal::global(|| Page::Home);

pub static INITIAL_INVENTORY_CATEGORY: GlobalSignal<Option<u32>> = Signal::global(|| None);

fn get_layout_mode(page: Page) -> LayoutMode {
    match page {
        Page::Adventure | Page::Home => LayoutMode::FullWidth,
        _ => LayoutMode::Default,
    }
}

fn should_show_sidebar(page: Page) -> bool {
    !matches!(page, Page::Home)
}

#[component]
pub fn Layout() -> Element {
    use_user_profile_state();
    use_inventory_state();
    use_pokemon_state();

    let page = *CURRENT_PAGE.read();
    let layout_mode = get_layout_mode(page);
    let show_sidebar = should_show_sidebar(page);

    let (is_logged_in, needs_welcome) = {
        let user_state = USER_STATE.read();
        let profile = &user_state.profile;
        let logged_in = profile.is_some();
        let welcome = profile
            .as_ref()
            .map(|p| {
                let is_new = p.is_new_player;
                let total_pm = p.total_pokemons.unwrap_or(0);
                is_new || total_pm == 0
            })
            .unwrap_or(false);
        (logged_in, welcome)
    };

    rsx! {
        div { class: "app-layout",
            if !is_logged_in {
                div { class: "loading-container",
                    div { class: "loading-spinner" }
                    p { "加载中..." }
                }
            } else if needs_welcome {
                Welcome {}
            } else {
                Header {}

                div { class: "app-main",
                    div { class: "app-content",
                        PageContainer { mode: layout_mode, PageContent {} }
                    }
                    if show_sidebar {
                        Sidebar {}
                    }
                }

                Footer {}
            }
            LoadingOverlay {}
        }
    }
}
