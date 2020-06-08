use warp::reject::{Rejection, custom};
use crate::Error;
use crate::db::*;
use argon2::{self, Config};

// A hasher and verifier that lives in a static variable with all its settings
// This along with immutable operation makes it completely free from cloning
pub struct Hasher {
  config: Config<'static>,
}
impl Hasher {
  pub async fn hash(&'static self, password: String) -> Result<String, Error> {
    let func = move || {
      let salt: [u8; 32] = rand::random();
      let hash = argon2::hash_raw(
        password.as_bytes(),
        &salt,
        &(self.config),
      )?;
      let mut res = base64::encode(hash);
      res.push('$');
      res.push_str(&base64::encode(salt));
      Ok(res)
    };
    tokio::task::spawn_blocking(func)
      .await?
  }
  pub async fn verify(&'static self, password: String, passhash: String)
    -> Result<bool, Error>
  {
    let func = move || {
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
        &(self.config),
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
