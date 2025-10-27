use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuItem};
use tauri::tray::TrayIconBuilder;

// // 托盘菜单
// pub fn build_menu(app: &AppHandle) -> Menu<()> {
//     let menu = Menu::new().add_item(CustomMenuItem::new("quit".to_string(), "Quit"));
//     menu // 返回
// }

// // // 托盘事件
// pub fn handler(app: &AppHandle) {
//     use tauri::tray::TrayIconBuilder;

//     TrayIconBuilder::new().on_menu_event(|app, event| match event.id.as_ref() {
//         "quit" => {
//             println!("quit menu item was clicked");
//             app.exit(0);
//         }
//         _ => {
//             println!("menu item {:?} not handled", event.id);
//         }
//     });
// }
//
// fn handle_open_coco(app: tauri::AppHandle) {
//     println!("Open coco");
// }
//
// fn handle_hide_coco(app: tauri::AppHandle) {
//     println!("Hide coco");
// }

pub fn enable_tray(app: &mut tauri::App) {
    // 退出按钮
    let quit_i = MenuItem::with_id(app, "quit", "Quit Coco", true, None::<&str>).unwrap();
    // 设置按钮
    let settings_i = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>).unwrap();
    // 打开按钮
    let open_i = MenuItem::with_id(app, "open", "Open Coco", true, None::<&str>).unwrap();
    // 关于按钮
    let about_i = MenuItem::with_id(app, "about", "About Coco", true, None::<&str>).unwrap();
    // 隐藏按钮
    let hide_i = MenuItem::with_id(app, "hide", "Hide Coco", true, None::<&str>).unwrap();

    // 按照一定顺序，把按钮放到菜单里
    let menu = MenuBuilder::new(app)
        .item(&open_i)
        .separator()
        .item(&hide_i)
        .item(&about_i)
        .item(&settings_i)
        .separator()
        .item(&quit_i)
        .build()
        .unwrap();

    let _tray = TrayIconBuilder::with_id("tray")
        .icon(Image::from_bytes(include_bytes!("../icons/icon.png")).expect("Failed to load icon"))
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "open" => {
                println!("quit menu open item was clicked");
                // handle_open_coco(tauri::AppHandle::from(app));
            }
            "hide" => {
                println!("quit menu hide item was clicked");
                // handle_hide_coco(tauri::AppHandle::from(app));
            }
            "about" => {
                println!("quit menu about item was clicked");
                // let _ = app.emit("open_settings", "about");
            }
            "settings" => {
                println!("quit menu settings item was clicked");
                // let _ = app.emit("open_settings", "");
            }
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("unhandled menu item id: {}", "hello");
            }
        })
        .build(app)
        .unwrap();
}
