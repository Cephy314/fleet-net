#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod connection;
mod radio;

use crate::connection::ServerConnection;
use fleet_net_common::types::ServerInfo;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

struct AppState {
    connection: Arc<Mutex<ServerConnection>>,
}

#[tauri::command]
async fn connect_to_server(
    state: State<'_, AppState>,
    server_addr: String,
    cert_path: Option<String>,
) -> Result<String, String> {
    let mut conn = state.connection.lock().await;

    conn.connect(&server_addr, cert_path.as_deref())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_server_info(state: State<'_, AppState>) -> Result<ServerInfo, String> {
    let conn = state.connection.lock().await;
    conn.read_server_info()
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    // Initialize Logging
    fleet_net_common::logging::init_tracing();

    let app_state = AppState {
        connection: Arc::new(Mutex::new(ServerConnection::new())),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![connect_to_server, get_server_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
