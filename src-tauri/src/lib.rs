mod datatypes;
mod ws;
mod clip;

use std::sync::{LazyLock, OnceLock};

use datatypes::WsSender;

use tauri::{async_runtime::{self, Mutex}, AppHandle, Manager};

use crate::{clip::start_clip, ws::start_ws};

const WS_URL: &str = "ws://localhost:3000";

static CLIPBOARD: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
static CURRENT_CLIP: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
static ENABLED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn app_handle() -> &'static AppHandle {
    APP_HANDLE.get().unwrap()
}

#[tauri::command]
fn toggle() {
    async_runtime::spawn(async {
        let mut enabled = ENABLED.lock().await;
        *enabled = !*enabled;
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx) = async_runtime::channel(100);
    tauri::Builder::default()
        .manage(WsSender { tx })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![toggle])
        .setup(move |app| {
            APP_HANDLE.set(app.app_handle().to_owned()).unwrap();
            app.get_webview_window("main").unwrap().set_title("ClipSync").unwrap();
            start_clip();
            start_ws(WS_URL.to_string(), rx);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
