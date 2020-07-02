use super::*;

pub async fn add_user(db: DbPool, user: User)
  -> Result<User, crate::Error>
{
  Ok(
    sqlx::query_as!(User,
      "INSERT INTO users (id, nick, email, pass, role) VALUES (DEFAULT, $1, $2, $3, $4) RETURNING id, nick, email, pass, role",
      &user.nick,
      &user.email,
      &user.pass,
      &user.role
    )
      .fetch_one(&db).await?
  )
}

pub async fn get_user(db: DbPool, id: i32)
  -> Result<User, crate::Error>
{
  Ok(
    sqlx::query_as!(User,
      "SELECT id, nick, email, pass, role FROM users WHERE id = $1",
      id
    )
      .fetch_one(&db).await?
  )
}

pub async fn update_user(db: DbPool, user: User)
  -> Result<User, crate::Error>
{
  Ok(
    sqlx::query_as!(User,
      "UPDATE users SET nick = $1, email = $2, pass = $3, role = $4 WHERE id = $5 RETURNING id, nick, email, pass, role",
      &user.nick,
      &user.email,
      &user.pass,
      &user.role,
      user.id
    )
      .fetch_one(&db).await?
  )
}    

pub async fn set_admin(db: DbPool, username: String, passhash: String)
  -> Result<(), crate::Error>
{
  sqlx::query!(
    "INSERT INTO users(id, nick, email, pass, role) VALUES(0, 'admin', $1, $2, 'admin')
     ON CONFLICT (id) DO
     UPDATE SET nick = 'admin', email = $1, pass = $2, role = 'admin' WHERE users.id = 0",
    &username,
    &passhash
  )
    .execute(&db).await?;
  Ok(())
}
