// For environment based configuration
use dotenv::dotenv;
// Database interactions
use sqlx;
// Routing and web serving
use warp::Filter;
// Deserialising form input
//use serde;
// Validating form input
#[macro_use]
extern crate validator_derive;
extern crate validator;

mod error;
pub use error::Error;

mod conf;
mod db;
mod handlers;
mod auth;

pub struct State {
  dbpool: sqlx::PgPool,
  hash_key: String,
}

#[tokio::main]
async fn main() {
  dotenv().unwrap(); // Reads the .env file in run-dir and sets the environment variables accordingly

  let conf = conf::get_conf();

  let dbpool = sqlx::PgPool::new(&conf.db_url)
    .await
    .expect("Failed to connect to database");

  let state = State {
    dbpool: dbpool,
    hash_key: conf.hash_key,
  };

  warp::serve(handlers::routes(state).recover(error::handle_rejection))
    .run((conf.bind_ip, conf.bind_port))
    .await;
}
