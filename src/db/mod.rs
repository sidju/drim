pub use sqlx::PgPool as DbPool;
use crate::Error;
use sqlx::prelude::*;
use sqlx::types::chrono::NaiveDateTime as DateTime;

mod models;
pub use models::*;

pub mod users;
pub mod sessions;

pub async fn init(db_url: &str) -> DbPool {
  let pool = DbPool::new(db_url).await.expect("Failed to connect to database.");
  // After connecting create or replace the migration function and run it
  (&pool).execute(std::include_str!("migrate.sql"))
    .await.expect("Failed to check/fix database schema.");
  pool
}
