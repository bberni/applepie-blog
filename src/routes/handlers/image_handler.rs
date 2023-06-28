use axum::{body, extract::Path, response::IntoResponse};
use http::Response;
use mime_guess;
use std::fs::File;
use std::io::Read;

pub async fn image_handler(Path(id): Path<String>) -> impl IntoResponse {
    let filepath = "src/images/".to_owned() + &id;
    let mimetype = mime_guess::from_path(&filepath).first_raw().unwrap();
    let mut file = File::open(filepath).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    Response::builder()
        .status(http::StatusCode::OK)
        .header("Content-Type", mimetype)
        .body(body::boxed(body::Full::from(buffer)))
        .unwrap()
}
