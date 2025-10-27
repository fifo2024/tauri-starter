// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::info;
use std::thread::{sleep, spawn};
use std::time::Duration;
use tauri::tray::TrayIconBuilder;
use tauri::webview::WebviewWindowBuilder;
use tauri::window::{Effect, EffectsBuilder};
use tauri::{Manager, WebviewUrl, Window};
mod tray;

// #[tauri::command]
// async fn create_window(app_handle: tauri::AppHandle) -> Result<(), String> {
//     WebviewWindowBuilder::new(&app_handle, "main", WebviewUrl::App("index.html".into()))
//         .title("My App")
//         .inner_size(800.0, 600.0)
//         .resizable(true)
//         .decorations(true)
//         .build()
//         .map_err(|e| e.to_string())?;

//     Ok(())
// }

// main.rs
// #[tauri::command]
// async fn send_message(window: Window, target: String, message: String) -> Result<(), String> {
//     if let Some(target_window) = window.app_handle().get_window(&target) {
//         target_window
//             .emit("message", message)
//             .map_err(|e| e.to_string())?;
//     }
//     Ok(())
// }
// change-menu-status
//
// use tauri::{
//     image::Image,
//     menu::{CheckMenuItemBuilder, IconMenuItem, MenuBuilder, MenuItem, SubmenuBuilder},
// };
//
// fn main() {
//     tauri::Builder::default()
//         .setup(|app| {
//             let check_sub_item_en = CheckMenuItemBuilder::with_id("en", "EN")
//                 .checked(true)
//                 .build(app)?;
//
//             let check_sub_item_zh = CheckMenuItemBuilder::with_id("zh", "ZH")
//                 .checked(false)
//                 .build(app)?;
//
//             let text_menu = MenuItem::with_id(
//                 app,
//                 "change_text",
//                 &"Change menu".to_string(),
//                 true,
//                 Some("Ctrl+Z"),
//             )
//             .unwrap();
//
//             let menu_item = SubmenuBuilder::new(app, "Change menu")
//                 .item(&text_menu)
//                 .items(&[&check_sub_item_en, &check_sub_item_zh])
//                 .build()?;
//             let menu = MenuBuilder::new(app).items(&[&menu_item]).build()?;
//             app.set_menu(menu)?;
//             app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
//                 match event.id().0.as_str() {
//                     "change_text" => {
//                         text_menu
//                             .set_text("changed menu text")
//                             .expect("Change text error");
//
//                         text_menu
//                             .set_text("changed menu text")
//                             .expect("Change text error");
//                     }
//                     "en" | "zh" => {
//                         check_sub_item_en
//                             .set_checked(event.id().0.as_str() == "en")
//                             .expect("Change check error");
//                         check_sub_item_zh
//                             .set_checked(event.id().0.as_str() == "zh")
//                             .expect("Change check error");
//                         check_sub_item_zh
//                             .set_accelerator(Some("Ctrl+L"))
//                             .expect("Change accelerator error");
//                     }
//                     _ => {
//                         println!("unexpected menu event");
//                     }
//                 }
//             });
//
//             Ok(())
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("Setup started"); // 调试输出
            info!("This will be logged to a file!");
            // let win_width = 800.;
            // let win_height = 600.;
            //
            // let mut window_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
            //
            // window_builder = window_builder
            //     // 设置窗口大小
            //     .inner_size(win_width, win_height);
            //
            // window_builder.build()?;
            let main_window = app.get_webview_window("main").unwrap();
            let splashscreen_window = app.get_webview_window("splashscreen").unwrap();

            println!("Windows acquired"); // 调试输出

            // spawn(move || {
            //     // 模拟初始化过程
            //     initialize_app().unwrap();
            //     println!("{}", "exe".to_string());
            //     splashscreen_window.close().unwrap();
            //     main_window.show().unwrap();
            // });

            // 使用 Tauri 的异步运行时
            tauri::async_runtime::spawn(async move {
                println!("Spawned thread started"); // 调试输出

                match initialize_app().await {
                    Ok(_) => {
                        println!("Initialization completed successfully");
                        splashscreen_window
                            .close()
                            .expect("Failed to close splashscreen");
                        main_window.show().expect("Failed to show main window");
                    }
                    Err(e) => {
                        eprintln!("Initialization failed: {}", e);
                        splashscreen_window
                            .close()
                            .expect("Failed to close splashscreen");
                    }
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build());
    tauri_starter_lib::run();
}

async fn initialize_app() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initialization started"); // 调试输出
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("Initialization finished"); // 调试输出
    Ok(())
}

// #[tauri::command]
// async fn close_splashscreen(window: Window) {
//     // Close splashscreen
//     window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
//     // Show main window
//     window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
// }

// fn initialize_app() -> Result<(), Box<dyn std::error::Error>> {
//     // 这里放置你的初始化代码
//     sleep(Duration::from_secs(2));
//     Ok(())
// }

// Create the command:
// This command must be async so that it doesn't run on the main thread.
// #[tauri::command]
// async fn close_splashscreen(window: Window) {
//     // Close splashscreen
//     window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
//     // Show main window
//     window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
// }

// let tray = TrayIconBuilder::new().build(app)?;
// let tray = TrayIconBuilder::new()
//     .icon(app.default_window_icon().unwrap().clone())
//     .build(app);

// fn main() {
//     tauri::Builder::default()
//         .setup(move |app| {
//             let win_width = 800.;
//             let win_height = 700.;

//             let effects = EffectsBuilder::new()
//                 .effects(vec![Effect::Acrylic, Effect::Blur])
//                 .build();

//             let mut window_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

//             window_builder = window_builder
//                 // 设置窗口大小
//                 .inner_size(win_width, win_height)
//                 // 设置窗口透明背景
//                 // .transparent(true)
//                 // 设置窗口磨砂背景
//                 .effects(effects)
//                 // 使用黑色主题
//                 .theme(Some(tauri::Theme::Dark));

//             let _ = window_builder.build()?;

//             Ok(())
//         })
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
