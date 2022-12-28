use std::collections::HashMap;
use dotenv::dotenv;
use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;
// use std::net::SocketAddr;

use db::app_configure::{AppConfigure, DataType};
// static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    // let _ = POOL.set(pool);
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("Server listening on {}...", addr);
    let _ = AppConfigure::delete(&pool, 4).await;
    let rows = AppConfigure::all(&pool).await;
    println!("{:#?}", rows);
}
