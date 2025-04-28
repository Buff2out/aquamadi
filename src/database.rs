use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn get_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let user_pg = env::var("POSTGRES_USER")
        .expect("POSTGRES_USER not found in env");
    let password_pg = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found in env");
    let db_pg = env::var("POSTGRES_DB").expect("POSTGRES_DB not found in env");


    /* 
        postgres://aquamadi:secret@localhost:5432/aquamadi_db
    */
    let url = format!("postgres://{user_pg}:{password_pg}@localhost:5432/{db_pg}");
        
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
}