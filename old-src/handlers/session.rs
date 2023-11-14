use warp::Rejection;
use warp::Filter;
use crate::db::{DbPool, User, sessions};
use crate::error::AuthErr;
use crate::error::Error;

use futures::future::Either;

// Filter for optionally getting user
pub fn try_user(db: DbPool)
  -> impl Filter<Extract = (Option<User>,), Error = Rejection> + Clone
{
  warp::cookie::optional("auth")
    .and_then( move | session: Option<String> | { match session {
      None => Either::Left(async { Ok(None) }),
      Some(uuid) => {
        use futures::future::FutureExt;
        Either::Right(
          sessions::get_user(db.clone(), uuid)
            .then(| res | async { match res {
              Ok(user) => Ok(Some(user)),
              Err(Error::Sqlx(sqlx::Error::RowNotFound)) => Ok(None),
              Err(Error::Uuid(_)) => Ok(None), // Ignore invalid uuids
              Err(Error::Sqlx(e)) => Err(warp::reject::custom(AuthErr::Sqlx(e))),
              Err(x) => Err(warp::reject::custom(AuthErr::Impossible(format!("{:?}",x)))),
            }})
        )
      },
    }})
}
// Filter that requires user to be logged in
pub fn user(db: DbPool) 
  -> impl Filter<Extract = (User,), Error = Rejection> + Clone
{
  warp::cookie::cookie("auth")
    .and_then( move | uuid: String | {
      use futures::future::FutureExt;
      sessions::get_user(db.clone(), uuid)
        .then(| res | async { match res {
          Ok(user) => Ok(user),
          Err(Error::Sqlx(sqlx::Error::RowNotFound)) => {
            Err(warp::reject::custom(AuthErr::Unauthorized))
          },
          Err(Error::Uuid(_)) => Err(warp::reject::custom(AuthErr::Unauthorized)),
          Err(Error::Sqlx(e)) => Err(warp::reject::custom(AuthErr::Sqlx(e))),
          Err(x) => Err(warp::reject::custom(AuthErr::Impossible(format!("{:?}",x)))),
        }})
    })
}
/*
// Filter than requires GM to be logged in
pub fn gm(db: DbPool)
  -> impl Filter<Extract = (User,), Error = Rejection> + Clone
{
  user(db)
    .and_then(| user: User | {
      if &user.role == "user" {
        Err(AuthErr::Forbidden)
      }
      else {
        Ok(user)
      }
    })
}
// Filter than requires GM to be logged in
pub fn admin(db: DbPool)
  -> impl Filter<Extract = (User,), Error = Rejection> + Clone
{
  user(db)
    .and_then(| user: User | {
      if &user.role != "admin" {
        Err(AuthErr::Forbidden)
      }
      else {
        Ok(user)
      }
    })
}
*/
