use axum::Json;
use serde::Deserialize;
use crate::check_hash;
#[derive(Deserialize)]
pub struct Params {
    hash: String,
}
pub async fn checkpass(Json(params): Json<Params>) -> String {
    return check_hash(params.hash).to_string()
}
