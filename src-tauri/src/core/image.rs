use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 图片信息结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageInfo {
    pub path: String,   // 图片路径
    pub name: String,   // 图片名称
    pub width: u32,     // 图片宽度
    pub height: u32,    // 图片高度
    pub format: String, // 图片格式
    pub size: u64,      // 图片大小（字节）
    pub modified: u64,  // 最后修改时间
}

/// 图片处理错误
#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("图片处理错误: {0}")]
    Image(#[from] image::ImageError),

    #[error("无效的路径: {0}")]
    InvalidPath(String),

    #[error("不支持的图片格式")]
    UnsupportedFormat,

    #[error("其他错误: {0}")]
    Other(String),
}

/// 图片处理结果类型
pub type Result<T> = std::result::Result<T, ImageError>;

/// 获取图片信息
pub fn get_image_info(path: &str) -> Result<ImageInfo> {
    let path_obj = Path::new(path);

    // 检查文件是否存在
    if !path_obj.exists() {
        return Err(ImageError::InvalidPath(path.to_string()));
    }

    // 获取文件元数据
    let metadata = fs::metadata(path)?;

    // 读取图片
    let img = image::open(path)?;

    // 获取文件名
    let name = path_obj
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // 获取图片格式
    let format = path_obj
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("unknown")
        .to_lowercase();

    // 获取修改时间
    let modified = metadata
        .modified()
        .map(|time| {
            time.duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
        .unwrap_or(0);

    Ok(ImageInfo {
        path: path.to_string(),
        name,
        width: img.width(),
        height: img.height(),
        format,
        size: metadata.len(),
        modified,
    })
}

/// 读取目录中的所有图片
pub fn read_image_list(dir_path: &str) -> Result<Vec<ImageInfo>> {
    let path = Path::new(dir_path);

    if !path.exists() || !path.is_dir() {
        return Err(ImageError::InvalidPath(dir_path.to_string()));
    }

    let mut images = Vec::new();

    // 支持的图片格式扩展名
    let supported_formats = [
        "jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "tiff", "tif",
    ];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        // 检查是否为文件且扩展名是支持的图片格式
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if supported_formats.contains(&ext.to_lowercase().as_str()) {
                    if let Ok(info) = get_image_info(path.to_str().unwrap_or_default()) {
                        images.push(info);
                    }
                }
            }
        }
    }

    Ok(images)
}

/// 旋转图片
pub fn rotate_image(path: &str, angle: i32) -> Result<String> {
    // 读取图片
    let mut img = image::open(path)?;

    // 根据角度旋转
    img = match angle {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => return Err(ImageError::Other(format!("不支持的旋转角度: {}", angle))),
    };

    // 创建输出路径
    let path_obj = Path::new(path);
    let file_stem = path_obj
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    let extension = path_obj
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");

    let parent = path_obj.parent().unwrap_or(Path::new(""));
    let output_path = parent.join(format!("{}_rotated.{}", file_stem, extension));

    // 保存图片
    img.save(&output_path)?;

    Ok(output_path.to_str().unwrap_or_default().to_string())
}

/// 转换图片格式
pub fn convert_image_format(path: &str, format: &str) -> Result<String> {
    // 读取图片
    let img = image::open(path)?;

    // 获取目标格式
    let target_format = match format.to_lowercase().as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "gif" => ImageFormat::Gif,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "ico" => ImageFormat::Ico,
        _ => return Err(ImageError::UnsupportedFormat),
    };

    // 创建输出路径
    let path_obj = Path::new(path);
    let file_stem = path_obj
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    let parent = path_obj.parent().unwrap_or(Path::new(""));
    let output_path = parent.join(format!("{}.{}", file_stem, format.to_lowercase()));

    // 保存图片
    img.save_with_format(&output_path, target_format)?;

    Ok(output_path.to_str().unwrap_or_default().to_string())
}

/// 调整图片大小
pub fn resize_image(path: &str, width: u32, height: u32) -> Result<String> {
    // 读取图片
    let img = image::open(path)?;

    // 调整大小
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    // 创建输出路径
    let path_obj = Path::new(path);
    let file_stem = path_obj
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image");

    let extension = path_obj
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");

    let parent = path_obj.parent().unwrap_or(Path::new(""));
    let output_path = parent.join(format!("{}_resized.{}", file_stem, extension));

    // 保存图片
    resized.save(&output_path)?;

    Ok(output_path.to_str().unwrap_or_default().to_string())
}
