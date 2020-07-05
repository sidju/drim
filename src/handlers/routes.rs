use warp::{Filter, Rejection, Reply};
use warp::http::StatusCode;
use std::convert::Infallible;

use crate::error::AuthErr;

// ------------------------------------ //
// Some helpers for constructing routes //
// ------------------------------------ //
pub fn with_var<C>(var: C)
  -> impl Filter<Extract = (C,), Error = Infallible> + Clone
where
  C: Clone + Send
{
  // The clone is needed since the produced closure will be run multiple times
  warp::any().map(move || var.clone())
}
pub fn with_db(db: sqlx::PgPool)
  -> impl Filter<Extract = (sqlx::PgPool,), Error = Infallible> + Clone
{
  // The clone is needed since the produced closure will be run multiple times
  warp::any().map(move || db.clone())
}

pub async fn handle_rejection(err: Rejection)
  -> Result<impl Reply, std::convert::Infallible>
{
  let (code, body) =
    if let Some(error) = err.find::<AuthErr>() {
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
