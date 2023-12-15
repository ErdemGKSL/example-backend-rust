mod routes;
pub mod library;

#[tokio::main]
async fn main() {
    let app = routes::get_router();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9876").await.unwrap();
    println!("Server started at {}", "http://localhost:9876");

    axum::serve(listener, app).await.unwrap();
}