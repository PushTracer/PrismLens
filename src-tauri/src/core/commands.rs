use crate::core::image::{
    convert_image_format, get_directory_images, read_image_info, read_image_list, resize_image,
    rotate_image, ImageBasicInfo, ImageInfo,
};
use tauri::command;

/// 获取图片信息命令
#[command]
pub async fn read_image_info_command(path: String) -> Result<ImageInfo, String> {
    read_image_info(&path).map_err(|e| e.to_string())
}

/// 读取目录中的所有图片命令
#[command]
pub async fn read_image_list_command(dir_path: String) -> Result<Vec<ImageInfo>, String> {
    read_image_list(&dir_path).map_err(|e| e.to_string())
}

/// 获取目录中的所有图片简略信息命令
#[command]
pub async fn get_directory_images_command(dir_path: String) -> Result<Vec<ImageBasicInfo>, String> {
    get_directory_images(&dir_path).map_err(|e| e.to_string())
}

/// 旋转图片命令
#[command]
pub async fn rotate_image_command(path: String, angle: i32) -> Result<String, String> {
    rotate_image(&path, angle).map_err(|e| e.to_string())
}

/// 转换图片格式命令
#[command]
pub async fn convert_image_format_command(path: String, format: String) -> Result<String, String> {
    convert_image_format(&path, &format).map_err(|e| e.to_string())
}

/// 调整图片大小命令
#[command]
pub async fn resize_image_command(path: String, width: u32, height: u32) -> Result<String, String> {
    resize_image(&path, width, height).map_err(|e| e.to_string())
}
