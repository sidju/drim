// For environment based configuration
use dotenv::dotenv;
// Database interactions
use sqlx;

mod db;
mod handlers;

#[tokio::main]
async fn main() {
  dotenv().unwrap(); // Reads the .env file in run-dir and sets the environment variables accordingly

  let dbpool = sqlx::PgPool::new("postgres://postgres:TeMpOr@172.16.20.2/postgres").await.expect("Failed to connect to database");

  warp::serve(handlers::routes(dbpool))
    .run(([0,0,0,0],8000)) // Declare listening adress in array, int tuple
    .await;
}
