use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().setup(|app| {
        let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default(),)
            .title("AnoGO",)
            .min_inner_size(420.0, 720.0,)
            .inner_size(420.0, 720.0,)
            .center();

        #[cfg(not(target_os = "macos"))]
        let win_builder = win_builder.decorations(false,).transparent(true,);

        #[cfg(target_os = "macos")]
        let win_builder = win_builder
            .decorations(true,)
            .hidden_title(true,)
            .title_bar_style(tauri::TitleBarStyle::Overlay,);

        let _window = win_builder.build().unwrap();

        #[cfg(dev)]
        _window.open_devtools();

        Ok((),)
    },);

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let _ = app
                    .get_webview_window("main",)
                    .expect("no main window",)
                    .set_focus();
            },),)
            .plugin(tauri_plugin_updater::Builder::new().build(),);
    }

    builder
        .plugin(tauri_plugin_dialog::init(),)
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ),)
                .level(log::LevelFilter::Debug,)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal,)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll,)
                .build(),
        )
        .plugin(tauri_plugin_os::init(),)
        .plugin(tauri_plugin_process::init(),)
        .plugin(tauri_plugin_clipboard_manager::init(),)
        .plugin(tauri_plugin_opener::init(),)
        .invoke_handler(tauri::generate_handler![
            commands::list_games,
            commands::write_file,
            commands::analyze_figure
        ],)
        .run(tauri::generate_context!(),)
        .expect("error while running tauri application",);
}
