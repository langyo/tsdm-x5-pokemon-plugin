use crate::prelude::*;

use crate::components::common::{LoadingOverlay, ToastProvider};

#[component]
pub fn AppProviders(children: Element) -> Element {
    rsx! {
        ToastProvider {}
        LoadingOverlay {}
        {children}
    }
}
