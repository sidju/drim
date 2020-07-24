use warp::{Filter, Rejection, Reply};
use warp::http::StatusCode;
use std::convert::Infallible;

use crate::error::AuthErr;

// ------------------------------------ //
// Some helpers for constructing routes //
// ------------------------------------ //
pub fn with_var<T>(var: T)
  -> impl Filter<Extract = (T,), Error = Infallible> + Clone
where
  T: Clone + Send
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
  -> Result<impl Reply, Infallible>
{
  let response =
    if let Some(error) = err.find::<AuthErr>() {
      match error {
        AuthErr::Unauthorized => {
          warp::redirect::temporary(warp::http::Uri::from_static("/login"))
            .into_response()
        },
        AuthErr::Forbidden => {
          warp::reply::with_status(
            "You aren't allowed to view this page.",
            StatusCode::FORBIDDEN,
          )
            .into_response()
        },
        _ => {
          eprintln!("Unhandled rejection: {:?}", err);
          warp::reply::with_status(
            "Something went wrong in an unexpected way. We'll look into it.",
            StatusCode::INTERNAL_SERVER_ERROR,
          )
            .into_response()
        },
      }
    }
    else if let Some(error) = err.find::<warp::filters::body::BodyDeserializeError>() {
      eprintln!("Bad request: {:?}", error);
      warp::reply::with_status(
        "That request didn't make any sense to the server... If this persists please contact the administrator.",
        StatusCode::BAD_REQUEST,
      )
        .into_response()
    }
    else {
      eprintln!("Unhandled rejection: {:?}", err);
      warp::reply::with_status(
        "Something went wrong in an unexpected way. We'll look into it.",
        StatusCode::INTERNAL_SERVER_ERROR,
      )
        .into_response()
    }
  ;
  Ok(response)
}
