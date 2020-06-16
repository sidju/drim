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
  let conf = conf::get_conf();

  let dbpool = db::init(&conf.db_url).await;

  let mut hasher = auth::Hasher::new();
  hasher.set_secret(&conf.hash_key);

  let state = State {
    dbpool: dbpool,
    hash_key: conf.hash_key,
  };

  warp::serve(handlers::routes(state).recover(error::handle_rejection))
    .run((conf.bind_ip, conf.bind_port))
    .await;
}
