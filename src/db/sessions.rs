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

pub async fn add(db: DbPool, session: Session)
  -> Result<Session, Error>
{
  Ok(
    sqlx::query_as!(Session,
      "INSERT INTO sessions (id, uuid, userid, until) VALUES (DEFAULT, $1, $2, $3) RETURNING id, uuid, userid, until",
      session.uuid,
      session.userid,
      session.until
    )
      .fetch_one(&db).await?
  )
}
