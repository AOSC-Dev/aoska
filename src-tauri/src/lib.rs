pub mod command;
pub mod common;
#[cfg(target_os = "linux")]
mod window_frame;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let builder =
                tauri::WebviewWindowBuilder::from_config(app, &app.config().app.windows[0])?;
            #[cfg(target_os = "linux")]
            window_frame::build(builder)?;
            #[cfg(not(target_os = "linux"))]
            builder.build()?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .manage(command::AppState::default())
        .manage(common::amo::AmoBackend::default())
        .invoke_handler(tauri::generate_handler![
            command::fetch_catalog_json,
            command::get_endpoint_base_url,
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
