mod routes;
use routes::create_routes;
use sha2::{Digest, Sha256};
use hex;
use std::{env, process};

pub async fn run() {
    match check_vars() {
        Ok(_) => (), 
        Err(why) => {
            println!("{}", why);
            process::exit(1);
        }
    }
    let app = create_routes().await;
    let port = env::var("BLOG_SERVER_PORT").unwrap();

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

pub fn check_hash(hash: String) -> i32 {
    let password = env::var("BLOG_SERVER_PASSWORD").unwrap();
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hex::encode(hasher.finalize());
    if result == hash {
        return 1;
    } else {
        return 0;
    }
}

fn check_vars() -> Result<(), String> {
    match env::var("BLOG_SERVER_PORT") {
        Ok(_) => (),
        Err(_) => {return Err("Environment variable BLOG_SERVER_PORT not found".to_string())}
    };
    match env::var("BLOG_SERVER_PASSWORD") {
        Ok(_) => Ok(()),
        Err(_) => Err("Environment variable BLOG_SERVER_PASSWORD not found".to_string())
    }
}