// Import functionalities we'll be using
use log::info;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Manager, State};
use tokio::time::{sleep, Duration};
mod tray;

// Create a struct we'll use to track the completion of
// setup related tasks
struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

// Our main entrypoint in a version 2 mobile compatible app
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Don't write code before Tauri starts, write it in the
    // setup hook instead!
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        // Register a `State` to be managed by Tauri
        // We need write access to it so we wrap it in a `Mutex`
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        // Add a command we can use to check
        .invoke_handler(tauri::generate_handler![
            greet,
            set_complete,
            switch_tray_icon
        ])
        // Use the setup hook to execute setup related tasks
        // Runs before the main loop, so no windows are yet created
        .setup(|app| {
            // Spawn setup as a non-blocking task so the windows can be
            // created and ran while it executes
            spawn(setup(app.handle().clone()));
            // 延迟更新检查
            // tauri::async_runtime::spawn(async move {
            //     update(app.handle().clone()).await.unwrap();
            // });
            // 注册事件
            tray::enable_tray(app);

            // The hook expects an Ok result
            Ok(())
        })
        // Run the app
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
//     if let Some(update) = app.updater()?.check().await? {
//         let mut downloaded = 0;

//         // alternatively we could also call update.download() and update.install() separately
//         update
//             .download_and_install(
//                 |chunk_length, content_length| {
//                     downloaded += chunk_length;
//                     println!("downloaded {downloaded} from {content_length:?}");
//                 },
//                 || {
//                     println!("download finished");
//                 },
//             )
//             .await?;

//         println!("update installed");
//         app.restart();
//     }

//     Ok(())
// }

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello {name} from Rust!");
    info!("Hello {}!", name);
    info!("This will be logged to a file dddd!");
    name.to_string()
}

// A custom task for setting the state of a setup task
#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    // Lock the state without write access
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    // Check if both tasks are completed
    if state_lock.backend_task && state_lock.frontend_task {
        // Setup is complete, we can close the splashscreen
        // and unhide the main window!
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

// An async function that does some heavy setup task
async fn setup(app: AppHandle) -> Result<(), ()> {
    // Fake performing some heavy action for 3 seconds
    println!("Performing really heavy backend setup task...");
    sleep(Duration::from_secs(3)).await;
    println!("Backend setup task completed!");
    // Set the backend task as being completed
    // Commands can be ran as regular functions as long as you take
    // care of the input arguments yourself
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}

#[tauri::command]
fn switch_tray_icon(app: tauri::AppHandle, is_dark_mode: bool) {
    let app_handle = app.app_handle();

    println!("is_dark_mode: {}", is_dark_mode);

    const DARK_ICON_PATH: &[u8] = include_bytes!("../icons/dark.png");
    const LIGHT_ICON_PATH: &[u8] = include_bytes!("../icons/light.png");

    // 根据 app 的主题切换 图标
    let icon_path: &[u8] = if is_dark_mode {
        DARK_ICON_PATH
    } else {
        LIGHT_ICON_PATH
    };

    // 获取托盘
    let tray = match app_handle.tray_by_id("tray") {
        Some(tray) => tray,
        None => {
            eprintln!("Tray with ID 'tray' not found");
            return;
        }
    };

    // 设置图标
    if let Err(e) = tray.set_icon(Some(
        tauri::image::Image::from_bytes(icon_path)
            .unwrap_or_else(|e| panic!("Failed to load icon from bytes: {}", e)),
    )) {
        eprintln!("Failed to set tray icon: {}", e);
    }
}
