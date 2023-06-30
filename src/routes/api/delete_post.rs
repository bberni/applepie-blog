use axum::{extract::Extension, Json};
use std::fs;
use crate::check_hash;
use serde::Deserialize;
use sqlx::{SqlitePool, Row};

#[derive(Deserialize)]
pub struct FromCMS {
    hash: String,
    id: i32,
}
pub async fn delete_post(
    Extension(pool): Extension<SqlitePool>,
    Json(params): Json<FromCMS>,
) -> http::StatusCode {

    if check_hash(params.hash) != 1 {
        return http::StatusCode::FORBIDDEN;
    };
    let ext_query  = "SELECT image_ext FROM posts WHERE id = ?";
    let result = match sqlx::query(ext_query).bind(params.id).fetch_one(&pool).await {
        Ok(x) => x, 
        Err(_) => {return http::StatusCode::INTERNAL_SERVER_ERROR;}
    }; 
    let ext: String = result.get("image_ext");

    let query = "DELETE FROM posts WHERE id = ?";
    match sqlx::query(query).bind(params.id).execute(&pool).await {
        Ok(_) => {}
        Err(_) => {return http::StatusCode::INTERNAL_SERVER_ERROR}
        ,
    };
    match fs::remove_file(format!("src/images/{}.{}", params.id, ext)) { 
        Err(_) => {return http::StatusCode::INTERNAL_SERVER_ERROR},
        _ => {}
    }
    return http::StatusCode::OK;
}
