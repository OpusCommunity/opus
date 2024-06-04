/**--------------------------------------------------------------------------------------------------
 *   Copyright (c) OpusCommunity. All rights reserved.
 *   Licensed under the MIT License. See License.txt in the project root for license information.
 *--------------------------------------------------------------------------------------------------*/
use log::{error, LevelFilter};
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_window_state::StateFlags;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn setup() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stderr),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .level(LevelFilter::Info)
                .level_for("app::config", LevelFilter::Trace)
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(windows) = app.get_webview_window("main") {
                if let Err(e) = windows.unminimize() {
                    error!("failed to unminimize main window: {e}");
                }
                if let Err(e) = windows.set_focus() {
                    error!("failed to set focus on main window: {e}");
                }
            }
        }))
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(StateFlags::all())
                .build(),
        )
        .plugin(state::Builder::default().build())
        .plugin(grpc::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
