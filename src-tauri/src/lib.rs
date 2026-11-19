pub mod command;
pub mod common;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let preferences = common::config::CatalogPreferences::new(
                app.path().app_config_dir()?,
                std::env::var("AOSKA_ASMR_URL").ok(),
            );
            app.manage(command::AppState::new(preferences));
            Ok(())
        })
        .manage(common::amo::AmoBackend::default())
        .invoke_handler(tauri::generate_handler![
            command::fetch_catalog_json,
            command::get_catalog_settings,
            command::set_catalog_settings,
            command::pm_capabilities,
            command::pm_list_updates,
            command::pm_update_summary,
            command::pm_tum_updates,
            command::pm_list_installed,
            command::pm_list_catalog_installed,
            command::app_launch,
            command::pm_package_state,
            command::pm_plan_update,
            command::pm_plan_install,
            command::pm_plan_remove,
            command::pm_start_update,
            command::pm_start_install,
            command::pm_start_remove,
            command::pm_start_refresh,
            command::pm_operation_status,
            command::pm_operation_result,
            command::pm_operation_logs,
            command::pm_cancel_operation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
