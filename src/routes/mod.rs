mod api;
mod handlers;
mod index;
mod show_post;
use api::{checkpass::checkpass, delete_post::delete_post, new_post::new_post, post_list::post_list};
use axum::{
    body::Body,
    routing::{get, post},
    Extension, Router,
};
use handlers::{image_handler::image_handler, styles_handler::styles_handler};
use index::index;
use show_post::show_post;
use sqlx::sqlite::SqlitePoolOptions;
use std::{fs::File, path::Path, sync::Arc};

use tera::Tera;

pub async fn create_routes() -> Router<(), Body> {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("base", include_str!("../templates/base.html")),
        ("index", include_str!("../templates/index.html")),
        ("post", include_str!("../templates/post.html")),
    ])
    .unwrap();

    if !Path::new("./database.sqlite3").exists() {
        File::create("./database.sqlite3").unwrap();
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect("sqlite:database.sqlite3")
        .await
        .unwrap();
    let create_table_query = "
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            date TEXT,
            image_ext TEXT,
            content TEXT
        )
    ";
    sqlx::query(&create_table_query)
        .execute(&pool)
        .await
        .unwrap();

    Router::new()
        .route("/", get(index))
        .route("/styles.css", get(styles_handler))
        .route("/images/:id", get(image_handler))
        .route("/api/checkpass", post(checkpass))
        .route("/api/new_post", post(new_post))
        .route("/api/delete_post", post(delete_post))
        .route("/post", get(show_post))
        .route("/api/post_list", post(post_list))
        .layer(Extension(Arc::new(tera)))
        .layer(Extension(pool))
}
