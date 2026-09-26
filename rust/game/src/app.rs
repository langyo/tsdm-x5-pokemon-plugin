use crate::prelude::*;

use crate::components::{
    common::{AppProviders, GlobalModalProvider, PopupProvider},
    layout::Layout,
};

#[component]
pub fn App() -> Element {
    rsx! {
        AppProviders {
            PopupProvider {
                GlobalModalProvider {
                    Layout {}
                }
            }
        }
    }
}
