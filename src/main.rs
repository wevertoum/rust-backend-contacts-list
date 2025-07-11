use axum::Router;
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod db;
mod handlers;
mod models;
mod routes;
#[derive(Clone)]
pub struct AppState {
    db_conn: Arc<DatabaseConnection>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_conn = Arc::new(db::init().await.expect("DB init failed"));
    let app_state = AppState { db_conn };
    let app = Router::new()
        .merge(routes::contacts::routes())
        .merge(routes::users::routes())
        .layer(CorsLayer::very_permissive())
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
