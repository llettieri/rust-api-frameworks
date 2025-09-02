mod models;
mod routers;
mod services;

use axum::Router;
use axum::http::StatusCode;
use routers::vehicles::vehicles_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let root_router: Router = Router::new()
        .nest("/vehicle/v1/vehicles", vehicles_router())
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not Found") });

    // Define IP and port listener (TCP)
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Serve the application
    axum::serve(listener, root_router).await.unwrap();
}
