use axum::Json;
use hex;
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
pub struct Params {
    hash: String,
}
pub async fn checkpass(Json(params): Json<Params>) -> String {
    let password = include_str!("../../password.txt");
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hex::encode(hasher.finalize());
    println!("{:?}", result);
    if result == params.hash {
        return "1".to_owned();
    } else {
        return "0".to_owned();
    }
}
