#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{ CustomMenuItem, Menu };



fn main() {
    let close = CustomMenuItem::new("close".to_string(), "Schließen");
    let menu = Menu::new()
        .add_item(close);

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "close" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .run(context)
        .expect("error while running tauri application");
}
