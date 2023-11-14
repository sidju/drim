use super::*;

pub fn routes(state: crate::State)
  -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
  let login_get_path = warp::get()
    .map_async(login_get);

  let login_post_path = warp::post()
    .and(with_var(state.hasher.clone()))
    .and(with_var(state.dbpool.clone()))
    .and(warp::body::content_length_limit(1024 * 32))
    .and(warp::body::form())
    .map_async(login_post);

  let login = warp::path("login")
    .and(login_get_path.or(login_post_path));

/*
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
*/
  login //.or(add_user)
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginView {
  topbar: Option<String>,
  email: String,
  remember_me: Option<String>,
  wrong: bool,
}
#[derive(Debug, Deserialize)]
pub struct LoginForm {
  email: String,
  password: String,
  remember_me: Option<String>,
}
pub async fn login_get() -> Result<impl Reply, Error> {
  let body = LoginView{
    topbar: None,
    email: "".to_string(),
    remember_me: None,
    wrong: false,
  }.render()?;
  Ok(warp::reply::html(body))
}

//pub fn login_post(query: std::collections::HashMap<String, String>) -> impl warp::reply::Reply {
pub async fn login_post(hasher: Hasher, db: DbPool, query: LoginForm)
  -> Result<impl warp::Reply, Error>
{
  println!("This was the login post submitted: {:?}", query);

  let user = match users::get_by_email(db.clone(), query.email.clone()).await {
    Ok(user) => Ok(user),
    // If the user doesn't exist we still do the hashing, to not reveal with accounts exist
    Err(Error::Sqlx(sqlx::Error::RowNotFound)) => Ok(User{
      id: -1,
      nick: "".to_string(),
      email: "".to_string(),
      // A completely valid password to take the normal time verifying
      // To prevent guessing it and getting in we exclude user.id == -1 below
      pass: "k4HOIbNuN5/+SPcrNhSE2VokxSEplChw1eLXsit+Lpg=$Av6cv6xcbyPtxl8zNKPh0AzQalJSIlrlxbN31uhGSQc=".to_string(),
      role: "".to_string(),
    }),
    Err(e) => Err(e),
  }?;

  Ok(
    if 
      hasher.verify(query.password, user.pass).await.unwrap() &&
      user.id != -1
    {
      let until = match query.remember_me {
        Some(_) => chrono::offset::Utc::now() + chrono::Duration::weeks(26),
        // Avoid session cookies, since we cannot predict how long they last
        // Use 24h cookies instead
        None => chrono::offset::Utc::now() + chrono::Duration::days(1),
      };
      let session = sessions::add(db, Session{
        id: -1,
        uuid: uuid::Uuid::new_v4(),
        userid: user.id,
        until: until.naive_utc()
      }).await?;

      // Session is created and all is good.
      // Give the auth cookie to the user and redirect them
      let redirect = warp::redirect::temporary(warp::http::Uri::from_static("/"));
      warp::reply::with_header(redirect, "set-cookie",
        &format!("auth={}; Expires={}; SameSite=Strict; Secure; HttpOnly",
          session.uuid,
          until.format("%a, %d %b %Y %H:%M:%S GMT"),
        )
      )
        .into_response()
    }
    else { 
      let body = LoginView{
        topbar: None,
        email: query.email,
        remember_me: query.remember_me,
        wrong: true,
      }.render()?;
      warp::reply::html(body)
        .into_response()
    }
  )
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

  Ok(try_render(AddUserView::new("SOON tm".to_string())).unwrap())
}
pub async fn add_user_post(hasher: Hasher, db: DbPool, auth: String, form: AddUserForm)
  -> Result<impl Reply, Rejection>
{
  eprintln!("Form received: {:?}", form);
  form.validate().unwrap();
  Ok(try_render(AddUserView::new("SOON tm".to_string())).unwrap())
}
