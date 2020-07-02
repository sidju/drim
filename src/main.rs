// Routing and web serving
use warp::Filter;
// Validating form input requires macro import
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
  dbpool: db::DbPool,
  hasher: auth::Hasher,
}

#[tokio::main]
async fn main() {
  // Get the configuration from environment + .env file
  let conf = conf::get_conf();

  let dbpool = db::init(&conf.db_url).await;

  let mut hasher = auth::Hasher::new();
  hasher.set_secret(&conf.hash_key);
  hasher.set_ad("Galf_drim".into());

  let admin_passhash = hasher.hash(conf.admin_password.clone()).await
    .expect("Failed to hash admin password.");
  db::users::set_admin(dbpool.clone(), conf.admin_username, admin_passhash).await
    .expect("Failed to set admin password.");

  let state = State {
    dbpool: dbpool,
    hasher: hasher,
  };

  warp::serve(handlers::routes(state).recover(error::handle_rejection))
    .run((conf.bind_ip, conf.bind_port))
    .await;
}
