// 导入核心模块
pub mod core;
pub mod protocol;

use crate::core::commands::*;
use protocol::localimage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_image_info_command,
            read_image_list_command,
            rotate_image_command,
            convert_image_format_command,
            resize_image_command
        ]);
    // 2. 注册本地图片协议
    let builder = localimage::register_local_image_protocol(builder);

    // 3. 运行
    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
