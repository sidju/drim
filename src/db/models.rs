// Structs to represent database tables
// Unlikely to ever be used
#[derive(Debug)]
pub struct Role {
  pub name: String,
}
// Used to represent accounts
pub struct User {
  pub id: u32,
  pub email: String,
  pub pass: String,
  pub role: String,
}
// Player data, no matter if they have an account or not
pub struct Player {
  pub id: u32,
  pub name: String,
  pub nationalid: String,
  pub email: String,
  pub telephone: String,
  pub active: bool,
  pub paid: bool,
  pub inebas: bool,
}
