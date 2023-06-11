mod index;

use axum::{
    body::Body, 
    routing::{get},
    Router
};

use index::index;

pub fn create_routes() -> Router<(), Body> {
    Router::new()
        .route("/", get(index))
}