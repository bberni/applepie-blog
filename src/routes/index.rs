use axum::{response::Html, Extension};
use serde::Serialize;
use sqlx::{FromRow, Row, SqlitePool};
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

struct DatabasePost {
    id: i32,
    title: String,
    date: String,
    image_ext: String,
    content: String,
}

impl FromRow<'_, sqlx::sqlite::SqliteRow> for DatabasePost {
    fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            date: row.try_get("date")?,
            image_ext: row.try_get("image_ext")?,
            content: row.try_get("content")?,
        })
    }
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
    for row in rows {
        posts.push(PostToShow {
            id: row.id,
            title: row.title,
            date: row.date,
            preview: (row.content[..100].to_owned() + "..."),
            img_path: format!("/images/{}.{}", row.id, row.image_ext),
        });
    }
    context.insert("posts", &posts);

    Html(templates.render("index", &context).unwrap())
}
