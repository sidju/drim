use warp::{Filter, Rejection};
use warp::reply::Reply;
use askama::Template;
use validator::Validate;
use serde::Deserialize;

use crate::db::*;
use crate::Error;
use crate::auth::*;

mod routes;
use routes::*;

mod login;


pub fn routes(state: crate::State)
  -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone
{
  let index = warp::path::end()
    .and(with_opt_auth())
    .map(| id | format!("This is the index. ID is {:?}", id));

  let hello = warp::path("pool")
    .and(with_var(state.dbpool.clone()))
    .map(move |pool| format!("The pool debug is {:?}.", pool));

  let roles = warp::path("roles")
    .and(with_var(state.dbpool.clone()))
    .and_then(crate::db::roles);

  index.or(hello).or(roles).or(login::routes(state))
}
pub fn try_render(view: impl Template)
  -> Result<impl Reply, Rejection>
{
  view.render()
    .map(|body| warp::reply::html(body))
    .map_err(|e| warp::reject::custom(Error::from(e)))
}
