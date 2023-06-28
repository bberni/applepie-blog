use axum::{extract::Query, response::Html, Extension};
use::std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool, sqlite::SqliteRow};
use tera::{Context, Tera};
#[derive(Deserialize)]
pub struct Id {
    id: i32,
}
#[derive(Serialize)]
struct Post { 
    title: String, 
    date: String,
    content: String
}

impl FromRow<'_, SqliteRow> for Post {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            title: row.try_get("title")?,
            date: row.try_get("date")?,
            content: row.try_get("content")?,
        })
    }
}


pub async fn show_post(
    Extension(pool): Extension<SqlitePool>,
    Extension(templates): Extension<Arc<Tera>>,
    id: Query<Id>,
) -> Html<String> {
    let mut context = Context::new();
    let Id { id } = id.0;
    let query = "SELECT title, date, content FROM posts WHERE id = ?";
    let row = match sqlx::query_as::<_, Post>(query)
        .bind(id)
        .fetch_one(&pool)
        .await {
            Ok(x) => x, 
            Err(_) => {return Html("not found".to_string())}
        }
        ;
    context.insert("post", &row);
    Html(templates.render("post", &context).unwrap())
}
