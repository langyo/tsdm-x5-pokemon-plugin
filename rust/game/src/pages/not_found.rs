use crate::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page-home",
            div { class: "welcome-message",
                h1 { "欢迎来到宠物世界" }
                p { "在这个梦想和冒险相伴的宠物世界一定要有勇气走下去！" }
            }
        }
    }
}
