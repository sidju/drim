pub use sqlx::PgPool as DbPool;
use sqlx::prelude::*;
use std::convert::Infallible;

mod models;
pub use models::*;

pub mod users;

pub async fn init(db_url: &str) -> DbPool {
  let pool = DbPool::new(db_url).await.expect("Failed to connect to database.");
  // After connecting create or replace the migration function and run it
  (&pool).execute(std::include_str!("migrate.sql"))
    .await.expect("Failed to check/fix database schema.");
  pool
}
// Todo: Create user and session tables along with functions to interact with them

pub async fn roles(mut db: sqlx::PgPool)
  -> Result<impl warp::Reply, Infallible>
{
  let fut =  sqlx::query_as!(Role, "SELECT name FROM roles")
    .fetch_all(&db)
    .await;
  match fut {
    Ok(roles) => Ok(format!("The found roles were: {:?}", roles)),
    Err(e) => Ok(format!("No roles found. Error occured: {:?}", e)),
  }
}
