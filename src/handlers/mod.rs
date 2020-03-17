use warp::{Filter, Rejection};
use askama::Template;
use crate::db;

mod routes;
use routes::*;

pub fn routes(dbpool: sqlx::PgPool)
  -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone
{
  let login_get_path = warp::get()
    .map(login_get);

  let login_post_path = warp::post()
    .and(warp::body::content_length_limit(1024 * 32))
    .and(warp::body::form())
    .map(login_post);

  let login = warp::path("login")
    .and(login_get_path.or(login_post_path));

  let index = warp::path::end()
    .and(with_opt_auth())
    .map(| id | format!("This is the index. ID is {:?}", id));

  let hello = warp::path("pool")
    .and(with_db(dbpool.clone()))
    .map(move |pool| format!("The pool debug is {:?}.", pool));

  let roles = warp::path("roles")
    .and(with_db(dbpool.clone()))
    .and_then(db::roles);

  index.or(hello).or(roles).or(login)
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginView {
  topbar: Option<String>,
  username: String,
  remember_me: Option<String>,
}
pub fn login_get() -> impl warp::reply::Reply {
  let body = match (LoginView{
    topbar: None,
    username: "".to_string(),
    remember_me: None,
  }).render() {
    Ok(res) => res,
    Err(e) => format!("Error occured: {}", e),
  };
  warp::reply::html(body)
}

pub fn login_post(query: std::collections::HashMap<String, String>) -> impl warp::reply::Reply {
  let body = format!("This was the login post submitted: {:?}", query);

    // Create a response using the body above
    warp::http::Response::builder()
      .header(warp::http::header::SET_COOKIE, "auth=test-value; HttpOnly; SameSite=Strict")
      .body(body)
}

