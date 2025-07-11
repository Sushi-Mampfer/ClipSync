use serde::{Deserialize, Serialize};
use tauri::async_runtime::Sender;

pub struct WsSender {
    pub tx: Sender<String>
}

#[derive(Serialize, Deserialize)]
pub struct WsMsg {
    pub id: String,
    pub data: String,
}