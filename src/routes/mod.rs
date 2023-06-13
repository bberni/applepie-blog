mod index;
mod styles_handler;
mod image_handler;
mod checkpass;
use axum::{
    body::Body, 
    routing::{get, post},
    Router,
    Extension
};
use std::sync::Arc;
use tera::Tera;
use index::index;
use sqlx::sqlite::SqlitePoolOptions;
use styles_handler::styles_handler;
use image_handler::image_handler;
use checkpass::checkpass;
pub async fn create_routes() -> Router<(), Body> {

    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("base", include_str!("../templates/base.html")),
        ("index", include_str!("../templates/index.html")),
    ])
    .unwrap();
    
    let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect("sqlite:database.sqlite3")
            .await.unwrap();
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            date TEXT,
            image_ext TEXT,
            content TEXT
        )
    "#;
    sqlx::query(&create_table_query)
        .execute(&pool)
        .await.unwrap();

    Router::new()
        .route("/", get(index))
        .route("/styles.css", get(styles_handler))
        .route("/images/:id", get(image_handler))
        .route("/api/checkpass", post(checkpass))
        .layer(Extension(Arc::new(tera)))
        .layer(Extension(pool))

}