use std::io::Read;
use std::path::PathBuf;
use tauri::http::response::Builder;
use tauri::http::{Request as HttpRequest, Response as HttpResponse};
use tauri::{Runtime, UriSchemeContext};

/// 处理自定义的 prism:// 协议请求
/// 根据请求的路径读取本地文件内容并作为响应返回
pub fn prism_protocol_handler<R: Runtime>(
    //  保持泛型参数 R: Runtime
    _context: UriSchemeContext<'_, R>, //  保持 UriSchemeContext 参数
    request: HttpRequest<Vec<u8>>,     //  保持 HttpRequest 参数
) -> HttpResponse<Vec<u8>> {
    //  返回类型改为直接返回 HttpResponse
    // 响应也要带 Vec<u8>
    // 获取请求的路径，去掉协议头 "prism://"
    let path = request.uri().to_string().replace("prism://", "");
    let file_path = PathBuf::from(path);

    // 检查文件是否存在并读取内容
    if file_path.exists() && file_path.is_file() {
        match std::fs::File::open(&file_path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        // 根据文件扩展名设置 Content-Type
                        let mime_type = match file_path.extension().and_then(|s| s.to_str()) {
                            Some("png") => "image/png",
                            Some("jpg") | Some("jpeg") => "image/jpeg",
                            Some("gif") => "image/gif",
                            Some("bmp") => "image/bmp",
                            Some("webp") => "image/webp",
                            _ => "application/octet-stream", // 默认类型
                        };

                        // 构建 HTTP 响应
                        match Builder::new()
                            .header("Content-Type", mime_type)
                            .body(buffer)
                        {
                            Ok(response) => response,
                            Err(_) => Builder::new()
                                .status(500)
                                .body("Internal Server Error".as_bytes().to_vec())
                                .unwrap(), // 构建失败返回500
                        }
                    }
                    Err(_) => Builder::new()
                        .status(500)
                        .body("Failed to read file".as_bytes().to_vec())
                        .unwrap(), // 读取失败返回500
                }
            }
            Err(_) => Builder::new()
                .status(404)
                .body("File not found".as_bytes().to_vec())
                .unwrap(), // 打开失败返回404
        }
    } else {
        // 文件不存在或不是文件，返回 404 错误
        Builder::new()
            .status(404)
            .body("File not found".as_bytes().to_vec())
            .unwrap() // 构建失败返回404
    }
}
