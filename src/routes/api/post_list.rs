use crate::check_hash;
use axum::{
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
#[derive(Deserialize)]
pub struct FromCMS {
    hash: String,
}

#[derive(Serialize, FromRow)]
struct Post {
    id: i32,
    title: String,
    date: String,
}

#[derive(Serialize)]
pub struct PostList {
    posts: Vec<Post>,
}

pub async fn post_list(
    Extension(pool): Extension<SqlitePool>,
    Json(params): Json<FromCMS>,
) -> Response {
    if check_hash(params.hash) != 1 {
        return StatusCode::FORBIDDEN.into_response();
    };

    let query = "SELECT id, title, date FROM posts";
    let rows = match sqlx::query_as::<_, Post>(query).fetch_all(&pool).await {
        Ok(posts) => posts,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    return Json(PostList { posts: rows }).into_response();
}
