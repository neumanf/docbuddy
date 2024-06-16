use tauri::{App, Window, GlobalShortcutManager};

pub fn update_default(app: &mut App, window: &Window) {
    let mut shortcuts = app.global_shortcut_manager();

    shortcuts.register("Alt+D", move || {
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            window.hide().unwrap();
            window.show().unwrap();
        }
    });
}

pub fn update_close(app: &mut App, window: Window) {
    let mut shortcuts = app.global_shortcut_manager();

    shortcuts.register("Escape", move || {
        window.hide().unwrap();
    });
}
