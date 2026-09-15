use image::DynamicImage;
use image::ImageFormat;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// 支持的图片格式扩展名
pub const SUPPORTED_IMAGE_EXTENSIONS: [&str; 9] = [
    "jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "tiff", "tif",
];

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

/// 图片简略信息结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageBasicInfo {
    pub path: String,  // 图片路径
    pub name: String,  // 图片名称
    pub size: u64,     // 图片大小（字节）
    pub modified: u64, // 最后修改时间
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

/// 判断是否为受支持的图片文件
fn is_supported_image(path: &Path) -> bool {
    path.is_file()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| SUPPORTED_IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
}

/// 读取文件最后修改时间（Unix 秒），获取失败时返回 0
fn modified_secs(metadata: &fs::Metadata) -> u64 {
    metadata
        .modified()
        .map(|time| {
            time.duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
        .unwrap_or(0)
}

/// 校验调用方传入的输出路径，确保目录存在且路径非空
fn validate_output_path(output_path: &str) -> Result<PathBuf> {
    if output_path.trim().is_empty() {
        return Err(ImageError::InvalidPath(output_path.to_string()));
    }

    let path = PathBuf::from(output_path);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(ImageError::InvalidPath(output_path.to_string()));
        }
    }

    Ok(path)
}

/// 读取图片信息
pub fn read_image_info(path: &str) -> Result<ImageInfo> {
    let path_obj = Path::new(path);

    // 检查文件是否存在
    if !path_obj.exists() {
        return Err(ImageError::InvalidPath(path.to_string()));
    }

    // 获取文件元数据
    let metadata = fs::metadata(path)?;

    // 只读取图片头部信息
    let reader = ImageReader::open(path)?;
    let dimensions = reader.into_dimensions()?;

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

    Ok(ImageInfo {
        path: path.to_string(),
        name,
        width: dimensions.0,
        height: dimensions.1,
        format,
        size: metadata.len(),
        modified: modified_secs(&metadata),
    })
}

/// 读取目录中的所有图片 （读取全部信息，性能消耗过大，暂时不使用）
pub fn read_image_list(dir_path: &str) -> Result<Vec<ImageInfo>> {
    let path = Path::new(dir_path);

    if !path.exists() || !path.is_dir() {
        return Err(ImageError::InvalidPath(dir_path.to_string()));
    }

    let mut images = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if is_supported_image(&path) {
            if let Ok(info) = read_image_info(path.to_str().unwrap_or_default()) {
                images.push(info);
            }
        }
    }

    Ok(images)
}

/// 获取目录中所有图片的基础信息
pub fn get_directory_images(dir_path: &str) -> Result<Vec<ImageBasicInfo>> {
    let path = Path::new(dir_path);

    if !path.exists() || !path.is_dir() {
        return Err(ImageError::InvalidPath(dir_path.to_string()));
    }

    let mut images = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if !is_supported_image(&path) {
            continue;
        }

        let metadata = fs::metadata(&path)?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        images.push(ImageBasicInfo {
            path: path.to_string_lossy().into_owned(),
            name,
            size: metadata.len(),
            modified: modified_secs(&metadata),
        });
    }

    // 按文件名排序，保证展示顺序稳定
    images.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(images)
}

/// 转换图片格式，保存到调用方指定的路径
pub fn convert_image_format(path: &str, format: &str, output_path: &str) -> Result<String> {
    let format = format.to_lowercase();

    // 获取目标格式
    let target_format = match format.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "gif" => ImageFormat::Gif,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "ico" => ImageFormat::Ico,
        _ => return Err(ImageError::UnsupportedFormat),
    };

    let output_path = validate_output_path(output_path)?;

    let img = image::open(path)?;

    // 如果目标格式是 JPEG，去除透明度通道
    let img = if matches!(target_format, ImageFormat::Jpeg) {
        DynamicImage::ImageRgb8(img.to_rgb8())
    } else {
        img
    };

    // 保存图片
    img.save_with_format(&output_path, target_format)?;

    Ok(output_path.to_string_lossy().into_owned())
}

/// 调整图片大小，保存到调用方指定的路径
pub fn resize_image(path: &str, width: u32, height: u32, output_path: &str) -> Result<String> {
    if width == 0 || height == 0 {
        return Err(ImageError::Other("宽度和高度必须大于 0".to_string()));
    }

    let output_path = validate_output_path(output_path)?;

    // 读取图片
    let img = image::open(path)?;

    // 调整大小
    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    // 保存图片
    resized.save(&output_path)?;

    Ok(output_path.to_string_lossy().into_owned())
}
