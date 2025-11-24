pub mod listener;

use crate::state::AppState;

pub fn start(state: &AppState) {
    listener::start_keylogger(state.key_counter.clone());
}