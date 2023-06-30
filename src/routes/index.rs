use axum::{response::Html, Extension};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use std::sync::Arc;
use tera::{Context, Tera};
use lazy_static::lazy_static;
use regex::Regex;
#[derive(Serialize)]
struct PostToShow {
    id: i32,
    title: String,
    date: String,
    description: String,
    img_path: String,
}

#[derive(FromRow)]
struct DatabasePost {
    id: i32,
    title: String,
    date: String,
    image_ext: String,
    description: String,
}


pub async fn index(
    Extension(pool): Extension<SqlitePool>,
    Extension(templates): Extension<Arc<Tera>>,
) -> Html<String> { 
    lazy_static! {
        static ref RE: Regex = Regex::new("<[^<]+?>").unwrap();
    }
    let mut context = Context::new();
    let query = "SELECT id, title, date, image_ext, description FROM posts";
    let rows = sqlx::query_as::<_, DatabasePost>(query)
        .fetch_all(&pool)
        .await
        .unwrap();
    let mut posts: Vec<PostToShow> = Vec::new();
    for row in rows {
        posts.push(PostToShow {
            id: row.id,
            title: row.title,
            date: row.date,
            description: row.description,
            img_path: format!("/images/{}.{}", row.id, row.image_ext),
        });
    }
    context.insert("posts", &posts);

    Html(templates.render("index", &context).unwrap())
}
