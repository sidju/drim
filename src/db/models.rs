// Structs to represent database tables

// Unlikely to ever be used
#[derive(Debug)]
pub struct Role {
  pub name: String,
}

// Used to represent accounts
pub struct User {
  id: u32,
  email: String,
  pass: String,
  role: String,
}

// Player data, no matter if they have an account or not
pub struct Player {
  id: u32,
  name: String,
  nationalid: String,
  email: String,
  telephone: String,
  active: bool,
  paid: bool,
  inebas: bool,
}
