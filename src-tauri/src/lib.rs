pub mod command;
pub mod common;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(command::AppState::default())
        .invoke_handler(tauri::generate_handler![
            command::fetch_by_category,
            command::fetch_detail,
            command::fetch_index,
            command::fetch_recommend,
            command::fetch_tum_update,
            command::fetch_update_count,
            command::fetch_update_detail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
