use crate::core::image::{
    convert_image_format, get_image_info, read_image_list, resize_image, rotate_image, ImageInfo,
};
use tauri::command;

/// 示例问候函数
#[command]
pub fn greet(name: &str) -> String {
    format!("你好, {}! 欢迎使用 PrismLens!", name)
}

/// 获取图片信息命令
#[command]
pub fn get_image_info_command(path: String) -> Result<ImageInfo, String> {
    get_image_info(&path).map_err(|e| e.to_string())
}

/// 读取目录中的所有图片命令
#[command]
pub fn read_image_list_command(dir_path: String) -> Result<Vec<ImageInfo>, String> {
    read_image_list(&dir_path).map_err(|e| e.to_string())
}

/// 旋转图片命令
#[command]
pub fn rotate_image_command(path: String, angle: i32) -> Result<String, String> {
    rotate_image(&path, angle).map_err(|e| e.to_string())
}

/// 转换图片格式命令
#[command]
pub fn convert_image_format_command(path: String, format: String) -> Result<String, String> {
    convert_image_format(&path, &format).map_err(|e| e.to_string())
}

/// 调整图片大小命令
#[command]
pub fn resize_image_command(path: String, width: u32, height: u32) -> Result<String, String> {
    resize_image(&path, width, height).map_err(|e| e.to_string())
}
