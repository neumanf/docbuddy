use std::process::Command;

use tauri::{App, GlobalShortcutManager, Window};

pub fn update_default(app: &mut App, window: &Window) -> Result<(), tauri::Error> {
    let mut shortcuts = app.global_shortcut_manager();
    let window = window.clone();

    shortcuts.register("Alt+D", move || {
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            window.show().unwrap();
            Command::new("wmctrl")
                .args(["-a", "<sth>"])
                .output()
                .expect("Failed to set focus");
        }
    })?;

    Ok(())
}

pub fn update_close(app: &mut App, window: &Window) -> Result<(), tauri::Error> {
    let mut shortcuts = app.global_shortcut_manager();
    let window = window.clone();

    shortcuts.register("Escape", move || {
        window.hide().unwrap();
    })?;

    Ok(())
}
