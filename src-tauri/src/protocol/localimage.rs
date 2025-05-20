use std::{fs, thread};
use tauri::{
    http::{Request, Response},
    Builder, Runtime, UriSchemeContext, UriSchemeResponder,
};

pub fn register_local_image_protocol<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.register_asynchronous_uri_scheme_protocol(
        "local-image",
        move |_ctx: UriSchemeContext<R>,
              request: Request<Vec<u8>>,
              responder: UriSchemeResponder| {
            let path = request.uri().path()[1..].to_string();
            thread::spawn(move || match fs::read(&path) {
                Ok(data) => {
                    let mime_type = mime_guess::from_path(&path)
                        .first_or_octet_stream()
                        .essence_str()
                        .to_string();
                    let response = Response::builder()
                        .header("Content-Type", mime_type)
                        .status(200)
                        .body(data)
                        .unwrap();
                    responder.respond(response);
                }
                Err(err) => {
                    let response = Response::builder()
                        .header("Content-Type", "text/plain")
                        .status(404)
                        .body(format!("Failed to read {}: {}", path, err).into_bytes())
                        .unwrap();
                    responder.respond(response);
                }
            });
        },
    )
}
