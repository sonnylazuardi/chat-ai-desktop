#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let github = CustomMenuItem::new("github".to_string(), "View on Github");
    let twitter = CustomMenuItem::new("twitter".to_string(), "Author on Twitter");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github)
        .add_item(twitter);

    let tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                app.get_window("main").unwrap().show().unwrap();
                app.get_window("main").unwrap().set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "twitter" => {
                    api::shell::open(
                        &app.get_window("main").unwrap().shell_scope(),
                        "https://twitter.com/sonnylazuardi".to_string(),
                        None,
                    )
                    .unwrap();
                }
                "github" => {
                    api::shell::open(
                        &app.get_window("main").unwrap().shell_scope(),
                        "https://github.com/sonnylazuardi/chatgpt-desktop".to_string(),
                        None,
                    )
                    .unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(context)
        .expect("error while running tauri application");
}
