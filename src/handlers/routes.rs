use warp::{Filter, Rejection};

// ------------------------------------ //
// Some helpers for constructing routes //
// ------------------------------------ //

pub fn with_db(db: sqlx::PgPool)
  -> impl Filter<Extract = (sqlx::PgPool,), Error = std::convert::Infallible> + Clone
{
  // The clone is needed since the produced closure will be run multiple times
  warp::any().map(move || db.clone())
}
pub fn with_opt_auth() -> impl Filter<Extract = (Option<String>,), Error = std::convert::Infallible> + Clone
{
  warp::cookie::optional("auth")
}
pub fn with_auth() -> impl Filter<Extract = (Option<String>,), Error = std::convert::Infallible> + Clone
{
  warp::cookie::optional("auth")
}

