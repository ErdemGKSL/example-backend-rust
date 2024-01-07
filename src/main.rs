mod routes;
pub mod library;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let port = std::env::var("PORT").expect("PORT must be set in .env file");

    let db: PgPool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let app = routes::get_router(db);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:".to_owned() + port.as_str()).await.unwrap();
    println!("Server started at http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}