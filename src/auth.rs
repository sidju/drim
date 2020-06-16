use warp::reject::{Rejection, custom};
use crate::Error;
use crate::db::*;
use argon2::{self, Config};
// A hasher and verifier that keeps all its settings internally
pub struct Hasher{
  // The secret and ad stored separate, since the config uses references
  // The config is kept partially constructed and is assembled in run-thread
  // Otherwise reference lifetimes cause issues
  secret: Vec<u8>,
  ad: Vec<u8>,
  // The Config must not contain any non-static references
  config: Config<'static>,
}
impl Hasher {
  pub fn new() -> Self {
    Self{
      ad: Vec::new(),
      secret: Vec::new(),
      config: Config{
        ad: &[],
        secret: &[],
        hash_length: 32,
        lanes: 8,
        mem_cost: 4096,
        time_cost: 3,
        thread_mode: argon2::ThreadMode::Parallel,
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
      }
    }
  }
  pub fn set_secret(&mut self, b64_secret: &str) {
    let secret = base64::decode(&b64_secret).unwrap();
    self.secret = secret;
  }
  pub fn set_ad(&mut self, b64_ad: &str) {
    let ad = base64::decode(&b64_ad).unwrap();
    self.ad = ad;
  }
  pub async fn hash(&self, password: String) -> Result<String, Error> {
    // Clone the relevant data for the config
    let secret = self.secret.clone();
    let ad = self.ad.clone();
    let  config = self.config.clone();
    let func = move || {
      // Construct the conf from its parts
      let mut conf = config;
      conf.secret = &secret;
      conf.ad = &ad;
      let salt: [u8; 32] = rand::random();
      let hash = argon2::hash_raw(
        password.as_bytes(),
        &salt,
        &conf,
      )?;
      let mut res = base64::encode(hash);
      res.push('$');
      res.push_str(&base64::encode(salt));
      Ok(res)
    };
    tokio::task::spawn_blocking(func)
      .await?
  }
  pub async fn verify(&self, password: String, passhash: String)
    -> Result<bool, Error>
  {
    // Clone the relevant data for the config
    let secret = self.secret.clone();
    let ad = self.ad.clone();
    let  config = self.config.clone();
    let func = move || {
      // Construct the conf from its parts
      let mut conf = config;
      conf.secret = &secret;
      conf.ad = &ad;
      let mut old_hash = Vec::new();
      let mut salt = Vec::new();
      for (i, data) in passhash.split('$').enumerate() {
        if i == 0 { old_hash = base64::decode(&data)?; }
        else if i == 1 { salt = base64::decode(&data)?; break; }
        else { break; }
      }
      let hash = argon2::hash_raw(
        password.as_bytes(),
        &salt,
        &conf,
      )?;
      let res = hash == old_hash;
      Ok(res)
    };
    tokio::task::spawn_blocking(func)
      .await?
  }
}
pub fn validate_role(role: &str)
  -> Result<(), validator::ValidationError>
{
  if role == "admin" || role == "gm" || role == "user" {
    Ok(())
  }
  else {
    Err(validator::ValidationError::new("Invalid role"))
  }
}
pub fn get_user(db: DbPool, auth: String)
  -> Result<User, Rejection>
{
  Err(custom(Error::Unauthorized))
}
