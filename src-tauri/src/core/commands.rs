use crate::core::image::{
    convert_image_format, get_directory_images, read_image_info, read_image_list, resize_image,
    ImageBasicInfo, ImageInfo,
};
use tauri::async_runtime::spawn_blocking;
use tauri::command;

/// 获取图片信息命令
#[command]
pub async fn read_image_info_command(path: String) -> Result<ImageInfo, String> {
    spawn_blocking(move || read_image_info(&path))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

/// 读取目录中的所有图片命令
#[command]
pub async fn read_image_list_command(dir_path: String) -> Result<Vec<ImageInfo>, String> {
    spawn_blocking(move || read_image_list(&dir_path))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

/// 获取目录中的所有图片简略信息命令
#[command]
pub async fn get_directory_images_command(dir_path: String) -> Result<Vec<ImageBasicInfo>, String> {
    spawn_blocking(move || get_directory_images(&dir_path))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

/// 转换图片格式命令（保存到调用方指定的路径）
#[command]
pub async fn convert_image_format_command(
    path: String,
    format: String,
    output_path: String,
) -> Result<String, String> {
    spawn_blocking(move || convert_image_format(&path, &format, &output_path))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

/// 调整图片大小命令（保存到调用方指定的路径）
#[command]
pub async fn resize_image_command(
    path: String,
    width: u32,
    height: u32,
    output_path: String,
) -> Result<String, String> {
    spawn_blocking(move || resize_image(&path, width, height, &output_path))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}
