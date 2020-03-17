use sqlx;
use std::convert::Infallible;

mod models;
pub use models::*;

// Todo: Create user and session tables along with functions to interact with them

pub async fn roles(mut db: sqlx::PgPool)
  -> Result<impl warp::Reply, Infallible>
{
  let fut =  sqlx::query_as!(Role, "SELECT name FROM roles")
    .fetch_all(&mut db)
    .await;
  match fut {
    Ok(roles) => Ok(format!("The found roles were: {:?}", roles)),
    Err(e) => Ok(format!("No roles found. Error occured: {:?}", e)),
  }
}
