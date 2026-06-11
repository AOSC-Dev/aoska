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
            command::fetch_update_detail,
            command::get_endpoint_base_url,
            command::pm_capabilities,
            command::pm_list_updates,
            command::pm_update_summary,
            command::pm_list_installed,
            command::pm_package_state,
            command::pm_start_update,
            command::pm_start_install,
            command::pm_start_remove,
            command::pm_start_refresh,
            command::pm_operation_status,
            command::pm_operation_result,
            command::pm_operation_logs,
            command::pm_cancel_operation,
            command::start_upgrade,
            command::start_install,
            command::oma_is_busy,
            command::start_remove,
            command::oma_unit_status,
            command::oma_unit_logs,
            command::follow_oma_logs,
            command::stop_follow_oma_logs,
            command::oma_unit_result,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
