use dioxus::prelude::*;

use super::{Page, CURRENT_PAGE};

/// 图片资源基础路径 (对应 PHP $imgpath)
pub const IMG_PATH: &str = "source/plugin/pokemon/images";

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "app-header",
            div { class: "header-inner",
                // 左侧 Logo：精灵球 + 品牌名
                div { class: "header-logo",
                    a {
                        class: "logo-link",
                        onclick: move |_| *CURRENT_PAGE.write() = Page::Home,
                        img {
                            class: "logo-mascot",
                            src: "{IMG_PATH}/item/jlq.gif",
                            alt: "Pokeball",
                        }
                        span { class: "logo-text", "Online Void Pokemon" }
                    }
                }

                // 右侧导航 — 设计稿: 个人中心 商店 宠物中心 野外冒险
                nav { class: "header-nav",
                    a {
                        class: if *CURRENT_PAGE.read() == Page::MyPokemon { "nav-link active" } else { "nav-link" },
                        onclick: move |_| *CURRENT_PAGE.write() = Page::MyPokemon,
                        "个人中心"
                    }
                    a {
                        class: if *CURRENT_PAGE.read() == Page::Shop { "nav-link active" } else { "nav-link" },
                        onclick: move |_| *CURRENT_PAGE.write() = Page::Shop,
                        "商店"
                    }
                    a {
                        class: if *CURRENT_PAGE.read() == Page::PokemonCenter { "nav-link active" } else { "nav-link" },
                        onclick: move |_| *CURRENT_PAGE.write() = Page::PokemonCenter,
                        "宠物中心"
                    }
                    a {
                        class: if *CURRENT_PAGE.read() == Page::Adventure { "nav-link active" } else { "nav-link" },
                        onclick: move |_| *CURRENT_PAGE.write() = Page::Adventure,
                        "野外冒险"
                    }
                }
            }
        }
    }
}
