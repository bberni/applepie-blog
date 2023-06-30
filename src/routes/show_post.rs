use axum::{extract::Query, response::Html, Extension};
use::std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tera::{Context, Tera};
use markdown;
#[derive(Deserialize)]
pub struct Id {
    id: i32,
}
#[derive(Serialize, FromRow)]
struct Post { 
    title: String, 
    date: String,
    content: String
}


pub async fn show_post(
    Extension(pool): Extension<SqlitePool>,
    Extension(templates): Extension<Arc<Tera>>,
    id: Query<Id>,
) -> Html<String> {
    let mut context = Context::new();
    let Id { id } = id.0;
    let query = "SELECT title, date, content FROM posts WHERE id = ?";
    let mut row = match sqlx::query_as::<_, Post>(query)
        .bind(id)
        .fetch_one(&pool)
        .await {
            Ok(x) => x, 
            Err(_) => {return Html("not found".to_string())}
        }
        ;
    row.content = markdown::to_html(row.content.as_str());
    context.insert("post", &row);
    Html(templates.render("post", &context).unwrap())
}
