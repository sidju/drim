use warp::{Filter, Rejection};
use std::convert::Infallible;
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
pub fn with_opt_auth() -> impl Filter<Extract = (Option<String>,), Error = Infallible> + Clone
{
  warp::cookie::optional("auth")
}
pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone
{
  warp::cookie::cookie("auth")
}
