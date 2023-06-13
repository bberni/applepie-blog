use axum::response::IntoResponse;
use http::Response;

pub async fn styles_handler() -> impl IntoResponse {
    Response::builder()
        .status(http::StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../static/styles.css").to_owned())
        .unwrap()

}