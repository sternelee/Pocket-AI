// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod errors;
mod init;
mod utils;
mod log_utils;
mod services;

use chrono::Local;
use log::LevelFilter;
use tauri::Manager;
use tauri_plugin_log::{
    fern::colors::{Color, ColoredLevelConfig},
    Target, 
    TargetKind,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let colors = ColoredLevelConfig {
        error: Color::Red,
        warn: Color::Yellow,
        debug: Color::Green,
        info: Color::Blue,
        trace: Color::White,
    };
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::create_model,
            commands::list_models,
            commands::update_model,
            commands::delete_model,
            commands::list_remote_models,
            commands::list_settings,
            commands::upsert_setting,
            commands::create_conversation,
            commands::create_blank_conversation,
            commands::list_conversations,
            commands::delete_conversation,
            commands::update_conversation,
            commands::get_options,
            commands::update_options,
            commands::update_subject,
            commands::update_conversation_model,
            commands::create_message,
            commands::list_messages,
            commands::get_system_message,
            commands::update_message,
            commands::hard_delete_messages,
            commands::hard_delete_message,
            commands::call_bot,
            commands::create_prompt,
            commands::list_prompts,
            commands::update_prompt,
            commands::delete_prompt,
            commands::get_sys_info,
        ])
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([Target::new(TargetKind::Stdout), Target::new(TargetKind::Webview)])
                .level(LevelFilter::Debug)
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        colors.color(record.level()),
                        record.target(),
                        message
                    ))
                })
                .build(),
        )
        .setup(|app| {
            // Open dev tools in debug builds
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            // Initialization
            init::init(app).expect("Failed to initialize app");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
