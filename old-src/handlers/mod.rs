use warp::{Filter, Rejection};
use warp::reply::Reply;
use std::convert::Infallible;
use askama::Template;
use validator::Validate;
use serde::Deserialize;

use crate::db::*;
use crate::Error;
use crate::auth::*;

mod session;
use session::*;

mod routes;
pub use routes::*;

mod login;

pub fn routes(state: crate::State)
  -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
  let index = warp::path::end()
    .and(try_user(state.dbpool.clone()))
    .map(| user | format!("This is the index. user is {:?}", user));

  let hello = warp::path("pool")
    .and(with_var(state.dbpool.clone()))
    .map(move |pool| format!("The pool debug is {:?}.", pool));

  index
    .or(hello)
    .or(login::routes(state))
}
pub fn try_render(view: impl Template)
  -> Result<impl Reply, Error>
{
  view.render()
    .map(|body| warp::reply::html(body))
    .map_err(|e| Error::from(e))
}
