use crate::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct AppState {
    pub loading: bool,
    pub loading_message: String,
}

impl AppState {
    pub fn start_loading(&mut self, message: &str) {
        self.loading = true;
        self.loading_message = message.to_string();
    }

    pub fn stop_loading(&mut self) {
        self.loading = false;
        self.loading_message = String::new();
    }
}

pub fn set_global_loading(loading: bool, message: Option<&str>) {
    let mut state = crate::state::APP_STATE.write();
    if loading {
        state.start_loading(message.unwrap_or("加载中..."));
    } else {
        state.stop_loading();
    }
}

pub fn start_global_loading(message: &str) {
    set_global_loading(true, Some(message));
}

pub fn stop_global_loading() {
    set_global_loading(false, None);
}

pub fn use_global_loading() -> bool {
    crate::state::APP_STATE.read().loading
}

pub fn get_loading_message() -> String {
    crate::state::APP_STATE.read().loading_message.clone()
}
