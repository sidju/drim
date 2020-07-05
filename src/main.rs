// Routing and web serving
use warp::Filter;
// Validating form input requires macro import
#[macro_use]
extern crate validator_derive;
extern crate validator;

extern crate uuid;

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

  // Initiate database connection
  let dbpool = db::init(&conf.db_url).await;

  // Initiate hashing struct
  let mut hasher = auth::Hasher::new();
  hasher.set_secret(&conf.hash_key);
  hasher.set_ad("Galf_drim".into());

  // Set the admin account details
  let admin_passhash = hasher.hash(conf.admin_password.clone()).await
    .expect("Failed to hash admin password.");
  db::users::set_admin(dbpool.clone(), conf.admin_username, admin_passhash).await
    .expect("Failed to set admin password.");

  // Wrap it into the a state struct
  let state = State {
    dbpool: dbpool,
    hasher: hasher,
  };

  // Start serving based on routes from handlers
  warp::serve(handlers::routes(state).recover(handlers::handle_rejection))
    .run((conf.bind_ip, conf.bind_port))
    .await;
}
