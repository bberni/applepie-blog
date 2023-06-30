use axum::{Extension, Json};
use http::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;
use base64::{Engine as _, engine::general_purpose};
use crate::check_hash;
use std::{fs::File, io::Write, error};
#[derive(Deserialize)]
pub struct FromCMS {
    hash: String,
    title: String,
    date: String,
    image: String,
    image_ext: String,
    content: String,
}

fn save_image(b64image: String, id: i32, ext: &String) -> Result<(), Box<dyn error::Error>> {
    let img_bytes = match general_purpose::URL_SAFE_NO_PAD.decode(b64image) {
        Ok(x) => x,
        Err(why) => {return Err(Box::new(why));}
    };

    let mut file = File::create("src/images/".to_string() + format!("{}.{}", id, ext).as_str()).unwrap();
    match file.write_all(&img_bytes) {
        Ok(_) => Ok(()),
        Err(why) => Err(Box::new(why))
    }
}
pub async fn new_post(
    Extension(pool): Extension<SqlitePool>,
    Json(params): Json<FromCMS>,
) -> http::StatusCode {
    
    if check_hash(params.hash) != 1 {
        return StatusCode::FORBIDDEN;
    };
    if !["jpg", "jpeg", "png", "bmp", "webp"].contains(&params.image_ext.as_str()){
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    let insert_query = "
        INSERT INTO posts (title, date, image_ext, content)
        VALUES (?, ?, ?, ?)";   
    match sqlx::query(insert_query)
        .bind(params.title)
        .bind(params.date)
        .bind(&params.image_ext)
        .bind(params.content)
        .execute(&pool)
        .await {
            Ok(_) => {},
            Err(_) => {return StatusCode::INTERNAL_SERVER_ERROR;}
        };
        let id_query = "SELECT MAX(id) FROM posts"; 
        let max_id = sqlx::query_scalar::<_, i32>(id_query)
            .fetch_one(&pool)
            .await.unwrap(); 
        match save_image(params.image, max_id, &params.image_ext) {
            Err(_) => {return StatusCode::INTERNAL_SERVER_ERROR;}
            _ => {return StatusCode::OK}
        }
}
