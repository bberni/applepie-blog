use axum::{response::Html, Extension};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Serialize)]
struct PostToShow {
    id: i32,
    title: String,
    date: String,
    preview: String,
    img_path: String,
}

#[derive(FromRow)]
struct DatabasePost {
    id: i32,
    title: String,
    date: String,
    image_ext: String,
    content: String,
}


pub async fn index(
    Extension(pool): Extension<SqlitePool>,
    Extension(templates): Extension<Arc<Tera>>,
) -> Html<String> {
    let mut context = Context::new();
    let query = "SELECT * FROM posts";
    let rows = sqlx::query_as::<_, DatabasePost>(query)
        .fetch_all(&pool)
        .await
        .unwrap();
    let mut posts: Vec<PostToShow> = Vec::new();
    let mut preview: String;
    for row in rows {
        if row.content.len() < 101 {
            preview = row.content[..row.content.len()].to_owned();
        } else {
            preview = row.content[..100].to_owned() + "...";
        }
        posts.push(PostToShow {
            id: row.id,
            title: row.title,
            date: row.date,
            preview: preview,
            img_path: format!("/images/{}.{}", row.id, row.image_ext),
        });
    }
    context.insert("posts", &posts);

    Html(templates.render("index", &context).unwrap())
}
