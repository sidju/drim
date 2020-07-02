use super::*;

pub fn routes(state: crate::State)
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

  let add_user_get_path = warp::get()
    .and(with_var(state.dbpool.clone()))
    .and(with_auth())
    .and_then(add_user_get)
  ;
  let add_user_post_path = warp::post()
    .and(with_var(state.hasher.clone()))
    .and(with_db(state.dbpool.clone()))
    .and(with_auth())
    .and(warp::body::content_length_limit(1024*32))
    .and(warp::body::form())
    .and_then(add_user_post)
  ;

  let add_user = warp::path("adduser")
    .and(add_user_get_path.or(add_user_post_path))
  ;

  login.or(add_user)
}

#[derive(Deserialize, Validate, Debug)]
pub struct AddUserForm {
  #[validate(email)]
  email: String,
  #[validate(length(min = 8))]
  password: String,
  #[validate(must_match = "password")]
  password_conf: String,
  #[validate(custom = "crate::auth::validate_role")]
  role: String,
}
#[derive(Template)]
#[template(path = "add_user.html")]
struct AddUserView {
  topbar: Option<String>,
  username: String,
  role: String,
}
impl AddUserView {
  pub fn new(topbar: String) -> Self {
    Self{
      topbar: Some(topbar),
      username: String::new(),
      role: "user".to_string(),
    }
  }
}
pub async fn add_user_get(db: DbPool, auth: String)
  -> Result<impl Reply, Rejection>
{
//  let user = get_user(db, auth)?;

//  if user.role != "admin" {
//    Err(warp::reject::custom(Error::Forbidden))?;
//  }

  try_render(AddUserView::new("SOON tm".to_string()))
}
pub async fn add_user_post(hasher: Hasher, db: DbPool, auth: String, form: AddUserForm)
  -> Result<impl Reply, Rejection>
{
  eprintln!("Form received: {:?}", form);
  form.validate().unwrap();
  try_render(AddUserView::new("SOON tm".to_string()))
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

