use super::*;

pub fn error_500() -> impl warp::reply::Reply {
  warp::reply::html("500: Internal server error")
}
pub fn error_403() -> impl warp::reply::Reply {
  warp::reply::html("405: Unauthorised.")
}

pub fn internal_error(e: impl std::error::Error) -> impl warp::reply::Reply {
  println!("Internal error occured: {:?}", e);
  error_500()
}
pub fn try_render(view: impl Template) -> impl warp::reply::Reply {
  match view.render() {
    Ok(body) => warp::reply::html(body),
    Err(e) => {println!("{:?}", e); error_500()},
  }
}
