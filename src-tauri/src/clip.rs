use std::{time::Duration};

use clippers::Clipboard;
use tauri::{async_runtime, Emitter};
use tokio::time::sleep;

use crate::{app_handle, datatypes::WsMsg, ws::send_msg, CLIPBOARD, CURRENT_CLIP, ENABLED};

pub fn start_clip() {
    async_runtime::spawn(async {
        clip_loop().await;
    });
}

pub async fn clip_loop() {
    loop {
        sleep(Duration::from_millis(10)).await;
        {
            if !*ENABLED.lock().await {
                continue;
            }
        }
        let text_opt = {
            let _lock = CLIPBOARD.lock().await;
            Clipboard::get().read().and_then(|d| d.as_text().map(|t| t.to_owned()))
        };
        if let Some(text) = text_opt {
            let text_2 = text.clone();
            let changed = {
                let mut current = CURRENT_CLIP.lock().await;
                if text != *current {
                    *current = text;
                    true
                } else {
                    false
                }
            };
            if changed {
                send_msg(WsMsg { id: "clip".to_owned(), data: text_2.to_owned() }).await;
                app_handle().emit("clip", text_2.to_owned()).unwrap();
            }
        }
    }
}

pub async fn set_clip(text: String) {
    if !*ENABLED.lock().await {
        return
    }
    let _lock = CLIPBOARD.lock().await;
    let mut current_clip = CURRENT_CLIP.lock().await;
    if *current_clip != text {
        *current_clip = text.clone();
        Clipboard::get().write_text(text).unwrap();
    }
}