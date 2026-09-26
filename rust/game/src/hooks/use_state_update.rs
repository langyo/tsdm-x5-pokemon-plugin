use crate::prelude::*;

use crate::utils::api_client::NewApiClient;
use _utils::types::api_pokemon::UpdateStateResponse;

#[allow(dead_code)]
pub static STATE_UPDATE_RESULT: GlobalSignal<Option<UpdateStateResponse>> = Signal::global(|| None);
#[allow(dead_code)]
pub static STATE_UPDATE_LOADING: GlobalSignal<bool> = Signal::global(|| false);
#[allow(dead_code)]
pub static STATE_UPDATE_ERROR: GlobalSignal<Option<String>> = Signal::global(|| None);

#[allow(dead_code)]
pub static STATE_UPDATE_INTERVAL_MS: GlobalSignal<u64> = Signal::global(|| 300_000);
#[allow(dead_code)]
pub static STATE_UPDATE_ENABLED: GlobalSignal<bool> = Signal::global(|| true);

#[allow(dead_code)]
pub fn trigger_state_update() {
    spawn(async move {
        *STATE_UPDATE_LOADING.write() = true;

        let api = NewApiClient::new();
        match api.update_pokemon_state().await {
            Ok(response) => {
                *STATE_UPDATE_RESULT.write() = Some(response);
                *STATE_UPDATE_ERROR.write() = None;
            }
            Err(e) => {
                *STATE_UPDATE_ERROR.write() = Some(e.to_string());
            }
        }

        *STATE_UPDATE_LOADING.write() = false;
    });
}

#[allow(dead_code)]
pub fn get_state_multiplier_text(multiplier: f64) -> String {
    if multiplier > 1.0 {
        format!("+{:.0}%", (multiplier - 1.0) * 100.0)
    } else if multiplier < 1.0 {
        format!("-{:.0}%", (1.0 - multiplier) * 100.0)
    } else {
        String::new()
    }
}
