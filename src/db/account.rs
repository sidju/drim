use super::*;

pub async fn add_user(mut db: sqlx::PgPool, user: User)
  -> Result<User, crate::Error>
{
  let query = sqlx::query_as!(User, "INSERT INTO users (email, pass, role) VALUES ($1, $2, $3) RETURNING email, pass, role")
    .bind(&user.email)
    .bind(&user.pass)
    .bind(&user.role)
    .fetch_one(&mut db)?
}
