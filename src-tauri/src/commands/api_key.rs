use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn get_api_key(state: State<'_, Mutex<AppData>>) -> Option<String> {
    let state = state.lock().unwrap();
    state.api_key.clone()
}

#[tauri::command]
pub fn set_api_key(state: State<'_, Mutex<AppData>>, api_key: String) {
    let mut state = state.lock().unwrap();
    state.api_key = Some(api_key);
}
