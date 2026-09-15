// 导入核心模块
pub mod core;

use crate::core::commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_image_info_command,
            read_image_list_command,
            get_directory_images_command,
            convert_image_format_command,
            resize_image_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
