mod database;
mod error;

use axum::{Router, routing::get};
use database::get_db_pool;
use tracing::info;

#[tokio::main]
async fn main() {
    // Инициализация логов
    tracing_subscriber::fmt().init();
    
    // Загружаем .env
    dotenvy::dotenv().ok();
    
    // Подключаемся к БД
    let pool = get_db_pool().await.expect("Failed to connect to DB");
    info!("✅ Successfully connected to database");

    // Создаем роутер Axum
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool);

    // Запускаем сервер
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("🚀 Server started on port 3000");
    axum::serve(listener, app).await.unwrap();
}

// Простейший эндпоинт для проверки
async fn health_check() -> &'static str {
    "OK"
}