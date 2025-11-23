use reqwest::cookie::CookieStore;
use std::sync::{Arc, Mutex};
use tauri::Manager;

mod commands;
use commands::{
    api_key::{get_api_key, set_api_key},
    auth::{am_logged_in, login},
};

struct AppData {
    cookie_jar: Arc<reqwest::cookie::Jar>,
    api_key: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthCache {
    cookies: Option<String>,
    api_key: Option<String>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    {
        // Workaround for rendering issues on linux according to https://github.com/tauri-apps/tauri/issues/13493#issuecomment-3006864836
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODR", "1")
    }

    // load cookies from file
    let cookie_jar = reqwest::cookie::Jar::default();
    let mut api_key = None;
    if let Ok(cookie_str) = std::fs::read_to_string("auth.json") {
        if let Ok(cookie_json) = serde_json::from_str::<AuthCache>(&cookie_str) {
            if let Some(cookies) = &cookie_json.cookies {
                let domain = reqwest::Url::parse("https://api.vrchat.cloud").unwrap();
                cookie_jar.add_cookie_str(cookies, &domain);
            }
            api_key = cookie_json.api_key;
        }
    }

    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData {
                cookie_jar: Arc::new(cookie_jar),
                api_key,
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            am_logged_in,
            login,
            get_api_key,
            set_api_key
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, e| match e {
            tauri::RunEvent::ExitRequested { .. } => {
                let state = app.state::<Mutex<AppData>>();
                let state = state.lock().unwrap();

                let domain = reqwest::Url::parse("https://api.vrchat.cloud").unwrap();
                println!("All cookies {:?}", state.cookie_jar);
                let header = state.cookie_jar.clone().cookies(&domain).unwrap();
                let cookie = header.to_str().unwrap();

                // overwrite the cookie file
                let auth_cache = AuthCache {
                    cookies: Some(cookie.to_string()),
                    api_key: state.api_key.clone(),
                };
                let auth_json = serde_json::to_string_pretty(&auth_cache).unwrap();
                std::fs::write("auth.json", auth_json).unwrap();
            }
            _ => {}
        });
}
