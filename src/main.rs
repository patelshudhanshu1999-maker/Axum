pub mod database;
pub mod handlers;
pub mod models;

use axum::{
    Extension, Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let client = match database::connect().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error: Could not connect to MongoDB. Make sure it's running on localhost:27017.");
            eprintln!("Full error details: {:?}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/hello", get(handlers::auth::hello))
        .route("/register", post(handlers::auth::register))
        .layer(Extension(client));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
