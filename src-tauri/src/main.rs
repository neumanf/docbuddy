// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use tauri::{
    App, CustomMenuItem, Manager, PhysicalPosition, PhysicalSize, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};

mod commands;
mod shortcuts;

#[tokio::main]
async fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "open" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .setup(setup)
        .invoke_handler(tauri::generate_handler![commands::search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<(dyn Error + 'static)>> {
    let handle = app.handle();
    let window = handle.get_window("main").unwrap();

    let screen = window.current_monitor()?.unwrap();
    let screen_position = screen.position();
    let screen_size = PhysicalSize::<i32> {
        width: screen.size().width as i32,
        height: screen.size().height as i32,
    };
    let window_size = PhysicalSize::<i32> {
        width: window.outer_size()?.width as i32,
        height: window.outer_size()?.height as i32,
    };

    let center_offset = 200;
    let physical_pos = PhysicalPosition {
        x: screen_position.x + ((screen_size.width / 2) - (window_size.width / 2)),
        y: screen_position.y + ((screen_size.height / 2) - (window_size.height / 2)) - center_offset,
    };

    let _ = window.set_position(tauri::Position::Physical(physical_pos));

    shortcuts::update_default(app, &window)?;
    shortcuts::update_close(app, &window)?;

    Ok(())
}
