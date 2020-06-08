use warp::{Rejection, Reply};
use warp::http::StatusCode;

// The crate wide error type
#[derive(Debug)]
pub enum Error {
  Render(askama::Error),
  Hash(argonautica::Error),
  Hashing(argon2::Error),
  Block(tokio::task::JoinError),
  Base64(base64::DecodeError),
  Forbidden,
  Unauthorized, // Not signed in
}
impl warp::reject::Reject for Error {}
impl Error {
  pub fn from_to_reject(error: impl Into<Self>) -> Rejection {
    warp::reject::custom(Into::<Self>::into(error))
  }
}
impl std::convert::From<askama::Error> for Error {
  fn from(error: askama::Error) -> Self {
    Self::Render(error)
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
pub async fn handle_rejection(err: Rejection)
  -> Result<impl Reply, std::convert::Infallible>
{
  let (code, body) =
    if let Some(error) = err.find::<Error>() {
      match error {
        _ => {
          eprintln!("Unhandled rejection: {:?}", err);
          (StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
        },
      }
    }
    else if let Some(error) = err.find::<warp::filters::body::BodyDeserializeError>() {
      eprintln!("Bad request: {:?}", error);
      (StatusCode::BAD_REQUEST, "Bad request")
    }
    else {
      eprintln!("Unhandled rejection: {:?}", err);
      (StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
    }
  ;
  Ok(warp::reply::with_status(body, code))
}
