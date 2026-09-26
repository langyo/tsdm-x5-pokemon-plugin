use crate::prelude::*;

use crate::components::{
    common::{AppProviders, ErrorPopup, GlobalModalProvider, ModalProvider, PopupProvider},
    layout::Layout,
};

#[component]
pub fn App() -> Element {
    rsx! {
        AppProviders {
            ModalProvider {
                PopupProvider {
                    GlobalModalProvider {
                        Layout {}
                        ErrorPopup {}
                    }
                }
            }
        }
    }
}
