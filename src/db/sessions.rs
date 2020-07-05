use super::*;

pub async fn get_user(db: DbPool, uuid: String)
  -> Result<User, Error>
{
  let parsed = sqlx::types::Uuid::parse_str(&uuid)?;

  Ok(
    sqlx::query_as!(User,
      "SELECT users.id, users.nick, users.email, users.pass, users.role FROM sessions JOIN users ON sessions.userid = users.id WHERE sessions.uuid = $1",
      parsed
    )
      .fetch_one(&db).await?
  )
}
