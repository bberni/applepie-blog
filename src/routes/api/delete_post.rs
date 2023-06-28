use axum::{extract::Extension, Json};

use crate::check_hash;
use serde::Deserialize;
use sqlx::SqlitePool;

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

    let query = "DELETE FROM posts WHERE id = ?";
    match sqlx::query(query).bind(params.id).execute(&pool).await {
        Ok(_) => {}
        Err(_) => {return http::StatusCode::INTERNAL_SERVER_ERROR}
        ,
    };

    return http::StatusCode::OK;
}
