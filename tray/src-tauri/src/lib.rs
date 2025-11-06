// Module declarations
mod audio_thread;
mod commands;
mod composer;
mod config;
mod mapper;
mod metrics;

use commands::AppState;
use single_instance::SingleInstance;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Ensure only one instance is running
    let instance = SingleInstance::new("syssonic-tray").unwrap();
    if !instance.is_single() {
        eprintln!("Another instance of SysSonic is already running");
        std::process::exit(1);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize application state
            app.manage(AppState::new());

            // Create system tray menu
            let show_hide = MenuItemBuilder::with_id("show_hide", "Show/Hide Window").build(app)?;
            let separator1 = tauri::menu::PredefinedMenuItem::separator(app)?;

            let start = MenuItemBuilder::with_id("start", "▶ Start Sonification").build(app)?;
            let stop = MenuItemBuilder::with_id("stop", "⏹ Stop").build(app)?;

            let separator2 = tauri::menu::PredefinedMenuItem::separator(app)?;

            let volume_25 = MenuItemBuilder::with_id("vol_25", "25%").build(app)?;
            let volume_50 = MenuItemBuilder::with_id("vol_50", "50%").build(app)?;
            let volume_75 = MenuItemBuilder::with_id("vol_75", "75%").build(app)?;
            let volume_100 = MenuItemBuilder::with_id("vol_100", "100%").build(app)?;

            let volume_menu = tauri::menu::SubmenuBuilder::new(app, "Volume")
                .item(&volume_25)
                .item(&volume_50)
                .item(&volume_75)
                .item(&volume_100)
                .build()?;

            let separator3 = tauri::menu::PredefinedMenuItem::separator(app)?;

            let export = MenuItemBuilder::with_id("export", "💾 Export Snapshot").build(app)?;
            let settings = MenuItemBuilder::with_id("settings", "⚙ Settings").build(app)?;

            let separator4 = tauri::menu::PredefinedMenuItem::separator(app)?;

            let quit = MenuItemBuilder::with_id("quit", "❌ Quit").build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&show_hide)
                .item(&separator1)
                .item(&start)
                .item(&stop)
                .item(&separator2)
                .item(&volume_menu)
                .item(&separator3)
                .item(&export)
                .item(&settings)
                .item(&separator4)
                .item(&quit)
                .build()?;

            // Create tray icon
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show_hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    "start" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-command", "start");
                        }
                    }
                    "stop" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-command", "stop");
                        }
                    }
                    "vol_25" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-command", serde_json::json!({"action": "volume", "value": 0.25}));
                        }
                    }
                    "vol_50" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-command", serde_json::json!({"action": "volume", "value": 0.50}));
                        }
                    }
                    "vol_75" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-command", serde_json::json!({"action": "volume", "value": 0.75}));
                        }
                    }
                    "vol_100" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-command", serde_json::json!({"action": "volume", "value": 1.0}));
                        }
                    }
                    "export" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("tray-command", "export");
                        }
                    }
                    "settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            let _ = window.emit("tray-command", "settings");
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Start with window hidden if configured
            if let Some(window) = app.get_webview_window("main") {
                let state: tauri::State<AppState> = app.state();
                let config = state.config.lock().unwrap();
                if config.start_minimized {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_audio,
            commands::stop_audio,
            commands::pause_audio,
            commands::resume_audio,
            commands::set_volume,
            commands::get_audio_state,
            commands::get_current_metrics,
            commands::get_musical_params,
            commands::export_audio,
            commands::get_config,
            commands::save_config,
            commands::update_config_field,
            commands::poll_audio_events,
            commands::get_system_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
