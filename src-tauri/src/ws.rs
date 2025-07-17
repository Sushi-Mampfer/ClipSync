use futures_util::{SinkExt, StreamExt};
use serde_json::{from_str, to_string};
use tauri::{async_runtime::{self, Receiver}, Emitter, Manager};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{app_handle, clip::set_clip, datatypes::{WsMsg, WsSender}, ENABLED};

async fn handle_msg(msg: &str) {
    if msg == "" {
        return
    }
    let msg: WsMsg = from_str(msg).unwrap();
    if msg.id == "con" {
        app_handle().emit("con", msg.data).unwrap();
    } else if msg.id == "clip" {
        set_clip(msg.data.clone()).await;
        app_handle().emit("clip", msg.data.clone()).unwrap();
        println!("Clip emited: {}", msg.data);
    }
}

pub async fn send_msg(msg: WsMsg) {
    match app_handle().state::<WsSender>().tx.send(to_string(&msg).unwrap()).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error sending to the websocket: {}", e);
            app_handle().emit("err", "websocket").unwrap();
            {
                *ENABLED.lock().await = false;
            }
            return;
        }
    }
}

pub fn start_ws(url: String, mut rx: Receiver<String>) {
    async_runtime::spawn(async move {
        let ws = match connect_async(&url).await {
            Ok((ws, _)) => ws,
            Err(e) => {
                eprintln!("Error connecting to the websocket: {}", e);
                app_handle().emit("err", "websocket").unwrap();
                {
                    *ENABLED.lock().await = false;
                }
                return;
            }
        };
        let (mut ws_send, mut ws_recv) = ws.split();
        let ws_writer = async_runtime::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = ws_send.send(Message::Text(msg.into())).await {
                    eprintln!("Error sending to the websocket: {}", e);
                    app_handle().emit("err", "websocket").unwrap();
                    {
                        *ENABLED.lock().await = false;
                    }
                    return;
                }
            }
        });

        while let Some(msg) = ws_recv.next().await {
            if let Ok(msg) = msg {
                handle_msg(msg.to_text().unwrap()).await;
            }
        }

        ws_writer.await.ok();
    });
}