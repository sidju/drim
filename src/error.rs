// The rejection for the auth filter
#[derive(Debug)]
pub enum AuthErr {
  Forbidden,
  Unauthorized, // Not signed in
  Sqlx(sqlx::Error),
  Impossible(String),
}
impl warp::reject::Reject for AuthErr {}

// The crate wide error type
#[derive(Debug)]
pub enum Error {
  Render(askama::Error),
  Hash(argonautica::Error),
  Hashing(argon2::Error),
  Block(tokio::task::JoinError),
  Base64(base64::DecodeError),
  Sqlx(sqlx::Error),
  Uuid(uuid::Error),
}

impl std::convert::From<askama::Error> for Error {
  fn from(error: askama::Error) -> Self {
    Self::Render(error)
  }
}
impl std::convert::From<uuid::Error> for Error {
  fn from(error: uuid::Error) -> Self {
    Self::Uuid(error)
  }
}
impl std::convert::From<argonautica::Error> for Error {
  fn from(error: argonautica::Error) -> Self {
    Self::Hash(error)
  }
}
impl std::convert::From<argon2::Error> for Error {
  fn from(error: argon2::Error) -> Self {
    Self::Hashing(error)
  }
}
impl std::convert::From<tokio::task::JoinError> for Error {
  fn from(error: tokio::task::JoinError) -> Self {
    Self::Block(error)
  }
}
impl std::convert::From<base64::DecodeError> for Error {
  fn from(error: base64::DecodeError) -> Self {
    Self::Base64(error)
  }
}
impl std::convert::From<sqlx::Error> for Error {
  fn from(error: sqlx::Error) -> Self {
    Self::Sqlx(error)
  }
}
